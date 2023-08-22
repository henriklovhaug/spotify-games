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
			console.log(event.data);
		};

		socket.onclose = () => {
			console.log("disconnected");
			setTimeout(try_reconnect, 5000);
		};
	});
</script>

<div class="m-0 flex h-full w-screen flex-col items-center justify-center">
	<form action="?/skip" method="post" use:enhance class="m-0">
		<button type="submit" class="h-80 w-80 animate-pulse rounded-full bg-green-500">Skip</button>
	</form>
</div>
