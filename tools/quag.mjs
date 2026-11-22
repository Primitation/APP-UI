#!/usr/bin/env node
// quag: copy Quag UI parts from this repo INTO an app. Apps never depend on APP-UI; they own their copy.
//
//   node tools/quag.mjs setup                          tell Claude Code where this clone is (~/.claude/CLAUDE.md)
//   node tools/quag.mjs list [tui|desktop]             the parts you can copy
//   node tools/quag.mjs new <tui|desktop> <dir>        start an app from a template (+ the parts it uses)
//   node tools/quag.mjs add <app-dir> <part...>        copy parts (and what they need) into the app
//   node tools/quag.mjs unreal <project-dir>           copy the QuagStyle plugin into <project>/Plugins
//   node tools/quag.mjs check                          repo self-test: every part builds on its own
//
// Options: --force (replace copies the app changed), --mascot (new / unreal: include the mascot, PRIVATE apps only).
// A part is named by its id (`elements/empty_state`, `atoms/Button`), its file name, or a name it exports (`EmptyState`).
// Node 18+, no dependencies. Works the same on Windows, Linux and macOS.

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { execFileSync, spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const RUST = path.join(ROOT, "quag-theme");
const SOLID = path.join(RUST, "solid");
const WEB = path.join(RUST, "web");
const TEMPLATES = path.join(ROOT, "templates");
const KIT_DIR = "quag"; // the app's copy lives in src/quag/
const MANIFEST = "quag.json";
const LEVELS = ["atoms", "molecules", "elements", "ui"];

const slash = (p) => p.split(path.sep).join("/");
const read = (p) => fs.readFileSync(p, "utf8");
const die = (msg) => { console.error(`quag: ${msg}`); process.exit(1); };

// ───────────────────────────── the two kits ─────────────────────────────
//
// A kit = every copyable file of one stack: { id, src, dest, core, names, deps(text) }.
// `id` is the part's name (path inside the kit, no extension); `dest` is its path inside the app's src/quag/.
// Core files are copied into every app; parts only when asked (or needed by another copied file).

function rustKit() {
  const files = [];
  const add = (id, src, dest, core = false) => files.push({ id, src, dest, core });
  add("core", path.join(RUST, "src/lib.rs"), "mod.rs", true);
  for (const f of ["consts", "fmt", "keymap", "ui"]) add(f, path.join(RUST, `src/${f}.rs`), `${f}.rs`, true);
  for (const lv of ["atoms", "molecules", "elements"])
    for (const f of fs.readdirSync(path.join(RUST, "src", lv)).sort())
      if (f.endsWith(".rs") && f !== "mod.rs" && f !== "tests.rs")
        add(`${lv}/${f.slice(0, -3)}`, path.join(RUST, "src", lv, f), `${lv}/${f}`);
  add("mascot", path.join(RUST, "src/mascot.rs"), "mascot.rs");
  for (const f of files) {
    const text = read(f.src);
    f.names = rustNames(text);
    f.text = text;
  }
  // Which file defines each name (core files and parts alike).
  const owner = new Map();
  for (const f of files) for (const n of f.names) if (!owner.has(n)) owner.set(n, f);
  for (const f of files) {
    const used = rustUses(f.text);
    f.deps = [...new Set([...used].map((n) => owner.get(n)).filter((o) => o && o !== f))];
  }
  return { stack: "tui", files, extras: { mascot: [[path.join(RUST, "assets/quagsire.gif"), "quagsire.gif"]] } };
}

/** Top-level public names of a Rust file. */
function rustNames(text) {
  const names = new Set();
  for (const m of text.matchAll(/^pub (?:const )?(?:fn|struct|enum|const|static|type|trait|mod) ([A-Za-z_]\w*)/gm)) names.add(m[1]);
  for (const m of text.matchAll(/^pub use [^;]*?([A-Za-z_]\w*)\s*;/gm)) names.add(m[1]);
  for (const m of text.matchAll(/^pub use [^;{]*\{([^}]*)\}\s*;/gm))
    for (const n of m[1].split(",")) { const t = n.trim().split(/\s+as\s+/).pop(); if (t) names.add(t); }
  return names;
}

/** Names a Rust file refers to: types/consts anywhere, functions only when called or path-qualified. */
function rustUses(text) {
  const code = text
    .replace(/"(?:\\.|[^"\\])*"/g, '""')
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .replace(/\/\/.*$/gm, "");
  const used = new Set();
  for (const m of code.matchAll(/(?<![\w.])([A-Za-z_]\w*)(?=(\s*\()?)/g)) {
    const [, name, call] = m;
    const before = code.slice(Math.max(0, m.index - 3), m.index);
    if (/^[A-Z]/.test(name)) used.add(name);
    else if (before.endsWith("::") || (call && !/fn\s$/.test(code.slice(Math.max(0, m.index - 3), m.index)))) used.add(name);
  }
  for (const m of code.matchAll(/\buse\s[^;]*;/g)) for (const n of m[0].matchAll(/[A-Za-z_]\w*/g)) used.add(n[0]);
  return used;
}

function solidKit() {
  const files = [];
  const add = (id, src, dest, core = false) => files.push({ id, src, dest, core });
  for (const f of ["constants.ts", "fmt.ts", "keymap.tsx", "ui/AppShell.tsx"]) add(f.replace(/\.tsx?$/, ""), path.join(SOLID, "src", f), f, true);
  add("quag.css", path.join(WEB, "quag.css"), "quag.css", true);
  add("game", path.join(SOLID, "src/game.ts"), "game.ts");
  for (const lv of LEVELS)
    for (const f of fs.readdirSync(path.join(SOLID, "src", lv)).sort())
      if (/\.tsx?$/.test(f) && f !== "index.ts" && !files.some((x) => x.dest === `${lv}/${f}`))
        add(`${lv}/${f.replace(/\.tsx?$/, "")}`, path.join(SOLID, "src", lv, f), `${lv}/${f}`);
  add("mascot", null, null);
  const byDest = new Map(files.map((f) => [f.dest, f]));
  for (const f of files) {
    f.text = f.src ? read(f.src) : "";
    f.names = new Set([...f.text.matchAll(/^export (?:declare )?(?:async )?(?:function|const|let|class|interface|type|enum)\s+([A-Za-z_]\w*)/gm)].map((m) => m[1]));
    f.deps = [];
    if (!f.dest || !/\.tsx?$/.test(f.dest)) continue;
    for (const m of f.text.matchAll(/from\s+"(\.\.?\/[^"]+)"/g)) {
      const base = path.posix.normalize(path.posix.join(path.posix.dirname(f.dest), m[1]));
      const dep = byDest.get(`${base}.ts`) ?? byDest.get(`${base}.tsx`);
      if (!dep) die(`${f.id}: can't resolve import "${m[1]}"`);
      f.deps.push(dep);
    }
  }
  return {
    stack: "desktop", files,
    extras: { mascot: [[path.join(WEB, "quagsire.gif"), "quagsire.gif"], [path.join(WEB, "quagsire-still.png"), "quagsire-still.png"]] },
  };
}

const kitFor = (stack) => (stack === "tui" ? rustKit() : stack === "desktop" ? solidKit() : die(`unknown stack "${stack}" (tui | desktop)`));

/** Finds a part by id, file name, or exported name. */
function findPart(kit, query) {
  const q = query.toLowerCase().replace(/\.(rs|tsx?|css)$/, "");
  const norm = (s) => s.toLowerCase().replace(/[_-]/g, "");
  const byId = kit.files.filter((f) => f.id.toLowerCase() === q);
  if (byId.length) return byId[0];
  const byName = kit.files.filter((f) => f.names.has(query));
  if (byName.length === 1) return byName[0];
  const byStem = kit.files.filter((f) => norm(f.id.split("/").pop()) === norm(q));
  if (byStem.length === 1) return byStem[0];
  const hits = [...new Set([...byName, ...byStem])];
  if (hits.length > 1) die(`"${query}" matches ${hits.map((f) => f.id).join(", ")}: say which one`);
  die(`no part "${query}" in the ${kit.stack} kit (run: node tools/quag.mjs list ${kit.stack})`);
}

/** The files to copy for these parts: core + parts + everything they need. */
function closure(kit, parts) {
  const out = new Set();
  const visit = (f) => { if (out.has(f)) return; out.add(f); f.deps.forEach(visit); };
  kit.files.filter((f) => f.core).forEach(visit);
  parts.forEach(visit);
  return [...out];
}

// ───────────────────────────── writing into an app ─────────────────────────────

function detectStack(app) {
  const m = path.join(app, MANIFEST);
  if (fs.existsSync(m)) return JSON.parse(read(m)).stack;
  if (fs.existsSync(path.join(app, "Cargo.toml"))) return "tui";
  if (fs.existsSync(path.join(app, "package.json"))) return "desktop";
  die(`${app}: no Cargo.toml or package.json, so I can't tell the stack`);
}

/** How a kit file reads once it lives in the app (Rust: `crate::` → `crate::quag::`). */
function transform(kit, f, text) {
  if (kit.stack !== "tui") return text;
  text = text.replace(/\bcrate::/g, `crate::${KIT_DIR}::`).replace(/\bcrate as\b/g, `crate::${KIT_DIR} as`);
  if (f.id === "mascot") text = text.replace(/include_bytes!\("[^"]*quagsire\.gif"\)/, 'include_bytes!("quagsire.gif")');
  return text;
}

/** Writes `dest`, unless the app changed its copy (then it's reported and kept, or replaced with --force). */
function put(dest, data, opts, report) {
  const rel = slash(path.relative(opts.app, dest));
  if (fs.existsSync(dest)) {
    const old = fs.readFileSync(dest);
    if (Buffer.compare(old, Buffer.from(data)) === 0) return;
    if (!opts.force && !opts.regenerated) { report.kept.push(rel); return; }
  }
  fs.mkdirSync(path.dirname(dest), { recursive: true });
  fs.writeFileSync(dest, data);
  report.wrote.push(rel);
}

function generateRust(kit, dir, opts, report) {
  const present = (rel) => fs.existsSync(path.join(dir, rel));
  // level mod.rs: the library's doc + one `mod x; pub use x::*;` per copied file
  for (const lv of ["atoms", "molecules", "elements"]) {
    const d = path.join(dir, lv);
    if (!fs.existsSync(d)) continue;
    const lib = read(path.join(RUST, "src", lv, "mod.rs"));
    const doc = lib.split("// quag:parts")[0].trimEnd();
    const order = [...lib.matchAll(/^mod (\w+);/gm)].map((m) => m[1]);
    const have = fs.readdirSync(d).filter((f) => f.endsWith(".rs") && f !== "mod.rs").map((f) => f.slice(0, -3));
    have.sort((a, b) => (order.indexOf(a) + 1 || 1e9) - (order.indexOf(b) + 1 || 1e9));
    const body = have.map((n) => `mod ${n};\npub use ${n}::*;`).join("\n");
    put(path.join(d, "mod.rs"), `${doc}\n\n// quag:parts (written by quag add: one line pair per copied file)\n${body}\n`, { ...opts, regenerated: true }, report);
  }
  // src/quag/mod.rs: the library root with only the modules that were copied
  const lib = read(path.join(RUST, "src/lib.rs"));
  const [head, rest] = lib.split("// quag:mods-start");
  const tail = rest.split("// quag:mods-end")[1];
  const mods = ["atoms", "consts", "elements", "fmt", "keymap", "molecules", "ui"]
    .filter((m) => present(`${m}.rs`) || present(`${m}/mod.rs`)).map((m) => `pub mod ${m};`);
  if (present("mascot.rs")) mods.push("", '#[cfg(feature = "mascot")]', "pub mod mascot;");
  const docEnd = head.match(/^(?:\/\/![^\n]*\n)*/)[0]; // [^\n], not `.`: `.` stops at the \r of CRLF files
  const note = "//!\n//! Copied from APP-UI by `quag add` (parts listed in quag.json). This app owns this copy.\n\n";
  const allow = "#![allow(dead_code, unused_imports)] // a copied kit: an app doesn't use every helper of every part\n";
  const text = docEnd + note + allow + head.slice(docEnd.length) + `// quag:mods (written by quag add)\n${mods.join("\n")}\n` + tail;
  put(path.join(dir, "mod.rs"), transform(kit, { id: "core" }, text), { ...opts, regenerated: true }, report);
}

function generateSolid(kit, dir, opts, report) {
  const lines = ["/** The Quag parts this app copied from APP-UI (written by `quag add`; the app owns them). */"];
  const order = kit.files.filter((f) => f.dest && /\.tsx?$/.test(f.dest));
  for (const f of order) {
    if (!fs.existsSync(path.join(dir, f.dest))) continue;
    const spec = "./" + f.dest.replace(/\.tsx?$/, "");
    if (f.id === "fmt") lines.push(`export * as fmt from "${spec}";`);
    else if (f.id === "constants") lines.push(`export * as consts from "${spec}";`);
    else lines.push(`export * from "${spec}";`);
  }
  put(path.join(dir, "index.ts"), lines.join("\n") + "\n", { ...opts, regenerated: true }, report);
}

function addParts(app, queries, opts = {}) {
  app = path.resolve(app);
  if (!fs.existsSync(app)) die(`${app} doesn't exist`);
  const stack = detectStack(app);
  const kit = kitFor(stack);
  const manifestPath = path.join(app, MANIFEST);
  const manifest = fs.existsSync(manifestPath) ? JSON.parse(read(manifestPath)) : { stack, parts: [] };
  const wanted = [...new Set([...(opts.keepManifest ? manifest.parts : []), ...queries])].map((q) => findPart(kit, q));
  const dir = path.join(app, "src", KIT_DIR);
  const report = { wrote: [], kept: [] };
  const o = { app, force: !!opts.force };
  for (const f of closure(kit, wanted)) {
    // (Rust's root mod.rs is written by generateRust, with only the modules that were copied.)
    if (f.dest && !(stack === "tui" && f.id === "core")) put(path.join(dir, f.dest), transform(kit, f, f.text), o, report);
    for (const [src, dest] of (f.id === "mascot" && kit.extras.mascot) || []) put(path.join(dir, dest), fs.readFileSync(src), o, report);
  }
  (stack === "tui" ? generateRust : generateSolid)(kit, dir, o, report);
  manifest.parts = [...new Set([...manifest.parts, ...wanted.map((f) => f.id)])].sort();
  manifest.from = repoVersion();
  put(manifestPath, JSON.stringify({ stack, ...manifest }, null, 2) + "\n", { ...o, regenerated: true }, report);

  if (!opts.quiet) {
    console.log(`${stack} app ${app}: ${report.wrote.length} file(s) written${report.wrote.length ? ":" : ""}`);
    report.wrote.forEach((r) => console.log(`  + ${r}`));
  }
  if (report.kept.length) {
    console.log(`kept (the app changed them; --force replaces them with APP-UI's version):`);
    report.kept.forEach((r) => console.log(`  ! ${r}`));
  }
  if (!opts.quiet && wanted.some((f) => f.id === "mascot"))
    console.log("mascot: PRIVATE apps only (DESIGN.md §6)." + (stack === "tui" ? ' Build with `--features mascot` (the template\'s Cargo.toml declares it).' : ""));
  return report;
}

function repoVersion() {
  try { return execFileSync("git", ["-C", ROOT, "rev-parse", "--short", "HEAD"], { encoding: "utf8" }).trim(); }
  catch { return "unknown"; }
}

// ───────────────────────────── commands ─────────────────────────────

function cmdSetup() {
  const home = path.join(os.homedir(), ".claude");
  const target = path.join(home, "CLAUDE.md");
  const block = read(path.join(ROOT, "tools/claude-block.md")).replaceAll("{{APP_UI}}", slash(ROOT)).trim();
  const START = "<!-- quag:start (written by APP-UI/tools/quag.mjs setup; run it again after moving the clone) -->";
  const END = "<!-- quag:end -->";
  fs.mkdirSync(home, { recursive: true });
  let text = fs.existsSync(target) ? read(target) : "";
  const re = /<!-- quag:start[^>]*-->[\s\S]*?<!-- quag:end -->/;
  const section = `${START}\n${block}\n${END}`;
  // The README's earlier hand-pasted block: its heading up to the next heading, marker or the end of the file.
  const old = /^# UI design \(all my apps\)\r?\n[\s\S]*?(?=\r?\n# |\r?\n<!--|(?![\s\S]))/m;
  if (re.test(text)) text = text.replace(re, section);
  else if (old.test(text)) text = text.replace(old, section + "\n");
  else text = (text.trimEnd() ? text.trimEnd() + "\n\n" : "") + section + "\n";
  fs.writeFileSync(target, text);
  console.log(`Claude Code now uses Quag from ${slash(ROOT)} (written to ${target}).`);
}

function cmdList(stack) {
  for (const s of stack ? [stack] : ["tui", "desktop"]) {
    const kit = kitFor(s);
    console.log(`\n${s === "tui" ? "TERMINAL (Rust / ratatui)" : "DESKTOP (Solid + quag.css)"}  ·  always copied: ${kit.files.filter((f) => f.core).map((f) => f.id).join(", ")}`);
    let level = "";
    for (const f of kit.files.filter((f) => !f.core)) {
      const lv = f.id.includes("/") ? f.id.split("/")[0] : "other";
      if (lv !== level) { console.log(`  ${lv}`); level = lv; }
      const names = [...f.names].filter((n) => /^[A-Z]/.test(n) || s === "tui").slice(0, 8).join(" ");
      console.log(`    ${f.id.padEnd(26)} ${names}`);
    }
  }
  console.log("\nCopy with: node tools/quag.mjs add <app-dir> <part or name>...   (live previews: the explorers, see STRUCTURE.md)");
}

function copyDir(src, dest, skip = new Set(["node_modules", "target", "dist", KIT_DIR])) {
  fs.mkdirSync(dest, { recursive: true });
  for (const e of fs.readdirSync(src, { withFileTypes: true })) {
    if (skip.has(e.name)) continue;
    const s = path.join(src, e.name), d = path.join(dest, e.name);
    e.isDirectory() ? copyDir(s, d, skip) : fs.copyFileSync(s, d);
  }
}

function cmdNew(stack, dir, opts) {
  if (!["tui", "desktop"].includes(stack) || !dir) die("usage: new <tui|desktop> <dir>");
  const app = path.resolve(dir);
  if (fs.existsSync(app) && fs.readdirSync(app).length) die(`${app} isn't empty`);
  const tpl = path.join(TEMPLATES, stack === "tui" ? "tui-app" : "desktop-app");
  copyDir(tpl, app);
  const name = path.basename(app).toLowerCase().replace(/[^a-z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "quag-app";
  if (stack === "tui") {
    const p = path.join(app, "Cargo.toml");
    fs.writeFileSync(p, read(p).replace(/^name = "quag-app".*$/m, `name = "${name}"`));
  } else {
    const p = path.join(app, "package.json");
    fs.writeFileSync(p, read(p).replace(/"name": "quag-app"/, `"name": "${name}"`));
  }
  const parts = JSON.parse(read(path.join(tpl, MANIFEST))).parts;
  addParts(app, opts.mascot ? [...parts, "mascot"] : parts, { quiet: true, keepManifest: true });
  console.log(`New ${stack} app in ${app} (from templates/${path.basename(tpl)}, parts copied into src/${KIT_DIR}/).`);
  console.log(stack === "tui" ? `  cd ${dir} && cargo run` : `  cd ${dir} && npm install && npm run dev`);
}

function cmdUnreal(project, opts) {
  if (!project) die("usage: unreal <project-dir> [--mascot]");
  const dest = path.join(path.resolve(project), "Plugins", "QuagStyle");
  const src = path.join(RUST, "unreal", "QuagStyle");
  const skip = new Set(["Binaries", "Intermediate", ...(opts.mascot ? [] : ["Mascot"])]);
  if (fs.existsSync(dest) && !opts.force) die(`${dest} exists (the project owns it; --force replaces it)`);
  copyDir(src, dest, skip);
  console.log(`Copied QuagStyle to ${dest}${opts.mascot ? " (with the mascot: PRIVATE plugins only)" : " (no mascot: it switches itself off)"}.`);
  console.log('Next: add { "Name": "QuagStyle", "Enabled": true } to your .uplugin and PrivateDependencyModuleNames.Add("QuagStyle").');
}

// ───────────────────────────── check: every part builds on its own ─────────────────────────────

/** Runs a program (cargo, or a Node CLI from solid/node_modules: no shell, same on every OS). */
function run(cmd, args, cwd) {
  const bin = { tsc: "typescript/bin/tsc", vite: "vite/bin/vite.js" }[cmd];
  const r = bin
    ? spawnSync(process.execPath, [path.join(SOLID, "node_modules", bin), ...args], { cwd, encoding: "utf8" })
    : spawnSync(cmd, args, { cwd, encoding: "utf8" });
  return { ok: r.status === 0, out: (r.stdout || "") + (r.stderr || "") };
}

function cmdCheck() {
  if (!fs.existsSync(path.join(SOLID, "node_modules"))) die("run `npm install` in quag-theme/solid first");
  const tmp = path.join(ROOT, ".quag-check");
  fs.rmSync(tmp, { recursive: true, force: true });
  let failed = 0;
  const step = (label, res) => {
    console.log(`${res.ok ? "ok  " : "FAIL"} ${label}`);
    if (!res.ok) { failed++; console.log(res.out.split("\n").filter((l) => /error|warning|-->/.test(l)).slice(0, 30).join("\n")); }
  };

  // Rust: one binary per part, each with ONLY that part's closure under src/bin/<part>/quag/. One `cargo check`.
  const rk = rustKit();
  const rdir = path.join(tmp, "rust");
  const cargo = read(path.join(RUST, "Cargo.toml"));
  const dep = (n) => cargo.match(new RegExp(`^${n} = .*$`, "m"))[0];
  const bins = [], seen = new Set();
  for (const part of [null, ...rk.files.filter((f) => !f.core)]) {
    const bin = part ? part.id.replace(/\//g, "-").replace(/_/g, "-") : "core-only";
    const app = path.join(rdir, "app-" + bin);
    fs.mkdirSync(path.join(app, "src"), { recursive: true });
    fs.writeFileSync(path.join(app, "Cargo.toml"), "");
    addParts(app, part ? [part.id] : [], { quiet: true });
    fs.rmSync(path.join(app, "Cargo.toml"));
    fs.rmSync(path.join(app, MANIFEST));
    const bdir = path.join(rdir, "src", "bin", bin);
    fs.mkdirSync(bdir, { recursive: true });
    fs.renameSync(path.join(app, "src", KIT_DIR), path.join(bdir, KIT_DIR));
    fs.writeFileSync(path.join(bdir, "main.rs"), "mod quag;\nfn main() {}\n");
    fs.rmSync(app, { recursive: true });
    bins.push(bin); seen.add(bin);
  }
  fs.writeFileSync(path.join(rdir, "Cargo.toml"),
    `[package]\nname = "quag-check"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n\n[dependencies]\n${dep("crossterm")}\n${dep("ratatui")}\n${dep("image")}\n${dep("ratatui-image")}\n\n[features]\nmascot = ["dep:ratatui-image", "dep:image"]\n\n[workspace]\n`);
  fs.copyFileSync(path.join(RUST, "Cargo.lock"), path.join(rdir, "Cargo.lock"));
  const target = ["--target-dir", path.join(RUST, "target", "quag-check")];
  step(`rust: ${bins.length} parts, each alone`, run("cargo", ["check", "--bins", "--features", "mascot", ...target], rdir));

  // Solid: one folder per part with only its closure, one `tsc` (under solid/ so node_modules resolves).
  const sk = solidKit();
  const sdir = path.join(SOLID, ".quag-check");
  fs.rmSync(sdir, { recursive: true, force: true });
  for (const part of [null, ...sk.files.filter((f) => !f.core)]) {
    const app = path.join(sdir, part ? part.id.replace(/\//g, "-") : "core-only");
    fs.mkdirSync(path.join(app, "src"), { recursive: true });
    fs.writeFileSync(path.join(app, "package.json"), "{}");
    addParts(app, part ? [part.id] : [], { quiet: true });
  }
  fs.writeFileSync(path.join(sdir, "tsconfig.json"), JSON.stringify({ extends: "../tsconfig.json", compilerOptions: { noUnusedLocals: false }, include: ["**/src"] }));
  step(`solid: ${fs.readdirSync(sdir).length - 1} parts, each alone`, run("tsc", ["--noEmit", "-p", sdir], SOLID));
  fs.rmSync(sdir, { recursive: true, force: true });

  // Templates: a new app from each one builds.
  const tui = path.join(tmp, "my-tool");
  cmdNewQuiet("tui", tui);
  step("template tui-app: cargo build", run("cargo", ["build", ...target], tui));
  step("template tui-app --mascot: cargo build", (addParts(tui, ["mascot"], { quiet: true }), run("cargo", ["build", "--features", "mascot", ...target], tui)));
  const desk = path.join(tmp, "my-desk");
  cmdNewQuiet("desktop", desk);
  fs.symlinkSync(path.join(SOLID, "node_modules"), path.join(desk, "node_modules"), "junction");
  const tsc = run("tsc", ["--noEmit", "-p", desk], desk);
  step("template desktop-app: tsc + vite build", tsc.ok ? run("vite", ["build", "--logLevel", "error"], desk) : tsc);

  fs.rmSync(tmp, { recursive: true, force: true });
  if (failed) die(`${failed} check(s) failed`);
  console.log("all parts copy and build on their own");
}

function cmdNewQuiet(stack, dir) {
  const log = console.log; console.log = () => {};
  try { cmdNew(stack, dir, {}); } finally { console.log = log; }
}

// ───────────────────────────── main ─────────────────────────────

const argv = process.argv.slice(2);
const opts = { force: argv.includes("--force"), mascot: argv.includes("--mascot") };
// `~/x` → home/x (PowerShell and cmd don't expand ~ for programs)
const [cmd, ...args] = argv.filter((a) => !a.startsWith("--")).map((a) => (/^~(?=$|[\\/])/.test(a) ? os.homedir() + a.slice(1) : a));
switch (cmd) {
  case "setup": cmdSetup(); break;
  case "list": cmdList(args[0]); break;
  case "new": cmdNew(args[0], args[1], opts); break;
  case "add": if (args.length < 2) die("usage: add <app-dir> <part...>"); addParts(args[0], args.slice(1), opts); break;
  case "unreal": cmdUnreal(args[0], opts); break;
  case "check": cmdCheck(); break;
  default:
    console.log(read(fileURLToPath(import.meta.url)).split("\n").slice(1, 14).map((l) => l.replace(/^\/\/ ?/, "")).join("\n"));
    if (cmd) process.exit(1);
}
