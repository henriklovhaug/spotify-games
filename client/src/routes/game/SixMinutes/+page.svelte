<script lang="ts">
	import { enhance } from "$app/forms";
</script>

<div class="flex h-screen w-full flex-col items-center justify-center">
	<form action="?/skip" method="post" use:enhance>
		<button type="submit" class="h-80 w-80 rounded-full bg-green-500">Skip</button>
	</form>
</div>
