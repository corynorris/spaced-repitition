<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";

let { form } = $props();
const f = $derived(form as any);

let languageProfile = $state("generic");
let targetLanguage = $state("");
let sourceLanguage = $state("English");
let aiTopic = $state("");
let aiLoading = $state(false);
let aiError = $state("");
let aiPreview = $state<any>(null);

$effect(() => {
	if (languageProfile === "japanese" && !targetLanguage) {
		targetLanguage = "Japanese";
	}
});

async function generateAiCourse() {
	aiLoading = true;
	aiError = "";
	aiPreview = null;

	try {
		const response = await fetch(`${base}/api/ai/generate-course`, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				topic: aiTopic,
				sourceLanguage: sourceLanguage || "English",
				targetLanguage:
					targetLanguage ||
					(languageProfile === "japanese" ? "Japanese" : "Target language"),
				languageProfile,
			}),
		});
		const body = await response.json();
		if (!response.ok) throw new Error(body.error ?? "Generation failed");
		aiPreview = body.preview;
	} catch (err) {
		aiError = err instanceof Error ? err.message : "Generation failed";
	} finally {
		aiLoading = false;
	}
}
</script>

<main class="page">
  <a class="back" href="{base}/app">← Back to dashboard</a>
  <h1>New course</h1>

  <form method="POST" use:enhance class="form">
    <label>
      <span>Language profile</span>
      <select name="languageProfile" bind:value={languageProfile}>
        <option value="generic">Generic</option>
        <option value="japanese">Japanese</option>
      </select>
    </label>

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
        <span>Source language <em>(learner/base)</em></span>
        <input
          type="text"
          name="sourceLanguage"
          maxlength="80"
          placeholder="e.g. English"
          bind:value={sourceLanguage}
        />
      </label>

      <label>
        <span>Target language <em>(being studied)</em></span>
        <input
          type="text"
          name="targetLanguage"
          maxlength="80"
          placeholder="e.g. Japanese"
          bind:value={targetLanguage}
        />
      </label>
    </div>

    <button type="submit" class="button primary">Create course</button>
  </form>

  <section class="ai-panel">
    <h2>Generate with AI</h2>
    <label>
      <span>Course topic</span>
      <textarea
        rows="3"
        maxlength="300"
        bind:value={aiTopic}
        placeholder="e.g. beginner Japanese for ordering at restaurants"
      ></textarea>
    </label>
    <button class="button" type="button" disabled={aiLoading || !aiTopic} onclick={generateAiCourse}>
      {aiLoading ? "Generating..." : "Generate preview"}
    </button>
    {#if aiError || f?.aiError}
      <p class="error">{aiError || f.aiError}</p>
    {/if}

    {#if aiPreview}
      <div class="preview">
        <h3>{aiPreview.title}</h3>
        {#if aiPreview.description}
          <p>{aiPreview.description}</p>
        {/if}
        <ul>
          {#each aiPreview.lessons as lesson}
            <li>{lesson.title} ({lesson.type})</li>
          {/each}
        </ul>
        <form method="POST" action="?/createAiCourse" use:enhance>
          <input type="hidden" name="preview" value={JSON.stringify(aiPreview)} />
          <button class="button primary" type="submit">Create this course</button>
        </form>
      </div>
    {/if}
  </section>
</main>

<style>
  .page {
    max-width: 760px;
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
    gap: 1.25rem;
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

  .ai-panel {
    margin-top: 2rem;
    background: rgba(59, 66, 82, 0.78);
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 8px;
    display: grid;
    gap: 1rem;
    padding: 1rem;
  }

  .ai-panel h2 {
    margin: 0;
    font-size: 1.15rem;
  }

  .preview {
    border: 1px solid var(--c-border, #e0e0e0);
    border-radius: 8px;
    background: var(--c-bg-sub);
    padding: 1rem;
  }

  .preview h3 {
    margin: 0 0 0.5rem;
  }

  @media (max-width: 640px) {
    .row {
      grid-template-columns: 1fr;
    }

    .button {
      width: 100%;
    }
  }
</style>
