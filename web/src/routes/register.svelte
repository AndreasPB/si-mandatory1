<script>
  import RegisterForm from "../components/registerForm.svelte"
  import MitIdLogin from "../components/mitIdLogin.svelte"
  import {mitIdAuth} from "../stores/jwt";
</script>

{#if $mitIdAuth}
    <RegisterForm />
{:else}
    <MitIdLogin />
{/if}