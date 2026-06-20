# kuna feature worker — implement ONE angr-inspired feature, open ONE PR

You are an autonomous, highest-effort Claude Code worker running **inside an isolated git
worktree** on branch `{{BRANCH}}`. Your entire job this session is to close **one** specific
gap where the angr decompiler is better than kuna, by adding **one** option-gated kuna
feature to the **Rust** engine, verifying it, and opening a PR — then stop. A human reviewer
will resume this exact session on the PR, so leave the workspace and the per-feature record
complete and honest.

## Your target

- Worker id: `{{WORKER_ID}}`
- Opportunity (angr testcase): `{{OPPORTUNITY_ID}}`
- angr test method: `{{TEST_NAME}}`
- Binary: `{{BINARY}}`
- Function selector: `{{SELECTOR}}`   (arch override: `{{ARCH}}`)
- Feature slug: `{{SLUG}}`  → new module `decompiler/crates/kuna-decomp/src/kuna_{{SLUG}}.rs`, option name `{{SLUG}}` (pick a clean option name if `{{SLUG}}` is awkward), test `tests/stages/ghangr-{{SLUG}}.xml`, bundle `docs/features/{{SLUG}}/`.

## Environment (already set for this worktree)

- kuna python: `{{KUNA_PY}}`  (has the angr-driving pipeline deps, NO angr itself)
- The pipeline is run from the repo root: `{{KUNA_PY}} -m scripts.pipeline.compare ...` (it
  shells out to the angr venv).
- The Rust `kuna` CLI is `decompiler/target/release/kuna` — use it for `kuna catalog ...`
  and `kuna test ...`.
- `KUNA_ROOT`, `KUNA_DECOMP_DBG`, `KUNA_DECOMP_TEST`, `KUNA_SPECS` point at THIS worktree's tree
  and binaries (specs `.sla` are reused from the main tree — do **not** run `make specs`).
- Build with `make binaries` (cargo; the first build of the worktree is slowest, then incremental).

## Heartbeat (do this so the pipeline can observe you)

At the START of each phase below, run:
`{{KUNA_PY}} -m scripts.pipeline.state update --worker {{WORKER_ID}} --phase <PHASE>`
where `<PHASE>` ∈ analyze, design, code, build, test, docs, commit, pr. If you abort, run
`... update --worker {{WORKER_ID}} --status failed --note "<one line why>"`.

## Hard rules

1. **Exactly one feature.** No drive-by refactors. New logic goes in
   `decompiler/crates/kuna-decomp/src/kuna_{{SLUG}}.rs` (declare it as a `mod` in `lib.rs`).
   Edits to the ported core files are allowed ONLY where an anchor demands it (registering the
   action/option, a flag on the architecture struct); keep them minimal, mark each with a
   `// (kuna)` comment, and record them in `docs/UPSTREAM.md` *Divergence*.
2. **Never modify `docs/baseline.json`.** The loop must keep `kuna test --all --baseline docs/baseline.json`
   at PARITY OK without re-pinning. (You DO regenerate `docs/baseline-stages.json` to add your new stage test.)
3. **ElementId** in the 4000+ range — grep the existing `kuna_*.rs` sources and `stages.toml`
   for the highest id already used and pick the next free one.
4. **The option must be LLM-discoverable.** Add a `settableTable` row in
   `decompiler/crates/kuna-decomp/stages.toml` with EVERY field populated, especially:
   `source_decompiler="angr"`, `inspiration="{{TEST_NAME}}; <angr pass/class>; {{SELECTOR}}"`,
   `change_kind` ∈ correctness-fix|presentation-default|structure-recovery|opt-in-tool, plus `summary` (WHAT),
   `use_when` (the angr-vs-kuna symptom = WHEN), `example` (HOW). Register the option in
   `decompiler/crates/kuna-decomp/src/options.rs` (so it appears in `KUNA_OPTION_NAMES`).
5. **If an existing option already covers this gap** (check `kuna catalog --json`), do NOT
   duplicate it. Record that finding in `docs/features/{{SLUG}}/record.json`, set state `--status failed
   --note "covered by <option>"`, and stop without a PR.

## Protocol

### 1. analyze — reproduce and localize the gap
- Read `AGENTS.md`, `docs/stages.md`, `docs/stage-mapping.md`, `docs/divergences.md`, `docs/assertions.md`,
  `tests/stages/README.md`, and the **loweredswitch** feature as the canonical template:
  `git log --oneline | grep loweredswitch`, then read
  `decompiler/crates/kuna-decomp/src/kuna_loweredswitch.rs`, its anchors in `coreaction*.rs` /
  `universalaction.rs` / `options.rs` / `architecture.rs` / `stages.toml`, and
  `tests/stages/ghangr-loweredswitch.xml`.
- Reproduce the gap: `{{KUNA_PY}} -m scripts.pipeline.compare --entry {{TEST_NAME}}` (read both
  decompilations and the metrics/signals). Confirm angr really is better and pin down the *one*
  concrete structural difference you will fix. Name the owning stage/sub-stage from
  `docs/stage-mapping.md` and the real pass order in `coreaction*.rs` / `universalaction.rs`.
- Write `docs/features/{{SLUG}}/analysis.md` (what angr does better, the exact construct, the owning stage,
  and your hypothesis for the kuna change) and save the side-by-side as `docs/features/{{SLUG}}/angr-vs-kuna.txt`
  (`... compare --entry {{TEST_NAME}}` output).

### 2. design — decide the minimal feature (use a decider for judgment calls)
- Design the smallest pass/rule that produces angr-like output, gated by your new option (default-OFF while
  developing). For any genuine judgment call (which stage to hook, whether the construct generalizes, scope),
  spawn a **decider subagent** (use the Task/Agent tool) to make and justify the call, and record its decision
  verbatim in `docs/features/{{SLUG}}/record.json` under `"decisions"`. Write `docs/features/{{SLUG}}/plan.md`.

### 3. code — implement + register
- Create `kuna_{{SLUG}}.rs` (an Action/Rule modeled on `kuna_loweredswitch.rs`), add a `bool`
  option flag on the architecture struct (defaulted off in its reset path), register the action in
  the `universalAction` registration (`coreaction*.rs` / `universalaction.rs`) and the option in
  `options.rs`, and add the `settableTable` row (rule 4). Gate the action to early-return when the
  flag is off so default output is byte-identical.

### 4. build + test
- `make binaries`. Then write `tests/stages/ghangr-{{SLUG}}.xml`: a two-pass `decompilertest` — pass 1 with
  `option {{SLUG}} off` asserting the *bug* (current kuna rendering), pass 2 at default asserting the *fix*
  (angr-like rendering). Use a self-contained bytechunk or the testcase binary's bytes; map/name neighbour symbols.
- `make test-stages` must pass. Regenerate the stage baseline:
  `kuna test --datatests --datatests-dir tests/stages --save-baseline docs/baseline-stages.json`.

### 5. ablation → default decision
- Run the full ablation: `kuna test --all --baseline docs/baseline.json`.
- **If 0/675 upstream assertions change with the feature default-ON**, flip the option default to ON (set the
  flag default in the architecture reset path, `shipped` in the `settableTable` row), add a `docs/divergences.md`
  DIV-N entry, and re-verify PARITY OK. **Otherwise** keep it default-OFF (opt-in) — output stays byte-identical,
  no DIV entry. Either way the run MUST end at **PARITY OK** (you never re-pin `docs/baseline.json`).

### 6. gates (all must be green before committing)
- `kuna catalog --check`  → "catalog OK"
- `kuna test --all --baseline docs/baseline.json`  → PARITY OK
- `make test-stages`  → all pass
- Regenerate `docs/assertions.md`: `kuna catalog --markdown > docs/assertions.md`

### 7. docs + record
- `docs/UPSTREAM.md` *Divergence*: add a row per ported-core file you touched.
- `docs/stage-mapping.md`: note the new sub-stage/option if relevant.
- `docs/PROGRESS.md`: a `## Session ({{DATE}}) — {{SLUG}} (option {{SLUG}}[, DIV-N])` entry: the angr testcase,
  why angr was better, the mechanism, the ablation result, on/off default decision.
- Finalise `docs/features/{{SLUG}}/record.json`: `{ "opportunity": "{{OPPORTUNITY_ID}}", "test_name", "binary",
  "selector", "func_addr", "angr_version", "option", "flag", "element_id", "change_kind", "source_decompiler":"angr",
  "inspiration", "default_on": bool, "ablation_changed": N, "parity": "OK", "decisions": [...] }`.

### 8. commit + PR
- `git add -A && git commit` with a descriptive subject `{{SLUG}}: <one line>` and the trailer
  `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`.
- Write the PR body to `docs/features/{{SLUG}}/pr_body.md`: a short summary, a link to
  `docs/features/{{SLUG}}/analysis.md`, the mechanism, the option name + how to flip it, and the
  ablation/parity result, ending with `🤖 Generated with [Claude Code](https://claude.com/claude-code)`.
  Do NOT hand-write the before/after decompilation — `open_pr.sh` auto-appends a REAL captured
  before/after (kuna with the option off vs on, on the feature's target function) + the reference
  rendering, generated from `record.json`. So make sure `record.json` has correct `option`, `binary`,
  `selector`, `func_addr`, `default_on`. (Commit pr_body.md too.)
- Open the PR robustly: `tools/pipeline/open_pr.sh {{BRANCH}} "{{SLUG}}: <one line>" docs/features/{{SLUG}}/pr_body.md docs/features/{{SLUG}}/record.json`.
  This pushes the branch (SSH), generates+embeds the before/after demo, then runs `gh pr create`
  (compare-URL fallback if gh can't). The script prints the resulting URL on its last stdout line — capture it.
- Record the URL: `{{KUNA_PY}} -m scripts.pipeline.state done --worker {{WORKER_ID}} --opportunity "{{OPPORTUNITY_ID}}" --pr "<url>"`.

## Definition of done
A pushed `{{BRANCH}}` + open PR adding exactly one `kuna_{{SLUG}}.rs`, one LLM-discoverable `settableTable`
option (provenance fields populated), one `tests/stages/ghangr-{{SLUG}}.xml` + baseline bump, the doc touches,
a `docs/PROGRESS.md` entry, and a complete `docs/features/{{SLUG}}/` bundle — with catalog/parity/stage
gates all green — opened via `tools/pipeline/open_pr.sh` so the before/after demo is embedded.

**Negative result (you cannot reach a green, shippable feature** — e.g. the gap needs deeper restructuring than
one option-gated pass, or no firing stage test can be synthesized): this is a legitimate outcome. Commit your
analysis + `record.json` + any `*.patch` of the attempt to the LOCAL branch and set state
`--status failed --note "<why>"`. Do **NOT** push the branch or open a PR — the worktree + local commit preserve
it for review; leave no orphan remote branch.
