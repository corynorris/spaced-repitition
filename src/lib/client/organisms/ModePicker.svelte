<script lang="ts">
	import { enhance } from "$app/forms";
	import { base } from "$app/paths";
	import Button from "$lib/client/atoms/Button.svelte";
	import ModeCard from "$lib/client/ModeCard.svelte";

	let {
		mode = null,
		onchoose,
	}: {
		mode: "ai" | "manual" | null;
		onchoose: (mode: "ai" | "manual") => void;
	} = $props();
</script>

<section class="mode-picker" aria-label="Course creation method">
	<ModeCard
		title="AI generated"
		description="Describe the course and review a generated lesson preview before saving."
		selected={mode === "ai"}
		onclick={() => onchoose("ai")}
	/>
	<ModeCard
		title="Manual setup"
		description="Create an empty course now, then add notes yourself."
		selected={mode === "manual"}
		onclick={() => onchoose("manual")}
	/>
</section>

<style>
	.mode-picker {
		display: grid;
		gap: 1rem;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		margin-bottom: 1rem;
	}

	@media (max-width: 640px) {
		.mode-picker {
			grid-template-columns: 1fr;
		}
	}
</style>
