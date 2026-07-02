import { json } from "@sveltejs/kit";
import { z } from "zod";
import { db } from "$lib/server/db/client";
import { getCourseForUser } from "$lib/server/repositories/courses";
import { getLearnedNotesForCourse } from "$lib/server/repositories/notes";
import { generatePracticePreview } from "$lib/server/services/ai/generate";
import { assertAiRateLimit } from "$lib/server/services/ai/rate-limit";
import type { RequestHandler } from "./$types";

const inputSchema = z.object({
	courseId: z.string().uuid(),
	mode: z.enum(["sentences", "conversation"]),
	topic: z.string().trim().max(200).optional(),
	selectedNoteIds: z.array(z.string().uuid()).max(50).default([]),
});

export const POST: RequestHandler = async ({ request, locals }) => {
	if (!locals.user) return json({ error: "Unauthorized" }, { status: 401 });

	try {
		let body: unknown;
		try {
			body = await request.json();
		} catch {
			return json({ error: "Invalid request" }, { status: 400 });
		}

		const parsed = inputSchema.safeParse(body);
		if (!parsed.success) {
			return json({ error: "Invalid request" }, { status: 400 });
		}

		const course = await getCourseForUser(
			db,
			parsed.data.courseId,
			locals.user.id,
		);
		if (!course) return json({ error: "Course not found" }, { status: 404 });

		assertAiRateLimit(locals.user.id, "generate-practice");
		const notes = await getLearnedNotesForCourse(
			db,
			course.id,
			parsed.data.selectedNoteIds,
		);
		const preview = await generatePracticePreview({
			mode: parsed.data.mode,
			courseTitle: course.title,
			sourceLanguage: course.sourceLanguage,
			targetLanguage: course.targetLanguage,
			languageProfile: course.languageProfile,
			topic: parsed.data.topic,
			knownWords: notes.map((note) => ({
				term: note.term,
				reading: note.reading,
				definition: note.definition,
			})),
		});

		return json({ preview });
	} catch (err) {
		const status =
			err instanceof Error && err.name === "RateLimitError" ? 429 : 500;
		console.error("AI practice generation failed", err);
		return json({ error: "Could not generate practice." }, { status });
	}
};
