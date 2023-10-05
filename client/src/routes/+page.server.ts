import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { type Actions, redirect, fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Song } from "$lib/types";

export const load: PageServerLoad = async ({ fetch, params }) => {

	const response = await fetch(`http://${PUBLIC_BACKEND_URL}/queue`);
	if (response.ok) {
		const json: Song[] = await response.json();
		return {
			body: json,
		};
	} else {
		return fail(400, { message: "Bad Request" });
	}
};

export const actions: Actions = {
	search: async ({ request }) => {
		const data = await request.formData();
		const search_value = data.get("search");
		throw redirect(303, `/search/${search_value}`);
	},
	game: async ({ request }) => {
		const data = await request.formData();
		const game_id = data.get("game");
		throw redirect(303, `/game/${game_id}`);
	},
	endGame: async ({ fetch }) => {
		await fetch(`http://${PUBLIC_BACKEND_URL}/game/stop`, {
			method: "PUT",
		});
	},
};
