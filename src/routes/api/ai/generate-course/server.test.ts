import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	assertAiRateLimit: vi.fn(),
	generateCoursePreview: vi.fn(),
}));

vi.mock("$lib/server/services/ai/rate-limit", () => ({
	assertAiRateLimit: mocks.assertAiRateLimit,
}));

vi.mock("$lib/server/services/ai/generate", () => ({
	generateCoursePreview: mocks.generateCoursePreview,
}));

import { POST } from "./+server";

function makeEvent(body: unknown, userId: string | null = "user-1") {
	return {
		request: new Request("http://localhost/api/ai/generate-course", {
			method: "POST",
			body: typeof body === "string" ? body : JSON.stringify(body),
		}),
		locals: userId ? { user: { id: userId } } : {},
	} as Parameters<typeof POST>[0];
}

async function readJson(response: Response) {
	return response.json() as Promise<Record<string, unknown>>;
}

describe("POST /api/ai/generate-course", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mocks.generateCoursePreview.mockResolvedValue({
			title: "Japanese Basics",
			description: null,
			sourceLanguage: "English",
			targetLanguage: "Japanese",
			languageProfile: "japanese",
			lessons: [],
		});
	});

	it("requires authentication", async () => {
		const response = await POST(makeEvent({ topic: "travel" }, null));

		expect(response.status).toBe(401);
		expect(mocks.assertAiRateLimit).not.toHaveBeenCalled();
		expect(mocks.generateCoursePreview).not.toHaveBeenCalled();
	});

	it("rejects invalid JSON without consuming rate limit", async () => {
		const response = await POST(makeEvent("{bad json"));

		expect(response.status).toBe(400);
		expect(mocks.assertAiRateLimit).not.toHaveBeenCalled();
		expect(mocks.generateCoursePreview).not.toHaveBeenCalled();
	});

	it("returns a validated preview without persisting it", async () => {
		const response = await POST(
			makeEvent({
				topic: "restaurant phrases",
				sourceLanguage: "English",
				targetLanguage: "Japanese",
				languageProfile: "japanese",
			}),
		);
		const payload = await readJson(response);

		expect(response.status).toBe(200);
		expect(payload.preview).toMatchObject({ title: "Japanese Basics" });
		expect(mocks.assertAiRateLimit).toHaveBeenCalledWith(
			"user-1",
			"generate-course",
		);
		expect(mocks.generateCoursePreview).toHaveBeenCalledWith({
			topic: "restaurant phrases",
			sourceLanguage: "English",
			targetLanguage: "Japanese",
			languageProfile: "japanese",
		});
	});
});
