<script lang="ts">
  import { variables } from "../variables"
  import { auth } from "../stores/jwt"

  let formats = ["JSON", "XML", "YAML", "TSV"]

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
      action={() => {}}
      method="post"
      on:submit|preventDefault={handleSubmit}
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
            placeholder="I just took the biggest shit in my life mayn"
            required
          />
          <small>Enter message content here</small>
        </label>
      </div>
      <button type="submit">Create</button>
    </form>
  </div>
</article>
<article>
  <div>
    <hgroup>
      <h1>Get messages</h1>
    </hgroup>
    <form
      action={() => {}}
      method="get"
      on:submit|preventDefault={handleSubmit}
    >
      <div class="grid">
        <label for="topic">
          Topic
          <input type="text" name="name" placeholder="Medicine" required />
          <small>Topic of the messages to view</small>
        </label>
      </div>
      <label for="format">Message format</label>
      <select id="format" required>
        {#each formats as format}
            <option value={format} selected>{format}</option>
        {/each}
      </select>
      <button type="submit">Get</button>
    </form>
  </div>
</article>