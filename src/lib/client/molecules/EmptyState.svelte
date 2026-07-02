<script lang="ts">
	let {
		title,
		description,
	}: {
		title: string;
		description: string;
	} = $props();
</script>

<section class="empty-state">
	<h2>{title}</h2>
	<p>{description}</p>
</section>

<style>
	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
		color: var(--c-text-sub, #666);
	}

	h2 {
		color: var(--c-text);
		margin: 0 0 0.5rem;
	}

	p {
		color: var(--c-text-sub);
		line-height: 1.6;
		margin: 0;
	}
</style>
