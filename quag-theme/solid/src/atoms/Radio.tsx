import type { JSX } from "solid-js";

/** ATOM · one option of a single choice. Keeps a real radio <input> (arrow keys work in a group). */
export function Radio(props: { name: string; checked: boolean; onSelect?: () => void; disabled?: boolean; children?: JSX.Element }) {
  return (
    <label class="q-radio">
      <input type="radio" name={props.name} checked={props.checked} disabled={props.disabled} onChange={() => props.onSelect?.()} />
      {props.children}
    </label>
  );
}
