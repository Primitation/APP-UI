import { For } from "solid-js";
import { Button, type ButtonKind } from "../atoms/Button";

export interface ButtonSpec {
  label: string;
  kind?: ButtonKind;
  onClick?: () => void;
  disabled?: boolean;
  title?: string;
}

export interface ButtonGroupProps {
  /** The array of buttons. Desktop order rule: primary LAST (rightmost). */
  buttons: ButtonSpec[];
  align?: "start" | "end";
  /** Draw the box around the group. */
  boxed?: boolean;
}

/** MOLECULE · an array of buttons in a box. Doesn't know what the buttons do: the element does. */
export function ButtonGroup(props: ButtonGroupProps) {
  const cls = () => `q-btn-group${props.align === "end" ? " end" : ""}${props.boxed ? " is-boxed" : ""}`;
  return (
    <div class={cls()} role="group">
      <For each={props.buttons}>
        {(b) => (
          <Button kind={b.kind} onClick={b.onClick} disabled={b.disabled} title={b.title}>
            {b.label}
          </Button>
        )}
      </For>
    </div>
  );
}
