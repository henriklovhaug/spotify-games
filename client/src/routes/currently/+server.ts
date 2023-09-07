import { PUBLIC_BACKEND_URL } from "$env/static/public";
import type { Song } from "$lib/types";
import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ fetch }) => {
  const response = await fetch(
    `http://${PUBLIC_BACKEND_URL}/currently_playing`,
  );
  const data: Song = await response.json();

  return new Response(JSON.stringify(data));
};
