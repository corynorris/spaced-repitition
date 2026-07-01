import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  // Better Auth will populate this in the auth implementation milestone.
  event.locals.user = null;
  return resolve(event);
};
