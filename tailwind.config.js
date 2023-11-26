/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'bg-gray': '#1c1c1e',
        'dark-gray': '#18181b',
      },
    },
  },
  plugins: [],
}

