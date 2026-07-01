<script lang="ts">
  import { base } from "$app/paths";
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

<div class="app-shell">
  <header class="topbar">
    <a class="brand" href="{base}/">Spaced Repetition</a>
    <nav>
      {#if data.user}
        <a href="{base}/app">Dashboard</a>
      {:else}
        <a href="{base}/login">Sign in</a>
        <a href="{base}/register">Create account</a>
      {/if}
    </nav>
  </header>

  {@render children()}
</div>
