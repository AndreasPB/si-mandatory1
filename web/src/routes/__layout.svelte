<script>
  import "@picocss/pico/css/pico.min.css"
  import "font-awesome/css/font-awesome.min.css"
  import Navbar from "../components/navbar.svelte"
</script>

<Navbar />
<main class="container">
  <slot />
</main>
