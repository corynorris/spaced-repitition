<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		variant = "default",
		type = "button",
		disabled = false,
		href,
		onclick,
		children,
	}: {
		variant?: "default" | "primary" | "accent" | "danger";
		type?: "button" | "submit";
		disabled?: boolean;
		href?: string;
		onclick?: () => void;
		children?: Snippet;
	} = $props();
</script>

{#if href}
	<a class="button {variant}" {href}>
		{@render children?.()}
	</a>
{:else}
	<button class="button {variant}" {type} {disabled} {onclick}>
		{@render children?.()}
	</button>
{/if}

<style>
	.button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		font-weight: 700;
		min-height: 2.75rem;
		padding: 0.65rem 1rem;
		background: var(--c-bg-elevated, #3b4252);
		color: var(--c-text);
		cursor: pointer;
		font-size: 0.9rem;
		font-family: inherit;
		text-decoration: none;
		transition:
			background 0.15s,
			border-color 0.15s;
	}

	.button:hover:not(:disabled) {
		background: var(--c-bg-hover, #434c5e);
	}

	.button:disabled {
		opacity: 0.55;
		cursor: not-allowed;
	}

	.button.primary {
		background: var(--c-accent, #88c0d0);
		border-color: var(--c-accent, #88c0d0);
		color: var(--c-accent-text, var(--c-bg));
	}

	.button.primary:hover:not(:disabled) {
		background: var(--c-accent-hover, #8fbcbb);
		border-color: var(--c-accent-hover, #8fbcbb);
		opacity: 1;
	}

	.button.accent {
		background: var(--c-success, #a3be8c);
		border-color: var(--c-success, #a3be8c);
		color: var(--c-bg);
	}

	.button.accent:hover:not(:disabled) {
		opacity: 0.9;
	}

	.button.danger {
		background: var(--c-danger, #bf616a);
		border-color: var(--c-danger, #bf616a);
		color: var(--c-bg);
	}

	.button.danger:hover:not(:disabled) {
		opacity: 0.9;
	}
</style>
