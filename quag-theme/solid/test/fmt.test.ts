import { describe, expect, it } from "vitest";
import { age, bytes, bytesOf, uptime } from "../src/fmt";

const MIB = 1024 * 1024, GIB = 1024 * MIB;

// Same cases as the Rust `fmt` tests: both stacks must print the same text.
describe("fmt", () => {
  it("bytes", () => {
    expect(bytes(512)).toBe("512 B");
    expect(bytes(412 * MIB)).toBe("412 MiB");
    expect(bytes(1_503_238_553)).toBe("1.4 GiB");
    expect(bytes(GIB)).toBe("1 GiB");
  });
  it("bytesOf uses the limit's unit", () => {
    expect(bytesOf(412 * MIB, 512 * MIB)).toBe("412 / 512 MiB");
    expect(bytesOf(900 * MIB, 2 * GIB)).toBe("0.9 / 2 GiB");
  });
  it("age and uptime", () => {
    expect(age(3)).toBe("now");
    expect(age(190)).toBe("3m ago");
    expect(uptime(12_000)).toBe("3h 20m");
    expect(uptime(187_200)).toBe("2d 4h");
  });
});
