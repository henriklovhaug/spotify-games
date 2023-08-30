<script lang="ts">
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
    </ul>
  </div>
</div>
