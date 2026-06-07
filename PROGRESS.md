# kuna Progress Log

## Session goals (2026-06-06/07) — stage-model physicalization: split the decompiler into stages, fix 10 PHADE issues

- [x] Address each stage: implement the model physically (registry + console + assertions
      + observability); per-stage changelog in `docs/stage-implementation.md`
- [x] Verify all testcases pass after every change (PARITY OK at every one of the
      session's commits; `make test-stages` grown 4 → 43 assertions)
- [x] Critique the stages empirically (`docs/stage-critique.md`: per-issue stage-fit,
      ablation matrix, §13-navigation score, per-stage verdicts)
- [x] Fix + reproduce ≥10 PHADE issues through the stage model (10 fixed: GH-2786, 8471,
      6930, 6990, 1282, 7190, 8817, 8913, 9230, 1537)

## Results (2026-06-06/07)

- **Infrastructure** (each component a commit, PARITY OK gated): `kuna_stages` (stage
  registry: group→stage map, 40-row sub-stage catalog, surface routing — queryable via
  `stage list/map/status`); `kuna_console` (self-registering `IfaceKunaCapability` —
  console commands with ZERO upstream anchor edits); `kuna_assert` (`kassert
  <stage> <substage> ...` typed assertion API over Override/SymbolDB/proto/options, with
  computed minimal rewind scopes REPORTED per stage-model §12); `kuna_restartlog`
  (mechanism-c restart reasons recorded at all 5 trigger sites; the switchmulti
  multistage restart is now visible); `pipeline` (reduced-pipeline sub-queries,
  mechanism c′ at the console); `quality` (goto-count metric — the measurement half of
  roadmap #4). `STAGE_MAPPING.md` §0 remaps all 115 files to P0/S1–S9.
- **10 issues fixed through the model**, all reproduced+fixed in `tests/stages/` (each
  testcase asserts bug-under-default AND fix-under-assertion): 1 default-flip (GH-2786,
  invalid `--x` C; DIV-1, 0/675 upstream churn), 8 option-gated exposures (S2..S5,
  options `thumbfuncptr inferfuncentry returnpair booleanmask ovlesssimplify
  v850indirectbranch addcarrychain memsetrecover`, ElementIds 4002–4009), and 1 pure
  ROUTING fix (GH-1537 fixed by GH-9230's option with zero new code — exposed decisions
  generalize). Plus `V850.cspec` `ctbp`→`CTBP` case fix (V850 could not load at all in
  the case-sensitive C++ decompiler).
- **Method**: 12-issue triage + 6 hard implementations ran as parallel worktree-isolated
  subagent workflows; every fix gated on a default-off Architecture flag
  (`if (!flag) return 0;`) so upstream parity is structural, not tested-for. Worktree
  diffs replayed sequentially into master with per-issue gates.
- **Key empirical findings** (full critique in `docs/stage-critique.md`): symptom-stage ≠
  decision-stage (GH-8471/6930 look S9, live in S5 — the model's strongest validation);
  ablation matrix separates text-only (S9) from analysis-bearing (Band B) flips, with
  `returnpair single` breaking exactly the 3 upstream tests that need the pair (the
  definition of an assertion-worthy decision); GH-8748 negative result — else-if tail
  duplication is blocked by un-clonable live-out INDIRECT phis, scoping what
  quality-gated structuring (roadmap #4) actually requires.
- **Honest assessment**: defaults are byte-identical to upstream except DIV-1
  (`docs/divergences.md`); 9 of 10 fixes are opt-in assertions, so nothing is "fixed"
  for a user who never sets an option — the deliverable is named, tested, durable
  decision surfaces (and one real bug fix). S6/S7 attracted no issues (dataset bias);
  GH-28 deferred (repro diverges between debug/test harness builds); GH-8748 partial.
- **Engineering hazard fixed**: `OptionDatabase::registerOption` keys on
  `ElementId::find(name)` → options without registered ElementIds silently collide on
  `ELEM_UNKNOWN`; all kuna options now allocate 4000+ ElementIds.

## Session goals (2026-06-06) — stage-model prototype: fix a real issue via finer-grained stages

- [x] Pick a reproducible, dataset-backed Ghidra issue convertible to a kuna testcase
      (chose **GH-558** from `~/github/PHADE/issue_dataset`: 58 reproduced issues triaged
      by an 18-agent workflow; GH-558 = pure representation policy, zero plain-bug risk)
- [x] New testcase family for stage-model issue testcases: `tests/stages/` +
      `make test-stages` + `docs/baseline-stages.json` (upstream baseline untouched)
- [x] Map the issue to the stage model and implement the fix as finer-grained stages
- [x] Verify: testcases pass *because of* the stage split; upstream PARITY OK throughout
- [x] Writeup: `docs/prototypes/gh558.md`

## Results (2026-06-06)

- **First intentional divergence from upstream** (authorized): vendored-file edits are
  now permitted; tracked in `UPSTREAM.md` *Divergence*. kuna decompiler additions live
  in `decompiler/cpp/kuna_*.{cc,hh}` (auto-linked via the upstream Makefile's wildcard).
- **GH-558 fix 1 — `compareform`:** the `V <= c => V < c+1` canonicalization split into
  `canonicalcompare` (Band B, named group) + `presentcompare` (`ActionPresentCompareForm`
  at the S8→S9 boundary), connected by a provenance bit set in the single primitive
  `Funcdata::replaceLessequal`. P0 assertion: `option compareform canonical|original`.
- **GH-558 fix 2 — `arraynotation`:** standalone `PTRADD` rendering (`base + index` vs
  `&base[index]`) exposed as `option arraynotation on|off` in `PrintC::opPtradd`.
- **Key discovery:** the canonicalization decision fires from *three* sites across two
  stages — the rule pool AND the structuring-time branch-flips (`opFlipInPlaceExecute`/
  `opNormalizeFlip`); per-rule toggles can't reach it, and a cleanup-pool inverse runs
  too early. The working placement (after the last flip pass, before prototype/cast
  fixation) shows the model's S8→S9 boundary is a physical program point.
- **Experiments:** forcing `compareform original` globally changes 12/675 upstream
  assertions (pure text); removing `canonicalcompare` changes a *different* 9/675
  (analysis effects) — empirical evidence the two sub-stages are distinct decisions.
- **Honest assessment:** no decompiler behavior was "fixed" — defaults are byte-identical
  (PARITY OK 204/204 + 675/675 at every milestone); the issue is resolved purely by
  exposing decisions. Caveat recorded: assertions are currently function/global-grain;
  per-op anchors (DynamicHash) are future work.
- **Tests:** `make test-stages` → 4/4 (`gh558-compareform.xml`, `gh558-arraynotation.xml`
  — the latter uses the issue's own attached binary; before/after `p->r + iVar2` →
  `&p->r[iVar2]`).

## Session goals (2026-06-05)

- [x] Find the Ghidra decompiler testcases (deep-decompiler only) and create a way to run them
- [x] Identify the C/C++ decompiler source in Ghidra and how to extract it
- [x] Extract it into kuna and wrap it in a simple Python interface (binary + function → C)
- [x] Port the testcases to the stripped-down decompiler
- [x] Record a procedure for porting future upstream Ghidra decompiler changes
- [x] Map decompiler source files to STAGES.md (document mismatches/new stages)

## Results

- **Tests:** `decompiler/unittests/` (204 C++ unit tests) + `decompiler/datatests/` (83 XML
  regression tests → 675 assertions). Java GUI tests excluded. Harness = upstream
  `decomp_test_dbg`, wrapped by `kuna.run_tests` (parses unit/stderr + data/stdout, parity diff).
- **Parity:** kuna build reproduces the pristine-ghidra baseline EXACTLY — 204/204 unit,
  675/675 datatest assertions, identical pass set. `docs/baseline.json` is the recorded oracle;
  `python -m kuna.run_tests --baseline docs/baseline.json` → **PARITY OK**.
- **Wrapper:** `python -m kuna.decompile <bin> <func|0xaddr> [--addr]` drives `decomp_dbg`
  (`load file`/`read symbols`/`load function|addr`/`decompile`/`print C`); C captured via
  `openfile write` so prompts never leak in. Verified on a compiled binary by name and address.
- **Upstream sync:** `tools/sync_upstream.py` (diff vendored paths → rewrite prefixes → git apply
  → bump GHIDRA_REV). Validated clean forward-apply + non-mutating dry-run on two historical revs.
- **Stage mapping:** `STAGE_MAPPING.md` places all 115 .cc exactly once across the 19 stages +
  infrastructure, anchored to `universalAction`. Documented gaps: stages 06 Dataflow, 10 Memory,
  12 Aggregate Types, 15 Interproc have no dedicated module (emergent / folded into others).

## Build note (libbfd without root)

`decomp_dbg`/`decomp_opt`/`decomp_test_dbg` link libbfd. The proper prerequisite is
`sudo apt-get install -y binutils-dev`. This session had no sudo, so libbfd was fetched into
`./.bfdlocal` via `tools/fetch_bfd.sh` and the build run as `make BFD_PREFIX=$(pwd)/.bfdlocal`
(the Makefile injects it through the upstream `ADDITIONAL_FLAGS`/`BFDLIB` without editing
upstream files). `sleigh_opt` and all 148 `.sla` build without libbfd.

## Log

### 2026-06-05 — All 18 PORT_PROBLEMS fixed
- sync_upstream.py hardened: --no-renames + core.quotepath=false diff, hard-fail
  (PatchRewriteError) on quoted headers / rename-copy / binary markers, dry-run is
  truly side-effect free, write_ghidra_rev verifies its substitution, analyze()
  reports vendored-only kuna paths, all-skipped diffs short-circuit cleanly,
  .kuna_sync.patch gitignored. Verified end-to-end on synthetic repos (boundary
  renames, unicode, new processor module) + real-data reverse-sync dry-run.
- Makefile: .NOTPARALLEL (P8 race), jobserver-aware SUBJOBS (P9), nproc fallback
  (P11), libbfd.so located by wildcard not triplet guess + validated in check-deps
  and fetch_bfd.sh (P10), make test builds binaries/specs only when missing (P12).
- run_tests.py: indented/`Execution failed` error forms parsed + deduped and force
  nonzero exit (P13), 128+signal exit on signal-killed harness (P14), duplicate
  test names disambiguated with " @dupN" in the pass keyset (P15; baseline
  regenerated -> 879 keys = 204 unit + 675 data, PARITY OK re-verified), run()
  rejects names with mode='all' (P16). decompile.py: SLEIGHHOME-vs--s comment (P17).
- Docs: UPSTREAM.md apply-strategy paragraph rewritten to match reality (P18),
  CLAUDE.md sync paragraph aligned. Resolution table appended to PORT_PROBLEMS.md.

### 2026-06-05 — Post-port audit (PORT_PROBLEMS.md)
- Full audit of the port for translation bugs: blob-hash sweep of all 1039 vendored
  files vs upstream @ GHIDRA_REV (byte-identical, modes too), fresh clean rebuild
  (148/148 specs), 204/204 + 675/675 PARITY OK, e2e decompile (name/addr/stripped +
  error paths), every decompile.py console command and run_tests.py regex verified
  against vendored harness sources, STAGE_MAPPING 115/115 + line anchors verified.
- **Vendored translation is clean — zero content bugs.** 18 issues found in kuna-owned
  glue, all adversarially verified and recorded in `PORT_PROBLEMS.md`: 2 major (both
  future-sync risks in sync_upstream.py: --dry-run mutates UPSTREAM.md on the empty-diff
  path; boundary-crossing renames corrupt/abort a sync), 13 minor, 3 info. None affect
  current build/test/decompile correctness.

### 2026-06-05 — Session start, exploration & plan
- Located decompiler source: `ghidra/Ghidra/Features/Decompiler/src/decompile/`
  (cpp: 115 .cc / 114 .hh; upstream Makefile builds `decomp_dbg`, `decomp_test_dbg`,
  `sleigh_dbg`, `ghidra_dbg`, libs). Pure C++11; deps: libbfd (console/test binaries
  only), libz; bison/flex outputs are committed upstream.
- Located the deep-decompiler-only tests: `unittests/` (7 files, 204 TEST()s, pure C++)
  and `datatests/` (83 XML regression tests with embedded binaries + regex assertions).
  Java-side tests (`src/test.slow/java/...`) are GUI/integration — explicitly out of scope.
  Harness: `decomp_test_dbg [-sleighpath D] [-path DIR] [unittests|datatests [names]]`,
  exit code = #failures.
- Key runtime fact: no compiled `.sla` exist in the repo — specs must be compiled with
  the `sleigh` binary (`sleigh_opt -a <dir>`).
- Plan approved: vendor decompile/{cpp,unittests,datatests} + ALL processor spec modules,
  zero-modification policy, top-level Makefile driver, Python package (`kuna`),
  `tools/sync_upstream.py`, STAGE_MAPPING.md. Upstream rev: cef869af04c474...1644.
- Scaffolding written (.gitignore, README.md, UPSTREAM.md, this file).
- Baseline build of the pristine tree started in /tmp/kuna-baseline (parity oracle).
