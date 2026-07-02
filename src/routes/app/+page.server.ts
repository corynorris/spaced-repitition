import { listCoursesForUser } from "$lib/server/repositories/courses";
import { getReviewStats } from "$lib/server/services/scheduler";
import { db } from "$lib/server/db/client";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.user) return { courses: [] };

  const courses = await listCoursesForUser(db, locals.user.id);

  // Attach review stats to each course
  const withStats = await Promise.all(
    courses.map(async (course) => {
      const stats = await getReviewStats(db, course.id);
      return { ...course, stats };
    }),
  );

  return { courses: withStats };
};
