<script lang="ts">
	import { enhance } from "$app/forms";
	import Button from "$lib/client/atoms/Button.svelte";
	import Input from "$lib/client/atoms/Input.svelte";
	import Select from "$lib/client/atoms/Select.svelte";
	import Textarea from "$lib/client/atoms/Textarea.svelte";
	import ErrorBanner from "$lib/client/atoms/ErrorBanner.svelte";
	import Eyebrow from "$lib/client/atoms/Eyebrow.svelte";
	import FormField from "$lib/client/molecules/FormField.svelte";
	import SectionHeading from "$lib/client/SectionHeading.svelte";
	import type { Snippet } from "svelte";

	let {
		languageProfile = "generic",
		sourceLanguage = "English",
		targetLanguage = "",
		isJapanese = false,
		formErrors,
		formValues,
		children,
	}: {
		languageProfile?: string;
		sourceLanguage?: string;
		targetLanguage?: string;
		isJapanese?: boolean;
		formErrors?: Record<string, string[]>;
		formValues?: Record<string, string>;
		children?: Snippet;
	} = $props();

	const f = $derived(formValues);
	const err = $derived(formErrors);
</script>

<form method="POST" use:enhance class="panel">
	<input type="hidden" name="languageProfile" value={languageProfile} />

	<SectionHeading
		title="Manual course details"
		description="These fields set up the course shell. Vocabulary comes next."
	/>

	{#if err}
		<ErrorBanner message={Object.values(err).flat().join(", ")} />
	{/if}

	<FormField label="Language profile">
		<Select name="languageProfile" value={languageProfile}>
			<option value="generic">Generic</option>
			<option value="japanese">Japanese</option>
		</Select>
	</FormField>

	<FormField label="Title" error={err?.title?.join(", ")}>
		<Input
			name="title"
			required
			maxlength={120}
			placeholder={isJapanese ? "e.g. Japanese N5 Vocabulary" : "e.g. Biology Terms"}
			value={f?.title ?? ""}
		/>
	</FormField>

	<FormField label="Description" optional>
		<Textarea
			name="description"
			maxlength={2000}
			rows={3}
			placeholder="What this course is for"
			value={f?.description ?? ""}
		/>
	</FormField>

	<div class="row">
		<FormField label="Source language" optional>
			<Input
				name="sourceLanguage"
				maxlength={80}
				placeholder="e.g. English"
				value={f?.sourceLanguage ?? sourceLanguage}
			/>
		</FormField>

		<FormField label={isJapanese ? "Target language" : "Target subject/language"}>
			<Input
				name="targetLanguage"
				maxlength={80}
				placeholder={isJapanese ? "Japanese" : "e.g. Spanish, anatomy, calculus"}
				value={f?.targetLanguage ?? targetLanguage}
			/>
		</FormField>
	</div>

	{@render children?.()}

	<Button variant="primary" type="submit">Create manual course</Button>
</form>

<style>
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
