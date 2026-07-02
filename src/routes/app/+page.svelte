<script lang="ts">
import Button from "$lib/client/atoms/Button.svelte";
import EmptyState from "$lib/client/molecules/EmptyState.svelte";
import CourseCard from "$lib/client/organisms/CourseCard.svelte";

let { data } = $props();
</script>

<main class="page">
  <section class="page-header">
    <div>
      <p class="eyebrow">Dashboard</p>
      <h1>Your courses</h1>
    </div>
    <Button variant="primary" href="/app/courses/new">New course</Button>
  </section>

  {#if data.courses.length === 0}
    <EmptyState
      title="No courses yet"
      description="Create your first course to start building your vocabulary."
    />
  {:else}
    <section class="course-grid">
      {#each data.courses as course}
        <CourseCard showDue {course} />
      {/each}
    </section>
  {/if}
</main>

<style>
  .page {
    max-width: 1180px;
    margin: 0 auto;
    padding: clamp(1.5rem, 4vw, 3rem) 1rem;
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
    color: var(--c-accent, #88c0d0);
    font-size: 0.78rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0 0 0.25rem;
    text-transform: uppercase;
  }

  h1 {
    margin: 0;
    font-size: clamp(2rem, 4vw, 3.25rem);
    line-height: 1;
  }

  .course-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 290px), 1fr));
    gap: 1rem;
  }

  @media (max-width: 640px) {
    .page-header :global(.button) {
      width: 100%;
      text-align: center;
    }
  }
</style>
