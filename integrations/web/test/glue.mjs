// glue.mjs — exercises the REAL browser code path in Node, with no browser and
// no external dependency.
//
// It imports the shipped `kuna-web.js` (which drives the vendored
// @bjorn3/browser_wasi_shim — the SAME WASI implementation the browser uses) and
// runs `loadKuna().decompile()` (and the whole-binary `.project()` export)
// against a `dist/` served over HTTP, exactly as a page would. This is where
// the robust spec mechanism is proven end to end:
// kuna-web.js preloads only the small `specs-small.json` bundle, then LAZILY
// fetches each binary's `.sla` on demand — across formats (ELF + Mach-O) and
// architectures (x86-64 + AArch64). Only the DOM chrome in index.html is not
// exercised here (see the optional Puppeteer test in the README).
//
// Usage:  integrations/web/build.sh && node integrations/web/test/glue.mjs
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve, extname } from 'node:path';
import { loadKuna } from '../kuna-web.js';

const here = dirname(fileURLToPath(import.meta.url));
const dist = resolve(here, '../dist');
const fx = (n) => join(here, 'fixtures', n);

function fail(m) { console.error(`\x1b[31mFAIL\x1b[0m ${m}`); process.exit(1); }
if (!existsSync(join(dist, 'kuna_wasm.wasm')) || !existsSync(join(dist, 'specs-small.json')))
  fail(`dist/ not built — run integrations/web/build.sh first`);

const MIME = {
  '.wasm': 'application/wasm', '.js': 'text/javascript', '.html': 'text/html',
  '.json': 'application/json', '.sla': 'application/octet-stream', '.ldefs': 'text/xml',
  '.pspec': 'text/xml', '.cspec': 'text/xml', '.dwarf': 'application/octet-stream',
};

// Minimal static server rooted at dist/ (path-traversal safe), counting .sla fetches.
let slaFetches = 0;
const server = createServer(async (req, res) => {
  try {
    const rel = decodeURIComponent(new URL(req.url, 'http://x').pathname);
    const file = resolve(dist, '.' + rel);
    if (!file.startsWith(dist)) { res.writeHead(403).end(); return; }
    if (file.endsWith('.sla')) slaFetches++;
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': MIME[extname(file)] || 'application/octet-stream' });
    res.end(body);
  } catch { res.writeHead(404).end(); }
});

await new Promise((r) => server.listen(0, r));
const base = `http://localhost:${server.address().port}`;

try {
  const kuna = await loadKuna({ wasmUrl: `${base}/kuna_wasm.wasm`, specRoot: `${base}/specs` });

  // 1. ELF x86-64 — list + decompile one + decompile all (lazily fetches x86-64.sla).
  const elf = new Uint8Array(await readFile(fx('sample.elf')));
  if (kuna.formatName(elf) !== 'ELF') fail(`formatName(ELF) = ${kuna.formatName(elf)}`);
  const listed = await kuna.list(elf);
  if (!listed.functions.some((f) => f.name === 'sum_to')) fail(`x86-64 list missing 'sum_to'`);
  const main = await kuna.decompile(elf, 'main');
  if (!(main.functions[0]?.code || '').includes('sum_to(add(')) fail(`x86-64 decompile main body wrong`);
  const all = await kuna.decompile(elf);
  const bodied = all.functions.filter((f) => f.code).length;
  if (bodied < 3) fail(`x86-64 decompile-all only ${bodied} bodies`);
  console.log(`\x1b[32mOK\x1b[0m   ELF x86-64  (${listed.count} funcs, ${bodied} bodied; lazy-fetched x86-64.sla)`);

  // 2. ELF AArch64 — a different arch through the same handle (lazy AARCH64.sla).
  const a64 = new Uint8Array(await readFile(fx('sample_aarch64.o')));
  const a64res = await kuna.decompile(a64, 'add');
  if (!(a64res.functions[0]?.code || '').includes('return a0 + a1')) fail(`aarch64 'add' body wrong`);
  console.log(`\x1b[32mOK\x1b[0m   ELF AArch64  (lazy-fetched AARCH64.sla — different arch, no code change)`);

  // 3. Mach-O — a different FORMAT through the same handle (engine resolves it;
  //    lazily fetches x86-64.sla). Object bodies are thin, so assert on enumeration.
  const macho = new Uint8Array(await readFile(fx('sample_macho.o')));
  if (kuna.formatName(macho) !== 'Mach-O') fail(`formatName(Mach-O) = ${kuna.formatName(macho)}`);
  const mres = await kuna.list(macho);
  if (!mres.functions.some((f) => f.name === '_compute')) fail(`Mach-O list missing '_compute'`);
  console.log(`\x1b[32mOK\x1b[0m   Mach-O x86-64  (${mres.count} funcs; engine resolved a non-ELF format)`);

  // 4. Whole-binary project export through the same handle (the zip's payload).
  const proj = await kuna.project(elf, 'sample.elf');
  const wantFiles = ['README.md', 'sample.elf.asm', 'sample.elf.c', 'sample.elf.h'];
  const gotFiles = Object.keys(proj.files || {}).sort();
  if (gotFiles.join(',') !== wantFiles.join(','))
    fail(`project files = [${gotFiles.join(', ')}], want [${wantFiles.join(', ')}]`);
  if (!proj.files['sample.elf.c'].includes('// Function:')) fail(`project .c missing '// Function:' headers`);
  if (!(proj.count > 0)) fail(`project count = ${proj.count}`);
  console.log(`\x1b[32mOK\x1b[0m   project export  (${proj.count} functions -> 4 artifacts)`);

  console.log(`\n\x1b[32mGLUE OK\x1b[0m — kuna-web.js decompiles ELF + Mach-O, x86-64 + AArch64, client-side over HTTP; ${slaFetches} .sla lazily fetched, no per-format JS.`);
} catch (e) {
  fail(e.stack || e.message);
} finally {
  server.close();
}
