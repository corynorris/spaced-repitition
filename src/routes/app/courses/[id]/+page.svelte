<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";
import { labelForCardKind } from "$lib/cards/kinds";
import Skeleton from "$lib/client/Skeleton.svelte";
import { parseFuriganaMarkup } from "$lib/language/japanese";

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
  <a class="back" href="{base}/app">← Dashboard</a>

  <section class="course-header">
    <h1>{data.course.title}</h1>
    {#if data.course.description}
      <p class="desc">{data.course.description}</p>
    {/if}
    <div class="langs">
      <span class="lang">{data.course.languageProfile}</span>
      {#if data.course.sourceLanguage}
        <span class="lang">{data.course.sourceLanguage}</span>
      {/if}
      {#if data.course.sourceLanguage && data.course.targetLanguage}
        <span class="arrow">→</span>
      {/if}
      {#if data.course.targetLanguage}
        <span class="lang">{data.course.targetLanguage}</span>
      {/if}
    </div>
  </section>

  <section class="actions">
    <a class="button primary" href="{base}/app/courses/{data.course.id}/notes/new">
      + Add note
    </a>
    {#if data.stats.dueCards > 0}
      <a class="button accent" href="{base}/app/courses/{data.course.id}/review">
        Study ({data.stats.dueCards} due)
      </a>
    {:else if data.stats.totalCards > 0}
      <span class="done">All caught up! ✓</span>
    {/if}
  </section>

  <div class="course-layout">
    <section class="course-main">
      <section class="stats-bar" aria-label="Course stats">
        <span><strong>{data.stats.totalCards}</strong> cards</span>
        <span><strong>{data.stats.dueCards}</strong> due</span>
        <span><strong>{data.stats.reviewedToday}</strong> reviewed today</span>
      </section>

      {#if data.notes.length === 0}
        <section class="empty-state">
          <h2>No notes yet</h2>
          <p>Add your first vocabulary note to get started.</p>
        </section>
      {:else}
        <section class="notes-list" aria-label="Course notes">
          {#each data.notes as note}
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
                {:else if !isJapanese || display.showKanji}
                  <span class="term">{note.term}</span>
                {/if}
                {#if note.reading && (!isJapanese || display.showHiragana)}
                  <span class="reading">({note.reading})</span>
                {/if}
                {#if !isJapanese || display.showEnglish}
                  <span class="definition">{note.definition}</span>
                {/if}
              </div>
              {#if note.tags.length > 0}
                <div class="tags">
                  {#each note.tags as tag}
                    <span class="tag">{tag}</span>
                  {/each}
                </div>
              {/if}
            </div>
          {/each}
        </section>
      {/if}
    </section>

    <aside class="course-sidebar" aria-label="Course tools">
      <section class="settings-panel">
        <div class="panel-heading">
          <p class="eyebrow">Settings</p>
          <h2>Display and review</h2>
        </div>
        <form method="POST" action="?/updateSettings" use:enhance class="settings-form">
          <label>
            <span>Language profile</span>
            <select name="languageProfile" value={data.course.languageProfile}>
              <option value="generic">Generic</option>
              <option value="japanese">Japanese</option>
            </select>
          </label>

          <fieldset>
            <legend>Visible fields</legend>
            <div class="toggle-grid">
              {#if isJapanese}
                <label class="check"><input type="checkbox" name="showKanji" checked={display.showKanji} /> Kanji/Kana</label>
                <label class="check"><input type="checkbox" name="showHiragana" checked={display.showHiragana} /> Hiragana</label>
                <label class="check"><input type="checkbox" name="showFurigana" checked={display.showFurigana} /> Furigana</label>
                <label class="check"><input type="checkbox" name="showEnglish" checked={display.showEnglish} /> English</label>
              {:else}
                <label class="check"><input type="checkbox" name="showTerm" checked={display.showTerm} /> Term</label>
                <label class="check"><input type="checkbox" name="showReading" checked={display.showReading} /> Reading</label>
                <label class="check"><input type="checkbox" name="showDefinition" checked={display.showDefinition} /> Definition</label>
              {/if}
              <label class="check"><input type="checkbox" name="showExamples" checked={display.showExamples} /> Examples</label>
            </div>
          </fieldset>

          <fieldset>
            <legend>Active card types</legend>
            <div class="toggle-grid">
              {#each display.cardTypes as kind}
                <label class="check">
                  <input
                    type="checkbox"
                    name="activeCardTypes"
                    value={kind}
                    checked={display.activeCardTypes.includes(kind)}
                  />
                  {labelForCardKind(kind)}
                </label>
              {/each}
            </div>
          </fieldset>

          <div class="panel-actions">
            <button class="button" type="submit">Save settings</button>
            {#if f?.settingsSaved}
              <span class="saved">Saved</span>
            {/if}
          </div>
        </form>
      </section>

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
        <button class="button" type="button" disabled={practiceLoading || data.notes.length === 0} onclick={generatePractice}>
          {practiceLoading ? "Generating..." : "Generate preview"}
        </button>
        {#if practiceError || f?.practiceError}
          <div class="error-card">
            <p class="error">{practiceError || f.practiceError}</p>
            <button class="retry-btn" onclick={generatePractice}>Retry</button>
          </div>
        {/if}
        {#if f?.practiceSaved}
          <p class="saved">Practice lesson added.</p>
        {/if}
        {#if practiceLoading && !practicePreview}
          <Skeleton lines={4} />
        {/if}
        {#if practicePreview}
          <div class="preview">
            <h3>{practicePreview.title}</h3>
            {#if practicePreview.mode === "sentences"}
              <ul>
                {#each practicePreview.sentences as sentence}
                  <li>{sentence.target} - {sentence.translation}</li>
                {/each}
              </ul>
            {:else}
              {#if practicePreview.setting}<p>{practicePreview.setting}</p>{/if}
              <ul>
                {#each practicePreview.dialogue as turn}
                  <li>{turn.speaker}: {turn.target} - {turn.translation}</li>
                {/each}
              </ul>
            {/if}
            <form method="POST" action="?/createPractice" use:enhance>
              <input type="hidden" name="preview" value={JSON.stringify(practicePreview)} />
              <button class="button primary" type="submit">Add practice lesson</button>
            </form>
          </div>
        {/if}
      </section>

      <section class="ai-panel">
        <h2>Discuss this course</h2>
        <label>
          <span>Question</span>
          <textarea rows="3" maxlength="1000" bind:value={discussQuestion}></textarea>
        </label>
        <button class="button" type="button" disabled={discussionLoading || !discussQuestion} onclick={askDiscussion}>
          {discussionLoading ? "Asking..." : "Ask"}
        </button>
        {#if discussionError}
          <div class="error-card">
            <p class="error">{discussionError}</p>
            <button class="retry-btn" onclick={askDiscussion}>Retry</button>
          </div>
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

  .back {
    font-size: 0.85rem;
    color: var(--c-text-sub, #666);
    text-decoration: none;
  }

  .back:hover {
    text-decoration: underline;
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

  .lang {
    background: var(--c-bg-sub, #f0f0f0);
    border: 1px solid var(--c-border);
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
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

  .button {
    display: inline-block;
    padding: 0.5rem 1.25rem;
    border-radius: 8px;
    text-decoration: none;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
    border: none;
  }

  .button.primary {
    background: var(--c-accent, #6366f1);
    color: var(--accent-text);
  }

  .button.accent {
    background: var(--success);
    color: var(--bg);
  }

  .button:hover {
    opacity: 0.9;
  }

  .done {
    color: var(--success);
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

  .stats-bar {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.75rem;
    font-size: 0.85rem;
    color: var(--c-text-sub, #888);
  }

  .stats-bar span {
    background: rgba(59, 66, 82, 0.78);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    padding: 0.8rem;
  }

  .stats-bar strong {
    color: var(--c-text);
    display: block;
    font-size: 1.45rem;
    line-height: 1;
    margin-bottom: 0.25rem;
  }

  .settings-panel,
  .ai-panel {
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 8px;
    background: rgba(59, 66, 82, 0.78);
    padding: 1rem;
  }

  .settings-panel h2,
  .ai-panel h2 {
    margin: 0 0 0.75rem;
    font-size: 1.05rem;
  }

  .panel-heading {
    margin-bottom: 0.8rem;
  }

  .panel-heading .eyebrow {
    margin-bottom: 0.2rem;
  }

  .settings-form,
  .ai-panel {
    display: grid;
    gap: 0.75rem;
  }

  fieldset {
    border: 1px solid var(--c-border);
    border-radius: 8px;
    display: grid;
    gap: 0.75rem;
    margin: 0;
    padding: 0.8rem;
  }

  legend {
    color: var(--c-text-sub);
    font-size: 0.78rem;
    font-weight: 700;
    padding: 0 0.35rem;
    text-transform: uppercase;
  }

  .toggle-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.5rem;
  }

  .check {
    display: inline-flex;
    align-items: center;
    background: rgba(46, 52, 64, 0.65);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    color: var(--c-text);
    gap: 0.5rem;
    font-weight: 500;
    min-height: 2.5rem;
    padding: 0.45rem 0.55rem;
  }

  .check input {
    accent-color: var(--accent);
    min-height: auto;
  }

  .panel-actions {
    align-items: center;
    display: flex;
    gap: 0.75rem;
  }

  .row.controls {
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.75rem;
  }

  .saved {
    color: var(--success);
    font-size: 0.85rem;
    font-weight: 600;
  }

  .error {
    color: var(--danger);
    margin: 0;
  }

  .error-card {
    background: var(--c-danger-bg, #fef2f2);
    border: 1px solid var(--c-danger-sub, #fecaca);
    border-radius: 8px;
    padding: 0.75rem 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .retry-btn {
    padding: 0.3rem 0.8rem;
    border: 1px solid var(--danger, #dc2626);
    border-radius: 6px;
    background: transparent;
    color: var(--danger, #dc2626);
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .retry-btn:hover {
    background: var(--danger, #dc2626);
    color: var(--bg);
  }

  .preview,
  .discussion {
    background: var(--c-bg-sub, #f0f0f0);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    padding: 1rem;
  }

  .preview h3 {
    margin: 0 0 0.5rem;
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--c-text-sub, #666);
  }

  .notes-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

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

  .tag {
    font-size: 0.7rem;
    background: var(--c-bg-sub, #f0f0f0);
    border: 1px solid var(--c-border);
    padding: 0.1rem 0.4rem;
    border-radius: 4px;
    color: var(--c-text-sub, #777);
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
    .actions .button,
    .panel-actions .button {
      width: 100%;
    }

    .panel-actions {
      align-items: stretch;
      flex-direction: column;
    }

    .stats-bar,
    .toggle-grid {
      grid-template-columns: 1fr;
    }

    .definition::before {
      content: "";
      margin: 0;
    }
  }
</style>
