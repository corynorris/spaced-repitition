<script lang="ts">
	import { enhance } from "$app/forms";
	import { base } from "$app/paths";
	import { labelForCardKind } from "$lib/cards/kinds";
	import {
		gradeSentenceCloze,
		gradeSentenceOrder,
		isAutoGradedCardKind,
		shuffleIndices,
		splitClozeSentence,
	} from "$lib/cards/grading";
	import { parseFuriganaMarkup } from "$lib/language/japanese";

	let { data } = $props();

	let currentIndex = $state(0);
	let showAnswer = $state(false);
	let finished = $state(false);

	// ---- Auto-graded card state ----
	let shuffledIndices = $state<number[]>([]);
	let selectedIndices = $state<number[]>([]);
	let typedAnswer = $state("");
	let submitted = $state(false);
	let isCorrect = $state<boolean | null>(null);
	let feedbackTimer = $state<ReturnType<typeof setTimeout> | null>(null);

	const currentCard = $derived(data.dueCards[currentIndex] ?? null);
	const dueLength = $derived(data.dueCards.length);
	const isJapanese = $derived(data.course.languageProfile === "japanese");
	const display = $derived(data.course.displayOptions);

	// ---- Card-type-specific derived data ----
	const autoGraded = $derived(
		currentCard ? isAutoGradedCardKind(currentCard.card.kind) : false,
	);

	const wordBlocks = $derived(
		(currentCard?.card.extra as Record<string, unknown> | undefined)
			?.word_blocks as string[] | undefined,
	);

	const correctOrder = $derived(
		(currentCard?.card.extra as Record<string, unknown> | undefined)
			?.correct_order as number[] | undefined,
	);

	const clozeSplit = $derived.by(() => {
		if (!currentCard || currentCard.card.kind !== "sentence_cloze")
			return null;
		const extra = currentCard.card.extra as Record<string, unknown> | undefined;
		const sentence = (extra?.sentence as string) ?? currentCard.card.prompt;
		return splitClozeSentence(sentence, currentCard.card.answer);
	});

	// ---- Derived helpers ----
	const availableBlocks = $derived(
		shuffledIndices.filter((i) => !selectedIndices.includes(i)),
	);

	const allBlocksSelected = $derived(
		selectedIndices.length === (shuffledIndices.length || 0),
	);

	// ---- Reset on card change ----
	$effect(() => {
		if (currentCard && autoGraded) {
			if (feedbackTimer) {
				clearTimeout(feedbackTimer);
				feedbackTimer = null;
			}
			submitted = false;
			isCorrect = null;
			typedAnswer = "";

			if (currentCard.card.kind === "sentence_order") {
				const blocks = wordBlocks ?? [];
				shuffledIndices = shuffleIndices(blocks.length);
				selectedIndices = [];
			}
		}
	});

	// ---- Manual rating (reveal cards) ----
	function submitRating(rating: string) {
		const form = document.getElementById("review-form") as HTMLFormElement;
		const ratingInput = document.getElementById(
			"rating-input",
		) as HTMLInputElement;
		ratingInput.value = rating;
		form.requestSubmit();
	}

	function advanceToNext() {
		showAnswer = false;
		if (feedbackTimer) {
			clearTimeout(feedbackTimer);
			feedbackTimer = null;
		}
		if (currentIndex < dueLength - 1) {
			currentIndex++;
		} else {
			finished = true;
		}
	}

	function rateThis(rating: string) {
		submitRating(rating);
		advanceToNext();
	}

	// ---- Auto-graded submission ----
	function submitAutoGraded() {
		if (!currentCard || submitted) return;

		let correct = false;
		if (currentCard.card.kind === "sentence_order") {
			correct = gradeSentenceOrder(
				selectedIndices,
				correctOrder ?? selectedIndices.map((_, i) => i),
			);
		} else if (currentCard.card.kind === "sentence_cloze") {
			correct = gradeSentenceCloze(
				typedAnswer,
				currentCard.card.answer,
				data.course.languageProfile,
			);
		}

		isCorrect = correct;
		submitted = true;

		feedbackTimer = setTimeout(() => {
			submitRating(correct ? "good" : "again");
			advanceToNext();
		}, 1200);
	}

	function cancelFeedbackAndAdvance() {
		if (feedbackTimer) {
			clearTimeout(feedbackTimer);
			feedbackTimer = null;
		}
		const rating = isCorrect ? "good" : "again";
		submitRating(rating);
		advanceToNext();
	}

	// ---- Sentence-order interactions ----
	function selectBlock(index: number) {
		if (submitted) return;
		selectedIndices = [...selectedIndices, index];
	}

	function removeBlock(index: number) {
		if (submitted) return;
		selectedIndices = selectedIndices.filter((i) => i !== index);
	}

	// ---- Keyboard handling ----
	function handleKeydown(e: KeyboardEvent) {
		if (finished) return;

		if (autoGraded) {
			if (submitted) {
				if (e.key === " " || e.key === "Enter") {
					e.preventDefault();
					cancelFeedbackAndAdvance();
				}
				return;
			}

			if (e.key === "Enter") {
				e.preventDefault();
				if (
					currentCard?.card.kind === "sentence_order" &&
					allBlocksSelected
				) {
					submitAutoGraded();
				} else if (currentCard?.card.kind === "sentence_cloze") {
					submitAutoGraded();
				}
			}
		} else {
			if (!showAnswer) {
				if (e.key === " " || e.key === "Enter") {
					e.preventDefault();
					showAnswer = true;
				}
			} else {
				switch (e.key) {
					case "1":
						rateThis("again");
						break;
					case "2":
						rateThis("hard");
						break;
					case "3":
						rateThis("good");
						break;
					case "4":
						rateThis("easy");
						break;
				}
			}
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="page">
	<a class="back" href="{base}/app/courses/{data.course.id}"
		>← Back to {data.course.title}</a
	>

	{#if dueLength === 0 || finished}
		<section class="done-state">
			<h1>All done! 🎉</h1>
			<p>No more cards due for review.</p>
			<a class="button primary" href="{base}/app/courses/{data.course.id}"
				>Back to course</a
			>
		</section>
	{:else}
		<section class="review-area">
			<div class="progress">
				{currentIndex + 1} / {dueLength}
			</div>

			<!-- ── Reveal cards: recognition / recall / reading_recognition ── -->
			{#if !autoGraded}
				<div class="card" class:flipped={showAnswer}>
					<div class="card-front">
						<span class="kind"
							>{currentCard ? labelForCardKind(currentCard.card.kind) : ""}</span
						>
						<p class="prompt">{currentCard?.card.prompt}</p>
						{#if !showAnswer}
							<button class="reveal-btn" onclick={() => (showAnswer = true)}>
								Show answer (Space)
							</button>
						{/if}
					</div>

					<div class="card-back">
						<span class="kind">Answer</span>
						{#if isJapanese && display.showFurigana && currentCard?.note.furigana}
							<p class="answer">
								{#each parseFuriganaMarkup(currentCard.note.furigana) as token}
									{#if token.type === "ruby"}
										<ruby>{token.base}<rt>{token.reading}</rt></ruby>
									{:else}
										{token.text}
									{/if}
								{/each}
							</p>
						{:else}
							<p class="answer">{currentCard?.card.answer}</p>
						{/if}
						{#if currentCard?.note.reading && (!isJapanese || display.showHiragana)}
							<p class="reading">{currentCard.note.reading}</p>
						{/if}
						{#if currentCard?.note.example && display.showExamples}
							<p class="example">{currentCard.note.example}</p>
							{#if currentCard?.note.exampleTranslation}
								<p class="example-trans"
									>{currentCard.note.exampleTranslation}</p
								>
							{/if}
						{/if}
					</div>
				</div>

				{#if showAnswer}
					<div class="ratings">
						<button class="rating again" onclick={() => rateThis("again")}>
							<span class="key">1</span>
							<span class="label">Again</span>
						</button>
						<button class="rating hard" onclick={() => rateThis("hard")}>
							<span class="key">2</span>
							<span class="label">Hard</span>
						</button>
						<button class="rating good" onclick={() => rateThis("good")}>
							<span class="key">3</span>
							<span class="label">Good</span>
						</button>
						<button class="rating easy" onclick={() => rateThis("easy")}>
							<span class="key">4</span>
							<span class="label">Easy</span>
						</button>
					</div>
				{/if}

				<!-- ── Sentence Order ── -->
			{:else if currentCard?.card.kind === "sentence_order"}
				<div class="card auto-graded">
					<span class="kind">{labelForCardKind(currentCard.card.kind)}</span>

					{#if submitted}
						<!-- Feedback -->
						<div
							class="feedback"
							class:correct={isCorrect}
							class:incorrect={!isCorrect}
						>
							<span class="result-icon">{isCorrect ? "✓" : "✗"}</span>
							<span class="result-label"
								>{isCorrect ? "Correct!" : "Incorrect"}</span>
						</div>
						<p class="correct-sentence">{currentCard.card.answer}</p>
						<p class="continue-hint">Press Space or Enter to continue</p>
					{:else}
						<!-- Prompt -->
						<p class="prompt">{currentCard.card.prompt}</p>
						<p class="hint">Arrange the words below to form the sentence.</p>

						<!-- Build area -->
						<div class="build-area">
							{#if selectedIndices.length === 0}
								<span class="build-placeholder"
									>Click words below to build the sentence…</span
								>
							{:else}
								{#each selectedIndices as index (index)}
									<button
										class="block selected"
										onclick={() => removeBlock(index)}
									>
										{wordBlocks?.[index] ?? ""}
									</button>
								{/each}
							{/if}
						</div>

						<!-- Available blocks -->
						<div class="word-blocks">
							{#each availableBlocks as index (index)}
								<button
									class="block"
									onclick={() => selectBlock(index)}
								>
									{wordBlocks?.[index] ?? ""}
								</button>
							{/each}
						</div>

						<button
							class="submit-btn"
							disabled={!allBlocksSelected}
							onclick={submitAutoGraded}
						>
							Submit
							{#if !allBlocksSelected}
								<span class="submit-hint">
									({selectedIndices.length}/{shuffledIndices.length} placed)
								</span>
							{/if}
						</button>
					{/if}
				</div>

				<!-- ── Sentence Cloze ── -->
			{:else if currentCard?.card.kind === "sentence_cloze"}
				<div class="card auto-graded">
					<span class="kind">{labelForCardKind(currentCard.card.kind)}</span>

					{#if submitted}
						<!-- Feedback -->
						<div
							class="feedback"
							class:correct={isCorrect}
							class:incorrect={!isCorrect}
						>
							<span class="result-icon">{isCorrect ? "✓" : "✗"}</span>
							<span class="result-label"
								>{isCorrect ? "Correct!" : "Incorrect"}</span>
						</div>
						{#if !isCorrect}
							<p class="correct-answer"
								>Answer: <strong>{currentCard.card.answer}</strong></p
							>
						{/if}
						<p class="continue-hint">Press Space or Enter to continue</p>
					{:else}
						<!-- Cloze sentence -->
						<p class="cloze-sentence">
							{#if clozeSplit?.found}
								<span class="cloze-before">{clozeSplit.before}</span>
								<span class="cloze-blank">___</span>
								<span class="cloze-after">{clozeSplit.after}</span>
							{:else}
								{currentCard.card.prompt}
							{/if}
						</p>

						<!-- Translation hint -->
						{#if (currentCard.card.extra as Record<string, unknown> | undefined)?.translation}
							<p class="hint">
								{(currentCard.card.extra as Record<string, unknown>)
									.translation as string}
							</p>
						{/if}

						<input
							type="text"
							class="cloze-input"
							bind:value={typedAnswer}
							placeholder="Type the missing word…"
							onkeydown={(e) => {
								if (e.key === "Enter") {
									e.preventDefault();
									if (typedAnswer.trim()) submitAutoGraded();
								}
							}}
						/>

						<button
							class="submit-btn"
							disabled={!typedAnswer.trim()}
							onclick={submitAutoGraded}
						>
							Submit
						</button>
					{/if}
				</div>
			{/if}
		</section>
	{/if}

	<!-- Hidden form for server-side rating submission -->
	<form
		id="review-form"
		method="POST"
		action="?/rate"
		use:enhance
		style="display:none"
	>
		<input
			type="hidden"
			name="cardId"
			value={currentCard?.card.id ?? ""}
		/>
		<input id="rating-input" type="hidden" name="rating" value="" />
	</form>
</main>

<style>
	.page {
		max-width: 880px;
		margin: 0 auto;
		padding: clamp(1.5rem, 4vw, 3rem) 1rem;
	}

	.back {
		font-size: 0.85rem;
		color: var(--c-text-sub, #666);
		text-decoration: none;
		display: inline-block;
		margin-bottom: 1rem;
	}

	.back:hover {
		text-decoration: underline;
	}

	.done-state {
		text-align: center;
		padding: 4rem 1rem;
	}

	.done-state h1 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
	}

	.done-state p {
		color: var(--c-text-sub, #666);
		margin-bottom: 1.5rem;
	}

	.button {
		display: inline-block;
		padding: 0.6rem 1.5rem;
		border-radius: 8px;
		text-decoration: none;
		font-weight: 600;
		font-size: 0.95rem;
		cursor: pointer;
		border: none;
	}

	.button.primary {
		background: var(--c-accent, #6366f1);
		color: var(--accent-text);
	}

	.button.primary:hover {
		opacity: 0.9;
	}

	.review-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
	}

	.progress {
		font-size: 0.85rem;
		color: var(--c-text-sub, #888);
	}

	/* ── Shared card base ── */
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
		color: var(--c-accent, #6366f1);
	}

	.reading {
		font-size: 0.95rem;
		color: var(--c-text-sub, #888);
		margin: 0;
	}

	.example {
		font-size: 0.9rem;
		color: var(--c-text-sub, #666);
		margin: 0.25rem 0 0;
		font-style: italic;
	}

	.example-trans {
		font-size: 0.85rem;
		color: var(--c-text-sub, #888);
		margin: 0;
	}

	.reveal-btn {
		margin-top: 0.5rem;
		padding: 0.5rem 1.5rem;
		border: 1px solid var(--c-border, #ccc);
		border-radius: 8px;
		background: var(--c-bg, white);
		cursor: pointer;
		font-size: 0.9rem;
		font-family: inherit;
		color: var(--c-text, #333);
	}

	.reveal-btn:hover {
		background: var(--c-bg-sub, #f5f5f5);
	}

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
		color: var(--bg);
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
		background: var(--danger);
	}

	.hard {
		background: var(--warning);
	}

	.good {
		background: var(--success);
	}

	.easy {
		background: var(--info);
	}

	/* ── Auto-graded card ── */
	.card.auto-graded {
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

	/* ── Sentence order blocks ── */
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
		background: var(--c-bg, white);
		cursor: pointer;
		font-size: 1.1rem;
		font-family: inherit;
		color: var(--c-text, #333);
		transition:
			background 0.15s,
			border-color 0.15s;
		user-select: none;
	}

	.block:hover {
		background: var(--c-bg-sub, #f0f0f0);
		border-color: var(--c-accent, #6366f1);
	}

	.block.selected {
		background: var(--c-accent, #6366f1);
		color: var(--accent-text);
		border-color: var(--c-accent, #6366f1);
	}

	.block.selected:hover {
		opacity: 0.85;
	}

	.submit-btn {
		padding: 0.6rem 2rem;
		border: none;
		border-radius: 8px;
		background: var(--c-accent, #6366f1);
		color: var(--accent-text);
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

	/* ── Cloze input ── */
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
		border-bottom: 2px solid var(--c-accent, #6366f1);
		color: var(--c-accent, #6366f1);
		font-weight: 700;
		padding: 0 0.3rem;
		margin: 0 0.2rem;
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
		transition: border-color 0.15s;
	}

	.cloze-input:focus {
		border-color: var(--c-accent, #6366f1);
	}

	/* ── Feedback ── */
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
		color: var(--success);
	}

	.feedback.incorrect {
		background: rgba(191, 97, 106, 0.18);
		color: var(--danger);
	}

	.result-icon {
		font-size: 1.3rem;
	}

	.correct-sentence {
		font-size: 1.3rem;
		font-weight: 600;
		text-align: center;
		margin: 0;
		color: var(--c-accent, #6366f1);
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

	.hint {
		font-size: 0.85rem;
		color: var(--c-text-sub, #888);
		margin: 0;
		text-align: center;
	}

	@media (max-width: 640px) {
		.ratings {
			grid-template-columns: repeat(2, 1fr);
		}

		.card-front,
		.card-back,
		.card.auto-graded {
			padding: 1.25rem;
		}

		.submit-btn,
		.reveal-btn,
		.button.primary {
			width: 100%;
		}
	}
</style>
