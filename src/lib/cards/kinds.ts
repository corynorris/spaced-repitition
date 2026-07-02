import { containsKanji } from "$lib/language/japanese";
import type {
	CardKind,
	DisplayOptions,
	LanguageProfile,
} from "$lib/language/profiles";

export const V1_CARD_KINDS: CardKind[] = [
	"recognition",
	"recall",
	"reading_recognition",
	"sentence_order",
	"sentence_cloze",
];

export const POST_V1_CARD_KINDS = [
	"typing",
	"multiple_choice",
	"listening",
] as const;

export interface CardDefinition {
	kind: CardKind;
	prompt: string;
	answer: string;
	extra?: Record<string, unknown>;
}

export interface CardBuildNote {
	term: string;
	reading?: string | null;
	definition: string;
}

export function buildVocabularyCards(
	note: CardBuildNote,
	profile: LanguageProfile,
	options: DisplayOptions,
): CardDefinition[] {
	const active = new Set(options.activeCardTypes ?? ["recognition", "recall"]);
	const cards: CardDefinition[] = [];

	if (active.has("recognition")) {
		cards.push({
			kind: "recognition",
			prompt: note.term,
			answer: note.definition,
		});
	}

	if (active.has("recall")) {
		cards.push({
			kind: "recall",
			prompt: note.definition,
			answer: note.term,
		});
	}

	if (
		profile === "japanese" &&
		active.has("reading_recognition") &&
		note.reading &&
		containsKanji(note.term)
	) {
		cards.push({
			kind: "reading_recognition",
			prompt: note.term,
			answer: note.reading,
		});
	}

	return cards;
}

export function buildSentenceCards(sentence: {
	target: string;
	translation: string;
	blocks?: string[];
	clozeAnswer?: string | null;
}): CardDefinition[] {
	const blocks = sentence.blocks?.length
		? sentence.blocks
		: sentence.target.split(/\s+/).filter(Boolean);

	return [
		{
			kind: "sentence_order",
			prompt: sentence.translation,
			answer: sentence.target,
			extra: {
				word_blocks: blocks,
				correct_order: blocks.map((_, index) => index),
				translation: sentence.translation,
			},
		},
		{
			kind: "sentence_cloze",
			prompt: sentence.target,
			answer: sentence.clozeAnswer ?? blocks[0] ?? sentence.target,
			extra: {
				sentence: sentence.target,
				translation: sentence.translation,
			},
		},
	];
}

export function labelForCardKind(kind: string): string {
	switch (kind) {
		case "recognition":
			return "Recognition";
		case "recall":
			return "Recall";
		case "reading_recognition":
			return "Reading";
		case "sentence_order":
			return "Sentence order";
		case "sentence_cloze":
			return "Cloze";
		default:
			return kind;
	}
}
