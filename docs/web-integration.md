# Running kuna in the browser (WebAssembly)

How kuna decompiles **entirely client-side in a web browser** — the target choice, the
one seam that makes it work, the virtual-filesystem mapping, the build, and the tests.
Audience: kuna developers extending or maintaining the web front-end. The practical
"build and serve it" guide is `integrations/web/README.md`.

This is a genuine in-browser decompiler: the engine runs *in the page* as WebAssembly.
Nothing is uploaded and no server participates in decompilation. A WASI runtime
implemented in JavaScript hosts the module; the browser is the whole stack.

---

## 1. The decision: WASI, not a rewrite

The engine touches the outside world through an unusually thin surface:

- **no threads, no `rayon`** anywhere in the pipeline;
- the only subprocess calls are in the *CLI* layer (`kuna` spawning `decomp_dbg`) or are
  **test-only** (the hermetic `go build` in `kuna-analysis`'s no-return test) — none in
  the engine;
- engine wall-clock (`Instant::now()`) is reached *only* when the per-function watchdog
  budget is armed, which no in-process caller here does;
- everything the decompiler loads — the target binary (via `LoadImage`) and the SLEIGH
  `.sla`/`.pspec`/`.cspec`/`.ldefs` (via `scan_language_database`) — arrives through plain
  `std::fs` **path reads**.

Those path reads are the key. **WASI** (`wasm32-wasip1`) gives a wasm module a POSIX-ish
filesystem backed by *preopened* directories, which a JS host can populate from memory.
So the engine's `std::fs::read(specfile)` / `std::fs::read(binary)` calls work **unchanged**
against a virtual filesystem the page assembles. The result: the full engine + analysis
stack (`kuna-base` · `kuna-num` · `kuna-sleigh` · `kuna-decomp` · `kuna-analysis` ·
`kuna-console`) and every dependency (`object`, `gimli`, `pdb`, the demanglers, `flate2`,
`smallvec`, `slotmap`) compiles to `wasm32-wasip1` with **zero source changes**.

The rejected alternative — `wasm32-unknown-unknown` + `wasm-bindgen` — gives a cleaner
JS-native API but has no filesystem, so it would require refactoring the engine's
spec-load and image-load reads to take byte buffers. That is real engine surgery for a
cosmetic API gain; WASI keeps the engine untouched, which is the whole point ("don't break
kuna"). A `wasm-bindgen` reactor front-end can be added *later* as an optimization (see §7)
without disturbing the WASI path.

## 2. Shape: a front-end crate + a browser harness

```
decompiler/crates/kuna-wasm/     the wasm (and native) binary: `kuna_wasm`
integrations/web/                the browser harness (assembles a static dist/)
```

`kuna-wasm` is a **purely additive leaf crate**. It depends only on existing engine crates
(`kuna-console`, `kuna-decomp`, `kuna-base`) and the already-present `object`; it adds no
new external dependency, defines no stage-model option, and changes no decompiler
behavior. `make binaries` builds a fixed crate list that does not include it, and
`check_spec.py` only scans `kuna-decomp`/`kuna-analysis`, so the only gate that touches it
is `make rust-test` (`cargo test --workspace`), which merely compiles it natively.

`kuna_wasm` is a faithful, self-contained restatement of `kuna decompile-all`'s core loop
(`kuna-cli/src/decompile_all.rs`) — reproduced rather than shared because `kuna-cli` pulls
in the subprocess/CLI machinery that cannot compile for wasm. It reuses the *exact* engine
entry points:

```
bootstrap_from_object(binary, "", [spec_root])   // load image + resolve arch + build translator
  → set "listing" on (+ "funcstart_patterns" for non-x86-64)   // decompile-all defaults (DIV-15/20)
  → commit_pending_analysis()                                   // the `read symbols` seam
  → for each function:
       decompile_func_full_with_override_dyn(...)  // the same drive kuna decompile-all uses
       print_c(...) / extract_variables(...)        // identical rendering
```

Its `--json` matches `kuna decompile-all --json` field-for-field (`name`, `address`,
`address_hex`, `size`, `code`, `error`, `variables[{name,type,kind,arg_index,stack_offset,
size}]`). CLI:

```
kuna_wasm <binary> <spec-root> list                     # enumerate functions (cheap)
kuna_wasm <binary> <spec-root> decompile [name|0xADDR]  # one function, or all
```

## 3. The virtual filesystem (the whole trick)

`integrations/web/kuna-web.js` drives [`@bjorn3/browser_wasi_shim`](https://github.com/bjorn3/browser_wasi_shim)
(vendored under `integrations/web/vendor/`, pinned in `VERSION`), a pure-JS WASI
**preview1** implementation. For each decompile it builds a fresh instance with two
preopened directories:

| Preopen | Guest path | Contents |
|---|---|---|
| specs | `/specs` | the SLEIGH tree — `Ghidra/Processors/x86/data/languages/{x86.ldefs, x86-64.sla, x86-64.pspec, x86-64-gcc.cspec, x86-64.dwarf}` (fetched once, `File` inodes reused across runs) |
| work | `/work` | `input.bin` — the user's uploaded bytes |

then runs `kuna_wasm /work/input.bin /specs decompile` with `stdout` captured via
`ConsoleStdout.lineBuffered`. The captured stdout is the JSON the UI renders. This is the
same preopen model Node's `node:wasi` uses — the parity test (`test/parity.mjs`) drives
the identical wasm through `node:wasi`, and the glue test (`test/glue.mjs`) drives it
through the real browser shim; both must agree with native.

**Minimal specs.** Only the x86-64 (gcc/ELF) set is shipped — **~536 KB**, verified to
produce *byte-identical* output to the full **29 MB** tree. `scan_language_database`
indexes whatever `.ldefs` it finds and lazily loads only the resolved language's files, so
a one-language tree is sufficient and fast. Adding an architecture = add its files to
`build.sh`'s `SPEC_FILES` and a manifest in `kuna-web.js`, then select by ELF machine.

## 4. Build & payload

`integrations/web/build.sh` builds `kuna_wasm.wasm` for `wasm32-wasip1`, applies
`wasm-opt -Oz` if available, and assembles a self-contained `integrations/web/dist/`
(page + glue + vendored shim + wasm + specs). Serve `dist/` with any static file server.

**Hosting on GitHub Pages.** `.github/workflows/pages.yml` runs this same build in CI
(stable Rust + `wasm32-wasip1`, `binaryen` for `wasm-opt`, a targeted `x86-64.slaspec`
compile instead of the full `make specs`) and deploys `dist/` via `actions/deploy-pages`.
All asset references are relative, so it serves correctly from a project subpath
(`https://<owner>.github.io/<repo>/`); no COOP/COEP headers are needed (no threads /
SharedArrayBuffer). Enable once under *Settings → Pages → Source = GitHub Actions*.

Payload for a full x86-64 decompiler in the tab: **~1.7 MB** wasm + **~0.46 MB** specs,
gzipped ≈ **2.2 MB**. Cold decompile of a small binary is sub-second (≈0.45 s measured in
Node `node:wasi` and in headless Chrome on the committed fixture).

## 5. Testing

Three layers, all runnable without a browser in CI:

1. **`test/parity.mjs`** — runs the wasm under `node:wasi` (the same WASI preview1 ABI the
   browser shim implements) and asserts its output is **byte-identical to the native
   `kuna_wasm`** across `list` + `decompile {all, main, sum_to, add}`. This proves the
   port is faithful, not degraded.
2. **`test/glue.mjs`** — imports the shipped `kuna-web.js` (which drives the vendored
   `@bjorn3` shim) and decompiles over HTTP against `dist/`, exercising the exact browser
   code path minus the DOM.
3. **Full UI (optional, not committed)** — a `puppeteer-core` script drives `index.html`
   in real Chrome: upload the fixture, wait for the code panel, assert the rendered C.
   Verified passing (Chrome 131, ~0.46 s) during development; kept out of the committed
   suite to avoid a browser/`puppeteer` dependency.

The committed x86-64 ELF fixture (`test/fixtures/sample.elf`, source `sample.c`) exercises
argument passing, a call chain, and a `for`-loop so structuring is covered.

## 6. Guarantees (why this doesn't break kuna)

- No engine file changes: the wasm target reuses the native code paths verbatim.
- No new dependency, no new stage-model option, no output change: `kuna_wasm` == the
  `decompile-all` path, so `docs/divergences.md`, `phases.toml`, and `docs/options.md` are
  untouched.
- The four gates (`make test`, `make test-stages`, `make rust-test`, `make check-spec`)
  are unaffected — the crate is invisible to all but `rust-test`, where it only compiles.

## 7. Limitations & future work

- **x86-64 only in the demo** — an intentional payload choice, not an engine limit (§3).
- **Re-bootstraps per request** — a WASI *command* module runs `_start` and exits, so
  "decompile all" (one run over every function) is the efficient path the UI uses. A
  `wasm-bindgen` **reactor** front-end (bootstrap once, export `decompile(name)`) would let
  the page keep a warm `Architecture` across clicks; it can be added beside this crate
  without touching the WASI path or the engine.
- **No `wasm-opt` in the default toolchain** — the shipped wasm is unoptimized (~7 MB raw,
  ~1.7 MB gzipped); installing `binaryen` shrinks it further.

## 8. Pointers

- Harness & commands: `integrations/web/README.md`
- The crate: `decompiler/crates/kuna-wasm/{Cargo.toml, src/lib.rs, src/main.rs}`
- The `decompile-all` loop it mirrors: `decompiler/crates/kuna-cli/src/decompile_all.rs`
- The engine entry it reuses: `kuna_console::engine::bootstrap_from_object`
