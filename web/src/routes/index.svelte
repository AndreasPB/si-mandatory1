<script lang="ts">
  import LoginForm from "../components/loginForm.svelte"
  import MessageForm from "../components/messageForm.svelte"
  import {auth} from "../stores/jwt";
</script>

{#if $auth}
    <MessageForm />
{:else}
    <LoginForm />
{/if}
<article>
    <hgroup>Admin powers</hgroup>
        <article>
            <a href="/register">Go to registration, where you can test out our Rest API. Create and view messages in whichever format you choose!</a>
        </article>
        <article>
            <a href="/overview">Get an overview of users and their associated phone numbers</a>
        </article>
</article>