<script lang="ts">
	import { enhance } from "$app/forms";
	import Button from "$lib/client/atoms/Button.svelte";

	let {
		preview,
	}: {
		preview: {
			title: string;
			mode: string;
			sentences?: Array<{ target: string; translation: string }>;
			setting?: string;
			dialogue?: Array<{ speaker: string; target: string; translation: string }>;
		};
	} = $props();
</script>

<div class="preview">
	<h3>{preview.title}</h3>
	{#if preview.mode === "sentences"}
		<ul>
			{#each preview.sentences ?? [] as sentence}
				<li>{sentence.target} — {sentence.translation}</li>
			{/each}
		</ul>
	{:else}
		{#if preview.setting}<p>{preview.setting}</p>{/if}
		<ul>
			{#each preview.dialogue ?? [] as turn}
				<li>{turn.speaker}: {turn.target} — {turn.translation}</li>
			{/each}
		</ul>
	{/if}
	<form method="POST" action="?/createPractice" use:enhance>
		<input type="hidden" name="preview" value={JSON.stringify(preview)} />
		<Button variant="primary" type="submit">Add practice lesson</Button>
	</form>
</div>

<style>
	.preview {
		background: var(--c-bg-sub, #f0f0f0);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		padding: 1rem;
	}

	h3 {
		margin: 0 0 0.5rem;
	}
</style>
