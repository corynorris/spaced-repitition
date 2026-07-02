/**
 * Structured JSON logger for observability.
 * Logs to stdout with timestamp, level, and context.
 * Designed to be machine-parseable without logging full user prompts.
 */

type LogLevel = "info" | "warn" | "error";

interface LogEntry {
	ts: string;
	level: LogLevel;
	msg: string;
	ctx?: Record<string, unknown>;
}

function emit(entry: LogEntry): void {
	// Use stdout so container orchestrators pick it up naturally.
	// biome-ignore lint/suspicious/noConsole: structured logger
	console.log(JSON.stringify(entry));
}

function now(): string {
	return new Date().toISOString();
}

function fmt(entry: LogEntry): string {
	// biome-ignore lint/suspicious/noConsole: not using console here
	return `${entry.ts} [${entry.level.toUpperCase()}] ${entry.msg}${entry.ctx ? " " + JSON.stringify(entry.ctx) : ""}`;
}

export const logger = {
	info(msg: string, ctx?: Record<string, unknown>) {
		emit({ ts: now(), level: "info", msg, ctx });
	},

	warn(msg: string, ctx?: Record<string, unknown>) {
		emit({ ts: now(), level: "warn", msg, ctx });
	},

	error(msg: string, ctx?: Record<string, unknown>) {
		emit({ ts: now(), level: "error", msg, ctx });
	},

	/** Log an AI model call. Never logs the full prompt. */
	aiCall(params: {
		model: string;
		latencyMs: number;
		attempt: number;
		success: boolean;
		tokenCount?: number;
		errorCode?: string;
		userId?: string;
	}) {
		this.info("ai_call", {
			model: params.model,
			latency_ms: params.latencyMs,
			attempt: params.attempt,
			success: params.success,
			token_count: params.tokenCount,
			error_code: params.errorCode,
			uid: params.userId,
		});
	},

	/** Log a rate-limit denial (no user prompt data). */
	rateLimitDenied(params: {
		userId: string;
		scope: string;
		windowMs: number;
	}) {
		this.warn("rate_limit_denied", {
			uid: params.userId,
			scope: params.scope,
			window_ms: params.windowMs,
		});
	},

	/** Log an AI validation failure. */
	aiValidationFailed(params: {
		model: string;
		reason: string;
		userId?: string;
	}) {
		this.warn("ai_validation_failed", {
			model: params.model,
			reason: params.reason,
			uid: params.userId,
		});
	},
} as const;

export { fmt };
