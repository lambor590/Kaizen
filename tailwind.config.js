import daisyui from 'daisyui'
import typography from '@tailwindcss/typography'

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{js,ts,astro,tsx}'],
  theme: {
    extend: {},
  },
  plugins: [typography, daisyui],
}
