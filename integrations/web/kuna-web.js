// kuna-web.js — run the kuna decompiler entirely in the browser, for WHATEVER
// binary the CLI supports (ELF / PE / Mach-O / COFF, every architecture kuna
// ships a `.sla` for). Exposes `list` (enumerate functions), `decompile` (one
// function or all of them), and `project` (whole-binary .c/.h/.asm/README
// export — the "Download Binary Source" zip).
//
// It loads the `kuna_wasm` WebAssembly module (the engine's in-process decompile
// path, compiled to wasm32-wasip1) and drives it through
// @bjorn3/browser_wasi_shim — a pure-JS WASI preview1 implementation. The SLEIGH
// specs and the user's binary live in an in-memory virtual filesystem (WASI
// preopens); the decompiler reads them exactly as on a native filesystem.
// NOTHING is sent to a server: the decompiler executes in the page.
//
// Robustness (no per-format/per-arch JS logic — the ENGINE resolves everything):
//   • Preload the SMALL runtime spec files once — all `.ldefs`/`.pspec`/`.cspec`/
//     `.dwarf` (~1.7 MB / ~180 KB gzipped, bundled as `specs-small.json`). With
//     these, the engine resolves the SLEIGH language for ANY supported binary.
//   • The one heavy per-language file, the `.sla`, is fetched LAZILY: run the
//     decompiler; if it reports `Could not find .sla file for <lang-id>`, map that
//     id → its `.sla` (via the ldefs we already ship), fetch it, and retry.
// This mirrors the CLI: the engine does all format/arch detection; the page just
// serves files on demand. See ../../docs/web-integration.md §3.
import {
  WASI,
  File,
  OpenFile,
  Directory,
  PreopenDirectory,
  ConsoleStdout,
} from './vendor/browser_wasi_shim/dist/index.js';

// Cosmetic only (status label) — NOT used to pick specs (the engine does that).
export function formatName(bytes) {
  if (bytes.length >= 4 && bytes[0] === 0x7f && bytes[1] === 0x45 && bytes[2] === 0x4c && bytes[3] === 0x46)
    return 'ELF';
  if (bytes.length >= 2 && bytes[0] === 0x4d && bytes[1] === 0x5a) return 'PE';
  if (bytes.length >= 4) {
    const be = (bytes[0] << 24) | (bytes[1] << 16) | (bytes[2] << 8) | bytes[3];
    const magics = [0xfeedface, 0xfeedfacf, 0xcefaedfe, 0xcffaedfe, 0xcafebabe]; // Mach-O (+fat)
    if (magics.includes(be >>> 0)) return 'Mach-O';
  }
  return 'binary';
}

/**
 * Build one kuna_wasm command, delegating the byte-size policy to Rust.
 *
 * `language` is the output language: 'auto' (the default) follows the binary, so
 * a Rust binary renders as Rust. The engine owns that policy too -- passing
 * 'auto' through rather than resolving it here keeps the browser and the CLI on
 * one implementation.
 */
export function wasmCommandArgs(command, arg, mode = 'auto', language = 'auto') {
  const argv = ['/work/input.bin', '/specs', command];
  if (arg) argv.push(arg);
  argv.push('--mode', mode);
  argv.push('--language', language);
  return argv;
}

// base64 → Uint8Array (works in both browser and Node — `atob` is global in both).
function b64ToBytes(b64) {
  const bin = atob(b64);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
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

// Compile the wasm, preferring streaming compilation but falling back to a
// buffered compile when the server doesn't send `Content-Type: application/wasm`.
async function compileWasm(url) {
  try {
    return await WebAssembly.compileStreaming(fetch(url));
  } catch (_) {
    const resp = await fetch(url);
    if (!resp.ok) throw new Error(`wasm fetch failed (${resp.status}): ${url}`);
    return WebAssembly.compile(await resp.arrayBuffer());
  }
}

// Extract `id → { slafile, dir }` from an `.ldefs` file's `<language …>` tags.
function parseLdefs(text, dir, map) {
  const re = /<language\b([\s\S]*?)>/g;
  let m;
  while ((m = re.exec(text))) {
    const attrs = m[1];
    const id = /id="([^"]*)"/.exec(attrs)?.[1];
    const sla = /slafile="([^"]*)"/.exec(attrs)?.[1];
    if (id && sla) map.set(id, { slafile: sla, dir });
  }
}

/**
 * Load the decompiler once: compile the wasm and preload the small spec files.
 * `.sla` files are fetched lazily per binary. Returns `{ list, decompile,
 * project, formatName }`.
 *
 * @param {object} opts
 * @param {string} opts.wasmUrl        URL of kuna_wasm.wasm
 * @param {string} opts.specRoot       base URL the spec tree lives under
 * @param {string} [opts.smallBundleUrl]  URL of specs-small.json (default:
 *                                         `${specRoot}-small.json`)
 */
export async function loadKuna({ wasmUrl, specRoot, smallBundleUrl }) {
  const base = specRoot.replace(/\/$/, '');
  const bundleUrl = smallBundleUrl || `${base}-small.json`;

  const [wasmModule, bundle] = await Promise.all([
    compileWasm(wasmUrl),
    fetch(bundleUrl).then((r) => {
      if (!r.ok) throw new Error(`spec bundle fetch failed (${r.status}): ${bundleUrl}`);
      return r.json();
    }),
  ]);

  // Build the virtual /specs tree from the small files, and the lang-id → .sla map.
  const tree = new Map();
  const langMap = new Map();
  for (const [rel, b64] of Object.entries(bundle)) {
    const bytes = b64ToBytes(b64);
    insertPath(tree, rel, new File(bytes));
    if (rel.endsWith('.ldefs')) {
      parseLdefs(new TextDecoder().decode(bytes), rel.slice(0, rel.lastIndexOf('/')), langMap);
    }
  }
  const fetchedSla = new Set(); // rel paths already fetched (cache across calls)

  // Resolve a language id from an engine error (which may carry a trailing
  // `:compiler`) to its `.sla`, trimming id segments until one matches.
  function slaForLangId(id) {
    let key = id;
    while (key) {
      if (langMap.has(key)) return langMap.get(key);
      const i = key.lastIndexOf(':');
      if (i < 0) return null;
      key = key.slice(0, i);
    }
    return null;
  }

  async function fetchSla(dir, slafile) {
    const rel = `${dir}/${slafile}`;
    if (fetchedSla.has(rel)) return;
    const r = await fetch(`${base}/${rel}`);
    if (!r.ok) throw new Error(`failed to fetch spec ${rel} (${r.status})`);
    insertPath(tree, rel, new File(new Uint8Array(await r.arrayBuffer())));
    fetchedSla.add(rel);
  }

  // One decompiler invocation over the current virtual FS. Instantiation is
  // async on purpose: synchronous `new WebAssembly.Instance` is disallowed on the
  // browser main thread for modules >4 KB (Node permits it), so we `await`
  // `WebAssembly.instantiate`; the decompile itself then runs synchronously
  // inside `wasi.start`.
  async function runOnce(binaryBytes, argv) {
    const stdout = [];
    const stderr = [];
    const fds = [
      new OpenFile(new File([])), // stdin (unused)
      ConsoleStdout.lineBuffered((l) => stdout.push(l)),
      ConsoleStdout.lineBuffered((l) => stderr.push(l)),
      new PreopenDirectory('/specs', tree),
      new PreopenDirectory('/work', new Map([['input.bin', new File(new Uint8Array(binaryBytes))]])),
    ];
    const wasi = new WASI(['kuna_wasm', ...argv], [], fds, { debug: false });
    const instance = await WebAssembly.instantiate(wasmModule, { wasi_snapshot_preview1: wasi.wasiImport });
    let exitCode = 0;
    try {
      exitCode = wasi.start(instance);
    } catch (e) {
      exitCode = e && typeof e.code === 'number' ? e.code : 1;
    }
    return { exitCode, stdout: stdout.join('\n'), stderr: stderr.join('\n') };
  }

  // Run, lazily fetching any `.sla` the engine reports missing, then retrying.
  async function invoke(binaryBytes, argv) {
    for (let round = 0; round < 8; round++) {
      const res = await runOnce(binaryBytes, argv);
      if (res.exitCode === 0) return res;
      const m = /Could not find \.sla file for (\S+)/.exec(res.stderr);
      if (m) {
        const hit = slaForLangId(m[1]);
        const rel = hit && `${hit.dir}/${hit.slafile}`;
        if (hit && !fetchedSla.has(rel)) {
          await fetchSla(hit.dir, hit.slafile);
          continue;
        }
      }
      return res; // genuine failure (or nothing new to fetch)
    }
    throw new Error('too many spec-fetch rounds (unexpected)');
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
    /** Format label for the status line (ELF / PE / Mach-O / binary). */
    formatName,
    /** Enumerate functions: `{binary, count, functions:[{name, address, address_hex}]}`. */
    async list(binaryBytes, { mode = 'auto', language = 'auto' } = {}) {
      return parseOrThrow(
        await invoke(binaryBytes, wasmCommandArgs('list', undefined, mode, language)),
        'list',
      );
    },
    /**
     * Decompile. With no `target`, decompiles ALL functions; otherwise a single
     * function by name or `0x`-address. Returns the `decompile-all --json` shape.
     */
    async decompile(binaryBytes, target, { mode = 'auto', language = 'auto' } = {}) {
      return parseOrThrow(
        await invoke(binaryBytes, wasmCommandArgs('decompile', target, mode, language)),
        'decompile',
      );
    },
    /**
     * Export the whole binary as a recompile-oriented project (every function,
     * one run). `displayName` names the artifacts. Returns `{binary, name,
     * count, ok, failed, files: {"<name>.c", "<name>.h", "<name>.asm",
     * "README.md"}}`.
     */
    async project(binaryBytes, displayName, { mode = 'auto' } = {}) {
      // The project export is C-only; no language is threaded here on purpose.
      return parseOrThrow(
        await invoke(binaryBytes, wasmCommandArgs('project', displayName, mode)),
        'project',
      );
    },
  };
}
