import { svelteKitHandler } from "better-auth/svelte-kit";
import { auth } from "$lib/server/auth";
import type { Handle } from "@sveltejs/kit";
import { building } from "$app/environment";

export const handle: Handle = async ({ event, resolve }) => {
	// Let Better Auth handle its own API routes.
	if (event.url.pathname.startsWith("/api/auth")) {
		return svelteKitHandler({ auth, event, resolve, building });
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
