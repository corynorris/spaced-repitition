<script lang="ts">
	let {
		text,
		optional = false,
	}: {
		text: string;
		optional?: boolean;
	} = $props();
</script>

<span class="label">
	{text}
	{#if optional}<em>(optional)</em>{/if}
</span>

<style>
	.label {
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--c-text);
	}

	.label em {
		font-weight: 400;
		color: var(--c-text-sub, #888);
		font-size: 0.8rem;
	}
</style>
