import type { JSX } from "solid-js";

export interface CheckboxProps {
  checked: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  /** Render as a switch instead of a box. Same atom, different variant. */
  toggle?: boolean;
  children?: JSX.Element;
}

/** ATOM · checkbox (or toggle). Keeps a real <input> inside for keyboard and screen readers. */
export function Checkbox(props: CheckboxProps) {
  return (
    <label class={props.toggle ? "q-check q-toggle" : "q-check"}>
      <input
        type="checkbox"
        role={props.toggle ? "switch" : undefined}
        checked={props.checked}
        disabled={props.disabled}
        onChange={(e) => props.onChange?.(e.currentTarget.checked)}
      />
      {props.children}
    </label>
  );
}
