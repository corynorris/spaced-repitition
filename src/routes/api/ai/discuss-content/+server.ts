import { json } from "@sveltejs/kit";
import { z } from "zod";
import { db } from "$lib/server/db/client";
import { getCourseForUser } from "$lib/server/repositories/courses";
import { listNotesForCourse } from "$lib/server/repositories/notes";
import { discussContent } from "$lib/server/services/ai/generate";
import { assertAiRateLimit } from "$lib/server/services/ai/rate-limit";
import type { RequestHandler } from "./$types";

const inputSchema = z.object({
	courseId: z.string().uuid(),
	question: z.string().trim().min(1).max(1000),
	scope: z
		.enum(["course", "note", "sentence", "conversation"])
		.default("course"),
	noteIds: z.array(z.string().uuid()).max(20).default([]),
	content: z.string().trim().max(3000).optional(),
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

		assertAiRateLimit(locals.user.id, "discuss-content");
		const notes = await listNotesForCourse(db, course.id);
		const selected =
			parsed.data.noteIds.length > 0
				? notes.filter((note) => parsed.data.noteIds.includes(note.id))
				: notes.slice(0, 30);
		const noteContext = selected
			.map(
				(note) =>
					`${note.term}${note.reading ? ` (${note.reading})` : ""}: ${note.definition}`,
			)
			.join("\n");
		const context = [
			`scope: ${parsed.data.scope}`,
			parsed.data.content ? `content: ${parsed.data.content}` : "",
			noteContext,
		]
			.filter(Boolean)
			.join("\n");

		const answer = await discussContent({
			courseTitle: course.title,
			sourceLanguage: course.sourceLanguage,
			targetLanguage: course.targetLanguage,
			context,
			question: parsed.data.question,
		});

		return json({ answer });
	} catch (err) {
		const status =
			err instanceof Error && err.name === "RateLimitError" ? 429 : 500;
		console.error("AI discussion failed", err);
		return json({ error: "Could not discuss this content." }, { status });
	}
};
