import { z } from "zod";

export const languageProfileSchema = z.enum(["generic", "japanese"]);
export type LanguageProfile = z.infer<typeof languageProfileSchema>;

export const cardKindSchema = z.enum([
	"recognition",
	"recall",
	"reading_recognition",
	"sentence_order",
	"sentence_cloze",
]);

export type CardKind = z.infer<typeof cardKindSchema>;

export const displayOptionsSchema = z.object({
	showTerm: z.boolean().optional(),
	showReading: z.boolean().optional(),
	showDefinition: z.boolean().optional(),
	showKanji: z.boolean().optional(),
	showHiragana: z.boolean().optional(),
	showFurigana: z.boolean().optional(),
	showEnglish: z.boolean().optional(),
	showExamples: z.boolean().optional(),
	cardTypes: z.array(cardKindSchema).optional(),
	activeCardTypes: z.array(cardKindSchema).optional(),
});

export type DisplayOptions = z.infer<typeof displayOptionsSchema>;

const genericCardTypes: CardKind[] = ["recognition", "recall"];
const japaneseCardTypes: CardKind[] = [
	"recognition",
	"recall",
	"reading_recognition",
];

export function defaultDisplayOptions(
	profile: LanguageProfile,
): Required<DisplayOptions> {
	if (profile === "japanese") {
		return {
			showTerm: true,
			showReading: true,
			showDefinition: true,
			showKanji: true,
			showHiragana: false,
			showFurigana: false,
			showEnglish: true,
			showExamples: true,
			cardTypes: japaneseCardTypes,
			activeCardTypes: ["recognition", "recall"],
		};
	}

	return {
		showTerm: true,
		showReading: true,
		showDefinition: true,
		showKanji: true,
		showHiragana: false,
		showFurigana: false,
		showEnglish: true,
		showExamples: true,
		cardTypes: genericCardTypes,
		activeCardTypes: genericCardTypes,
	};
}

export function inferLanguageProfile(
	targetLanguage: string | null | undefined,
): LanguageProfile {
	const normalized = targetLanguage?.trim().toLowerCase();
	if (normalized && ["ja", "jpn", "japanese", "日本語"].includes(normalized)) {
		return "japanese";
	}
	return "generic";
}

export function normalizeLanguageProfile(
	profile: unknown,
	targetLanguage?: string | null,
): LanguageProfile {
	const parsed = languageProfileSchema.safeParse(profile);
	return parsed.success ? parsed.data : inferLanguageProfile(targetLanguage);
}

export function normalizeDisplayOptions(
	profile: LanguageProfile,
	options: unknown,
): Required<DisplayOptions> {
	const defaults = defaultDisplayOptions(profile);
	const parsed = displayOptionsSchema.safeParse(options);
	if (!parsed.success) return defaults;

	const activeCardTypes =
		parsed.data.activeCardTypes ?? defaults.activeCardTypes;
	const allowed = new Set(defaults.cardTypes);

	return {
		...defaults,
		...parsed.data,
		cardTypes: defaults.cardTypes,
		activeCardTypes: activeCardTypes.filter((kind) => allowed.has(kind)),
	};
}
