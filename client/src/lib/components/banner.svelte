<script lang="ts">
  import Search from "$lib/components/search.svelte";

  let className: string;
  export { className as class };
</script>

<div
  class="relative left-0 top-0 flex w-screen justify-between bg-green {className}"
>
  <a href="/" class="mt-3 px-3 text-black no-underline">Home</a>
  <Search />
</div>
