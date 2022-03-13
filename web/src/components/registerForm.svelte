<script lang="ts">
  import { variables } from "../variables"

  let pressedSubmit: boolean
  let registerSuccess: boolean
  let registerFail: boolean
  let errorMsg: string

  const handleSubmit = async (event: SubmitEvent) => {
    pressedSubmit = true
    setTimeout(() => (pressedSubmit = false), 2000)

    const form = event.target as HTMLFormElement

    // Sending body as x-www-form-url-encoded
    const res = await fetch(form.action, {
      method: form.method,
      body: new URLSearchParams([...(new FormData(form) as any)]),
    })
      .then((response: Response) => response)
      .catch(error => console.log(error))

    if (res) {
      if (res.status === 200 || res.status === 201) {
        registerSuccess = true
        setTimeout(() => window.location.replace("/login"), 3000)
      } else {
        registerFail = true
        errorMsg = `${res.status} ${res.statusText}`
      }
    }
  }
</script>

<article>
  <div>
    <hgroup>
      <h1>Register</h1>
      <h2>Enter your stuff</h2>
    </hgroup>
    <form
      action={`${variables.rustApi}/usasder`}
      method="post"
      on:submit|preventDefault={handleSubmit}
    >
      <div class="grid">
        <label for="name">
          Name
          <input type="text" name="name" placeholder="John Smith" required />
        </label>
        <label for="phone">
          Phone
          <input type="number" name="phone" placeholder="12345678" required />
          <small>We'll never share your phone number with anyone else.</small>
        </label>
      </div>
      {#if pressedSubmit}
        {#if registerSuccess}
          <button aria-busy="true" disabled>Registered!</button>
          <small>Redirecting you to the login page</small>
        {:else}
          <button aria-busy="true" class="secondary" disabled>Registering...</button>
        {/if}
      {:else if registerFail}
        <button type="submit">Try again</button>
        <small>{errorMsg}</small>
      {:else}
        <button type="submit">Register</button>
      {/if}
    </form>
  </div>
</article>
