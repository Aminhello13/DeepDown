/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // DeepDown Purple-Gray Theme
        deepx: {
          50:  "#f4f2ff",
          100: "#ece8ff",
          200: "#d9d0ff",
          300: "#b8a6ff",
          400: "#9a7dff",
          500: "#7c3aed",
          600: "#6d28d9",
          700: "#5b21b6",
          800: "#4c1d95",
          900: "#2e1065",
          950: "#1a0b2e",
        },
        grayx: {
          50:  "#f8f8fc",
          100: "#eeeef5",
          200: "#dddde8",
          300: "#c0c0d0",
          400: "#9a9ab0",
          500: "#6e6e85",
          600: "#4d4d63",
          700: "#363648",
          800: "#1a1b2f",
          900: "#0d0e1a",
        }
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
    },
  },
  plugins: [],
};
