import { BACKEND_URL } from '$env/static/private';
import type { Song } from '$lib/types';
import { type Actions, fail } from '@sveltejs/kit';

export const actions: Actions = {
	search: async ({ request, fetch }) => {
		const data = await request.formData();
		const search_value = data.get('search');
		const response = await fetch(`http://${BACKEND_URL}/search?search=${search_value}`);
		if (response.ok) {
			const json: Song[] = await response.json();
			console.log(json);
			return {
				body: json
			};
		} else {
			return fail(400, { message: 'Bad Request' });
		}
	}
};
