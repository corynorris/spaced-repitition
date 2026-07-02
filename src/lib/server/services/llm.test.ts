import { describe, expect, it } from "vitest";
import { normalizeLlmBaseUrl } from "./llm";

describe("LLM client helpers", () => {
	it("normalizes base URLs without duplicating /v1", () => {
		expect(normalizeLlmBaseUrl("http://localhost:8080/v1")).toBe(
			"http://localhost:8080",
		);
		expect(normalizeLlmBaseUrl("http://localhost:8080/v1/")).toBe(
			"http://localhost:8080",
		);
		expect(normalizeLlmBaseUrl("http://localhost:8080/")).toBe(
			"http://localhost:8080",
		);
	});
});
