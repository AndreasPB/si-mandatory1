<script lang="ts">
  const API_URL = "https://apiv3.test.streamchaser.tv"
  const fetchUsers = fetch(`${API_URL}/user`).then(res => res.json())
</script>

<table role="grid">
  <thead>
    <tr>
      <th scope="col">#</th>
      <th scope="col">Name</th>
      <th scope="col">Phone</th>
      <th scope="col">Description</th>
    </tr>
  </thead>
  {#await fetchUsers}
    <tbody>
      <tr>
        <th scope="row">1</th>
        <td aria-busy="true">Loading...</td>
        <td aria-busy="true">Loading...</td>
        <td aria-busy="true">Loading...</td>
      </tr>
    </tbody>
  {:then users}
    {#each users as user, i}
      <tbody>
        <tr>
          <th scope="row">{i + 1}</th>
          <td>{user.name}</td>
          <td>{user.phone}</td>
          <td>{user.description}</td>
        </tr>
      </tbody>
    {/each}
  {:catch error}
    {error}
  {/await}
</table>
