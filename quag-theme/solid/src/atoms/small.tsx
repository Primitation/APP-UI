import { BADGE_MAX } from "../constants";
import { type JSX, Show, For } from "solid-js";

/** The four status kinds. Only for real states; never decoration. */
export type StatusKind = "ok" | "warn" | "err" | "info";

/** ATOM · `#tag` pill (Gill purple). */
export function Tag(props: { children: JSX.Element }) {
  return <span class="q-tag">{props.children}</span>;
}

/** ATOM · keyboard key. */
export function Kbd(props: { children: JSX.Element }) {
  return <kbd>{props.children}</kbd>;
}

/** ATOM · status icon + optional text, always color AND icon (✓ ! ✗ i). */
export function Status(props: { kind: StatusKind; children?: JSX.Element }) {
  return <span class={`q-status ${props.kind}`}>{props.children}</span>;
}

/** ATOM · thin progress bar. Put the numbers next to it, never on it. */
export function Progress(props: { value: number; label?: string }) {
  return (
    <div class="q-progress" role="progressbar" aria-valuenow={Math.round(props.value * 100)} aria-valuemin={0} aria-valuemax={100} aria-label={props.label}>
      <span style={{ width: `${Math.max(0, Math.min(1, props.value)) * 100}%` }} />
    </div>
  );
}

/** ATOM · smooth progress: the fill glides to each new value. For things that move by small amounts (downloads, copies). */
export function SmoothProgress(props: { value: number; label?: string }) {
  return (
    <div class="q-progress is-smooth" role="progressbar" aria-valuenow={Math.round(props.value * 100)} aria-valuemin={0} aria-valuemax={100} aria-label={props.label}>
      <span style={{ width: `${Math.max(0, Math.min(1, props.value)) * 100}%` }} />
    </div>
  );
}

/** ATOM · progress by steps: one segment per step. Done steps are full, the current one fills with `current` (0–1),
 *  `failed` turns the current step red. Say which step next to it: "Step 3 of 5 · Health check". */
export function StepProgress(props: { total: number; done: number; current?: number; failed?: boolean; label?: string }) {
  const fill = (i: number) => (i < props.done ? 1 : i === props.done ? Math.max(0, Math.min(1, props.current ?? 0)) : 0);
  return (
    <div class={`q-step-progress${props.failed ? " is-failed" : ""}`} role="progressbar" aria-valuenow={props.done} aria-valuemin={0}
      aria-valuemax={props.total} aria-label={props.label}>
      <For each={Array.from({ length: props.total }, (_, i) => i)}>
        {(i) => <span class={i === props.done ? "is-current" : undefined}><span style={{ width: `${fill(i) * 100}%` }} /></span>}
      </For>
    </div>
  );
}

/** ATOM · spinner (with a text label next to it: "Loading…"). */
export function Spinner() {
  return <span class="q-spinner" role="status" aria-label="Loading" />;
}

/** ATOM · divider line. */
export function Divider() {
  return <hr class="q-divider" />;
}

/** `ok` `warn` `err`: text in a status color (a failure title, a danger menu item). */
export type Tone = "title" | "heading" | "text" | "muted" | "faint" | "accent" | "ok" | "warn" | "err";

/** ATOM · text in one of the six tones. Use instead of picking colors. */
export function Text(props: { tone?: Tone; children: JSX.Element; as?: "span" | "p" | "div" }) {
  const cls = () => `q-t-${props.tone ?? "text"}`;
  return props.as === "p" ? <p class={cls()}>{props.children}</p>
    : props.as === "div" ? <div class={cls()}>{props.children}</div>
    : <span class={cls()}>{props.children}</span>;
}

/** ATOM · form label and help/error text. */
export function Label(props: { for?: string; children: JSX.Element }) {
  return <label class="q-label" for={props.for}>{props.children}</label>;
}
export function Help(props: { error?: boolean; children: JSX.Element }) {
  return <div class={props.error ? "q-help is-error" : "q-help"}>{props.children}</div>;
}

/** ATOM · a count (unread, results). 0 shows nothing; over 99 shows 99+. */
export function Badge(props: { count: number; label?: string }) {
  return (
    <Show when={props.count > 0}>
      <span class="q-badge" aria-label={props.label ?? `${props.count}`}>{props.count > BADGE_MAX ? `${BADGE_MAX}+` : props.count}</span>
    </Show>
  );
}

/** ATOM · a link (Quag blue, underlined). */
export function Link(props: { href?: string; onClick?: () => void; children: JSX.Element }) {
  return <a class="q-link" href={props.href ?? "#"} onClick={(e) => { if (props.onClick) { e.preventDefault(); props.onClick(); } }}>{props.children}</a>;
}

/** ATOM · loading placeholder where text will be. Keeps the layout while loading. */
export function Skeleton(props: { width?: string; height?: string }) {
  return <span class="q-skeleton" aria-hidden="true" style={{ width: props.width ?? "100%", height: props.height }} />;
}
