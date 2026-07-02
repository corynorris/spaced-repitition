<script lang="ts">
	import { base } from "$app/paths";

	let {
		href,
		label,
	}: {
		href: string;
		label: string;
	} = $props();
</script>

<a class="back" href={base + href}>← {label}</a>

<style>
	.back {
		font-size: 0.85rem;
		color: var(--c-text-sub, #666);
		text-decoration: none;
	}

	.back:hover {
		text-decoration: underline;
	}
</style>
