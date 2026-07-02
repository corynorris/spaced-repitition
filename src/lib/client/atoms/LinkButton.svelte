<script lang="ts">
	let {
		onclick,
		children,
	}: {
		onclick?: () => void;
		children: import("svelte").Snippet;
	} = $props();
</script>

<button class="link-button" type="button" {onclick}>
	{@render children()}
</button>

<style>
	.link-button {
		background: none;
		border: none;
		color: var(--c-text);
		cursor: pointer;
		font: inherit;
		font-size: 0.95rem;
		padding: 0;
	}

	.link-button:hover {
		text-decoration: underline;
		color: var(--c-accent, #88c0d0);
	}
</style>
