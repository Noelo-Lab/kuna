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
| `decompiler/` | **The engine.** Cargo workspace: `kuna-base`/`kuna-num`/`kuna-sleigh`/`kuna-decomp` (the decompiler), `kuna-analysis` (the program-prep loader/analyzer tier — ELF markup, strings, DWARF, …; the Ghidra "Run Analysis" layer), `kuna-console` (the `decomp_dbg`/`decomp_test_dbg` binaries), `kuna-slacomp` (the SLEIGH compiler, binary `slacomp`), `kuna-cli` (the user-facing `kuna` binary), `kuna-harness`/`kuna-lift-diff` (dev test harness). `kuna-decomp/src/` is organized into stage-named folders (`s1_partition/`…`s9_emit/`, `substrate/`, `p0_knowledge/`, `infra/`) per `docs/stages.md`. See `docs/RUST_PORT.md`. |
| `tests/datatests/` | Upstream XML regression tests (83 files → 675 assertions). The corpus `make test` runs. Vendored. |
| `tests/stages/` | kuna-owned stage-model issue testcases (`make test-stages`, baseline `docs/baseline-stages.json`). |
| `tests/golden/` | Differential golden vectors for the workspace test suite (`make rust-test`). |
| `specs/Ghidra/Processors/` | Vendored SLEIGH specs (all upstream modules). `.sla` are **built artifacts** (gitignored), produced by `slacomp`. |
| `Makefile` | Build driver (kuna-owned, Rust-only). |
| `scripts/` | Python helpers backing the feature `pipeline/` (out of scope for the engine): `decompile.py` (thin library shim) and `paths.py`. The user-facing CLI is the Rust `kuna` binary, not these. |
| `tools/pipeline/` | Driver + worker for the continuous feature pipeline (`run.sh`, `worker.sh`, `open_pr.sh`, `worker_prompt.md`, `install_gh.sh`). See `docs/pipeline.md`. |
| `tools/sync_upstream.py` | Port upstream Ghidra changes into kuna (specs + datatests only — the C++ source is gone). |
| `docs/RUST_PORT.md` | **The port summary**: what was ported (decompiler + SLEIGH compiler), why, how, and the validation gates. Detailed port history lives under `docs/rust-port/` (ADRs, `losses.md`, `verification.md`, `plan.md`). |
| `docs/stages.md` | The normative stage model (P0 plane, S1–S9, Band B, feedback edges); full model in `docs/stage-model.md`. |
| `docs/stage-mapping.md` | Maps every upstream source module to a stage (P0/S1–S9). The live registry is queryable at the console (`stage list/map/catalog`); the Rust implementation is under `decompiler/crates/kuna-decomp/`. |
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

# Flip a stage-model assertion per decompilation (the LLM control surface)
kuna catalog --json                                            # discover settable assertions
kuna decompile ./a.out main --option compareform canonical
kuna decompile ./sparc.elf main --option returnpair single
```

`kuna test` parses the harness's two streams separately (unit results on **stderr**,
datatest results on **stdout**) and exits nonzero on any failure or baseline regression.
`kuna decompile` drives `decomp_dbg` as a subprocess and captures `print C` via
`openfile write` so interactive prompts never pollute the output; `--option NAME VALUE`
(repeatable) and `--kassert "<args>"` flip stage-model sub-stage assertions per run.
`kuna catalog` is the **discovery half of the LLM control API**: it parses the decompiler's
`stage catalog` JSON (single source of truth: `settableTable`, generated from
`decompiler/crates/kuna-decomp/stages.toml`) into the documented, flippable assertion list —
`--json` for an agent, `--markdown` to regenerate `docs/assertions.md`, `--check` to fail
on catalog/registration drift (CI; cross-checks the catalog against
`kuna_decomp::options::KUNA_OPTION_NAMES` in-process). The full catalog also renders to
`docs/assertions.md`; the model behind it is `docs/stages.md` / `docs/stage-model.md`, and
the defaults are recorded in `docs/divergences.md`.

The still-Python feature pipeline (`scripts/pipeline/`, out of scope for the engine) imports
the thin library shim `scripts/decompile.py::decompile`. There is no installable package —
run it from the repo root (`python -m scripts.pipeline.<mod>`). See `docs/pipeline.md`.

## Tests

Two gates, both Rust:

- `make test` — the datatest parity gate: the `kuna` CLI drives `decomp_test_dbg` over the
  83-file / 675-assertion XML corpus (`tests/datatests/`) and checks it against
  `docs/baseline.json` (expect **PARITY OK**). `make test-stages` does the same over the
  kuna-owned `tests/stages/` corpus against `docs/baseline-stages.json`.
- `make rust-test` — the full cargo workspace suite: the ported unit tests, the golden
  differential vectors, and the SLEIGH-compiler `.sla` content-parity tests.

`docs/baseline.json` is the recorded **kuna oracle**: since DIV-2 (`docs/divergences.md`)
kuna's defaults intentionally diverge from upstream — 22 datatest assertions were re-pinned
in place to kuna's default output (eight stage-model sub-stage fixes on by default; per-option
`option <name> off` restores the upstream rendering).

Any change to the build or the vendored tree must keep `make test` at **PARITY OK** (and
`make rust-test` green) before committing.

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
- Issue-derived stage-model testcases go in `tests/stages/` (`make test-stages`,
  baseline `docs/baseline-stages.json`); see `tests/stages/README.md`.
- Don't commit build artifacts (`decompiler/target/`, `*.sla`) — they're gitignored.
- To understand a source file's role, start from `docs/stages.md` (the stage→folder layout)
  and `docs/stage-mapping.md`, then the real pass order in
  `decompiler/crates/kuna-decomp/src/infra/universalaction.rs` + the `coreaction_*.rs` files
  (now under their stage folders, e.g. `s5_types/coreaction_infertypes.rs`,
  `s9_emit/coreaction_render.rs`). **Code comments cite their C++ origin as `decompiler/cpp/<file>.{cc,hh}`** —
  these are **upstream Ghidra** anchors (the tree kuna was ported from, at the `GHIDRA_REV` in
  `docs/UPSTREAM.md`, recoverable from git history or an upstream checkout), *not* paths in
  this repo's `decompiler/` workspace.
```
