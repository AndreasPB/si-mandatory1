<script lang="ts">
  const API_URL = "http://localhost:2000"
  const fetchUsers = fetch(`${API_URL}/find-user`).then(res => res.json())
</script>

<table>
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
        <td>Loading...</td>
        <td>Loading...</td>
        <td>Loading...</td>
      </tr>
    </tbody>
  {:then users}
    {#each users as user, i}
      <tbody>
        <tr>
          <th scope="row">{i + 1}</th>
          <td>{user.name}</td>
          <td>{user.phone}</td>
          <td>{users.description}</td>
        </tr>
      </tbody>
    {/each}
  {:catch error}
    {error}
  {/await}
</table>
