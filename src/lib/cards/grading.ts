import { normalizeJapaneseAnswer } from "$lib/language/japanese";

/**
 * Check whether a user's word-block ordering matches the correct order.
 * Both arrays should contain the same indices for a valid comparison.
 */
export function gradeSentenceOrder(
	userOrder: number[],
	correctOrder: number[],
): boolean {
	if (userOrder.length !== correctOrder.length) return false;
	return userOrder.every((val, idx) => val === correctOrder[idx]);
}

/**
 * Grade a typed cloze answer against the expected answer.
 * For Japanese courses, normalizes both strings before comparing.
 */
export function gradeSentenceCloze(
	userAnswer: string,
	expectedAnswer: string,
	languageProfile?: string | null,
): boolean {
	if (languageProfile === "japanese") {
		return (
			normalizeJapaneseAnswer(userAnswer) ===
			normalizeJapaneseAnswer(expectedAnswer)
		);
	}
	return (
		userAnswer.trim().toLowerCase() === expectedAnswer.trim().toLowerCase()
	);
}

/** Card kinds that are auto-graded on the client side. */
const AUTO_GRADED_KINDS = new Set(["sentence_order", "sentence_cloze"]);

export function isAutoGradedCardKind(kind: string): boolean {
	return AUTO_GRADED_KINDS.has(kind);
}

/**
 * Produce a shuffled array of indices [0..length-1].
 * Used to randomize the display order of word blocks for sentence_order cards.
 */
export function shuffleIndices(length: number): number[] {
	const indices = Array.from({ length }, (_, i) => i);
	for (let i = indices.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[indices[i], indices[j]] = [indices[j], indices[i]];
	}
	return indices;
}

/**
 * Find where the cloze answer appears in the full sentence and return
 * the parts before, the answer, and after.  If the answer does not appear
 * verbatim the sentence is returned unchanged and answerParts is null.
 */
export function splitClozeSentence(
	sentence: string,
	answer: string,
): {
	before: string;
	answer: string;
	after: string;
	found: boolean;
} {
	const idx = sentence.indexOf(answer);
	if (idx === -1) {
		return { before: sentence, answer: "", after: "", found: false };
	}
	return {
		before: sentence.slice(0, idx),
		answer,
		after: sentence.slice(idx + answer.length),
		found: true,
	};
}
