<script lang="ts">
  import { onNavigate } from "$app/navigation";
  import { PUBLIC_BACKEND_URL } from "$env/static/public";
  import type { WSMessage } from "$lib/types";
  import { onMount } from "svelte";

  async function handleSubmit() {
    let data = new FormData();
    await fetch("?/skip", {
      method: "POST",
      body: data,
    });
  }

  let socket: WebSocket;

  onMount(() => {
    socket = new WebSocket("ws://" + PUBLIC_BACKEND_URL + "/ws");
    socket.onopen = () => {
      console.log("connected");
    };

    socket.onmessage = (event) => {
      try {
        const parsed: WSMessage = JSON.parse(event.data);
        if (parsed.channel != "SixMinutes") return;
        ws_message = parsed;
      } catch (error) {
        console.log(error);
      }
    };

    socket.onclose = () => {
      console.log("disconnected");
    };
  });

  onNavigate(() => {
    if (socket) socket.close();
  });

  let ws_message: WSMessage;
  let artist: string;
  let title: string;
  $: if (ws_message && ws_message.artist) artist = ws_message.artist;
  $: if (ws_message && ws_message.song) title = ws_message.song;

  let timer: number = 20;
  setInterval(() => (timer > 0 ? (timer -= 1) : 0), 1000);
</script>

<div class="m-0 flex h-full w-screen flex-col items-center justify-evenly">
  <div class="flex flex-col items-center border-gray-50 px-2">
    {#if !ws_message}
      <h2>Waiting for song...</h2>
    {:else if ws_message.message === "Game over"}
      <h2 class="text-center text-3xl font-extrabold">Game Over</h2>
      <h3 class="text-xl">Thanks for playing!</h3>
    {:else}
      <h2 class="px-8 text-center text-3xl font-extrabold">{title}</h2>
      <h3 class="text-xl">{artist}</h3>
      <h3 class="text-xl">{timer}</h3>
    {/if}
  </div>
  <form method="post" on:submit|preventDefault={handleSubmit} class="m-0">
    <button
      type="submit"
      class="h-72 w-72 animate-pulse rounded-full bg-green text-xl font-bold text-black"
      on:click={() => (timer = 20)}
      >Skip
    </button>
  </form>
</div>
