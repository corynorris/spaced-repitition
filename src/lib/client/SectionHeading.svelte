<script lang="ts">
	let {
		title,
		description,
	}: {
		title: string;
		description?: string;
	} = $props();
</script>

<div class="section-heading">
	<h2>{title}</h2>
	{#if description}
		<p>{description}</p>
	{/if}
</div>
