<script lang="ts">
  import { variables } from "../variables"
  import { auth } from "../stores/jwt"

  // TODO: Add JWT to session/local storage

  const handleSubmit = async (event: SubmitEvent) => {
    const form = event.target as HTMLFormElement

    // Sending body as x-www-form-url-encoded
    const res = await fetch(form.action, {
      method: form.method,
      body: new URLSearchParams([...(new FormData(form) as any)]),
    })
      .then((response: Response) => response)
      .catch(error => console.log(error))
    if (res && res.status === 200) {
      $auth = await res.json()
    }
  }
</script>

<article>
  <div>
    <hgroup>
      <h1>Login</h1>
      <h2>Enter your stuff</h2>
    </hgroup>
    <form
      action={`${variables.pythonApi}/login`}
      method="post"
      on:submit|preventDefault={handleSubmit}
    >
      <div class="grid">
        <label for="phone">
          Phone
          <input type="number" name="phone" placeholder="12345678" required />
        </label>
        <label for="password">
          Password
          <input type="text" name="password" placeholder="1234" required />
          <small>This is the token you've received by SMS.</small>
        </label>
      </div>
      <button type="submit">Log in</button>
    </form>
  </div>
</article>
