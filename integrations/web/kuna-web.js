// kuna-web.js — run the kuna decompiler entirely in the browser.
//
// This loads the `kuna_wasm` WebAssembly module (the engine's in-process
// decompile path, compiled to wasm32-wasip1) and drives it through
// @bjorn3/browser_wasi_shim — a pure-JS WASI preview1 implementation. The
// SLEIGH specs and the user's binary are placed in an in-memory virtual
// filesystem (WASI preopens); the decompiler reads them exactly as it would on
// a native filesystem. NOTHING is sent to a server: the decompiler executes in
// the page. See ../../docs/web-integration.md.
import {
  WASI,
  File,
  OpenFile,
  Directory,
  PreopenDirectory,
  ConsoleStdout,
} from './vendor/browser_wasi_shim/dist/index.js';

// Default spec manifest: the minimal x86-64 (gcc/ELF) SLEIGH set — verified to
// produce byte-identical output to the full 29 MB spec tree. Paths are relative
// to `specRoot` and mirror the on-disk `Ghidra/Processors/...` layout the
// language database scans.
export const X86_64_SPECS = [
  'Ghidra/Processors/x86/data/languages/x86.ldefs',
  'Ghidra/Processors/x86/data/languages/x86-64.sla',
  'Ghidra/Processors/x86/data/languages/x86-64.pspec',
  'Ghidra/Processors/x86/data/languages/x86-64-gcc.cspec',
  'Ghidra/Processors/x86/data/languages/x86-64.dwarf',
];

// Compile the wasm, preferring streaming compilation but falling back to a
// buffered compile when the server doesn't send `Content-Type: application/wasm`
// (many static file servers don't), which would make `compileStreaming` reject.
async function compileWasm(url) {
  try {
    return await WebAssembly.compileStreaming(fetch(url));
  } catch (_) {
    const resp = await fetch(url);
    if (!resp.ok) throw new Error(`wasm fetch failed (${resp.status}): ${url}`);
    return WebAssembly.compile(await resp.arrayBuffer());
  }
}

// Insert `inode` at `path` (slash-separated) into a nested Map tree, creating
// intermediate Directory nodes as needed.
function insertPath(rootMap, path, inode) {
  const parts = path.split('/').filter((p) => p.length > 0);
  let map = rootMap;
  for (let i = 0; i < parts.length - 1; i++) {
    const name = parts[i];
    let child = map.get(name);
    if (!child) {
      child = new Directory(new Map());
      map.set(name, child);
    }
    map = child.contents;
  }
  map.set(parts[parts.length - 1], inode);
}

/**
 * Load the decompiler once: compile the wasm and fetch the SLEIGH specs into an
 * in-memory tree. Returns a handle with `.decompile()` / `.list()`.
 *
 * @param {object} opts
 * @param {string} opts.wasmUrl   URL of kuna_wasm.wasm
 * @param {string} opts.specRoot  base URL the spec files live under
 * @param {string[]} [opts.specFiles]  spec paths relative to specRoot
 */
export async function loadKuna({ wasmUrl, specRoot, specFiles = X86_64_SPECS }) {
  const [wasmModule, ...specBufs] = await Promise.all([
    compileWasm(wasmUrl),
    ...specFiles.map((f) =>
      fetch(`${specRoot.replace(/\/$/, '')}/${f}`).then((r) => {
        if (!r.ok) throw new Error(`spec fetch failed (${r.status}): ${f}`);
        return r.arrayBuffer();
      })
    ),
  ]);

  // Build the /specs tree once; the File inodes are immutable and reused across
  // every decompile call (only the /work binary changes).
  const specTree = new Map();
  specFiles.forEach((f, i) => {
    insertPath(specTree, f, new File(new Uint8Array(specBufs[i])));
  });

  async function invoke(binaryBytes, argv) {
    const stdoutChunks = [];
    const stderrChunks = [];
    const fds = [
      new OpenFile(new File([])), // stdin (unused)
      ConsoleStdout.lineBuffered((line) => stdoutChunks.push(line)),
      ConsoleStdout.lineBuffered((line) => stderrChunks.push(line)),
      new PreopenDirectory('/specs', specTree),
      new PreopenDirectory(
        '/work',
        new Map([['input.bin', new File(new Uint8Array(binaryBytes))]])
      ),
    ];
    const wasi = new WASI(['kuna_wasm', ...argv], [], fds, { debug: false });
    const instance = await WebAssembly.instantiate(wasmModule, {
      wasi_snapshot_preview1: wasi.wasiImport,
    });
    let exitCode = 0;
    try {
      exitCode = wasi.start(instance);
    } catch (e) {
      // WASIProcExit carries a nonzero code; surface stderr instead.
      exitCode = e && typeof e.code === 'number' ? e.code : 1;
    }
    return {
      exitCode,
      stdout: stdoutChunks.join('\n'),
      stderr: stderrChunks.join('\n'),
    };
  }

  function parseOrThrow(res, what) {
    if (res.exitCode !== 0) {
      throw new Error(res.stderr || `${what} failed (exit ${res.exitCode})`);
    }
    try {
      return JSON.parse(res.stdout);
    } catch (e) {
      throw new Error(`${what}: could not parse decompiler output: ${e.message}`);
    }
  }

  return {
    /** Enumerate functions: `{binary, count, functions:[{name, address, address_hex}]}`. */
    async list(binaryBytes) {
      const res = await invoke(binaryBytes, ['/work/input.bin', '/specs', 'list']);
      return parseOrThrow(res, 'list');
    },
    /**
     * Decompile. With no `target`, decompiles ALL functions; otherwise a single
     * function by name or `0x`-address. Returns the `decompile-all --json` shape.
     */
    async decompile(binaryBytes, target) {
      const argv = ['/work/input.bin', '/specs', 'decompile'];
      if (target) argv.push(target);
      const res = await invoke(binaryBytes, argv);
      return parseOrThrow(res, 'decompile');
    },
  };
}
