/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./static/**/*.js",
    "./templates/**/*.go.tmpl",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui")
  ],
}
