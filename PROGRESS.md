# kuna Progress Log

## Session (2026-06-09) — angr-style default naming (`option namestyle`, DIV-5)

Re-skinned the decompiler's default output to read like the angr decompiler, behind a
single master toggle `option namestyle angr|ghidra` (default `angr`; `ghidra` restores
upstream naming byte-for-byte).

**What changed (default output):**
- locals/temps/`extraout_*`/`unaff_*`/`in_*` → `v1, v2, v3 …`; arguments → `a0, a1 …`.
- global data (`<type>Ram<addr>`, volatile annotations) → `dat_<addr>` (lowercase, no
  leading zeros, no `0x`); unnamed functions `func_0x…` → `sub_<addr>`; labels
  `code_r0x…` → `label_<addr>`.
- each **local declaration** gets a trailing source-location comment: `// rax` (register)
  or `// stack - 0x10` (frame-relative). Real/recommended names are preserved.

**Mechanism.** New `kuna_naming.{hh,cc}` (`OptionNameStyle`, helpers, ElementId 4020) +
a per-`Architecture` flag `name_style_angr` (default on). Naming intercepted in
`Scope::buildDefaultName` (aN/dat_/vN) and the `persist` arm of
`ScopeInternal::buildVariableName`; functions in `Architecture::nameFunction`,
`PrintC::genericFunctionName`, `FspecSpace::printRaw`; labels in `PrintC::emitLabel`;
the comment in `PrintC::emitVarDeclStatement`; volatile data in `PrintC::pushAnnotation`.
The cross-function default-name guard (`ActionNameVars::makeRec`) now uses
`kunaIsGeneratedName` (recognises `param_N` + `aN`/`vN`).

**Tests.** Re-pinned **185/675** datatest assertions across 50 files to the new default
names (+ `pointerrel.xml` script `rename`/`retype` names), and **25** stages assertions
across 17 `gh*.xml` files — all via parallel sub-agent workflows that dump each test's new
output (`KUNA_DUMP=1`) and verify zero failures per file. New `tests/stages/namestyle.xml`
decompiles a loop (unnamed-helper call + global write) once at the default and once under
`option namestyle ghidra`, pinning both the angr names and the restored Ghidra names — the
"it goes away when you flip the scheme" proof. **PARITY OK: 204/204 unit + 675/675
datatests; stages 141/141; catalog OK.** `docs/divergences.md` DIV-5, `docs/baseline-stages.json`
→ 141 keys, `docs/assertions.md` regenerated, `UPSTREAM.md` *Divergence* updated.

## Session (2026-06-08) — port angr's LoweredSwitchSimplifier (`option loweredswitch`)

Implemented a port of angr's `LoweredSwitchSimplifier` (SAILR, USENIX Security 2024):
reconstruct a GCC-lowered comparison cascade (a binary-search `if/else` tree over one
variable) back into a C `switch`. Validated on coreutils `fmt`/`main`.

**Result.** With `option loweredswitch on`, `fmt`/`main` renders `switch(iVar5)` with **all
9 getopt cases + `default`** (0x70 'p', 0x74 't', 0x75 'u', 0x77 'w', 0x73 's', 0x67 'g',
0x63 'c', and the two negative options -0x83/-0x82), correctly nested inside the getopt
`while (... != -1)` loop — matching angr's expected output for
`test_reverting_switch_clustering_and_lowering_fmt_main`. **DIV-4 default-on** and still
**PARITY OK: 204/204 unit + 675/675 datatests, catalog OK** — see "Default-on flip" below.
New committed testcase `tests/stages/ghangr-loweredswitch.xml` (7 assertions, pass-1 sets the
option off; `docs/baseline-stages.json` → 128 keys).

**Default-on flip (DIV-4).** Flipping `recover_lowered_switch` on naively regressed **10/675**
upstream assertions: the pass also converted *hand-written* linear `if/else-if` chains
(`elseif.xml` ×6, `copytrim.xml` ×3, `partialunion.xml` ×1) to switches. Discriminator added:
**require the GCC binary-search structure** — the cascade must contain ≥1 range/`jle` split
(`sawRange` in `recoverCascade`). A purely linear equality chain is a hand-written `if/else-if`
and is left alone; a compiler-lowered switch has the balanced range-split tree. With the guard
the ablation is **0/675 changed** and `fmt`/`main` still recovers. `option loweredswitch off`
restores the upstream rendering. (`docs/divergences.md` DIV-4.)

**Methodology extended to all of angr's SwitchLowering testcases.** Ran every angr
`LoweredSwitchSimplifier` testcase through kuna (default-on) and recorded the outcome:

| angr testcase | binary | kuna result |
|---|---|---|
| `..._fmt_main` | `fmt` (PIE) | ✅ **full** — `switch` + 9 cases + default. Committed: `ghangr-loweredswitch.xml`. |
| `..._mv_o2_main` | `mv_-O2` (PIE) | ✅ **partial** — `switch` + 9 explicit cases + default; 3 special-return values (110, −131, −130) fold into the default's getopt loop. Committed: `ghangr-loweredswitch-mv.xml`. |
| `..._stat_human_fstype_*` | `stat.o` | ⚠️ **capped** — a 68-case sparse binary-search tree; the synthesized switch makes Ghidra's structurer emit an unstructured (`t_multigoto`) switch and abort with *"Case block has become detached from switch"*. Now guarded by the **≤16-case fail-safe cap**, so it falls back to the if-chain (no crash). |
| `..._cksum_digest_print_filename` | `cksum-digest.o` | ⚪ **n/a** — the cascade is a *linear* equality chain (10/13/92), which the binary-search guard correctly skips; the raw `.o` also has unapplied relocations. |
| `..._filename_unescape` | `b2sum-digest.o` | ⚪ **n/a** — not a ≥3-case single-variable cascade in Ghidra's SSA view (the chars are reloaded); ON==OFF. |
| `..._cat_main` (+ `_no_endpoint_dup`) | `cat.o` | ⚪ **n/a** — `main` is in `.text.startup` at section-offset 0 and collides with `.text` at vaddr 0 when kuna loads the raw `.o`, so the wrong function decompiles; also a real `.rodata` jump table. |

**Two findings worth recording.**
1. **Harness limitation, not a feature limitation:** kuna's `.o`-loading / bytechunk path does
   not apply ELF relocations or resolve section overlaps, so the four relocatable-`.o` angr
   testcases (`cksum`/`b2sum`/`cat`/`stat`) cannot be reproduced as committed kuna datatests the
   way the linked PIEs (`fmt`, `mv`) can. Applying relocations on `.o` load is the prerequisite
   to porting them.
2. **Structurer ceiling:** very large synthesized switches (e.g. `stat`'s 68-case tree) exceed
   Ghidra's `CollapseStructure` ability to form a clean `BlockSwitch` and abort the function. The
   ≤16-case cap keeps the default-on pass fail-safe; lifting it needs structurer robustness work
   (handle a `t_multigoto` switch block / detached case gracefully). All angr SAILR examples are
   ≤11 cases, so the cap does not affect them.

Committed second testcase: `tests/stages/ghangr-loweredswitch-mv.xml` (`mv_-O2`/main, PIE; 6
assertions; `docs/baseline-stages.json` → 134 keys). The general/native angr switch tests
(`test_decompiling_switch0/1/2`, `..._abnormal_switch_case_*`, etc.) are **real jump-table**
switches recovered by Ghidra's existing `BRANCHIND` machinery, not comparison-cascade lowering,
so they are out of scope for `loweredswitch`.

**The architectural problem (what made this hard).** angr edits its *structured region
graph* (an S7 artifact) and emits a `SwitchCase` node. **Ghidra has no region-graph edit
hook**: a `BlockSwitch` is only ever produced by `CollapseStructure::ruleBlockSwitch` from a
`BlockBasic` flagged `f_switch_out`, which is set only for a `CPUI_BRANCHIND`-terminated
block backed by a `JumpTable` — i.e. a Ghidra `switch` is fundamentally an **S2 artifact**
recovered from a *lifted indirect jump*. When the compiler lowered the switch to comparisons
there is no BRANCHIND. So the port **synthesizes the S2 artifact** (a BRANCHIND + a
hand-built, pre-labelled JumpTable) from the control-flow pattern and lets the existing
structurer + printer emit the switch. This **inverts the usual S1→S2 dependency** (lifting
produces the BRANCHIND that S2 explains; here a late pass produces it from a CFG pattern) —
a stage-model feedback edge realized via the existing **restart (mechanism c)**:
- `ActionLowerSwitchDetect` (S2 `switch-model`, placed after `ActionSwitchNorm`): on the
  simplified CFG, cluster the cascade (Varnode-identity, range-node traversal, advancing past
  the `-1` EOF sentinel guard), record it in a survives-`clear()` sticky side table, request a
  restart.
- `ActionLowerSwitchInstall` (before `ActionHeritage`, gated to the pre-SSA window by
  `getHeritagePass()==0`): replay the recorded surgery — head CBRANCH → BRANCHIND(V), rewire
  out-edges to the case targets + default, build the labelled JumpTable — so the following
  heritage rebuilds phi over the corrected CFG. **No MULTIEQUAL/phi hand-patching** (the whole
  reason for the restart-based, pre-SSA strategy).

**Phase ideas this stressed / broke (documented).**
- The model assumed S2 jump-table recovery is *downstream* of S1 lifting. This pass makes S2
  the producer of a BRANCHIND that lifting never emitted — the S1→S2 inversion above. It is
  the first kuna divergence that *creates* a CFG artifact rather than gating an existing
  decision.
- It is the first kuna pass that performs **CFG surgery** (op replacement + edge rewiring +
  block removal). All prior divergences gate a rule/flag/label. This required the restart
  vehicle to stay SSA-safe.

**Limitations / what was not possible without more work (honest report):**
1. **Post-SSA CFG surgery is effectively off-limits.** Doing the rewrite directly on the
   simplified (post-heritage) CFG would require hand-repairing MULTIEQUAL input slots across
   multi-edge additions and block deletions — the exact bug class the restart avoids. The
   restart costs one extra decompilation pass per function.
2. **`maxrestarts == 1` for the `universal` group** (coreaction.cc). A function that *also*
   needs a genuine multi-stage-jump restart in the same invocation would lose ours (the switch
   then stays an if-chain). Because the hint is sticky, a second decompilation recovers it;
   raising the budget to 2 is a one-line global change if co-occurrence proves common.
3. **`JumpTable::foldInGuards` is called unconditionally** even for a fully-labelled table, so
   a synthetic table needs a non-null model. It must be a **non-override** model
   (`JumpModelTrivial`): `clearJumpTables()` *preserves* override tables across a restart's
   `clear()`, and `followFlow` then re-`switchOver`s the stale table and segfaults. Required a
   1-line `(kuna)` `JumpTable::kunaSetTrivialModel` member (no public model setter existed).
4. **MVP scope (conservative, fires on fmt/main):** single switch variable; pure
   comparison-spine blocks only (case bodies/default fail the purity check and bound the
   cascade as leaves); distinct case targets; ≥3 cases / ≥2 distinct targets; the `-1` EOF
   sentinel is special-cased as the loop guard. angr's Type-A "extra statements before the
   comparison" handling and shared-target (fall-through) cases are not yet supported.
5. **Bytechunk regression test needs the binary's neighbours mapped + named** to bound main's
   flow (it falls through past its own end after a `call error`), exactly as `read symbols`
   does for the whole ELF; only `main` is decompiled.

Anchor edits (all `(kuna)`-marked, in `UPSTREAM.md` *Divergence*): `funcdata.hh`,
`jumptable.hh`, `architecture.{hh,cc}`, `options.cc`, `coreaction.cc`, `kuna_stages.cc`,
`kuna_restartlog.{hh,cc}`. New files: `kuna_loweredswitch.{hh,cc}` (ElementId 4019).

---

## Session goals (2026-06-08) — extend stage-model fixes to ALL remaining reproduced PHADE issues

- [x] Triage every remaining reproduced PHADE issue (46) — reproduce in `decomp_dbg`,
      bucket as stage-exposure / spec-fix / not-viable, localize to a stage. Durable ledger
      in `docs/issue-coverage.md`.
- [x] Fix every viable stage-exposure and spec-fix via per-issue TDD (failing testcase →
      gated fix → catalog/parity/test-stages gates → one commit each). PARITY OK and
      `catalog --check` green at every commit.
- [x] Flip the ablation-clean fixes default-on (DIV-3) after the full-suite ablation.
- [x] Update living docs + ledger; one testcase per fixed issue in `tests/stages/`.

### Results (2026-06-08)

- **26 of 46 issues fixed** (one commit each, full parity 204/204 + 675/675 at every commit):
  - **16 via SLEIGH spec-fix**: 9001 (HCS12 BRN), 1243 (8051 ADDC carry), 7418/7139/8790/9184
    (x86 rel16 mask / disp16 sign / prefetch reg-form / FMA YMM256), 5897/8456/8391/7890
    (ARM cpy→mov / MRS-banked / Thumb CDE mrrc / VFP vcvt), 1951+8844 (avr8 branch wrap),
    4788 (V850 RH850 Bcond), 6389 (8085 undoc), 7451 (RISC-V Zfa fli.s), 6904 (PPC e500 mr).
  - **10 via option-gated stage-exposure** (ElementIds 4010–4018): 1276+8777 `flagcompare`,
    9191 `switchmodbound`, 8017 `stackprobeloop`, 8467 `dynamichashmax`, 8500 `stackalias`,
    8724 `arraystride`, 6882 `sparcstructret`, 9203 `condexeplace`, 9218 `inputvarnodeadjust`.
- **DIV-3**: six corpus-clean non-destructive options flipped default-on (ablation: 0/675
  upstream assertions changed) — `flagcompare`, `stackprobeloop`, `dynamichashmax`,
  `arraystride`, `condexeplace`, `inputvarnodeadjust`. The three destructive ones
  (`switchmodbound`, `stackalias`, `sparcstructret`) stay opt-in. (`docs/divergences.md`.)
- **13 not-viable** (precise reasons in the ledger): 809, 1708, 2033, 6333, 6342, 6664, 6682,
  6723, 6836, 7377, 8028, 8694, 8794 — plus 3 reclassified spec→not-viable (766, 5666, 7332).
- **2 already-correct**: 3847 (FYL2X) and 3723 (ARM `udf`, misfiled as AArch64).
- **2 deferred** (attempted, non-convergent): 6674 (V850 free-register switch — distinct from
  the x86 LOAD-table case `switchmodbound` fixes), 6858 (stripped-PIE main — distinct from
  8017's gcc probe loop; full-function testcase aborts the harness).
- **Tooling**: `KUNA_DUMP` env var on the test harness (echoes captured console output;
  default-off, zero behavior change) — the triage/repro substrate for raw-byte issues.
- **Method**: read-only triage workflow + worktree-isolated implementation workflows
  (spec-fix and stage-exposure lanes), then serial replay on `main` with the binding
  parity/catalog/test-stages gate per commit. `make` parallelism pinned to `NJOBS=4`
  (an unbounded `-j` OOM-killed a mid-session build).
- `tests/stages/` grew 48 → 121 assertions; all 46 remaining PHADE issues are now terminal.

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
  roadmap #4). `docs/stage-mapping.md` §0 remaps all 115 files to P0/S1–S9.
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
- **Addendum (2026-06-07, user decision)**: **DIV-2** — the eight non-destructive fixes
  are now ON BY DEFAULT (compareform=original, arraynotation, thumbfuncptr,
  inferfuncentry, booleanmask, addcarrychain, ovlesssimplify, memsetrecover with a new
  >=2-COPYs/>=16-bytes fill guard). `returnpair single` and `v850indirectbranch` stay
  opt-in (ablation-proven destructive). 22/675 upstream datatest assertions re-pinned
  in place to the new renderings (all compareform/arraynotation text forms; 15 files,
  updated+verified by parallel subagents); both baselines regenerated; the nine
  affected stage testcases inverted (default = fix, `option ... off` = old behavior).
  Full record: `docs/divergences.md` DIV-2. Gates: 204/204 + 675/675 PARITY OK (kuna
  oracle) + test-stages 43/43.
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
