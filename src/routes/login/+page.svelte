<script lang="ts">
import { enhance } from "$app/forms";
import { base } from "$app/paths";
import { authClient } from "$lib/client/auth";

let { data, form } = $props();

function signInWithZitadel() {
	authClient.signIn.oauth2({ providerId: "zitadel" });
}
</script>

<main class="auth-page">
  <form class="auth-card" method="POST" use:enhance>
    <p class="eyebrow">Sign in</p>
    <h1>Welcome back</h1>

    {#if form?.error}
      <p class="error">{form.error}</p>
    {/if}

    {#if data.zitadelEnabled}
      <button class="button primary" type="button" onclick={signInWithZitadel}>
        Sign in with Zitadel
      </button>
      <div class="divider"><span>or</span></div>
    {/if}

    <label>
      Email
      <input name="email" type="email" autocomplete="email" required />
    </label>
    <label>
      Password
      <input name="password" type="password" autocomplete="current-password" required />
    </label>
    <button class="button primary" type="submit">Sign in</button>
    <p class="fine-print">
      Don't have an account? <a href="{base}/register">Create one</a>.
    </p>
  </form>
</main>
