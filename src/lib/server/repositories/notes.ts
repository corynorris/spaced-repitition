import { eq, and, like, or, desc, lte } from "drizzle-orm";
import { notes, cards, reviewStates } from "$lib/server/db/schema";
import type { Database } from "$lib/server/db/client";
import type { NoteInput } from "$lib/validation/note";

export async function listNotesForCourse(db: Database, courseId: string) {
  return db
    .select()
    .from(notes)
    .where(eq(notes.courseId, courseId))
    .orderBy(desc(notes.createdAt));
}

export async function getNoteForCourse(
  db: Database,
  noteId: string,
  courseId: string,
) {
  const [note] = await db
    .select()
    .from(notes)
    .where(and(eq(notes.id, noteId), eq(notes.courseId, courseId)))
    .limit(1);
  return note ?? null;
}

export async function searchNotes(
  db: Database,
  courseId: string,
  query: string,
) {
  const pattern = `%${query}%`;
  return db
    .select()
    .from(notes)
    .where(
      and(
        eq(notes.courseId, courseId),
        or(like(notes.term, pattern), like(notes.definition, pattern)),
      ),
    )
    .orderBy(desc(notes.createdAt))
    .limit(20);
}

/** Create a note and generate two cards (recognition + recall). */
export async function createNoteWithCards(
  db: Database,
  courseId: string,
  input: NoteInput,
) {
  return db.transaction(async (tx) => {
    const [note] = await tx
      .insert(notes)
      .values({
        courseId,
        term: input.term,
        reading: input.reading ?? null,
        definition: input.definition,
        example: input.example ?? null,
        exampleTranslation: input.exampleTranslation ?? null,
        partOfSpeech: input.partOfSpeech ?? null,
        notes: input.notes ?? null,
        tags: input.tags,
      })
      .returning();

    // Generate two cards per note
    const cardDefs: Array<{
      noteId: string;
      kind: string;
      prompt: string;
      answer: string;
    }> = [
      {
        noteId: note.id,
        kind: "recognition",
        prompt: note.term,
        answer: note.definition,
      },
      {
        noteId: note.id,
        kind: "recall",
        prompt: note.definition,
        answer: note.term,
      },
    ];

    const createdCards = await tx.insert(cards).values(cardDefs).returning();

    // Create initial review states for each card
    if (createdCards.length > 0) {
      await tx.insert(reviewStates).values(
        createdCards.map((c) => ({
          cardId: c.id,
          state: "new" as const,
        })),
      );
    }

    return { note, cards: createdCards };
  });
}

/** Get cards for a note with their review states. */
export async function getCardsForNote(db: Database, noteId: string) {
  return db
    .select()
    .from(cards)
    .where(eq(cards.noteId, noteId));
}

/** Get all cards for a course with their review states. */
export async function getCardsWithStatesForCourse(
  db: Database,
  courseId: string,
) {
  return db
    .select({
      card: cards,
      reviewState: reviewStates,
    })
    .from(cards)
    .innerJoin(notes, eq(cards.noteId, notes.id))
    .leftJoin(reviewStates, eq(cards.id, reviewStates.cardId))
    .where(eq(notes.courseId, courseId));
}

/** Get due cards for review in a course. */
export async function getDueCardsForCourse(db: Database, courseId: string) {
  const now = new Date();
  return db
    .select({
      card: cards,
      note: notes,
      reviewState: reviewStates,
    })
    .from(cards)
    .innerJoin(notes, eq(cards.noteId, notes.id))
    .innerJoin(reviewStates, eq(cards.id, reviewStates.cardId))
    .where(
      and(
        eq(notes.courseId, courseId),
        eq(cards.isSuspended, false),
        // Due if dueAt <= now
        lte(reviewStates.dueAt, now),
      ),
    )
    .orderBy(reviewStates.dueAt)
    .limit(20);
}
