<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";
import BackLink from "$lib/client/molecules/BackLink.svelte";
import Button from "$lib/client/atoms/Button.svelte";
import Input from "$lib/client/atoms/Input.svelte";
import Select from "$lib/client/atoms/Select.svelte";
import Textarea from "$lib/client/atoms/Textarea.svelte";
import ErrorBanner from "$lib/client/atoms/ErrorBanner.svelte";
import FormField from "$lib/client/molecules/FormField.svelte";
import SectionHeading from "$lib/client/SectionHeading.svelte";
import PageHeading from "$lib/client/PageHeading.svelte";
import ModePicker from "$lib/client/organisms/ModePicker.svelte";
import CourseForm from "$lib/client/organisms/CourseForm.svelte";
import AiCoursePreview from "$lib/client/organisms/AiCoursePreview.svelte";

let { form } = $props();
const f = form as any;

const initial = f;
let mode = $state<"ai" | "manual" | null>(initial?.mode ?? null);
let languageProfile = $state(initial?.values?.languageProfile ?? "generic");
let targetLanguage = $state(initial?.values?.targetLanguage ?? "");
let sourceLanguage = $state(initial?.values?.sourceLanguage ?? "English");
let aiTopic = $state("");
let aiLoading = $state(false);
let aiError = $state("");
let aiPreview = $state<any>(null);

const isJapanese = $derived(languageProfile === "japanese");

$effect(() => {
	if (languageProfile === "japanese" && !targetLanguage) {
		targetLanguage = "Japanese";
	}
});

function chooseMode(nextMode: "ai" | "manual") {
	mode = nextMode;
	aiError = "";
	aiPreview = null;
}

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
  <BackLink href="{base}/app" label="Back to dashboard" />

  <PageHeading eyebrow="Create course" title="Start with AI or build it manually." />

  <ModePicker {mode} onchoose={chooseMode} />

  {#if mode === "manual"}
    <CourseForm
      {languageProfile}
      {sourceLanguage}
      {targetLanguage}
      {isJapanese}
      formValues={f?.values}
      formErrors={f?.errors}
    />
  {:else if mode === "ai"}
    <section class="panel">
      <SectionHeading
        title="AI course generator"
        description="Set the language context, then describe the course you want."
      />

      <div class="row">
        <FormField label="Language profile">
          <Select name="languageProfile" value={languageProfile} onchange={(e) => languageProfile = (e.target as HTMLSelectElement).value}>
            <option value="generic">Generic</option>
            <option value="japanese">Japanese</option>
          </Select>
        </FormField>

        <FormField label="Source language">
          <Input
            name="sourceLanguage"
            maxlength={80}
            placeholder="e.g. English"
            value={sourceLanguage}
            oninput={(e) => sourceLanguage = (e.target as HTMLInputElement).value}
          />
        </FormField>
      </div>

      <FormField label={isJapanese ? "Target language" : "Target subject/language"}>
        <Input
          name="targetLanguage"
          maxlength={80}
          placeholder={isJapanese ? "Japanese" : "e.g. Spanish, anatomy, calculus"}
          value={targetLanguage}
          oninput={(e) => targetLanguage = (e.target as HTMLInputElement).value}
        />
      </FormField>

      <FormField label="Course request">
        <Textarea
          name="aiTopic"
          rows={4}
          maxlength={300}
          placeholder={isJapanese
            ? "e.g. beginner Japanese for ordering at restaurants"
            : "e.g. intro anatomy terms for first-year nursing"}
          value={aiTopic}
          oninput={(e) => aiTopic = (e.target as HTMLTextAreaElement).value}
        />
      </FormField>

      <Button disabled={aiLoading || !aiTopic} onclick={generateAiCourse}>
        {aiLoading ? "Generating..." : "Generate preview"}
      </Button>

      <ErrorBanner message={aiError || f?.aiError} />

      {#if aiPreview}
        <AiCoursePreview preview={aiPreview} />
      {/if}
    </section>
  {/if}
</main>

<style>
  .page {
    max-width: 920px;
    margin: 0 auto;
    padding: clamp(1.5rem, 4vw, 3rem) 1rem;
  }

  .panel {
    background: rgba(59, 66, 82, 0.78);
    border: 1px solid var(--c-border);
    border-radius: 8px;
    display: grid;
    gap: 1.25rem;
    padding: 1rem;
  }

  .row {
    display: grid;
    gap: 1rem;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  @media (max-width: 640px) {
    .row {
      grid-template-columns: 1fr;
    }
  }
</style>
