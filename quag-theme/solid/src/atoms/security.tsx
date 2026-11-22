import { COPIED_MS, MASK, SECRET_MASK_MAX, SECRET_TAIL } from "../constants";
import { createSignal, createEffect, on } from "solid-js";

export type Severity = "critical" | "high" | "medium" | "low" | "info";
export const SEVERITIES: Severity[] = ["critical", "high", "medium", "low", "info"];
const LABEL: Record<Severity, string> = { critical: "CRIT", high: "HIGH", medium: "MED", low: "LOW", info: "INFO" };

/** ATOM · severity of a finding. CRIT filled red, HIGH red outline, MED yellow, LOW blue, INFO grey. Same width. */
export function SeverityBadge(props: { level: Severity }) {
  return <span class={`q-sev ${props.level}`} aria-label={`severity ${props.level}`}>{LABEL[props.level]}</span>;
}

/** ATOM · a secret: masked (last 4 visible), Show/Hide and Copy. Revealed = yellow border (it's exposed). */
/** `shown`: let the page show / hide it too (a key like S); the Show button still works. */
export function SecretField(props: { value: string; label?: string; shown?: boolean }) {
  const [shown, setShown] = createSignal(false);
  createEffect(on(() => props.shown, (v) => { if (v !== undefined) setShown(v); }));
  const [copied, setCopied] = createSignal(false);
  const masked = () => MASK.repeat(Math.min(Math.max(props.value.length - SECRET_TAIL, 0), SECRET_MASK_MAX)) + props.value.slice(-SECRET_TAIL);
  const copy = async () => {
    await navigator.clipboard?.writeText(props.value).catch(() => {});
    setCopied(true);
    setTimeout(() => setCopied(false), COPIED_MS);
  };
  return (
    <div class={`q-secret${shown() ? " is-revealed" : ""}`} role="group" aria-label={props.label ?? "Secret"}>
      <code>{shown() ? props.value : masked()}</code>
      <button type="button" onClick={() => setShown(!shown())} aria-pressed={shown()}>{shown() ? "Hide" : "Show"}</button>
      <button type="button" onClick={copy}>{copied() ? "Copied" : "Copy"}</button>
    </div>
  );
}
