import { eq, and, desc } from "drizzle-orm";
import { courses } from "$lib/server/db/schema";
import type { Database } from "$lib/server/db/client";
import type { CourseInput } from "$lib/validation/course";

export async function listCoursesForUser(db: Database, userId: string) {
  return db
    .select()
    .from(courses)
    .where(and(eq(courses.ownerId, userId), eq(courses.isArchived, false)))
    .orderBy(desc(courses.updatedAt));
}

export async function getCourseForUser(
  db: Database,
  courseId: string,
  userId: string,
) {
  const [course] = await db
    .select()
    .from(courses)
    .where(and(eq(courses.id, courseId), eq(courses.ownerId, userId)))
    .limit(1);
  return course ?? null;
}

export async function createCourseForUser(
  db: Database,
  userId: string,
  input: CourseInput,
) {
  const [course] = await db
    .insert(courses)
    .values({
      ownerId: userId,
      title: input.title,
      description: input.description ?? null,
      sourceLanguage: input.sourceLanguage ?? null,
      targetLanguage: input.targetLanguage ?? null,
    })
    .returning();
  return course;
}

export async function updateCourse(
  db: Database,
  courseId: string,
  userId: string,
  input: Partial<CourseInput & { isArchived: boolean }>,
) {
  const [course] = await db
    .update(courses)
    .set({
      ...(input.title !== undefined && { title: input.title }),
      ...(input.description !== undefined && { description: input.description }),
      ...(input.sourceLanguage !== undefined && { sourceLanguage: input.sourceLanguage }),
      ...(input.targetLanguage !== undefined && { targetLanguage: input.targetLanguage }),
      ...(input.isArchived !== undefined && { isArchived: input.isArchived }),
      updatedAt: new Date(),
    })
    .where(and(eq(courses.id, courseId), eq(courses.ownerId, userId)))
    .returning();
  return course ?? null;
}
