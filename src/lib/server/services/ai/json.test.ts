import { describe, expect, it } from "vitest";
import { aiCoursePreviewSchema } from "./schemas";
import { extractJsonObject } from "./json";

describe("AI JSON helpers", () => {
	it("extracts the first complete JSON object from markdown", () => {
		expect(extractJsonObject('```json\n{"a":{"b":1}}\n```\nextra')).toEqual({
			a: { b: 1 },
		});
	});

	it("preserves braces inside strings while extracting JSON", () => {
		expect(
			extractJsonObject('prefix {"text":"use { braces } here"} suffix'),
		).toEqual({
			text: "use { braces } here",
		});
	});

	it("rejects invalid AI course previews with Zod", () => {
		const parsed = aiCoursePreviewSchema.safeParse({
			title: "",
			lessons: [],
		});

		expect(parsed.success).toBe(false);
	});
});
