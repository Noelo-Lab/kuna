// glue.mjs — exercises the REAL browser code path in Node, with no browser and
// no external dependency.
//
// It imports the shipped `kuna-web.js` (which drives the vendored
// @bjorn3/browser_wasi_shim — the SAME WASI implementation the browser uses) and
// runs `loadKuna().decompile()` against a `dist/` served over HTTP, exactly as a
// page would. Only the DOM chrome in index.html is not exercised here (see the
// optional Puppeteer test in the README for a full-UI check).
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
const fixture = join(here, 'fixtures/sample.elf');

function fail(m) { console.error(`\x1b[31mFAIL\x1b[0m ${m}`); process.exit(1); }
if (!existsSync(join(dist, 'kuna_wasm.wasm')))
  fail(`dist/ not built — run integrations/web/build.sh first`);

const MIME = {
  '.wasm': 'application/wasm', '.js': 'text/javascript', '.html': 'text/html',
  '.sla': 'application/octet-stream', '.ldefs': 'text/xml', '.pspec': 'text/xml',
  '.cspec': 'text/xml', '.dwarf': 'application/octet-stream',
};

// Minimal static server rooted at dist/ (path-traversal safe).
const server = createServer(async (req, res) => {
  try {
    const rel = decodeURIComponent(new URL(req.url, 'http://x').pathname);
    const file = resolve(dist, '.' + rel);
    if (!file.startsWith(dist)) { res.writeHead(403).end(); return; }
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': MIME[extname(file)] || 'application/octet-stream' });
    res.end(body);
  } catch {
    res.writeHead(404).end();
  }
});

await new Promise((r) => server.listen(0, r));
const port = server.address().port;
const base = `http://localhost:${port}`;

try {
  const kuna = await loadKuna({ wasmUrl: `${base}/kuna_wasm.wasm`, specRoot: `${base}/specs` });
  const bytes = new Uint8Array(await readFile(fixture));

  // list
  const listed = await kuna.list(bytes);
  if (!listed.functions.some((f) => f.name === 'sum_to'))
    fail(`list did not find 'sum_to' (got ${listed.count} functions)`);
  console.log(`\x1b[32mOK\x1b[0m   list  (${listed.count} functions via the browser shim)`);

  // decompile one
  const one = await kuna.decompile(bytes, 'main');
  const code = one.functions[0]?.code || '';
  if (!code.includes('sum_to(add(')) fail(`decompile main lacked expected body:\n${code}`);
  console.log(`\x1b[32mOK\x1b[0m   decompile main  (${code.split('\n').length} lines)`);

  // decompile all
  const all = await kuna.decompile(bytes);
  const ok = all.functions.filter((f) => f.code).length;
  if (ok < 3) fail(`decompile-all produced only ${ok} bodies`);
  console.log(`\x1b[32mOK\x1b[0m   decompile all  (${ok}/${all.count} functions bodied)`);

  console.log(`\n\x1b[32mGLUE OK\x1b[0m — kuna-web.js + browser WASI shim decompile client-side over HTTP.`);
} catch (e) {
  fail(e.stack || e.message);
} finally {
  server.close();
}
