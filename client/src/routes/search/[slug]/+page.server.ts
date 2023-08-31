import type { Song } from "$lib/types";
import { type Actions, fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { PUBLIC_BACKEND_URL } from "$env/static/public";

export const load: PageServerLoad = async ({ fetch, params }) => {
  const slug = params.slug;

  const response = await fetch(
    `http://${PUBLIC_BACKEND_URL}/search?search=${slug}`,
  );
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
  queue: async ({ request, fetch }) => {
    const data = await request.formData();

    const song: Song = {
      id: data.get("id") as string,
      name: data.get("name") as string,
      artist: data.get("artist") as string,
      album: data.get("album") as string,
      duration: +!data.get("duration"),
    };

    console.log(JSON.stringify(song) + "stonk");

    const response = await fetch(`http://${PUBLIC_BACKEND_URL}/queue`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(song),
    });

    if (!response.ok) {
      return fail(400, { message: "Bad Request" });
    }

    console.log("Song successfully queued");
  },
};
