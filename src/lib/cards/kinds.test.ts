import { describe, expect, it } from "vitest";
import { buildSentenceCards, buildVocabularyCards } from "./kinds";
import {
	defaultDisplayOptions,
	type DisplayOptions,
} from "$lib/language/profiles";

describe("card builders", () => {
	it("builds base vocabulary recognition and recall cards", () => {
		const cards = buildVocabularyCards(
			{ term: "食べる", reading: "たべる", definition: "to eat" },
			"generic",
			defaultDisplayOptions("generic"),
		);

		expect(cards.map((card) => card.kind)).toEqual(["recognition", "recall"]);
	});

	it("adds Japanese reading recognition when enabled and kanji exists", () => {
		const options: DisplayOptions = {
			...defaultDisplayOptions("japanese"),
			activeCardTypes: ["recognition", "recall", "reading_recognition"],
		};
		const cards = buildVocabularyCards(
			{ term: "食べる", reading: "たべる", definition: "to eat" },
			"japanese",
			options,
		);

		expect(cards.map((card) => card.kind)).toEqual([
			"recognition",
			"recall",
			"reading_recognition",
		]);
	});

	it("builds sentence order and cloze cards", () => {
		const cards = buildSentenceCards({
			target: "私は水を飲みます",
			translation: "I drink water.",
			blocks: ["私", "は", "水", "を", "飲みます"],
			clozeAnswer: "水",
		});

		expect(cards.map((card) => card.kind)).toEqual([
			"sentence_order",
			"sentence_cloze",
		]);
		expect(cards[0].extra).toMatchObject({ translation: "I drink water." });
		expect(cards[1].answer).toBe("水");
	});
});
