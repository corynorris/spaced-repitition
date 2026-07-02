<script lang="ts">
  import { enhance } from "$app/forms";
  import { base } from "$app/paths";

  let { data, form } = $props();
  const f = $derived(form as any);
</script>

<main class="page">
  <a class="back" href="{base}/app/courses/{data.course.id}"
    >← Back to {data.course.title}</a
  >
  <h1>Add note</h1>

  <form method="POST" use:enhance class="form">
    <div class="row">
      <label>
        <span>Term *</span>
        <input
          type="text"
          name="term"
          required
          maxlength="250"
          placeholder="e.g. 食べる"
          value={f?.values?.term ?? ""}
        />
        {#if f?.errors?.term}
          <span class="error">{f.errors.term.join(", ")}</span>
        {/if}
      </label>

      <label>
        <span>Reading <em>(optional)</em></span>
        <input
          type="text"
          name="reading"
          maxlength="250"
          placeholder="e.g. たべる"
          value={f?.values?.reading ?? ""}
        />
      </label>
    </div>

    <label>
      <span>Definition *</span>
      <textarea
        name="definition"
        required
        maxlength="2000"
        rows="2"
        placeholder="e.g. to eat"
      >{f?.values?.definition ?? ""}</textarea>
      {#if f?.errors?.definition}
        <span class="error">{f.errors.definition.join(", ")}</span>
      {/if}
    </label>

    <div class="row">
      <label>
        <span>Part of speech <em>(optional)</em></span>
        <input
          type="text"
          name="partOfSpeech"
          maxlength="80"
          placeholder="e.g. verb"
          value={f?.values?.partOfSpeech ?? ""}
        />
      </label>

      <label>
        <span>Tags <em>(comma-separated)</em></span>
        <input
          type="text"
          name="tags"
          maxlength="200"
          placeholder="e.g. food, N5, common"
          value={f?.values?.tags ?? ""}
        />
      </label>
    </div>

    <label>
      <span>Example sentence <em>(optional)</em></span>
      <input
        type="text"
        name="example"
        maxlength="2000"
        placeholder="e.g. 毎日朝ごはんを食べる"
        value={f?.values?.example ?? ""}
      />
    </label>

    <label>
      <span>Example translation <em>(optional)</em></span>
      <input
        type="text"
        name="exampleTranslation"
        maxlength="2000"
        placeholder="e.g. I eat breakfast every day"
        value={f?.values?.exampleTranslation ?? ""}
      />
    </label>

    <label>
      <span>Personal notes <em>(optional)</em></span>
      <textarea
        name="notes"
        maxlength="5000"
        rows="2"
        placeholder="Any mnemonics or extra context..."
      >{f?.values?.notes ?? ""}</textarea>
    </label>

    <button type="submit" class="button primary">Add note</button>
  </form>
</main>

<style>
  .page {
    max-width: 560px;
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

  h1 {
    margin: 0.5rem 0 1.5rem;
    font-size: 1.5rem;
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  label span {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--c-text, #333);
  }

  label span em {
    font-weight: 400;
    color: var(--c-text-sub, #888);
    font-size: 0.8rem;
  }

  input,
  textarea {
    padding: 0.6rem 0.75rem;
    border: 1px solid var(--c-border, #ccc);
    border-radius: 8px;
    font-size: 0.95rem;
    font-family: inherit;
    background: var(--c-bg, white);
    color: var(--c-text, #333);
  }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: var(--c-accent, #6366f1);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }

  .row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  @media (max-width: 480px) {
    .row {
      grid-template-columns: 1fr;
    }
  }

  .error {
    font-size: 0.8rem;
    color: #e53e3e;
  }

  .button {
    padding: 0.6rem 1.5rem;
    border-radius: 8px;
    border: none;
    font-weight: 600;
    font-size: 0.95rem;
    cursor: pointer;
    align-self: flex-start;
  }

  .button.primary {
    background: var(--c-accent, #6366f1);
    color: white;
  }

  .button.primary:hover {
    opacity: 0.9;
  }
</style>
