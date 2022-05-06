<script lang="ts">
  import { variables } from "../variables"
  import { auth } from "../stores/jwt"


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
      $auth = await res.text()
    }
  }
</script>

<article>
  <div>
    <hgroup>
      <h1>Create message</h1>
    </hgroup>
    <form
      action={`${variables.pythonApi}/login`}
      method="post"
      on:submit|preventDefault={handleSubmit}
    >
      <div class="grid">
        <label for="content">
          Topic
          <input type="text" name="topic" placeholder="Medicine" required />
          <small>Enter topic of the message here</small>
        </label>
        <label for="content">
          Content
          <input type="text" name="content" placeholder="I just took the biggest shit in my life mayn" required />
          <small>Enter message content here</small>
        </label>
      </div>
      <button type="submit">Create</button>
    </form>
  </div>
</article>