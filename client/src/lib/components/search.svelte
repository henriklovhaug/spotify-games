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
		<button class="h-fit border-none bg-none" on:click={() => (visible = !visible)}>
			<img src="icons/search-icon.png" alt="search icon" class=" pr-2 pt-2" />
		</button>
	</div>
{:else}
	<form class="m-0 flex w-screen justify-center pt-2" method="post" action="?/search" use:enhance>
		<label for="search-box">
			<input
				class="h-72 w-screen rounded-md"
				name="search"
				type="text"
				id="search-box"
				on:focusout={handle_submit}
				value={search}
			/>
		</label>
	</form>
{/if}
