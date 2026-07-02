import { z } from "zod";

export const noteInput = z.object({
	term: z.string().trim().min(1).max(250),
	reading: z.string().trim().max(250).optional().nullable(),
	furigana: z.string().trim().max(1000).optional().nullable(),
	definition: z.string().trim().min(1).max(2000),
	example: z.string().trim().max(2000).optional().nullable(),
	exampleTranslation: z.string().trim().max(2000).optional().nullable(),
	partOfSpeech: z.string().trim().max(80).optional().nullable(),
	notes: z.string().trim().max(5000).optional().nullable(),
	tags: z.array(z.string().trim().min(1).max(40)).max(20).default([]),
});

export type NoteInput = z.infer<typeof noteInput>;
