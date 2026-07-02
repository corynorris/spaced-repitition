interface Bucket {
	count: number;
	resetAt: number;
}

const buckets = new Map<string, Bucket>();

export function assertAiRateLimit(userId: string, scope: string) {
	const limit = Number(process.env.AI_RATE_LIMIT_PER_USER ?? 5);
	const windowMs = Number(process.env.AI_RATE_LIMIT_WINDOW_MS ?? 60000);
	const now = Date.now();
	const key = `${userId}:${scope}`;
	const current = buckets.get(key);

	if (!current || current.resetAt <= now) {
		buckets.set(key, { count: 1, resetAt: now + windowMs });
		return;
	}

	if (current.count >= limit) {
		const retryAfter = Math.ceil((current.resetAt - now) / 1000);
		// Import lazily to avoid circular dependency concerns
		import("$lib/server/logger").then(
			({ logger }) =>
				logger.rateLimitDenied({ userId, scope, windowMs }),
			() => {},
		);
		const error = new Error(`Rate limit exceeded. Retry after ${retryAfter}s.`);
		error.name = "RateLimitError";
		throw error;
	}

	current.count++;
}
