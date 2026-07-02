<script lang="ts">
import { base } from "$app/paths";
import AppShell from "$lib/client/templates/AppShell.svelte";
import "../styles.css";

let { children, data } = $props();
</script>

<svelte:head>
  <title>Spaced Repetition</title>
  <meta
    name="description"
    content="A vocabulary-focused spaced repetition app for building durable memory."
  />
</svelte:head>

<AppShell user={data.user}>
  {@render children()}
</AppShell>
