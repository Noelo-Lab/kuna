// run-wasm.mjs — run the kuna decompiler wasm under Node's built-in WASI
// (preview1). This is the SAME WASI preview1 ABI a browser WASI shim
// (@bjorn3/browser_wasi_shim) implements, so a green run here is direct evidence
// the wasm decompiles client-side with no server. The specs tree and the target
// binary are exposed to the guest through preopened virtual-filesystem
// directories — exactly how the browser harness preloads them.
//
// Usage:
//   node --experimental-wasi-unstable-preview1 run-wasm.mjs \
//        <wasm> <specs-dir> <binary> <list|decompile> [name|0xaddr]
//
// Writes the decompiler's stdout (JSON) to this process's stdout.
import { WASI } from 'node:wasi';
import { readFile } from 'node:fs/promises';
import { basename, dirname, resolve } from 'node:path';

const [wasmPath, specsDir, binaryPath, cmd, arg] = process.argv.slice(2);
if (!wasmPath || !specsDir || !binaryPath || !cmd) {
  console.error('usage: run-wasm.mjs <wasm> <specs-dir> <binary> <list|decompile> [name|0xaddr]');
  process.exit(64);
}

const workDir = resolve(dirname(binaryPath));
const guestBinary = '/work/' + basename(binaryPath);
const args = ['kuna_wasm', guestBinary, '/specs', cmd];
if (arg !== undefined) args.push(arg);

const wasi = new WASI({
  version: 'preview1',
  args,
  env: {},
  preopens: {
    '/specs': resolve(specsDir),
    '/work': workDir,
  },
  returnOnExit: true,
});

const wasm = await WebAssembly.compile(await readFile(wasmPath));
const instance = await WebAssembly.instantiate(wasm, wasi.getImportObject());
const code = wasi.start(instance);
process.exit(code ?? 0);
