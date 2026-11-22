/**
 * KEYMAP · a page's keys, written ONCE. The StatusBar hints, the HelpOverlay (?) and the
 * CommandPalette (Ctrl K) all read it, and `useKeymap` dispatches key presses from it,
 * so what's shown is exactly what works.
 */
import { createSignal, onCleanup, onMount } from "solid-js";
import { CommandPalette } from "./elements/CommandPalette";
import { HelpOverlay, type KeyGroup } from "./elements/HelpOverlay";
import type { MenuItem } from "./molecules/Menu";

export interface Binding {
  /** Displayed key(s), space-separated alternatives: "Ctrl K", "↑ ↓", "Del". */
  key: string;
  what: string;
  group: string;
  /** Short text for the StatusBar; omit = not in the status bar. */
  hint?: string;
  /** Name in the command palette; omit = not a command. */
  command?: string;
  danger?: boolean;
  /** Real key(s) to listen to, KeyboardEvent.key with modifiers: "ctrl+k", "ArrowDown", "?". Omit = display only. */
  match?: string[];
  /** Gets the matched key, so one binding ("↑ ↓", "1 2 3") can cover several keys. */
  run?: (key: string) => void;
}

export class Keymap {
  constructor(readonly bindings: Binding[]) {}
  hints(): [string, string][] {
    return this.bindings.filter((b) => b.hint).map((b) => [b.key, b.hint!]);
  }
  commands(): MenuItem[] {
    return this.bindings
      .filter((b) => b.command)
      .map((b) => ({ label: b.command!, key: b.key, danger: b.danger, onSelect: () => b.run?.(b.match?.[0] ?? "") }))
      .sort((a, b) => Number(!!a.danger) - Number(!!b.danger));
  }
  groups(): KeyGroup[] {
    const out: KeyGroup[] = [];
    for (const b of this.bindings) {
      const g = out.find((x) => x[0] === b.group);
      if (g) g[1].push([b.key, b.what]);
      else out.push([b.group, [[b.key, b.what]]]);
    }
    return out;
  }
  /** The key name used in `match`: "ctrl+k", "ArrowDown", "shift+ArrowDown", "?", "n". Shift only counts on arrows. */
  static nameOf(e: KeyboardEvent): string {
    const shift = e.shiftKey && e.key.startsWith("Arrow") && !(e.ctrlKey || e.metaKey) ? "shift+" : "";
    return `${e.ctrlKey || e.metaKey ? "ctrl+" : ""}${shift}${e.key.length === 1 ? e.key.toLowerCase() : e.key}`;
  }
  find(e: KeyboardEvent): Binding | undefined {
    const name = Keymap.nameOf(e);
    return this.bindings.find((b) => b.match?.includes(name));
  }
}

/** Listen to the page's keys (ignored while typing in a field, except ctrl+ shortcuts). */
export function useKeymap(keymap: () => Keymap) {
  const onKey = (e: KeyboardEvent) => {
    // a focused part (tree, tabs, menu, split…) already used this key: don't act twice
    if (e.defaultPrevented) return;
    // Enter / Space on a focused button or link press it (the browser does): the page doesn't act too
    if ((e.key === "Enter" || e.key === " ") && (e.target as HTMLElement)?.closest?.("button, a[href], [role=button]")) return;
    const typing = (e.target as HTMLElement)?.closest?.("input, textarea, select");
    if (typing && !(e.ctrlKey || e.metaKey)) return;
    if (document.querySelector(".q-backdrop")) return; // an overlay owns the keys
    const b = keymap().find(e);
    if (b?.run) { e.preventDefault(); b.run(Keymap.nameOf(e)); }
  };
  onMount(() => document.addEventListener("keydown", onKey));
  onCleanup(() => document.removeEventListener("keydown", onKey));
}

/**
 * Everything a page needs for its keys, from ONE list:
 *   const keys = createPageKeys([...bindings]);
 *   <AppShell status={<StatusBar hints={keys.hints()} />}> … {keys.overlays} </AppShell>
 * Adds the keys every page has (Ctrl K commands, ? help, Esc), listens to them,
 * and renders the CommandPalette and HelpOverlay, both read from the same list.
 */
export function createPageKeys(bindings: Binding[]) {
  const [palette, setPalette] = createSignal(false);
  const [help, setHelp] = createSignal(false);
  const keymap = new Keymap([
    ...bindings,
    { key: "Ctrl K", what: "commands", group: "App", hint: "commands", match: ["ctrl+k"], run: () => setPalette(true) },
    { key: "?", what: "this help", group: "App", hint: "keys", command: "Show keys", match: ["?"], run: () => setHelp(true) },
    { key: "Esc", what: "close / back", group: "App" },
  ]);
  useKeymap(() => keymap);
  const overlays = (
    <>
      <CommandPalette open={palette()} items={keymap.commands()} onClose={() => setPalette(false)} />
      <HelpOverlay open={help()} groups={keymap.groups()} onClose={() => setHelp(false)} />
    </>
  );
  return { keymap, hints: () => keymap.hints(), overlays, openHelp: () => setHelp(true), openPalette: () => setPalette(true) };
}
