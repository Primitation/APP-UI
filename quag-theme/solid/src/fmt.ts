/** Number formatting shared by all parts (same rules as the Rust `fmt` module). */
import { BYTE_UNITS, BYTES_STEP, DAY, HOUR, MINUTE, NOW_SECS, ONE_DECIMAL_BELOW } from "./constants";
const UNITS: readonly string[] = BYTE_UNITS;
const trim = (s: string) => (s.includes(".") ? s.replace(/\.?0+$/, "") : s);

/** `512 B`, `412 MiB`, `1.4 GiB` (binary units). */
export function bytes(b: number): string {
  let v = b, i = 0;
  while (v >= BYTES_STEP && i < UNITS.length - 1) { v /= BYTES_STEP; i++; }
  if (i === 0) return `${b} B`;
  return `${v < ONE_DECIMAL_BELOW ? trim(v.toFixed(1)) : v.toFixed(0)} ${UNITS[i]}`;
}
/** `412 / 512 MiB`: used in the limit's unit. */
export function bytesOf(used: number, limit: number): string {
  const [num, unit] = bytes(limit).split(" ");
  const u = used / BYTES_STEP ** Math.max(UNITS.indexOf(unit), 0);
  return `${u < ONE_DECIMAL_BELOW && unit !== "B" ? trim(u.toFixed(1)) : u.toFixed(0)} / ${num} ${unit}`;
}
/** `now`, `12s ago`, `3m ago`, `2h ago`, `4d ago`. */
export function age(s: number): string {
  if (s < NOW_SECS) return "now";
  if (s < MINUTE) return `${s}s ago`;
  if (s < HOUR) return `${Math.floor(s / MINUTE)}m ago`;
  if (s < DAY) return `${Math.floor(s / HOUR)}h ago`;
  return `${Math.floor(s / DAY)}d ago`;
}
/** `45s`, `12m`, `3h 20m`, `2d 4h`. */
export function uptime(s: number): string {
  if (s < MINUTE) return `${s}s`;
  if (s < HOUR) return `${Math.floor(s / MINUTE)}m`;
  if (s < DAY) return `${Math.floor(s / HOUR)}h ${Math.floor((s % HOUR) / MINUTE)}m`;
  return `${Math.floor(s / DAY)}d ${Math.floor((s % DAY) / HOUR)}h`;
}
