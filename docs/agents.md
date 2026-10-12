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
| `scripts/repipe/` + `tools/repipe/` | The RE-friction loop (`docs/re-pipeline.md`): Codex Sol-low testers reverse-engineer crackmes with kuna and record where it fails them; Codex Sol-high-or-above builders close those gaps and self-merge. Durable backlog in `docs/re-needs/`; promoted regression probes in `tests/cli/`. |
| `integrations/` | Front-ends embedding the engine: `ghidra/` (kuna as stock Ghidra's decompiler core), `web/` (the project site + in-browser decompiler at `kuna.noelo.org`). |

## Build & test

Only prerequisite: a Rust toolchain. Develop in the workspace directly
(`cd decompiler && cargo build/test ...`); the Makefile is the driver (`make` builds the
binaries into `decompiler/target/release/` and compiles the specs).

**Four gates — run all of them before every commit:**

| Gate | Checks | Expect |
|---|---|---|
| `make test` | datatest corpus vs `docs/baseline.json` | **PARITY OK** (675/675) |
| `make test-stages` | stage-issue corpus vs `docs/baseline-stages.json` | **PARITY OK** |
| `make rust-test` | full cargo workspace suite + `docs/options.md` freshness | green |
| `make check-spec` | `docs/spec/` anchors + inline code paths resolve; each phase folder owned by exactly one chapter (`--strict` adds option-mention coverage) | green |

CI runs all four (plus `kuna catalog --check`) on every push to main —
`.github/workflows/tests.yml`. On a **pull request from a branch in this repo** the
workspace suite is skipped and only the parity gates run; **you are the gate for
`make rust-test` on those PRs**, which is why it is on the list above. To demand it from
CI on a particular PR, add the **`full-ci`** label — that label is itself a trigger, so
the suite starts on the label alone. (It also always runs pre-merge on a fork PR, and via
*Run workflow*.) Run all four locally regardless: the workspace suite is the long pole in
CI, so local failures are found far sooner.

The workspace integration tests require built processor specs. Missing or
unusable specs are failures, not skips; use the worktree symlink procedure below
when reusing another checkout's build. Tests for optional external toolchains may
skip when the tool is absent, but a failed build with an installed tool is an error.

Python tooling unit tests run with `make test-tools` and are also required by CI.
They use temporary state directories, not the live pipeline inventory.

CI also runs `make lint`, treating Clippy warnings as errors in the CLI and the
base and numeric libraries, not the whole workspace or their test targets.

The Rust test profile uses optimization level 1 for real-image tests, with debug
assertions and integer-overflow checks enabled. Ordinary development builds retain
Cargo's unoptimized default.

Most crates build their integration tests into one `integration` binary. Register a
new `tests/x.rs` in that crate's `tests/integration.rs` (`#[path = "x.rs"] mod x;`,
with `use crate::common;`, never `mod common;`). A test that mutates env or other
process-global state, or relaunches itself by test name, gets its own `[[test]]`
instead. `make rust-test` and `make test-tools` enforce this. Focused run:
`cargo test -p PKG --test integration x::`.

- **Never re-pin `docs/baseline.json` to absorb a regression** — fix the code or make the
  change opt-in. The only sanctioned re-pins are an intentional upstream sync or a
  deliberate default change, and the commit message says which (`kuna test --save-baseline`).
  Adding a stage test DOES re-record the stages baseline:
  `kuna test --datatests --datatests-dir tests/stages --save-baseline docs/baseline-stages.json`.
- `docs/options.md` is generated — after touching option metadata:
  `decompiler/target/release/kuna catalog --markdown > docs/options.md`.
- When doing builds and tests on a local system, always be aware of the total RAM available on the machine.
  You should always stay below 90% RAM usage. If you detect it occuring, kill it, then request help
  from the user and if it is ok to allow it.
- A core principle of Kuna is speed. Always check that a new feature on by default does not majorly slow
  down Kuna.

## The `kuna` CLI

The user-facing binary (`decompiler/crates/kuna-cli` → `decompiler/target/release/kuna`).
`kuna docs` prints the embedded manual (cli, options, phases, modes) and `kuna --help` lists
the subcommands. Decisions are flipped per run with `--option NAME VALUE`; discover them with
`kuna catalog --json`.

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
  `tier` + `symptoms`) plus a handler in `src/p0_knowledge/kuna_option_dispatch.rs`; `kuna catalog
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
  passes the speed budget. Record what the default now does in the option's `phases.toml`
  row and its `docs/spec/` chapter — there is no separate registry to update.
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
- **Never `git stash` when other agents may be working in sibling worktrees.** `refs/stash` is
  a single stack shared by every worktree of the repo, so a concurrent `stash pop` can hand
  your uncommitted work to another worktree — this has already happened. To A/B a pre-change
  build, copy the file aside (`cp x.rs /tmp/…`) or build from a second checkout.
- In a worktree, build with `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
  CARGO_PROFILE_TEST_DEBUG=0` and delete `decompiler/target/debug` when the workspace suite
  finishes — the default debug profile costs ~20-30 GB per worktree and has filled this
  machine's disk mid-run. Never `make specs` in a worktree; reuse the main tree's via
  `KUNA_SPECS`/`SLEIGHHOME`.
- **`KUNA_SPECS`/`SLEIGHHOME` do not reach the cargo workspace suite.** Those two env vars
  cover `make test`/`make test-stages` and the CLI, but the `make rust-test` targets resolve
  specs as `<repo>/specs` relative to their own crate, so in a worktree ~22 of them fail with
  "Could not find .sla file" no matter what is exported. Symlink the main tree's built
  `.sla` files into the worktree's `specs/` before running the suite (they are gitignored,
  so the tree stays clean) — do not `make specs`.
- Adding a `tests/stages/*.xml` also bumps a hard-coded corpus file count in
  `decompiler/crates/kuna-base/src/xml.rs` and requires re-recording
  `docs/baseline-stages.json`. Two such PRs in flight WILL conflict on both; resolve the count
  to base + all merged, and re-record the baseline rather than hand-merging it.
- Any time any public thing is created fully automatically, it should start with `[AUTOMATED]`. That goes for PRs, Issues (opening and responses), and most importantly replies or comments to issues/PRs. It should also be in the commit message, but can go outside of the tagline and more inside the extended part.
- **PR bodies** lead with a runnable repro, stay under one screen, and never mention our
  internal process (milestones, review rounds, agents, gate runs). The full rules are in
  `docs/pr-bodies.md`; read it before opening or editing a PR.

## Doc map (look up on demand — don't preload)

| Doc | Open it when you need |
|---|---|
| `docs/phases.md` | The phase model on one screen (P0–P9, Band B, feedback edges). |
| `docs/spec/` | How any algorithm/pass actually works (start `00-overview.md`). |
| `docs/options.md` | The generated option catalog (tiers, symptoms, flip guidance). |
| `docs/cli.md` | The full `kuna` CLI reference. |
| `docs/pr-bodies.md` | How to write a PR body (read before opening or editing any PR). |
| `docs/improvement-pipeline.md` | The autonomous improvement pipeline + standing requirements for feature PRs. |
| `docs/re-pipeline.md` | The RE-friction loop: agents solve crackmes with kuna, record where it fails them, and close those gaps. The second, self-merging lane. |
| `docs/decbench-loop.md` | The decbench benchmark / improvement campaign. |
| `docs/modes.md` | `--mode auto\|reliable\|aggressive\|fast` option presets and size thresholds. |
| `docs/missing-ghidra-analyses.md` | The `kuna-analysis` tier: the analyzer gap vs Ghidra, pass contract, commit gating. |
| `docs/ghidra-integration.md` | kuna as Ghidra's decompiler core (architecture + wire protocol). |
| `docs/web-integration.md` | The WASM/browser front-end and the project site. |
| `docs/devcontainer.md` | The reproducible build container + cross-arch fixture builds. |
| `docs/nightly.md` | The nightly CI: full gates plus a decbench sample (score and speed changes night to night). |
| `docs/release.md` | The MAJOR.MINOR version scheme (`VERSION` file + commit count, `make version`) and the binary release CI. |
| `docs/history.md` | **Rarely needed.** The far past: milestone timeline, the C++→Rust port + its verification, a frozen index for old `DIV-N` citations, vendored-tree provenance (`GHIDRA_REV`) + sync procedure. |
