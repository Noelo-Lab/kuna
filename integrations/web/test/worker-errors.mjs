// worker-errors.mjs — pin fatal Worker lifecycle behavior without a wasm build.
import assert from 'node:assert/strict';
import { KunaWorkerClient } from '../kuna-worker-client.js';

const options = (workerFactory) => ({
  workerUrl: 'https://worker.test/kuna-worker.js',
  wasmUrl: 'https://worker.test/kuna_wasm.wasm',
  specRoot: 'https://worker.test/specs',
  workerFactory,
});
const blockedPattern = /content blocker or browser privacy setting.*allow this site and reload/i;

assert.throws(
  () => new KunaWorkerClient(options(() => { throw new Error('blocked by client'); })),
  blockedPattern,
  'a synchronous Worker constructor failure gives actionable guidance',
);

class ControlledWorker {
  postMessage(message) {
    this.messages ??= [];
    this.messages.push(message);
  }

  reply(method, result = true) {
    const message = this.messages.find((candidate) => candidate.method === method);
    this.onmessage?.({ data: { id: message.id, ok: true, result } });
  }

  terminate() {
    this.terminations = (this.terminations || 0) + 1;
  }
}

let startupWorker;
const blocked = new KunaWorkerClient(options(() => {
  startupWorker = new ControlledWorker();
  return startupWorker;
}));
const extraStartupRequest = blocked.request('list');
startupWorker.onerror({ message: '' });
const [readyError, extraError] = await Promise.all([
  blocked.ready().catch((error) => error),
  extraStartupRequest.catch((error) => error),
]);
assert.match(readyError.message, blockedPattern, 'a blocked startup gives actionable guidance');
assert.strictEqual(extraError, readyError, 'all pending startup requests receive one fatal error');
assert.strictEqual(
  await blocked.request('list').catch((error) => error),
  readyError,
  'requests after a fatal Worker error receive the same error instead of hanging',
);
startupWorker.onmessageerror({});
startupWorker.onerror({ message: 'later error' });
assert.equal(startupWorker.terminations, 1, 'duplicate fatal events do not terminate twice');
assert.strictEqual(
  await blocked.request('list').catch((error) => error),
  readyError,
  'a later fatal event cannot overwrite the first error',
);
blocked.close();

let decodeWorker;
const unreadable = new KunaWorkerClient(options(() => {
  decodeWorker = new ControlledWorker();
  return decodeWorker;
}));
decodeWorker.onmessageerror({});
await assert.rejects(
  unreadable.ready(),
  /unreadable response/,
  'a message decoding failure is terminal',
);
unreadable.close();

let runtimeWorker;
const runtime = new KunaWorkerClient(options(() => {
  runtimeWorker = new ControlledWorker();
  return runtimeWorker;
}));
runtimeWorker.reply('init');
await runtime.ready();
const runtimeRequests = [runtime.request('list'), runtime.request('project')];
runtimeWorker.onerror({ message: 'runtime exploded' });
const runtimeErrors = await Promise.all(runtimeRequests.map((promise) => promise.catch((error) => error)));
assert.equal(runtimeErrors[0].message, 'runtime exploded', 'post-init errors remain runtime errors');
assert.strictEqual(runtimeErrors[1], runtimeErrors[0], 'all pending runtime requests receive one error');
runtimeWorker.onmessageerror({});
assert.strictEqual(
  await runtime.request('list').catch((error) => error),
  runtimeErrors[0],
  'a duplicate runtime event cannot overwrite the terminal error',
);
assert.equal(runtimeWorker.terminations, 1, 'a runtime failure terminates once');
runtime.close();

console.log('WORKER ERROR OK — constructor, startup, decode, runtime, and duplicate failures are terminal');
