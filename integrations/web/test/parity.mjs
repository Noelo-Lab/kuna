// parity.mjs — the web integration's automated gate.
//
// Proves TWO things without needing a browser:
//   1. The wasm decompiler RUNS client-side (under Node's WASI preview1, the
//      same ABI the browser WASI shim implements), reading the specs + binary
//      from a virtual filesystem.
//   2. Its output is byte-identical to the NATIVE `kuna_wasm` build — i.e. the
//      wasm port is a faithful decompiler, not a degraded one.
//
// It runs `list` + several `decompile` cases over each committed fixture
// (x86-64 ELF + AArch64 object) and diffs native vs wasm (normalizing only the
// absolute `binary` path, which legitimately differs between the host FS and the
// guest's virtual FS). Specs come from the full repo tree, so the loader
// auto-resolves each fixture's architecture.
//
// Usage:  node parity.mjs           (paths auto-resolved from the repo layout)
//         SPECS=... node parity.mjs  (override the spec root)
import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, '../../..'); // integrations/web/test -> repo root

const WASM = process.env.WASM ||
  join(repo, 'decompiler/target/wasm32-wasip1/release/kuna_wasm.wasm');
const NATIVE = process.env.NATIVE ||
  join(repo, 'decompiler/target/release/kuna_wasm');
const SPECS = process.env.SPECS || join(repo, 'specs');
const RUN_WASM = join(here, 'run-wasm.mjs');

// { fixture, arch, cases } — cases are argv tails for kuna_wasm.
const FIXTURES = [
  {
    fixture: join(here, 'fixtures/sample.elf'),
    arch: 'x86-64',
    cases: [['list'], ['decompile'], ['decompile', 'main'], ['decompile', 'sum_to'], ['decompile', 'add']],
  },
  {
    fixture: join(here, 'fixtures/sample_aarch64.o'),
    arch: 'aarch64',
    cases: [['list'], ['decompile'], ['decompile', 'sum_to'], ['decompile', 'add']],
  },
];

function fail(msg) { console.error(`\x1b[31mFAIL\x1b[0m ${msg}`); process.exit(1); }

for (const [label, p] of [['wasm', WASM], ['native', NATIVE], ['specs', SPECS]]) {
  if (!existsSync(p)) fail(`missing ${label}: ${p}\n  (build first: integrations/web/build.sh, and \`make specs\` for the .sla)`);
}

const normalize = (s) => s.replace(/"binary":\s*"[^"]*"/g, '"binary": "<binary>"').trim();
const runNative = (fx, args) =>
  execFileSync(NATIVE, [fx, SPECS, ...args], { encoding: 'utf8', maxBuffer: 64 << 20 });
const runWasm = (fx, args) =>
  execFileSync('node', ['--experimental-wasi-unstable-preview1', RUN_WASM, WASM, SPECS, fx, ...args],
    { encoding: 'utf8', maxBuffer: 64 << 20 });

let passed = 0, total = 0;
for (const { fixture, arch, cases } of FIXTURES) {
  if (!existsSync(fixture)) fail(`missing fixture: ${fixture}`);
  for (const c of cases) {
    total++;
    const label = `${arch}: ${c.join(' ')}`;
    let nat, wsm;
    try { nat = runNative(fixture, c); } catch (e) { fail(`native \`${label}\` crashed: ${e.message}`); }
    try { wsm = runWasm(fixture, c); } catch (e) { fail(`wasm \`${label}\` crashed: ${e.message}`); }
    const n = normalize(nat), w = normalize(wsm);
    if (n !== w) {
      console.error(`--- native (${label})\n${n.slice(0, 800)}`);
      console.error(`+++ wasm   (${label})\n${w.slice(0, 800)}`);
      fail(`native != wasm for \`${label}\``);
    }
    if (!n.includes('"functions"')) fail(`\`${label}\` produced no functions array`);
    console.log(`\x1b[32mOK\x1b[0m   ${label}  (${w.length} bytes, native==wasm)`);
    passed++;
  }
}

console.log(`\n\x1b[32mPARITY OK\x1b[0m — ${passed}/${total} cases across ${FIXTURES.length} arches: wasm runs under WASI and matches native byte-for-byte.`);
