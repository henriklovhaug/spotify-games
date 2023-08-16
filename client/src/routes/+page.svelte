<script lang="ts">
	import SongBox from '$lib/components/songBox.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<div class="flex flex-col items-center">
	<h1>Welcome to Spotify Game</h1>
	<div>
		<ul>
			<li>
				<p>TODO</p>
			</li>
		</ul>
	</div>
	{#if form?.body}
		{#each form.body as song}
			<!-- Every songbox will add to queue when clicked on -->
			<SongBox
				id={song.id}
				name={song.name}
				artist={song.artist}
				duration={song.duration}
			/>
		{/each}
	{/if}
</div>
