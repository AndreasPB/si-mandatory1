<article>
  <div>
    <hgroup>
      <h1>Register</h1>
      <h2>Enter your stuff</h2>
    </hgroup>
    <form>
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
