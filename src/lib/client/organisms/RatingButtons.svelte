<script lang="ts">
	let {
		onrating,
	}: {
		onrating: (rating: string) => void;
	} = $props();
</script>

<div class="ratings">
	<button class="rating again" onclick={() => onrating("again")}>
		<span class="key">1</span>
		<span class="label">Again</span>
	</button>
	<button class="rating hard" onclick={() => onrating("hard")}>
		<span class="key">2</span>
		<span class="label">Hard</span>
	</button>
	<button class="rating good" onclick={() => onrating("good")}>
		<span class="key">3</span>
		<span class="label">Good</span>
	</button>
	<button class="rating easy" onclick={() => onrating("easy")}>
		<span class="key">4</span>
		<span class="label">Easy</span>
	</button>
</div>

<style>
	.ratings {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.5rem;
		width: 100%;
		max-width: 640px;
	}

	.rating {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.2rem;
		padding: 0.75rem 0.5rem;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		font-family: inherit;
		font-weight: 600;
		font-size: 0.85rem;
		color: var(--c-bg);
		transition: opacity 0.15s;
	}

	.rating:hover {
		opacity: 0.85;
	}

	.key {
		font-size: 0.7rem;
		opacity: 0.7;
		background: rgba(255, 255, 255, 0.2);
		padding: 0.1rem 0.4rem;
		border-radius: 4px;
	}

	.again {
		background: var(--c-danger);
	}

	.hard {
		background: var(--c-warning, #ebcb8b);
	}

	.good {
		background: var(--c-success);
	}

	.easy {
		background: var(--c-info, #5e81ac);
	}

	@media (max-width: 640px) {
		.ratings {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
