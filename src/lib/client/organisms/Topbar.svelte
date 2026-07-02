<script lang="ts">
	import { enhance } from "$app/forms";
	import { base } from "$app/paths";
	import LinkButton from "$lib/client/atoms/LinkButton.svelte";

	let {
		user,
	}: {
		user: { id: string; email: string; name?: string | null } | null;
	} = $props();
</script>

<header class="topbar">
	<a class="brand" href={user ? `${base}/app` : `${base}/`}>Spaced Repetition</a>
	<nav>
		{#if user}
			<a href="{base}/app">Dashboard</a>
			<form method="POST" action="{base}/signout" use:enhance style="display:inline">
				<LinkButton>Sign out</LinkButton>
			</form>
		{:else}
			<a href="{base}/login">Sign in</a>
			<a href="{base}/register">Create account</a>
		{/if}
	</nav>
</header>

<style>
	.topbar {
		align-items: center;
		background: rgba(59, 66, 82, 0.92);
		border-bottom: 1px solid var(--c-border);
		display: flex;
		justify-content: space-between;
		padding: 1rem clamp(1rem, 4vw, 2rem);
		position: sticky;
		top: 0;
		z-index: 10;
		backdrop-filter: blur(14px);
	}

	.brand {
		font-weight: 700;
		color: var(--c-accent, #88c0d0);
		text-decoration: none;
	}

	nav {
		display: flex;
		gap: 1rem;
		font-size: 0.95rem;
		align-items: center;
		flex-wrap: wrap;
		justify-content: flex-end;
	}

	nav a {
		color: inherit;
		text-decoration: none;
	}

	nav a:hover {
		text-decoration: underline;
		color: var(--c-accent, #88c0d0);
	}

	@media (max-width: 640px) {
		.topbar {
			align-items: flex-start;
			gap: 0.75rem;
			padding: 0.85rem 1rem;
		}

		nav {
			gap: 0.7rem;
			font-size: 0.9rem;
		}
	}
</style>
