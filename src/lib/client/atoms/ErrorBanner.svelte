<script lang="ts">
	let {
		message,
		onretry,
	}: {
		message?: string;
		onretry?: () => void;
	} = $props();
</script>

{#if message}
	{#if onretry}
		<div class="error-card">
			<p class="error">{message}</p>
			<button class="retry-btn" onclick={onretry}>Retry</button>
		</div>
	{:else}
		<p class="error">{message}</p>
	{/if}
{/if}

<style>
	.error {
		background: var(--c-danger-bg, rgba(191, 97, 106, 0.15));
		border: 1px solid var(--c-danger);
		border-radius: 0.5rem;
		color: var(--c-danger);
		font-size: 0.9rem;
		padding: 0.65rem 0.75rem;
		margin: 0;
	}

	.error-card {
		background: var(--c-danger-bg, #fef2f2);
		border: 1px solid var(--c-danger-sub, #fecaca);
		border-radius: 8px;
		padding: 0.75rem 1rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.retry-btn {
		padding: 0.3rem 0.8rem;
		border: 1px solid var(--c-danger, #dc2626);
		border-radius: 6px;
		background: transparent;
		color: var(--c-danger, #dc2626);
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
		white-space: nowrap;
		font-family: inherit;
	}

	.retry-btn:hover {
		background: var(--c-danger, #dc2626);
		color: var(--c-bg);
	}
</style>
