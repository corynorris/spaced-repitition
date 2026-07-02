<script lang="ts">
	import { labelForCardKind } from "$lib/cards/kinds";
	import { gradeSentenceCloze, splitClozeSentence } from "$lib/cards/grading";

	let {
		card,
		languageProfile,
		onrating,
	}: {
		card: {
			kind: string;
			prompt: string;
			answer: string;
			extra?: Record<string, unknown>;
		};
		languageProfile: string;
		onrating: (rating: string) => void;
	} = $props();

	let typedAnswer = $state("");
	let submitted = $state(false);
	let isCorrect = $state<boolean | null>(null);
	let feedbackTimer = $state<ReturnType<typeof setTimeout> | null>(null);

	const kindLabel = $derived(labelForCardKind(card.kind));
	const translation = $derived(card.extra?.translation as string | undefined);
	const clozeSplit = $derived.by(() => {
		const extra = card.extra;
		const sentence = (extra?.sentence as string) ?? card.prompt;
		return splitClozeSentence(sentence, card.answer);
	});

	function submit() {
		if (submitted || !typedAnswer.trim()) return;
		const correct = gradeSentenceCloze(
			typedAnswer,
			card.answer,
			languageProfile,
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
		typedAnswer = "";
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
		if (e.key === "Enter" && typedAnswer.trim()) {
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
		{#if !isCorrect}
			<p class="correct-answer">Answer: <strong>{card.answer}</strong></p>
		{/if}
		<p class="continue-hint">Press Space or Enter to continue</p>
	{:else}
		<p class="cloze-sentence">
			{#if clozeSplit?.found}
				<span class="cloze-before">{clozeSplit.before}</span>
				<span class="cloze-blank">___</span>
				<span class="cloze-after">{clozeSplit.after}</span>
			{:else}
				{card.prompt}
			{/if}
		</p>

		{#if translation}
			<p class="hint">{translation}</p>
		{/if}

		<input
			type="text"
			class="cloze-input"
			bind:value={typedAnswer}
			placeholder="Type the missing word…"
			onkeydown={(e) => {
				if (e.key === "Enter") {
					e.preventDefault();
					if (typedAnswer.trim()) submit();
				}
			}}
		/>

		<button class="submit-btn" disabled={!typedAnswer.trim()} onclick={submit}>
			Submit
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

	.cloze-sentence {
		font-size: 1.5rem;
		font-weight: 500;
		text-align: center;
		margin: 0;
		line-height: 1.6;
	}

	.cloze-blank {
		display: inline-block;
		min-width: 3rem;
		border-bottom: 2px solid var(--c-accent, #88c0d0);
		color: var(--c-accent, #88c0d0);
		font-weight: 700;
		padding: 0 0.3rem;
		margin: 0 0.2rem;
	}

	.hint {
		font-size: 0.85rem;
		color: var(--c-text-sub, #888);
		margin: 0;
		text-align: center;
	}

	.cloze-input {
		width: 100%;
		max-width: 280px;
		padding: 0.6rem 1rem;
		border: 1px solid var(--c-border, #ccc);
		border-radius: 8px;
		font-size: 1.1rem;
		font-family: inherit;
		text-align: center;
		outline: none;
		background: var(--c-bg);
		color: var(--c-text);
		transition: border-color 0.15s;
	}

	.cloze-input:focus {
		border-color: var(--c-accent, #88c0d0);
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

	.correct-answer {
		font-size: 1rem;
		text-align: center;
		margin: 0;
		color: var(--c-text-sub, #666);
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
