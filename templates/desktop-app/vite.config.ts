import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

// The Quag parts are plain source in src/quag/ (copied from APP-UI): nothing to alias.
export default defineConfig({
  plugins: [solid()],
});
