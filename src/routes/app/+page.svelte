<script lang="ts">
  import { base } from "$app/paths";
</script>

<main class="page">
  <section class="page-header">
    <div>
      <p class="eyebrow">Dashboard</p>
      <h1>Your courses</h1>
    </div>
    <a class="button primary" href="{base}/app/courses/new">New course</a>
  </section>

  <section class="empty-state">
    <h2>No courses yet</h2>
    <p>
      The first implementation milestone will wire course persistence, then note
      entry, generated cards, and FSRS review queues.
    </p>
  </section>
</main>
