// worker.mjs — exercise the shipped module Worker and its RPC client in Node.
import assert from 'node:assert/strict';
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

const server = createServer(async (req, res) => {
  try {
    const rel = decodeURIComponent(new URL(req.url, 'http://worker.test').pathname);
    const file = resolve(dist, '.' + rel);
    if (!file.startsWith(dist)) {
      res.writeHead(403).end();
      return;
    }
    const body = await readFile(file);
    res.writeHead(200, {
      'content-type': MIME[extname(file)] || 'application/octet-stream',
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

  console.log(
    `WORKER OK — ${inventory.functions.length} functions inventoried, ` +
    `one address decompiled lazily (${inventoryMs} ms inventory + ${bodyMs} ms body), ` +
    'cancellation restarted the Worker, the host event loop stayed live, ' +
    `${project.bytes.length} ZIP bytes transferred`,
  );
} finally {
  client.close();
  server.close();
}
