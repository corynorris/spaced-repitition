import { error } from "@sveltejs/kit";
import { getCourseForUser } from "$lib/server/repositories/courses";
import { listNotesForCourse } from "$lib/server/repositories/notes";
import { getReviewStats } from "$lib/server/services/scheduler";
import { db } from "$lib/server/db/client";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, locals }) => {
  if (!locals.user) throw error(401, "Unauthorized");

  const course = await getCourseForUser(db, params.id, locals.user.id);
  if (!course) throw error(404, "Course not found");

  const [notes, stats] = await Promise.all([
    listNotesForCourse(db, course.id),
    getReviewStats(db, course.id),
  ]);

  return { course, notes, stats };
};
