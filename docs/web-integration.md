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
integrations/web/                the project site + browser harness (assembles a static dist/)
```

The deployed site is two pages: a **landing page** at `/` and the **decompiler
application** at `/decompile/` (§4.1). Only the second one loads the wasm; everything below
describes it.

`kuna-wasm` is a **purely additive leaf crate**. It depends only on existing engine crates
(`kuna-console`, `kuna-decomp`, `kuna-base`) and the already-present `object`; it adds no
new external dependency, defines no stage-model option, and changes no decompiler
behavior. `make binaries` builds a fixed crate list that does not include it, and
`check_spec.py` only scans `kuna-decomp`/`kuna-analysis`, so the only gate that touches it
is `make rust-test` (`cargo test --workspace`), which merely compiles it natively.

`kuna_wasm` runs `kuna decompile-all`'s core loop via the **shared decompile-project
core** — `kuna_console::project` (`decompile_targets` + the `.c`/`.h`/`.asm`/`README.md`
artifact builders, moved there from `kuna-cli` so wasm32-wasip1 can reach them without
`kuna-cli`'s subprocess/CLI machinery, which cannot compile for wasm). It reuses the
*exact* engine entry points:

```
bootstrap_from_object(binary, "", [spec_root])   // load image + resolve arch + build translator
  → set "listing" on (+ "funcstart_patterns" for non-x86-64)   // decompile-all defaults (DIV-15/20)
  → commit_pending_analysis()                                   // the `read symbols` seam
  → kuna_console::project::decompile_targets(...)  // the same loop kuna decompile-all runs
```

Its `--json` is `kuna decompile-all --json`'s fields (`name`, `address`, `address_hex`,
`size`, `code`, `error`, `variables[{name,type,kind,arg_index,stack_offset,size}]`) plus
one kuna-wasm-only per-function field: `"kind"` — `"func"` | `"plt"` | `"thunk"`
(`kuna-wasm/src/classify.rs`: an `object`-crate re-parse marks entries inside import-stub
sections — the `.plt` family, Mach-O symbol stubs — or named as imports as `"plt"`, and
lone-jump entries (`ConsoleProgram::lone_jump_target`, direct-to-another-function or
indirect) as `"thunk"`; the UI folds those below a divider). CLI:

```
kuna_wasm <binary> <spec-root> list                       # enumerate functions (cheap)
kuna_wasm <binary> <spec-root> decompile [name|0xADDR]    # one function, or all
kuna_wasm <binary> <spec-root> project [<display-name>]   # whole-binary .c/.h/.asm/README
```

`project` is the `kuna decompile-project` flow with the folder write replaced by one JSON
document — `{binary, name, count, ok, failed, files:{"<name>.c", "<name>.h", "<name>.asm",
"README.md"}}` (artifacts named after `<display-name>`, default the binary's basename;
whole binary only). The page's **Download Binary Source** button runs it and zips the four
artifacts client-side (`integrations/web/zip.js`, a dependency-free STORE zip writer). The
only artifact difference vs the CLI is the README's `Path` row, which shows the display
name instead of a canonicalized host path (there is none in the virtual FS).

## 3. The virtual filesystem (the whole trick)

`integrations/web/kuna-web.js` drives [`@bjorn3/browser_wasi_shim`](https://github.com/bjorn3/browser_wasi_shim)
(vendored under `integrations/web/vendor/`, pinned in `VERSION`), a pure-JS WASI
**preview1** implementation. For each decompile it builds a fresh instance with two
preopened directories:

| Preopen | Guest path | Contents |
|---|---|---|
| specs | `/specs` | the SLEIGH tree — all the small runtime files preloaded, plus each `.sla` added as it's lazily fetched (`File` inodes reused across runs) |
| work | `/work` | `input.bin` — the user's uploaded bytes |

then runs `kuna_wasm /work/input.bin /specs decompile` with `stdout` captured via
`ConsoleStdout.lineBuffered`. The captured stdout is the JSON the UI renders. This is the
same preopen model Node's `node:wasi` uses — the parity test (`test/parity.mjs`) drives
the identical wasm through `node:wasi`, and the glue test (`test/glue.mjs`) drives it
through the real browser shim; both must agree with native.

**Robust, format-agnostic specs (whatever the CLI supports).** The demo carries
**no per-format or per-arch logic** — the *engine* detects the format
(ELF/PE/Mach-O/COFF) and resolves the SLEIGH language for any binary, exactly as
`kuna decompile-all` does. Two facts make a lightweight lazy scheme possible:

- The decompiler reads only the **runtime** spec files — `.ldefs` + `.pspec` +
  `.cspec` + `.dwarf` (not the `.sinc`/`.slaspec` SLEIGH *source*). Those total
  just **~1.7 MB (~180 KB gzipped)** across the whole tree, so `build.sh` bundles
  them into `specs-small.json`, which `kuna-web.js` preloads once. With them
  present, `scan_language_database` can resolve **any** binary.
- The one heavy per-language file — the **`.sla`** (~475 KB) — is the only thing
  fetched per binary. When it's absent the engine fails with
  `Could not find .sla file for <language-id>`; `kuna-web.js` maps that id → its
  `.sla` (via the `slafile=` in the ldefs it already holds), fetches it from the
  server, and retries. Fetched `.sla` are cached across decompiles.

So the browser downloads the ~180 KB bundle once, then ~475 KB per distinct
language — and supports every format/architecture kuna has a `.sla` for, with the
engine as the single source of truth (no `e_machine` parsing, no arch manifest).
Verified end to end here: ELF (x86-64, AArch64) and Mach-O `native == wasm ==
browser`, and a real PE executable (152 functions) through the browser lazy path.

## 4. Build & payload

`integrations/web/build.sh` builds `kuna_wasm.wasm` for `wasm32-wasip1`, applies
`wasm-opt -Oz` if available, and assembles a self-contained `integrations/web/dist/`:
the site + glue + vendored shim + wasm, the **full runtime SLEIGH tree** under `specs/`
(every `.ldefs`/`.pspec`/`.cspec`/`.dwarf`/`.sla` — ~15 MB static, lazily fetched), and
the `specs-small.json` preload bundle. Serve `dist/` with any static file server.

### 4.1 The site layout

```
/                     index.html          landing page: hero, compare, goals
/decompile/           decompile/index.html the decompiler application (loads the wasm)
/assets/              css/site.css · fonts/ · img/ · js/highlight-c.js
/compare-samples.js   the compare section's data (samples + rival outputs)
/CNAME                kuna.noelo.org — the custom domain, copied into the bundle
/kuna-web.js /zip.js /kuna_wasm.wasm /specs/ /specs-small.json /vendor/
```

The engine-facing files stay at the **root** — `/decompile/` reaches them with `../`, so
`test/glue.mjs` and `test/parity.mjs` (which serve `dist/` and import `kuna-web.js`
directly) are unaffected by the page move, and a project subpath still works.

The design shares the Noelo Lab site's palette and typefaces (`noelo.org`, BSD-2-Clause;
provenance note at the top of `assets/css/site.css`) but not its layout: kuna's pages are
tool pages — one display line, then monospace throughout, small red-ticked section labels
instead of a lab-page rail. One stylesheet serves both pages; `assets/js/highlight-c.js` is
the single C highlighter shared by the compare panes and the function view. The landing
page is otherwise inert — no wasm, no network — and `compare-samples.js` is pure data, so
adding a comparison is a data edit (its header documents the schema; every pane must be
verbatim tool output).

The samples are **mined, not chosen by hand**: `python3 -m scripts.decbench.showcase` reads
the DecBench results tree for optimized, medium-sized functions where kuna out-scores IDA
and no rival out-scores kuna, dumps all five panes plus the original source per candidate,
and re-decompiles each one with the current build (`--verify`) so a shipped pane is still
byte-for-byte what kuna prints today. Every sample carries the measured GED for the pair on
screen (`ged:` in the sample, rendered under the dropdowns). A mined candidate is never
shipped unread — the selection procedure, including what disqualifies a sample, is
`docs/decbench-loop.md` → *Finding good kuna examples*.

The section shows **provenance and the measured score, and nothing else** — no captions,
and a neutral dropdown label (`fn() — project binary, arch`); the reader draws their own
conclusion from the two panes. The right pane defaults to IDA. The one display-only
normalization is in `index.html`'s `tightenHeader`: kuna and Ghidra both print a blank line
between a function's signature and its opening brace, and it is collapsed so the panes
start level. It is whitespace, and only before a column-0 `{` — the committed data stays
byte-verbatim, which is what `--verify` checks against.

**Hosting on GitHub Pages.** `.github/workflows/pages.yml` runs this same build in CI
(stable Rust + `wasm32-wasip1`, `binaryen` for `wasm-opt`, `make specs` to compile the
whole `.sla` tree) and deploys `dist/` via `actions/deploy-pages`. All asset references are
relative, so it serves correctly either from the custom domain or from a project subpath
(`https://<owner>.github.io/<repo>/`); no COOP/COEP headers are needed (no threads /
SharedArrayBuffer). Enable once under *Settings → Pages → Source = GitHub Actions*.

The site is **`kuna.noelo.org`**: `integrations/web/CNAME` is copied into the bundle by
`build.sh`, and DNS points that name at GitHub Pages. With the Actions deploy flow the
repo's *Settings → Pages → Custom domain* field is the authoritative half — set it there
too, or the CNAME file alone may not claim the name.

Payload: **~1.7 MB** wasm (gzipped, shared) + a **~180 KB** gzipped spec bundle once, then
**~475 KB** per distinct language (`.sla`, fetched on demand and cached). The ~15 MB of
`.sla` sit on the server; only what a binary actually resolves to transfers. Cold decompile
of a small binary is sub-second (≈0.45 s measured in Node `node:wasi` and in headless
Chrome on the committed fixtures).

## 5. Testing

Three layers, all runnable without a browser in CI, spanning **multiple formats and
architectures**:

1. **`test/parity.mjs`** — runs the wasm under `node:wasi` (the same WASI preview1 ABI the
   browser shim implements) and asserts its output is **byte-identical to the native
   `kuna_wasm`** across `list` + `decompile {…}` + a whole-binary `project` export for each
   fixture (15 cases across ELF x86-64, ELF AArch64, and Mach-O x86-64). This proves the
   port is faithful, not degraded.
2. **`test/glue.mjs`** — imports the shipped `kuna-web.js` (which drives the vendored
   `@bjorn3` shim) and decompiles over HTTP against `dist/`, exercising the exact browser
   code path minus the DOM — and specifically the **robust lazy-spec mechanism**: it
   preloads only `specs-small.json`, then decompiles an ELF (x86-64), an ELF (AArch64), and
   a **Mach-O** through the same handle, lazily fetching each `.sla`, with no per-format JS —
   plus the `project` export the download button uses.
   (**`test/zip.mjs`**, a fourth gate needing no build, structurally validates the `zip.js`
   writer: it re-parses its own archive, recomputes every CRC-32 independently, and asserts
   byte-determinism.)
3. **Full UI (optional, not committed)** — a `puppeteer-core` script drives
   `decompile/index.html` in real Chrome: uploads an ELF then a Mach-O, waits for the code
   panel / status, asserts the rendered C and the detected format. Verified passing during
   development; kept out of the committed suite to avoid a browser/`puppeteer` dependency.
   (Headless Chrome alone can do the same without `puppeteer`: copy the page into `dist/`
   with an appended module script that sets `#file`'s `files` from a `DataTransfer` and
   dispatches `change`, then run `--headless --virtual-time-budget=… --dump-dom`.)

Fixtures (all benign, small, reproducible from the committed source via the comment
header): `sample.elf` (x86-64 ELF, rich body — call chain + `for`-loop), `sample_aarch64.o`
(AArch64), `sample_macho.o` (Mach-O x86-64 — a second *format*). **PE** executables were
verified separately against a real PE (152 functions) through the browser lazy path; no
benign PE is committed because this environment has no PE linker.

## 6. Guarantees (why this doesn't break kuna)

- No decompiler-core changes: the wasm target reuses the native code paths verbatim. The
  console tier hosts the **shared** decompile-project core (`kuna_console::project`, moved
  from `kuna-cli` with the CLI's `decompile-all`/`decompile-project` outputs verified
  byte-identical across the move) plus one additive probe (`ConsoleProgram::
  lone_jump_target`) that no native output path calls.
- No new dependency, no new stage-model option, no native output change: `kuna_wasm` == the
  `decompile-all` path (+ the wasm-only `"kind"` field and `project` command), so
  `docs/history.md`, `phases.toml`, and `docs/options.md` are untouched.
- The four gates (`make test`, `make test-stages`, `make rust-test`, `make check-spec`)
  are unaffected — the crate is invisible to all but `rust-test`, where it only compiles.

## 7. Limitations & future work

- **Supports whatever the CLI supports** — every format (ELF/PE/Mach-O/COFF) and every
  architecture kuna ships a `.sla` for, resolved by the engine with no per-format JS (§3).
  Object files (`.o`/Mach-O `MH_OBJECT`) decompile to thin bodies — an engine-level
  relocation limit, not a demo one; linked executables are unaffected.
- **Re-bootstraps per request** — a WASI *command* module runs `_start` and exits, so
  "decompile all" (one run over every function) is the efficient path the UI uses. A
  `wasm-bindgen` **reactor** front-end (bootstrap once, export `decompile(name)`) would let
  the page keep a warm `Architecture` across clicks; it can be added beside this crate
  without touching the WASI path or the engine.
- **No `wasm-opt` in the default toolchain** — the shipped wasm is unoptimized (~7 MB raw,
  ~1.7 MB gzipped); installing `binaryen` shrinks it further.

## 8. Pointers

- Harness & commands: `integrations/web/README.md`
- The crate: `decompiler/crates/kuna-wasm/{Cargo.toml, src/lib.rs, src/main.rs, src/classify.rs}`
- The shared decompile loop + artifact builders: `decompiler/crates/kuna-console/src/project.rs`
  (the CLI wrappers: `decompiler/crates/kuna-cli/src/{decompile_all.rs, decompile_project.rs}`)
- The engine entry it reuses: `kuna_console::engine::bootstrap_from_object`
