import { error, redirect } from "@sveltejs/kit";
import { base } from "$app/paths";
import { noteInput } from "$lib/validation/note";
import { getCourseForUser } from "$lib/server/repositories/courses";
import { createNoteWithCards } from "$lib/server/repositories/notes";
import { db } from "$lib/server/db/client";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, locals }) => {
  if (!locals.user) throw error(401, "Unauthorized");
  const course = await getCourseForUser(db, params.id, locals.user.id);
  if (!course) throw error(404, "Course not found");
  return { course };
};

export const actions: Actions = {
  default: async ({ request, params, locals }) => {
    if (!locals.user) throw error(401, "Unauthorized");

    const course = await getCourseForUser(db, params.id, locals.user.id);
    if (!course) throw error(404, "Course not found");

    const formData = await request.formData();
    const tagsStr = (formData.get("tags") as string) || "";
    const tags = tagsStr
      .split(",")
      .map((t) => t.trim())
      .filter(Boolean);

    const data = {
      term: formData.get("term"),
      reading: formData.get("reading") || null,
      definition: formData.get("definition"),
      example: formData.get("example") || null,
      exampleTranslation: formData.get("exampleTranslation") || null,
      partOfSpeech: formData.get("partOfSpeech") || null,
      notes: formData.get("notes") || null,
      tags,
    };

    const parsed = noteInput.safeParse(data);
    if (!parsed.success) {
      return {
        errors: parsed.error.flatten().fieldErrors,
        values: { ...data, tags: tagsStr },
        course,
      };
    }

    await createNoteWithCards(db, course.id, parsed.data);
    throw redirect(303, `${base}/app/courses/${course.id}`);
  },
};
