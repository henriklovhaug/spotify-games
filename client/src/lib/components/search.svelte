<script lang="ts">
  import { enhance } from "$app/forms";
  import { afterUpdate } from "svelte";

  function handle_submit(event: Event) {
    event.preventDefault();
    const element = event.target as HTMLFormElement;
    const value = element.value;
    console.log(value);
    search = value;
    visible = !visible;
  }

  afterUpdate(() => {
    if (visible) {
      input.focus();
    }
  });

  let visible = false;
  let search = "";

  let input: HTMLInputElement;
</script>

{#if !visible}
  <div class="h-full object-contain">
    <button
      class="h-full border-none bg-none"
      on:click={() => (visible = !visible)}
    >
      <img
        src="/icons/search-icon.png"
        alt="search icon"
        class=" h-5/6 py-2 pr-2"
      />
    </button>
  </div>
{:else}
  <form
    class="m-0 flex w-screen justify-center pt-2"
    method="post"
    action="/?/search"
    use:enhance
  >
    <label for="search-box">
      <input
        class="h-6 w-4/5 rounded-md bg-neutral-200 text-black"
        name="search"
        type="text"
        id="search-box"
        on:focusout={handle_submit}
        value={search}
        bind:this={input}
      />
    </label>
  </form>
{/if}
