import adapter from "@sveltejs/adapter-node";
import { sveltekit } from "@sveltejs/kit/vite";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

const base = (process.env.BASE_PATH ?? "") as "" | `/${string}`;

export default defineConfig(async () => ({
	plugins: [
		await sveltekit({
			preprocess: vitePreprocess(),
			adapter: adapter({ out: "build" }),
			paths: { base },
		}),
	],
}));
