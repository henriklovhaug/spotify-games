<script lang="ts">
	import { enhance } from "$app/forms";
	import { PUBLIC_BACKEND_URL } from "$env/static/public";
	import { onMount } from "svelte";

	let socket;

	onMount(() => {
		socket = new WebSocket("ws://" + PUBLIC_BACKEND_URL + "/ws");
		socket.onopen = () => {
			console.log("connected");
		};

		socket.onmessage = (event) => {
			if (event.data === ws_message) return;
			ws_message = event.data;
			timer = 20;
			console.log(ws_message);
		};

		socket.onclose = () => {
			console.log("disconnected");
		};
	});

	let ws_message: String = "";
	$: artist = ws_message.split("#")[0];
	$: title = ws_message.split("#")[1];

	let timer: number = 0;
	setInterval(() => (timer > 0 ? (timer -= 1) : 0), 1000);
</script>

<div class="m-0 flex h-full w-screen flex-col items-center justify-evenly">
	<div class="flex flex-col items-center border-gray-50 px-2">
		{#if ws_message == ""}
			<h2>Waiting for song...</h2>
		{:else}
			<h2 class="text-center text-3xl font-extrabold">{title}</h2>
			<h3 class="text-xl">{artist}</h3>
			<h3 class="text-xl">{timer}</h3>
		{/if}
	</div>
	<form action="?/skip" method="post" use:enhance class="m-0">
		<button type="submit" class="h-80 w-80 animate-pulse rounded-full bg-green-500">Skip</button>
	</form>
</div>
