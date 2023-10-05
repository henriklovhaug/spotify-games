<script lang="ts">
  import { PUBLIC_BACKEND_URL } from "$env/static/public";

  const response = fetch(`http://${PUBLIC_BACKEND_URL}/queue`).then((resp) =>
    resp.json(),
  );
</script>

{#await response then content}
  {#each content as song}
    {JSON.stringify(song)}
  {/each}
{/await}
