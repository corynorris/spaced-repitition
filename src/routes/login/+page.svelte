<script lang="ts">
import { enhance } from "$app/forms";
import AuthForm from "$lib/client/organisms/AuthForm.svelte";
import Button from "$lib/client/atoms/Button.svelte";
import Input from "$lib/client/atoms/Input.svelte";
import FormField from "$lib/client/molecules/FormField.svelte";

let { data, form } = $props();
</script>

<AuthForm mode="login" error={form?.error} zitadelEnabled={data.zitadelEnabled}>
  <FormField label="Email">
    <Input name="email" type="email" autocomplete="email" required />
  </FormField>
  <FormField label="Password">
    <Input name="password" type="password" autocomplete="current-password" required />
  </FormField>
  <Button variant="primary" type="submit">Sign in</Button>
</AuthForm>
