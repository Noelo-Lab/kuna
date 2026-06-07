# Next-session plan prompt — extend stage-model fixes to all remaining reproducible Ghidra issues

> Paste the section below ("PROMPT") to start the next session. Everything above it is
> context for you (the human). The prompt is self-contained but points at the docs that
> already encode our conventions so it stays short.

## Where we are (read before starting)

Over the last sessions we built the kuna stage model physically (registry, `kassert`,
`pipeline`, `quality`, `restarts`, restart observability) and fixed **11 PHADE issues**
through it (GH-558, 2786, 8471, 6930, 6990, 1282, 7190, 8817, 8913, 9230, 1537), each as
a default-off-or-DIV-flipped, test-pinned sub-stage exposure. Then we flipped the 8
non-destructive ones on by default (DIV-2) and built an LLM control API
(`stage catalog` JSON → `kuna.catalog` → `docs/assertions.md`; `kuna.decompile
--option/--kassert`; drift guard `kuna.catalog --check`).

The PHADE Ghidra dataset (`~/github/PHADE/issue_dataset/ghidra/`) has **58 reproduced
issues**; **11 done**, **3 explored/deferred** (GH-28 harness-divergent, GH-8748 partial
needs SSA surgery, GH-8772 already-fixed), **46 remaining reproduced**. By primary stage
(legacy 19-stage numbering in the dataset → our model):

- **Frontend decode/lift — old 00/01/02/03 (~27 issues): mostly NOT our pattern.** These
  are missing-instruction / wrong-p-code / new-arch-support bugs. Some are real SLEIGH
  **spec fixes** (like the V850.cspec `ctbp`→`CTBP` we did — a legit, valuable bucket);
  many are genuine new-instruction-support that is out of scope. Triage decides which.
- **Analysis/backend — old 05/06/07/08/09/10/12/13/14 (~19 issues): the sweet spot.**
  These map to S2–S9 sub-stage decisions exactly like the 11 we fixed.

## The proven per-issue workflow (test-based development — do NOT deviate)

For every issue: **write the failing test first, then make it pass, gated and parity-safe.**

1. **Triage** the issue against the kuna C++ harness: read
   `issues/<n>.json` + `bodies/<n>.md` + `repro/<n>.py`, obtain bytes (PHADE
   `binaries/<n>/`, or `repro/<n>.py` raw_hex + language_id, or compile-from-source for
   Tier C), and **reproduce the symptom in `decomp_dbg`** — quote the buggy C verbatim.
   Decide the bucket: stage-exposure / spec-fix / not-viable (with reason).
2. **Write the testcase first** in `tests/stages/gh<n>-<name>.xml` (the double-decompile
   pattern): pass-1 default asserts the **current** output, pass-2 with the
   option/kassert asserts the **fixed** output. The bug assertion must pass on the
   unfixed tree today (proves repro).
3. **Implement the fix gated default-off** (the golden rule): new
   `decompiler/cpp/kuna_<name>.{hh,cc}` (Rule/Action + Option), a default-false
   `bool` flag on `Architecture`, body begins `if (!data.getArch()->flag) return 0;`.
   Minimal `(kuna)`-marked anchor edits. **Allocate a 4000+ ElementId** for the option
   (next free: **4010** — `OptionDatabase::registerOption` keys on `ElementId::find`, so
   options without ids silently collide). Add a `kuna_stages.cc` `surfaceTable` row AND a
   `settableTable` catalog row (values, shipped default, destructive flag, stage,
   substage, strength, rewind, issue, summary, use_when, example).
4. **Gate**: `make binaries` → testcase passes both assertions in `decomp_test_dbg` →
   `python -m kuna.run_tests --all --baseline docs/baseline.json` = **PARITY OK** →
   `make test-stages` → regenerate `docs/baseline-stages.json` →
   `python -m kuna.catalog --check` = OK.
5. **Commit per issue** (repo message style; `Co-Authored-By` trailer). Update
   `UPSTREAM.md` *Divergence* rows for any anchor edits.

**Default-on policy (DIV pattern):** ship the fix off by default first. If it is a clear,
non-destructive improvement, you may flip it on by default like DIV-2 — but only after
the ablation (`flip default → full suite → count changed assertions`) shows the corpus
effect is text-only or nil; record it in `docs/divergences.md`, re-pin the changed
upstream datatest assertions in place, and regenerate `docs/baseline.json`. Destructive
or context-dependent decisions (per the ablation) stay opt-in with
`destructive_as_default: true` in the catalog.

**Scaling:** triage the whole pool in one parallel `Workflow` (worktree isolation off —
read-only); implement the analysis/backend fixes in a parallel **worktree-isolated**
`Workflow` (each agent gated default-off so parity is structural), then replay the
worktree diffs sequentially into `master` with per-issue gates. This is exactly the
flow that landed the last 6 — see the prior workflow scripts under the session's
`workflows/scripts/` if useful.

**Where the conventions already live:** `CLAUDE.md` (build/test/Python), `docs/stages.md` +
`docs/stage-model.md` (the model + §13 symptom→sub-stage navigation), `docs/stage-mapping.md`
§0 (file→stage), `docs/stage-implementation.md` (per-stage changelog),
`docs/divergences.md` (DIV-1/DIV-2 + the default-flip recipe), `docs/assertions.md` +
`kuna_stages.cc settableTable` (the catalog), `tests/stages/README.md` + the existing
`gh*.xml` (testcase templates), `UPSTREAM.md` *Divergence*.

---

## PROMPT (paste this to start the session)

We are extending the kuna stage-model work to **all remaining reproducible Ghidra issues**
in the PHADE dataset, using strict test-based development. Read `docs/next-session-plan.md`
(this file) top to bottom first, plus `CLAUDE.md`, `docs/stages.md`, `docs/stage-model.md`,
`docs/divergences.md`, and skim `docs/stage-implementation.md` and `tests/stages/*.xml`.

Goals:
- [ ] Triage every remaining reproduced PHADE issue (the 46 not yet done/deferred; the
      reproduced set is `repro.status == "reproduced"` in
      `~/github/PHADE/issue_dataset/ghidra/issues/*.json`). For each: reproduce in
      `decomp_dbg`, quote the bug, and bucket it as **stage-exposure** (our pattern),
      **spec-fix** (SLEIGH/cspec, like V850.cspec), or **not-viable** (with reason —
      e.g. needs new-instruction support, or only manifests in the Java/GUI front end).
- [ ] Fix every stage-exposure and spec-fix issue via the per-issue TDD workflow above:
      failing testcase first, then a default-off-gated `kuna_<name>` fix (ElementId 4010+),
      `surfaceTable` + `settableTable` catalog rows, PARITY OK + test-stages + catalog
      `--check` gates, one commit each. Flip clearly-non-destructive fixes on by default
      (DIV-N) only after the ablation, recording each in `docs/divergences.md`.
- [ ] Keep `python -m kuna.run_tests --all --baseline docs/baseline.json` at PARITY OK and
      `python -m kuna.catalog --check` green at every commit; grow `tests/stages/` (and
      `docs/baseline-stages.json`) with one testcase per fixed issue.
- [ ] Update the living docs as you go: `docs/stage-implementation.md` (per-stage),
      `docs/stage-critique.md` (per-issue stage-fit + any new ablations),
      `docs/stage-mapping.md` if new files land, `UPSTREAM.md` *Divergence*, `PROGRESS.md`
      session block, and regenerate `docs/assertions.md` (`python -m kuna.catalog
      --markdown`).
- [ ] Maintain a running ledger (a checklist in `PROGRESS.md` or a new
      `docs/issue-coverage.md`) of every reproduced issue → bucket → status →
      testcase/commit, so "all remaining" is verifiable, not vibes.

Start by running the triage workflow over all remaining reproduced issues (parallel,
read-only), then work the stage-exposure and spec-fix buckets in difficulty order
(bank easy analysis/render wins first; group same-sub-stage issues). Use the
worktree-isolated `Workflow` pattern for the hard implementations and replay diffs
sequentially with per-issue gates. Be honest in the ledger about not-viable issues —
a precise "needs Tensilica instruction support, out of scope" is a real result. Commit
at every issue; never `git add -A` (a pre-existing `D PORT_PROBLEMS.md` may be present —
stage paths explicitly). Work on `master`.
