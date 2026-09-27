// engine.mjs — what each peer's own copy of the engine guarantees, and where
// it does not. It runs the shipped wasm under node:wasi, one process per call:
//  1. one document for one (file, directives, mode), whatever the directives'
//     order, so peers only need to agree on the set;
//  2. the mode changes what a symbol means, so the mode must be one shared
//     setting (the same `v20` is a different variable in Fast);
//  3. the output language does not change any symbol, so C or Rust can stay
//     each person's choice;
//  4. two directive forms read files (`@FILE`, `bytes ADDR @FILE`), so a
//     directive from another peer must never carry them.
//   integrations/web/build.sh && node docs/features/decompile2-collab/spike/engine.mjs
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { statSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '../../../..');
const web = join(root, 'integrations/web');
const WASM = join(web, 'dist/kuna_wasm.wasm');
const SPECS = join(web, 'dist/specs');
const SAMPLE = join(web, 'test/fixtures/sample.elf');
const I386 = join(root, 'decompiler/crates/kuna-analysis/tests/fixtures/i386_pie_nl');

function wasm(bin, cmd, target, { mode = 'auto', language = 'auto', directives = [] } = {}) {
  const args = [cmd, ...(target ? [target] : []), '--mode', mode, '--language', language, ...directives.flatMap((d) => ['--assert', d])];
  const res = spawnSync('node', ['--experimental-wasi-unstable-preview1', join(web, 'test/run-wasm.mjs'), WASM, SPECS, bin, ...args],
    { encoding: 'utf8', maxBuffer: 64 << 20 });
  const out = res.stdout.trim();
  const err = res.stderr.split('\n').filter((l) => l.startsWith('error:')).join('\n');
  return out.startsWith('{') ? JSON.parse(out) : { error: err || out, status: res.status };
}
const body = (doc) => {
  const copy = { ...doc };
  delete copy.assertions;
  return createHash('sha256').update(JSON.stringify(copy)).digest('hex').slice(0, 16);
};
const outcomes = (doc) => (doc.assertions || []).map((r) => JSON.stringify(r)).sort().join('\n');
const declOf = (doc, name) => (doc.function.code.split('\n').find((l) => new RegExp(`\\b${name};`).test(l)) || '').trim();
const lines = [];

// 1. deterministic and order-blind
const SET = ['function 0x1149=adder', 'prototype 0x1161 long sum_to(int count)', 'bytes 0x11af 05',
  'type v1 unsigned long total', 'comment 0x11b5 calls adder first'];
const orders = [SET, [...SET].reverse(), [SET[3], SET[0], SET[4], SET[2], SET[1]]];
const first = wasm(SAMPLE, 'inspect', 'main', { directives: orders[0] });
assert.equal(body(wasm(SAMPLE, 'inspect', 'main', { directives: orders[0] })), body(first), 'the same request in another process prints the same document');
for (const order of orders.slice(1)) {
  const doc = wasm(SAMPLE, 'inspect', 'main', { directives: order });
  assert.equal(body(doc), body(first), `directive order does not change the output (${order.join(' | ')})`);
  assert.equal(outcomes(doc), outcomes(first), 'and each directive gets the same outcome');
}
assert.notEqual(body(wasm(SAMPLE, 'inspect', 'main')), body(first), 'the directives do change the output');
lines.push(`order: 1 document (sha256 ${body(first)}) for 5 directives in 3 orders across 4 processes`);

// 2. the mode changes what a symbol means
const rename = ['name v20 counter'];
const inAuto = wasm(I386, 'inspect', '0x1190', { mode: 'auto', directives: rename });
const inFast = wasm(I386, 'inspect', '0x1190', { mode: 'fast', directives: rename });
assert.equal(inAuto.assertions[0].status, 'applied');
assert.equal(inFast.assertions[0].status, 'applied', 'the same rename applies in both modes');
const autoDecl = declOf(inAuto, 'counter');
const fastDecl = declOf(inFast, 'counter');
assert.notEqual(autoDecl.split('//')[1], fastDecl.split('//')[1], 'but it names a different variable in each');
const autoCount = wasm(I386, 'list', null, { mode: 'auto' }).functions.length;
const fastCount = wasm(I386, 'list', null, { mode: 'fast' }).functions.length;
assert.notEqual(autoCount, fastCount, 'and the modes do not even find the same functions');
lines.push(`modes: "name v20 counter" in i386_pie_nl main applies in both, but declares \`${autoDecl}\` in Automatic and \`${fastDecl}\` in Fast; list finds ${autoCount} vs ${fastCount} functions`);

// 3. the output language does not change a symbol
const vars = (doc) => JSON.stringify(doc.function.variables.map((v) => [v.name, v.kind, v.stack_offset, v.size, v.arg_index]));
let same = 0;
for (const fn of ['main', 'sum_to', 'add']) {
  const c = wasm(SAMPLE, 'inspect', fn, { language: 'c', directives: ['name v1 total'] });
  const rust = wasm(SAMPLE, 'inspect', fn, { language: 'rust', directives: ['name v1 total'] });
  assert.equal(vars(rust), vars(c), `${fn}: the same variables in C and in Rust`);
  assert.equal(outcomes(rust), outcomes(c), `${fn}: the same outcome for a rename`);
  assert.notEqual(rust.function.code, c.function.code, `${fn}: the code itself differs`);
  same++;
}
lines.push(`language: ${same}/3 functions list the same variables and give a rename the same outcome in C and in Rust`);

// 4. two directive forms read files
const size = statSync(join(web, 'test/fixtures/sample.c')).size;
const whole = wasm(SAMPLE, 'inspect', 'main', { directives: ['@/work/sample.c'] });
assert.match(whole.error || '', /^error: \/work\/sample\.c:2: /, 'a directive that is @FILE makes the engine open that file and parse its lines as directives');
assert.equal(whole.status, 1);
const patch = wasm(SAMPLE, 'inspect', 'main', { directives: ['bytes 0x11af @/work/sample.c'] });
const span = /maps (0x[0-9a-f]+)-(0x[0-9a-f]+)/.exec(patch.assertions[0].detail);
assert.equal(Number(BigInt(span[2]) - BigInt(span[1])), size, 'bytes ADDR @FILE reads the whole file into a patch');
lines.push(`@FILE: "@/work/sample.c" makes the engine parse that file (${whole.error.split('\n')[0].slice(0, 58)}…); "bytes 0x11af @/work/sample.c" reads all ${size} bytes of it into a patch`);

console.log('ENGINE OK\n  ' + lines.join('\n  '));
