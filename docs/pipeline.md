# The kuna feature pipeline (angr-inspired, continuous, LLM-augmentable)

This is the normative description of `scripts/pipeline` + `tools/pipeline/`: a continuous,
observable loop that turns "study a better decompiler, port one improvement at a time"
(the methodology behind the `loweredswitch` feature) into an automated, reviewable
feature factory for the **Rust** kuna engine.

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

## The four decisions baked in

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
| `scripts/pipeline/reference/` | The "better decompiler" seam. `ReferenceDecompiler` protocol + `AngrReference` (drives `_angr_runner.py` under the angr venv) + `KunaReference` (kuna itself, registry key `kuna`). Add reko/BinaryNinja/IDA later as a new adapter — everything downstream is decompiler-agnostic. |
| `scripts/pipeline/worklist.py` | AST-parses angr's `test_decompiler.py` into `(binary, function)` targets (captures `arch`/`load_debug_info`/custom-options). → `docs/pipeline/worklist.json`. |
| `scripts/pipeline/compare.py` | Reference vs kuna on the SAME function; structural metrics (gotos/labels/switch/loops/loc) + "where the reference is better" signals. Structural, never raw-text (kuna emits `sub_<addr>` / different names). |
| `scripts/pipeline/sweep.py` + `rank.py` | Run `compare` across the corpus (concurrent), rank the gaps. → `docs/pipeline/opportunities.json` + `docs/pipeline/matrix.md`. |
| `scripts/pipeline/select.py` | Pick the next unclaimed, highest-score opportunity for a worker. |
| `scripts/pipeline/state.py` | flock-guarded worker inventory + opportunity claims (dedup) + heartbeats. Lives in `.kuna-pipeline/` (gitignored). |
| `scripts/pipeline/status.py` | Live observability: worker table + count, worktrees, open PRs. |
| `tools/pipeline/worker_prompt.md` | The templated one-feature iteration prompt (the definition-of-done). |
| `tools/pipeline/worker.sh` | Launch ONE worker: worktree + initial build + headless `claude -p` (highest-effort model). |
| `tools/pipeline/run.sh` | The driver loop: keep N workers running until the backlog/time budget is exhausted; GC merged worktrees. |
| `tools/pipeline/open_pr.sh` | Push the branch and open the PR (with a generated before/after demo). |
| `tools/pipeline/install_gh.sh` | One-time no-root `gh` install + auth from the existing credential. |

The Python pipeline is not an installed package — run it from the repo root so the
`scripts` package is importable (`python -m scripts.pipeline.<mod>`).

## Run it

```bash
# one-time: PR tooling
tools/pipeline/install_gh.sh

# refresh the backlog (after kuna changes, or to widen scope)
python -m scripts.pipeline.worklist --only-existing            # docs/pipeline/worklist.json
python -m scripts.pipeline.sweep --arch x86 --jobs 8           # docs/pipeline/{opportunities.json,matrix.md}

# do exactly one feature (the proof) ...
PIPELINE_WORKERS=1 tools/pipeline/run.sh --once
# ... or run continuously for a working day with two workers:
PIPELINE_WORKERS=2 PIPELINE_HOURS=8 tools/pipeline/run.sh

# observe (separate terminal)
python -m scripts.pipeline.status --watch

# stop gracefully (in-flight workers finish):
touch .kuna-pipeline/STOP
```

## Inspect one gap by hand

```bash
python -m scripts.pipeline.compare --entry test_decompiling_dir_gcc_O0_main   # angr vs kuna + metrics + signals
python -m scripts.pipeline.select --json                                       # what the loop would pick next
```

## The LLM-augmentability contract (PHASES)

Every feature the loop ships MUST register a fully self-describing `settableTable` row in
`decompiler/crates/kuna-decomp/stages.toml`, so the next agent can discover and choose it
from `kuna catalog --json` without reading the engine source. Beyond the original fields
(`summary`=WHAT, `use_when`=WHEN, `values`+`example`=HOW, `stage`/`substage`, `strength`,
`destructive`), pipeline features carry provenance:

- `source_decompiler` — where the behavior was observed/ported (`"angr"`, `"ghidra-upstream"`, ...).
- `inspiration` — the concrete origin (the angr test method + pass/class + the function).
- `change_kind` — `correctness-fix` | `presentation-default` | `structure-recovery` |
  `opt-in-tool`. This is the field that gates default-ON eligibility (an LLM reads it to
  judge whether flipping is cosmetic, structural, or semantic).

The drift gate (`kuna catalog --check`, `tests/stages/kuna-catalog.xml`) enforces
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
