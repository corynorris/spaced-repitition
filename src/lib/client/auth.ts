import { createAuthClient } from "better-auth/svelte";

// better-auth 1.7 registers generic OAuth providers as first-class social
// providers, so no client plugin is needed (`signIn.social({ provider })`).
export const authClient = createAuthClient();
