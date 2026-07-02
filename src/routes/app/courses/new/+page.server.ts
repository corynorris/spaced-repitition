import { fail, redirect } from "@sveltejs/kit";
import { base } from "$app/paths";
import { courseInput } from "$lib/validation/course";
import { createCourseForUser } from "$lib/server/repositories/courses";
import { aiCoursePreviewSchema } from "$lib/server/services/ai/schemas";
import { persistGeneratedCourse } from "$lib/server/services/ai/persist";
import { db } from "$lib/server/db/client";
import type { Actions } from "./$types";

function isUniqueCourseTitleError(err: unknown) {
	return (
		typeof err === "object" &&
		err !== null &&
		"code" in err &&
		(err as { code?: unknown }).code === "23505" &&
		"constraint_name" in err &&
		(err as { constraint_name?: unknown }).constraint_name ===
			"course_owner_title_idx"
	);
}

function courseCreateError(err: unknown) {
	if (isUniqueCourseTitleError(err)) {
		return "You already have a course with this title.";
	}

	console.error("Course creation failed", err);
	return "Could not create the course. Please try again.";
}

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
			return fail(400, {
				errors: parsed.error.flatten().fieldErrors,
				values: data,
				mode: "manual",
			});
		}

		let course;
		try {
			course = await createCourseForUser(db, locals.user.id, parsed.data);
		} catch (err) {
			return fail(isUniqueCourseTitleError(err) ? 409 : 500, {
				formError: courseCreateError(err),
				values: data,
				mode: "manual",
			});
		}

		throw redirect(303, `${base}/app/courses/${course.id}`);
	},

	createAiCourse: async ({ request, locals }) => {
		if (!locals.user) throw redirect(303, `${base}/login`);

		const formData = await request.formData();
		const rawPreview = formData.get("preview");
		if (typeof rawPreview !== "string") {
			return fail(400, { aiError: "Missing preview.", mode: "ai" });
		}

		let preview: unknown;
		try {
			preview = JSON.parse(rawPreview);
		} catch {
			return fail(400, {
				aiError: "Generated course preview is no longer valid.",
				mode: "ai",
			});
		}

		const parsed = aiCoursePreviewSchema.safeParse(preview);
		if (!parsed.success) {
			return fail(400, {
				aiError: "Generated course preview is no longer valid.",
				mode: "ai",
			});
		}

		let course;
		try {
			course = await persistGeneratedCourse(
				db,
				locals.user.id,
				parsed.data,
			);
		} catch (err) {
			return fail(isUniqueCourseTitleError(err) ? 409 : 500, {
				aiError: courseCreateError(err),
				mode: "ai",
			});
		}

		throw redirect(303, `${base}/app/courses/${course.id}`);
	},
};
