<script lang="ts">
	import Label from "$lib/client/atoms/Label.svelte";
	import Input from "$lib/client/atoms/Input.svelte";
	import Textarea from "$lib/client/atoms/Textarea.svelte";
	import Select from "$lib/client/atoms/Select.svelte";
	import ErrorBanner from "$lib/client/atoms/ErrorBanner.svelte";
	import type { Snippet } from "svelte";

	let {
		label,
		optional = false,
		error,
		children,
	}: {
		label: string;
		optional?: boolean;
		error?: string;
		children: Snippet;
	} = $props();
</script>

<label class="form-field">
	<Label text={label} {optional} />
	{@render children()}
	<ErrorBanner message={error} />
</label>

<style>
	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
		color: var(--c-text-sub);
		font-size: 0.9rem;
		font-weight: 700;
	}
</style>
