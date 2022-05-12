<script>
  import { mitIdAuth } from "../stores/jwt.ts"
</script>

<svelte:window
  on:message={e => {
    $mitIdAuth = e.data
  }}
/>
<!-- TODO: Write the users name in the welcome message -->

<h1>Welcome to the page! Please login with MitID</h1>
<iframe
  title="MitID"
  src="https://mitid.eu.pythonanywhere.com/"
  height="1000px"
  width="100%"
/>
