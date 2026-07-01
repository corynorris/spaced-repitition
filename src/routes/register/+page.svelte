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
    <p class="eyebrow">Create account</p>
    <h1>Start learning</h1>

    {#if form?.error}
      <p class="error">{form.error}</p>
    {/if}

    {#if data.zitadelEnabled}
      <button class="button primary" type="button" onclick={signInWithZitadel}>
        Sign up with Zitadel
      </button>
      <div class="divider"><span>or</span></div>
    {/if}

    <label>
      Name
      <input name="name" autocomplete="name" />
    </label>
    <label>
      Email
      <input name="email" type="email" autocomplete="email" required />
    </label>
    <label>
      Password
      <input name="password" type="password" autocomplete="new-password" required />
    </label>
    <button class="button primary" type="submit">Create account</button>
    <p class="fine-print">
      Already have an account? <a href="{base}/login">Sign in</a>.
    </p>
  </form>
</main>
