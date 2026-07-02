<script lang="ts">
	import { parseFuriganaMarkup } from "$lib/language/japanese";
	import Badge from "$lib/client/atoms/Badge.svelte";

	let {
		note,
		isJapanese = false,
		display,
	}: {
		note: {
			term: string;
			reading?: string | null;
			definition: string;
			furigana?: string | null;
			tags: string[];
		};
		isJapanese?: boolean;
		display: {
			showKanji?: boolean;
			showHiragana?: boolean;
			showFurigana?: boolean;
			showEnglish?: boolean;
			showTerm?: boolean;
			showReading?: boolean;
			showDefinition?: boolean;
		};
	} = $props();

	const showTerm = $derived(
		!isJapanese || display.showKanji !== false,
	);
	const showReading = $derived(
		isJapanese ? display.showHiragana !== false : display.showReading !== false,
	);
	const showDefinition = $derived(
		isJapanese ? display.showEnglish !== false : display.showDefinition !== false,
	);
</script>

<div class="note-item">
	<div class="note-main">
		{#if isJapanese && display.showFurigana && note.furigana}
			<span class="term">
				{#each parseFuriganaMarkup(note.furigana) as token}
					{#if token.type === "ruby"}
						<ruby>{token.base}<rt>{token.reading}</rt></ruby>
					{:else}
						{token.text}
					{/if}
				{/each}
			</span>
		{:else if showTerm}
			<span class="term">{note.term}</span>
		{/if}
		{#if note.reading && showReading}
			<span class="reading">({note.reading})</span>
		{/if}
		{#if showDefinition}
			<span class="definition">{note.definition}</span>
		{/if}
	</div>
	{#if note.tags.length > 0}
		<div class="tags">
			{#each note.tags as tag}
				<Badge text={tag} />
			{/each}
		</div>
	{/if}
</div>

<style>
	.note-item {
		background: rgba(59, 66, 82, 0.64);
		padding: 0.9rem 1rem;
		border: 1px solid var(--c-border, #e0e0e0);
		border-radius: 8px;
	}

	.note-main {
		display: flex;
		gap: 0.5rem;
		align-items: baseline;
		flex-wrap: wrap;
	}

	.term {
		font-weight: 700;
		font-size: 1.05rem;
	}

	.reading {
		color: var(--c-text-sub, #888);
		font-size: 0.9rem;
	}

	.definition {
		color: var(--c-text-sub, #555);
		font-size: 0.9rem;
	}

	.definition::before {
		content: "—";
		margin-right: 0.5rem;
		color: var(--c-text-sub, #aaa);
	}

	.tags {
		display: flex;
		gap: 0.35rem;
		margin-top: 0.4rem;
		flex-wrap: wrap;
	}

	@media (max-width: 640px) {
		.definition::before {
			content: "";
			margin: 0;
		}
	}
</style>
