/** forge demo data: game design assets. Same numbers as the terminal explorer's forge.rs. */
import type { NodeSpec, Pt, Range, TrackSpec, WireSpec } from "../../src";

export interface CurveAsset { name: string; what: string; x: Range; y: Range; xUnit: string; yUnit: string; points: Pt[] }

export const CURVES: CurveAsset[] = [
  { name: "difficulty", what: "Enemy stat multiplier over a run.", x: [0, 60], y: [0.5, 3], xUnit: "min", yUnit: "×", points: [[0, 0.8], [10, 1], [25, 1.4], [40, 2.1], [50, 2.2], [60, 2.8]] },
  { name: "xp to next level", what: "XP needed for each level.", x: [1, 50], y: [0, 50_000], xUnit: "", yUnit: "xp", points: [[1, 100], [10, 2_400], [20, 8_000], [35, 22_000], [50, 48_000]] },
  { name: "damage falloff", what: "Damage share by distance.", x: [0, 40], y: [0, 1], xUnit: "m", yUnit: "×", points: [[0, 1], [8, 1], [20, 0.55], [32, 0.2], [40, 0.1]] },
];

export const LOOT: [string, [string, number][]][] = [
  ["Slime", [["Gel", 50], ["Coin pouch", 30], ["Small potion", 15], ["Slime crown", 5]]],
  ["Golem", [["Stone", 60], ["Iron ore", 25], ["Golem core", 10], ["Ancient rune", 4], ["Nothing", 1]]],
  ["Mire King (boss)", [["Crown of reeds", 40], ["Bog blade", 30], ["Mire scale ×5", 25], ["Quagsire charm", 5]]],
];

export type Stats = [hp: number, dps: number, cost: number];
export interface Unit { name: string; cls: string; now: Stats; prev: Stats }
export const UNITS: Unit[] = [
  { name: "Pikeman", cls: "infantry", now: [420, 18, 60], prev: [400, 18, 60] },
  { name: "Archer", cls: "ranged", now: [260, 22, 75], prev: [260, 20, 75] },
  { name: "Knight", cls: "cavalry", now: [780, 30, 140], prev: [780, 34, 140] },
  { name: "Mage", cls: "ranged", now: [220, 48, 120], prev: [220, 36, 110] },
  { name: "Golem", cls: "siege", now: [1500, 25, 200], prev: [1500, 25, 220] },
  { name: "Scout", cls: "cavalry", now: [300, 12, 50], prev: [300, 12, 50] },
];

export const SEQUENCE: TrackSpec[] = [
  { name: "Camera", clips: [{ start: 0, len: 6, label: "wide: bog" }, { start: 6, len: 8, label: "dolly → king" }, { start: 14, len: 10, label: "close-up" }, { start: 24, len: 12, label: "orbit fight" }] },
  { name: "Dialogue", clips: [{ start: 4, len: 5, label: "KING: Who…" }, { start: 15, len: 7, label: "HERO: Your reign ends" }] },
  { name: "Music", clips: [{ start: 0, len: 14, label: "mire_ambient" }, { start: 14, len: 26, label: "boss_theme" }] },
  { name: "SFX", clips: [{ start: 2, len: 1.5, label: "splash" }, { start: 13, len: 2, label: "roar" }, { start: 24, len: 1, label: "hit" }] },
  { name: "Events", clips: [{ start: 23.5, len: 1, label: "spawn adds" }, { start: 36, len: 1, label: "phase 2" }] },
];
export const SEQ_SPAN: Range = [0, 40];

/** DPS per 100 gold. */
export const value = (s: Stats) => (s[1] / s[2]) * 100;
export const median = () => { const v = UNITS.map((u) => value(u.now)).sort((a, b) => a - b); return v[Math.floor(v.length / 2)]; };

/** The "fireball" ability: two events, each a chain left → right (same graph as forge.rs). */
export const GRAPH: NodeSpec[] = [
  { title: "On Cast", inputs: [], outputs: ["exec", "caster"] },
  { title: "Has mana?", inputs: ["exec", "caster"], outputs: ["true", "false"] },
  { title: "Spend mana", inputs: ["exec"], outputs: ["exec"] },
  { title: "Spawn projectile", inputs: ["exec"], outputs: ["exec"] },
  { title: "Say \"No mana\"", inputs: ["exec"], outputs: [] },
  { title: "On Hit", inputs: [], outputs: ["exec", "target"] },
  { title: "Deal damage", inputs: ["exec", "target"], outputs: ["exec", "target"] },
  { title: "Apply burn", inputs: ["exec", "target"], outputs: [] },
];
export const GRAPH_AT: Pt[] = [[24, 24], [244, 24], [464, 24], [684, 24], [464, 140], [24, 280], [244, 280], [464, 280]];
export const WIRES: WireSpec[] = [
  { from: [0, 0], to: [1, 0] }, { from: [0, 1], to: [1, 1] },
  { from: [1, 0], to: [2, 0] }, { from: [1, 1], to: [4, 0] }, { from: [2, 0], to: [3, 0] },
  { from: [5, 0], to: [6, 0] }, { from: [5, 1], to: [6, 1] },
  { from: [6, 0], to: [7, 0] }, { from: [6, 1], to: [7, 1] },
];
export const NODE_PARAMS: [string, string][][] = [
  [["Trigger", "ability key pressed"]], [["Needs", "20 mana"]], [["Cost", "20 mana"]],
  [["Speed", "18 m/s"], ["Lifetime", "2 s"]], [["Line", "Not enough mana."]], [["Trigger", "projectile hits"]],
  [["Amount", "35"], ["Type", "fire"]], [["Damage", "4 / s"], ["Duration", "3 s"]],
];

/** Where the project data stands: shown by every forge panel the same way. */
export type Load = "ready" | "loading" | "failed";
