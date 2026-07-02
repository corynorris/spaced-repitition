<script lang="ts">
	let {
		title,
		description,
		selected = false,
		onclick,
	}: {
		title: string;
		description: string;
		selected?: boolean;
		onclick: () => void;
	} = $props();
</script>

<button type="button" class="mode-card" class:selected onclick={onclick}>
	<span class="mode-title">{title}</span>
	<span class="mode-copy">{description}</span>
</button>

<style>
	.mode-card {
		background: var(--c-bg-elevated, #3b4252);
		border: 2px solid var(--c-border, #4c566a);
		border-radius: 12px;
		padding: 1.5rem;
		cursor: pointer;
		text-align: left;
		transition: all 0.2s ease;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.mode-card:hover {
		background: var(--c-bg-hover, #434c5e);
		border-color: var(--c-accent, #88c0d0);
	}

	.mode-card.selected {
		background: rgba(136, 192, 208, 0.15);
		border-color: var(--c-accent, #88c0d0);
		box-shadow: 0 0 0 1px var(--c-accent, #88c0d0);
	}

	.mode-title {
		font-size: 1.1rem;
		font-weight: 700;
		color: var(--c-text, #eceff4);
	}

	.mode-copy {
		font-size: 0.9rem;
		color: var(--c-text-sub, #d8dee9);
		line-height: 1.5;
	}
</style>
