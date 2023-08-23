import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { PUBLIC_BACKEND_URL } from "$env/static/public";

export const load: PageServerLoad = async ({ fetch, params }) => {
  let game: string = params.slug;

  const response = await fetch(`http://${PUBLIC_BACKEND_URL}/game/${game}`, {
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
