import { error } from "@sveltejs/kit";
import { getCourseForUser } from "$lib/server/repositories/courses";
import { getDueCardsForCourse } from "$lib/server/repositories/notes";
import { processReview } from "$lib/server/services/scheduler";
import { reviewRatingInput } from "$lib/validation/review";
import { db } from "$lib/server/db/client";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, locals }) => {
  if (!locals.user) throw error(401, "Unauthorized");

  const course = await getCourseForUser(db, params.id, locals.user.id);
  if (!course) throw error(404, "Course not found");

  const dueCards = await getDueCardsForCourse(db, course.id);

  return { course, dueCards };
};

export const actions: Actions = {
  rate: async ({ request, params, locals }) => {
    if (!locals.user) throw error(401, "Unauthorized");

    const course = await getCourseForUser(db, params.id, locals.user.id);
    if (!course) throw error(404, "Course not found");

    const formData = await request.formData();
    const cardId = formData.get("cardId") as string;
    const rating = formData.get("rating") as string;

    const parsed = reviewRatingInput.safeParse(rating);
    if (!parsed.success || !cardId) {
      return { success: false };
    }

    await processReview(db, cardId, parsed.data);
    return { success: true };
  },
};
