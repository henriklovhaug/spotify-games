<script lang="ts">
  import type { QueueSong } from "$lib/types";
  import { fly } from "svelte/transition";

  let className: string;

  export { className as class };

  export let active: boolean = false;
  export let song: QueueSong;

  // Uncomment this to test the popup
  $: if (active) {
    setTimeout(() => {
      active = false;
    }, 3000);
  }
</script>

{#if active}
  <div
    transition:fly={{ delay: 250, duration: 700 }}
    class="flex w-full flex-col items-center bg-green {className}"
  >
    <h1 class="my-2 text-3xl text-black">{song.name} - {song.artist}</h1>
    <h2 class="my-2 text-black">Was added to the queue</h2>
  </div>
{/if}
