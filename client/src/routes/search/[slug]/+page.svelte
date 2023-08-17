<script lang="ts">
	import SongBox from '$lib/components/songBox.svelte';
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<div class="flex flex-col items-center">
	{#if data?.body}
		{#each data.body as song}
			<!-- Every songbox will add to queue when clicked on -->
			<SongBox
				id={song.id}
				name={song.name}
				artist={song.artist}
				duration={song.duration}
				album={song.album}
			/>
		{/each}
	{/if}
</div>
