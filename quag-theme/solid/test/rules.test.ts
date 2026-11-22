import { describe, expect, it } from "vitest";
import { isBusy, isUp, runLabel, runTone, usageTone } from "../src/atoms/servers";
import { USAGE_WARN } from "../src/constants";

describe("usage colors (DESIGN.md: 80 % yellow, 100 % red)", () => {
  it("thresholds", () => {
    expect(usageTone(0.5)).toBeUndefined();
    expect(usageTone(USAGE_WARN)).toBe("warn");
    expect(usageTone(1)).toBe("err");
    expect(usageTone(1.3)).toBe("err");
  });
});

describe("run states (stopped is not an error)", () => {
  it("tones", () => {
    expect(runTone({ kind: "running" })).toBe("ok");
    expect(runTone({ kind: "stopped" })).toBe("neutral");
    expect(runTone({ kind: "paused" })).toBe("neutral");
    expect(runTone({ kind: "exited", code: 0 })).toBe("neutral");
    expect(runTone({ kind: "exited", code: 137 })).toBe("err");
    expect(runTone({ kind: "flapping", restarts: 5 })).toBe("warn");
    expect(runTone({ kind: "restarting" })).toBe("warn");
  });
  it("labels and flags", () => {
    expect(runLabel({ kind: "exited", code: 137 })).toBe("crashed 137");
    expect(runLabel({ kind: "flapping", restarts: 5 })).toBe("restarted 5×");
    expect(isUp({ kind: "flapping", restarts: 2 })).toBe(true);
    expect(isBusy({ kind: "stopping" })).toBe(true);
    expect(isUp({ kind: "stopped" })).toBe(false);
  });
});
