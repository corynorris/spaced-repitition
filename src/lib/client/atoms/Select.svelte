<script lang="ts">
	let {
		name,
		value = "",
		onchange,
		children,
	}: {
		name: string;
		value?: string;
		onchange?: (e: Event) => void;
		children: import("svelte").Snippet;
	} = $props();

	let current = $state(value);
	$effect(() => { current = value; });
</script>

<select {name} bind:value={current} {onchange}>
	{@render children()}
</select>

<style>
	select {
		appearance: none;
		background-image:
			linear-gradient(45deg, transparent 50%, var(--c-text-sub) 50%),
			linear-gradient(135deg, var(--c-text-sub) 50%, transparent 50%);
		background-position:
			calc(100% - 18px) 1.15rem,
			calc(100% - 12px) 1.15rem;
		background-size:
			6px 6px,
			6px 6px;
		background-repeat: no-repeat;
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		font: inherit;
		min-height: 2.75rem;
		padding: 0.65rem 2.25rem 0.65rem 0.75rem;
		background-color: var(--c-bg);
		color: var(--c-text);
		width: 100%;
	}

	select:focus {
		outline: none;
		border-color: var(--c-accent, #88c0d0);
		box-shadow: 0 0 0 3px rgba(136, 192, 208, 0.2);
	}
</style>
