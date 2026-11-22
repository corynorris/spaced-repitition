import { resolve } from "$app/paths";

/**
 * Deployment base path without the trailing slash ("" or "/srs").
 *
 * SvelteKit 3 removed the `base` export from `$app/paths` in favour of
 * `resolve()`; this keeps the previous `base + "/route"` idiom working.
 */
export const base = resolve("/").replace(/\/$/, "");
