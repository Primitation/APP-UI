import { For } from "solid-js";

/** MOLECULE · one of a few short options as joined buttons (time range, view mode). ←/→ move. */
export function Segmented(props: { options: string[]; value: number; onChange?: (i: number) => void; label?: string }) {
  const key = (e: KeyboardEvent) => {
    if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
    e.preventDefault();
    const n = props.options.length;
    props.onChange?.((props.value + (e.key === "ArrowRight" ? 1 : n - 1)) % n);
  };
  return (
    <div class="q-segmented" role="radiogroup" aria-label={props.label} onKeyDown={key}>
      <For each={props.options}>
        {(o, i) => <button type="button" role="radio" aria-checked={i() === props.value} tabIndex={i() === props.value ? 0 : -1} onClick={() => props.onChange?.(i())}>{o}</button>}
      </For>
    </div>
  );
}
