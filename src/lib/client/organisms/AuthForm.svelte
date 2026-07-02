<script lang="ts">
	import { enhance } from "$app/forms";
	import { base } from "$app/paths";
	import Button from "$lib/client/atoms/Button.svelte";
	import Divider from "$lib/client/atoms/Divider.svelte";
	import ErrorBanner from "$lib/client/atoms/ErrorBanner.svelte";
	import { authClient } from "$lib/client/auth";
	import type { Snippet } from "svelte";

	let {
		mode,
		error,
		zitadelEnabled = false,
		children,
	}: {
		mode: "login" | "register";
		error?: string;
		zitadelEnabled?: boolean;
		children: Snippet;
	} = $props();

	function signInWithZitadel() {
		authClient.signIn.oauth2({ providerId: "zitadel" });
	}
</script>

<main class="auth-page">
	<form class="auth-card" method="POST" use:enhance>
		{#if mode === "login"}
			<p class="eyebrow">Sign in</p>
			<h1>Welcome back</h1>
		{:else}
			<p class="eyebrow">Create account</p>
			<h1>Start learning</h1>
		{/if}

		<ErrorBanner message={error} />

		{#if zitadelEnabled}
			<Button variant="primary" onclick={signInWithZitadel}>
				{mode === "login" ? "Sign in with Zitadel" : "Sign up with Zitadel"}
			</Button>
			<Divider />
		{/if}

		{@render children()}

		<p class="fine-print">
			{#if mode === "login"}
				Don't have an account? <a href="{base}/register">Create one</a>.
			{:else}
				Already have an account? <a href="{base}/login">Sign in</a>.
			{/if}
		</p>
	</form>
</main>

<style>
	.auth-page {
		max-width: 460px;
		margin: 0 auto;
		padding: clamp(2rem, 6vw, 5rem) 1rem;
	}

	.auth-card {
		background: var(--c-bg-elevated, #3b4252);
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		display: grid;
		gap: 1rem;
		padding: 1.5rem;
	}

	h1 {
		font-size: clamp(2rem, 4vw, 3rem);
		letter-spacing: 0;
		line-height: 1;
		margin: 0;
		color: var(--c-text);
	}

	.eyebrow {
		color: var(--c-accent, #88c0d0);
		font-size: 0.78rem;
		font-weight: 700;
		letter-spacing: 0.08em;
		margin: 0;
		text-transform: uppercase;
	}

	.fine-print {
		color: var(--c-text-sub);
		line-height: 1.6;
		font-size: 0.9rem;
		margin: 0;
	}

	a {
		color: var(--c-accent, #88c0d0);
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}
</style>
