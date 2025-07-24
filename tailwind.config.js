/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html"
  ],
  theme: {
    extend: {
      colors: {
        'brand-purple': '#6B46C1',
        'brand-light-purple': '#F0EFFF',
        'brand-dark-purple': '#2D3748',
        'brand-gray': '#A0AEC0',
      },
      fontFamily: {
        'sans': ['Inter', 'sans-serif'],
        'mono': ['"Roboto Mono"', 'monospace'],
      },
    },
  },
  plugins: [],
}