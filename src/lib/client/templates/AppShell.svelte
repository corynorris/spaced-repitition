<script lang="ts">
	import Topbar from "$lib/client/organisms/Topbar.svelte";
	import type { Snippet } from "svelte";

	let {
		user,
		children,
	}: {
		user: { id: string; email: string; name?: string | null } | null;
		children: Snippet;
	} = $props();
</script>

<div class="app-shell">
	<Topbar {user} />
	{@render children()}
</div>

<style>
	.app-shell {
		min-height: 100vh;
	}
</style>
