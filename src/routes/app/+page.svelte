<script lang="ts">
  import { base } from "$app/paths";

  let { data } = $props();
</script>

<main class="page">
  <section class="page-header">
    <div>
      <p class="eyebrow">Dashboard</p>
      <h1>Your courses</h1>
    </div>
    <a class="button primary" href="{base}/app/courses/new">New course</a>
  </section>

  {#if data.courses.length === 0}
    <section class="empty-state">
      <h2>No courses yet</h2>
      <p>Create your first course to start building your vocabulary.</p>
    </section>
  {:else}
    <section class="course-grid">
      {#each data.courses as course}
        <a class="course-card" href="{base}/app/courses/{course.id}">
          <h3>{course.title}</h3>
          {#if course.description}
            <p class="desc">{course.description}</p>
          {/if}
          <div class="langs">
            {#if course.sourceLanguage}
              <span class="lang">{course.sourceLanguage}</span>
            {/if}
            {#if course.sourceLanguage && course.targetLanguage}
              <span class="arrow">→</span>
            {/if}
            {#if course.targetLanguage}
              <span class="lang">{course.targetLanguage}</span>
            {/if}
          </div>
          <div class="stats">
            <span>
              {course.stats.dueCards} due
            </span>
            <span>
              {course.stats.totalCards} cards
            </span>
            <span>
              {course.stats.reviewedToday} today
            </span>
          </div>
        </a>
      {/each}
    </section>
  {/if}
</main>

<style>
  .page {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 2rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .eyebrow {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--c-text-sub, #666);
    margin: 0 0 0.25rem;
  }

  h1 {
    margin: 0;
    font-size: 1.75rem;
  }

  .button {
    display: inline-block;
    padding: 0.5rem 1.25rem;
    border-radius: 8px;
    text-decoration: none;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
  }

  .button.primary {
    background: var(--c-accent, #6366f1);
    color: white;
  }

  .button.primary:hover {
    opacity: 0.9;
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--c-text-sub, #666);
  }

  .course-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 1rem;
  }

  .course-card {
    display: block;
    padding: 1.25rem;
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 12px;
    text-decoration: none;
    color: inherit;
    transition: box-shadow 0.15s, border-color 0.15s;
  }

  .course-card:hover {
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
    border-color: var(--c-accent, #6366f1);
  }

  .course-card h3 {
    margin: 0 0 0.4rem;
    font-size: 1.1rem;
  }

  .desc {
    font-size: 0.85rem;
    color: var(--c-text-sub, #666);
    margin: 0 0 0.6rem;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .langs {
    font-size: 0.8rem;
    color: var(--c-text-sub, #888);
    margin-bottom: 0.75rem;
  }

  .lang {
    background: var(--c-bg-sub, #f0f0f0);
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
  }

  .arrow {
    margin: 0 0.3rem;
  }

  .stats {
    display: flex;
    gap: 1rem;
    font-size: 0.8rem;
    color: var(--c-text-sub, #888);
  }

  .stats span:first-child {
    color: var(--c-accent, #6366f1);
    font-weight: 600;
  }
</style>
