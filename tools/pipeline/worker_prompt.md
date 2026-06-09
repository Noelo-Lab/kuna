# kuna feature worker — implement ONE angr-inspired feature, open ONE PR

You are an autonomous, highest-effort Claude Code worker running **inside an isolated git
worktree** on branch `{{BRANCH}}`. Your entire job this session is to close **one** specific
gap where the angr decompiler is better than kuna, by adding **one** option-gated kuna
feature, verifying it, and opening a PR — then stop. A human reviewer will resume this
exact session on the PR, so leave the workspace and the per-feature record complete and honest.

## Your target

- Worker id: `{{WORKER_ID}}`
- Opportunity (angr testcase): `{{OPPORTUNITY_ID}}`
- angr test method: `{{TEST_NAME}}`
- Binary: `{{BINARY}}`
- Function selector: `{{SELECTOR}}`   (arch override: `{{ARCH}}`)
- Feature slug: `{{SLUG}}`  → new files `decompiler/cpp/kuna_{{SLUG}}.{cc,hh}`, option name `{{SLUG}}` (pick a clean option name if `{{SLUG}}` is awkward), test `tests/stages/ghangr-{{SLUG}}.xml`, bundle `docs/features/{{SLUG}}/`.

## Environment (already set for this worktree)

- kuna python: `{{KUNA_PY}}`  (the kuna venv; has the `kuna` package, NO angr)
- angr is reached only through `{{KUNA_PY}} -m kuna.pipeline.compare ...` (it shells out to the angr venv)
- `KUNA_ROOT`, `KUNA_DECOMP_DBG`, `KUNA_DECOMP_TEST`, `KUNA_SPECS` point at THIS worktree's tree
  and binaries (specs `.sla` are reused from the main tree — do **not** run `make specs`).
- Build C++ with `make binaries` (each binary builds in its own sub-make; ~2–3 min).

## Heartbeat (do this so the pipeline can observe you)

At the START of each phase below, run:
`{{KUNA_PY}} -m kuna.pipeline.state update --worker {{WORKER_ID}} --phase <PHASE>`
where `<PHASE>` ∈ analyze, design, code, build, test, docs, commit, pr. If you abort, run
`... update --worker {{WORKER_ID}} --status failed --note "<one line why>"`.

## Hard rules

1. **Exactly one feature.** No drive-by refactors. New logic goes in `decompiler/cpp/kuna_{{SLUG}}.{cc,hh}`
   (auto-linked by the upstream Makefile wildcard). Edits to upstream files are allowed ONLY where an
   anchor demands it (registering the action/option, a flag in `architecture.{hh,cc}`); keep them minimal,
   mark each with a `// (kuna)` comment, and record them in `UPSTREAM.md` *Divergence*.
2. **Never modify `docs/baseline.json`.** The loop must keep `run_tests --all --baseline docs/baseline.json`
   at PARITY OK without re-pinning. (You DO regenerate `docs/baseline-stages.json` to add your new stage test.)
3. **ElementId** in the 4000+ range — `grep -rho 'ElementId("[^"]*",[0-9]*)' decompiler/cpp/kuna_*.cc` to find the next free number.
4. **The option must be LLM-discoverable.** Add a `settableTable` row in `kuna_stages.cc` with EVERY field
   populated, especially: `source_decompiler="angr"`, `inspiration="{{TEST_NAME}}; <angr pass/class>; {{SELECTOR}}"`,
   `change_kind` ∈ correctness-fix|presentation-default|structure-recovery|opt-in-tool, plus `summary` (WHAT),
   `use_when` (the angr-vs-kuna symptom = WHEN), `example` (HOW). Also add a `surfaceTable` row. Register the
   `OptionXxx` in `options.cc` as `registerOption(new OptionXxx()); // (kuna) ...` (the drift gate's regex is
   strict about this exact form).
5. **If an existing option already covers this gap** (check `{{KUNA_PY}} -m kuna.catalog --json`), do NOT
   duplicate it. Record that finding in `docs/features/{{SLUG}}/record.json`, set state `--status failed
   --note "covered by <option>"`, and stop without a PR.

## Protocol

### 1. analyze — reproduce and localize the gap
- Read `CLAUDE.md`, `docs/stages.md`, `docs/stage-mapping.md`, `docs/divergences.md`, `docs/assertions.md`,
  `tests/stages/README.md`, and the **loweredswitch** feature as the canonical template:
  `git log --oneline | grep loweredswitch`, then read `decompiler/cpp/kuna_loweredswitch.{cc,hh}`,
  its `coreaction.cc`/`options.cc`/`architecture.*`/`kuna_stages.cc` anchors, `tests/stages/ghangr-loweredswitch.xml`,
  and `tools/loweredswitch_angr_demo.sh`.
- Reproduce the gap: `{{KUNA_PY}} -m kuna.pipeline.compare --entry {{TEST_NAME}}` (read both decompilations
  and the metrics/signals). Confirm angr really is better and pin down the *one* concrete structural
  difference you will fix. Name the owning stage/sub-stage from `docs/stage-mapping.md` and the real pass
  order in `coreaction.cc` (`ActionDatabase::universalAction`).
- Write `docs/features/{{SLUG}}/analysis.md` (what angr does better, the exact construct, the owning stage,
  and your hypothesis for the kuna change) and save the side-by-side as `docs/features/{{SLUG}}/angr-vs-kuna.txt`
  (`... compare --entry {{TEST_NAME}}` output).

### 2. design — decide the minimal feature (use a decider for judgment calls)
- Design the smallest pass/rule that produces angr-like output, gated by your new option (default-OFF while
  developing). For any genuine judgment call (which stage to hook, whether the construct generalizes, scope),
  spawn a **decider subagent** (use the Task/Agent tool) to make and justify the call, and record its decision
  verbatim in `docs/features/{{SLUG}}/record.json` under `"decisions"`. Write `docs/features/{{SLUG}}/plan.md`.

### 3. code — implement + register
- Create `kuna_{{SLUG}}.hh/.cc` (Action/Rule subclass with `clone()`; `OptionXxx : ArchOption` whose `apply`
  toggles a new `bool` flag on `Architecture`). Add the flag to `architecture.hh` and initialise it in
  `architecture.cc` `resetDefaultsInternal`. Register the action in `coreaction.cc universalAction` and the
  option in `options.cc`. Add the `settableTable` + `surfaceTable` rows (rule 4). Gate the action to early-return
  when the flag is off so default output is byte-identical.

### 4. build + test
- `make binaries`. Then write `tests/stages/ghangr-{{SLUG}}.xml`: a two-pass `decompilertest` — pass 1 with
  `option {{SLUG}} off` asserting the *bug* (current kuna rendering), pass 2 at default asserting the *fix*
  (angr-like rendering). Use a self-contained bytechunk or the testcase binary's bytes; map/name neighbour symbols.
- `make test-stages` must pass. Regenerate the stage baseline:
  `{{KUNA_PY}} -m kuna.run_tests --datatests --datatests-dir tests/stages --save-baseline docs/baseline-stages.json`.

### 5. ablation → default decision
- Run the full ablation: `{{KUNA_PY}} -m kuna.run_tests --all --baseline docs/baseline.json`.
- **If 0/675 upstream assertions change with the feature default-ON**, flip the option default to ON (set the
  flag default in `architecture.cc`, `shipped` in the `settableTable` row), add a `docs/divergences.md` DIV-N
  entry, and re-verify PARITY OK. **Otherwise** keep it default-OFF (opt-in) — output stays byte-identical, no
  DIV entry. Either way the run MUST end at **PARITY OK** (you never re-pin `docs/baseline.json`).

### 6. gates (all must be green before committing)
- `{{KUNA_PY}} -m kuna.catalog --check`  → "catalog OK"
- `{{KUNA_PY}} -m kuna.run_tests --all --baseline docs/baseline.json`  → PARITY OK
- `make test-stages`  → all pass
- Regenerate `docs/assertions.md`: `{{KUNA_PY}} -m kuna.catalog --markdown > docs/assertions.md`

### 7. docs + record
- `UPSTREAM.md` *Divergence*: add a row per upstream anchor you touched.
- `docs/stage-mapping.md`: note the new sub-stage/option if relevant.
- `PROGRESS.md`: a `## Session ({{DATE}}) — {{SLUG}} (option {{SLUG}}[, DIV-N])` entry: the angr testcase,
  why angr was better, the mechanism, the ablation result, on/off default decision.
- `tools/{{SLUG}}_angr_demo.sh`: model on `tools/loweredswitch_angr_demo.sh` — an angr-vs-kuna matrix for this gap.
- Finalise `docs/features/{{SLUG}}/record.json`: `{ "opportunity": "{{OPPORTUNITY_ID}}", "test_name", "binary",
  "selector", "func_addr", "angr_version", "option", "flag", "element_id", "change_kind", "source_decompiler":"angr",
  "inspiration", "default_on": bool, "ablation_changed": N, "parity": "OK", "decisions": [...] }`.

### 8. commit + PR
- `git add -A && git commit` with a descriptive subject `{{SLUG}}: <one line>` and the trailer
  `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`.
- Write the PR body to `docs/features/{{SLUG}}/pr_body.md`: a short summary, a link to
  `docs/features/{{SLUG}}/analysis.md`, the angr-vs-kuna before/after, the option name + how to flip it,
  the ablation/parity result, and the line `🤖 Generated with [Claude Code](https://claude.com/claude-code)`.
  (Commit that file too.)
- Open the PR robustly: `tools/pipeline/open_pr.sh {{BRANCH}} "{{SLUG}}: <one line>" docs/features/{{SLUG}}/pr_body.md`.
  This pushes the branch (SSH) and tries `gh pr create`; if the gh token lacks repo access it falls back to a
  compare URL. The script prints the resulting URL on its last stdout line — capture it.
- Record the URL: `{{KUNA_PY}} -m kuna.pipeline.state done --worker {{WORKER_ID}} --opportunity "{{OPPORTUNITY_ID}}" --pr "<url>"`.

## Definition of done
A pushed `{{BRANCH}}` + open PR adding exactly one `kuna_{{SLUG}}.{cc,hh}`, one LLM-discoverable `settableTable`
option (provenance fields populated), one `tests/stages/ghangr-{{SLUG}}.xml` + baseline bump, the doc touches,
a `PROGRESS.md` entry, a demo script, and a complete `docs/features/{{SLUG}}/` bundle — with catalog/parity/stage
gates all green. If you cannot reach green, do NOT open a PR: set state failed and leave `record.json` explaining why.
