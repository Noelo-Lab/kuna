# CLAUDE.md

Guidance for working in this repository.

## What kuna is

kuna is a **standalone extraction of Ghidra's C++ decompiler** (the "deep decompiler,"
built around SLEIGH) lifted out of the Ghidra Java application so the core
decompilation pipeline can be studied, instrumented, and refined stage by stage.

It is **derived from Ghidra** (https://github.com/NationalSecurityAgency/ghidra),
Apache-2.0 licensed. The C++ source under `decompiler/` and the SLEIGH specs under
`specs/` are **verbatim copies of upstream Ghidra** — see `UPSTREAM.md` for the exact
commit (`GHIDRA_REV`) and path map. Upstream changes can be pulled in later; see
**Porting upstream changes** below.

## Layout

| Path | What |
|---|---|
| `decompiler/cpp/` | Upstream C++ decompiler source + upstream Makefile. **Diverged from upstream as of 2026-06 (GH-558 prototype)** — kuna additions live in `kuna_*.cc/.hh` (auto-linked); anchor edits to upstream files are minimal and tracked in `UPSTREAM.md` *Divergence*. |
| `decompiler/unittests/` | Upstream C++ unit tests + kuna's `testkunaregion.cc` (207 tests). Vendored. |
| `decompiler/datatests/` | Upstream XML regression tests (83 files → 675 assertions). Vendored. |
| `specs/Ghidra/Processors/` | Vendored SLEIGH specs (all upstream modules). `.sla` are **built artifacts** (gitignored). |
| `Makefile` | Build driver (kuna-owned). |
| `kuna/` | Python package: `decompile.py`, `run_tests.py`, `catalog.py`, `paths.py`, and `pipeline/` (the angr-inspired feature loop — see `docs/pipeline.md`). |
| `tools/pipeline/` | Driver + worker for the continuous feature pipeline (`run.sh`, `worker.sh`, `worker_prompt.md`, `install_gh.sh`). |
| `tools/sync_upstream.py` | Port upstream Ghidra changes into kuna. |
| `tools/fetch_bfd.sh` | Fetch libbfd without root (see Build). |
| `docs/stages.md` | The normative stage model (P0 plane, S1–S9, Band B, feedback edges); full model in `docs/stage-model.md`. |
| `docs/stage-mapping.md` | Every `.cc` mapped to a stage: §0 = current model (P0/S1–S9, matches the runtime registry `kuna_stages.cc`); legacy 19-stage tables kept for per-file role descriptions. |
| `docs/baseline.json` | Recorded test-pass oracle (parity check) — the **kuna** oracle since DIV-2 (`docs/divergences.md`), no longer pristine-upstream. |

## Build

Prereqs: `g++` (C++11), `make`, `zlib1g-dev`, and **`binutils-dev`** (libbfd — required to
link `decomp_dbg`/`decomp_opt`/`decomp_test_dbg`). `bison`/`flex` are only needed if you
edit a `.y`/`.l` grammar (the generated outputs are committed upstream).

```bash
make            # binaries + specs
make binaries   # decomp_opt, decomp_dbg, sleigh_opt, decomp_test_dbg
make specs      # compile all .slaspec -> .sla with the freshly built sleigh compiler
make test       # run the upstream harness directly (unit tests + datatests)
make clean
```

**No root for libbfd?** `sleigh_opt` and all `.sla` build without it; only the
console/test binaries need it. Fetch it locally and inject via `BFD_PREFIX` (this routes
through the upstream Makefile's own `ADDITIONAL_FLAGS`/`BFDLIB` knobs — it does **not**
edit upstream):

```bash
./tools/fetch_bfd.sh                      # -> ./.bfdlocal (gitignored)
make BFD_PREFIX="$(pwd)/.bfdlocal" all
```

Build gotcha (already handled by the Makefile's `touch-generated`): the committed
bison/flex outputs must stay newer than their `.y`/`.l` sources or `make` will try to
regenerate them. Always build through the top-level `Makefile`, not by calling the
upstream Makefile directly. Each upstream binary must be built in its **own** `make`
invocation (the upstream Makefile keys dependency selection on a single `MAKECMDGOALS`).

## Python tooling

Install editable into the project venv (`~/.virtualenvs/kuna`): `pip install -e .`

```bash
# Run the decompiler test suite with baseline parity checking
python -m kuna.run_tests --all --baseline docs/baseline.json   # expect: PARITY OK

# Decompile a function from a binary
python -m kuna.decompile ./a.out main
python -m kuna.decompile ./stripped.bin 0x401040 --addr

# Flip a stage-model assertion per decompilation (the LLM control surface)
python -m kuna.catalog --json                                  # discover settable assertions
python -m kuna.decompile ./a.out main --option compareform canonical
python -m kuna.decompile ./sparc.elf main --option returnpair single
```

`run_tests` parses the harness's two streams separately (unit results on **stderr**,
datatest results on **stdout**) and exits nonzero on any failure or baseline regression.
`decompile` drives `decomp_dbg` as a subprocess and captures `print C` via
`openfile write` so interactive prompts never pollute the output; `--option NAME VALUE`
(repeatable) and `--kassert "<args>"` flip stage-model sub-stage assertions per run.
`catalog` is the **discovery half of the LLM control API**: it parses the decompiler's
`stage catalog` JSON (single source of truth: `settableTable` in `kuna_stages.cc`) into
the documented, flippable assertion list — `--json` for an agent, `--markdown` to
regenerate `docs/assertions.md`, `--check` to fail on catalog/registration drift (CI).
The full catalog also renders to `docs/assertions.md`; the model behind it is
`docs/stages.md` / `docs/stage-model.md`, and the defaults are recorded in
`docs/divergences.md`.

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
- New functionality → new files (`decompiler/cpp/kuna_*.cc/.hh` for decompiler code —
  the upstream Makefile's `$(wildcard *.cc)` links them automatically). Edits to
  upstream files are allowed when an anchor demands it, but keep them minimal, mark
  them with a `(kuna)` comment, and record them in `UPSTREAM.md` *Divergence*.
- kuna ElementIds use the 4000+ range (upstream max ~290); kuna PcodeOp addlflags
  bits start at 0x1000.
- Issue-derived stage-model testcases go in `tests/stages/` (`make test-stages`,
  baseline `docs/baseline-stages.json`); see `tests/stages/README.md`.
- Don't commit build artifacts (binaries, `*.o`, `*.sla`, `.bfdlocal/`) — they're gitignored.
- To understand a source file's role, start from `docs/stage-mapping.md` and the real pass
  order in `decompiler/cpp/coreaction.cc` (`ActionDatabase::universalAction`).
