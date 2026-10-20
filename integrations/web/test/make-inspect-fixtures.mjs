// make-inspect-fixtures.mjs — regenerate the study view's fixtures
// (test/fixtures/inspect-{main,sum_to,add}.json, list-sample.json from
// sample.elf, inspect-make_item.json from structs.elf, and the Data tab's
// data-globals*.json and inspect-globals-stripped.json from globals.elf and
// its stripped twin) from the
// NATIVE kuna_wasm build, so the page's pure
// renderers are tested against exactly what the engine emits, without a wasm
// build. The `binary` path is rewritten to the file name so the fixtures do
// not depend on where the repo lives.
//
// Usage:  (cd decompiler && cargo build --release -p kuna-wasm)
//         node integrations/web/test/make-inspect-fixtures.mjs
import { execFileSync } from 'node:child_process';
import { writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, '../../..');
const native = process.env.NATIVE || join(repo, 'decompiler/target/release/kuna_wasm');
const specs = process.env.SPECS || join(repo, 'specs');

const runOn = (name, ...args) => {
  const doc = JSON.parse(execFileSync(native, [join(here, 'fixtures', name), specs, ...args], { maxBuffer: 64 << 20 }).toString());
  doc.binary = name;
  return doc;
};
const run = (...args) => runOn('sample.elf', ...args);
const write = (file, doc) => writeFileSync(join(here, 'fixtures', file), JSON.stringify(doc, null, 1) + '\n');

for (const name of ['main', 'sum_to', 'add']) write(`inspect-${name}.json`, run('inspect', name));
write('list-sample.json', run('list'));
write('inspect-make_item.json', runOn('structs.elf', 'inspect', 'make_item'));
const typed = ['data 0x4040 int primes[8]', 'data 0x4060 point origin', 'data 0x4070 char *banner',
  'data 0x4078 int (*op)(int)', 'data 0x4080 int *favorite'].flatMap((d) => ['--assert', d]);
write('data-globals.json', runOn('globals.elf', 'data', '0x4040'));
write('data-globals-typed.json', runOn('globals.elf', 'data', '0x4060', ...typed));
write('data-globals-bss.json', runOn('globals.elf', 'data', '0x40c0'));
write('data-globals-stripped.json', runOn('globals-stripped.elf', 'data', '0x4040'));
write('inspect-globals-stripped.json', runOn('globals-stripped.elf', 'inspect', 'main'));
write('uses-globals.json', runOn('globals.elf', 'uses', '0x4000', '0x88'));
console.log('wrote inspect-{main,sum_to,add,make_item,globals-stripped}.json, list-sample.json, data-globals*.json and uses-globals.json from', native);
