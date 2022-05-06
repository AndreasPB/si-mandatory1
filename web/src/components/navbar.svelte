<script>
  import {auth, mitIdAuth} from "../stores/jwt"
  import jwt_decode from "jwt-decode";
</script>

<nav class="container-fluid">
  <ul>
    <li><a href="/"><strong>Brand</strong></a></li>
  </ul>
  <ul>
    {#if $mitIdAuth}
      <li><a href="/overview">Overview</a></li>
      <li><a href="/register">Register</a></li>
    {/if}
    {#if $auth}
      <li style="margin: 0; padding: 0">{jwt_decode($auth).name}</li>
      <li style="margin: 0; padding-left: 6px"><i class="fa fa-user" aria-hidden="true"></i></li>
    {:else}
       <li><a href="/login">Login</a></li>
    {/if}
    <!-- TODO: Add logout(destroy stored JWT) -->
  </ul>
</nav>
