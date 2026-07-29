# The kuna improvement pipeline

**The task**: find a case where kuna's decompilation is worse than a reference (angr, the
decbench corpus, or a report), root-cause it to one phase decision, and ship ONE
option-gated feature that closes it — verified, measured, and reviewable. This file is the
runbook: an agent (interactive session or headless worker) should be able to execute the
whole loop from what's written here. Nothing ever lands on `main` without a
human-reviewed PR.

One iteration:

1. **Find** a case kuna is bad at (§1 — three routes).
2. **Investigate**: reproduce it, confirm the reference is genuinely better, localize the
   decision to a phase (§2).
3. **Implement** one option-gated feature, modelled on `kuna_loweredswitch.rs` (§3).
4. **Verify & ship**: ablation decides the default, speed is a gate, four green gates,
   PR with a real before/after (§4).

A negative result (gap needs deeper restructuring; no firing testcase can be built) is a
legitimate outcome — record it and stop (§4). Not every gap is a feature.

## 0. Prerequisites

```bash
make binaries && make specs      # decomp_dbg / kuna at decompiler/target/release/, .sla specs
cd <repo-root>                   # every `python -m scripts.pipeline.<mod>` below runs from the root
```

Environment (defaults live in `scripts/pipeline/config.py` — `KUNA_PY`'s in the
`tools/pipeline/*.sh` drivers; they assume the Linux pipeline host — **on other machines
export the overrides**):

| Env var | Meaning (default) |
|---|---|
| `KUNA_PIPELINE_ANGR_PYTHON` | interpreter that can `import angr` (`~/.virtualenvs/decbench/bin/python`) |
| `KUNA_PIPELINE_ANGR_REPO` | angr checkout for worklist/units parsing (`/home/mahaloz/github/angr-dev/angr`) |
| `KUNA_PIPELINE_BIN_ROOT` | angr `binaries` checkout (`/home/mahaloz/github/angr-dev/binaries`) |
| `KUNA_PY` | interpreter for the shell drivers (`~/.virtualenvs/kuna/bin/python`) |
| `KUNA_PIPELINE_SPEED_BUDGET_PCT` | speed gate (5) |
| `KUNA_PIPELINE_STATE_DIR` | worker state (`.kuna-pipeline/`, gitignored) |

Two venvs by design: `KUNA_PY` runs the pipeline (no third-party deps — plain `python3`
works); `KUNA_PIPELINE_ANGR_PYTHON` is only shelled out to for angr decompilation. If the
angr venv is broken, `worklist`/`units` still work (pure AST parsing); `compare`/`sweep`
need a working `import angr` (bootstrap: any venv with `pip install angr==<ref_version>`;
the binary corpus is the angr-dev `binaries` checkout). The angr **version that actually ran is pinned into every
result** (`ref_version`) — trust that over any doc.

Generated backlog artifacts land in `docs/improvement-pipeline/` (created on demand):
`worklist.json`, `opportunities.json`, `matrix.md`, `units.json`.

## 1. Find a case kuna is bad at

### Route A — the angr testcase sweep (the automated backlog)

```bash
python3 -m scripts.pipeline.worklist --only-existing     # parse angr's test_decompiler.py → worklist.json
python3 -m scripts.pipeline.sweep --arch x86 --jobs 6 --progress   # angr-vs-kuna over the corpus → opportunities.json + matrix.md
python3 -m scripts.pipeline.units                        # angr peephole/optimization passes as 1-PR "small units" → units.json
python3 -m scripts.pipeline.select --json                # the next unclaimed, highest-score opportunity
```

`select` needs `sweep`'s `opportunities.json` (and `units`' `units.json`) on disk — with
an empty backlog it prints nothing and exits 1.

Ranking is structural (switch recovery=5, kuna-failed=4, fewer-gotos=3, more-loops=3,
fewer-labels=2, cast-noise=1, shorter=1). **Signals are a prefilter, not truth** — a
`recovery-failure`/huge-loc-delta row can be a kuna failure on a large function; you must
confirm in §2. `sweep` rebuilds the worklist in-memory each run (it does not read
`worklist.json`), and stores no decompiled text — re-run `compare` to see the actual C.
`select --kind small-unit|structural` balances the mix; small units (a single angr
peephole/optimization class) are the best first pick: self-contained, one PR, known
provenance.

### Route B — decbench: real binaries scored against real source

The decbench campaign (`docs/decbench-loop.md` — read it before using this route) scores
decompilers per-function on a compiled-project corpus with a graph-edit-distance metric
vs the original source. `scripts.decbench.mine` mines the angr-perfect pool (`angr GED = 0, kuna GED > 0`) —
real-world cases the testcase sweep can't see (ghidra-perfect analogs need an ad-hoc
`decbench improvements -b ghidra` run):

```bash
python3 -m scripts.decbench.status                        # campaign tracker
python3 -m scripts.decbench.mine --select 28              # needs a results tree (KUNA_DECBENCH_RESULTS)
python3 -m scripts.decbench.triage --case <id>            # stored angr + stored kuna + FRESH kuna, side by side
```

GED interpretation: 0 = perfect structural recovery; `inf` = missing/unparseable, not
"bad"; large CFGs are approximated; a degenerate source CFG makes the score meaningless.
Historically **~half of cross-decompiler GED gaps are scoring artifacts** — always run the
triage checklist in `docs/decbench-loop.md` before turning a case into a feature.

### Route C — direct comparison (a report, a hunch, any binary)

```bash
python3 -m scripts.pipeline.compare /path/to/bin <func> --reference angr   # side-by-side + metrics + signals
decompiler/target/release/kuna decompile-all ./bin --json                  # eyeball a whole binary (JSON per function)
```

`compare` takes a name, `0x`-address, or `@entry` as selector, and works on any binary —
this is the route for user reports and upstream Ghidra issues (the original GH-issue
campaign is summarized in `docs/history.md`).

**Before claiming any case**: check it isn't already covered — sweep `kuna catalog
--json` for an existing option whose `use_when`/`symptoms` match, and re-run the case
with that option on. If an existing option closes it, that's a **default-flip candidate
or a per-function runtime choice, not a new feature** — record it and pick another case.

Claiming works the same interactively as in the fleet (any stable string as worker id):
`python3 -m scripts.pipeline.state claim --worker <your-id> --opportunity <id>` (exit 0 =
claimed, 1 = taken); a dead end is `state update --worker <id> --status failed --note
"<reason>"`; a parked proposal is `state proposal --worker <id> --opportunity <id> --pr
<url> --branch <branch>`.

## 2. Investigate: reproduce, confirm, localize

Goal: `docs/features/<slug>/analysis.md` naming **one concrete structural difference**
and **the one phase decision** that owns it. If you can't name both, you aren't done
investigating.

1. **Reproduce** and save the evidence:
   ```bash
   python3 -m scripts.pipeline.compare --entry <test_name> > docs/features/<slug>/angr-vs-kuna.txt
   ```
   (`--entry` resolves Route-A worklist test names; for Route B/C use the positional form
   `compare <binary> <selector> --reference angr`, or `scripts.decbench.triage --case <id>`.)
2. **Confirm the reference is genuinely better.** Read both outputs. Better = structurally
   closer to plausible source (a `switch` instead of an if-cascade, a loop instead of
   gotos, fewer labels) — not just shorter. If kuna is actually right or the difference
   is taste, record a negative result and stop.
3. **Localize the decision to a phase.** The model is P0 (knowledge plane) + P1–P9 with
   Band B (P3–P6) as a mutual fixed point — one screen: `docs/phases.md`; per-pass
   algorithms: `docs/spec/` (each phase folder has exactly one owning chapter, found via
   its `Anchors:` header). Tools, in the order that usually works:
   - **Symptom index**: `docs/options.md` opens with an "If the output shows… → try
     `option`" table. Caveat from practice: *the symptom's phase is often not the
     decision's phase* — wrong-looking constants in the C are usually destroyed in Band B,
     not mis-rendered at P9.
   - **Option-flip diagnosis**: flip nearby decisions and diff the output —
     `kuna decompile ./bin fn --option <name> <value>` (repeatable), or
     `--kassert "<phase> <subphase> <value>"` for a typed P0 assertion.
   - **Console session** for the pipeline's own view:
     ```bash
     printf 'load file ./bin\nread symbols\nload function fn\ndecompile\nquality\nregion tree\nrestarts\nphase status\nquit\n' \
       | SLEIGHHOME=$PWD/specs decompiler/target/release/decomp_dbg
     ```
     `quality` = the goto/structure metric; `restarts` = why the function restarted;
     `region tree` = the P7 region view; `pipeline <variant>` runs reduced sub-queries;
     plus the whole Ghidra console (`print raw`, `graph controlflow`, `list action`).
     Note `kuna decompile-all` injects `option listing on` (and `funcstart_patterns`/
     `aif` on non-x86-64) — pass the same options to single-function runs when comparing.
   - **Root-cause in the reference**: find the angr pass/class that produces the better
     shape. Ground truth is the *installed* angr in the venv (the version that ran), not
     angr master; use the checkout for readability.
4. **Decide scope.** Small = one Action/Rule-shaped pass, one module, hooked at one
   registration point (like `kuna_loweredswitch.rs`). **Large** — any of: a new pass
   *type* or infrastructure; structuring work beyond one gated pass; >3 ported-core
   anchor files; >1 new module — goes the `[PROPOSAL]` route (§4), design first, no
   implementation before human approval. When the call is close, have a second
   opinion (a subagent) make it, and record its verdict verbatim in `record.json`.
5. **Pick the slug and write the plan.** The slug becomes the option name,
   `kuna_<slug>.rs`, `docs/features/<slug>/`, and the branch suffix (Route A: use
   `select`'s `SLUG` field). Write `docs/features/<slug>/plan.md` (the design) before
   starting §3.

## 3. Implement: one option-gated feature

Canonical template: `decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitch.rs`
(+ its `phases.toml` row and `tests/stages/ghangr-loweredswitch.xml`). Model the code,
registration, and test on it. New logic lives in `kuna_<slug>.rs` inside the owning phase
folder; edits to ported-core files only where an anchor demands it, each marked
`// (kuna)`. Develop with the option **default-OFF** — default output stays
byte-identical until the ablation says otherwise.

The artifact checklist (every item, or the gates fail opaquely):

1. `decompiler/crates/kuna-decomp/src/<phase>/kuna_<slug>.rs` — the feature, early-return
   when its flag is off. Flag on the architecture struct, defaulted in its reset path.
   ElementId from the 4000+ range (grep for the current highest).
2. `[[settable]]` row in `decompiler/crates/kuna-decomp/phases.toml` — **every field**,
   including the provenance contract (§below): `summary`, `use_when`, `values`+`example`,
   `tier`, `symptoms`, `source_decompiler`, `inspiration`, `change_kind`.
3. Option name registered in `src/p0_knowledge/options.rs` (`KUNA_OPTION_NAMES`).
4. **Bump the hard-coded catalog counts** (three places; grep the current total):
   `src/p0_knowledge/kuna_phases/tests.rs` (count asserts),
   `decompiler/crates/kuna-decomp/tests/catalog_bytecompat.rs` (count asserts **and** the
   `tests/fixtures/phase_catalog.json` fixture — regenerate it with the console
   `openfile write`/`phase catalog` recapture documented in that file's header), and the
   `tests/stages/kuna-catalog.xml` stringmatch counts (bump whichever bucket the new
   option's `source_decompiler`/`change_kind` lands in).
5. **Two-pass stage test** `tests/stages/ghangr-<slug>.xml` (`ghdec-<slug>.xml` for
   decbench-derived fixes with no angr analog): pass 1 `option <slug> off` asserts the
   BUG, pass 2 `option <slug> on` asserts the FIX (as `ghangr-loweredswitch.xml` does —
   the explicit `on` stays correct whatever §4 decides the default is), over a real
   `binary → decompile` path
   (bytechunk allowed only if it reproduces the same construct). A `min="1" max="1"`
   stringmatch spans both passes' transcript — absent in pass 1 AND present in pass 2.
   Then re-record the stages baseline:
   ```bash
   decompiler/target/release/kuna test --datatests --datatests-dir tests/stages --save-baseline docs/baseline-stages.json
   ```
6. Regenerate the catalog doc: `decompiler/target/release/kuna catalog --markdown > docs/options.md`.
7. Update the owning `docs/spec/` chapter; `make check-spec`.

## 4. Verify & ship

**Ablation decides the default.** `kuna test` has no option flag — to measure the flip,
temporarily set the option's default to ON (its architecture reset path + the
`phases.toml` row's `default =`), `make binaries`, then run
`kuna test --all --baseline docs/baseline.json`; revert the flip if the table below says
opt-in:

| Ablation (option ON) | Speed (`timeit`) | Ship as |
|---|---|---|
| 0/675 assertions change | within budget (≤5%) | **default-ON**: keep the flipped `default =`, add a DIV row in `docs/history.md`, and say "On by default (DIV-N)" in the row's `use_when` prose |
| 0/675 assertions change | over budget / unmeasured | default-OFF opt-in, `speed_forced_off: true` — no DIV |
| >0 assertions change | — | default-OFF opt-in — no DIV |

**Never re-pin `docs/baseline.json`.** The speed gate can only demote default-ON to
opt-in. Measure with:

```bash
python3 -m scripts.pipeline.timeit --record docs/features/<slug>/record.json \
  --option <slug> --binary <bin> --selector <fn> --repeat 5   # splices speed_* into the record
```

**The four gates, all green, before commit**:
`make test` (**PARITY OK** 675/675) · `make test-stages` (**PARITY OK**) ·
`make rust-test` · `make check-spec`. Plus `kuna catalog --check` → `catalog OK`.

**The PR** carries the per-feature bundle `docs/features/<slug>/`:

- `analysis.md` — why the reference is better, the exact construct, the owning phase.
- `angr-vs-kuna.txt` — the side-by-side the decision rests on.
- `plan.md` — the design (and `proposal.md` on the large route).
- `record.json` — the machine record. The tooling reads these exact keys (`timeit`
  writes the `speed_*` block; `prdemo` reads `option/binary/selector/func_addr/
  test_name/default_on/demo_extra_options`; decbench's `rescore --record` splices
  `ged_before/ged_after`):
  ```json
  { "opportunity": "<test_name>::<selector>", "test_name": "...", "binary": "...",
    "selector": "...", "func_addr": "0x...", "angr_version": "...", "option": "<slug>",
    "flag": "<arch-struct field>", "element_id": 4xxx, "change_kind": "...",
    "source_decompiler": "angr", "inspiration": "...", "default_on": false,
    "ablation_changed": 0, "parity": "OK", "decisions": ["..."] }
  ```

Do **not** hand-write before/after decompilation into the PR body —
`tools/pipeline/open_pr.sh <branch> "<title>" <body.md> <record.json>` appends a real
captured off/on demo (`scripts.pipeline.prdemo`) from `record.json`, so the record's
`option/binary/selector/func_addr` must be correct — and verify the demo actually landed
in the PR body (`open_pr.sh` degrades to a WARN and opens the PR without it if `prdemo`
fails, e.g. on a bad `KUNA_PY`). Branch names: `feat/angr-<slug>`
(`feat/decbench-<slug>` for route B).

**Large features** ([PROPOSAL] route): commit only `analysis.md` + `proposal.md` +
partial `record.json`, open a **draft** PR titled `[PROPOSAL] <slug>: …`
(`open_pr.sh --draft`), park it (`state proposal`), and stop. Approval re-dispatches
implementation on the same branch (`IMPL_PROPOSAL=1 RESUME_BRANCH=…`); `--undraft` when
green.

**Negative results**: commit the analysis + `record.json` (+ a `.patch` of any attempt)
to the local branch, mark the state `failed` with a one-line reason, **do not push, do
not open a PR** — no orphan remote branches. A recorded dead end is worth more than a
forced feature.

## Standing requirements (every pipeline PR MUST satisfy these)

Non-negotiable; the worker prompt enforces them per-feature and the orchestrator
re-queues violations.

1. **One PR per ported feature.** Exactly one `kuna_<slug>.rs` + one option + one stage
   test + one `docs/features/<slug>/` bundle per PR. No drive-by changes.
2. **End-to-end testcase.** The two-pass stage test of §3.5 — option off = the bug,
   default = the fix, over the full `binary → decompile(addr|func)` path.
3. **Output-changing ⇒ logged + flaggable.** Any feature that can change emitted C sits
   behind a runtime `--option <name>` (`phases.toml` settableTable + `options.rs`) and is
   recorded in `docs/history.md` (a DIV row iff default-ON). No silent output change
   ever reaches default.
4. **Always measure + record speed (speed is critical).** Every `record.json` carries the
   `timeit` speed block (§4). Over `KUNA_PIPELINE_SPEED_BUDGET_PCT` (default +5%) ⇒
   default-OFF opt-in. The speed gate never re-pins `docs/baseline.json`.
5. **Large/multi-part features go through a draft `[PROPOSAL]` PR first** (§4) — an
   explicit human go/no-go before implementation.
6. **The spec is live.** Any behavior change updates the owning `docs/spec/` chapter in
   the same PR; `make check-spec` must pass.

## The headless fleet (optional automation)

The same loop, driven autonomously: `run.sh` keeps N workers alive, each an isolated
git-worktree `claude -p` session executing `tools/pipeline/worker_prompt.md` (the
templated per-feature version of §§2–4, with heartbeats and the state protocol).

```bash
tools/pipeline/install_gh.sh                       # one-time PR tooling (Linux host)
PIPELINE_WORKERS=1 tools/pipeline/run.sh --once    # exactly one feature
PIPELINE_WORKERS=2 PIPELINE_HOURS=8 tools/pipeline/run.sh   # a working day, two workers
python3 -m scripts.pipeline.status --watch         # live: workers, worktrees, PRs, proposals
touch .kuna-pipeline/STOP                          # graceful stop; in-flight workers finish
```

Mechanics an operator should know: workers claim opportunities atomically
(`state claim`, exit code 0/1); state always lives in the **main** tree
(`KUNA_PIPELINE_STATE_DIR`) so `status` sees worktree workers; worktrees reuse the main
tree's compiled specs (`KUNA_SPECS` — never `make specs` in a worktree); session ids are
recorded for `claude --resume` review; merged/closed-PR worktrees are GC'd, open ones
kept. Proposals: `status --proposals` lists parked drafts;
`state approve --opportunity <id>` green-lights one.

## Machinery reference

| Piece | What |
|---|---|
| `scripts/pipeline/reference/` | `ReferenceDecompiler` protocol; registry `angr`, `kuna`. Add another decompiler by implementing the protocol and registering it — everything downstream is decompiler-agnostic (`compare --reference <name>`). |
| `scripts/pipeline/worklist.py` | angr `test_decompiler.py` → `(binary, function)` targets (AST parse, never executes angr). |
| `scripts/pipeline/compare.py` | reference vs kuna on the same function; metrics + "where the reference is better" signals. Structural, never raw-text. |
| `scripts/pipeline/sweep.py` + `rank.py` | `compare` across the corpus → ranked `opportunities.json` + human `matrix.md`. |
| `scripts/pipeline/units.py` | angr peephole/optimization passes → self-contained 1-PR "small units". |
| `scripts/pipeline/select.py` | next unclaimed, highest-score opportunity (`--shell` / `--json`). |
| `scripts/pipeline/timeit.py` | the speed gate: off/on medians + delta into `record.json`. |
| `scripts/pipeline/prdemo.py` | real captured before/after markdown for the PR body. |
| `scripts/pipeline/state.py` / `status.py` | flock-guarded claims/heartbeats/proposals; live observability. |
| `scripts/decbench/` | the real-binary route: mine/triage/rescore/status/showcase (`docs/decbench-loop.md`). |
| `scripts/port_status.py` | angr→kuna port coverage report (from `phases.toml` provenance + the sweep). |
| `tools/pipeline/` | headless drivers: `run.sh`, `worker.sh`, `worker_prompt.md`, `open_pr.sh`, `install_gh.sh`. |

## The provenance contract (what makes features LLM-discoverable)

Every shipped option's `phases.toml` row must let the *next* agent discover and judge it
from `kuna catalog --json` alone: `summary` (WHAT), `use_when` (WHEN — the symptom),
`values` + `example` (HOW), `tier` (`transform` = the on/off control surface | `core` |
`analysis`), `symptoms` (2–5 output-shaped greppable phrases), plus provenance:

- `source_decompiler` — where the behavior came from (`"angr"`, `"ghidra-upstream"`, …).
- `inspiration` — the concrete origin (test/case id + pass/class + function; decbench
  cases use the `decbench:<case-id>` prefix — tooling keys on it).
- `change_kind` — `correctness-fix` | `presentation-default` | `structure-recovery` |
  `analysis-enablement` | `opt-in-tool`; the field an agent reads to judge whether
  flipping is cosmetic, structural, or semantic.

`kuna catalog --check` + the `tests/stages/kuna-catalog.xml` fence enforce the contract.

## Known caveats

- **Host paths**: `config.py`'s angr-checkout defaults are the Linux pipeline host's;
  export `KUNA_PIPELINE_ANGR_REPO`/`KUNA_PIPELINE_BIN_ROOT`/`KUNA_PY` elsewhere. A
  missing angr checkout crashes `worklist` (FileNotFoundError on `test_decompiler.py`);
  a wrong binaries root silently empties it.
- **angr version skew**: results pin the installed venv's version (`ref_version`);
  root-cause against that version, use the checkout only for readability.
- **Prefilter ≠ verdict**: sweep signals and decbench GED both misfire (kuna-failure
  rows, scoring artifacts) — §2's confirm step is mandatory before any implementation.
- **Sweep scope**: run with `--arch x86` (best spec overlap, fastest) — an unfiltered
  sweep covers every arch in the worklist; other arches work but expect more kuna-side
  load/spec gaps.
- **A reference failure mutes kuna's side**: when the angr leg errors, `compare` skips
  the kuna leg and prints "(no output)" — that is not a kuna failure; run
  `kuna decompile` directly to see kuna's output.
- **`select --skip-custom` is currently a no-op** — filter custom-option testcases at
  sweep time (`sweep --skip-custom`) instead.
