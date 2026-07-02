<script lang="ts">
  import { base } from "$app/paths";

  let { data } = $props();
</script>

<main class="page">
  <a class="back" href="{base}/app">← Dashboard</a>

  <section class="course-header">
    <h1>{data.course.title}</h1>
    {#if data.course.description}
      <p class="desc">{data.course.description}</p>
    {/if}
    <div class="langs">
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

  <section class="stats-bar">
    <span>{data.stats.totalCards} cards</span>
    <span>{data.stats.dueCards} due</span>
    <span>{data.stats.reviewedToday} reviewed today</span>
  </section>

  {#if data.notes.length === 0}
    <section class="empty-state">
      <h2>No notes yet</h2>
      <p>Add your first vocabulary note to get started.</p>
    </section>
  {:else}
    <section class="notes-list">
      {#each data.notes as note}
        <div class="note-item">
          <div class="note-main">
            <span class="term">{note.term}</span>
            {#if note.reading}
              <span class="reading">({note.reading})</span>
            {/if}
            <span class="definition">{note.definition}</span>
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
</main>

<style>
  .page {
    max-width: 720px;
    margin: 0 auto;
    padding: 2rem 1rem;
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
    margin: 1rem 0 1.5rem;
  }

  h1 {
    font-size: 1.75rem;
    margin: 0 0 0.5rem;
  }

  .desc {
    color: var(--c-text-sub, #666);
    margin: 0 0 0.5rem;
    line-height: 1.5;
  }

  .langs {
    font-size: 0.85rem;
    color: var(--c-text-sub, #888);
  }

  .lang {
    background: var(--c-bg-sub, #f0f0f0);
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
    color: white;
  }

  .button.accent {
    background: #10b981;
    color: white;
  }

  .button:hover {
    opacity: 0.9;
  }

  .done {
    color: #10b981;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .stats-bar {
    display: flex;
    gap: 1.5rem;
    font-size: 0.85rem;
    color: var(--c-text-sub, #888);
    padding: 0.75rem 0;
    border-bottom: 1px solid var(--c-border, #e0e0e0);
    margin-bottom: 1.5rem;
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
    padding: 0.75rem 1rem;
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 10px;
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
    padding: 0.1rem 0.4rem;
    border-radius: 4px;
    color: var(--c-text-sub, #777);
  }
</style>
