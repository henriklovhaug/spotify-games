import { BACKEND_URL } from "$env/static/private";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, params }) => {
  let game: string = params.slug;

  console.log(game);

  const response = await fetch(`http://${BACKEND_URL}/game/${game}`, {
    method: "PUT",
  });

  console.log(await response.text());

  if (response.ok) {
    return {
      success: true,
    };
  } else {
    throw redirect(303, "/");
  }
};
