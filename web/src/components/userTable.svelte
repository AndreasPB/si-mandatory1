<script lang="ts">
  import { variables } from "../variables"
  import { mitIdAuth } from "../stores/jwt"

  interface User {
    name: string
    phone: number
    email: string
    password: string
  }

  const fetchUsers: Promise<User[] | void> = fetch(`${variables.pythonApi}/user`, {
    headers: { auth: $mitIdAuth },
  })
    .then((res: Response) => {
      if (res.status === 419) {
        console.log(res.statusText)
        $mitIdAuth = ""
      }
      return res.json()
    })
    .catch((error: Error) => {
      console.log(error)
    })
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
        <td aria-busy="true">Loading...</td>
      </tr>
    </tbody>
  {:then users}
    {#if users}
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
    {/if}
  {:catch error}
    {error}
  {/await}
</table>
