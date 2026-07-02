<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";
import { labelForCardKind } from "$lib/cards/kinds";
import { isAutoGradedCardKind } from "$lib/cards/grading";
import BackLink from "$lib/client/molecules/BackLink.svelte";
import Button from "$lib/client/atoms/Button.svelte";
import ReviewCard from "$lib/client/organisms/ReviewCard.svelte";
import RatingButtons from "$lib/client/organisms/RatingButtons.svelte";
import SentenceOrderCard from "$lib/client/organisms/SentenceOrderCard.svelte";
import ClozeCard from "$lib/client/organisms/ClozeCard.svelte";
import type { Component } from "svelte";

let { data } = $props();

let currentIndex = $state(0);
let showAnswer = $state(false);
let finished = $state(false);

// Refs to auto-graded card component instances
let sentenceOrderRef = $state<SentenceOrderCard | null>(null);
let clozeCardRef = $state<ClozeCard | null>(null);

const currentCard = $derived(data.dueCards[currentIndex] ?? null);
const dueLength = $derived(data.dueCards.length);
const isJapanese = $derived(data.course.languageProfile === "japanese");
const display = $derived(data.course.displayOptions);

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
	sentenceOrderRef?.nextCard();
	clozeCardRef?.nextCard();
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

function handleAutoGradedRating(rating: string) {
	submitRating(rating);
	advanceToNext();
}

function handleKeydown(e: KeyboardEvent) {
	if (finished) return;

	// Let auto-graded cards handle their own keys first
	if (autoGraded) {
		if (sentenceOrderRef?.handleKeydown(e)) return;
		if (clozeCardRef?.handleKeydown(e)) return;
		return;
	}

	// Reveal cards
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
</script>

<svelte:window onkeydown={handleKeydown} />

<main class="page">
	<BackLink href="{base}/app/courses/{data.course.id}" label="Back to {data.course.title}" />

	{#if dueLength === 0 || finished}
		<section class="done-state">
			<h1>All done! 🎉</h1>
			<p>No more cards due for review.</p>
			<Button variant="primary" href="{base}/app/courses/{data.course.id}">Back to course</Button>
		</section>
	{:else}
		<section class="review-area">
			<div class="progress">
				{currentIndex + 1} / {dueLength}
			</div>

			<!-- Reveal cards -->
			{#if !autoGraded && currentCard}
				<ReviewCard
					card={currentCard.card}
					{isJapanese}
					{display}
					revealed={showAnswer}
					onreveal={() => (showAnswer = true)}
				/>

				{#if showAnswer}
					<RatingButtons onrating={rateThis} />
				{/if}

			<!-- Sentence Order -->
			{:else if currentCard?.card.kind === "sentence_order" && currentCard}
				<SentenceOrderCard
					bind:this={sentenceOrderRef}
					card={currentCard.card}
					wordBlocks={wordBlocks ?? []}
					{correctOrder}
					onrating={handleAutoGradedRating}
				/>

			<!-- Sentence Cloze -->
			{:else if currentCard?.card.kind === "sentence_cloze" && currentCard}
				<ClozeCard
					bind:this={clozeCardRef}
					card={currentCard.card}
					languageProfile={data.course.languageProfile}
					onrating={handleAutoGradedRating}
				/>
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

	@media (max-width: 640px) {
		:global(.button) {
			width: 100%;
		}
	}
</style>
