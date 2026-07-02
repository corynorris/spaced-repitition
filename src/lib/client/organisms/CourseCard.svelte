<script lang="ts">
	import { base } from "$app/paths";
	import Badge from "$lib/client/atoms/Badge.svelte";

	let {
		course,
		showDue = false,
	}: {
		course: {
			id: string;
			title: string;
			description?: string | null;
			sourceLanguage?: string | null;
			targetLanguage?: string | null;
			stats: {
				dueCards: number;
				totalCards: number;
				reviewedToday: number;
			};
		};
		showDue?: boolean;
	} = $props();
</script>

<a class="course-card" href="{base}/app/courses/{course.id}">
	<h3>{course.title}</h3>
	{#if course.description}
		<p class="desc">{course.description}</p>
	{/if}
	<div class="langs">
		{#if course.sourceLanguage}
			<Badge text={course.sourceLanguage} />
		{/if}
		{#if course.sourceLanguage && course.targetLanguage}
			<span class="arrow">→</span>
		{/if}
		{#if course.targetLanguage}
			<Badge text={course.targetLanguage} />
		{/if}
	</div>
	<div class="stats">
		{#if showDue}
			<span class="due">{course.stats.dueCards} due</span>
		{/if}
		<span>{course.stats.totalCards} cards</span>
		<span>{course.stats.reviewedToday} today</span>
	</div>
</a>

<style>
	.course-card {
		display: block;
		padding: 1.25rem;
		border: 1px solid var(--c-border, #e0e0e0);
		border-radius: 8px;
		background: rgba(59, 66, 82, 0.78);
		text-decoration: none;
		color: inherit;
		min-height: 12rem;
		transition:
			transform 0.15s,
			box-shadow 0.15s,
			border-color 0.15s;
	}

	.course-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
		border-color: var(--c-accent, #88c0d0);
	}

	h3 {
		margin: 0 0 0.4rem;
		font-size: 1.1rem;
	}

	.desc {
		font-size: 0.85rem;
		color: var(--c-text-sub, #666);
		margin: 0 0 0.6rem;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.langs {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.4rem;
		font-size: 0.8rem;
		color: var(--c-text-sub, #888);
		margin-bottom: 0.75rem;
	}

	.arrow {
		margin: 0 0.3rem;
	}

	.stats {
		display: flex;
		gap: 1rem;
		font-size: 0.8rem;
		color: var(--c-text-sub, #888);
	}

	.due {
		color: var(--c-accent, #88c0d0);
		font-weight: 600;
	}

	@media (max-width: 640px) {
		.stats {
			flex-wrap: wrap;
			gap: 0.6rem 1rem;
		}
	}
</style>
