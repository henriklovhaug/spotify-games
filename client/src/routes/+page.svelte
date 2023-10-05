<script lang="ts">
  import { enhance } from "$app/forms";
  import SearchResults from "$lib/components/searchResults.svelte";
  import type { GameMode } from "$lib/gamemodes";
  import type { PageData } from "./$types";
  export let data: PageData;

  const games: GameMode[] = [
    {
      name: "6-Minutes",
      url: "SixMinutes",
    },
    {
      name: "Rattling Bog",
      url: "RattlingBog",
    },
    {
      name: "Opus",
      url: "Opus",
    },
    {
      name: "Der Palmerna bor",
      url: "Palmerna",
    },
    {
      name: "Thunder",
      url: "Thunder",
    },
  ];
</script>

<div class="flex items-center justify-around">
  <div class="mt-8 self-start">
    <h2 class="text-center text-xl">Start a Game</h2>
    <ul>
      {#each games as game}
        <li>
          <form action="?/game" method="post">
            <input type="hidden" name="game" value={game.url} />
            <button
              class="my-2 w-32 whitespace-normal rounded border border-green bg-transparent px-4 py-2 font-semibold text-green hover:border-transparent hover:bg-emerald-600 hover:text-white"
              type="submit"
            >
              {game.name}
            </button>
          </form>
        </li>
      {/each}
      <form action="?/endGame" method="post" use:enhance>
        <button
          class="border-red text-red my-2 w-32 whitespace-normal rounded border border-red-400 bg-transparent px-4 py-2 font-semibold text-red-400 hover:border-transparent hover:bg-red-600 hover:text-white"
          type="submit"
        >
          End Game
        </button>
      </form>
    </ul>
  </div>
  <div class="mt-8 flex flex-col justify-items-center self-start">
    <h2 class="mb-2 text-center text-xl">Song Queue</h2>
    {#if data?.body}
      {#each data.body as song}
        <div class="rounded border border-green bg-transparent mb-4 p-2 text-green">
			<h3>{song.name}</h3>
		</div>
      {/each}
    {/if}
  </div>
</div>
