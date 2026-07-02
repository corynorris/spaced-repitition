import { z } from "zod";
import { languageProfileSchema } from "$lib/language/profiles";

export const aiVocabularyItemSchema = z.object({
	term: z.string().trim().min(1).max(250),
	reading: z.string().trim().max(250).nullable().optional(),
	furigana: z.string().trim().max(1000).nullable().optional(),
	definition: z.string().trim().min(1).max(2000),
	example: z.string().trim().max(2000).nullable().optional(),
	exampleTranslation: z.string().trim().max(2000).nullable().optional(),
	partOfSpeech: z.string().trim().max(80).nullable().optional(),
	tags: z.array(z.string().trim().min(1).max(40)).max(8).default([]),
});

export const aiSentenceSchema = z.object({
	target: z.string().trim().min(1).max(500),
	translation: z.string().trim().min(1).max(500),
	furigana: z.string().trim().max(1000).nullable().optional(),
	blocks: z.array(z.string().trim().min(1).max(80)).min(1).max(24).optional(),
	clozeAnswer: z.string().trim().max(120).nullable().optional(),
});

export const aiDialogueTurnSchema = z.object({
	speaker: z.string().trim().min(1).max(40),
	target: z.string().trim().min(1).max(500),
	translation: z.string().trim().min(1).max(500),
	furigana: z.string().trim().max(1000).nullable().optional(),
});

export const aiLessonPreviewSchema = z.discriminatedUnion("type", [
	z.object({
		type: z.literal("vocabulary"),
		title: z.string().trim().min(1).max(120),
		items: z.array(aiVocabularyItemSchema).min(1).max(40),
	}),
	z.object({
		type: z.literal("sentence_practice"),
		title: z.string().trim().min(1).max(120),
		sentences: z.array(aiSentenceSchema).min(1).max(20),
	}),
	z.object({
		type: z.literal("conversation"),
		title: z.string().trim().min(1).max(120),
		setting: z.string().trim().max(200).nullable().optional(),
		dialogue: z.array(aiDialogueTurnSchema).min(2).max(20),
	}),
]);

export const aiCoursePreviewSchema = z.object({
	title: z.string().trim().min(1).max(120),
	description: z.string().trim().max(2000).nullable().optional(),
	sourceLanguage: z.string().trim().max(80).default("English"),
	targetLanguage: z.string().trim().max(80).default("Japanese"),
	languageProfile: languageProfileSchema.default("japanese"),
	lessons: z.array(aiLessonPreviewSchema).min(1).max(12),
});

export const aiPracticePreviewSchema = z.object({
	mode: z.enum(["sentences", "conversation"]),
	title: z.string().trim().min(1).max(120),
	sentences: z.array(aiSentenceSchema).max(20).default([]),
	setting: z.string().trim().max(200).nullable().optional(),
	dialogue: z.array(aiDialogueTurnSchema).max(20).default([]),
});

export type AiCoursePreview = z.infer<typeof aiCoursePreviewSchema>;
export type AiPracticePreview = z.infer<typeof aiPracticePreviewSchema>;
export type AiLessonPreview = z.infer<typeof aiLessonPreviewSchema>;
