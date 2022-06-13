<script lang="ts">
  import { variables } from "../variables"
  import { auth } from "../stores/jwt"

  let pressedSubmit: boolean
  let createSuccess: boolean
  let createFail: boolean
  let errorMsg: string

  const handleCreateMessage = async (event: SubmitEvent) => {
    pressedSubmit = true
    setTimeout(() => (pressedSubmit = false), 2000)
    const form = event.target as HTMLFormElement

    // Sending body as x-www-form-url-encoded
    const res = await fetch(form.action, {
      method: form.method,
      headers: { auth: $auth },
      body: new URLSearchParams([...(new FormData(form) as any)]),
    })
      .then((response: Response) => response)
      .catch(error => console.log(error))

    if (res) {
      if (res.status === 200 || res.status === 201) {
        createSuccess = true
      } else {
        createFail = true
        errorMsg = `${res.status} ${res.statusText}`
      }
    }
  }
</script>

<article>
  <div>
    <hgroup>
      <h1>Create message</h1>
    </hgroup>
    <form
      action={`${variables.pythonApi}/create-message`}
      method="post"
      on:submit|preventDefault={handleCreateMessage}
    >
      <div class="grid">
        <label for="topic">
          Topic
          <input type="text" name="topic" placeholder="Medicine" required />
          <small>Enter topic of the message here</small>
        </label>
        <label for="content">
          Content
          <input
            type="text"
            name="content"
            placeholder="Medicine information"
            required
          />
          <small>Enter message content here</small>
        </label>
      </div>
      {#if pressedSubmit}
        {#if createSuccess}
          <button aria-busy="true" disabled>Creating message!</button>
          <small>Message successfully created</small>
        {:else}
          <button aria-busy="true" class="secondary" disabled
            >Trying to create message...</button
          >
        {/if}
      {:else if createFail}
        <button type="submit">Failed to create message</button>
        <small>{errorMsg}</small>
      {:else}
        <button type="submit">Create Message</button>
      {/if}
    </form>
  </div>
</article>
