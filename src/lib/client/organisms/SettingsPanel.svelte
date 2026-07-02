<script lang="ts">
	import { enhance } from "$app/forms";
	import { labelForCardKind } from "$lib/cards/kinds";
	import Button from "$lib/client/atoms/Button.svelte";

	let {
		course,
		form,
	}: {
		course: {
			languageProfile: string;
			displayOptions: {
				showKanji?: boolean;
				showHiragana?: boolean;
				showFurigana?: boolean;
				showEnglish?: boolean;
				showTerm?: boolean;
				showReading?: boolean;
				showDefinition?: boolean;
				showExamples?: boolean;
				cardTypes: string[];
				activeCardTypes: string[];
			};
		};
		form?: Record<string, unknown>;
	} = $props();

	const isJapanese = $derived(course.languageProfile === "japanese");
	const display = $derived(course.displayOptions);
	const f = $derived(form as { settingsSaved?: boolean } | undefined);
</script>

<section class="settings-panel">
	<div class="panel-heading">
		<p class="eyebrow">Settings</p>
		<h2>Display and review</h2>
	</div>
	<form method="POST" action="?/updateSettings" use:enhance class="settings-form">
		<label>
			<span>Language profile</span>
			<select name="languageProfile" value={course.languageProfile}>
				<option value="generic">Generic</option>
				<option value="japanese">Japanese</option>
			</select>
		</label>

		<fieldset>
			<legend>Visible fields</legend>
			<div class="toggle-grid">
				{#if isJapanese}
					<label class="check"><input type="checkbox" name="showKanji" checked={display.showKanji} /> Kanji/Kana</label>
					<label class="check"><input type="checkbox" name="showHiragana" checked={display.showHiragana} /> Hiragana</label>
					<label class="check"><input type="checkbox" name="showFurigana" checked={display.showFurigana} /> Furigana</label>
					<label class="check"><input type="checkbox" name="showEnglish" checked={display.showEnglish} /> English</label>
				{:else}
					<label class="check"><input type="checkbox" name="showTerm" checked={display.showTerm} /> Term</label>
					<label class="check"><input type="checkbox" name="showReading" checked={display.showReading} /> Reading</label>
					<label class="check"><input type="checkbox" name="showDefinition" checked={display.showDefinition} /> Definition</label>
				{/if}
				<label class="check"><input type="checkbox" name="showExamples" checked={display.showExamples} /> Examples</label>
			</div>
		</fieldset>

		<fieldset>
			<legend>Active card types</legend>
			<div class="toggle-grid">
				{#each display.cardTypes as kind}
					<label class="check">
						<input
							type="checkbox"
							name="activeCardTypes"
							value={kind}
							checked={display.activeCardTypes.includes(kind)}
						/>
						{labelForCardKind(kind)}
					</label>
				{/each}
			</div>
		</fieldset>

		<div class="panel-actions">
			<Button type="submit">Save settings</Button>
			{#if f?.settingsSaved}
				<span class="saved">Saved</span>
			{/if}
		</div>
	</form>
</section>

<style>
	.settings-panel {
		border: 1px solid var(--c-border, #e0e0e0);
		border-radius: 8px;
		background: rgba(59, 66, 82, 0.78);
		padding: 1rem;
	}

	h2 {
		margin: 0 0 0.75rem;
		font-size: 1.05rem;
	}

	.panel-heading {
		margin-bottom: 0.8rem;
	}

	.panel-heading .eyebrow {
		margin-bottom: 0.2rem;
		color: var(--c-accent, #88c0d0);
		font-size: 0.78rem;
		font-weight: 700;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.settings-form {
		display: grid;
		gap: 0.75rem;
	}

	label {
		color: var(--c-text-sub);
		display: grid;
		font-size: 0.9rem;
		font-weight: 700;
		gap: 0.4rem;
	}

	label span {
		color: var(--c-text-sub);
	}

	select {
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		font: inherit;
		min-height: 2.75rem;
		padding: 0.65rem 0.75rem;
		background: var(--c-bg);
		color: var(--c-text);
	}

	fieldset {
		border: 1px solid var(--c-border);
		border-radius: 8px;
		display: grid;
		gap: 0.75rem;
		margin: 0;
		padding: 0.8rem;
	}

	legend {
		color: var(--c-text-sub);
		font-size: 0.78rem;
		font-weight: 700;
		padding: 0 0.35rem;
		text-transform: uppercase;
	}

	.toggle-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.5rem;
	}

	.check {
		display: inline-flex;
		align-items: center;
		background: rgba(46, 52, 64, 0.65);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		color: var(--c-text);
		gap: 0.5rem;
		font-weight: 500;
		min-height: 2.5rem;
		padding: 0.45rem 0.55rem;
	}

	.check input {
		accent-color: var(--c-accent, #88c0d0);
		min-height: auto;
	}

	.panel-actions {
		align-items: center;
		display: flex;
		gap: 0.75rem;
	}

	.saved {
		color: var(--c-success);
		font-size: 0.85rem;
		font-weight: 600;
	}

	@media (max-width: 640px) {
		.toggle-grid {
			grid-template-columns: 1fr;
		}

		.panel-actions {
			align-items: stretch;
			flex-direction: column;
		}
	}
</style>
