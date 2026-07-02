<script lang="ts">
	let {
		text,
	}: {
		text: string;
	} = $props();
</script>

<span class="badge">{text}</span>

<style>
	.badge {
		background: var(--c-bg-sub, #f0f0f0);
		border: 1px solid var(--c-border);
		padding: 0.15rem 0.5rem;
		border-radius: 4px;
		font-size: 0.8rem;
		color: var(--c-text-sub, #888);
	}
</style>
