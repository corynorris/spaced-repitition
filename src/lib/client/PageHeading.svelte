<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		eyebrow,
		title,
		children,
	}: {
		eyebrow?: string;
		title: string;
		children?: Snippet;
	} = $props();
</script>

<section class="page-heading">
	{#if eyebrow}
		<p class="eyebrow">{eyebrow}</p>
	{/if}
	<h1>{title}</h1>
	{@render children?.()}
</section>
