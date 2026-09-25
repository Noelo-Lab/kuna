// kuna-worker.js — isolate synchronous WASI execution from the browser UI.
import { loadKuna } from './kuna-web.js';
import { makeZip } from './zip.js';

let kuna = null;
let binary = null;
let fileName = 'binary';
let mode = 'auto';
let language = 'auto';

function requireKuna() {
  if (!kuna) throw new Error('worker is not initialized');
  return kuna;
}

function requireBinary() {
  if (!binary) throw new Error('no binary is loaded');
  return binary;
}

function safeName(name) {
  return String(name || 'binary').replace(/[^A-Za-z0-9._-]/g, '_') || 'binary';
}

/** The `--assert` directives of one request: strings only, in order. */
function directives(list) {
  return Array.isArray(list) ? list.filter((d) => typeof d === 'string' && d.trim() !== '') : [];
}

function session(params) {
  return { mode, language, assertions: directives(params.assertions) };
}

async function dispatch(method, params) {
  switch (method) {
    case 'init':
      kuna = await loadKuna(params);
      return { result: true };
    case 'setBinary':
      requireKuna();
      binary = params.bytes instanceof Uint8Array ? params.bytes : new Uint8Array(params.bytes);
      fileName = params.fileName || 'binary';
      mode = params.mode || 'auto';
      language = params.language || 'auto';
      return {
        result: {
          format: kuna.formatName(binary),
          size: binary.byteLength,
        },
      };
    case 'list':
      return { result: await requireKuna().list(requireBinary(), session(params)) };
    case 'decompile':
      return {
        result: await requireKuna().decompile(requireBinary(), params.target, session(params)),
      };
    case 'inspect':
      return {
        result: await requireKuna().inspect(requireBinary(), params.target, session(params)),
      };
    case 'read':
      return {
        result: await requireKuna().read(
          requireBinary(), params.address, params.length, session(params),
        ),
      };
    case 'xrefs':
      return {
        result: await requireKuna().xrefs(requireBinary(), params.target, session(params)),
      };
    case 'project': {
      const name = safeName(params.displayName || fileName);
      const project = await requireKuna().project(requireBinary(), name, {
        mode,
        assertions: directives(params.assertions),
      });
      const entries = Object.entries(project.files)
        .map(([entryName, data]) => ({ name: `${name}.kuna/${entryName}`, data }));
      const zip = makeZip(entries);
      return {
        result: {
          downloadName: `${name}.kuna.zip`,
          bytes: zip.buffer,
          count: project.count,
          ok: project.ok,
          failed: project.failed,
        },
        transfer: [zip.buffer],
      };
    }
    default:
      throw new Error(`unknown worker method: ${method}`);
  }
}

/** What a failed request carried besides its message, in a cloneable shape. */
function detailOf(error) {
  if (!error || typeof error !== 'object') return null;
  const detail = {};
  if (typeof error.exitCode === 'number') detail.exitCode = error.exitCode;
  if (typeof error.stderr === 'string') detail.stderr = error.stderr;
  if (error.payload !== undefined) detail.payload = error.payload;
  return Object.keys(detail).length ? detail : null;
}

self.onmessage = async ({ data }) => {
  const { id, method, params = {} } = data || {};
  if (!Number.isSafeInteger(id) || typeof method !== 'string') return;
  try {
    const { result, transfer = [] } = await dispatch(method, params);
    self.postMessage({ id, ok: true, result }, transfer);
  } catch (error) {
    self.postMessage({
      id,
      ok: false,
      error: error instanceof Error ? error.message : String(error),
      detail: detailOf(error),
    });
  }
};
