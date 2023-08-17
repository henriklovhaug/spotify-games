import { redirect, type Actions } from "@sveltejs/kit";

export const actions: Actions = {
  search: async ({ request }) => {
    const data = await request.formData();
    const search_value = data.get("search");
    throw redirect(303, `/search/${search_value}`);
  },
};
