import { z } from "zod";

export const reviewRatingInput = z.enum(["again", "hard", "good", "easy"]);
export type ReviewRatingInput = z.infer<typeof reviewRatingInput>;
