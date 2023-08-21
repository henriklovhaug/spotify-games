<script lang="ts">
	const games = ['SixMinutes', 'RattlingBog'];
</script>

<div class="flex flex-col items-center">
	<h1>Spotify Game</h1>
	<div>
		<ul>
			{#each games as game}
				<li>
					<form action="?/game" method="post">
						<input type="hidden" name="game" id={game} value={game} />
						<button
							class="my-2 rounded border border-blue-500 bg-transparent px-4 py-2 font-semibold text-blue-700 hover:border-transparent hover:bg-blue-500 hover:text-white"
							type="submit"
						>
							{game}
						</button>
					</form>
				</li>
			{/each}
		</ul>
	</div>
</div>
