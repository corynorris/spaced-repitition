<script lang="ts">
	import StatBox from "$lib/client/molecules/StatBox.svelte";

	let {
		totalCards,
		dueCards,
		reviewedToday,
	}: {
		totalCards: number;
		dueCards: number;
		reviewedToday: number;
	} = $props();
</script>

<section class="stats-bar" aria-label="Course stats">
	<StatBox value={totalCards} label="cards" />
	<StatBox value={dueCards} label="due" />
	<StatBox value={reviewedToday} label="reviewed today" />
</section>

<style>
	.stats-bar {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 0.75rem;
	}

	@media (max-width: 640px) {
		.stats-bar {
			grid-template-columns: 1fr;
		}
	}
</style>
