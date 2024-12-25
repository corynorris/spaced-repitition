import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

import { config } from "dotenv";
import path from "path";
config(); // Load env file
const apiUrl = process.env.VITE_API_URL;

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    proxy: {
      "/api": {
        target: apiUrl || "http://localhost:5000",
        changeOrigin: true,
      },
    },
  },
});
