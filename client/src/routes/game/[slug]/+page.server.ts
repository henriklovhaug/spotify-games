import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { PUBLIC_BACKEND_URL } from "$env/static/public";
import type { Song } from "$lib/types";

export const load: PageServerLoad = async ({ fetch, params }) => {
  let game: string = params.slug;

  const response = await fetch(`http://${PUBLIC_BACKEND_URL}/game/${game}`, {
    method: "PUT",
  });

  if (response.ok) {
    console.log("Game successfully updated");
    const current_response = await fetch(
      `http://${PUBLIC_BACKEND_URL}/game/${game}`,
    );
    const current_data: Song = await current_response.json();
    return {
      data: current_data,
    };
  } else {
    throw redirect(303, "/");
  }
};
