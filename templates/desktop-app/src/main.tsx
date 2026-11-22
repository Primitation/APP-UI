/* A new Quag desktop app. Read APP-UI/DESIGN.md (§8 desktop) and STRUCTURE.md before adding UI.
 * Levels: atoms → molecules (from quag-solid) → elements (src/elements.tsx) → pages (src/pages/). */
import "/home/primitation/APP-UI/quag-theme/web/quag.css";
import { render } from "solid-js/web";
import { HomePage } from "./pages/HomePage";

render(() => <HomePage />, document.getElementById("root")!);
