import type { Actions } from "@sveltejs/kit";

export const actions: Actions = {
  search: async ({ request, cookies }) => {
    console.log(request);
    console.log(cookies);
    // TODO
  },
};
