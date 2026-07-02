<script lang="ts">
	let {
		title,
		description,
		selected = false,
		onclick,
	}: {
		title: string;
		description: string;
		selected?: boolean;
		onclick: () => void;
	} = $props();
</script>

<button type="button" class="mode-card" class:selected onclick={onclick}>
	<span class="mode-title">{title}</span>
	<span class="mode-copy">{description}</span>
</button>
