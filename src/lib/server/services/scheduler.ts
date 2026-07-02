import {
  createEmptyCard,
  fsrs,
  Rating,
  type Card,
  type Grade,
  type RecordLogItem,
} from "ts-fsrs";
import { and, eq } from "drizzle-orm";
import { reviewStates, reviewLogs, cards } from "$lib/server/db/schema";
import type { Database } from "$lib/server/db/client";
import type { ReviewRatingInput } from "$lib/validation/review";

const scheduler = fsrs();

function ratingToFsrs(rating: ReviewRatingInput): Rating {
  switch (rating) {
    case "again":
      return Rating.Again;
    case "hard":
      return Rating.Hard;
    case "good":
      return Rating.Good;
    case "easy":
      return Rating.Easy;
  }
}

function dbStateToCard(state: typeof reviewStates.$inferSelect): Card {
  return {
    due: state.dueAt,
    stability: state.stability ?? 0,
    difficulty: state.difficulty ?? 0,
    elapsed_days: state.elapsedDays,
    scheduled_days: state.scheduledDays,
    reps: state.reps,
    lapses: state.lapses,
    state: cardStateToFsrs(state.state),
    last_review: state.updatedAt,
    learning_steps: 0,
  };
}

function cardStateToFsrs(
  state: "new" | "learning" | "review" | "relearning" | "suspended",
): 0 | 1 | 2 | 3 {
  switch (state) {
    case "new":
      return 0; // New
    case "learning":
      return 1; // Learning
    case "review":
      return 2; // Review
    case "relearning":
      return 3; // Relearning
    case "suspended":
      return 2; // Fallback: treat suspended as review
  }
}

function fsrsStateToDb(
  state: 0 | 1 | 2 | 3,
): "new" | "learning" | "review" | "relearning" {
  switch (state) {
    case 0:
      return "new";
    case 1:
      return "learning";
    case 2:
      return "review";
    case 3:
      return "relearning";
  }
}

/** Process a review for a card and return the updated review state. */
export async function processReview(
  db: Database,
  cardId: string,
  rating: ReviewRatingInput,
) {
  const [state] = await db
    .select()
    .from(reviewStates)
    .where(eq(reviewStates.cardId, cardId))
    .limit(1);

  if (!state) {
    // Initialize a new review state if missing
    const card = createEmptyCard(new Date());
    const fsrsRating = ratingToFsrs(rating);
    const result: RecordLogItem = scheduler.repeat(card, new Date())[fsrsRating as Grade];

    const [newState] = await db
      .insert(reviewStates)
      .values({
        cardId,
        state: fsrsStateToDb(result.card.state),
        dueAt: result.card.due,
        stability: result.card.stability,
        difficulty: result.card.difficulty,
        elapsedDays: result.card.elapsed_days,
        scheduledDays: result.card.scheduled_days,
        reps: result.card.reps,
        lapses: result.card.lapses,
        updatedAt: new Date(),
      })
      .returning();

    await db.insert(reviewLogs).values({
      cardId,
      rating,
      previousState: {},
      nextState: {
        state: result.card.state,
        due: result.card.due.toISOString(),
      },
    });

    return newState;
  }

  const card = dbStateToCard(state);
  const fsrsRating = ratingToFsrs(rating);
  const result: RecordLogItem = scheduler.repeat(card, new Date())[fsrsRating as Grade];

  const [updated] = await db
    .update(reviewStates)
    .set({
      state: fsrsStateToDb(result.card.state),
      dueAt: result.card.due,
      stability: result.card.stability,
      difficulty: result.card.difficulty,
      elapsedDays: result.card.elapsed_days,
      scheduledDays: result.card.scheduled_days,
      reps: result.card.reps,
      lapses: result.card.lapses,
      updatedAt: new Date(),
    })
    .where(eq(reviewStates.cardId, cardId))
    .returning();

  await db.insert(reviewLogs).values({
    cardId,
    rating,
    previousState: {
      state: card.state,
      due: card.due.toISOString(),
      stability: card.stability,
      difficulty: card.difficulty,
    },
    nextState: {
      state: result.card.state,
      due: result.card.due.toISOString(),
    },
  });

  return updated;
}

import { sql } from "drizzle-orm";

/** Get review session stats for a course using raw SQL for simplicity. */
export async function getReviewStats(db: Database, courseId: string) {
  const [due] = await db.execute<{ count: number }>(
    sql`SELECT COUNT(*)::int as count FROM card c
     INNER JOIN note n ON c.note_id = n.id
     INNER JOIN review_state rs ON c.id = rs.card_id
     WHERE n.course_id = ${courseId} AND c.is_suspended = false AND rs.due_at <= NOW()`,
  );

  const [total] = await db.execute<{ count: number }>(
    sql`SELECT COUNT(*)::int as count FROM card c
     INNER JOIN note n ON c.note_id = n.id
     WHERE n.course_id = ${courseId}`,
  );

  const [today] = await db.execute<{ count: number }>(
    sql`SELECT COUNT(DISTINCT rl.card_id)::int as count FROM review_log rl
     INNER JOIN card c ON rl.card_id = c.id
     INNER JOIN note n ON c.note_id = n.id
     WHERE n.course_id = ${courseId} AND rl.reviewed_at >= CURRENT_DATE`,
  );

  return {
    dueCards: due?.count ?? 0,
    totalCards: total?.count ?? 0,
    reviewedToday: today?.count ?? 0,
  };
}
