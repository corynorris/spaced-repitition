<script lang="ts">
  import { enhance } from "$app/forms";
  import { base } from "$app/paths";

  let { data } = $props();

  let currentIndex = $state(0);
  let showAnswer = $state(false);
  let finished = $state(false);

  const currentCard = $derived(data.dueCards[currentIndex] ?? null);

  function rateThis(rating: string) {
    showAnswer = false;

    // Submit via form
    const form = document.getElementById("review-form") as HTMLFormElement;
    const ratingInput = document.getElementById("rating-input") as HTMLInputElement;
    ratingInput.value = rating;
    form.requestSubmit();

    // Move to next card
    if (currentIndex < data.dueCards.length - 1) {
      currentIndex++;
    } else {
      finished = true;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (finished) return;
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
  <a class="back" href="{base}/app/courses/{data.course.id}"
    >← Back to {data.course.title}</a
  >

  {#if data.dueCards.length === 0 || finished}
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
        {currentIndex + 1} / {data.dueCards.length}
      </div>

      <div class="card" class:flipped={showAnswer}>
        <div class="card-front">
          <span class="kind">{currentCard?.card.kind === "recognition" ? "Term" : "Definition"}</span>
          <p class="prompt">{currentCard?.card.prompt}</p>
          {#if !showAnswer}
            <button class="reveal-btn" onclick={() => (showAnswer = true)}>
              Show answer (Space)
            </button>
          {/if}
        </div>

        <div class="card-back">
          <span class="kind">Answer</span>
          <p class="answer">{currentCard?.card.answer}</p>
          {#if currentCard?.note.reading}
            <p class="reading">{currentCard.note.reading}</p>
          {/if}
          {#if currentCard?.note.example}
            <p class="example">{currentCard.note.example}</p>
            {#if currentCard?.note.exampleTranslation}
              <p class="example-trans">{currentCard.note.exampleTranslation}</p>
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
    </section>
  {/if}

  <!-- Hidden form for submission -->
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
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem 1rem;
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
    color: white;
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

  .card {
    width: 100%;
    max-width: 480px;
    min-height: 220px;
    perspective: 800px;
  }

  .card-front,
  .card-back {
    padding: 2rem;
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 16px;
    background: var(--c-bg, white);
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
    font-size: 2rem;
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
    max-width: 480px;
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
    color: white;
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
</style>
