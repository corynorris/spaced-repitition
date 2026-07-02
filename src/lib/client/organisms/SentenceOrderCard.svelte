<script lang="ts">
	import { labelForCardKind } from "$lib/cards/kinds";

	let {
		card,
		wordBlocks,
		correctOrder,
		onrating,
	}: {
		card: {
			kind: string;
			prompt: string;
			answer: string;
		};
		wordBlocks: string[];
		correctOrder?: number[];
		onrating: (rating: string) => void;
	} = $props();

	let selectedIndices = $state<number[]>([]);
	let submitted = $state(false);
	let isCorrect = $state<boolean | null>(null);
	let feedbackTimer = $state<ReturnType<typeof setTimeout> | null>(null);

	// Shuffle indices on mount
	function makeShuffled() {
		return [...Array(wordBlocks.length).keys()].sort(() => Math.random() - 0.5);
	}
	let shuffledIndices = $state(makeShuffled());

	const availableBlocks = $derived(
		shuffledIndices.filter((i) => !selectedIndices.includes(i)),
	);
	const allBlocksSelected = $derived(
		selectedIndices.length === shuffledIndices.length,
	);
	const kindLabel = $derived(labelForCardKind(card.kind));

	function selectBlock(index: number) {
		if (submitted) return;
		selectedIndices = [...selectedIndices, index];
	}

	function removeBlock(index: number) {
		if (submitted) return;
		selectedIndices = selectedIndices.filter((i) => i !== index);
	}

	function submit() {
		if (submitted) return;
		const correct = selectedIndices.every(
			(val, i) => val === (correctOrder?.[i] ?? i),
		);
		isCorrect = correct;
		submitted = true;

		feedbackTimer = setTimeout(() => {
			onrating(correct ? "good" : "again");
		}, 1200);
	}

	function cancelFeedbackAndAdvance() {
		if (feedbackTimer) {
			clearTimeout(feedbackTimer);
			feedbackTimer = null;
		}
		onrating(isCorrect ? "good" : "again");
	}

	export function nextCard() {
		if (feedbackTimer) {
			clearTimeout(feedbackTimer);
			feedbackTimer = null;
		}
		submitted = false;
		isCorrect = null;
		selectedIndices = [];
		shuffledIndices = makeShuffled();
	}

	export function handleKeydown(e: KeyboardEvent): boolean {
		if (submitted) {
			if (e.key === " " || e.key === "Enter") {
				e.preventDefault();
				cancelFeedbackAndAdvance();
				return true;
			}
			return false;
		}
		if (e.key === "Enter" && allBlocksSelected) {
			e.preventDefault();
			submit();
			return true;
		}
		return false;
	}
</script>

<div class="card auto-graded">
	<span class="kind">{kindLabel}</span>

	{#if submitted}
		<div class="feedback" class:correct={isCorrect} class:incorrect={!isCorrect}>
			<span class="result-icon">{isCorrect ? "✓" : "✗"}</span>
			<span class="result-label">{isCorrect ? "Correct!" : "Incorrect"}</span>
		</div>
		<p class="correct-sentence">{card.answer}</p>
		<p class="continue-hint">Press Space or Enter to continue</p>
	{:else}
		<p class="prompt">{card.prompt}</p>
		<p class="hint">Arrange the words below to form the sentence.</p>

		<div class="build-area">
			{#if selectedIndices.length === 0}
				<span class="build-placeholder">Click words below to build the sentence…</span>
			{:else}
				{#each selectedIndices as index (index)}
					<button class="block selected" onclick={() => removeBlock(index)}>
						{wordBlocks[index] ?? ""}
					</button>
				{/each}
			{/if}
		</div>

		<div class="word-blocks">
			{#each availableBlocks as index (index)}
				<button class="block" onclick={() => selectBlock(index)}>
					{wordBlocks[index] ?? ""}
				</button>
			{/each}
		</div>

		<button class="submit-btn" disabled={!allBlocksSelected} onclick={submit}>
			Submit
			{#if !allBlocksSelected}
				<span class="submit-hint">
					({selectedIndices.length}/{shuffledIndices.length} placed)
				</span>
			{/if}
		</button>
	{/if}
</div>

<style>
	.card {
		padding: 2rem;
		border: 1px solid var(--c-border, #e0e0e0);
		border-radius: 8px;
		background: rgba(59, 66, 82, 0.86);
		box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		min-height: 220px;
		max-width: 640px;
		width: 100%;
	}

	.kind {
		font-size: 0.7rem;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--c-text-sub, #999);
	}

	.prompt {
		font-size: 1.3rem;
		font-weight: 600;
		text-align: center;
		margin: 0;
	}

	.hint {
		font-size: 0.85rem;
		color: var(--c-text-sub, #888);
		margin: 0;
		text-align: center;
	}

	.build-area {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		justify-content: center;
		min-height: 2.5rem;
		padding: 0.5rem;
		border: 2px dashed var(--c-border, #ccc);
		border-radius: 8px;
		width: 100%;
		align-items: center;
	}

	.build-placeholder {
		color: var(--c-text-sub, #999);
		font-size: 0.85rem;
	}

	.word-blocks {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		justify-content: center;
		width: 100%;
	}

	.block {
		padding: 0.4rem 0.8rem;
		border: 1px solid var(--c-border, #ccc);
		border-radius: 6px;
		background: var(--c-bg);
		cursor: pointer;
		font-size: 1.1rem;
		font-family: inherit;
		color: var(--c-text);
		transition:
			background 0.15s,
			border-color 0.15s;
		user-select: none;
	}

	.block:hover {
		background: var(--c-bg-sub, #f0f0f0);
		border-color: var(--c-accent, #88c0d0);
	}

	.block.selected {
		background: var(--c-accent, #88c0d0);
		color: var(--c-accent-text, var(--c-bg));
		border-color: var(--c-accent, #88c0d0);
	}

	.block.selected:hover {
		opacity: 0.85;
	}

	.submit-btn {
		padding: 0.6rem 2rem;
		border: none;
		border-radius: 8px;
		background: var(--c-accent, #88c0d0);
		color: var(--c-accent-text, var(--c-bg));
		font-weight: 600;
		font-size: 0.95rem;
		font-family: inherit;
		cursor: pointer;
		transition: opacity 0.15s;
	}

	.submit-btn:hover:not(:disabled) {
		opacity: 0.9;
	}

	.submit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.submit-hint {
		font-weight: 400;
		font-size: 0.8rem;
		opacity: 0.8;
		margin-left: 0.5rem;
	}

	.feedback {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1.5rem;
		border-radius: 8px;
		font-weight: 700;
		font-size: 1.1rem;
	}

	.feedback.correct {
		background: rgba(163, 190, 140, 0.18);
		color: var(--c-success);
	}

	.feedback.incorrect {
		background: rgba(191, 97, 106, 0.18);
		color: var(--c-danger);
	}

	.result-icon {
		font-size: 1.3rem;
	}

	.correct-sentence {
		font-size: 1.3rem;
		font-weight: 600;
		text-align: center;
		margin: 0;
		color: var(--c-accent, #88c0d0);
	}

	.continue-hint {
		font-size: 0.8rem;
		color: var(--c-text-sub, #999);
		margin: 0;
	}

	@media (max-width: 640px) {
		.card {
			padding: 1.25rem;
		}
	}
</style>
