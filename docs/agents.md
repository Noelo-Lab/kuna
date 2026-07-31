# AGENTS.md

Guidance for working in this repository. (`CLAUDE.md` and `AGENTS.md` at the repo root are
symlinks to this file, `docs/agents.md`.) This file holds only what every task needs;
everything else is one lookup away via the **doc map** at the bottom.

## What kuna is

kuna is an **agent-first decompiler written in Rust**: a decompilation engine plus a SLEIGH
compiler, organized around an explicit phase model whose decision points are exposed as
per-run, flippable options — the LLM control surface is the product. It started as a Rust
port of Ghidra's decompiler (Apache-2.0 — see `LICENSE` and `NOTICE`) and has since
diverged on its own defaults and features; the origin story lives in `docs/history.md`, and 
is not needed for day-to-day work.

## Layout

| Path | What |
|---|---|
| `decompiler/` | The engine — a cargo workspace. `kuna-decomp` is the decompiler (`src/` is phase-foldered: `p0_knowledge/`…`p9_emit/`, plus `substrate/` and `infra/`); `kuna-analysis` the loader/analyzer tier (ELF markup, strings, DWARF, function discovery); `kuna-sleigh`/`kuna-slacomp` the SLEIGH runtime/compiler (binary `slacomp`); `kuna-console` the `decomp_dbg`/`decomp_test_dbg` binaries; `kuna-cli` the user-facing `kuna` binary; `kuna-ghidra` and `kuna-wasm` the Ghidra and browser front-ends; `kuna-base`/`kuna-num`/`kuna-harness`/`kuna-lift-diff` support. |
| `tests/datatests/` | Vendored XML regression corpus (83 files / 675 assertions) — what `make test` runs. |
| `tests/stages/` | kuna-owned issue testcases — `make test-stages`. Conventions: `tests/stages/README.md`. |
| `tests/golden/` | Differential golden vectors for the workspace suite (`make rust-test`). |
| `specs/Ghidra/Processors/` | Vendored SLEIGH specs. `.sla` files are built artifacts (gitignored), produced by `slacomp`. |
| `scripts/` + `tools/pipeline/` | Python helpers (`decompile.py` library shim, `paths.py`, `pipeline/`, `decbench/`) + driver for the improvement pipeline (`docs/improvement-pipeline.md`) and the decbench campaign (`docs/decbench-loop.md`). |
| `integrations/` | Front-ends embedding the engine: `ghidra/` (kuna as stock Ghidra's decompiler core), `web/` (the project site + in-browser decompiler at `kuna.noelo.org`). |

## Build & test

Only prerequisite: a Rust toolchain. Develop in the workspace directly
(`cd decompiler && cargo build/test ...`); the Makefile is the driver:

```bash
make            # binaries + specs
make binaries   # decomp_dbg / decomp_test_dbg / slacomp / kuna  → decompiler/target/release/
make specs      # compile all .slaspec → .sla with slacomp
```

**Four gates — run all of them before every commit:**

| Gate | Checks | Expect |
|---|---|---|
| `make test` | datatest corpus vs `docs/baseline.json` | **PARITY OK** (675/675) |
| `make test-stages` | stage-issue corpus vs `docs/baseline-stages.json` | **PARITY OK** |
| `make rust-test` | full cargo workspace suite + `docs/options.md` freshness | green |
| `make check-spec` | `docs/spec/` anchors + inline code paths resolve; each phase folder owned by exactly one chapter (`--strict` adds option-mention coverage) | green |

CI runs all four (plus `kuna catalog --check`) on every pull request and every push to
main — `.github/workflows/tests.yml`. Run them locally anyway: the workspace suite is the
long pole in CI, so local failures are found far sooner.

- **Never re-pin `docs/baseline.json` to absorb a regression** — fix the code or make the
  change opt-in. The only sanctioned re-pins are an intentional upstream sync or a
  DIV-recorded default change (`kuna test --save-baseline`; see `docs/history.md`).
  Adding a stage test DOES re-record the stages baseline:
  `kuna test --datatests --datatests-dir tests/stages --save-baseline docs/baseline-stages.json`.
- `docs/options.md` is generated — after touching option metadata:
  `decompiler/target/release/kuna catalog --markdown > docs/options.md`.

## The `kuna` CLI

The user-facing binary (`decompiler/crates/kuna-cli` → `decompiler/target/release/kuna`).
The commands agents use most:

```bash
kuna decompile ./a.out main                        # one function (or an address with --addr)
kuna decompile-all ./a.out --json                  # whole binary in one in-process load
kuna functions ./a.out --json                      # enumerate functions
kuna decompile-project ./a.out                     # export .c/.h/.asm/README project folder
kuna catalog --json                                # discover the settable options
kuna decompile ./a.out main --option NAME VALUE    # flip a decision for this run
kuna test --all --baseline docs/baseline.json      # the parity gate
```

Full reference (flags, JSON schemas, watchdog, project-export artifacts): **`docs/cli.md`**.

## The phase model

The engine is organized as ordered phases **P1–P9** (partition → lift/flow → dataflow →
calls → types → variables → regions → structure → emit) plus an orthogonal **P0
knowledge/configuration plane**; source folders are named after them. Folders are a
taxonomy — the real pass order is `universal_sched` in
`decompiler/crates/kuna-decomp/src/infra/universalaction.rs`. Named decision points inside
phases are **settable assertions/options** (`--option NAME VALUE`, discovered via
`kuna catalog`). One-screen model: `docs/phases.md`; normative algorithms: `docs/spec/`.

## Adding features (the enforced rules)

- **Anything that can change emitted C ships behind a named option** — a `settableTable`
  row in `decompiler/crates/kuna-decomp/phases.toml` (every field populated, including
  `tier` + `symptoms`) plus registration in `src/p0_knowledge/options.rs`; `kuna catalog
  --check` must stay green. Options can take values, not just on/off. New logic goes in a
  `kuna_<slug>.rs` module inside its owning phase folder (canonical template:
  `p2_lift/kuna_loweredswitch.rs`). This is for *features* — behavior that is a judgment
  call, not universally better; a strict bug fix that only corrects wrong output needs no
  flag (when in doubt, gate it).
- **Adding an option also bumps hard-coded catalog counts** — the count tests in
  `kuna_phases/tests.rs`, the `tests/catalog_bytecompat.rs` fixture, and the
  `tests/stages/kuna-catalog.xml` count assertions. Grep for the current total, or
  `make rust-test`/`make test-stages` fail opaquely.
- **Default-ON needs evidence**: only if the flip changes 0/675 datatest assertions and
  passes the speed budget; every default flip gets a DIV row in `docs/history.md`
  (a `transform`-tier flip also updates the option's `phases.toml` row prose).
- **The spec is live**: every new feature or behavior change is described in natural-language
  prose in the owning `docs/spec/` chapter in the same PR — not just an anchor update (each
  phase folder has exactly one owning chapter; find yours via its `Anchors:` header). Run
  `make check-spec`.
- **One PR per feature**, with an end-to-end `tests/stages/` testcase (two-pass: option
  off = the bug, default = the fix) and a measured speed delta. Large/multi-part features
  go through a draft `[PROPOSAL]` PR first.
- Full normative list: `docs/improvement-pipeline.md` → *Standing requirements*.

## Conventions

- kuna ElementIds use the 4000+ range; kuna PcodeOp addlflags bits start at 0x1000.
- Code comments citing `decompiler/cpp/<file>.{cc,hh}` are **upstream Ghidra anchors** —
  the C++ tree kuna was ported from, *not* paths in this repo. The pinned upstream commit
  (`GHIDRA_REV`) and the vendored-tree sync procedure are in `docs/history.md`.
- Minimize comments. We should almost never have comments inline. Comments belong in mostly
  two places: the function header or the file header. The function header ones should be 
  minimal as well. Only comment inline when it is a confusing or complex hack.
- New functionality → new modules; match the surrounding code's conventions (ported files
  name methods after their C++ originals).
- Don't commit build artifacts (`decompiler/target/`, `*.sla`).
- Commit at milestones with descriptive messages.

## Doc map (look up on demand — don't preload)

| Doc | Open it when you need |
|---|---|
| `docs/phases.md` | The phase model on one screen (P0–P9, Band B, feedback edges). |
| `docs/spec/` | How any algorithm/pass actually works (start `00-overview.md`). |
| `docs/options.md` | The generated option catalog (tiers, symptoms, flip guidance). |
| `docs/cli.md` | The full `kuna` CLI reference. |
| `docs/improvement-pipeline.md` | The autonomous improvement pipeline + standing requirements for feature PRs. |
| `docs/decbench-loop.md` | The decbench benchmark / improvement campaign. |
| `docs/modes.md` | `--mode reliable\|aggressive\|fast` option presets. |
| `docs/missing-ghidra-analyses.md` | The `kuna-analysis` tier: the analyzer gap vs Ghidra, pass contract, commit gating. |
| `docs/ghidra-integration.md` | kuna as Ghidra's decompiler core (architecture + wire protocol). |
| `docs/web-integration.md` | The WASM/browser front-end and the project site. |
| `docs/devcontainer.md` | The reproducible build container + cross-arch fixture builds. |
| `docs/release.md` | The MAJOR.MINOR version scheme (`VERSION` file + commit count, `make version`) and the binary release CI. |
| `docs/history.md` | The condensed project history: milestone timeline, the C++→Rust port + its verification, the DIV registry (why a default differs from upstream), vendored-tree provenance (`GHIDRA_REV`) + sync procedure. |
