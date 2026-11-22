import { For } from "solid-js";
import { Radio } from "../atoms";

let n = 0;

/** MOLECULE · one choice among a few. A row; `column` stacks them (it also wraps when narrow). */
export function RadioGroup(props: { label: string; options: string[]; value: number; onChange?: (i: number) => void; column?: boolean }) {
  const name = `q-radio-${++n}`;
  return (
    <fieldset class={`q-radio-group${props.column ? " is-column" : ""}`} aria-label={props.label}>
      <For each={props.options}>
        {(o, i) => <Radio name={name} checked={props.value === i()} onSelect={() => props.onChange?.(i())}>{o}</Radio>}
      </For>
    </fieldset>
  );
}
