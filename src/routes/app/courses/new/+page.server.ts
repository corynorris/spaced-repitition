import { redirect } from "@sveltejs/kit";
import { base } from "$app/paths";
import { courseInput } from "$lib/validation/course";
import { createCourseForUser } from "$lib/server/repositories/courses";
import { aiCoursePreviewSchema } from "$lib/server/services/ai/schemas";
import { persistGeneratedCourse } from "$lib/server/services/ai/persist";
import { db } from "$lib/server/db/client";
import type { Actions } from "./$types";

export const actions: Actions = {
	default: async ({ request, locals }) => {
		if (!locals.user) throw redirect(303, `${base}/login`);

		const formData = await request.formData();
		const data = {
			title: formData.get("title"),
			description: formData.get("description") || null,
			sourceLanguage: formData.get("sourceLanguage") || null,
			targetLanguage: formData.get("targetLanguage") || null,
			languageProfile: formData.get("languageProfile") || "generic",
		};

		const parsed = courseInput.safeParse(data);
		if (!parsed.success) {
			return {
				errors: parsed.error.flatten().fieldErrors,
				values: data,
			};
		}

		const course = await createCourseForUser(db, locals.user.id, parsed.data);
		throw redirect(303, `${base}/app/courses/${course.id}`);
	},

	createAiCourse: async ({ request, locals }) => {
		if (!locals.user) throw redirect(303, `${base}/login`);

		const formData = await request.formData();
		const rawPreview = formData.get("preview");
		if (typeof rawPreview !== "string") {
			return { aiError: "Missing preview." };
		}

		let preview: unknown;
		try {
			preview = JSON.parse(rawPreview);
		} catch {
			return { aiError: "Generated course preview is no longer valid." };
		}

		const parsed = aiCoursePreviewSchema.safeParse(preview);
		if (!parsed.success) {
			return { aiError: "Generated course preview is no longer valid." };
		}

		const course = await persistGeneratedCourse(
			db,
			locals.user.id,
			parsed.data,
		);
		throw redirect(303, `${base}/app/courses/${course.id}`);
	},
};
