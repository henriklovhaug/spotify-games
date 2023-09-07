<script lang="ts">
  import type { PageData } from "./$types";
  export let data: PageData;

  const song = data.data;
</script>

<div class="flex w-full flex-col">
  <h1>{song.name}</h1>
  <h1>{song.artist}</h1>
  {#if song.album_url}
    <img src={song.album_url} alt="album cover" />
    {:else}
    <h1>No album cover</h1>
  {/if}
</div>
