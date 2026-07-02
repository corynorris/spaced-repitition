import {
	cards,
	courses,
	lessons,
	notes,
	reviewStates,
} from "$lib/server/db/schema";
import type { Database } from "$lib/server/db/client";
import { eq } from "drizzle-orm";
import { buildSentenceCards, buildVocabularyCards } from "$lib/cards/kinds";
import {
	defaultDisplayOptions,
	normalizeDisplayOptions,
	normalizeLanguageProfile,
} from "$lib/language/profiles";
import type {
	AiCoursePreview,
	AiLessonPreview,
	AiPracticePreview,
} from "./schemas";

async function insertCards(
	tx: Parameters<Parameters<Database["transaction"]>[0]>[0],
	noteId: string,
	cardDefs: Array<{
		kind: string;
		prompt: string;
		answer: string;
		extra?: Record<string, unknown>;
	}>,
) {
	if (cardDefs.length === 0) return;

	const createdCards = await tx
		.insert(cards)
		.values(
			cardDefs.map((card) => ({
				noteId,
				kind: card.kind,
				prompt: card.prompt,
				answer: card.answer,
				extra: card.extra ?? {},
			})),
		)
		.returning();

	await tx.insert(reviewStates).values(
		createdCards.map((card) => ({
			cardId: card.id,
			state: "new" as const,
		})),
	);
}

export async function persistGeneratedCourse(
	db: Database,
	userId: string,
	preview: AiCoursePreview,
) {
	return db.transaction(async (tx) => {
		const languageProfile = normalizeLanguageProfile(
			preview.languageProfile,
			preview.targetLanguage,
		);
		const displayOptions = defaultDisplayOptions(languageProfile);

		const [course] = await tx
			.insert(courses)
			.values({
				ownerId: userId,
				title: preview.title,
				description: preview.description ?? null,
				sourceLanguage: preview.sourceLanguage,
				targetLanguage: preview.targetLanguage,
				languageProfile,
				displayOptions,
			})
			.returning();

		for (const [orderIndex, lessonPreview] of preview.lessons.entries()) {
			await persistLesson(
				tx,
				course.id,
				lessonPreview,
				orderIndex,
				languageProfile,
				displayOptions,
			);
		}

		return course;
	});
}

export async function persistPracticePreview(
	db: Database,
	course: {
		id: string;
		languageProfile: string;
		displayOptions: Record<string, unknown>;
	},
	preview: AiPracticePreview,
) {
	return db.transaction(async (tx) => {
		const [lesson] = await tx
			.insert(lessons)
			.values({
				courseId: course.id,
				title: preview.title,
				type:
					preview.mode === "sentences" ? "sentence_practice" : "conversation",
				metadata:
					preview.mode === "conversation"
						? {
								generatedBy: "ai",
								setting: preview.setting ?? null,
								dialogue: preview.dialogue,
							}
						: { generatedBy: "ai" },
				orderIndex: await nextLessonOrder(tx, course.id),
			})
			.returning();

		if (preview.mode === "sentences") {
			for (const sentence of preview.sentences) {
				const [note] = await tx
					.insert(notes)
					.values({
						courseId: course.id,
						lessonId: lesson.id,
						term: sentence.target,
						furigana: sentence.furigana ?? null,
						definition: sentence.translation,
						tags: ["ai", "sentence"],
					})
					.returning();

				await insertCards(tx, note.id, buildSentenceCards(sentence));
			}
		} else {
			for (const [index, turn] of preview.dialogue.entries()) {
				const [note] = await tx
					.insert(notes)
					.values({
						courseId: course.id,
						lessonId: lesson.id,
						term: turn.target,
						furigana: turn.furigana ?? null,
						definition: turn.translation,
						notes: `${turn.speaker} in ${preview.setting ?? "conversation"}`,
						tags: ["ai", "conversation"],
					})
					.returning();

				await insertCards(
					tx,
					note.id,
					buildSentenceCards({
						target: turn.target,
						translation: turn.translation,
						clozeAnswer: turn.target.split(/\s+/)[0] ?? turn.target,
					}).map((card) => ({
						...card,
						extra: { ...card.extra, speaker: turn.speaker, turnIndex: index },
					})),
				);
			}
		}

		return lesson;
	});
}

async function persistLesson(
	tx: Parameters<Parameters<Database["transaction"]>[0]>[0],
	courseId: string,
	lessonPreview: AiLessonPreview,
	orderIndex: number,
	rawLanguageProfile: string,
	rawDisplayOptions: Record<string, unknown>,
) {
	const languageProfile = normalizeLanguageProfile(rawLanguageProfile);
	const displayOptions = normalizeDisplayOptions(
		languageProfile,
		rawDisplayOptions,
	);
	const [lesson] = await tx
		.insert(lessons)
		.values({
			courseId,
			title: lessonPreview.title,
			type: lessonPreview.type,
			metadata:
				lessonPreview.type === "conversation"
					? {
							generatedBy: "ai",
							setting: lessonPreview.setting ?? null,
							dialogue: lessonPreview.dialogue,
						}
					: { generatedBy: "ai" },
			orderIndex,
		})
		.returning();

	if (lessonPreview.type === "vocabulary") {
		for (const item of lessonPreview.items) {
			const [note] = await tx
				.insert(notes)
				.values({
					courseId,
					lessonId: lesson.id,
					term: item.term,
					reading: item.reading ?? null,
					furigana: item.furigana ?? null,
					definition: item.definition,
					example: item.example ?? null,
					exampleTranslation: item.exampleTranslation ?? null,
					partOfSpeech: item.partOfSpeech ?? null,
					tags: item.tags,
				})
				.returning();

			await insertCards(
				tx,
				note.id,
				buildVocabularyCards(note, languageProfile, displayOptions),
			);
		}
	} else if (lessonPreview.type === "sentence_practice") {
		for (const sentence of lessonPreview.sentences) {
			const [note] = await tx
				.insert(notes)
				.values({
					courseId,
					lessonId: lesson.id,
					term: sentence.target,
					furigana: sentence.furigana ?? null,
					definition: sentence.translation,
					tags: ["ai", "sentence"],
				})
				.returning();

			await insertCards(tx, note.id, buildSentenceCards(sentence));
		}
	} else {
		for (const [index, turn] of lessonPreview.dialogue.entries()) {
			const [note] = await tx
				.insert(notes)
				.values({
					courseId,
					lessonId: lesson.id,
					term: turn.target,
					furigana: turn.furigana ?? null,
					definition: turn.translation,
					notes: `${turn.speaker} in ${lessonPreview.setting ?? "conversation"}`,
					tags: ["ai", "conversation"],
				})
				.returning();

			await insertCards(
				tx,
				note.id,
				buildSentenceCards({
					target: turn.target,
					translation: turn.translation,
					clozeAnswer: turn.target.split(/\s+/)[0] ?? turn.target,
				}).map((card) => ({
					...card,
					extra: { ...card.extra, speaker: turn.speaker, turnIndex: index },
				})),
			);
		}
	}
}

async function nextLessonOrder(
	tx: Parameters<Parameters<Database["transaction"]>[0]>[0],
	courseId: string,
) {
	const existing = await tx
		.select({ orderIndex: lessons.orderIndex })
		.from(lessons)
		.where(eq(lessons.courseId, courseId))
		.orderBy(lessons.orderIndex);

	return (existing.at(-1)?.orderIndex ?? -1) + 1;
}
