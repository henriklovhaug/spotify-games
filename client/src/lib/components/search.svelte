<script lang="ts">
	import { enhance } from '$app/forms';

	function handle_submit(event: Event) {
		event.preventDefault();
		const element = event.target as HTMLFormElement;
		const value = element.value;
		console.log(value);
		search = value;
		visible = !visible;
	}

	let visible = false;
	let search = '';
</script>

{#if !visible}
	<div class="search-icon">
		<button on:click={() => (visible = !visible)}>
			<img src="icons/search-icon.png" alt="search icon" />
		</button>
	</div>
{:else}
	<form method="post" action="?/search" use:enhance>
		<label for="search-box">
			<input type="text" id="search-box" on:focusout={handle_submit} value={search} />
		</label>
	</form>
{/if}

<style lang="scss">
	img {
		height: 70%;
		width: auto;
		padding-top: 0.5rem;
		padding-right: 0.5rem;
	}

	form {
		margin: 0;
		width: 100%;
		display: flex;
		justify-content: center;
		padding-top: 0.5rem;
	}

	button {
		background: none;
		border: none;
		height: 100%;
	}

	input {
		height: 70%;
		width: 70vw;
		border-radius: 0.5rem;
		background-color: rgba($color: green, $alpha: 0.3);
	}
</style>
