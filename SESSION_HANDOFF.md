# Session handoff — stage-model physicalization (2026-06-06/07)

State snapshot for resuming the interrupted session on another machine. Delete this
file once the session's goals are complete. The approved plan lives at
`~/.claude/plans/today-we-doing-a-foamy-toucan.md` (included in the transfer bundle);
the session transcript (resumable via `claude --resume`) is session
`b9019d80-fde8-4bb9-8ee6-edd559523ef5` in the bundled `.claude/projects` dir.

## Session goals (user's original ask)

1. Implement the stage model (STAGES.md / docs/stage-model.md) throughout the
   decompiler — split into stages/sub-stages ("address each stage and say how you
   modify the decompiler").
2. Verify all testcases still pass.
3. Critique the stages empirically.
4. Fix+reproduce ≥10 issues from the PHADE dataset (`~/github/PHADE/issue_dataset`)
   through the stage model. Commit at milestones.

User-approved scope decisions: structural+observability depth (scoped re-run and
quality-gated rollback ship observable halves only + designs); triage at execution
with stage diversity; **unambiguous bugs may flip defaults** (documented in
docs/divergences.md; policy fixes stay behind default-off options = PARITY OK).

## DONE (all committed, every commit gated on PARITY OK 204/204+675/675)

Infrastructure (Phase 1, commits b6b15ce..e97f105):
- `kuna_stages.{hh,cc}` — registry: group→stage map, 40-row sub-stage catalog,
  surface routing. `kuna_console.{hh,cc}` — self-registering IfaceKunaCapability
  (zero anchor edits): `stage list/map/status`, `pipeline` (c′ sub-queries),
  `quality` (goto metric), `restarts`. `kuna_assert.{hh,cc}` — `kassert` typed
  assertion API with reported minimal rewind scopes. `kuna_restartlog.{hh,cc}` —
  restart-reason recording (5 one-line anchors in heritage/jumptable/fspec).
- Console-matcher gotcha learned: a bare command with free-form args cannot coexist
  with a registered subcommand sharing its first word — handle `list` internally.
- Datatest harness matches `fileoptr` (bulk stream), NOT `optr` — new commands that
  need test assertions must write to `*status->fileoptr`.

STAGE_MAPPING.md §0 remap to P0/S1–S9 (commit 288ab12).

Issue fixes — 4 of ≥10 banked + 1 spec fix (commits f2398f1..166a108):
| # | Issue | Stage | Mode | Option |
|---|---|---|---|---|
| 1 | GH-2786 `--x` invalid C | S9 | **default-flip** (DIV-1, 0/675 changed) | — |
| 2 | GH-8471 Thumb funcptr→hex | S5 const-pointer | option | `thumbfuncptr` |
| 3 | GH-6930 funcptr→0x100000 | S5 const-pointer | option | `inferfuncentry` |
| 4 | GH-6990 SPARC CONCAT44 void return | S4 trial-finalization | option | `returnpair single` |
| — | V850.cspec `ctbp`→`CTBP` case fix (unblocks V850 entirely) | — | spec fix | — |

Stage testcases: tests/stages/ 29/29 green (docs/baseline-stages.json regenerated
per commit). ElementIds used so far: 4000-4003 (next free: 4004).

## IN FLIGHT (lost to the crash — restart this)

A Workflow (`phade-hardfixes`, run wf_08b61316-629) was implementing the 6 remaining
issues in parallel git worktrees when the session died. **It produced nothing** (the
worktrees at `.claude/worktrees/wf_08b61316-629-*` have zero source edits — safe to
`git worktree remove --force` + delete the `worktree-wf_*` branches). The full
workflow script (with complete per-issue prompts) is preserved in the bundle at
`workflows/scripts/phade-hardfixes-wf_08b61316-629.js` — relaunch it or implement
sequentially.

The 6 remaining issues (per-issue verified root-cause cards in `/tmp/kuna-fixcards/
<n>.json` — restore these to /tmp on the new machine; backup copy in the bundle):
- GH-1282 (8051 `(b<<7)>>7`, S3 new rule; RuleLeftRight's `(sa&7)!=0` gate is the
  blocker; option `booleanmask`)
- GH-8913 (6502 carry-chain CONCAT11/CARRY1, S5 new rule matching PIECE of
  add+carry, two carry forms; option `addcarrychain`)
- GH-7190 (V850 flag-compare garbage, S3 new rule; V850 loads now)
- GH-8748 (else{if}→else-if, S8; gated Action duplicating shared tails via
  data.nodeSplit, modeled on ActionReturnSplit; option `splittail`)
- GH-9230 (unrolled memset, S5 constseq extension; its card repro_xml throws
  DecoderError — remove `readonly="true"` from bytechunk and `--` from XML comments)
- GH-8817 (V850 jmp[reg] CALLIND→BRANCHIND, S2; `override flow ... branch` does NOT
  convert CALLIND in practice (verified); synthetic repro has no populated jump
  table so pass-2 should assert the computed-call rendering is gone, not a full
  switch)

KEY TECHNIQUE for all 6 (the de-risking insight): gate each fix body on a default-off
Architecture bool (`if (!data.getArch()->flag) return 0;`) → PARITY OK is automatic;
the rule only needs to be correct on its own testcase when enabled. Follow the
kuna_returnpair/kuna_inferfuncentry/kuna_thumbfuncptr pattern exactly (option class +
arch flag + registry row + UPSTREAM.md rows + tests/stages double-decompile XML +
baseline-stages regen). Deferred as not-clean: GH-28 (repro diverges between
decomp_dbg and decomp_test_dbg builds), GH-8772 (already fixed at our rev —
regression-lock testcase optional).

## REMAINING after the 10 issues

- Phase 3: fill the `TBD-PHASE2` markers in docs/stage-implementation.md (uncommitted
  draft, now committed as WIP) with the final issue list per stage.
- Phase 5: docs/stage-critique.md (WIP draft committed) — fill the §1 table rows for
  the remaining issues, run the §2 ablation matrix (toggle each kuna option over the
  83 upstream datatests, count changed assertions, classify text vs analysis), settle
  §6 verdicts. Key finding already recorded: GH-8471/6930 prove symptom-stage ≠
  decision-stage (both look S9, live in S5) — the model's strongest validation.
- Phase 6: PROGRESS.md session block; tests/stages/README.md rows for the 6 new
  testcases; final full gate; delete this file.

## Critical environment facts

- Build: `make BFD_PREFIX="$(pwd)/.bfdlocal" binaries` (libbfd fetched via
  `tools/fetch_bfd.sh` — .bfdlocal is machine-local, regenerate on the new box).
- Tests: `make test-stages`; parity `~/.virtualenvs/kuna/bin/python -m kuna.run_tests
  --all --baseline docs/baseline.json` (venv: `pip install -e .` into a fresh venv).
- decomp_dbg driving: `SLEIGHHOME=$REPO/specs ./decomp_dbg` + heredoc; datatest arch
  ids sometimes need the 5-field form (e.g. `8051:BE:16:default:default`).
- PHADE dataset expected at `~/github/PHADE/issue_dataset` (109M — copy it too).
