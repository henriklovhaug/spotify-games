<script lang="ts">
	export let name: string;
	export let artist: string;
	export let duration: number;

	$: time = new Date(duration).toISOString().slice(14, 19);
</script>

<button class="bg-green-700 rounded-md my-2 w-fit">
	<div class=" flex w-4/6 flex-row items-center justify-center">
		<div class="text-lg">{name}</div>
		<div class="flex flex-col">
			<div>{artist}</div>
			<div>{time}</div>
		</div>
	</div>
</button>
