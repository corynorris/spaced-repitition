<script lang="ts">
	let {
		name,
		type = "text",
		value = "",
		placeholder = "",
		required = false,
		maxlength,
		autocomplete,
		oninput,
		onchange,
	}: {
		name: string;
		type?: string;
		value?: string;
		placeholder?: string;
		required?: boolean;
		maxlength?: number;
		autocomplete?: string;
		oninput?: (e: Event) => void;
		onchange?: (e: Event) => void;
	} = $props();

	let current = $state(value);
	$effect(() => { current = value; });
</script>

<input
	{name}
	{type}
	bind:value={current}
	{placeholder}
	{required}
	{maxlength}
	autocomplete={autocomplete as any}
	{oninput}
	{onchange}
/>

<style>
	input {
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		font: inherit;
		min-height: 2.75rem;
		padding: 0.65rem 0.75rem;
		background: var(--c-bg);
		color: var(--c-text);
		width: 100%;
	}

	input:focus {
		outline: none;
		border-color: var(--c-accent, #88c0d0);
		box-shadow: 0 0 0 3px rgba(136, 192, 208, 0.2);
	}
</style>
