<script lang="ts">
  import { variables } from "../variables"

  const fetchUsers = fetch(`${variables.pythonApi}/user`).then(res =>
    res.json().catch(error => console.log(error))
  )
</script>

<table role="grid">
  <thead>
    <tr>
      <th scope="col">#</th>
      <th scope="col">Name</th>
      <th scope="col">Phone</th>
      <th scope="col">Email</th>
      <th scope="col">Password</th>
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
          <td>{user.email}</td>
          <td>{user.password}</td>
        </tr>
      </tbody>
    {/each}
  {:catch error}
    {error}
  {/await}
</table>
