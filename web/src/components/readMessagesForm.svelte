<script lang="ts">
  import { variables } from "../variables"
  import { auth } from "../stores/jwt"

  let topic: string
  let messages: []
  let loaded: boolean

  const handleFetchMessages = async () => {
    const res = await fetch(`${variables.pythonApi}/read-messages/${topic}`, {
      headers: { auth: $auth },
    }).then((response: Response) => response)

    messages = await res.json()
    loaded = true
  }
</script>

<article>
  <div>
    <hgroup>
      <h1>Read messages</h1>
    </hgroup>
    <div class="grid">
      <label>
        Topic
        <input bind:value={topic} type="text" name="topic" placeholder="Medicine" required />
        <small>Enter topic of the message here</small>
      </label>
    </div>
    <button type="submit" on:click={handleFetchMessages}>Fetch Messages</button>
  </div>
  {#if loaded}
    <h1>Messages</h1>
    <table role="grid">
      <thead>
        <tr>
          <th scope="col">#</th>
          <th scope="col">Content</th>
        </tr>
      </thead>
      {#if messages}
        {#each messages as message, i}
          <tbody>
            <tr>
              <th scope="row">{i + 1}</th>
              <td>{message.content}</td>
            </tr>
          </tbody>
        {/each}
      {/if}
    </table>
  {/if}
</article>
