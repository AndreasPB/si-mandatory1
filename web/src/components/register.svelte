<script lang="ts">
  const API_URL = "http://localhost:9000"

  const handleSubmit = async (event: SubmitEvent) => {
    const form = event.target as HTMLFormElement

    // Sending body as x-www-form-url-encoded
    await fetch(form.action, {
      method: form.method,
      body: new URLSearchParams([...(new FormData(form) as any)]),
    })
      .then((response: Response) => response.json())
      .then(json => json)
      .catch(error => console.log(error))
  }
</script>

<article>
  <div>
    <hgroup>
      <h1>Register</h1>
      <h2>Enter your stuff</h2>
    </hgroup>
    <form
      action={`${API_URL}/user`}
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
      <label for="description"
        >Description
        <input
          type="text"
          name="description"
          placeholder="Not sure why this is here tbh"
        />
      </label>
      <button type="submit">Submit</button>
    </form>
  </div>
</article>
