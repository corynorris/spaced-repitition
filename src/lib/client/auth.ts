import { base } from "#lib/paths";
import { createAuthClient } from "better-auth/svelte";

// The app is mounted under a base path (e.g. `/srs`) and Better Auth's client
// resolves requests from `baseURL` + `basePath` (a bare `basePath` is ignored),
// so pass the browser origin explicitly. During SSR the client is never used.
const authPath = `${base}/api/auth`;

export const authClient = createAuthClient({
	...(typeof window !== "undefined" ? { baseURL: window.location.origin } : {}),
	basePath: authPath,
});
