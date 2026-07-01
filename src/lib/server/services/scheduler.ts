import type { ReviewRatingInput } from "$lib/validation/review";

export interface ReviewCardState {
  dueAt: Date;
  stability: number | null;
  difficulty: number | null;
  elapsedDays: number;
  scheduledDays: number;
  reps: number;
  lapses: number;
  state: "new" | "learning" | "review" | "relearning" | "suspended";
}

export function scheduleReview(_state: ReviewCardState, _rating: ReviewRatingInput) {
  throw new Error("FSRS scheduling will be implemented with ts-fsrs.");
}
