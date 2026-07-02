<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";
import BackLink from "$lib/client/molecules/BackLink.svelte";
import EmptyState from "$lib/client/molecules/EmptyState.svelte";
import Button from "$lib/client/atoms/Button.svelte";
import ErrorBanner from "$lib/client/atoms/ErrorBanner.svelte";
import Skeleton from "$lib/client/atoms/Skeleton.svelte";
import Badge from "$lib/client/atoms/Badge.svelte";
import CourseStatBar from "$lib/client/organisms/CourseStatBar.svelte";
import NoteItem from "$lib/client/organisms/NoteItem.svelte";
import SettingsPanel from "$lib/client/organisms/SettingsPanel.svelte";
import PracticePreview from "$lib/client/organisms/PracticePreview.svelte";

let { data, form } = $props();
const f = $derived(form as any);
const isJapanese = $derived(data.course.languageProfile === "japanese");
const display = $derived(data.course.displayOptions);
let practiceMode = $state("sentences");
let practiceTopic = $state("");
let practiceLoading = $state(false);
let practiceError = $state("");
let practicePreview = $state<any>(null);
let discussQuestion = $state("");
let discussionLoading = $state(false);
let discussionError = $state("");
let discussionAnswer = $state("");

async function generatePractice() {
	practiceLoading = true;
	practiceError = "";
	practicePreview = null;

	try {
		const response = await fetch(`${base}/api/ai/generate-practice`, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				courseId: data.course.id,
				mode: practiceMode,
				topic: practiceTopic,
			}),
		});
		const body = await response.json();
		if (!response.ok) throw new Error(body.error ?? "Generation failed");
		practicePreview = body.preview;
	} catch (err) {
		practiceError = err instanceof Error ? err.message : "Generation failed";
	} finally {
		practiceLoading = false;
	}
}

async function askDiscussion() {
	discussionLoading = true;
	discussionError = "";
	discussionAnswer = "";

	try {
		const response = await fetch(`${base}/api/ai/discuss-content`, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				courseId: data.course.id,
				question: discussQuestion,
				scope: "course",
			}),
		});
		const body = await response.json();
		if (!response.ok) throw new Error(body.error ?? "Discussion failed");
		discussionAnswer = body.answer;
	} catch (err) {
		discussionError = err instanceof Error ? err.message : "Discussion failed";
	} finally {
		discussionLoading = false;
	}
}
</script>

<main class="page">
  <BackLink href="{base}/app" label="Dashboard" />

  <section class="course-header">
    <h1>{data.course.title}</h1>
    {#if data.course.description}
      <p class="desc">{data.course.description}</p>
    {/if}
    <div class="langs">
      <Badge text={data.course.languageProfile} />
      {#if data.course.sourceLanguage}
        <Badge text={data.course.sourceLanguage} />
      {/if}
      {#if data.course.sourceLanguage && data.course.targetLanguage}
        <span class="arrow">→</span>
      {/if}
      {#if data.course.targetLanguage}
        <Badge text={data.course.targetLanguage} />
      {/if}
    </div>
  </section>

  <section class="actions">
    <Button variant="primary" href="{base}/app/courses/{data.course.id}/notes/new">
      + Add note
    </Button>
    {#if data.stats.dueCards > 0}
      <Button variant="accent" href="{base}/app/courses/{data.course.id}/review">
        Study ({data.stats.dueCards} due)
      </Button>
    {:else if data.stats.totalCards > 0}
      <span class="done">All caught up! ✓</span>
    {/if}
  </section>

  <div class="course-layout">
    <section class="course-main">
      <CourseStatBar
        totalCards={data.stats.totalCards}
        dueCards={data.stats.dueCards}
        reviewedToday={data.stats.reviewedToday}
      />

      {#if data.notes.length === 0}
        <EmptyState
          title="No notes yet"
          description="Add your first vocabulary note to get started."
        />
      {:else}
        <section class="notes-list" aria-label="Course notes">
          {#each data.notes as note}
            <NoteItem {note} {isJapanese} {display} />
          {/each}
        </section>
      {/if}
    </section>

    <aside class="course-sidebar" aria-label="Course tools">
      <SettingsPanel course={data.course} form={f} />

      <section class="ai-panel">
        <h2>AI practice</h2>
        <div class="row controls">
          <label>
            <span>Mode</span>
            <select bind:value={practiceMode}>
              <option value="sentences">Sentences</option>
              <option value="conversation">Conversation</option>
            </select>
          </label>
          <label>
            <span>Topic <em>(optional)</em></span>
            <input type="text" maxlength="200" bind:value={practiceTopic} placeholder="e.g. at a cafe" />
          </label>
        </div>
        <Button disabled={practiceLoading || data.notes.length === 0} onclick={generatePractice}>
          {practiceLoading ? "Generating..." : "Generate preview"}
        </Button>
        {#if practiceError || f?.practiceError}
          <ErrorBanner message={practiceError || f.practiceError} onretry={generatePractice} />
        {/if}
        {#if f?.practiceSaved}
          <p class="saved">Practice lesson added.</p>
        {/if}
        {#if practiceLoading && !practicePreview}
          <Skeleton lines={4} />
        {/if}
        {#if practicePreview}
          <PracticePreview preview={practicePreview} />
        {/if}
      </section>

      <section class="ai-panel">
        <h2>Discuss this course</h2>
        <label>
          <span>Question</span>
          <textarea rows="3" maxlength="1000" bind:value={discussQuestion}></textarea>
        </label>
        <Button disabled={discussionLoading || !discussQuestion} onclick={askDiscussion}>
          {discussionLoading ? "Asking..." : "Ask"}
        </Button>
        {#if discussionError}
          <ErrorBanner message={discussionError} onretry={askDiscussion} />
        {/if}
        {#if discussionLoading && !discussionAnswer}
          <Skeleton lines={3} width="80%" />
        {/if}
        {#if discussionAnswer}
          <p class="discussion">{discussionAnswer}</p>
        {/if}
      </section>
    </aside>
  </div>
</main>

<style>
  .page {
    max-width: 1180px;
    margin: 0 auto;
    padding: clamp(1.5rem, 4vw, 3rem) 1rem;
  }

  .course-header {
    margin: 1rem 0 1.25rem;
  }

  h1 {
    font-size: clamp(2rem, 4vw, 3.25rem);
    line-height: 1;
    margin: 0 0 0.5rem;
  }

  .desc {
    color: var(--c-text-sub, #666);
    margin: 0 0 0.5rem;
    line-height: 1.5;
  }

  .langs {
    align-items: center;
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    font-size: 0.85rem;
    color: var(--c-text-sub, #888);
  }

  .arrow {
    margin: 0 0.3rem;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .done {
    color: var(--c-success);
    font-weight: 600;
    font-size: 0.9rem;
  }

  .course-layout {
    align-items: start;
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(300px, 360px);
    gap: 1.25rem;
  }

  .course-main,
  .course-sidebar {
    display: grid;
    gap: 1rem;
  }

  .course-sidebar {
    position: sticky;
    top: 5.25rem;
  }

  .notes-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .ai-panel {
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 8px;
    background: rgba(59, 66, 82, 0.78);
    padding: 1rem;
    display: grid;
    gap: 0.75rem;
  }

  .ai-panel h2 {
    margin: 0 0 0.75rem;
    font-size: 1.05rem;
  }

  label {
    color: var(--c-text-sub);
    display: grid;
    font-size: 0.9rem;
    font-weight: 700;
    gap: 0.4rem;
  }

  label span {
    color: var(--c-text-sub);
  }

  label span em {
    font-weight: 400;
    color: var(--c-text-muted);
  }

  input, textarea, select {
    border: 1px solid var(--c-border);
    border-radius: 0.5rem;
    font: inherit;
    min-height: 2.75rem;
    padding: 0.65rem 0.75rem;
    background: var(--c-bg);
    color: var(--c-text);
  }

  input:focus, textarea:focus, select:focus {
    outline: none;
    border-color: var(--c-accent, #88c0d0);
    box-shadow: 0 0 0 3px rgba(136, 192, 208, 0.2);
  }

  textarea {
    resize: vertical;
  }

  .controls {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }

  .saved {
    color: var(--c-success);
    font-size: 0.85rem;
    font-weight: 600;
  }

  .discussion {
    background: var(--c-bg-sub, #f0f0f0);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    padding: 1rem;
  }

  @media (max-width: 900px) {
    .course-layout {
      grid-template-columns: 1fr;
    }

    .course-sidebar {
      position: static;
    }
  }

  @media (max-width: 640px) {
    .actions :global(.button) {
      width: 100%;
    }
  }
</style>
