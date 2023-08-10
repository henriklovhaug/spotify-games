<script lang="ts">
	let visible = false;
</script>

{#if !visible}
	<div class="search-icon">
		<div class="placeholder" />
	</div>
{:else}
	<label for="search-box">
		<input type="text" id="search-box" />
	</label>
{/if}

<style lang="scss">
</style>
