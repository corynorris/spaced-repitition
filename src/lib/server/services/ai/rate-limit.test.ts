import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

describe("AI rate limiting", () => {
	beforeEach(() => {
		vi.resetModules();
		vi.stubEnv("AI_RATE_LIMIT_PER_USER", "2");
		vi.stubEnv("AI_RATE_LIMIT_WINDOW_MS", "1000");
		vi.useFakeTimers();
		vi.setSystemTime(new Date("2026-01-01T00:00:00Z"));
	});

	afterEach(() => {
		vi.useRealTimers();
		vi.unstubAllEnvs();
	});

	it("limits requests per user and scope", async () => {
		const { assertAiRateLimit } = await import("./rate-limit");

		expect(() => assertAiRateLimit("user-1", "generate-course")).not.toThrow();
		expect(() => assertAiRateLimit("user-1", "generate-course")).not.toThrow();
		expect(() => assertAiRateLimit("user-1", "generate-course")).toThrow(
			/Rate limit exceeded/,
		);
	});

	it("uses independent buckets for different scopes", async () => {
		const { assertAiRateLimit } = await import("./rate-limit");

		expect(() => assertAiRateLimit("user-1", "generate-course")).not.toThrow();
		expect(() => assertAiRateLimit("user-1", "generate-practice")).not.toThrow();
		expect(() => assertAiRateLimit("user-1", "generate-practice")).not.toThrow();
		expect(() => assertAiRateLimit("user-1", "generate-practice")).toThrow(
			/Rate limit exceeded/,
		);
	});

	it("resets the bucket after the window expires", async () => {
		const { assertAiRateLimit } = await import("./rate-limit");

		assertAiRateLimit("user-1", "discuss-content");
		assertAiRateLimit("user-1", "discuss-content");

		vi.advanceTimersByTime(1000);

		expect(() => assertAiRateLimit("user-1", "discuss-content")).not.toThrow();
	});
});
