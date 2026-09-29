// worker.mjs — exercise the shipped module Worker and its RPC client in Node.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { createServer } from 'node:http';
import { dirname, extname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { Worker as NodeWorker } from 'node:worker_threads';
import {
  KunaWorkerCancelledError,
  KunaWorkerClient,
} from '../kuna-worker-client.js';

const here = dirname(fileURLToPath(import.meta.url));
const dist = resolve(here, '../dist');
const fixture = join(here, 'fixtures', 'sample.elf');

if (!existsSync(join(dist, 'kuna_wasm.wasm')) ||
    !existsSync(join(dist, 'kuna-worker.js'))) {
  throw new Error('dist/ not built — run integrations/web/build.sh first');
}

const MIME = {
  '.wasm': 'application/wasm',
  '.js': 'text/javascript',
  '.json': 'application/json',
  '.sla': 'application/octet-stream',
  '.ldefs': 'text/xml',
  '.pspec': 'text/xml',
  '.cspec': 'text/xml',
  '.dwarf': 'application/octet-stream',
};

let wasmRequests = 0;
let specRequests = 0;
let etag = null;
let wasmOverride = null;
const server = createServer(async (req, res) => {
  try {
    const rel = decodeURIComponent(new URL(req.url, 'http://worker.test').pathname);
    if (rel.endsWith('.wasm')) wasmRequests++;
    if (rel.includes('/specs')) specRequests++;
    const file = resolve(dist, '.' + rel);
    if (!file.startsWith(dist)) {
      res.writeHead(403).end();
      return;
    }
    const body = rel.endsWith('.wasm') && wasmOverride ? wasmOverride : await readFile(file);
    res.writeHead(200, {
      'content-type': MIME[extname(file)] || 'application/octet-stream',
      ...(etag ? { etag } : {}),
    });
    res.end(body);
  } catch {
    res.writeHead(404).end();
  }
});

await new Promise((resolveListen) => server.listen(0, resolveListen));
const base = `http://localhost:${server.address().port}`;

const bridge = new URL(
  'data:text/javascript,' + encodeURIComponent(`
    import { parentPort, workerData } from 'node:worker_threads';
    globalThis.self = globalThis;
    globalThis.postMessage = (data, transfer) => parentPort.postMessage(data, transfer);
    await import(workerData.moduleUrl);
    parentPort.on('message', (data) => globalThis.onmessage({ data }));
  `),
);

let workersCreated = 0;
let workersTerminated = 0;
const sessionModes = [];

class BrowserWorker {
  constructor(moduleUrl) {
    workersCreated++;
    this.node = new NodeWorker(bridge, {
      type: 'module',
      workerData: { moduleUrl },
    });
    this.node.on('message', (data) => this.onmessage?.({ data }));
    this.node.on('error', (error) => this.onerror?.({
      message: error.message,
      error,
    }));
  }

  postMessage(data, transfer) {
    if (data?.method === 'setBinary') sessionModes.push(data.params.mode);
    this.node.postMessage(data, transfer);
  }

  terminate() {
    workersTerminated++;
    void this.node.terminate();
  }
}

function zipNames(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const eocd = bytes.length - 22;
  assert.equal(view.getUint32(eocd, true), 0x06054b50, 'project result is a ZIP');
  const count = view.getUint16(eocd + 10, true);
  let offset = view.getUint32(eocd + 16, true);
  const names = [];
  for (let i = 0; i < count; i++) {
    assert.equal(view.getUint32(offset, true), 0x02014b50, 'central-directory entry');
    const nameLength = view.getUint16(offset + 28, true);
    const extraLength = view.getUint16(offset + 30, true);
    const commentLength = view.getUint16(offset + 32, true);
    names.push(new TextDecoder().decode(bytes.subarray(offset + 46, offset + 46 + nameLength)));
    offset += 46 + nameLength + extraLength + commentLength;
  }
  return names.sort();
}

const client = new KunaWorkerClient({
  workerUrl: pathToFileURL(join(dist, 'kuna-worker.js')),
  wasmUrl: `${base}/kuna_wasm.wasm`,
  specRoot: `${base}/specs`,
  workerFactory: (url) => new BrowserWorker(url),
});

try {
  await client.ready();
  const binary = new Uint8Array(await readFile(fixture));
  const inventoryStart = performance.now();
  const inventory = await client.load(binary, {
    fileName: 'sample.elf',
    mode: 'auto',
  });
  const inventoryMs = Math.round(performance.now() - inventoryStart);
  assert.equal(inventory.format, 'ELF');
  assert.equal(sessionModes[0], 'auto', 'the Worker preserves the automatic mode policy');
  const main = inventory.functions.find((fn) => fn.name === 'main');
  assert.ok(main, 'inventory contains main');
  assert.equal('code' in main, false, 'inventory does not eagerly decompile bodies');

  const bodyStart = performance.now();
  const first = await client.decompile(main.address_hex);
  const bodyMs = Math.round(performance.now() - bodyStart);
  assert.equal(first.functions.length, 1, 'address click decompiles one function');
  assert.match(first.functions[0].code, /sum_to\(add\(/);

  const cancelled = client.decompile(main.address_hex);
  await new Promise((resolveImmediate) => setImmediate(resolveImmediate));
  client.cancel('test hard cancellation');
  await assert.rejects(
    cancelled,
    (error) => error instanceof KunaWorkerCancelledError,
    'cancelling terminates the active Worker request',
  );
  await client.ready();
  const afterRestart = await client.decompile(main.address_hex);
  assert.match(afterRestart.functions[0].code, /sum_to\(add\(/);
  assert.ok(workersCreated >= 2, 'cancellation recreates the Worker');
  assert.ok(workersTerminated >= 1, 'cancellation terminates the blocked Worker');

  let heartbeat = 0;
  const timer = setInterval(() => heartbeat++, 10);
  let project;
  try {
    project = await client.project('sample.elf');
  } finally {
    clearInterval(timer);
  }
  assert.ok(heartbeat > 0, 'the page event loop advances during project export');
  assert.deepEqual(
    zipNames(project.bytes),
    [
      'sample.elf.kuna/README.md',
      'sample.elf.kuna/sample.elf.asm',
      'sample.elf.kuna/sample.elf.c',
      'sample.elf.kuna/sample.elf.h',
    ],
  );
  assert.deepEqual(
    Object.keys(project).sort(),
    ['bytes', 'count', 'downloadName', 'failed', 'ok'],
    'the Worker returns only ZIP bytes and small metadata',
  );
  assert.ok(
    sessionModes.every((mode) => mode === 'auto'),
    'session rehydration preserves automatic mode selection',
  );

  // The session's output language reaches every request: loading with `rust`
  // renders the same function as Rust, where the Worker once dropped it.
  const rustInventory = await client.load(binary, {
    fileName: 'sample.elf',
    mode: 'auto',
    language: 'rust',
  });
  assert.ok(rustInventory.functions.some((fn) => fn.name === 'main'), 'rust session inventories main');
  const rust = await client.decompile('main');
  assert.match(rust.functions[0].code, /fn main|let mut|unsafe/, 'the language control changes the output');
  const backToC = await client.load(binary, { fileName: 'sample.elf', language: 'c' });
  assert.ok(backToC.functions.length > 0);
  assert.doesNotMatch((await client.decompile('main')).functions[0].code, /let mut/, 'C session renders C');

  const hashing = new KunaWorkerClient({
    workerUrl: pathToFileURL(join(dist, 'kuna-worker.js')),
    wasmUrl: `${base}/kuna_wasm.wasm`,
    specRoot: `${base}/specs`,
    workerFactory: (url) => new BrowserWorker(url),
  });
  try {
    await hashing.ready();
    assert.equal(hashing.build, null, 'nothing is hashed while the decompiler starts');
    const wasm = createHash('sha256').update(await readFile(join(dist, 'kuna_wasm.wasm'))).digest('hex');
    assert.equal(await hashing.buildId(), wasm, 'asked for, the build id is the SHA-256 of the wasm the Worker compiled');
    assert.equal(client.build, null, 'a page that does not ask for it pays nothing');
    const before = wasmRequests;
    hashing.cancel('a new Worker');
    await hashing.ready();
    assert.equal(await hashing.buildId(), wasm, 'and a respawned Worker has the same one');
    assert.equal(wasmRequests - before, 0, 'the new Worker runs the engine the page compiled, so it downloads nothing');
    const racing = new KunaWorkerClient({
      workerUrl: pathToFileURL(join(dist, 'kuna-worker.js')),
      wasmUrl: `${base}/kuna_wasm.wasm`,
      specRoot: `${base}/specs`,
      workerFactory: (url) => new BrowserWorker(url),
    });
    try {
      await racing.ready();
      const asked = racing.buildId();
      await new Promise((done) => setTimeout(done, 5));
      racing.cancel('a new Worker');
      assert.equal(await asked, wasm, 'a cancel while the build id was being worked out asks the new Worker');
    } finally {
      racing.close();
    }
  } finally {
    hashing.close();
  }

  // The site is deployed again while the page is open: the same wasm under a
  // new validator, then another wasm. A restarted Worker keeps running the
  // engine and the spec files this page loaded, so the build id stays.
  etag = '"one"';
  const pinned = new KunaWorkerClient({
    workerUrl: pathToFileURL(join(dist, 'kuna-worker.js')),
    wasmUrl: `${base}/kuna_wasm.wasm`,
    specRoot: `${base}/specs`,
    workerFactory: (url) => new BrowserWorker(url),
  });
  const heard = [];
  pinned.onbuild = (build, was) => heard.push([build, was]);
  try {
    await pinned.load(binary, { fileName: 'sample.elf' });
    await pinned.decompile(main.address_hex);
    const wasm = createHash('sha256').update(await readFile(join(dist, 'kuna_wasm.wasm'))).digest('hex');
    assert.equal(await pinned.buildId(), wasm);
    const [wasmBefore, specsBefore] = [wasmRequests, specRequests];
    etag = '"two"';
    pinned.cancel('a new Worker');
    await pinned.ready();
    assert.equal(wasmRequests - wasmBefore, 0, 'the same wasm deployed again: a restarted Worker downloads nothing');
    assert.deepEqual(heard, [], 'and the page does not hear of a new build');
    assert.equal(await pinned.buildId(), wasm);
    wasmOverride = Buffer.concat([await readFile(join(dist, 'kuna_wasm.wasm')), Buffer.from([0, 5, 4, 0x6e, 0x65, 0x77, 0x21])]);
    etag = '"three"';
    pinned.cancel('another Worker');
    assert.match((await pinned.decompile(main.address_hex)).functions[0].code, /sum_to\(add\(/, 'the restarted Worker decompiles');
    assert.equal(wasmRequests - wasmBefore, 0, 'another wasm deployed: still nothing downloaded');
    assert.equal(specRequests - specsBefore, 0, 'nor any spec file');
    assert.deepEqual(heard, []);
    assert.equal(await pinned.buildId(), wasm, 'the build id is the engine this page runs');
  } finally {
    pinned.close();
    etag = null;
    wasmOverride = null;
  }

  // A browser that cannot hand a compiled module to the page: a restarted
  // Worker compiles the server's wasm again, so the page hears that its
  // build id is no longer known.
  class NoModuleWorker extends BrowserWorker {
    constructor(url) {
      super(url);
      this.node.removeAllListeners('message');
      this.node.on('message', (data) => this.onmessage?.({ data: data?.result?.engine ? { ...data, result: { ...data.result, engine: null } } : data }));
    }
  }
  const unpinned = new KunaWorkerClient({
    workerUrl: pathToFileURL(join(dist, 'kuna-worker.js')),
    wasmUrl: `${base}/kuna_wasm.wasm`,
    specRoot: `${base}/specs`,
    workerFactory: (url) => new NoModuleWorker(url),
  });
  const lost = [];
  unpinned.onbuild = (build, was) => lost.push([build, was]);
  try {
    const wasm = createHash('sha256').update(await readFile(join(dist, 'kuna_wasm.wasm'))).digest('hex');
    assert.equal(await unpinned.buildId(), wasm);
    unpinned.cancel('a new Worker');
    await unpinned.ready();
    assert.deepEqual(lost, [[null, wasm]], 'the page hears the id is no longer known');
    assert.equal(await unpinned.buildId(), wasm, 'and asks for it again');
  } finally {
    unpinned.close();
  }

  console.log(
    `WORKER OK — ${inventory.functions.length} functions inventoried, ` +
    `one address decompiled lazily (${inventoryMs} ms inventory + ${bodyMs} ms body), ` +
    'cancellation restarted the Worker, the host event loop stayed live, ' +
    `${project.bytes.length} ZIP bytes transferred, the session language reached the engine, ` +
    'the build id is the hash of the compiled wasm, worked out only when asked, ' +
    'and a restarted Worker runs the engine and spec files the page loaded',
  );
} finally {
  client.close();
  server.close();
}
