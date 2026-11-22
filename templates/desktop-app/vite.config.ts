import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

const QUAG = "/home/primitation/APP-UI/quag-theme";

export default defineConfig({
  plugins: [solid()],
  resolve: {
    alias: { "quag-solid": `${QUAG}/solid/src/index.ts` },
    // one copy of Solid, even though quag-solid's source lives in another folder
    dedupe: ["solid-js"],
  },
  server: { fs: { allow: [".", QUAG] } },
});
