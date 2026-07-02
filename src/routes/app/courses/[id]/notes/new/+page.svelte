<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";

let { data, form } = $props();
const f = $derived(form as any);
const isJapanese = $derived(data.course.languageProfile === "japanese");
</script>

<main class="page">
  <a class="back" href="{base}/app/courses/{data.course.id}"
    >← Back to {data.course.title}</a
  >
  <h1>Add note</h1>

  <form method="POST" use:enhance class="form">
    <div class="row">
      <label>
        <span>{isJapanese ? "Kanji/Kana term" : "Term"} *</span>
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
        <span>{isJapanese ? "Hiragana reading" : "Reading"} <em>(optional)</em></span>
        <input
          type="text"
          name="reading"
          maxlength="250"
          placeholder="e.g. たべる"
          value={f?.values?.reading ?? ""}
        />
      </label>
    </div>

    {#if isJapanese}
      <label>
        <span>Furigana <em>(optional bracket markup)</em></span>
        <input
          type="text"
          name="furigana"
          maxlength="1000"
          placeholder="e.g. 食[た]べる or 日本語[にほんご]"
          value={f?.values?.furigana ?? ""}
        />
      </label>
    {/if}

    <label>
      <span>{isJapanese ? "English" : "Definition"} *</span>
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
    max-width: 820px;
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

  h1 {
    margin: 0.5rem 0 1.5rem;
    font-size: clamp(2rem, 4vw, 3rem);
    line-height: 1;
  }

  .form {
    background: rgba(59, 66, 82, 0.78);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    display: grid;
    gap: 1rem;
    padding: 1rem;
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
    color: var(--danger);
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
    color: var(--accent-text);
  }

  .button.primary:hover {
    opacity: 0.9;
  }
</style>
