<script>
  import { auth, mitIdAuth } from "../stores/jwt"
  import jwt_decode from "jwt-decode"
</script>

<nav class="container-fluid">
  <ul>
    <li><a href="/"><strong>SI-mandatory1</strong></a></li>
  </ul>
  <ul>
    {#if $mitIdAuth}
      <li><a href="/overview">Overview</a></li>
      <li><a href="/register">Register</a></li>
    {/if}
    {#if $auth}
      <li>
        <details role="list" dir="rtl">
          <summary aria-haspopup="listbox" role="link"
            >{jwt_decode($auth).name}
            <i class="fa fa-user" aria-hidden="true" />
          </summary>
          <ul role="listbox">
            <li
              on:click={() => {
                $auth = ""
              }}
            >
              <a>Sign out</a>
            </li>
          </ul>
        </details>
      </li>
    {:else}
      <li><a href="/">Login</a></li>
    {/if}
    <!-- TODO: Add logout(destroy stored JWT) -->
  </ul>
</nav>
