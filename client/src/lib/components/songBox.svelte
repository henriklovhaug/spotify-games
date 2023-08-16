<script lang="ts">
	export let name: string;
	export let artist: string;
	export let duration: number;
	export let id: string;
  export let album: string;

	$: time = new Date(duration).toISOString().slice(14, 19);
</script>

<div class="my-2 w-4/6">
	<form action="?/queue" method="post">
		<input type="hidden" name="id" value={id} />
		<input type="hidden" name="name" id="name" value={name} />
    <input type="hidden" name="artist" id="artist" value={artist} />
    <input type="hidden" name="duration" id="duration" value={duration} />
    <input type="hidden" name="album" id="album" value={album} />
		<button class="flex w-full flex-col rounded-lg bg-green-700" type="submit">
			<span class="w-full text-lg">{name}</span>
			<span class="flex w-full flex-row">
				<span class="w-3/5">{artist}</span>
				<span>{time}</span>
			</span>
		</button>
	</form>
</div>
