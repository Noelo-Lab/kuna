// kuna-worker-client.js — request/response facade for the decompiler Worker.

export class KunaWorkerCancelledError extends Error {
  constructor(message = 'decompiler operation cancelled') {
    super(message);
    this.name = 'KunaWorkerCancelledError';
  }
}

function asUrl(value, base) {
  return new URL(value, base).href;
}

export class KunaWorkerClient {
  constructor({
    workerUrl = new URL('./kuna-worker.js', import.meta.url),
    wasmUrl,
    specRoot,
    smallBundleUrl,
    baseUrl = globalThis.document?.baseURI || import.meta.url,
    workerFactory,
  }) {
    if (!wasmUrl || !specRoot) throw new Error('wasmUrl and specRoot are required');
    this.workerUrl = asUrl(workerUrl, baseUrl);
    this.initParams = {
      wasmUrl: asUrl(wasmUrl, baseUrl),
      specRoot: asUrl(specRoot, baseUrl).replace(/\/$/, ''),
    };
    if (smallBundleUrl) this.initParams.smallBundleUrl = asUrl(smallBundleUrl, baseUrl);
    this.workerFactory = workerFactory || ((url, options) => new Worker(url, options));
    this.pending = new Map();
    this.nextId = 1;
    this.generation = 0;
    this.session = null;
    this.workerSession = null;
    this.closed = false;
    this.spawn();
  }

  spawn() {
    const generation = ++this.generation;
    const worker = this.workerFactory(this.workerUrl, { type: 'module' });
    this.worker = worker;
    this.workerSession = null;
    worker.onmessage = ({ data }) => {
      if (generation !== this.generation) return;
      const pending = this.pending.get(data?.id);
      if (!pending) return;
      this.pending.delete(data.id);
      if (data.ok) pending.resolve(data.result);
      else pending.reject(new Error(data.error || 'decompiler worker request failed'));
    };
    worker.onerror = (event) => {
      if (generation !== this.generation) return;
      const message = event?.message || 'decompiler worker failed';
      this.rejectPending(new Error(message));
    };
    this.readyPromise = this.request('init', this.initParams);
    this.readyPromise.catch(() => {});
  }

  request(method, params = {}, transfer = []) {
    if (this.closed) return Promise.reject(new Error('decompiler worker is closed'));
    const id = this.nextId++;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      try {
        this.worker.postMessage({ id, method, params }, transfer);
      } catch (error) {
        this.pending.delete(id);
        reject(error);
      }
    });
  }

  rejectPending(error) {
    for (const pending of this.pending.values()) pending.reject(error);
    this.pending.clear();
  }

  async ready() {
    return this.readyPromise;
  }

  checkGeneration(generation) {
    if (generation !== this.generation) {
      throw new KunaWorkerCancelledError();
    }
  }

  async setWorkerSession(generation = this.generation) {
    await this.readyPromise;
    this.checkGeneration(generation);
    if (!this.session) throw new Error('no binary is loaded');
    const session = this.session;
    if (this.workerSession === session) return;
    const details = await this.request('setBinary', {
      bytes: session.bytes,
      fileName: session.fileName,
      mode: session.mode,
      language: session.language,
    });
    this.checkGeneration(generation);
    if (this.session !== session) {
      throw new KunaWorkerCancelledError('binary session superseded');
    }
    session.format = details.format;
    this.workerSession = session;
  }

  async load(binaryBytes, { fileName = 'binary', mode = 'auto', language = 'auto' } = {}) {
    const generation = this.generation;
    const bytes = binaryBytes instanceof Uint8Array
      ? binaryBytes
      : new Uint8Array(binaryBytes);
    const session = { bytes, fileName, mode, language };
    this.session = session;
    this.workerSession = null;
    await this.setWorkerSession(generation);
    const inventory = await this.request('list');
    this.checkGeneration(generation);
    if (this.session !== session) {
      throw new KunaWorkerCancelledError('binary session superseded');
    }
    return {
      ...inventory,
      format: session.format,
    };
  }

  async format() {
    const generation = this.generation;
    await this.setWorkerSession(generation);
    return this.session.format;
  }

  async list() {
    const generation = this.generation;
    await this.setWorkerSession(generation);
    const result = await this.request('list');
    this.checkGeneration(generation);
    return result;
  }

  async decompile(target) {
    const generation = this.generation;
    await this.setWorkerSession(generation);
    const result = await this.request('decompile', { target });
    this.checkGeneration(generation);
    return result;
  }

  async project(displayName) {
    const generation = this.generation;
    await this.setWorkerSession(generation);
    const result = await this.request('project', { displayName });
    this.checkGeneration(generation);
    return { ...result, bytes: new Uint8Array(result.bytes) };
  }

  cancel(message = 'decompiler operation cancelled') {
    if (this.closed) return;
    const error = new KunaWorkerCancelledError(message);
    this.generation++;
    this.worker.terminate();
    this.rejectPending(error);
    this.spawn();
  }

  clear(message = 'decompiler session cleared') {
    this.session = null;
    this.cancel(message);
  }

  close() {
    if (this.closed) return;
    this.closed = true;
    this.generation++;
    this.worker.terminate();
    this.rejectPending(new KunaWorkerCancelledError('decompiler worker closed'));
    this.session = null;
  }
}
