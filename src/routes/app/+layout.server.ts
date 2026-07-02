import { redirect } from "@sveltejs/kit";
import { base } from "$app/paths";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(303, `${base}/login`);
	}

	return {
		user: locals.user,
	};
};
