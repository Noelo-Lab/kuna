# w10-dominant-copy
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent)
date: 2026-06-15
gate: KUNA_ENGINE=rust decomp_test_dbg datatests -> 48/456 assertions pass (base rust-port: 47/456), exec-failures flat at 28/456; the one new pass is condconst "Conditional Constant #10" (`v1 = x;`), the dominant-copy hoist's product.

## Scope

`Merge::buildDominantCopy` (merge.cc:1151-1238) + `Merge::processHighDominantCopy`
(1316-1337) + the `ActionMergeCopy`/`ActionDominantCopy` wrappers (coreaction.hh:388,
1018; apply bodies `mergeOpcode(CPUI_COPY)` / `processCopyTrims`). NOTE: the prompt's
`markInternalSingleCopies` does not exist at this Ghidra rev; the in-scope marker is
`markInternalCopies` (merge.cc:1444) → `processHighRedundantCopy`, which was already
ported on the base. The DIFF adds only: `build_dominant_copy_impl` (the IR-surgery body),
the persistent `Funcdata::covermerge` + `with_covermerge` wiring, `Cover::add_ref_point_for`,
and wires the four merge-action wrappers to drive over the persistent engine. The chain
functions (`process_copy_trims`, `process_high_dominant_copy`, `find_all_into_copies`,
`compare_copy_by_in_varnode`, `mark_redundant_copies`, `check_copy_pair`) already existed
on the base and were re-read for faithfulness.

## Faithfulness (FOCUS 1) — CONFIRMED

`build_dominant_copy_impl` is a faithful line-by-line transcription of `buildDominantCopy`:
same `findCommonBlock` hoist (find_common_block_set, block.rs:1359, faithful to the
multi-block `FlowBlock::findCommonBlock`), same `bCover` (skip COPY instances whose in0
copyShadows rootVn — `varnode_copy_shadow` faithful to varnode.cc:996), same `aCover`
(def at domVn + ref points at each non-dominant outVn read), same `bCover.intersect(aCover)>1`
decision, same `count<=1` "don't bother" guard, same `totalReplace`+`opDestroy`+
`high->merge` redirect. The C++ `op->setMark()`/`isMark()`/`clearMark()` op-flag is a
function-local communication channel (set in loop1/count-guard, read+cleared in loop2, no
external reader) — the Rust `marked: Vec<bool>` reproduces it exactly. The action wrappers
match (`mergeOpcode(CPUI_COPY)` / `processCopyTrims`). The persistent `covermerge` faithfully
models the single `Funcdata::covermerge` member whose `copyTrims` survive
`ActionMergeRequired`→`ActionDominantCopy`.

## No special-casing (FOCUS 2) — CONFIRMED

grep of the diff for address/offset/function-name hardcoding: NONE. The only `boolless`/
`condconst` mentions are in doc comments describing the testbed slices, never in logic. No
`if name==`/`if addr==`/`0x....` conditionals in added code.

## The hoist is REAL (FOCUS 3) — CONFIRMED by instrumented re-run

Instrumented `build_dominant_copy_impl` and ran the full datatest corpus through the Rust
engine: the hoist fires 3 times (size=2, size=3, size=2 groups) on genuine multi-COPY
trim groups — NOT dead-on-arrival. condconst_conn (the prompt's condconst_conn target):
its parameter-into-stack-local trim COPYs across predecessor blocks fold to ONE dominating
`v1 = x;` hoisted to the common dominator, rendering exactly as the C++ oracle's first body
statement. A NON-dominant set is correctly NOT hoisted: boolless's single-source body stays
byte-identical to the C++ B5 oracle (the `count<=1` guard + cover-intersection decline).
The `&&` short-circuit join the oracle shows (`if ((x == 0) && (y != 10))`) is NOT produced
— that is the still-dormant `BlockCondition`/`ruleBlockOr` structuring (LOSS-159), upstream
of render and NOT this item's responsibility. condconst_conn renders empty `if`/`else` arms
around `v1 = 0x14;` instead. This is the documented LOSS-160 restoration trigger landing its
dominant-copy half.

## New positive passes are REAL parity (FOCUS 4) — CONFIRMED

Per-assertion diff base→branch: exactly ONE flip, condconst "Conditional Constant #10"
(`v1 = x;`) FAIL→PASS, and the harness `print C` for condconst_conn genuinely contains
`v1 = x;`. The IR carries a real CPUI_COPY (anti-special-casing test asserts this), not a
printer string. Total: base 47/456 → branch 48/456. Execution-failure set ("Did not apply")
flat at 28 → 28 (no regression). C++ oracle untouched: 207/207 unit + 675/675 datatest,
PARITY OK (re-confirmed after all activity).

## boolless + readstruct (FOCUS 5)

boolless: byte-identical to the C++ B5 oracle base==branch (dedicated B5 test + the new
anti-mis-hoist test both pass; its high is not mis-merged). readstruct: there is no
`readstruct.xml` datatest at this rev (struct-reading datatests exist — elseif/nestedoffset/
pointerrel/etc. — none named readstruct); the prompt's "readstruct" is taken as the
struct-corpus, which is unaffected by the diff (the hoist only touches multi-COPY trim
groups, and the struct datatests' assertion pass-state is unchanged base→branch). cargo
test --workspace: ALL green. Execution-failure count: not regressed.

## Hunt list

- Signedness: clean. New loops use `int4` indices (`size>=2`, `pos>=0`, `i in 0..size`)
  matching C++ `int4`; `count` (int4) decrements at most `size` times, never below 0;
  `mark_redundant_copies` `j>=0` on signed i32 (-1 exits, faithful to C++ for(;j>=0;)).
- Integer widths: clean. `compareCopyByInVarnode` keys are `getCreateIndex()` (u64), block
  index (int4), `SeqNum::getOrder()` (uintm) — all matched. No promotion-sensitive arith.
- Wrapping: clean. No unsigned arithmetic on uintb-lineage values in the new code; only
  index arithmetic on bounded vectors.
- Comparator totality: clean. `compare_copy_by_in_varnode` is a strict-weak order (tertiary
  key `SeqNum::getOrder()` is unique → no ties). Adversarial test pins antisymmetry on
  colliding create-index. `sort_by` is stable but ties are impossible, so == C++ unstable sort.
- Iteration order: clean. `find_all_into_copies` iterates `high->getInstance(i)` order then
  sorts by the faithful comparator (Vec + sort_by, no HashMap/HashSet). `process_copy_trims`
  iterates `copyTrims` insertion order (`mem::take` of a Vec). No HashMap/HashSet in ported logic.
- Off-by-one/do-while: clean. `find_common_block_set` keeps the do-while at-least-once
  (loop+break). Group-walk `while pos+sz<len && next==in` boundary matches C++.
- Erase-while-iterating: clean. loop2 iterates `copy[]` by index (not the live list);
  destroyed ops are not revisited; `process_copy_trims` takes the trims vec first.
- Exception→Result partial-state: see F2 (minor). `?` propagation in build_dominant_copy_impl
  is discarded by `ActionDominantCopy::apply` (`let _ = ...`); C++ `processCopyTrims` is void
  and cannot error on this path, so no active divergence, but a mid-mutation Err would leave
  partial IR with no restart (latent).

## Mechanical pass

- port-audit: not present in this workspace (no `port-audit` crate) — N/A.
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`: none in production code;
  one `.unwrap()` in a unit test only.
- clippy `-p kuna-decomp -- -D warnings` (lib): CLEAN (0). The new test file is clippy-clean.
  Pre-existing `--tests` clippy errors (kuna-decomp 54, kuna-sleigh 10) and `unused import`
  warnings exist identically on the base rust-port — out of scope, not introduced here.

findings:
  - F1 (informational): `merge_two_highs` uses a FRESH-EMPTY local `HighIntersectTest`,
    where C++ `buildDominantCopy` passes `(HighIntersectTest *)0` (NULL) to `high->merge`.
    Behaviorally equivalent: `HighVariable::merge` skips `moveIntersectTests` on NULL; a
    fresh-empty cache's `moveIntersectTests` is a no-op (no edges to move). The persistent
    `covermerge.test_cache` is left untouched by both, matching C++ (stale-then-refresh).
    Faithful; noted for completeness, not a defect.
       cpp: decompiler/cpp/merge.cc:1236 (`high->merge(domVn->getHigh(),(HighIntersectTest *)0,true)`)
       rust: rust/crates/kuna-decomp/src/funcdata.rs:1849 (merge_two_highs, local cache)
  - F2 (low): the `?`-propagated errors in `build_dominant_copy_impl` are silently discarded
    by `ActionDominantCopy::apply` (`let _ = merge.process_copy_trims(data)`). C++
    `processCopyTrims` is void (no error path on the merged tree), so this is inert today, but
    an Err mid-mutation would leave partial IR with no restart/cleanup, unlike a C++ that
    cannot reach it. Latent, output-neutral on the corpus.
       cpp: decompiler/cpp/coreaction.hh:1023 (apply -> void processCopyTrims)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:792 (`let _ = merge.process_copy_trims(data)`)
  - F3 (style/low): `full_varnode_cover` REBUILDS each member cover fresh (`cover.rebuild`)
    where C++ reads the cached `vn->getCover()`. Defensible (the comment cites cache-dirtiness)
    and output-correct on the corpus (the merged-tree covers are current, so rebuild == cached),
    but it is a deliberate non-literal deviation; a stale-vs-rebuilt cover could in principle
    differ if the cache and graph ever disagree.
       cpp: decompiler/cpp/merge.cc:1194 (`bCover.merge(*vn->getCover())`)
       rust: rust/crates/kuna-decomp/src/funcdata.rs:1829 (full_varnode_cover, rebuild)
  - F4 (style/nit): bare `as usize` index casts in `build_dominant_copy_impl` (`(pos+i) as
    usize`, `size as usize`, `i as usize`) carry no `// cast:` justification. Provably
    non-negative (caller guards size>=2, pos>=0, i in 0..size), matching C++ int4 indexing;
    correctness-neutral, flagged only against the protocol's bare-cast rule.
       cpp: decompiler/cpp/merge.cc:1155-1234 (int4 indexing)
       rust: rust/crates/kuna-decomp/src/funcdata.rs:1697-1799

adversarial tests (5, committed to the branch):
  - rust/crates/kuna-decomp/tests/verify_w10_dominant_copy.rs
      * w10_dc_condconst_conn_hoist_is_real_v1_eq_x — e2e via the Rust decomp_test_dbg: the
        dominant COPY hoists to `v1 = x;` (Conditional Constant #10), backed by a real CPUI_COPY.
      * w10_dc_boolless_not_over_merged_byte_identical — anti-mis-hoist: boolless byte-identical
        to the C++ B5 oracle (a non-dominant single-source body is NOT folded).
      * w10_dc_condconst_conn_distinct_sources_both_survive — per-source grouping: `v1 = x;`
        AND `v1 = 0x14;` both survive (distinct sources are two groups, not collapsed).
  - rust/crates/kuna-decomp/src/merge.rs (unit):
      * w10_dc_compare_copy_total_order_on_colliding_create_index — comparator antisymmetry on
        equal create-index across distinct input Varnodes.
      * w10_dc_grouping_keys_on_varnode_identity_not_create_index — the group boundary keys on
        source-Varnode identity, not create-index.

losses: LOSS-161 (the `&&`-structuring render is not realized — dormant LOSS-159
`BlockCondition` path — and the wave's "closes the conditional-structuring cluster" framing
is an over-claim; the dominant-copy COPY-consolidation half IS closed and exercised, +1 real
positive). Cross-refs LOSS-160 (this is its declared restoration trigger landing its
dominant-copy half).
