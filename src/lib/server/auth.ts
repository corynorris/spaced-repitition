import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { genericOAuth } from "better-auth/plugins";
import { sveltekitCookies } from "better-auth/svelte-kit";
import { getRequestEvent } from "$app/server";
import { db } from "$lib/server/db/client";

const issuer = process.env.ZITADEL_ISSUER?.replace(/\/+$/, "");
const zitadelEnabled = Boolean(
  issuer && process.env.ZITADEL_CLIENT_ID && process.env.ZITADEL_CLIENT_SECRET
);

const baseURL = process.env.BETTER_AUTH_URL ?? "http://localhost:5173";

export const auth = betterAuth({
  appName: "Spaced Repetition",
  baseURL,
  basePath: "/api/auth",
  secret: process.env.BETTER_AUTH_SECRET ?? "dev-secret-change-me-minimum-32-chars",
  database: drizzleAdapter(db, {
    provider: "pg"
  }),
  emailAndPassword: {
    enabled: true
  },
  plugins: [
    sveltekitCookies(() => getRequestEvent()),
    ...(zitadelEnabled
      ? [
          genericOAuth({
            config: [
              {
                providerId: "zitadel",
                clientId: process.env.ZITADEL_CLIENT_ID as string,
                clientSecret: process.env.ZITADEL_CLIENT_SECRET as string,
                discoveryUrl: `${issuer}/.well-known/openid-configuration`,
                scopes: ["openid", "profile", "email"],
                pkce: true
              }
            ]
          })
        ]
      : [])
  ]
});

export type Session = typeof auth.$Infer.Session;
