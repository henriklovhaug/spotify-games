<script lang="ts">
  import { enhance } from "$app/forms";

  function handle_submit(event: Event) {
    event.preventDefault();
    const element = event.target as HTMLFormElement;
    const value = element.value;
    console.log(value);
    search = value;
  }

  let search = "";
</script>

<div class="w-full rounded border border-solid border-green">
  <form
    class="flex justify-center bg-black p-0"
    method="post"
    action="/?/search"
    use:enhance
  >
    <input
      class="m-1 h-full w-full rounded-md bg-transparent p-0 text-green outline-none"
      name="search"
      type="text"
      id="search-box"
      on:focusout={handle_submit}
      value={search}
      placeholder="Search for new song to queue"
    />
  </form>
</div>
