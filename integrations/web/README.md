# The kuna site (and the in-browser decompiler)

**kuna.noelo.org** — the project site, and the decompiler that runs **entirely client-side
in a web browser**: no server, no backend, no upload. Two pages, one static bundle:

| URL | Page |
|---|---|
| `/` | **Landing** — what kuna is, a side-by-side compare section, the three project goals. Inert: no wasm, no network. |
| `/decompile/` | **The decompiler** — load an ELF/PE/Mach-O and read its C, decompiled in the tab. |

The engine (Ghidra's decompiler, ported to Rust) compiles to `wasm32-wasip1` and runs in
the page under a pure-JS WASI shim; the SLEIGH specs and the binary you decompile live in
an in-memory virtual filesystem. This is a real in-browser decompiler, not a thin client to
a remote service. The architecture, seams, and design rationale are in
**`docs/web-integration.md`** (the site layout is §4.1).

The look shares the Noelo Lab site's palette and typefaces (`noelo.org`, BSD-2-Clause —
provenance note at the top of `assets/css/site.css`) but is laid out as a tool page: light,
flat, monospace throughout, ink + paper + the mark's red.

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

Open the landing page, follow **Try it** to `/decompile/`, click **Load binary**, and pick
any **ELF, PE, or Mach-O** binary. A module Worker inventories the functions without
blocking the page; click one to decompile just that address and see its C, **syntax
highlighted** (comments/strings/keywords/types/numbers/calls, plus the kuna type family and
`dat_<hex>` globals). The sidebar groups **PLT stubs and import thunks** (the engine's
per-function `kind`) below a divider, in purple, so real code stays on top. The
**Download Binary Source** button exports the whole binary as a recompile-oriented project
— `<name>.c` / `<name>.h` / `<name>.asm` / `README.md`, the `kuna decompile-project`
artifacts — and builds the `<name>.kuna.zip` inside the Worker. Only the final ZIP buffer
crosses back to the page. It supports whatever the CLI supports — every format and
architecture kuna has a `.sla` for — with no per-format configuration (the engine resolves
each binary; the Worker fetches only the one `.sla` it needs). See
`docs/web-integration.md` §3.

WASI execution is synchronous once `wasi.start()` enters the WebAssembly module. The page
therefore cancels an inventory, function, or project operation by terminating the Worker,
not by sending it a message that cannot be processed until decompilation finishes. The RPC
client immediately creates a fresh Worker and retains the selected binary so the next
operation can rebuild the session. Cancellation is a responsiveness boundary, not a claim
that the underlying decompilation takes less time or memory.

The WASM front-end also selects a decompiler mode from the uploaded byte length: binaries
below 500 KiB use `aggressive`, binaries from 500 KiB through just below 2 MiB use
`reliable`, and binaries at or above 2 MiB use `fast`. The same automatic choice applies to
function listing, decompilation, and project export. The JavaScript glue requests
`--mode auto`; the Rust front-end owns the exact boundary policy.

## Hosted on GitHub Pages

`.github/workflows/pages.yml` builds this bundle and publishes it to GitHub Pages on every
push to `main` that touches `integrations/web/`, `decompiler/`, or `specs/` (and on manual
dispatch). The workflow builds the SLEIGH specs (`make specs`), runs `build.sh`,
and deploys `dist/` — nothing is committed. The site is served at **`kuna.noelo.org`**
(the `CNAME` in the bundle), and equally from `https://<owner>.github.io/<repo>/` — every
asset path is relative, so a project subpath just works.

**One-time enablement** (repo admin): *Settings → Pages → Build and deployment → Source =
**GitHub Actions***. The next push to `main` (or a manual run from the Actions tab) deploys.

## What's here

| Path | Role |
|---|---|
| `index.html` | The landing page: hero, the compare section, the three goals. Static — its only script wires the two dropdowns. |
| `decompile/index.html` | The decompiler application (upload → inventory → lazy highlighted C, stubs grouped, cancellable project-zip download). Reaches the worker and shared assets at the bundle root with `../`. |
| `compare-samples.js` | Data for the compare section: `SAMPLES` (kuna's output per function) × `RIVALS` (the right-hand pane), with each sample's measured DecBench GED. Adding a comparison is a data edit; the header documents the schema. Every pane must be **verbatim** tool output — mine and vet new ones with `python3 -m scripts.decbench.showcase` (`docs/decbench-loop.md` → *Finding good kuna examples*). |
| `assets/` | The shared design system: `css/site.css`, `fonts/` (Jost, Roboto Mono), `img/` (mark + favicon, derived from `assets/kuna.png`), and `js/highlight-c.js` — the one C highlighter both pages use. |
| `CNAME` | The custom domain (`kuna.noelo.org`); `build.sh` copies it into `dist/`. Repo *Settings → Pages → Custom domain* must agree. |
| `kuna-web.js` | The glue: loads the wasm, preloads the small spec bundle, lazily fetches each binary's `.sla` on demand (driven by the engine's own resolution), and runs the decompiler under the WASI shim with the automatic size-based mode policy; `list`/`decompile` return the `decompile-all --json` shape, `project` the whole-binary export. |
| `kuna-worker.js` | The module Worker: owns the binary and WASI handle, lists first, decompiles one selected address, and builds whole-project ZIPs off the UI thread. |
| `kuna-worker-client.js` | The page-side RPC client. Cancellation terminates/recreates its Worker and rehydrates the retained binary on the next request. |
| `zip.js` | Dependency-free STORE-only ZIP writer (CRC-32, UTF-8 names, fixed timestamp → deterministic), invoked inside the Worker for Download Binary Source. |
| `vendor/browser_wasi_shim/` | Vendored [`@bjorn3/browser_wasi_shim`](https://github.com/bjorn3/browser_wasi_shim) (MIT/Apache-2.0) — a pure-JS WASI **preview1** implementation. Pinned in `VERSION`. |
| `build.sh` | Builds `kuna_wasm.wasm`, copies the full runtime SLEIGH tree + the shim + both pages + `assets/` into `dist/`, and bundles the small spec files into `specs-small.json`. `wasm-opt -Oz` is applied if present. |
| `test/` | Automated gates (below) + committed ELF/Mach-O fixtures. |
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

# C. Zip writer — no build needed (zip.js has no wasm dependency).
node integrations/web/test/zip.mjs

# D. Automatic-mode argv wiring — no build needed.
node integrations/web/test/auto-mode.mjs

# E. Worker boundary — real module Worker, lazy address decompile, hard
#    cancellation/restart, and worker-side project ZIP.
node integrations/web/test/worker.mjs
```

- **`parity.mjs`** proves the decompiler *runs* under a WASI runtime and matches native
  byte-for-byte across `list` + several `decompile` cases + a whole-binary `project` export,
  and checks the automatic mode immediately below and at both size boundaries.
- **`glue.mjs`** proves the exact browser stack (the vendored WASI shim + `kuna-web.js`)
  works, minus the DOM — including the lazy-`.sla` mechanism across ELF + Mach-O, x86-64 +
  AArch64, and the `project` export the download button uses.
- **`zip.mjs`** validates `zip.js` structurally: it re-parses its own output (EOCD, central
  directory, local headers), recomputes every CRC-32 independently, asserts the archive is
  byte-deterministic, and (if a system `unzip` exists) runs `unzip -t` as a bonus check.
- **`auto-mode.mjs`** pins the explicit `--mode auto` WASM argv for all three browser
  operations and the programmatic explicit-mode override.
- **`worker.mjs`** runs the shipped module Worker and RPC client through Node's worker
  threads, proving inventory-first/lazy-address behavior, terminate-and-recreate
  cancellation, session rehydration, and transfer of a complete project ZIP without its
  intermediate artifact object.
- **`run-wasm.mjs`** is a small reusable CLI runner (used by `parity.mjs`; also handy for
  driving the wasm by hand under `node:wasi`).

### Optional: full-UI check in real Chrome

A headless-browser smoke test (drives `decompile/index.html` in Chrome, uploads the
fixture, reads the rendered C) needs `puppeteer-core` + a local Chrome/Chromium and so is
not committed. A ready-to-run script:

```bash
npm i -g puppeteer-core   # or local; needs a Chrome on PATH
# then a ~40-line script: serve dist/, launch chrome, uploadFile(sample.elf),
# waitForFunction(code contains 'return'), assert. See docs/web-integration.md §Testing.
```

No `puppeteer` at hand? Plain headless Chrome can drive the real page too: copy
`dist/decompile/index.html` to a scratch page with a module script appended that sets
`#file`'s `files` from a `DataTransfer` and dispatches `change`, then run
`--headless --virtual-time-budget=60000 --dump-dom` (or `--screenshot`) against it.

## Scope

The demo supports **whatever the CLI supports** — every format (ELF/PE/Mach-O/COFF) and
every architecture kuna ships a `.sla` for — **with nothing to configure**. `build.sh`
ships the full runtime SLEIGH tree and bundles the small files; `kuna-web.js` preloads that
bundle and lazily fetches each binary's `.sla` on demand, driven by the *engine's* own
language resolution (no `e_machine` parsing, no per-arch manifest). Adding a processor to
kuna automatically makes it available here once its `.sla` builds. See
`docs/web-integration.md` §3 for the mechanism.

## Payload

**~1.7 MB** wasm (gzipped) + a **~180 KB** gzipped spec bundle, once; then **~475 KB** per
distinct language (`.sla`, fetched on demand and cached). The full `.sla` tree (~15 MB) sits
on the server; only what a binary resolves to actually transfers. Cold decompile of a small
binary is well under a second.
