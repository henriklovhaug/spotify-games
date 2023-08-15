import { BACKEND_URL } from "$env/static/private";
import { type Actions, fail } from "@sveltejs/kit";

export const actions: Actions = {
  search: async ({ request, cookies, fetch }) => {
    console.log(request);
    console.log(cookies);
    const data = await request.formData();
    const search_value = data.get("search-box");
    const response = await fetch(
      `http://${BACKEND_URL}/search?search=${search_value}`,
    );
    if (response.ok) {
      const json: Song = await response.json();
      return {
        body: json,
      };
    } else {
      return fail(400, { message: "Bad Request" });
    }
  },
};
