import { beforeEach, describe, expect, it, vi } from "vitest";

const courseId = "11111111-1111-4111-8111-111111111111";

const mocks = vi.hoisted(() => ({
	assertAiRateLimit: vi.fn(),
	discussContent: vi.fn(),
	getCourseForUser: vi.fn(),
	listNotesForCourse: vi.fn(),
}));

vi.mock("$lib/server/db/client", () => ({ db: {} }));

vi.mock("$lib/server/services/ai/rate-limit", () => ({
	assertAiRateLimit: mocks.assertAiRateLimit,
}));

vi.mock("$lib/server/services/ai/generate", () => ({
	discussContent: mocks.discussContent,
}));

vi.mock("$lib/server/repositories/courses", () => ({
	getCourseForUser: mocks.getCourseForUser,
}));

vi.mock("$lib/server/repositories/notes", () => ({
	listNotesForCourse: mocks.listNotesForCourse,
}));

import { POST } from "./+server";

function makeEvent(body: unknown, userId: string | null = "user-1") {
	return {
		request: new Request("http://localhost/api/ai/discuss-content", {
			method: "POST",
			body: typeof body === "string" ? body : JSON.stringify(body),
		}),
		locals: userId ? { user: { id: userId } } : {},
	} as Parameters<typeof POST>[0];
}

describe("POST /api/ai/discuss-content", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mocks.getCourseForUser.mockResolvedValue({
			id: courseId,
			title: "Japanese Basics",
			sourceLanguage: "English",
			targetLanguage: "Japanese",
		});
		mocks.listNotesForCourse.mockResolvedValue([
			{
				id: "22222222-2222-4222-8222-222222222222",
				term: "水",
				reading: "みず",
				definition: "water",
			},
			{
				id: "33333333-3333-4333-8333-333333333333",
				term: "食べる",
				reading: "たべる",
				definition: "to eat",
			},
		]);
		mocks.discussContent.mockResolvedValue("Use 水 for plain water.");
	});

	it("checks ownership before discussing course content", async () => {
		mocks.getCourseForUser.mockResolvedValue(null);

		const response = await POST(
			makeEvent({ courseId, question: "What does this mean?" }),
		);

		expect(response.status).toBe(404);
		expect(mocks.assertAiRateLimit).not.toHaveBeenCalled();
		expect(mocks.discussContent).not.toHaveBeenCalled();
	});

	it("rejects malformed JSON without consuming rate limit", async () => {
		const response = await POST(makeEvent("{bad json"));

		expect(response.status).toBe(400);
		expect(mocks.assertAiRateLimit).not.toHaveBeenCalled();
		expect(mocks.getCourseForUser).not.toHaveBeenCalled();
	});

	it("sends selected note context to the read-only discussion service", async () => {
		const response = await POST(
			makeEvent({
				courseId,
				question: "How do I use this word?",
				scope: "note",
				noteIds: ["33333333-3333-4333-8333-333333333333"],
				content: "Current card",
			}),
		);
		const payload = await response.json();

		expect(response.status).toBe(200);
		expect(payload).toEqual({ answer: "Use 水 for plain water." });
		expect(mocks.assertAiRateLimit).toHaveBeenCalledWith(
			"user-1",
			"discuss-content",
		);
		expect(mocks.discussContent).toHaveBeenCalledWith({
			courseTitle: "Japanese Basics",
			sourceLanguage: "English",
			targetLanguage: "Japanese",
			context: "scope: note\ncontent: Current card\n食べる (たべる): to eat",
			question: "How do I use this word?",
		});
	});
});
