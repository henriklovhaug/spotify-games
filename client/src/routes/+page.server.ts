import { BACKEND_URL } from "$env/static/private";
import type { Song } from "$lib/types";
import { type Actions, fail } from "@sveltejs/kit";

export const actions: Actions = {
  search: async ({ request, fetch }) => {
    const data = await request.formData();
    const search_value = data.get("search");
    const response = await fetch(
      `http://${BACKEND_URL}/search?search=${search_value}`,
    );
    if (response.ok) {
      const json: Song[] = await response.json();
      console.log(json);
      return {
        body: json,
      };
    } else {
      return fail(400, { message: "Bad Request" });
    }
  },

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

    const response = await fetch(`http://${BACKEND_URL}/queue`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(song),
    });

    if (!response.ok) {
      return fail(400, { message: "Bad Request" });
    }
  },
};
