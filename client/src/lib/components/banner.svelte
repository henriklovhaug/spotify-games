<script lang="ts">
	import Search from "$lib/components/search.svelte";

	let className: string;
	export { className as class };
</script>

<div class="relative left-0 top-0 flex w-screen justify-between bg-green-600 {className}">
	<a href="/" class="mt-2 no-underline">Home</a>
	<Search />
</div>
