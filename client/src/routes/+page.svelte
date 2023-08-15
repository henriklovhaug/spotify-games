<script lang="ts">
	import SongBox from '$lib/components/songBox.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

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
		<!-- content here -->
		<SongBox name={song.name} artist={song.artist} duration={song.duration} />
	{/each}
{/if}
