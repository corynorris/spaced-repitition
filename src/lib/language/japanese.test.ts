import { describe, expect, it } from "vitest";
import {
	containsKanji,
	normalizeJapaneseAnswer,
	parseFuriganaMarkup,
} from "./japanese";

describe("Japanese language utilities", () => {
	it("parses bracket furigana markup into safe tokens", () => {
		expect(parseFuriganaMarkup("今日[きょう]は日本語[にほんご]")).toEqual([
			{ type: "ruby", base: "今日", reading: "きょう" },
			{ type: "text", text: "は" },
			{ type: "ruby", base: "日本語", reading: "にほんご" },
		]);
	});

	it("detects kanji", () => {
		expect(containsKanji("食べる")).toBe(true);
		expect(containsKanji("たべる")).toBe(false);
	});

	it("normalizes spacing, punctuation, and width for answers", () => {
		expect(normalizeJapaneseAnswer(" ｶﾀｶﾅ 。 ")).toBe("カタカナ");
		expect(normalizeJapaneseAnswer("日 本 語")).toBe("日本語");
	});
});
