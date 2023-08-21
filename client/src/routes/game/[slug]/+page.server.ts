import { BACKEND_URL } from "$env/static/private";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, params }) => {
  let game: string = params.slug;

  const response = await fetch(`http://${BACKEND_URL}/game/${game}`, {
    method: "PUT",
  });

  if (response.ok) {
    console.log("Game successfully updated");
    return {
      success: true,
    };
  } else {
    throw redirect(303, "/");
  }
};
