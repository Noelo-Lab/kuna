// worker-harness.mjs — serve dist/ over HTTP and host the shipped module Worker
// in a Node worker thread, the way test/worker.mjs does, for the tests that
// drive the RPC client end to end.
import { readFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { createServer } from 'node:http';
import { dirname, extname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { Worker as NodeWorker } from 'node:worker_threads';
import { KunaWorkerClient } from '../kuna-worker-client.js';

const here = dirname(fileURLToPath(import.meta.url));
export const dist = resolve(here, '../dist');
export const fixture = (name) => join(here, 'fixtures', name);

const MIME = {
  '.wasm': 'application/wasm',
  '.js': 'text/javascript',
  '.mjs': 'text/javascript',
  '.html': 'text/html',
  '.css': 'text/css',
  '.json': 'application/json',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.woff2': 'font/woff2',
  '.c': 'text/plain',
  '.sla': 'application/octet-stream',
  '.ldefs': 'text/xml',
  '.pspec': 'text/xml',
  '.cspec': 'text/xml',
  '.dwarf': 'application/octet-stream',
};

export function requireDist() {
  if (!existsSync(join(dist, 'kuna_wasm.wasm')) || !existsSync(join(dist, 'kuna-worker.js'))) {
    throw new Error('dist/ not built — run integrations/web/build.sh first');
  }
}

/** A path-traversal-safe static server over `root`; resolves `{base, close}`. */
export async function serveStatic(root = dist, port = 0) {
  const server = createServer(async (req, res) => {
    try {
      let rel = decodeURIComponent(new URL(req.url, 'http://harness.test').pathname);
      if (rel.endsWith('/')) rel += 'index.html';
      const file = resolve(root, '.' + rel);
      if (!file.startsWith(root)) {
        res.writeHead(403).end();
        return;
      }
      const body = await readFile(file);
      res.writeHead(200, { 'content-type': MIME[extname(file)] || 'application/octet-stream' });
      res.end(body);
    } catch {
      res.writeHead(404).end();
    }
  });
  await new Promise((done) => server.listen(port, '127.0.0.1', done));
  return {
    base: `http://127.0.0.1:${server.address().port}`,
    close: () => new Promise((done) => server.close(done)),
  };
}

const bridge = new URL(
  'data:text/javascript,' + encodeURIComponent(`
    import { parentPort, workerData } from 'node:worker_threads';
    globalThis.self = globalThis;
    globalThis.postMessage = (data, transfer) => parentPort.postMessage(data, transfer);
    await import(workerData.moduleUrl);
    parentPort.on('message', (data) => globalThis.onmessage({ data }));
  `),
);

/** The browser `Worker` surface over a Node worker thread. */
export class BrowserWorker {
  constructor(moduleUrl) {
    this.node = new NodeWorker(bridge, { type: 'module', workerData: { moduleUrl } });
    this.node.on('message', (data) => this.onmessage?.({ data }));
    this.node.on('error', (error) => this.onerror?.({ message: error.message, error }));
  }

  postMessage(data, transfer) {
    this.node.postMessage(data, transfer);
  }

  terminate() {
    void this.node.terminate();
  }
}

/** A client on the built Worker, served from `base`. */
export function workerClient(base) {
  return new KunaWorkerClient({
    workerUrl: pathToFileURL(join(dist, 'kuna-worker.js')),
    wasmUrl: `${base}/kuna_wasm.wasm`,
    specRoot: `${base}/specs`,
    workerFactory: (url) => new BrowserWorker(url),
  });
}
