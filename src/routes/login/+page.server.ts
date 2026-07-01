import { fail, redirect } from "@sveltejs/kit";
import { base } from "$app/paths";
import { auth } from "$lib/server/auth";
import type { Actions, PageServerLoad } from "./$types";

const zitadelEnabled = Boolean(
  process.env.ZITADEL_ISSUER &&
  process.env.ZITADEL_CLIENT_ID &&
  process.env.ZITADEL_CLIENT_SECRET
);

export const load: PageServerLoad = async ({ locals }) => {
  if (locals.user) {
    throw redirect(303, `${base}/app`);
  }
  return { zitadelEnabled };
};

export const actions: Actions = {
  default: async ({ request }) => {
    const formData = await request.formData();
    const email = String(formData.get("email") ?? "").trim();
    const password = String(formData.get("password") ?? "");

    if (!email || !password) {
      return fail(400, { error: "Email and password are required." });
    }

    try {
      await auth.api.signInEmail({
        body: { email, password },
        headers: request.headers
      });
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Invalid email or password.";
      return fail(400, { error: message });
    }

    throw redirect(303, `${base}/app`);
  }
};
