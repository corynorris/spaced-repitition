<script lang="ts">
	let {
		text,
	}: {
		text: string;
	} = $props();
</script>

<p class="eyebrow">{text}</p>

<style>
	.eyebrow {
		color: var(--c-accent, #88c0d0);
		font-size: 0.78rem;
		font-weight: 700;
		letter-spacing: 0.08em;
		margin: 0 0 0.75rem;
		text-transform: uppercase;
	}
</style>
