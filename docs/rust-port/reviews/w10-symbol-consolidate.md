# w10-symbol-consolidate
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8), structurally separate from the porter
date: 2026-06-15
round: 1
branch: rport/w10-symbol-consolidate (porter HEAD 1ee9536; verifier tests committed at 4af6cfe)
base: merge-base with rust-port = 8e0cc4a (the state both this branch and the
      already-accepted w10-global-persist forked from)

gate:
  - cargo test --workspace (branch) -> 3469 passed / 0 failed / 37 ignored
  - cargo test -p kuna-decomp (incl. the 5 verifier tests) -> 2621 passed / 0 failed
  - cargo clippy -p kuna-decomp --lib -- -D warnings -> clean (forced rebuild)
  - cargo clippy -p kuna-decomp --test verify_w10_symbol_consolidate_verifier2
      -- -D warnings -> clean
  - Rust XML datatest harness (decomp_test_dbg, full corpus):
        merge-base (8e0cc4a): 668 applied / 204 passing
        branch     (1ee9536): 672 applied / 224 passing
        passing-SET diff base->branch: regressions EMPTY (STRICT SUPERSET); +20
          gained (Global cross #1/#3, Conditional Constant #6-9, Conditional
          Multi #1-4, Display Format #1-4, Double precision move #6, No for-loop
          global call #2/#3, No for-loop iterator used #4, Pointer to array #6,
          Skip Instruction #2).
  - C++ oracle (main tree, decomp_test_dbg): 207/207 unit + 675/675 datatest,
      PARITY OK — UNTOUCHED (the branch diff is rust-only; zero C++/spec files).
  - B0 list-action order: universalaction_listing.rs (decompile tree dump byte-
      equal to oracle) -> ok, unchanged.

## Summary

FAITHFUL PARTIAL with ZERO regression. The item consolidates the global-symbol
naming cluster on top of the (already-accepted) global-persist substrate:

  1. ACTIVATES `Funcdata::set_varnode_properties` (the global half of
     `localmap->queryProperties`, snapshot onto `glb` as `GlobalQuery`) — paints
     `mapped|addrtied|persist` (+ the type-locked Symbol's forced type) on
     global-mapped RAM stores.  This is LOSS-190's deferred early marking, now
     SAFE because the naming seam landed: the `varcross::Global cross #2`
     regression LOSS-190 guarded against does NOT occur (verified Success on BOTH
     base and branch).  LOSS-190's restoration criterion is MET.
  2. GLOBAL-SYMBOL NAMING (`name_local_highs` -> `GlobalQuery::name_for_varnode`):
     a global-data high binds its covering Symbol's display name (`glob1`,
     `globalfree`), placed at the C++ `linkSymbol` point and NOT consuming a `vN`
     slot — the local `vN` counter is unperturbed.
  3. `addr_tied_ranges` (`Merge::mergeAddrTied`/`VarnodeBank::overlapLoc`) ported
     faithfully (the `bounds`/subrange/`group_with` geometry matches; see below).
  4. heritage persist RETURN-COPY (heritage.cc:1677-1692) + the persist-gated
     `unknown_effect`/`return_address` INDIRECT-op (heritage.cc:1512-1521),
     line-for-line; `getOpFromConst` iop round-trip closed.
  5. printc: global highs skip the local var-decl emit; the constant display
     format now honors equate-Symbol-then-read-facing-type precedence.

getNameRepresentative / compareName (variable.rs) are UNCHANGED by this diff
(ported in a prior wave) and confirmed faithful: the compareName tie-break ORDER
is exactly C++ (namelock, unaffected, persist, input, addrtied, protoPartial,
internal, written, def-time) and getNameRepresentative iterates `inst[0]` then
`skip(1)` in `inst` order — matching `variable.cc:456-511`.

Divergences are deliberate, non-observable on the corpus, and cited as losses
(LOSS-194 new; LOSS-149/190/191 referenced) — hence ACCEPT-WITH-LOSSES.

## Faithfulness (verified against the C++ in scope)

- `Scope::queryProperties` (database.cc:1268-1286) -> `GlobalQuery::query_properties`:
  the three branches (covering-symbol `getAllFlags`; owned-scope
  `mapped|addrtied|persist|getProperty`; flagbase-only) are transcribed exactly;
  the `isConstant` guard returns 0.  Pinned by verifier V1/V2/V3.
- `ScopeInternal::findContainer` (database.cc:2278-2310) -> `find_container_*`:
  the smallest-containing `oldsize` walk + `inUse` gate + exact-size break match;
  `end = addr+size-1` (uintb wrap) reproduced with `wrapping_*`.  Pinned by V4
  (smallest-size flags) + V5 (inclusive last-byte boundary).
- `SymbolEntry::inUse` (database.cc:115-121) / `updateType` (136-145) ->
  `inUse`/`sized_type_geometry`: addrtied short-circuits true; the typelock gate
  is honored (a non-typelocked covering Symbol forces NO type).  Pinned by V5.
- `Merge::mergeAddrTied` (merge.cc:609-648) + `VarnodeBank::overlapLoc`
  (varnode.cc:1810-1838) -> `addr_tied_ranges`: I traced the `bounds` structure
  (2*G+1 entries for G subranges; `max=2*G`; `for i=0;i<max;i+=2` = one
  `mergeRangeMust` per subrange; `if (max>2)` == `groups.len()>1`; the
  `group_with` loop over `*bounds[2],4,...` == `groups.iter().skip(1)`); the
  Rust grouping by `(offset,size)` in first-seen loc-order is equivalent because
  the VarnodeLocSet sorts same-(addr,size) varnodes contiguously.  `off =
  (int4)(vn2->getOffset()-vn1->getOffset())` reproduced (justified cast).
- heritage `guard_returns` persist RETURN-COPY + `guard_calls` INDIRECT-op:
  line-for-line (same ops/flags/order); `is_dead` skip matches the persist
  branch (which, unlike the active-output branch, does NOT check getHaltType).

## Mandatory hunt list

- Signedness: clean. `find_container_*` compares `e.space_index != space_index`
  (int4/int4), `e.first > start`/`e.last < end` (u64/u64), `e.size < oldsize`/
  `oldsize == -1`/`e.size == size` (int4) — every pair same-typed.  `addr_tied`
  uses u64 offsets and int4 sizes throughout, no silent C++ signed/unsigned mix
  carried over.
- Integer widths: clean. `end = start + size - 1` uses `start.wrapping_add(size
  as u64).wrapping_sub(1)` (the C++ uintb widening of a non-negative size);
  `all_flags = extraflags | symbol_flags` exact; the `(int4)` group offset cast
  is justified (in-window delta, small).
- Wrapping: clean (pinned by the boundary tests).  Every uintb offset op
  (`max_off`, `end_off`, the group `off`) uses `wrapping_*`; the Address `+ -1`
  use-point uses `wrap_offset`.
- Comparator totality: n/a — compareName (the only ordered comparator in scope)
  is UNCHANGED and is a total order matching C++; no new sort/BTree comparator.
- Iteration-order provenance: clean.  `build_global_query` walks
  `maptable.iter().enumerate()` (per-space index order) then `rangemap.records()`
  (BTreeMap key = `(last,subsort,seq)` — deterministic).  `addr_tied_ranges`
  walks `vbank.iter_loc()` (BTreeMap<LocKey> = C++ VarnodeLocSet order).
  `name_local_highs` walks `iter_loc()` + dedups by `BTreeSet<HighVariableId>`.
  `guard_returns` walks `iter_code(CPUI_RETURN)` (list order).  `directwrite`
  taint is an order-independent fixpoint.  NO HashMap/HashSet in changed code
  (grep clean; the only "HashMap" tokens are comments noting its AVOIDANCE).
- Off-by-one / do-while / reverse iterators: one NOTED non-finding.  C++
  `findContainer` iterates the rangemap query result in REVERSE (`--res.second`);
  the Rust iterates `entries` FORWARD.  For the smallest-inUse selection, equal-
  size containers resolve to the FIRST in iteration order, so the order differs
  ONLY for two equal-size containers both covering the range with DIFFERENT flags
  — not constructible for global mapped storage (one symbol per address).  Same
  documented non-finding as the global-persist review.
- Erase-while-iterating: clean.  Every added loop collects to a Vec first
  (`iter_loc().collect()`, `iter_code().collect()`, `descend_iter().collect()`)
  then mutates; the RETURN-COPYs/INDIRECTs are inserted before existing ops, not
  appended to the iterated container.
- Exception -> Result partial-state: minor, non-finding.  `guard_returns` uses
  `Err(_) => continue` on `new_varnode_out`; `addr_tied_ranges`/`directwrite`
  use `.expect()` on live-id gets (the ids come from the just-collected live
  iterators, so they cannot be stale on valid IR).

## Mechanical pass

- grep (changed src files) — no `glob1/globaloct/globalfree/read_glob/dat_40080c`
  in code (only in doc comments explaining the mechanism); no hardcoded
  addresses (`0x30101*`) in code; no hardcoded display values (`05555`/
  `0b10101010`) in code; no function-name special-casing (`"func"` is only test-
  fixture names); no HashMap/HashSet; no todo!/unimplemented! on changed paths
  (the pre-existing `unimplemented_seam` guard_stores/guard_loads stay unreached:
  `high_ptr_possible = false`, unchanged).  Global names come from the
  symboltab via `GlobalQuery::name_for_varnode` -> the entry's `symbol_name`.
- B0 list-action order unchanged (universalaction_listing.rs byte-equal).
- No bitfield-reserved file edited; the diff is rust-only (zero C++/spec/docs).

## Adversarial pass (verifier-authored, committed at 4af6cfe)

tests/verify_w10_symbol_consolidate_verifier2.rs (5 tests, all pass):
  - v1_query_properties_returns_covering_symbol_all_flags
  - v2_query_properties_owned_no_symbol_is_mapped_addrtied_persist
  - v3_query_properties_flagbase_only_and_constant_guard
  - v4_smallest_containing_flags_win
  - v5_sized_type_only_when_typelocked_and_boundary_inclusive

These pin the queryProperties branch logic, the smallest-size flags selection,
the constant guard, the typelock gate, and the inclusive containment boundary —
the halves the porter's `*_adversarial.rs` (name_for_varnode-focused) does not
cover.  Derived from the C++ bodies, not the Rust output.

## Findings

- F1 (low / doc): `Funcdata::set_varnode_properties` (funcdata.rs:~1330) and
  `new_varnode_out` (ruleaction_4.rs:~120) carry STALE doc paragraphs that say
  the persist/addrtied marking is "DEFERRED (a no-op here, as in the W3 base)" —
  but the code below now ACTIVELY paints the flags (and the type-force).  The
  prose contradicts the implementation; it should be updated to describe the
  active path.  Behavior is correct (confirmed by the corpus); the comment is
  merely misleading for a future maintainer.  Not a blocker.
- F2 (low / faithfulness nit, non-observable): `name_for_global_varnode` is
  called with the varnode's actual size (`v_size`), whereas C++ `Funcdata::
  linkSymbol` (funcdata_varnode.cc:1190) queries `queryProperties(addr, 1, ...)`
  with size=1.  Identical outcome for the corpus (the globals are exact-size, so
  containment holds for both 1 and size); differs only for a varnode extending
  past its covering Symbol's last byte — not constructible here.  Cited context,
  not a divergence requiring a loss.
- F3 (low / faithfulness nit, non-observable): the printc constant display-format
  override (printc.rs push_vn_explicit_ir) sources the Symbol format from the
  EQUATE symbol only (`vn_high_display_format` -> `vn_high_equate_symbol`),
  whereas C++ `push_integer` (printc.cc:1376) overrides with ANY `high->
  getSymbol()->getDisplayFormat()`.  The displayformat corpus is driven by the
  read-facing TYPE format (the faithful `pushVnExplicit` path), so the four
  Display Format assertions pass at parity; a non-equate Symbol carrying a format
  differing from its type would diverge — not exercised by the corpus.

## Losses (cited)

- LOSS-194 (new, appended by this verifier): the call-side `unknown_effect`/
  `return_address` INDIRECT-op is emitted ONLY for PERSIST (global) ranges,
  narrowing the C++ unconditional emission; the `killedbycall` INDIRECT-creation
  + output-trial registration stay deferred (LOSS-149).  ZERO regression; the
  narrowing keys only on the (faithfully-derived) persist flag.
- LOSS-190 (referenced): its deferred early-marking body is now ACTIVATED safely;
  the restoration criterion is MET (Global cross #2 verified Success on base AND
  branch — no regression).
- LOSS-149 (referenced): the output-trial / killedbycall half remains deferred.
- LOSS-191 (referenced): the `directwrite` input-seed `has_store()` guard is
  unchanged on this branch (inherited from global-persist).

## Verdict rationale

Every new pass is REAL parity against the C++ oracle B5 (the oracle passes the
identical Display Format #1-4 / Conditional Constant #1-12 / Global cross #1-3
assertions the branch gained).  No previously-passing assertion regresses (the
passing set is a strict superset of the merge-base).  boolless stays byte-
identical (Success on both); condconst #1-5/#10-12 unchanged; floatcast stays
honestly FAIL (no fabricated pass).  The transcriptions are faithful, there is
no special-casing, the pipeline order is unchanged, the C++ oracle is untouched,
and the only divergences are deliberate, cited, non-observable narrowings.
ACCEPT-WITH-LOSSES.
