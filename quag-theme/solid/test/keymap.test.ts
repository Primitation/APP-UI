import { describe, expect, it, vi } from "vitest";
import { Keymap } from "../src/keymap";

const key = (k: string, ctrl = false) => ({ key: k, ctrlKey: ctrl, metaKey: false }) as KeyboardEvent;

describe("Keymap: a page's keys written once", () => {
  const del = vi.fn();
  const km = new Keymap([
    { key: "↑ ↓", what: "move", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"] },
    { key: "Del", what: "delete", group: "Act", command: "Delete task", danger: true, match: ["Delete"], run: del },
    { key: "N", what: "new task", group: "Act", hint: "new", command: "New task", match: ["n"] },
    { key: "Ctrl K", what: "commands", group: "App", hint: "commands", match: ["ctrl+k"] },
  ]);

  it("hints: only bindings with a hint, in order", () => {
    expect(km.hints()).toEqual([["↑ ↓", "move"], ["N", "new"], ["Ctrl K", "commands"]]);
  });
  it("commands: dangerous ones last", () => {
    expect(km.commands().map((c) => c.label)).toEqual(["New task", "Delete task"]);
  });
  it("commands run their binding", () => {
    km.commands().find((c) => c.label === "Delete task")!.onSelect?.();
    expect(del).toHaveBeenCalledWith("Delete");
  });
  it("groups keep first-seen order", () => {
    expect(km.groups().map((g) => g[0])).toEqual(["Move", "Act", "App"]);
  });
  it("find: letters ignore case, ctrl is a prefix", () => {
    expect(km.find(key("N"))?.what).toBe("new task");
    expect(km.find(key("k", true))?.what).toBe("commands");
    expect(km.find(key("ArrowDown"))?.what).toBe("move");
    expect(km.find(key("q"))).toBeUndefined();
  });
});
