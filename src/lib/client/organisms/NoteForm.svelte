<script lang="ts">
	import { enhance } from "$app/forms";
	import Button from "$lib/client/atoms/Button.svelte";
	import Input from "$lib/client/atoms/Input.svelte";
	import Textarea from "$lib/client/atoms/Textarea.svelte";
	import FormField from "$lib/client/molecules/FormField.svelte";

	let {
		isJapanese = false,
		formValues,
		formErrors,
	}: {
		isJapanese?: boolean;
		formValues?: Record<string, string>;
		formErrors?: Record<string, string[]>;
	} = $props();

	const f = $derived(formValues);
	const err = $derived(formErrors);
</script>

<form method="POST" use:enhance class="form">
	<div class="row">
		<FormField
			label={isJapanese ? "Kanji/Kana term" : "Term"}
			error={err?.term?.join(", ")}
		>
			<Input
				name="term"
				required
				maxlength={250}
				placeholder={isJapanese ? "e.g. 食べる" : "e.g. compute"}
				value={f?.term ?? ""}
			/>
		</FormField>

		<FormField
			label={isJapanese ? "Hiragana reading" : "Reading"}
			optional
		>
			<Input
				name="reading"
				maxlength={250}
				placeholder={isJapanese ? "e.g. たべる" : ""}
				value={f?.reading ?? ""}
			/>
		</FormField>
	</div>

	{#if isJapanese}
		<FormField label="Furigana" optional>
			<Input
				name="furigana"
				maxlength={1000}
				placeholder="e.g. 食[た]べる or 日本語[にほんご]"
				value={f?.furigana ?? ""}
			/>
		</FormField>
	{/if}

	<FormField
		label={isJapanese ? "English" : "Definition"}
		error={err?.definition?.join(", ")}
	>
		<Textarea
			name="definition"
			required
			maxlength={2000}
			rows={2}
			placeholder={isJapanese ? "e.g. to eat" : "e.g. to calculate"}
			value={f?.definition ?? ""}
		/>
	</FormField>

	<div class="row">
		<FormField label="Part of speech" optional>
			<Input
				name="partOfSpeech"
				maxlength={80}
				placeholder="e.g. verb"
				value={f?.partOfSpeech ?? ""}
			/>
		</FormField>

		<FormField label="Tags" optional>
			<Input
				name="tags"
				maxlength={200}
				placeholder="e.g. food, N5, common"
				value={f?.tags ?? ""}
			/>
		</FormField>
	</div>

	<FormField label="Example sentence" optional>
		<Input
			name="example"
			maxlength={2000}
			placeholder={isJapanese ? "e.g. 毎日朝ごはんを食べる" : ""}
			value={f?.example ?? ""}
		/>
	</FormField>

	<FormField label="Example translation" optional>
		<Input
			name="exampleTranslation"
			maxlength={2000}
			placeholder={isJapanese ? "e.g. I eat breakfast every day" : ""}
			value={f?.exampleTranslation ?? ""}
		/>
	</FormField>

	<FormField label="Personal notes" optional>
		<Textarea
			name="notes"
			maxlength={5000}
			rows={2}
			placeholder="Any mnemonics or extra context..."
			value={f?.notes ?? ""}
		/>
	</FormField>

	<Button variant="primary" type="submit">Add note</Button>
</form>

<style>
	.form {
		background: rgba(59, 66, 82, 0.78);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		display: grid;
		gap: 1rem;
		padding: 1rem;
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
</style>
