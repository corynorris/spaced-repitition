import { isAuthPath } from "better-auth/svelte-kit";
import { auth } from "#lib/server/auth";
import type { Handle } from "@sveltejs/kit/hooks";

export const handle: Handle = async ({ event, resolve }) => {
	// Let Better Auth handle its own API routes. `isAuthPath` compares the full
	// URL against baseURL + basePath, so it works when the app is mounted under
	// a base path (e.g. /srs) — a plain pathname prefix check does not, because
	// SvelteKit keeps the base in `event.url`.
	if (isAuthPath(event.url.href, auth.options)) {
		return auth.handler(event.request);
	}

	// For all other routes, populate locals.user from the session cookie.
	const session = await auth.api.getSession({ headers: event.request.headers });

	event.locals.user = session
		? {
				id: session.user.id,
				email: session.user.email,
				name: session.user.name ?? null,
			}
		: null;

	return resolve(event);
};
