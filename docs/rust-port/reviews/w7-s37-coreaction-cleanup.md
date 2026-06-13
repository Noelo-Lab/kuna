# w7-s37-coreaction-cleanup
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-13
round: 1

gate: `cd rust && cargo test -p kuna-decomp` -> item surface green
  - 1273 lib tests pass (incl. the 6 `coreaction_cleanup::tests`), 5 catalog_bytecompat,
    4 corpus_bootstrap, + all sibling verifier tests; the 4 new
    `verify_w7_s37_coreaction_cleanup` tests pass.
  - The ONE failure is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`: it needs a
    built `x86:LE:64` `.sla` (a gitignored `make specs` build artifact; only Toy `.sla`
    exist in this checkout). It is a pre-existing environment failure in a file this item
    does not own (`corpus_bootstrap.rs`), unrelated to `coreaction_cleanup.rs`.

## Scope

The item's HEAD commit (274cfb9) adds exactly `rust/crates/kuna-decomp/src/coreaction_cleanup.rs`
(1050 lines) + one `pub mod` line in `lib.rs`. The other files in the
`rust-port...branch` diff (blockaction.rs, merge.rs, cover.rs, variable.rs, funcdata.rs,
varnode.rs) arrive from MERGED sibling branches (w7-s6-merge, w7-s7-blockaction,
w7-s6-variable-cover) with their own verdict files and are out of scope here.

cpp_blob_sha verified: `coreaction.cc` = 28f6157f… and `coreaction.hh` = bc278439…
match the checklist record exactly. No blob drift.

This is a SEAM port: 13 of 14 `Action` `apply` bodies are no-ops returning 0 with the
C++ body carried as commented pseudocode; only `ActionAssignHigh` is realized. The
verification therefore re-derives (a) the `name()`/group/`flags` strings vs the C++
constructors, (b) the faithfulness of every transcribed pseudocode body, (c) the
`merge_actions()` schedule order vs `universalAction`, and (d) the one realized body.

## Hunt list

- Signedness: clean — no integer comparisons in the item file; all `apply` bodies
  return a literal `0`. The one realized body (`set_high_level`) is owned by a merged
  dependency (out of scope).
- Integer widths: clean — no integer arithmetic in scope. `ApplyResult`/`count` widths
  are owned by `action.rs` (`count: int4`), unchanged.
- Wrapping: clean — no arithmetic on `uintb`-lineage values in scope.
- Comparator totality: clean — no comparators defined in this item.
- Iteration-order provenance: clean — `merge_actions()` is a fixed `vec![]` literal in
  C++ `universalAction` order (coreaction.cc:6002-6028); no HashMap/HashSet anywhere
  (grep = 0). The transcribed (commented) loops record their C++ order (def-set / loc-set
  / basic-block-dominance / multlist index) but do not execute.
- Off-by-one / do-while / reverse iterators: the seamed `ActionMarkImplied` body is a C++
  `do {} while(!varstack.empty())` depth-first walk — transcribed as commented pseudocode
  with the at-least-once `do` preserved and the `desciter++` post-increment noted; it does
  not execute, so no Rust loop to mis-port. Re-derived against cc:3519-3558: faithful.
- Erase-while-iterating: clean — no live container mutation in scope (the seamed
  `clearMark` sweeps are commented, not run).
- Exception -> Result parity: clean — no `?`/throw sites; every body returns `0`
  infallibly, matching the C++ `int4 apply` that returns 0.

## Re-derivation of the transcribed bodies (all faithful)

- `ActionMarkExplicit` rust:235-263 vs cc:3340-3375 — maxref/`beginDef(0)` cut, the
  `desccount < 0` / `< -1` / `> 1` branches, `count += multipleInteraction`, the
  `isMark()` guard, the `clearMark` sweep: all present and ordered correctly.
- `ActionMarkImplied` rust:300-326 vs cc:3519-3558 — isFree/isExplicit/isImplied skips,
  the do-while DFS, `count += 1`, `checkImpliedCover`->`setExplicit` else `markImplied`,
  the outvn null/explicit/implied guard: faithful.
- `ActionNameVars` rust:720-746 vs cc:3076-3098 — linkSymbols / recoverName… /
  lookForBadJumpTables / lookForFuncParamNames, the `base=1` rename loop, assignDefaultNames:
  faithful.
- `ActionSetCasts` rust:787-814 vs cc:2812-2872 — dominance-ordered blocks, block-ordered
  ops, notPrinted/CPUI_CAST skips, PTRADD/PTRSUB rewrite, resolveUnion / outHighType
  resolveInFlow / castInput / LOAD-STORE checkPointerIssues / castOutput: faithful.
- `ActionHideShadow` rust:623-641 vs cc:5085-5104 — `endDef(written)`, the mark-dedup
  `hideShadows`+`count += 1`+`setMark` loop, the `clearMark` sweep: faithful.
- The 8 one-line `getMerge()`/`markIndirectOnly` delegations vs coreaction.hh
  (370/404/393/1023/382/415/1034/358): each comment matches the `.hh` body verbatim.

## Class-shell fidelity (realized + verified)

- All 14 `name()` strings, `rule_onceperfunc` flags, and `clone(grouplist)` group filters
  match the C++ constructors (coreaction.hh:331/342/353/365/377/388/399/410/441/462/483/
  1007/1018/1029). `rule_onceperfunc` = 8 matches the C++ enum.
- `merge_actions()` reproduces the C++ relative schedule order of the merge/casts leaves
  (cc:6002-6014, 6027, 6028), correctly excluding the interleaved non-merge actions
  (ActionDynamicSymbols@6009/6026, ActionLateDoNothing@6015, ActionBlockStructure@6016…,
  ActionMapGlobals@6025). The lone `"casts"` group on `ActionSetCasts` is preserved.
- `ActionAssignHigh::apply` (the one realized body) is `data.set_high_level(); 0` —
  exactly the C++ inline `{ data.setHighLevel(); return 0; }`; no `count += 1`
  (matches C++). `set_high_level` is a merged-dependency primitive (out of scope), its C++
  is idempotent (`if (highlevel_on) return;`), and the test confirms the idempotence.
- `clone_filtered` copies the whole `ActionBase` (`self.base.clone()`) — identical to the
  established `coreaction_early`/`coreaction_protos` convention (clone happens at database
  setup, counters are 0); not an item-specific divergence.

## Mechanical pass

- greps on `coreaction_cleanup.rs`: `todo!`=0, `unimplemented!`=0, `HashMap`=0,
  `HashSet`=0, `sort_unstable`=0, `partial_cmp`=0, bare `as` casts=0.
- `cargo clippy -p kuna-decomp --lib` clean (no warning touches the item file);
  `cargo clippy` on the new test file clean.

findings:
  - (none blocking) The item commit message claims "Each seam is reported in this item's
    losses" but no losses.md entry existed for this item. The verifier appended LOSS-114
    to record the 13 deferred bodies (the protocol permits the verifier to append). This
    is a documentation gap, not a code defect.

adversarial tests (rust/crates/kuna-decomp/tests/verify_w7_s37_coreaction_cleanup.rs):
  - w7_s37_names_groups_flags_are_ctor_verbatim — name/group/flags fed a non-schedule
    group token ("" and "x") to prove the group is the ctor arg, not a hard-coded literal.
  - w7_s37_merge_actions_exact_schedule_and_groups — full (name,group) vector pinned to
    cc:6002-6028, with the sole "casts" outlier asserted last.
  - w7_s37_seam_bodies_are_total_noops_under_repetition — every seamed apply returns 0 and
    keeps count==0 across 5 applies on a high-on Funcdata; the realized assignhigh is
    idempotent and signals no change; no seam alters the one observable Funcdata flag.
  - w7_s37_clone_filtered_group_filter_and_independence — mirror group-filter cases for a
    "merge" leaf vs the "casts" leaf, plus clone independence (clone count diverges, the
    original stays at 0).

losses: LOSS-114 (the 13 seamed merge-group apply bodies; one realized: ActionAssignHigh).
  Related: LOSS-112 (the Merge engine / MergeContext-without-Funcdata seam the merge
  delegations depend on), LOSS-094 (the analogous S4 prototype Action shells precedent),
  LOSS-104 (the blockaction structuring shells precedent).
