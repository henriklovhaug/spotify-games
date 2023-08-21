import { BACKEND_URL } from "$env/static/private";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch(`http://${BACKEND_URL}/game/SixMinutes`, {
    method: "PUT",
  });

  if (!response.ok) {
    throw redirect(303, "/");
  }
};
