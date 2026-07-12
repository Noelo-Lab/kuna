# kuna in the browser (WebAssembly)

Run the kuna decompiler **entirely client-side in a web browser** — no server, no
backend, no upload. The engine (Ghidra's decompiler, ported to Rust) compiles to
`wasm32-wasip1` and runs in the page under a pure-JS WASI shim; the SLEIGH specs and the
binary you decompile live in an in-memory virtual filesystem.

This is a real in-browser decompiler, not a thin client to a remote service. The
architecture, seams, and design rationale are in **`docs/web-integration.md`**.

## Quick start

```bash
# 1. Build the .sla specs once (they are gitignored build artifacts).
make specs

# 2. Build the wasm and assemble a self-contained static site into dist/.
integrations/web/build.sh

# 3. Serve it (any static server works — nothing talks to a backend).
integrations/web/build.sh --serve       # -> http://localhost:8000
#   or:  (cd integrations/web/dist && python3 -m http.server 8000)
```

Open the page, click **Load binary**, and pick an **x86-64 ELF**. The decompiler runs in
the tab and lists every function; click one to see its C.

## What's here

| Path | Role |
|---|---|
| `index.html` | The demo UI (upload → function list → C). Self-contained, theme-aware. |
| `kuna-web.js` | The glue: loads the wasm, fetches specs into an in-memory FS, runs the decompiler under the WASI shim, returns the `decompile-all --json` shape. |
| `vendor/browser_wasi_shim/` | Vendored [`@bjorn3/browser_wasi_shim`](https://github.com/bjorn3/browser_wasi_shim) (MIT/Apache-2.0) — a pure-JS WASI **preview1** implementation. Pinned in `VERSION`. |
| `build.sh` | Builds `kuna_wasm.wasm`, copies the minimal x86-64 spec set + the shim + the page into `dist/`. `wasm-opt -Oz` is applied if present. |
| `test/` | Automated gates (below) + a committed x86-64 ELF fixture. |
| `dist/` | Assembled output (gitignored — regenerate with `build.sh`). |

The wasm module itself is the `kuna-wasm` crate (`decompiler/crates/kuna-wasm`), a thin
front-end over the `kuna-console` in-process path. It builds natively too — that native
build is the parity oracle for the tests.

## Tests

All run under Node (no browser required for CI):

```bash
# 0. Prereqs: `make specs`, then `integrations/web/build.sh`, and a native
#    kuna_wasm:  (cd decompiler && cargo build --release -p kuna-wasm)

# A. WASI parity — the wasm runs under Node's node:wasi and its output is
#    byte-identical to the native kuna_wasm build (the port is faithful).
node integrations/web/test/parity.mjs

# B. Browser-glue — the ACTUAL browser code path (kuna-web.js + the vendored
#    @bjorn3 shim) decompiling over HTTP against dist/.
node integrations/web/test/glue.mjs
```

- **`parity.mjs`** proves the decompiler *runs* under a WASI runtime and matches native
  byte-for-byte across `list` + several `decompile` cases.
- **`glue.mjs`** proves the exact browser stack (the vendored WASI shim + `kuna-web.js`)
  works, minus the DOM.
- **`run-wasm.mjs`** is a small reusable CLI runner (used by `parity.mjs`; also handy for
  driving the wasm by hand under `node:wasi`).

### Optional: full-UI check in real Chrome

A headless-browser smoke test (drives `index.html` in Chrome, uploads the fixture, reads
the rendered C) needs `puppeteer-core` + a local Chrome/Chromium and so is not committed.
A ready-to-run script:

```bash
npm i -g puppeteer-core   # or local; needs a Chrome on PATH
# then a ~40-line script: serve dist/, launch chrome, uploadFile(sample.elf),
# waitForFunction(code contains 'return'), assert. See docs/web-integration.md §Testing.
```

## Scope & extending

The demo ships the **x86-64 (gcc/ELF)** SLEIGH set — verified byte-identical to the full
29 MB spec tree, but only ~536 KB. To support more architectures, add their spec files to
`SPEC_FILES` in `build.sh` and to `X86_64_SPECS`/a new manifest in `kuna-web.js`, then
pick the manifest by the uploaded binary's ELF machine. The engine already decompiles
every architecture kuna supports; only the spec-delivery layer is arch-scoped here.

## Payload

For a full x86-64 decompiler in the tab: **~1.7 MB** wasm (gzipped) + **~0.46 MB** specs
(gzipped) ≈ **2.2 MB**. Cold decompile of a small binary is well under a second.
