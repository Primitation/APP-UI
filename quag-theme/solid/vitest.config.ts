import { defineConfig } from "vitest/config";
import solid from "vite-plugin-solid";

// Logic tests of the library (happy-dom: a light DOM, no browser): formats, rules, keymaps. The explorer is checked by screenshots.
export default defineConfig({
  plugins: [solid()],
  test: { root: ".", include: ["test/**/*.test.{ts,tsx}"], environment: "happy-dom" },
});
