/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#1c1f26',
        'background-alt': '#121317',
        text: '#cecbc8',
      }
    },
  },
  plugins: [],
}
