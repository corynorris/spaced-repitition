<script lang="ts">
import { enhance } from "$app/forms";
import AuthForm from "$lib/client/organisms/AuthForm.svelte";
import Button from "$lib/client/atoms/Button.svelte";
import Input from "$lib/client/atoms/Input.svelte";
import FormField from "$lib/client/molecules/FormField.svelte";

let { data, form } = $props();
</script>

<AuthForm mode="register" error={form?.error} zitadelEnabled={data.zitadelEnabled}>
  <FormField label="Name" optional>
    <Input name="name" autocomplete="name" />
  </FormField>
  <FormField label="Email">
    <Input name="email" type="email" autocomplete="email" required />
  </FormField>
  <FormField label="Password">
    <Input name="password" type="password" autocomplete="new-password" required />
  </FormField>
  <Button variant="primary" type="submit">Create account</Button>
</AuthForm>
