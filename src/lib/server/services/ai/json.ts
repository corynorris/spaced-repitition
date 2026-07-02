import { z } from "zod";

export function extractJsonObject(content: string): unknown {
	const stripped = content
		.trim()
		.replace(/^```(?:json)?\s*/i, "")
		.replace(/\s*```$/i, "");

	const firstBrace = stripped.indexOf("{");
	if (firstBrace === -1) {
		throw new Error("No JSON object found");
	}

	let depth = 0;
	let inString = false;
	let escaped = false;

	for (let index = firstBrace; index < stripped.length; index++) {
		const char = stripped[index];

		if (inString) {
			if (escaped) {
				escaped = false;
			} else if (char === "\\") {
				escaped = true;
			} else if (char === '"') {
				inString = false;
			}
			continue;
		}

		if (char === '"') {
			inString = true;
		} else if (char === "{") {
			depth++;
		} else if (char === "}") {
			depth--;
			if (depth === 0) {
				return JSON.parse(stripped.slice(firstBrace, index + 1));
			}
		}
	}

	throw new Error("Incomplete JSON object");
}

export function summarizeZodError(error: z.ZodError): string {
	return error.issues
		.slice(0, 8)
		.map((issue) => `${issue.path.join(".") || "root"}: ${issue.message}`)
		.join("; ");
}
