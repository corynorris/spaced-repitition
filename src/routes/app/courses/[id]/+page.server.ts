import { error } from "@sveltejs/kit";
import {
	getCourseForUser,
	updateCourse,
} from "$lib/server/repositories/courses";
import { listNotesForCourse } from "$lib/server/repositories/notes";
import { getReviewStats } from "$lib/server/services/scheduler";
import {
	normalizeDisplayOptions,
	normalizeLanguageProfile,
} from "$lib/language/profiles";
import { aiPracticePreviewSchema } from "$lib/server/services/ai/schemas";
import { persistPracticePreview } from "$lib/server/services/ai/persist";
import { db } from "$lib/server/db/client";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, locals }) => {
	if (!locals.user) throw error(401, "Unauthorized");

	const course = await getCourseForUser(db, params.id, locals.user.id);
	if (!course) throw error(404, "Course not found");

	const [notes, stats] = await Promise.all([
		listNotesForCourse(db, course.id),
		getReviewStats(db, course.id),
	]);

	const languageProfile = normalizeLanguageProfile(
		course.languageProfile,
		course.targetLanguage,
	);
	const displayOptions = normalizeDisplayOptions(
		languageProfile,
		course.displayOptions,
	);

	return {
		course: { ...course, languageProfile, displayOptions },
		notes,
		stats,
	};
};

export const actions: Actions = {
	updateSettings: async ({ request, params, locals }) => {
		if (!locals.user) throw error(401, "Unauthorized");

		const course = await getCourseForUser(db, params.id, locals.user.id);
		if (!course) throw error(404, "Course not found");

		const formData = await request.formData();
		const languageProfile = normalizeLanguageProfile(
			formData.get("languageProfile"),
			course.targetLanguage,
		);
		const displayOptions = normalizeDisplayOptions(languageProfile, {
			showTerm: formData.get("showTerm") === "on",
			showReading: formData.get("showReading") === "on",
			showDefinition: formData.get("showDefinition") === "on",
			showKanji: formData.get("showKanji") === "on",
			showHiragana: formData.get("showHiragana") === "on",
			showFurigana: formData.get("showFurigana") === "on",
			showEnglish: formData.get("showEnglish") === "on",
			showExamples: formData.get("showExamples") === "on",
			activeCardTypes: formData.getAll("activeCardTypes"),
		});

		await updateCourse(db, course.id, locals.user.id, {
			languageProfile,
			displayOptions,
		});

		return { settingsSaved: true };
	},

	createPractice: async ({ request, params, locals }) => {
		if (!locals.user) throw error(401, "Unauthorized");

		const course = await getCourseForUser(db, params.id, locals.user.id);
		if (!course) throw error(404, "Course not found");

		const formData = await request.formData();
		const rawPreview = formData.get("preview");
		if (typeof rawPreview !== "string") {
			return { practiceError: "Missing preview." };
		}

		let preview: unknown;
		try {
			preview = JSON.parse(rawPreview);
		} catch {
			return {
				practiceError: "Generated practice preview is no longer valid.",
			};
		}

		const parsed = aiPracticePreviewSchema.safeParse(preview);
		if (!parsed.success) {
			return {
				practiceError: "Generated practice preview is no longer valid.",
			};
		}

		await persistPracticePreview(db, course, parsed.data);
		return { practiceSaved: true };
	},
};
