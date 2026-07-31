// parity.mjs — the web integration's automated gate.
//
// Proves TWO things without needing a browser:
//   1. The wasm decompiler RUNS client-side (under Node's WASI preview1, the
//      same ABI the browser WASI shim implements), reading the specs + binary
//      from a virtual filesystem.
//   2. Its output is byte-identical to the NATIVE `kuna_wasm` build — i.e. the
//      wasm port is a faithful decompiler, not a degraded one.
//
// It runs `list` + several `decompile` cases + a whole-binary `project` export
// over each committed fixture (x86-64 ELF + AArch64 object) and diffs native vs
// wasm (normalizing only the absolute `binary` path, which legitimately differs
// between the host FS and the guest's virtual FS; `project` gets the same
// explicit display name on both sides, so its artifacts match as-is). Specs
// come from the full repo tree, so the loader auto-resolves each fixture's
// architecture.
//
// Usage:  node parity.mjs           (paths auto-resolved from the repo layout)
//         SPECS=... node parity.mjs  (override the spec root)
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
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
    cases: [['list'], ['decompile'], ['decompile', 'main'], ['decompile', 'sum_to'], ['decompile', 'add'],
      ['project', 'sample.elf', '--mode', 'fast']],
  },
  {
    fixture: join(here, 'fixtures/sample_aarch64.o'),
    arch: 'aarch64',
    cases: [['list'], ['decompile'], ['decompile', 'sum_to'], ['decompile', 'add'],
      ['project', 'sample_aarch64.o']],
  },
  {
    fixture: join(here, 'fixtures/sample_macho.o'),
    arch: 'macho-x86-64',
    cases: [['list'], ['decompile'], ['decompile', '_add'], ['project', 'sample_macho.o']],
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
    const want = c[0] === 'project' ? '"files"' : '"functions"'; // project emits a files map
    if (!n.includes(want)) fail(`\`${label}\` produced no ${want} payload`);
    console.log(`\x1b[32mOK\x1b[0m   ${label}  (${w.length} bytes, native==wasm)`);
    passed++;
  }
}

const boundaryDir = mkdtempSync(join(tmpdir(), 'kuna-wasm-auto-mode-'));
const source = readFileSync(FIXTURES[0].fixture);
const boundaries = [
  [500 * 1024 - 1, 'aggressive'],
  [500 * 1024, 'reliable'],
  [2 * 1024 * 1024 - 1, 'reliable'],
  [2 * 1024 * 1024, 'fast'],
];
try {
  for (const [size, expectedMode] of boundaries) {
    total++;
    const fixture = join(boundaryDir, `sample-${size}.elf`);
    const padded = Buffer.alloc(size);
    source.copy(padded);
    writeFileSync(fixture, padded);

    const autoArgs = ['list', '--mode', 'auto'];
    const explicitArgs = ['list', '--mode', expectedMode];
    const natAuto = normalize(runNative(fixture, autoArgs));
    const natExplicit = normalize(runNative(fixture, explicitArgs));
    if (natAuto !== natExplicit) {
      fail(`auto mode at ${size} bytes did not match explicit ${expectedMode}`);
    }
    const wsmAuto = normalize(runWasm(fixture, autoArgs));
    if (wsmAuto !== natAuto) {
      fail(`native != wasm for auto mode at ${size} bytes`);
    }
    console.log(
      `\x1b[32mOK\x1b[0m   auto mode @ ${size} bytes -> ${expectedMode} (native==wasm)`,
    );
    passed++;
  }
} finally {
  rmSync(boundaryDir, { recursive: true, force: true });
}

console.log(`\n\x1b[32mPARITY OK\x1b[0m — ${passed}/${total} cases across ${FIXTURES.length} arches: wasm runs under WASI and matches native byte-for-byte.`);
