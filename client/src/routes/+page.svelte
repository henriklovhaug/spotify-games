<script lang="ts">
  import { enhance } from "$app/forms";
  import type { GameMode } from "$lib/gamemodes";

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
      url: "Palmerna"
    }
  ];
</script>

<div class="flex flex-col items-center">
  <div class="mt-8">
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
</div>
