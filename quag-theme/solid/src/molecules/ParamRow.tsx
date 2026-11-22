import { NumberField, type NumberFieldProps, Slider } from "../atoms";

let n = 0;

/** MOLECULE · a tunable number: label · slider · NumberField. Narrow container: the slider hides, the number stays. */
export function ParamRow(props: NumberFieldProps & { label: string; min: number; max: number; labelWidth?: string }) {
  const id = `q-param-${++n}`;
  return (
    <div style={{ "container-type": "inline-size" }}>
      <div class="q-param" style={{ "--label-w": props.labelWidth ?? "130px" }}>
        <label for={id}>{props.label}</label>
        <Slider value={props.value} onValue={props.onValue} min={props.min} max={props.max} step={props.step} aria-label={props.label} disabled={props.disabled} />
        <NumberField {...props} id={id} />
      </div>
    </div>
  );
}
