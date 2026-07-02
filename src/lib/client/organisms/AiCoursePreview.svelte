<script lang="ts">
	import { enhance } from "$app/forms";
	import Button from "$lib/client/atoms/Button.svelte";

	let {
		preview,
	}: {
		preview: {
			title: string;
			description?: string;
			languageProfile: string;
			sourceLanguage: string;
			targetLanguage: string;
			lessons: Array<{
				title: string;
				type: string;
				items?: unknown[];
				sentences?: unknown[];
				dialogue?: unknown[];
			}>;
		};
	} = $props();
</script>

<div class="preview">
	<div>
		<p class="eyebrow">Preview</p>
		<h3>{preview.title}</h3>
	</div>
	{#if preview.description}
		<p>{preview.description}</p>
	{/if}
	<dl class="preview-meta">
		<div>
			<dt>Profile</dt>
			<dd>{preview.languageProfile}</dd>
		</div>
		<div>
			<dt>Languages</dt>
			<dd>{preview.sourceLanguage} → {preview.targetLanguage}</dd>
		</div>
	</dl>
	<div class="lesson-list">
		{#each preview.lessons as lesson}
			<article class="lesson-preview">
				<h4>{lesson.title}</h4>
				<p>{lesson.type.replace("_", " ")}</p>
				{#if lesson.type === "vocabulary"}
					<span>{lesson.items?.length ?? 0} vocabulary items</span>
				{:else if lesson.type === "sentence_practice"}
					<span>{lesson.sentences?.length ?? 0} practice sentences</span>
				{:else}
					<span>{lesson.dialogue?.length ?? 0} dialogue turns</span>
				{/if}
			</article>
		{/each}
	</div>
	<form method="POST" action="?/createAiCourse" use:enhance>
		<input type="hidden" name="preview" value={JSON.stringify(preview)} />
		<Button variant="primary" type="submit">Create this AI course</Button>
	</form>
</div>

<style>
	.preview {
		background: var(--c-bg-elevated, #3b4252);
		border: 1px solid var(--c-border);
		border-radius: 8px;
		display: grid;
		gap: 1rem;
		padding: 1rem;
	}

	h3, h4, p {
		margin: 0;
	}

	.preview p {
		color: var(--c-text-sub);
		line-height: 1.5;
	}

	.eyebrow {
		color: var(--c-accent, #88c0d0);
		font-size: 0.78rem;
		font-weight: 700;
		letter-spacing: 0.08em;
		margin: 0 0 0.3rem;
		text-transform: uppercase;
	}

	.preview-meta {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.75rem;
		margin: 0;
	}

	.preview-meta div {
		border: 1px solid var(--c-border);
		border-radius: 8px;
		padding: 0.75rem;
	}

	dt {
		color: var(--c-text-sub);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
	}

	dd {
		margin: 0.25rem 0 0;
	}

	.lesson-list {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 180px), 1fr));
		gap: 0.75rem;
	}

	.lesson-preview {
		border: 1px solid var(--c-border);
		border-radius: 8px;
		padding: 0.75rem;
	}

	.lesson-preview h4 {
		margin-bottom: 0.35rem;
	}

	.lesson-preview p,
	.lesson-preview span {
		color: var(--c-text-sub);
		line-height: 1.5;
	}

	@media (max-width: 640px) {
		.preview-meta {
			grid-template-columns: 1fr;
		}
	}
</style>
