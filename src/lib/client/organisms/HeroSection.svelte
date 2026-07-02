<script lang="ts">
	import { base } from "$app/paths";
	import Button from "$lib/client/atoms/Button.svelte";

	let {
		eyebrow,
		title,
		lede,
	}: {
		eyebrow?: string;
		title: string;
		lede?: string;
	} = $props();
</script>

<main class="hero">
	<section>
		{#if eyebrow}
			<p class="eyebrow">{eyebrow}</p>
		{/if}
		<h1>{title}</h1>
		{#if lede}
			<p class="lede">{lede}</p>
		{/if}
		<div class="actions">
			<Button variant="primary" href="{base}/app">Open app</Button>
			<Button href="{base}/login">Sign in</Button>
		</div>
	</section>
</main>

<style>
	.hero {
		align-items: center;
		display: grid;
		min-height: calc(100vh - 65px);
		max-width: 1120px;
		margin: 0 auto;
		padding: clamp(2rem, 6vw, 5rem) 1rem;
	}

	.eyebrow {
		color: var(--c-accent, #88c0d0);
		font-size: 0.78rem;
		font-weight: 700;
		letter-spacing: 0.08em;
		margin: 0 0 0.75rem;
		text-transform: uppercase;
	}

	h1 {
		font-size: clamp(2rem, 7vw, 4.25rem);
		letter-spacing: 0;
		line-height: 1;
		margin: 0;
		color: var(--c-text);
	}

	.lede {
		color: var(--c-text-sub);
		font-size: 1.15rem;
		line-height: 1.6;
		max-width: 680px;
	}

	.actions {
		align-items: center;
		display: flex;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	@media (max-width: 640px) {
		.actions {
			align-items: stretch;
			flex-direction: column;
		}
	}
</style>
