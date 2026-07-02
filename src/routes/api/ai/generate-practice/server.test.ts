import { beforeEach, describe, expect, it, vi } from "vitest";

const courseId = "11111111-1111-4111-8111-111111111111";

const mocks = vi.hoisted(() => ({
	assertAiRateLimit: vi.fn(),
	generatePracticePreview: vi.fn(),
	getCourseForUser: vi.fn(),
	getLearnedNotesForCourse: vi.fn(),
}));

vi.mock("$lib/server/db/client", () => ({ db: {} }));

vi.mock("$lib/server/services/ai/rate-limit", () => ({
	assertAiRateLimit: mocks.assertAiRateLimit,
}));

vi.mock("$lib/server/services/ai/generate", () => ({
	generatePracticePreview: mocks.generatePracticePreview,
}));

vi.mock("$lib/server/repositories/courses", () => ({
	getCourseForUser: mocks.getCourseForUser,
}));

vi.mock("$lib/server/repositories/notes", () => ({
	getLearnedNotesForCourse: mocks.getLearnedNotesForCourse,
}));

import { POST } from "./+server";

function makeEvent(body: unknown, userId: string | null = "user-1") {
	return {
		request: new Request("http://localhost/api/ai/generate-practice", {
			method: "POST",
			body: typeof body === "string" ? body : JSON.stringify(body),
		}),
		locals: userId ? { user: { id: userId } } : {},
	} as Parameters<typeof POST>[0];
}

describe("POST /api/ai/generate-practice", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mocks.getCourseForUser.mockResolvedValue({
			id: courseId,
			title: "Japanese Basics",
			sourceLanguage: "English",
			targetLanguage: "Japanese",
			languageProfile: "japanese",
		});
		mocks.getLearnedNotesForCourse.mockResolvedValue([
			{
				id: "note-1",
				term: "水",
				reading: "みず",
				definition: "water",
			},
		]);
		mocks.generatePracticePreview.mockResolvedValue({
			mode: "sentences",
			title: "Water sentences",
			sentences: [],
			dialogue: [],
		});
	});

	it("checks course ownership before generating practice", async () => {
		mocks.getCourseForUser.mockResolvedValue(null);

		const response = await POST(
			makeEvent({ courseId, mode: "sentences", selectedNoteIds: [] }),
		);

		expect(response.status).toBe(404);
		expect(mocks.assertAiRateLimit).not.toHaveBeenCalled();
		expect(mocks.generatePracticePreview).not.toHaveBeenCalled();
	});

	it("rejects invalid requests before consuming rate limit", async () => {
		const response = await POST(makeEvent({ courseId: "not-a-uuid" }));

		expect(response.status).toBe(400);
		expect(mocks.assertAiRateLimit).not.toHaveBeenCalled();
		expect(mocks.getCourseForUser).not.toHaveBeenCalled();
	});

	it("generates a preview from learned words only", async () => {
		const response = await POST(
			makeEvent({
				courseId,
				mode: "sentences",
				topic: "ordering drinks",
				selectedNoteIds: ["22222222-2222-4222-8222-222222222222"],
			}),
		);

		expect(response.status).toBe(200);
		expect(mocks.getCourseForUser).toHaveBeenCalledWith({}, courseId, "user-1");
		expect(mocks.assertAiRateLimit).toHaveBeenCalledWith(
			"user-1",
			"generate-practice",
		);
		expect(mocks.getLearnedNotesForCourse).toHaveBeenCalledWith(
			{},
			courseId,
			["22222222-2222-4222-8222-222222222222"],
		);
		expect(mocks.generatePracticePreview).toHaveBeenCalledWith({
			mode: "sentences",
			courseTitle: "Japanese Basics",
			sourceLanguage: "English",
			targetLanguage: "Japanese",
			languageProfile: "japanese",
			topic: "ordering drinks",
			knownWords: [{ term: "水", reading: "みず", definition: "water" }],
		});
	});
});
