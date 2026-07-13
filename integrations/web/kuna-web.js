// kuna-web.js — run the kuna decompiler entirely in the browser.
//
// This loads the `kuna_wasm` WebAssembly module (the engine's in-process
// decompile path, compiled to wasm32-wasip1) and drives it through
// @bjorn3/browser_wasi_shim — a pure-JS WASI preview1 implementation. The
// SLEIGH specs and the user's binary are placed in an in-memory virtual
// filesystem (WASI preopens); the decompiler reads them exactly as it would on
// a native filesystem. NOTHING is sent to a server: the decompiler executes in
// the page. See ../../docs/web-integration.md.
//
// Multi-arch: the ENGINE decompiles every architecture kuna supports; the only
// arch-scoped part here is which SLEIGH spec files to preload. `loadKuna` reads
// the uploaded binary's ELF machine and lazily fetches + caches that arch's spec
// set (see `ARCHES`). Add an arch by adding its manifest here and shipping its
// spec files (see build.sh + docs/web-integration.md §3).
import {
  WASI,
  File,
  OpenFile,
  Directory,
  PreopenDirectory,
  ConsoleStdout,
} from './vendor/browser_wasi_shim/dist/index.js';

const X86_DIR = 'Ghidra/Processors/x86/data/languages';
const AARCH64_DIR = 'Ghidra/Processors/AARCH64/data/languages';

// ELF `e_machine` → the minimal SLEIGH spec set for that arch's default
// (gcc/ELF) language. `files` are paths relative to `specRoot`, mirroring the
// on-disk `Ghidra/Processors/...` layout `scan_language_database` scans. Each
// set is verified to produce byte-identical output to the full 29 MB spec tree.
export const ARCHES = {
  0x3e: {
    name: 'x86-64',
    files: [
      `${X86_DIR}/x86.ldefs`,
      `${X86_DIR}/x86-64.sla`,
      `${X86_DIR}/x86-64.pspec`,
      `${X86_DIR}/x86-64-gcc.cspec`,
      `${X86_DIR}/x86-64.dwarf`,
    ],
  },
  0xb7: {
    name: 'aarch64',
    files: [
      `${AARCH64_DIR}/AARCH64.ldefs`,
      `${AARCH64_DIR}/AARCH64.sla`,
      `${AARCH64_DIR}/AARCH64.pspec`,
      `${AARCH64_DIR}/AARCH64.cspec`,
      `${AARCH64_DIR}/AARCH64.dwarf`,
    ],
  },
};

// Read the ELF `e_machine` (u16 @ offset 18, honoring EI_DATA endianness).
// Returns null for a non-ELF buffer.
export function elfMachine(bytes) {
  if (bytes.length < 20 || bytes[0] !== 0x7f || bytes[1] !== 0x45 || bytes[2] !== 0x4c || bytes[3] !== 0x46)
    return null;
  const le = bytes[5] === 1; // EI_DATA: 1 = little-endian
  return le ? bytes[18] | (bytes[19] << 8) : (bytes[18] << 8) | bytes[19];
}

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
 * Load the decompiler once: compile the wasm. SLEIGH specs are fetched lazily
 * per architecture on first use and cached. Returns a handle with
 * `.decompile()` / `.list()` / `.archName()`.
 *
 * @param {object} opts
 * @param {string} opts.wasmUrl   URL of kuna_wasm.wasm
 * @param {string} opts.specRoot  base URL the spec files live under
 * @param {object} [opts.arches]  ELF-machine → manifest map (defaults to ARCHES)
 */
export async function loadKuna({ wasmUrl, specRoot, arches = ARCHES }) {
  const wasmModule = await compileWasm(wasmUrl);
  const base = specRoot.replace(/\/$/, '');
  const treeCache = new Map(); // machine → spec-tree Map (lazily built, reused)

  async function specTreeFor(machine) {
    if (treeCache.has(machine)) return treeCache.get(machine);
    const arch = arches[machine];
    if (!arch) return null;
    const bufs = await Promise.all(
      arch.files.map((f) =>
        fetch(`${base}/${f}`).then((r) => {
          if (!r.ok) throw new Error(`spec fetch failed (${r.status}): ${f}`);
          return r.arrayBuffer();
        })
      )
    );
    const tree = new Map();
    arch.files.forEach((f, i) => insertPath(tree, f, new File(new Uint8Array(bufs[i]))));
    treeCache.set(machine, tree);
    return tree;
  }

  async function invoke(binaryBytes, argv) {
    const machine = elfMachine(binaryBytes);
    if (machine === null) throw new Error('not an ELF file (bad magic)');
    const tree = await specTreeFor(machine);
    if (!tree) {
      const supported = Object.values(arches).map((a) => a.name).join(', ');
      throw new Error(
        `unsupported architecture (ELF e_machine 0x${machine.toString(16)}). ` +
          `Supported here: ${supported}. Add its specs — see docs/web-integration.md §3.`
      );
    }
    const stdoutChunks = [];
    const stderrChunks = [];
    const fds = [
      new OpenFile(new File([])), // stdin (unused)
      ConsoleStdout.lineBuffered((line) => stdoutChunks.push(line)),
      ConsoleStdout.lineBuffered((line) => stderrChunks.push(line)),
      new PreopenDirectory('/specs', tree),
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
      exitCode = e && typeof e.code === 'number' ? e.code : 1;
    }
    return { exitCode, stdout: stdoutChunks.join('\n'), stderr: stderrChunks.join('\n') };
  }

  function parseOrThrow(res, what) {
    if (res.exitCode !== 0) throw new Error(res.stderr || `${what} failed (exit ${res.exitCode})`);
    try {
      return JSON.parse(res.stdout);
    } catch (e) {
      throw new Error(`${what}: could not parse decompiler output: ${e.message}`);
    }
  }

  return {
    /** The supported arch name for these bytes, or null (unknown/non-ELF). */
    archName(binaryBytes) {
      const m = elfMachine(binaryBytes);
      return (m !== null && arches[m]?.name) || null;
    },
    /** Enumerate functions: `{binary, count, functions:[{name, address, address_hex}]}`. */
    async list(binaryBytes) {
      return parseOrThrow(await invoke(binaryBytes, ['/work/input.bin', '/specs', 'list']), 'list');
    },
    /**
     * Decompile. With no `target`, decompiles ALL functions; otherwise a single
     * function by name or `0x`-address. Returns the `decompile-all --json` shape.
     */
    async decompile(binaryBytes, target) {
      const argv = ['/work/input.bin', '/specs', 'decompile'];
      if (target) argv.push(target);
      return parseOrThrow(await invoke(binaryBytes, argv), 'decompile');
    },
  };
}
