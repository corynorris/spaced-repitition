import type { DisplayOptions } from "./profiles";

export type FuriganaToken =
	| { type: "text"; text: string }
	| { type: "ruby"; base: string; reading: string };

const kanjiPattern = /[\u3400-\u4dbf\u4e00-\u9fff]/;

export function containsKanji(text: string): boolean {
	return kanjiPattern.test(text);
}

export function parseFuriganaMarkup(
	text: string | null | undefined,
): FuriganaToken[] {
	if (!text) return [];

	const tokens: FuriganaToken[] = [];
	let cursor = 0;
	const pattern = /([\u3400-\u4dbf\u4e00-\u9fff々ヶ]+)\[([^\[\]]+)\]/gu;
	let match: RegExpExecArray | null;

	while ((match = pattern.exec(text)) !== null) {
		if (match.index > cursor) {
			tokens.push({ type: "text", text: text.slice(cursor, match.index) });
		}
		tokens.push({ type: "ruby", base: match[1], reading: match[2] });
		cursor = match.index + match[0].length;
	}

	if (cursor < text.length) {
		tokens.push({ type: "text", text: text.slice(cursor) });
	}

	return tokens;
}

export function normalizeJapaneseAnswer(text: string): string {
	return text
		.normalize("NFKC")
		.replace(/[\s\u3000]+/g, "")
		.replace(/[。．.、，,]/g, "")
		.trim();
}

export interface JapaneseDisplayNote {
	kanji: string;
	hiragana: string;
	furigana: FuriganaToken[];
	english: string;
	visible: {
		kanji: boolean;
		hiragana: boolean;
		furigana: boolean;
		english: boolean;
	};
}

export function buildJapaneseDisplay(
	note: {
		term: string;
		reading: string | null;
		furigana: string | null;
		definition: string;
	},
	options: DisplayOptions,
): JapaneseDisplayNote {
	return {
		kanji: note.term,
		hiragana: note.reading ?? "",
		furigana: parseFuriganaMarkup(note.furigana ?? note.term),
		english: note.definition,
		visible: {
			kanji: options.showKanji ?? true,
			hiragana: options.showHiragana ?? false,
			furigana: options.showFurigana ?? false,
			english: options.showEnglish ?? true,
		},
	};
}
