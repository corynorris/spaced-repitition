<script lang="ts">
	let {
		value,
		label,
	}: {
		value: string | number;
		label: string;
	} = $props();
</script>

<span class="stat-box">
	<strong>{value}</strong>
	{label}
</span>

<style>
	.stat-box {
		background: rgba(59, 66, 82, 0.78);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		padding: 0.8rem;
		font-size: 0.85rem;
		color: var(--c-text-sub, #888);
	}

	strong {
		color: var(--c-text);
		display: block;
		font-size: 1.45rem;
		line-height: 1;
		margin-bottom: 0.25rem;
	}
</style>
