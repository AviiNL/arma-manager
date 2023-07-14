/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: ["class", '[data-theme="dark"]'],
    theme: {
      extend: {},
    },
    daisyui: {
      themes: ["default", "light", "dark"],
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}