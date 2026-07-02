<script lang="ts">
	let {
		text = "or",
	}: {
		text?: string;
	} = $props();
</script>

<div class="divider"><span>{text}</span></div>

<style>
	.divider {
		align-items: center;
		color: var(--c-text-muted, #81a1c1);
		display: flex;
		font-size: 0.85rem;
		gap: 0.75rem;
	}

	.divider::before,
	.divider::after {
		border-top: 1px solid var(--c-border);
		content: "";
		flex: 1;
	}
</style>
