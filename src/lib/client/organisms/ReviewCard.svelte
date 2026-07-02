<script lang="ts">
	import { labelForCardKind } from "$lib/cards/kinds";
	import { parseFuriganaMarkup } from "$lib/language/japanese";

	let {
		card,
		isJapanese = false,
		display,
		onreveal,
		revealed = false,
	}: {
		card: {
			kind: string;
			prompt: string;
			answer: string;
		};
		isJapanese?: boolean;
		display: {
			showFurigana?: boolean;
			showHiragana?: boolean;
			showEnglish?: boolean;
			showExamples?: boolean;
			showKanji?: boolean;
			showReading?: boolean;
			showDefinition?: boolean;
		};
		onreveal: () => void;
		revealed?: boolean;
	} = $props();

	const kindLabel = $derived(labelForCardKind(card.kind));
</script>

<div class="card" class:flipped={revealed}>
	<div class="card-front">
		<span class="kind">{kindLabel}</span>
		<p class="prompt">{card.prompt}</p>
		{#if !revealed}
			<button class="reveal-btn" onclick={onreveal}>
				Show answer (Space)
			</button>
		{/if}
	</div>

	<div class="card-back">
		<span class="kind">Answer</span>
		<p class="answer">{card.answer}</p>
	</div>
</div>

<style>
	.card {
		width: 100%;
		max-width: 640px;
		min-height: 220px;
	}

	.card-front,
	.card-back {
		padding: 2rem;
		border: 1px solid var(--c-border, #e0e0e0);
		border-radius: 8px;
		background: rgba(59, 66, 82, 0.86);
		box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		min-height: 220px;
	}

	.card.flipped .card-front {
		display: none;
	}

	.card:not(.flipped) .card-back {
		display: none;
	}

	.kind {
		font-size: 0.7rem;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--c-text-sub, #999);
	}

	.prompt {
		font-size: clamp(1.7rem, 5vw, 2.75rem);
		font-weight: 700;
		text-align: center;
		margin: 0;
	}

	.answer {
		font-size: 1.5rem;
		font-weight: 600;
		text-align: center;
		margin: 0;
		color: var(--c-accent, #88c0d0);
	}

	.reveal-btn {
		margin-top: 0.5rem;
		padding: 0.5rem 1.5rem;
		border: 1px solid var(--c-border, #ccc);
		border-radius: 8px;
		background: var(--c-bg);
		cursor: pointer;
		font-size: 0.9rem;
		font-family: inherit;
		color: var(--c-text);
	}

	.reveal-btn:hover {
		background: var(--c-bg-sub, #f5f5f5);
	}

	@media (max-width: 640px) {
		.card-front,
		.card-back {
			padding: 1.25rem;
		}
	}
</style>
