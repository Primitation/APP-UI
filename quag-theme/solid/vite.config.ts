import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import { fileURLToPath } from "node:url";

// The explorer (component list + preview) is the dev/demo app of this package.
export default defineConfig({
  root: "explorer",
  base: "./",
  plugins: [solid()],
  server: { fs: { allow: [fileURLToPath(new URL("..", import.meta.url))] } },
  build: { outDir: "../dist-explorer", emptyOutDir: true },
});
