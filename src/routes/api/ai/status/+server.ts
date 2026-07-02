import { json } from "@sveltejs/kit";
import { loadLlmConfig } from "$lib/server/services/llm";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ locals }) => {
	if (!locals.user) {
		return json({ configured: false, authenticated: false }, { status: 401 });
	}

	const config = loadLlmConfig();
	return json({
		authenticated: true,
		configured: Boolean(config),
		model: config?.model ?? null,
		baseUrl: config?.baseUrl ?? null,
	});
};
