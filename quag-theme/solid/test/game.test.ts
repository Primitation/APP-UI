import { describe, expect, it } from "vitest";
import { curveNudge, nodeHeight, nudgePoint, portPoint, rulerStep, shares, wirePath } from "../src/game";
import { NODE_HEAD_H, NODE_PAD_Y, NODE_ROW_H, NODE_W, WIRE_BEND_MIN } from "../src/constants";
import { Keymap } from "../src/keymap";

describe("game design helpers (same rules as the Rust molecules)", () => {
  it("nudge keeps keys in order and in range; ends keep their x", () => {
    const p = nudgePoint([[0, 1], [0.5, 1.5], [1, 2]], 1, 0.9, 5, [0, 1], [0, 3]);
    expect(p[1]).toEqual([1, 3]);
    expect(nudgePoint(p, 0, 0.3, -9, [0, 1], [0, 3])[0]).toEqual([0, 0]);
  });
  it("arrows move by NUDGE of the range, shift by NUDGE_FINE", () => {
    const p: [number, number][] = [[0, 0], [50, 50], [100, 100]];
    expect(curveNudge(p, 1, "ArrowUp", [0, 100], [0, 100])[1]).toEqual([50, 55]);
    expect(curveNudge(p, 1, "shift+ArrowLeft", [0, 100], [0, 100])[1]).toEqual([49, 50]);
  });
  it("shares always total one", () => {
    expect(shares([50, 30, 15, 5]).reduce((a, b) => a + b)).toBeCloseTo(1);
    expect(shares([0, 0])).toEqual([0, 0]);
  });
  it("ruler steps are nice (same as Rust)", () => {
    expect(rulerStep(60, 10)).toBe(10);
    expect(rulerStep(12, 12)).toBe(1);
    expect(rulerStep(30, 4)).toBe(10);
  });
  it("shift only counts on arrows", () => {
    expect(Keymap.nameOf(new KeyboardEvent("keydown", { key: "ArrowUp", shiftKey: true }))).toBe("shift+ArrowUp");
    expect(Keymap.nameOf(new KeyboardEvent("keydown", { key: "?", shiftKey: true }))).toBe("?");
  });
  it("ports sit on the node's edges, row by row; wires leave right and enter left", () => {
    expect(portPoint([100, 50], 0, false)).toEqual([100, 50 + NODE_HEAD_H + NODE_PAD_Y + NODE_ROW_H / 2]);
    expect(portPoint([100, 50], 1, true)[0]).toBe(100 + NODE_W);
    expect(nodeHeight({ title: "x", inputs: ["a", "b"], outputs: [] })).toBe(NODE_HEAD_H + 2 * NODE_PAD_Y + 2 * NODE_ROW_H);
    expect(wirePath([0, 0], [10, 20])).toBe(`M 0 0 C ${WIRE_BEND_MIN} 0, ${10 - WIRE_BEND_MIN} 20, 10 20`);
  });
});
