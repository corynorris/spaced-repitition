import type { Database } from "$lib/server/db/client";
import type { CourseInput } from "$lib/validation/course";

export async function listCoursesForUser(_db: Database, _userId: string) {
  return [];
}

export async function createCourseForUser(
  _db: Database,
  _userId: string,
  _input: CourseInput
) {
  throw new Error("Course persistence will be implemented after auth is wired.");
}
