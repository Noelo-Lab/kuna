# CLAUDE.md

Guidance for working in this repository.

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
(`decompiler/datatests/`) remain vendored from upstream (`UPSTREAM.md` has the `GHIDRA_REV`).

## Layout

| Path | What |
|---|---|
| `rust/` | **The engine.** Cargo workspace: `kuna-base`/`kuna-num`/`kuna-sleigh`/`kuna-decomp` (the ported decompiler), `kuna-console` (the `decomp_dbg`/`decomp_test_dbg` binaries), `kuna-slacomp` (the ported SLEIGH compiler, binary `slacomp`). See `docs/RUST_PORT.md`. |
| `decompiler/datatests/` | Upstream XML regression tests (83 files → 675 assertions). The corpus the Rust harness runs. Vendored. |
| `specs/Ghidra/Processors/` | Vendored SLEIGH specs (all upstream modules). `.sla` are **built artifacts** (gitignored), produced by `slacomp`. |
| `Makefile` | Build driver (kuna-owned, Rust-only). |
| `kuna/` | Python helpers: `run_tests.py` (datatest harness), `slacomp.py` (.sla differential), `decompile.py`, `catalog.py`, `paths.py`, and `pipeline/`. (To be ported to Rust — see RUST_PORT follow-up.) |
| `tools/pipeline/` | Driver + worker for the continuous feature pipeline (`run.sh`, `worker.sh`, `worker_prompt.md`, `install_gh.sh`). |
| `tools/sync_upstream.py` | Port upstream Ghidra changes into kuna (now specs/datatests only — the C++ source is removed). |
| `docs/RUST_PORT.md` | **The port summary**: what was ported (decompiler + SLEIGH compiler), why, how, and the validation gates. The C++ tree (`decompiler/cpp/`, `decompiler/unittests/`) was removed once the port was proven; history retained in git. |
| `docs/stages.md` | The normative stage model (P0 plane, S1–S9, Band B, feedback edges); full model in `docs/stage-model.md`. |
| `docs/stage-mapping.md` | Every `.cc` mapped to a stage: §0 = current model (P0/S1–S9, matches the runtime registry `kuna_stages.cc`); legacy 19-stage tables kept for per-file role descriptions. |
| `docs/baseline.json` | Recorded test-pass oracle (parity check) — the **kuna** oracle since DIV-2 (`docs/divergences.md`), no longer pristine-upstream. |

## Build

**The decompiler and the SLEIGH compiler are now fully ported to Rust and the vendored
C++ tree has been removed — see `docs/RUST_PORT.md`.** Everything builds and tests through
the Rust workspace under `rust/`. Prereqs: a Rust toolchain (`cargo`). (No g++/libbfd/bison/
flex — those were only for the removed C++ tree.)

```bash
make            # binaries + specs (all Rust)
make binaries   # cargo build the Rust decomp_dbg/decomp_test_dbg + slacomp
make specs      # compile all .slaspec -> .sla with the Rust SLEIGH compiler (slacomp)
make test       # the 675/675 datatest parity (Rust harness + docs/baseline.json)
make rust-test  # the full cargo workspace suite (ported units + golden + .sla parity)
make clean
```

The decompiler binaries keep the upstream names (`decomp_dbg`, `decomp_test_dbg`); the
SLEIGH compiler is `slacomp` (matches `sleigh_opt`'s CLI: `slacomp <file.slaspec>`,
`-a <dir>` recurses). All under `rust/target/release/`. Work in the cargo workspace
directly (`cd rust && cargo build/test ...`) for development.

## The `kuna` CLI

The user-facing commands are the single Rust binary `kuna` (`rust/crates/kuna-cli`,
built to `rust/target/release/kuna` by `make binaries`) — the Python CLIs
(`kuna/{decompile,run_tests,catalog,slacomp}.py`) were ported to it and removed (see
`docs/RUST_PORT.md` and `docs/rust-port/cli-port.md`). Build it, then:

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
`rust/crates/kuna-decomp/stages.toml`) into the documented, flippable assertion list —
`--json` for an agent, `--markdown` to regenerate `docs/assertions.md`, `--check` to fail
on catalog/registration drift (CI; cross-checks the catalog against
`kuna_decomp::options::KUNA_OPTION_NAMES` in-process). The full catalog also renders to
`docs/assertions.md`; the model behind it is `docs/stages.md` / `docs/stage-model.md`, and
the defaults are recorded in `docs/divergences.md`.

The still-Python `kuna/pipeline/` (the autonomous feature loop, out of scope) imports the
thin library shim `kuna/decompile.py::decompile`; `pip install -e .` still installs the
`kuna` package for it.

## Tests

Scope is the **deep (C++) decompiler only**: 207 unit tests + 83 datatests (675
assertions), all run by the upstream `decomp_test_dbg`. `docs/baseline.json` is the
recorded **kuna oracle**: since DIV-2 (`docs/divergences.md`) kuna's defaults
intentionally diverge from upstream — 22 datatest assertions were re-pinned in place to
kuna's default output (eight stage-model sub-stage fixes on by default; per-option
`option <name> off` restores the upstream rendering). Ghidra's Java-side decompiler
tests (`Ghidra/Features/Decompiler/src/test.slow/java/`) are intentionally
**excluded** — they go through the Java front-end/GUI, not the C++ codebase.

Any change to the build or the vendored tree must keep `python -m kuna.run_tests
--baseline docs/baseline.json` at **PARITY OK** before committing.

## Porting upstream changes (kuna is derived from Ghidra)

kuna tracks a specific Ghidra commit (`GHIDRA_REV` in `UPSTREAM.md`). To pull newer
upstream changes into the vendored code:

```bash
git -C ~/github/ghidra fetch origin
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master --dry-run  # preview
python3 tools/sync_upstream.py --ghidra ~/github/ghidra --to origin/master            # apply
make test && python -m kuna.run_tests --baseline docs/baseline.json                    # re-verify
# then review `git diff` and commit
```

`sync_upstream.py` diffs the Ghidra checkout between `GHIDRA_REV` and the target
(with `--no-renames`, so renames arrive as delete+add and never straddle the vendored
boundary), restricted to the vendored paths, rewrites the path prefixes to kuna's
layout, applies with `git apply` (plain apply, `--3way` fallback), and bumps
`GHIDRA_REV` on success — never during `--dry-run`. It reports added/deleted vendored
files, hard-fails on anything unusual (quoted paths, binary changes, rename/copy lines)
instead of guessing, and warns when a `.y`/`.l` changed without its regenerated `.cc`
(would need bison/flex) or when a new datatest references an unvendored processor. A
manual fallback is documented in `UPSTREAM.md`. **Since the 2026-06 divergence the
apply is no longer guaranteed conflict-free** for the handful of modified vendored
files listed in `UPSTREAM.md` *Divergence* — expect `--3way`/manual resolution there.

When you update the baseline after an intentional upstream behavior change, regenerate
`docs/baseline.json` with `kuna.run_tests --save-baseline` against the new pristine tree.

## Conventions

- Commit at milestones with descriptive messages; keep `PROGRESS.md` current.
- Decompiler code lives in the `rust/` cargo workspace (`kuna-decomp` etc.); the SLEIGH
  compiler in `kuna-slacomp`. New functionality → new modules; match the surrounding code's
  conventions (the ported files name methods after their C++ originals).
- kuna ElementIds use the 4000+ range (upstream max ~290); kuna PcodeOp addlflags
  bits start at 0x1000.
- Issue-derived stage-model testcases go in `tests/stages/` (`make test-stages`,
  baseline `docs/baseline-stages.json`); see `tests/stages/README.md`.
- Don't commit build artifacts (`rust/target/`, `*.sla`) — they're gitignored.
- To understand a source file's role, start from `docs/stage-mapping.md` and the real pass
  order in `rust/crates/kuna-decomp/src/coreaction*.rs` (the `universalAction` registration).
  The original C++ anchors cited throughout the code/docs map to upstream Ghidra at the
  `GHIDRA_REV` in `UPSTREAM.md` (recoverable from git history or an upstream checkout).
