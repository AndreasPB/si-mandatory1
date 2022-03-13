<script>
  import "@picocss/pico/css/pico.min.css"
  import Navbar from "../components/navbar.svelte"

  // TODO: Redirect to login/register if not logged in
</script>

<Navbar />
<main class="container">
  <slot />
</main>
