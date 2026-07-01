import { z } from "zod";

export const courseInput = z.object({
  title: z.string().trim().min(1).max(120),
  description: z.string().trim().max(2000).optional().nullable(),
  sourceLanguage: z.string().trim().max(80).optional().nullable(),
  targetLanguage: z.string().trim().max(80).optional().nullable()
});

export type CourseInput = z.infer<typeof courseInput>;
