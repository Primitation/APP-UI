/* A new Quag desktop app. Read APP-UI/DESIGN.md (§8 desktop) and STRUCTURE.md before adding UI.
 * Levels: atoms → molecules (copied from APP-UI into src/quag/, owned by this app) → elements (src/elements.tsx) → pages (src/pages/). */
import "./quag/quag.css";
import { render } from "solid-js/web";
import { HomePage } from "./pages/HomePage";

render(() => <HomePage />, document.getElementById("root")!);
