import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { genericOAuth } from "better-auth/plugins";
import { sveltekitCookies } from "better-auth/svelte-kit";
import { getRequestEvent } from "$app/server";
import { db } from "#lib/server/db/client";
import * as schema from "#lib/server/db/schema";

const issuer = process.env.ZITADEL_ISSUER?.replace(/\/+$/, "");
const zitadelEnabled = Boolean(
	issuer && process.env.ZITADEL_CLIENT_ID && process.env.ZITADEL_CLIENT_SECRET,
);

const publicUrl = new URL(
	process.env.BETTER_AUTH_URL ?? "http://localhost:5173",
);

// Better Auth treats a path in `baseURL` as the auth base and ignores
// `basePath`, so keep `baseURL` at the origin and fold the app mount point
// (e.g. /srs) into `basePath`. The endpoints then live at
// <origin><app base>/api/auth/*, which is where the browser client calls.
const baseURL = publicUrl.origin;
const basePath = `${publicUrl.pathname.replace(/\/$/, "")}/api/auth`;

export const auth = betterAuth({
	appName: "Spaced Repetition",
	baseURL,
	basePath,
	secret:
		process.env.BETTER_AUTH_SECRET ?? "dev-secret-change-me-minimum-32-chars",
	database: drizzleAdapter(db, {
		provider: "pg",
		schema,
	}),
	emailAndPassword: {
		enabled: true,
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
								pkce: true,
							},
						],
					}),
				]
			: []),
	],
});

export type Session = typeof auth.$Infer.Session;
