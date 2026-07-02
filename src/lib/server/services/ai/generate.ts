import { callLlmWithRetry, loadLlmConfig } from "$lib/server/services/llm";
import { extractJsonObject, summarizeZodError } from "./json";
import { logger } from "$lib/server/logger";
import {
	aiCoursePreviewSchema,
	aiPracticePreviewSchema,
	type AiCoursePreview,
	type AiPracticePreview,
} from "./schemas";

type KnownWord = {
	term: string;
	reading: string | null;
	definition: string;
};

function requireLlmConfig() {
	const config = loadLlmConfig();
	if (!config) {
		throw new Error("LLM is not configured");
	}
	return config;
}

function validateOutput<T>(
	schema: {
		safeParse: (
			value: unknown,
		) => { success: true; data: T } | { success: false; error: any };
	},
	raw: string,
): T {
	const json = extractJsonObject(raw);
	const parsed = schema.safeParse(json);
	if (!parsed.success) {
		const reason = summarizeZodError(parsed.error);
		const config = loadLlmConfig();
		logger.aiValidationFailed({
			model: config?.model ?? "unknown",
			reason,
		});
		throw new Error(
			`AI output failed validation: ${reason}`,
		);
	}
	return parsed.data;
}

export async function generateCoursePreview(input: {
	topic: string;
	sourceLanguage: string;
	targetLanguage: string;
	languageProfile: "generic" | "japanese";
}): Promise<AiCoursePreview> {
	const config = requireLlmConfig();
	const topic = input.topic.slice(0, 300);

	const response = await callLlmWithRetry(config, {
		model: config.model,
		temperature: 0.4,
		max_tokens: 5000,
		messages: [
			{
				role: "system",
				content:
					"You generate compact SRS course previews as strict JSON only. Do not include markdown. Use targetLanguage for studied content and sourceLanguage for translations.",
			},
			{
				role: "user",
				content: `Create a ${input.languageProfile} language course preview.
Topic: ${topic}
sourceLanguage: ${input.sourceLanguage}
targetLanguage: ${input.targetLanguage}

Return this JSON shape:
{
  "title": "...",
  "description": "...",
  "sourceLanguage": "${input.sourceLanguage}",
  "targetLanguage": "${input.targetLanguage}",
  "languageProfile": "${input.languageProfile}",
  "lessons": [
    {"type":"vocabulary","title":"...","items":[{"term":"target word","reading":null,"furigana":null,"definition":"source translation","example":null,"exampleTranslation":null,"partOfSpeech":null,"tags":[]}]},
    {"type":"sentence_practice","title":"...","sentences":[{"target":"target sentence","translation":"source translation","furigana":null,"blocks":["target","blocks"],"clozeAnswer":"word"}]},
    {"type":"conversation","title":"...","setting":"...","dialogue":[{"speaker":"A","target":"target sentence","translation":"source translation","furigana":null}]}
  ]
}

For Japanese, term should be kanji/kana, reading hiragana when useful, and furigana should use bracket markup like 日本語[にほんご]. Include 8-12 vocabulary items, 3-5 sentences, and 4-8 dialogue turns.`,
			},
		],
	});

	return validateOutput(aiCoursePreviewSchema, response.content);
}

export async function generatePracticePreview(input: {
	mode: "sentences" | "conversation";
	courseTitle: string;
	sourceLanguage: string | null;
	targetLanguage: string | null;
	languageProfile: string;
	topic?: string;
	knownWords: KnownWord[];
}): Promise<AiPracticePreview> {
	const config = requireLlmConfig();
	const knownWords = input.knownWords.slice(0, 40);

	const response = await callLlmWithRetry(config, {
		model: config.model,
		temperature: 0.5,
		max_tokens: 4000,
		messages: [
			{
				role: "system",
				content:
					"You generate SRS practice previews as strict JSON only. Reuse the supplied known words naturally. Do not persist or invent progress state.",
			},
			{
				role: "user",
				content: `Course: ${input.courseTitle}
sourceLanguage: ${input.sourceLanguage ?? "English"}
targetLanguage: ${input.targetLanguage ?? "target language"}
languageProfile: ${input.languageProfile}
mode: ${input.mode}
topic: ${(input.topic ?? "").slice(0, 200)}
knownWords: ${JSON.stringify(knownWords)}

Return JSON:
For sentences:
{"mode":"sentences","title":"...","sentences":[{"target":"target sentence","translation":"source translation","furigana":null,"blocks":["target","blocks"],"clozeAnswer":"word"}],"dialogue":[]}
For conversation:
{"mode":"conversation","title":"...","setting":"...","sentences":[],"dialogue":[{"speaker":"A","target":"target sentence","translation":"source translation","furigana":null}]}

Generate 3-6 sentences or 4-8 dialogue turns. For Japanese furigana use bracket markup, never HTML.`,
			},
		],
	});

	const preview = validateOutput(aiPracticePreviewSchema, response.content);
	if (preview.mode !== input.mode) {
		return { ...preview, mode: input.mode };
	}
	return preview;
}

export async function discussContent(input: {
	courseTitle: string;
	sourceLanguage: string | null;
	targetLanguage: string | null;
	context: string;
	question: string;
}): Promise<string> {
	const config = requireLlmConfig();

	const response = await callLlmWithRetry(config, {
		model: config.model,
		temperature: 0.3,
		max_tokens: 1200,
		messages: [
			{
				role: "system",
				content:
					"You are a concise language tutor. Answer only about the supplied course context. Do not claim to save or change user data.",
			},
			{
				role: "user",
				content: `Course: ${input.courseTitle}
sourceLanguage: ${input.sourceLanguage ?? "English"}
targetLanguage: ${input.targetLanguage ?? "target language"}
Context:
${input.context.slice(0, 4000)}

Question: ${input.question.slice(0, 1000)}`,
			},
		],
	});

	return response.content.trim();
}
