# The kuna feature pipeline (angr-inspired, continuous, LLM-augmentable)

This is the normative description of `kuna.pipeline` + `tools/pipeline/`: a continuous,
observable loop that turns "study a better decompiler, port one improvement at a time"
(the methodology behind the `loweredswitch` feature) into an automated, reviewable
feature factory.

Each iteration:

1. **picks** the highest-value angr testcase where angr's decompilation beats kuna's,
2. **reproduces** the gap (angr vs kuna on the same function),
3. **implements ONE** option-gated kuna feature that closes it, modelled on the
   `loweredswitch` recipe,
4. **verifies** it (catalog drift, full parity, stage tests; ablation decides default-ON
   vs opt-in),
5. **opens a PR** with a complete, self-documenting per-feature bundle, preserving the
   Claude session so a human reviewer can resume it.

Nothing ever lands on `main` without a human-reviewed PR.

## The four decisions baked in (see `docs/.../plans` / commit history)

- **Drive**: headless `claude -p` worker, one feature per git-worktree-isolated session.
- **angr runtime**: installed **9.2.213** (`~/.virtualenvs/decbench`), version pinned into
  every feature record; re-point with `KUNA_PIPELINE_ANGR_PYTHON` (e.g. a future Python-3.12
  venv running the exact checkout, 9.2.222).
- **Feature policy**: ship default-ON only if the full ablation changes **0/675** upstream
  assertions, else default-OFF opt-in. The loop never re-pins `docs/baseline.json`.
- **PRs**: `gh` (installed no-root via `tools/pipeline/install_gh.sh`), on `feat/angr-<slug>`.

## Components

| Piece | What |
|---|---|
| `kuna/pipeline/reference/` | The "better decompiler" seam. `ReferenceDecompiler` protocol + `AngrReference` (drives `_angr_runner.py` under the angr venv). Add reko/BinaryNinja/IDA later as a new adapter — everything downstream is decompiler-agnostic. |
| `kuna/pipeline/worklist.py` | AST-parses angr's `test_decompiler.py` into `(binary, function)` targets (captures `arch`/`load_debug_info`/custom-options). → `docs/pipeline/worklist.json`. |
| `kuna/pipeline/compare.py` | Reference vs kuna on the SAME function; structural metrics (gotos/labels/switch/loops/loc) + "where the reference is better" signals. Structural, never raw-text (kuna emits `sub_<addr>` / different names). |
| `kuna/pipeline/sweep.py` + `rank.py` | Run `compare` across the corpus (concurrent), rank the gaps. → `docs/pipeline/opportunities.json` + `docs/pipeline/matrix.md`. |
| `kuna/pipeline/select.py` | Pick the next unclaimed, highest-score opportunity for a worker. |
| `kuna/pipeline/select_port.py` | Port-mode selector: next eligible `docs/rust-port/checklist.json` item (see "Port mode"). |
| `kuna/pipeline/state.py` | flock-guarded worker inventory + opportunity claims (dedup) + heartbeats. Lives in `.kuna-pipeline/` (gitignored). |
| `kuna/pipeline/status.py` | Live observability: worker table + count, worktrees, open PRs. |
| `tools/pipeline/worker_prompt.md` | The templated one-feature iteration prompt (the definition-of-done). |
| `tools/pipeline/worker_port_prompt.md` | Port-mode PORTER prompt: transcribe one checklist item's C++ modules into Rust. |
| `tools/pipeline/verifier_port_prompt.md` | Port-mode VERIFIER prompt: independent verdict per `docs/rust-port/verification.md`. |
| `tools/pipeline/worker.sh` | Launch ONE worker: worktree + initial build + headless `claude -p` (highest-effort model). |
| `tools/pipeline/run.sh` | The driver loop: keep N workers running until the backlog/time budget is exhausted; GC merged worktrees. `PIPELINE_MODE=port` switches the backlog (see "Port mode"). |
| `tools/pipeline/install_gh.sh` | One-time no-root `gh` install + auth from the existing credential. |

## Run it

```bash
# one-time: PR tooling
tools/pipeline/install_gh.sh

# refresh the backlog (after kuna changes, or to widen scope)
python -m kuna.pipeline.worklist --only-existing            # docs/pipeline/worklist.json
python -m kuna.pipeline.sweep --arch x86 --jobs 8           # docs/pipeline/{opportunities.json,matrix.md}

# do exactly one feature (the proof) ...
PIPELINE_WORKERS=1 tools/pipeline/run.sh --once
# ... or run continuously for a working day with two workers:
PIPELINE_WORKERS=2 PIPELINE_HOURS=8 tools/pipeline/run.sh

# observe (separate terminal)
python -m kuna.pipeline.status --watch

# stop gracefully (in-flight workers finish):
touch .kuna-pipeline/STOP
```

## Port mode (PIPELINE_MODE=port): the Rust-port backlog

The same driver/worker machinery, pointed at a different backlog: instead of angr-gap
opportunities, workers consume the Rust-port checklist (`docs/rust-port/checklist.json`)
and continue the port unattended. Everything else — worker counts, heartbeats, GC, STOP
file, time budget, `status --watch` — is shared. Default mode is untouched.

```bash
# continuous porting+verification with N workers for H hours
PIPELINE_MODE=port PIPELINE_WORKERS=2 PIPELINE_HOURS=8 tools/pipeline/run.sh

# smoke: do exactly one checklist item, then stop
PIPELINE_MODE=port PIPELINE_WORKERS=1 tools/pipeline/run.sh --once

# what the loop would pick next
python -m kuna.pipeline.select_port            # (--shell / --json)
```

How it differs from feature mode, piece by piece:

- **Selection** (`kuna.pipeline.select_port`): next eligible checklist item =
  `status: todo`, `attempts < 3`, unclaimed, all `depends_on` satisfied (dep status
  `landed`/`verified`/`done` in the checklist, OR the dep is recorded done in the state
  dir — i.e. its worker opened a PR that hasn't merged yet). `kind: infra` items (wave
  gates, scaffolding) are **never auto-selected** — they are orchestrator-only.
  Preference: `verify` before `port` (verification keeps pace), then lowest wave, then
  smallest `loc_estimate`. Claims/done are keyed by the checklist item id.
- **Branches**: workers branch `rport/<item-id>` off `rust-port` (`BASE_BRANCH` default
  in port mode). No C++ build happens in the worktree (the C++ tree is unchanged on
  `rust-port`); `KUNA_DECOMP_DBG`/`KUNA_DECOMP_TEST` point at the MAIN tree's binaries,
  and the build smoke is `cargo fetch && cargo build -p <crate>`.
- **Porter workers** (`kind: port`, prompt `tools/pipeline/worker_port_prompt.md`):
  faithful transcription of the item's C++ `modules` into the pre-seeded module slots of
  its crate, under the ADR rules (ordered containers, wrapping integer helpers,
  `Result<KunaError>`, exact `TEST(name)` parity); gates `cargo build/test/clippy -p
  <crate>` + the item's own `gate`; losses proposed in the PR body's `## Losses` section
  (never by editing `losses.md`); commits `rport/<item-id>: ...`; PR via `open_pr.sh`
  against `rust-port`.
- **Verifier workers** (`kind: verify`, prompt `tools/pipeline/verifier_port_prompt.md`):
  implement `docs/rust-port/verification.md` independently of the porter (no transcripts,
  no PR body). A verify item is only eligible once the port branch `rport/<port-item-id>`
  exists; the worker's worktree is created on its own branch `rport/<verify-item-id>`
  **from** that port branch, so the ported code is present and the >= 3 adversarial tests
  (`rust/crates/<crate>/tests/verify_<port_item>.rs`) plus the verdict file
  (`docs/rust-port/reviews/<port-item-id>.md`, its own single-file commit) apply cleanly
  on top. **The verify PR targets the PORT branch** (not `rust-port`), so its diff shows
  only tests+verdict; when the port PR merges and its branch is deleted, GitHub retargets
  the verify PR onto `rust-port` automatically. (Targeting `rust-port` directly would
  duplicate the port commits in two PRs.)
- **Status flow**: worker claims item (state dir) → opens PR → records `done` with the
  PR URL. The pipeline **never edits checklist statuses**; the orchestrator/human merges
  PRs and flips `status` to `landed` (port merged) / `verified` (verdict accepted)
  centrally, and owns `attempts` bumps on REJECT, `status.md`, `losses.md` transfers, and
  all `infra` items (the `W<k>-gate` rows). Until a dep's status is flipped, the state
  dir's `done` record is what lets the next item proceed within one continuous run.

## Inspect one gap by hand

```bash
python -m kuna.pipeline.compare --entry test_decompiling_dir_gcc_O0_main   # angr vs kuna + metrics + signals
python -m kuna.pipeline.select --json                                       # what the loop would pick next
```

## The LLM-augmentability contract (PHASES)

Every feature the loop ships MUST register a fully self-describing `settableTable` row in
`decompiler/cpp/kuna_stages.cc`, so the next agent can discover and choose it from
`python -m kuna.catalog --json` without reading C++. Beyond the original fields
(`summary`=WHAT, `use_when`=WHEN, `values`+`example`=HOW, `stage`/`substage`, `strength`,
`destructive`), pipeline features carry provenance:

- `source_decompiler` — where the behavior was observed/ported (`"angr"`, `"ghidra-upstream"`, ...).
- `inspiration` — the concrete origin (the angr test method + pass/class + the function).
- `change_kind` — `correctness-fix` | `presentation-default` | `structure-recovery` |
  `opt-in-tool`. This is the field that gates default-ON eligibility (an LLM reads it to
  judge whether flipping is cosmetic, structural, or semantic).

The drift gate (`python -m kuna.catalog --check`, `tests/stages/kuna-catalog.xml`) enforces
that every registered kuna option appears exactly once in the catalog.

## Per-feature review bundle (committed to the PR branch)

`docs/features/<slug>/` holds everything a reviewer (or a resuming Claude session) needs:

- `analysis.md` — why angr is better here, the exact construct, the owning stage.
- `angr-vs-kuna.txt` — the side-by-side the decision rests on.
- `plan.md` — the design.
- `record.json` — machine record: testcase, angr version, option/flag/ElementId,
  `change_kind`, `inspiration`, ablation result, parity note, decider rationale, and the
  resumable Claude `session_id`.

## Adding another reference decompiler

Implement `ReferenceDecompiler` (see `reference/base.py`) for the new tool, register it in
`reference/__init__.py::REGISTRY`, and (if it has its own testcase corpus) add a worklist
source. `compare`/`sweep`/`worker` need no changes — they operate on `ReferenceResult`.
Pass `--reference <name>` through `compare`.

## Cross-engine comparison: the Rust engine in the pipeline (W11/M4)

The kuna **Rust** port (`KUNA_ENGINE=rust`, the W11 `object`-crate ELF `LoadImage`) runs
through the *same* comparison machinery via two extra knobs, so a comparison row for a
real function can be `kuna-rust vs kuna-cpp` (a pure cross-port differential) or
`kuna-rust vs angr` — no special-casing in `compare`/`sweep`/`rank`:

- `--reference kuna-cpp` / `--reference kuna-rust` — a **kuna engine as a reference**
  (`reference/kuna_adapter.py`, registered as `kuna-cpp`/`kuna-rust`). Runs in the kuna
  venv (no second interpreter, unlike angr); pins the engine tag + resolved `decomp_dbg`
  path as its "version".
- `--kuna-engine {cpp,rust}` — which kuna port is the **decompiler-under-test** (default:
  `KUNA_ENGINE`, else `cpp`). Threaded into `kuna.decompile.decompile(engine=...)`, which
  resolves that engine's binary without mutating `KUNA_ENGINE` (concurrency-safe).

```bash
# kuna-rust vs kuna-cpp on a real ELF (build it first; cpp via the engine-specific override)
printf 'int g; int f(int a){g=a;return a*2+1;} int main(){return f(7);}' > /tmp/p.c
gcc -O0 /tmp/p.c -o /tmp/p.elf
KUNA_DECOMP_DBG_CPP=/path/to/cpp/decomp_dbg \
  python -m kuna.pipeline.compare /tmp/p.elf f --reference kuna-cpp --kuna-engine rust
# kuna-rust vs angr (three-way reachable when the angr venv is present)
python -m kuna.pipeline.compare /tmp/p.elf f --reference angr --kuna-engine rust
```

Because a single bare `KUNA_DECOMP_DBG`/`KUNA_DECOMP_TEST` can only name **one** binary,
the cross-engine path reads **engine-specific** overrides — `KUNA_DECOMP_DBG_CPP` /
`KUNA_DECOMP_DBG_RUST` (and `…_TEST_…`) — so each engine can point at its own out-of-tree
build (e.g. the C++ engine in the main tree, the Rust engine built in the worktree). The
bare override still works for the single-engine `KUNA_ENGINE=…` workflow.

## Known caveats

- **Version skew**: angr 9.2.213 ≠ the checkout's 9.2.222; pinned per feature for
  reproducibility. Decompiler output is version-sensitive — re-point `KUNA_PIPELINE_ANGR_PYTHON`
  for exact fidelity once Python 3.12 is available.
- **Signals are a prefilter, not truth**: ranking is structural/heuristic; the worker (and
  its decider subagent) confirm whether angr is *actually* better and what ONE feature
  closes the gap. `recovery-failure` / huge-loc-delta rows can be kuna failures on large
  functions — vet before picking.
- **Scope**: the default sweep is x86 (best SLEIGH-spec overlap, fastest). Other arches run
  via `--arch <family>` but may have more kuna-side load/spec gaps.
