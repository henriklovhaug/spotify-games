<script lang="ts">
	import { enhance } from "$app/forms";
	import { PUBLIC_BACKEND_URL } from "$env/static/public";
	import { onMount } from "svelte";

	let socket;

	function try_reconnect() {
		socket = new WebSocket("ws://" + PUBLIC_BACKEND_URL + "/ws");
	}

	onMount(() => {
		socket = new WebSocket("ws://" + PUBLIC_BACKEND_URL + "/ws");
		socket.onopen = () => {
			console.log("connected");
		};

		socket.onmessage = (event) => {
			ws_message = event.data;
			console.log(ws_message);
		};

		socket.onclose = () => {
			console.log("disconnected");
			setTimeout(try_reconnect, 5000);
		};
	});

	let ws_message: String = "";

	$: artist = ws_message.split("#")[0];
	$: title = ws_message.split("#")[1];
</script>

<div class="m-0 flex h-full w-screen flex-col items-center justify-evenly">
	<div class="flex flex-col items-center border-gray-50 px-2">
		{#if ws_message == ""}
			<h2>Waiting for song...</h2>
		{:else}
			<h2 class="text-center text-3xl font-extrabold">{title}</h2>
			<h3 class="text-xl">{artist}</h3>
		{/if}
	</div>
	<form action="?/skip" method="post" use:enhance class="m-0">
		<button type="submit" class="h-80 w-80 animate-pulse rounded-full bg-green-500">Skip</button>
	</form>
</div>
