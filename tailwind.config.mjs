import presetQuick from "franken-ui/shadcn-ui/preset-quick";

/** @type {import('tailwindcss').Config} */
export default {
  presets: [presetQuick()],
  content: ["./templates/**/*.html"],
};
