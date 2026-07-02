import { describe, expect, it } from "vitest";
import {
	gradeSentenceCloze,
	gradeSentenceOrder,
	isAutoGradedCardKind,
	shuffleIndices,
	splitClozeSentence,
} from "./grading";

describe("grading helpers", () => {
	describe("gradeSentenceOrder", () => {
		it("returns true when user order matches correct order", () => {
			expect(gradeSentenceOrder([0, 1, 2, 3], [0, 1, 2, 3])).toBe(true);
		});

		it("returns false when user order differs", () => {
			expect(gradeSentenceOrder([1, 0, 2, 3], [0, 1, 2, 3])).toBe(false);
		});

		it("returns false when length differs", () => {
			expect(gradeSentenceOrder([0, 1, 2], [0, 1, 2, 3])).toBe(false);
		});

		it("returns true for empty arrays", () => {
			expect(gradeSentenceOrder([], [])).toBe(true);
		});
	});

	describe("gradeSentenceCloze", () => {
		it("matches exact answers for generic profile", () => {
			expect(gradeSentenceCloze("water", "water")).toBe(true);
			expect(gradeSentenceCloze("Water", "water")).toBe(true);
			expect(gradeSentenceCloze("  water  ", "water")).toBe(true);
			expect(gradeSentenceCloze("juice", "water")).toBe(false);
		});

		it("uses Japanese normalization for Japanese profile", () => {
			expect(
				gradeSentenceCloze("にほんご", "にほんご", "japanese"),
			).toBe(true);
			expect(
				gradeSentenceCloze("日 本 語", "日本語", "japanese"),
			).toBe(true);
			expect(
				gradeSentenceCloze("ｶﾀｶﾅ", "カタカナ", "japanese"),
			).toBe(true);
		});

		it("does not normalize for generic profile", () => {
			// Without Japanese normalization these differ
			expect(
				gradeSentenceCloze("日 本 語", "日本語", "generic"),
			).toBe(false);
		});
	});

	describe("isAutoGradedCardKind", () => {
		it("identifies sentence_order and sentence_cloze as auto-graded", () => {
			expect(isAutoGradedCardKind("sentence_order")).toBe(true);
			expect(isAutoGradedCardKind("sentence_cloze")).toBe(true);
		});

		it("returns false for manual-review kinds", () => {
			expect(isAutoGradedCardKind("recognition")).toBe(false);
			expect(isAutoGradedCardKind("recall")).toBe(false);
			expect(isAutoGradedCardKind("reading_recognition")).toBe(false);
		});

		it("returns false for unknown or post-v1 kinds", () => {
			expect(isAutoGradedCardKind("typing")).toBe(false);
			expect(isAutoGradedCardKind("multiple_choice")).toBe(false);
			expect(isAutoGradedCardKind("")).toBe(false);
		});
	});

	describe("shuffleIndices", () => {
		it("returns an array of the correct length", () => {
			expect(shuffleIndices(5)).toHaveLength(5);
		});

		it("contains every index exactly once", () => {
			const shuffled = shuffleIndices(6);
			expect(shuffled.sort((a, b) => a - b)).toEqual([0, 1, 2, 3, 4, 5]);
		});

		it("returns empty for length 0", () => {
			expect(shuffleIndices(0)).toEqual([]);
		});
	});

	describe("splitClozeSentence", () => {
		it("splits around the answer when found", () => {
			const result = splitClozeSentence(
				"私は水を飲みます",
				"水",
			);
			expect(result.found).toBe(true);
			expect(result.before).toBe("私は");
			expect(result.answer).toBe("水");
			expect(result.after).toBe("を飲みます");
		});

		it("returns found=false when answer is not in sentence", () => {
			const result = splitClozeSentence("hello world", "bonjour");
			expect(result.found).toBe(false);
			expect(result.before).toBe("hello world");
		});

		it("handles answer at sentence start", () => {
			const result = splitClozeSentence("水を飲みます", "水");
			expect(result.found).toBe(true);
			expect(result.before).toBe("");
			expect(result.after).toBe("を飲みます");
		});

		it("handles answer at sentence end", () => {
			const result = splitClozeSentence("私は水", "水");
			expect(result.found).toBe(true);
			expect(result.before).toBe("私は");
			expect(result.after).toBe("");
		});
	});
});
