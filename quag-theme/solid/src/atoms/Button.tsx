import { type JSX, splitProps } from "solid-js";

export type ButtonKind = "primary" | "secondary" | "danger" | "ghost";

export interface ButtonProps extends Omit<JSX.ButtonHTMLAttributes<HTMLButtonElement>, "class"> {
  /** primary: max ONE per screen. danger: always followed by a ConfirmDialog. */
  kind?: ButtonKind;
  class?: string;
}

/** ATOM · one button. Generic: name it by what it is, never by what it does. */
export function Button(props: ButtonProps) {
  const [own, rest] = splitProps(props, ["kind", "class", "children", "type"]);
  const kindClass = () => (own.kind && own.kind !== "secondary" ? ` q-${own.kind}` : "");
  return (
    <button type={own.type ?? "button"} class={`q-btn${kindClass()}${own.class ? " " + own.class : ""}`} {...rest}>
      {own.children}
    </button>
  );
}
