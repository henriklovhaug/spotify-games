<script lang="ts">
	export let name: string;
	export let artist: string;
	export let duration: number;
</script>

<div class=" flex w-5/6 flex-row items-center justify-center">
	<div class="text-lg">{name}</div>
	<div class="flex flex-col">
		<div>{artist}</div>
		<div>{duration}</div>
	</div>
</div>
