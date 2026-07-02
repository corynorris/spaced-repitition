import { logger } from "$lib/server/logger";

export interface LlmMessage {
	role: "system" | "user" | "assistant";
	content: string;
}

export interface LlmRequest {
	model: string;
	messages: LlmMessage[];
	temperature: number;
	max_tokens: number;
}

export interface LlmResponse {
	content: string;
	model: string;
}

export interface LlmConfig {
	baseUrl: string;
	apiKey: string;
	model: string;
	timeoutMs: number;
}

export interface LlmRetryOptions {
	attempts?: number;
	baseDelayMs?: number;
	onRetry?: (attempt: number, error: Error) => void;
}

export class LlmError extends Error {
	constructor(
		message: string,
		public statusCode?: number,
		public body?: string,
	) {
		super(message);
		this.name = "LlmError";
	}
}

export function normalizeLlmBaseUrl(baseUrl: string): string {
	return baseUrl.replace(/\/+$/, "").replace(/\/v1$/, "");
}

export function loadLlmConfig(): LlmConfig | null {
	const baseUrl = process.env.LLM_BASE_URL?.trim();
	const model = process.env.LLM_MODEL?.trim();
	if (!baseUrl || !model) return null;

	return {
		baseUrl: normalizeLlmBaseUrl(baseUrl),
		model,
		apiKey: process.env.LLM_API_KEY ?? "",
		timeoutMs: Number(process.env.LLM_TIMEOUT_MS ?? 60000),
	};
}

export async function callLlm(
	config: LlmConfig,
	request: LlmRequest,
): Promise<LlmResponse> {
	const url = `${normalizeLlmBaseUrl(config.baseUrl)}/v1/chat/completions`;
	const controller = new AbortController();
	const timeout = setTimeout(() => controller.abort(), config.timeoutMs);

	let response: Response;
	try {
		const headers: Record<string, string> = {
			"Content-Type": "application/json",
		};
		if (config.apiKey) headers.Authorization = `Bearer ${config.apiKey}`;

		response = await fetch(url, {
			method: "POST",
			headers,
			body: JSON.stringify({
				model: request.model,
				messages: request.messages,
				temperature: request.temperature,
				max_tokens: request.max_tokens,
				stream: false,
			}),
			signal: controller.signal,
		});
	} catch (err) {
		clearTimeout(timeout);
		if (err instanceof Error && err.name === "AbortError") {
			throw new LlmError(`LLM request timed out after ${config.timeoutMs}ms`);
		}
		throw new LlmError(
			`LLM request failed: ${err instanceof Error ? err.message : String(err)}`,
		);
	}
	clearTimeout(timeout);

	if (!response.ok) {
		const body = await response.text().catch(() => "");
		throw new LlmError(
			`LLM returned status ${response.status}`,
			response.status,
			body,
		);
	}

	const data = await response.json();
	const msg = data?.choices?.[0]?.message;
	let content: string | undefined = msg?.content;
	if (typeof content !== "string" || content.length === 0) {
		content = msg?.reasoning_content;
	}
	if (typeof content !== "string" || content.length === 0) {
		throw new LlmError("LLM returned empty or missing content");
	}

	return {
		content,
		model: data.model ?? request.model,
	};
}

function isRetryableLlmError(err: unknown): boolean {
	if (!(err instanceof LlmError)) return false;
	if (err.statusCode === undefined) return true;
	return (
		err.statusCode === 408 || err.statusCode === 429 || err.statusCode >= 500
	);
}

function wait(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function callLlmWithRetry(
	config: LlmConfig,
	request: LlmRequest,
	options: LlmRetryOptions = {},
): Promise<LlmResponse & { attempts: number }> {
	const attempts = Math.max(1, options.attempts ?? 2);
	const baseDelayMs = options.baseDelayMs ?? 500;
	let lastError: unknown;

	for (let attempt = 1; attempt <= attempts; attempt++) {
		const startedAt = Date.now();
		try {
			const response = await callLlm(config, request);
			const latencyMs = Date.now() - startedAt;
			logger.aiCall({
				model: response.model,
				latencyMs,
				attempt,
				success: true,
			});
			return { ...response, attempts: attempt };
		} catch (err) {
			const latencyMs = Date.now() - startedAt;
			const errorCode =
				err instanceof LlmError ? `status_${err.statusCode ?? "network"}` : "unknown";
			logger.aiCall({
				model: config.model,
				latencyMs,
				attempt,
				success: false,
				errorCode,
			});
			lastError = err;
			if (attempt >= attempts || !isRetryableLlmError(err)) throw err;
			const error = err instanceof Error ? err : new Error(String(err));
			options.onRetry?.(attempt, error);
			await wait(baseDelayMs * 2 ** (attempt - 1));
		}
	}

	throw lastError instanceof Error
		? lastError
		: new LlmError(String(lastError));
}
