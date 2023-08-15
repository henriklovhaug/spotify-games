<script lang="ts">
	export let name: string;
	export let artist: string;
	export let duration: number;
</script>

<div class="container">
	<div class="name">{name}</div>
	<div class="info-container">
		<div class="artist">{artist}</div>
		<div class="duration">{duration}</div>
	</div>
</div>

<style lang="scss">
	.container {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		width: 70%;
	}

	.info-container {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.name {
		font-size: 1.5rem;
	}

	.artist {
		font-size: 1rem;
	}

	.duration {
		font-size: 0.8rem;
	}
</style>
