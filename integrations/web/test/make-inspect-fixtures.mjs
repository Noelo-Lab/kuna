// make-inspect-fixtures.mjs — regenerate the study view's fixtures
// (test/fixtures/inspect-{main,sum_to,add}.json, list-sample.json) from the
// NATIVE kuna_wasm build on test/fixtures/sample.elf, so the page's pure
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
const elf = join(here, 'fixtures/sample.elf');

const run = (...args) => {
  const doc = JSON.parse(execFileSync(native, [elf, specs, ...args], { maxBuffer: 64 << 20 }).toString());
  doc.binary = 'sample.elf';
  return doc;
};
const write = (file, doc) => writeFileSync(join(here, 'fixtures', file), JSON.stringify(doc, null, 1) + '\n');

for (const name of ['main', 'sum_to', 'add']) write(`inspect-${name}.json`, run('inspect', name));
write('list-sample.json', run('list'));
console.log('wrote inspect-{main,sum_to,add}.json and list-sample.json from', native);
