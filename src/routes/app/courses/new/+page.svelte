<script lang="ts">
  import { enhance } from "$app/forms";
  import { base } from "$app/paths";

  let { form } = $props();
</script>

<main class="page">
  <a class="back" href="{base}/app">← Back to dashboard</a>
  <h1>New course</h1>

  <form method="POST" use:enhance class="form">
    <label>
      <span>Title</span>
      <input
        type="text"
        name="title"
        required
        maxlength="120"
        placeholder="e.g. Japanese N5 Vocabulary"
        value={(form as any)?.values?.title ?? ""}
      />
      {#if (form as any)?.errors?.title}
        <span class="error">{(form as any).errors.title.join(", ")}</span>
      {/if}
    </label>

    <label>
      <span>Description <em>(optional)</em></span>
      <textarea
        name="description"
        maxlength="2000"
        rows="3"
        placeholder="Brief description of this course"
      >{(form as any)?.values?.description ?? ""}</textarea>
    </label>

    <div class="row">
      <label>
        <span>Source language <em>(optional)</em></span>
        <input
          type="text"
          name="sourceLanguage"
          maxlength="80"
          placeholder="e.g. Japanese"
          value={(form as any)?.values?.sourceLanguage ?? ""}
        />
      </label>

      <label>
        <span>Target language <em>(optional)</em></span>
        <input
          type="text"
          name="targetLanguage"
          maxlength="80"
          placeholder="e.g. English"
          value={(form as any)?.values?.targetLanguage ?? ""}
        />
      </label>
    </div>

    <button type="submit" class="button primary">Create course</button>
  </form>
</main>

<style>
  .page {
    max-width: 520px;
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
    gap: 1.25rem;
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
