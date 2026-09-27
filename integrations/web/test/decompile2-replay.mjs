// decompile2-replay.mjs — the study view's exported .kuna file, replayed by the
// native CLI (`kuna decompile <binary> <function> --assert @<file>`), in the
// cases where the order of directives decides the outcome: a struct defined
// before the struct and the global that use it; the later of two raw
// directives winning; a rename that frees a name before the rename that takes
// it. Each case replays the file a page on its own exports, and the file a page
// in a live session exports (its directives in the registers' birth order,
// built here from registers applied in the opposite order); no directive may
// be rejected. Skips (exit 0) without a built `kuna` (make binaries).
//   make binaries && node integrations/web/test/decompile2-replay.mjs
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Session } from '../decompile2/session.js';
import * as R from '../decompile2/collab/replica.js';

const root = join(dirname(fileURLToPath(import.meta.url)), '../../..');
const kuna = join(root, 'decompiler/target/release/kuna');
if (!existsSync(kuna)) {
  console.log('DECOMPILE2 REPLAY SKIPPED — no decompiler/target/release/kuna (make binaries)');
  process.exit(0);
}
const sample = join(root, 'integrations/web/test/fixtures/sample.elf');
const env = { ...process.env, SLEIGHHOME: process.env.SLEIGHHOME || join(root, 'specs') };
const dir = mkdtempSync(join(tmpdir(), 'kuna-replay-'));
const SUM = '0x1161';
const names = new Map([['0x1198', 'main'], [SUM, 'sum_to']]);

function replay(session, target) {
  const file = join(dir, 'sample.elf.kuna');
  writeFileSync(file, session.toFileText({ binary: 'sample.elf', target, nameOf: (a) => session.functionName(a) || names.get(a) || a }));
  const res = spawnSync(kuna, ['decompile', sample, target, '--assert', `@${file}`], { env, encoding: 'utf8' });
  assert.equal(res.status, 0, res.stderr);
  return { code: res.stdout, rejected: res.stderr.split('\n').filter((l) => /rejected/.test(l)) };
}

/** The same edits as a page in a live session holds them: registers applied in the opposite order, sent in birth order. */
function shared(edit) {
  const alone = new Session();
  edit(alone);
  R.adoptRawKeys(alone, 'ana00000');
  const r = new R.Replica('ana00000');
  for (const [k, v] of R.registersOf(alone)) r.set(k, v);
  const s = new Session();
  R.applyRegisters(s, r, [...r.regs.keys()].reverse());
  s.orderOf = R.birthOrder(s, r);
  return s;
}

const CASES = [
  ['a struct, then the struct and the global that use it', 'main', (s) => {
    s.setTypedef('point', 'struct point { int x; int y; } point;');
    s.setTypedef('line', 'struct line { struct point a; struct point b; } line;');
    s.setData('0x4010', 'line', 'gl');
  }, () => true],
  ['the later of two raw prototypes wins', 'sum_to', (s) => {
    for (let i = 1; i <= 10; i++) s.addRaw(`prototype ${SUM} long sum_to(int v${i})`);
  }, (code) => /sum_to\(int v10\)/.test(code)],
  ['a rename that frees a name, then the rename that takes it', 'sum_to', (s) => {
    s.setVar(SUM, 'v1', { name: 'i' });
    s.setVar(SUM, 'acc', { name: 'v1' });
  }, (code) => /\bint i;/.test(code)],
];

const done = [];
try {
  for (const [what, target, edit, expect] of CASES) {
    const alone = new Session();
    edit(alone);
    for (const [who, session] of [['alone', () => alone], ['in a live session', () => shared(edit)]]) {
      const { code, rejected } = replay(session(), target);
      assert.deepEqual(rejected, [], `${what} (${who}): nothing rejected`);
      assert.ok(expect(code), `${what} (${who}): ${code.split('\n')[0]}`);
    }
    done.push(what);
  }
} finally {
  rmSync(dir, { recursive: true, force: true });
}
console.log(`DECOMPILE2 REPLAY OK — the exported .kuna replays in the native CLI, alone and in a live session: ${done.join('; ')}`);
