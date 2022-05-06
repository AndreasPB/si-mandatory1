<script lang="ts">
  import UserTable from "../components/userTable.svelte"
  import MitIdLogin from "../components/mitIdLogin.svelte"
  import {mitIdAuth} from "../stores/jwt";
</script>

{#if $mitIdAuth}
    <UserTable />
{:else}
    <MitIdLogin />
{/if}
