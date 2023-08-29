<script lang="ts">
	import Banner from "$lib/components/banner.svelte";
	import "../app.css";
</script>

<div class="flex h-screen flex-col text-[#f4fefd]">
	<Banner class="h-14" />
	<slot />
</div>
