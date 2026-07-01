import type { Database } from "$lib/server/db/client";
import type { NoteInput } from "$lib/validation/note";

export async function searchNotes(_db: Database, _courseId: string, _query: string) {
  return [];
}

export async function createNoteWithCards(
  _db: Database,
  _courseId: string,
  _input: NoteInput
) {
  throw new Error("Note persistence will be implemented with card generation.");
}
