<script lang="ts">
  const games = ["SixMinutes", "RattlingBog"];
</script>

<div class="flex flex-col items-center">
  <div class="mt-8">
    <ul>
      {#each games as game}
        <li>
          <form action="?/game" method="post">
            <input type="hidden" name="game" id={game} value={game} />
            <button
              class="my-2 w-32 whitespace-normal rounded border border-green bg-transparent px-4 py-2 font-semibold text-green hover:border-transparent hover:bg-emerald-600 hover:text-white"
              type="submit"
            >
              {game}
            </button>
          </form>
        </li>
      {/each}
    </ul>
  </div>
</div>
