<script>
  import "@picocss/pico/css/pico.min.css"
  import Navbar from "../components/navbar.svelte"
</script>

<Navbar />
<main class="container">
  <slot />
</main>
