<script lang="ts">
  import Banner from "$lib/components/banner.svelte";
  import { onMount } from "svelte";
  import "../app.css";
  import type { QueueSong, WSMessage } from "$lib/types";
  import QueuePopup from "$lib/components/queuePopup.svelte";
  import { PUBLIC_BACKEND_URL } from "$env/static/public";

  let socket: WebSocket;
  let next_song: QueueSong;
  let active: boolean = false;

  onMount(() => {
    socket = new WebSocket("ws://" + PUBLIC_BACKEND_URL + "/ws");
    socket.onopen = () => {
      console.log("connected");
    };

    socket.onmessage = (event) => {
      console.log(event.data);
      try {
        const parsed: WSMessage = JSON.parse(event.data);
        if (parsed.channel != "QueueSong") return;
        if (parsed.song && parsed.artist) {
          next_song = {
            name: parsed.song,
            artist: parsed.artist,
          };
          active = true;
        }
      } catch (error) {
        console.log(error);
      }
    };

    socket.onclose = () => {
      console.log("disconnected");
    };
  });
</script>

<div class="flex h-screen flex-col text-[#f4fefd]">
  <Banner class="h-14" />
  <slot />
  <QueuePopup bind:song={next_song} bind:active class="" />
</div>
