<script lang="ts">
import BackLink from "$lib/client/molecules/BackLink.svelte";
import NoteForm from "$lib/client/organisms/NoteForm.svelte";

let { data, form } = $props();
const f = $derived(form as any);
const isJapanese = $derived(data.course.languageProfile === "japanese");
</script>

<main class="page">
  <BackLink href="/app/courses/{data.course.id}" label="Back to {data.course.title}" />
  <h1>Add note</h1>

  <NoteForm
    {isJapanese}
    formValues={f?.values}
    formErrors={f?.errors}
  />
</main>

<style>
  .page {
    max-width: 820px;
    margin: 0 auto;
    padding: clamp(1.5rem, 4vw, 3rem) 1rem;
  }

  h1 {
    margin: 0.5rem 0 1.5rem;
    font-size: clamp(2rem, 4vw, 3rem);
    line-height: 1;
  }
</style>
