import { type Actions, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { PUBLIC_BACKEND_URL } from "$env/static/public";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch(`http://${PUBLIC_BACKEND_URL}/game/SixMinutes`, {
    method: "PUT",
  });

  if (!response.ok) {
    throw redirect(303, "/");
  }
};

export const actions: Actions = {
  skip: async ({ fetch }) => {
    const response = await fetch(
      `http://${PUBLIC_BACKEND_URL}/sixminutes/skip`,
      {
        method: "PUT",
      },
    );

    console.log(response.status);
    return {
      status: response.status,
    };
  },
};
