import type { RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ request }) => {
  console.log("callback server", request.body);

  return new Response(JSON.stringify({ message: "callback server" }));
};
