# AGENTS.md

Guidance for working in this repository. (`CLAUDE.md` and `AGENTS.md` at the repo root
are symlinks to this file, `docs/agents.md`.)

## What kuna is

kuna is a **standalone Rust port of Ghidra's decompiler** (the "deep decompiler," built
around SLEIGH) and its **SLEIGH compiler** — extracted from the Ghidra Java application and
ported, line-faithfully, from C++ to Rust so the core pipeline can be studied, instrumented,
and refined stage by stage in a memory-safe, modular engine.

It began as a verbatim extraction of Ghidra's C++ decompiler (**derived from Ghidra**,
https://github.com/NationalSecurityAgency/ghidra, Apache-2.0). That C++ tree was then ported
to Rust and **removed once the port was proven equivalent** (675/675 decompiler datatests +
148/148 SLEIGH specs compiling to byte-identical `.sla`) — see **`docs/RUST_PORT.md`** for the
full what/why/how/validation. The SLEIGH `specs/` and the XML regression corpus
(`tests/datatests/`) remain vendored from upstream (`docs/UPSTREAM.md` has the `GHIDRA_REV`).

## Layout

| Path | What |
|---|---|
| `decompiler/` | **The engine.** Cargo workspace: `kuna-base`/`kuna-num`/`kuna-sleigh`/`kuna-decomp` (the decompiler), `kuna-analysis` (the program-prep loader/analyzer tier — ELF markup, strings, DWARF, …; the Ghidra "Run Analysis" layer), `kuna-console` (the `decomp_dbg`/`decomp_test_dbg` binaries), `kuna-slacomp` (the SLEIGH compiler, binary `slacomp`), `kuna-cli` (the user-facing `kuna` binary), `kuna-ghidra` (the ghidra-mode process front-end — the `kuna_ghidra` binary that speaks Ghidra's decompiler-process protocol; see `docs/ghidra-integration.md` and `integrations/`), `kuna-harness`/`kuna-lift-diff` (dev test harness). `kuna-decomp/src/` is organized into phase-named folders (`p1_partition/`…`p9_emit/`, `substrate/`, `p0_knowledge/`, `infra/`) per `docs/phases.md`. See `docs/RUST_PORT.md`. |
| `tests/datatests/` | Upstream XML regression tests (83 files → 675 assertions). The corpus `make test` runs. Vendored. |
| `tests/stages/` | kuna-owned stage-model issue testcases (`make test-stages`, baseline `docs/baseline-stages.json`). |
| `tests/golden/` | Differential golden vectors for the workspace test suite (`make rust-test`). |
| `specs/Ghidra/Processors/` | Vendored SLEIGH specs (all upstream modules). `.sla` are **built artifacts** (gitignored), produced by `slacomp`. |
| `Makefile` | Build driver (kuna-owned, Rust-only). |
| `scripts/` | Python helpers backing the feature `pipeline/` (out of scope for the engine): `decompile.py` (thin library shim) and `paths.py`. The user-facing CLI is the Rust `kuna` binary, not these. |
| `tools/pipeline/` | Driver + worker for the continuous feature pipeline (`run.sh`, `worker.sh`, `open_pr.sh`, `worker_prompt.md`, `install_gh.sh`). See `docs/pipeline.md`. |
| `tools/sync_upstream.py` | Port upstream Ghidra changes into kuna (specs + datatests only — the C++ source is gone). |
| `integrations/` | The Ghidra extension: a plugin that makes the **stock Ghidra GUI** spawn kuna's `kuna_ghidra` binary as its decompiler core (reflection exepath swap; binary ships in the module's `os/<platform>/`). See `docs/ghidra-integration.md`. |
| `docs/RUST_PORT.md` | **The port summary**: what was ported (decompiler + SLEIGH compiler), why, how, and the validation gates. Detailed port history lives under `docs/rust-port/` (ADRs, `losses.md`, `verification.md`, `plan.md`). |
| `docs/ghidra-integration.md` | Using kuna as Ghidra's decompiler core (Phases 1–4, seams, wire protocol); the core-agnostic interface spec is `docs/decompiler-core-interface.md`. |
| `docs/phases.md` | The normative phase model (P0 knowledge plane, P1-P9, Band B, feedback edges); the original derivation study is archived at `docs/history/stage-model.md`. |
| `docs/history/stage-mapping.md` | Maps every upstream source module to a stage (P0/S1–S9). The live registry is queryable at the console (`phase list/map/catalog`); the Rust implementation is under `decompiler/crates/kuna-decomp/`. |
| `docs/baseline.json` | Recorded test-pass oracle (parity check) — the **kuna** oracle since DIV-2 (`docs/divergences.md`), no longer pristine-upstream. |

## Build

Everything builds and tests through the Rust workspace under `decompiler/`. The only
prerequisite is a **Rust toolchain** (`cargo`).

```bash
make            # binaries + specs
make binaries   # cargo build decomp_dbg/decomp_test_dbg + slacomp + the kuna CLI
make specs      # compile all .slaspec -> .sla with the SLEIGH compiler (slacomp)
make test       # the 675/675 datatest parity (harness + docs/baseline.json)
make rust-test  # the full cargo workspace suite (ported units + golden + .sla parity)
make clean
```

The decompiler binaries keep the upstream names (`decomp_dbg`, `decomp_test_dbg`); the
SLEIGH compiler is `slacomp` (matches `sleigh_opt`'s CLI: `slacomp <file.slaspec>`,
`-a <dir>` recurses). All under `decompiler/target/release/`. Work in the cargo workspace
directly (`cd decompiler && cargo build/test ...`) for development.

## The `kuna` CLI

The user-facing commands are the single Rust binary `kuna` (`decompiler/crates/kuna-cli`,
built to `decompiler/target/release/kuna` by `make binaries`). Build it, then:

```bash
# Run the decompiler test suite with baseline parity checking
kuna test --all --baseline docs/baseline.json                  # expect: PARITY OK
kuna test --datatests --json                                   # machine-readable

# Decompile a function from a binary
kuna decompile ./a.out main
kuna decompile ./stripped.bin 0x401040 --addr

# Decompile a WHOLE binary in one in-process load (load-once, decompile-many)
kuna decompile-all ./a.out --json                              # machine-readable: every function
kuna decompile-all ./a.out --functions main,parse --json       # a subset
kuna functions ./a.out --json                                  # just enumerate (name + address)

# Flip a stage-model assertion per decompilation (the LLM control surface)
kuna catalog --json                                            # discover settable assertions
kuna decompile ./a.out main --option compareform canonical
kuna decompile ./sparc.elf main --option returnpair single
```

`kuna test` parses the harness's two streams separately (unit results on **stderr**,
datatest results on **stdout**) and exits nonzero on any failure or baseline regression.
`kuna decompile` drives `decomp_dbg` as a subprocess and captures `print C` via
`openfile write` so interactive prompts never pollute the output; `--option NAME VALUE`
(repeatable) and `--kassert "<args>"` flip phase-model sub-phase assertions per run.
`kuna decompile-all` / `kuna functions` are the **whole-binary, machine-readable** surface
(the benchmark + LLM path): they run *in-process* (`kuna_console::engine::bootstrap_from_object`
→ `commit_pending_analysis` → loop `decompile_func` + `print_c`), loading + analyzing the
binary **once** instead of `kuna decompile`'s subprocess-per-function (≈10×+ faster on a
many-function binary). `--json` emits `{binary,count,functions:[{name,address,size,code,error,
variables:[{name,type,kind,arg_index,stack_offset,size}]}]}` — per-function `code` is
byte-identical to `kuna decompile ... --option listing on`, `error` isolates a single failed
function, and `variables` (params in ABI order + DWARF/stack locals) feed type-recovery
scoring. **`decompile-all` builds the Listing analysis tier by default** (DIV-15, decbench F1):
it injects `option listing on` unless the caller names `listing`, so the default-on
`noreturn_propagate` call-graph fixpoint fires and a stripped binary's unnamed exit/fatal
wrappers no longer swallow the functions after them (`--option listing off` opts out;
`kuna functions` and the `kuna decompile`/console path keep listing off). `decompile-all` also
carries a **per-function watchdog**, `--max-fn-seconds N` (default 120, `0` disables): a
function whose decompile drive exceeds the budget is cut off cooperatively (deadline probes
at the action/rule-pool/heritage loop boundaries) and recorded as that function's `error`
("per-function decompile budget exceeded"), the batch continuing — driver policy, not a
stage-model settable (zero output change for any function that converges; the console /
`decomp_dbg` parity path never arms it). The decbench backend
(`decbench/decompilers/raw/kuna_raw.py`) shells out to `kuna decompile-all --json`.
`kuna catalog` is the **discovery half of the LLM control API**: it parses the decompiler's
`phase catalog` JSON (single source of truth: `settableTable`, generated from
`decompiler/crates/kuna-decomp/phases.toml`) into the documented, flippable assertion list —
`--json` for an agent, `--markdown` to regenerate `docs/options.md` (tier-grouped,
symptom-indexed; `--tier transform` filters to the control surface), `--check` to fail
on catalog/registration drift (CI; cross-checks the catalog against
`kuna_decomp::options::KUNA_OPTION_NAMES` in-process). The full catalog also renders to
`docs/options.md`; the model behind it is `docs/phases.md` / `docs/spec/`, and
the defaults are recorded in `docs/divergences.md`.

The still-Python feature pipeline (`scripts/pipeline/`, out of scope for the engine) imports
the thin library shim `scripts/decompile.py::decompile`. There is no installable package —
run it from the repo root (`python -m scripts.pipeline.<mod>`). See `docs/pipeline.md`. It
obeys five **standing requirements** (one PR/feature; end-to-end binary→addr/func testcase;
output-changing ⇒ logged + `--option`-flaggable with a `tier` + `symptoms`
assignment in `phases.toml` (a default-on flip of a `transform`-tier option needs a
DIV entry AND the row prose updated); always measure+record decompile speed;
large/multi-part features go through a `[PROPOSAL]` draft PR for human go/no-go) — see
`docs/pipeline.md` → *Standing requirements*.

## Known issues

- **Fixpoint non-convergence on certain fully-stripped x86-64 ELF binaries**
  (FIXED). The `loweredswitch` repair hook (`kuna_repair_lowered_switch_inputs`)
  mis-classified the constant `ActionConditionalConst` legitimately installed on a
  synthetic BRANCHIND as a broken input and re-pointed it at the register def on
  every heritage pass — a condconst↔repair tug-of-war that kept the repeatapply
  `mainloop` reporting one change forever (infinite loop, 100% CPU). Fixed by
  accepting heritage-known Varnodes (C++ `Varnode::isHeritageKnown()`:
  insert|constant|annotation) as healthy. The full known-hang set (9 stripped
  openssh binaries + bash `-O2`) now completes under the default `decompile-all`
  watchdog (`--max-fn-seconds`, default 120 s), which remains as a defensive
  bound. Past repro: `kuna decompile-all tests/hang-repro/ssh-sk-helper --addr
  0x1bd04` (now converges in well under a second); binaries + original writeup:
  **`tests/hang-repro/README.md`**.

## Tests

Three gates, all Rust:

- `make test` — the datatest parity gate: the `kuna` CLI drives `decomp_test_dbg` over the
  83-file / 675-assertion XML corpus (`tests/datatests/`) and checks it against
  `docs/baseline.json` (expect **PARITY OK**).
- `make test-stages` — the kuna-owned stage-model corpus (`tests/stages/`) against
  `docs/baseline-stages.json`. **Run this too on every change** (expect **PARITY OK**,
  158/158 since #9). Any failure means a regression — confirm against `main` if unsure.
- `make rust-test` — the full cargo workspace suite: the ported unit tests, the golden
  differential vectors, and the SLEIGH-compiler `.sla` content-parity tests.

`docs/baseline.json` is the recorded **kuna oracle**: since DIV-2 (`docs/divergences.md`)
kuna's defaults intentionally diverge from upstream — 22 datatest assertions were re-pinned
in place to kuna's default output (eight phase-model sub-phase fixes on by default; per-option
`option <name> off` restores the upstream rendering).

Any change to the build or the vendored tree must keep `make test` at **PARITY OK**,
`make rust-test` green, and `make test-stages` with **no new failures beyond the 2 known**
(above) — run all three before committing.

## Porting upstream changes (kuna is derived from Ghidra)

kuna tracks a specific Ghidra commit (`GHIDRA_REV` in `docs/UPSTREAM.md`). Only the SLEIGH
`specs/` and the XML `tests/datatests/` corpus are vendored now (the C++ source is gone). To
pull newer upstream changes into the vendored code:

```bash
git -C ~/github/ghidra fetch origin
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master --dry-run  # preview
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master            # apply
make test                                                                              # re-verify
# then review `git diff` and commit
```

`sync_upstream.py` diffs the Ghidra checkout between `GHIDRA_REV` and the target
(with `--no-renames`, so renames arrive as delete+add and never straddle the vendored
boundary), restricted to the vendored paths, rewrites the path prefixes to kuna's
layout, applies with `git apply` (plain apply, `--3way` fallback), and bumps
`GHIDRA_REV` on success — never during `--dry-run`. It reports added/deleted vendored
files, hard-fails on anything unusual (quoted paths, binary changes, rename/copy lines)
instead of guessing, and warns when a new datatest references an unvendored processor. A
manual fallback is documented in `docs/UPSTREAM.md`. **Since the 2026-06 divergence the
apply is no longer guaranteed conflict-free** for the handful of modified vendored
files listed in `docs/UPSTREAM.md` *Divergence* — expect `--3way`/manual resolution there.

When you update the baseline after an intentional upstream behavior change, regenerate
`docs/baseline.json` with `kuna test --save-baseline` against the new tree.

## Conventions

- Commit at milestones with descriptive messages; keep `docs/PROGRESS.md` current.
- Decompiler code lives in the `decompiler/` cargo workspace (`kuna-decomp` etc.); the SLEIGH
  compiler in `kuna-slacomp`. New functionality → new modules; match the surrounding code's
  conventions (the ported files name methods after their C++ originals).
- kuna ElementIds use the 4000+ range (upstream max ~290); kuna PcodeOp addlflags
  bits start at 0x1000.
- Issue-derived phase-model testcases go in `tests/stages/` (`make test-stages`,
  baseline `docs/baseline-stages.json`); see `tests/stages/README.md`.
- Don't commit build artifacts (`decompiler/target/`, `*.sla`) — they're gitignored.
- To understand a source file's role, start from `docs/phases.md` (the stage→folder layout)
  and `docs/history/stage-mapping.md`, then the real pass order in
  `decompiler/crates/kuna-decomp/src/infra/universalaction.rs` + the `coreaction_*.rs` files
  (now under their phase folders, e.g. `p5_types/coreaction_infertypes.rs`,
  `p9_emit/coreaction_render.rs`). **Code comments cite their C++ origin as `decompiler/cpp/<file>.{cc,hh}`** —
  these are **upstream Ghidra** anchors (the tree kuna was ported from, at the `GHIDRA_REV` in
  `docs/UPSTREAM.md`, recoverable from git history or an upstream checkout), *not* paths in
  this repo's `decompiler/` workspace.
```
