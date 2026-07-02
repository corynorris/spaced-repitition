<script lang="ts">
	let { lines = 3, width = "100%" }: { lines?: number; width?: string } =
		$props();
</script>

<div class="skeleton" style="width: {width}">
	{#each Array(lines) as _}
		<div class="skeleton-line"></div>
	{/each}
</div>

<style>
	.skeleton {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 1rem;
	}

	.skeleton-line {
		height: 1rem;
		border-radius: 4px;
		background: linear-gradient(
			90deg,
			var(--c-bg-sub, #e0e0e0) 25%,
			var(--c-bg, #f5f5f5) 50%,
			var(--c-bg-sub, #e0e0e0) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s ease-in-out infinite;
	}

	.skeleton-line:last-child {
		width: 60%;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}
</style>
