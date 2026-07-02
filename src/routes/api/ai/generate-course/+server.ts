import { json } from "@sveltejs/kit";
import { z } from "zod";
import { generateCoursePreview } from "$lib/server/services/ai/generate";
import { assertAiRateLimit } from "$lib/server/services/ai/rate-limit";
import { languageProfileSchema } from "$lib/language/profiles";
import type { RequestHandler } from "./$types";

const inputSchema = z.object({
	topic: z.string().trim().min(1).max(300),
	sourceLanguage: z.string().trim().min(1).max(80).default("English"),
	targetLanguage: z.string().trim().min(1).max(80).default("Japanese"),
	languageProfile: languageProfileSchema.default("japanese"),
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

		assertAiRateLimit(locals.user.id, "generate-course");
		const preview = await generateCoursePreview(parsed.data);
		return json({ preview });
	} catch (err) {
		const status =
			err instanceof Error && err.name === "RateLimitError" ? 429 : 500;
		console.error("AI course generation failed", err);
		return json({ error: "Could not generate a course preview." }, { status });
	}
};
