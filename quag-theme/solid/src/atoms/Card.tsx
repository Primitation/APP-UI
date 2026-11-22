import type { JSX } from "solid-js";

/** ATOM · the box: an empty container (card / panel). Focused = Quag outline. */
export function Card(props: { focused?: boolean; class?: string; style?: JSX.CSSProperties; children?: JSX.Element }) {
  return (
    <div class={`q-card${props.focused ? " is-focused" : ""}${props.class ? " " + props.class : ""}`} style={props.style}>
      {props.children}
    </div>
  );
}
