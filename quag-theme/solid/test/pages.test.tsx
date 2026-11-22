import { afterEach, describe, expect, it } from "vitest";
import { render } from "solid-js/web";
import { PAGES } from "../explorer/pond/pages";

// The explorer's pages, driven with real key events (happy-dom): who owns a key, and when.
let dispose: (() => void) | undefined;
afterEach(() => { dispose?.(); dispose = undefined; document.body.innerHTML = ""; });

const mount = (id: string) => {
  const page = PAGES.find((p) => p.id === id);
  if (!page) throw new Error(`no page ${id}`);
  const root = document.createElement("div");
  document.body.append(root);
  const Page = page.page;
  dispose = render(() => <Page />, root);
};
/** Press a key on `target` (bubbles up to the page's listeners, like a real press). Returns the event. */
const press = (key: string, target: EventTarget = document.body, init: KeyboardEventInit = {}) => {
  const e = new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true, ...init });
  target.dispatchEvent(e);
  return e;
};
const open = () => document.querySelectorAll(".q-backdrop").length;

describe("page keys", () => {
  it("no page takes Tab: the browser moves focus", () => {
    for (const p of PAGES) {
      mount(p.id);
      expect(press("Tab").defaultPrevented, p.id).toBe(false);
      dispose?.(); dispose = undefined; document.body.innerHTML = "";
    }
  });

  it("Esc closes help / palette and is marked handled, so the explorer doesn't also go back", () => {
    for (const p of PAGES) {
      for (const [opener, init] of [["?", {}], ["k", { ctrlKey: true }]] as const) {
        mount(p.id);
        press(opener, document.body, init);
        expect(open(), `${p.id} ${opener} opens`).toBe(1);
        const esc = press("Escape", document.activeElement ?? document.body);
        expect(open(), `${p.id}: closed`).toBe(0);
        expect(esc.defaultPrevented, `${p.id}: Esc marked handled`).toBe(true);
        dispose?.(); dispose = undefined; document.body.innerHTML = "";
      }
    }
  });

  it("an arrow used by a focused tree doesn't also move the page's list", () => {
    mount("containers");
    const row = () => document.querySelector(".q-table [aria-selected=true], .q-table .is-selected")?.textContent;
    const tree = document.querySelector<HTMLElement>(".q-tree")!;
    const treeSel = () => tree.querySelector("[aria-selected=true], .is-selected")?.textContent;
    const [r0, t0] = [row(), treeSel()];
    tree.focus();
    press("ArrowDown", tree);
    expect(treeSel()).not.toBe(t0);
    expect(row()).toBe(r0);
    press("ArrowDown"); // not in the tree: the page's list moves
    expect(row()).not.toBe(r0);
  });

  it("Enter on a focused button presses only the button, not the page's Enter too", () => {
    mount("hosts");
    const retry = [...document.querySelectorAll<HTMLButtonElement>("button")].find((b) => b.textContent?.includes("Retry"))!;
    let clicks = 0;
    retry.addEventListener("click", () => clicks++);
    retry.focus();
    const e = press("Enter", retry);
    expect(e.defaultPrevented).toBe(false); // the browser's own press goes ahead
    expect(document.querySelector(".q-toast")).toBeNull(); // the page's "Enter = open host" didn't run
    press("Enter"); // not on a button: the page's Enter works
    expect(document.querySelector(".q-toast")).not.toBeNull();
    expect(clicks).toBe(0); // happy-dom doesn't synthesize the click; the browser does
  });

  it("Del asks first; Esc keeps the task", () => {
    mount("tasks");
    const count = () => document.querySelectorAll(".q-list [role=option], .q-list li").length;
    const n = count();
    press("Delete");
    expect(document.querySelector("[role=alertdialog]")).not.toBeNull();
    press("Escape", document.activeElement ?? document.body);
    expect(open()).toBe(0);
    expect(count()).toBe(n);
  });

  it("tabs follow the data: Done lists the finished task", () => {
    mount("tasks");
    press("3");
    expect(document.body.textContent).toContain("Pick a color palette");
    press("2");
    expect(document.body.textContent).not.toContain("Fix save crash");
  });
});
