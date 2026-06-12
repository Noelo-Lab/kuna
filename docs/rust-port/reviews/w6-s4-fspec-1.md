# w6-s4-fspec-1
verdict: ACCEPT
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-12
round: 2 (re-verify of round-1 REJECT)

gate: `cd rust && cargo test -p kuna-decomp` -> PASS (item functionally green)
  - 958/958 lib tests pass; `fspec::tests` 29/29 pass.
  - `verify_w6_s4_fspec_1` 12/12 pass (8 round-1 + 4 round-2 verifier tests).
  - Only failure is the pre-existing **out-of-scope** `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`
    (needs an unbuilt `x86:LE:64` `.sla` artifact; LOSS-078). This item's diff does not touch
    `corpus_bootstrap.rs`, and the failure is environmental (missing build artifact), independent
    of the fspec port. Confirmed against base `rust-port` (file untouched by this item).
  - `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean (exit 0).
  - `cargo clippy -p kuna-decomp --test verify_w6_s4_fspec_1 -- -D warnings` -> clean.

## Round-1 blocker resolution (F1)

Round 1 REJECTed on **F1 (major)**: `ParamTrial::cmp` returned `Ordering::Greater` for
two null-`entry` trials in BOTH directions (`(None, _) => Greater`), violating antisymmetry —
a non-total comparator fed to `sort_unstable_by` (unspecified output, "silent output
corruption").

The repair (commit `b6e903c`, fspec.rs:1149-1154) splits the null cases out explicitly:
```
(None, None) => return Equal,     // both null: equivalent (C++ both `<` false)
(None, Some(_)) => return Greater,// self not "<" b (C++ fspec.cc:1898 false)
(Some(_), None) => return Less,   // self "<" b      (C++ fspec.cc:1899 true)
```
This is the faithful strict-weak-order translation of C++ `operator<` (fspec.cc:1895-1916,
lines 1898-1899 are the null-entry early `return false`/`return true`).

**Re-derived and independently confirmed**, not merely trusted:
- The verifier's round-1 failing test `two_null_entry_trials_are_equivalent_not_both_greater_w6s4`
  now PASSES with no edit to the test.
- New round-2 test `cmp_is_total_strict_weak_order_exhaustive_w6s4` exhaustively checks the
  three strict-weak-order laws (reflexivity, antisymmetry, transitivity) over a 7-element
  population spanning every comparator branch (group / entry-index tiebreak / exclusion offset /
  reverse-stack addr / size) plus two null trials — PASSES on the fix.
- Adversarial regression proof: temporarily reintroducing the round-1 defect makes
  `cmp_is_total_strict_weak_order_exhaustive_w6s4`, `fixed_position_compare_is_total_order_w6s4`,
  and `two_null_entry_trials_are_equivalent_not_both_greater_w6s4` all FAIL (3 failures), so the
  new tests are genuinely discriminating. Restored after.

## Hunt list (full independent re-derivation against the C++, not round-1 notes)

- **Signedness.** Clean. The `-Wno-sign-compare` loops in scope are all `i>=0`-bounded indices:
  `groupOverlap` (`i>=groupSet.size()` -> `usize` index, fspec.rs:288-308), `buildTrialMap`
  (`hitlist.size()<=grp` -> `(hitlist.len() as i32) <= grp`, fspec.rs:2537), `separateSections`
  (`nextSection > resourceStart.size()`, fspec.rs:2635), `characterizeAsParam`/`findEntry`
  (`index >= resolverMap.size()` -> `index < 0 || (index as usize) >= len`, fspec.rs:1934,1984).
  Faithful.
- **Integer widths.** Clean. `int4`->i32, `uint4`->u32, `uintb`->u64 throughout; member
  assignments `res.size = (int4)` carry `// cast: int4 -> uint4 member`; `(int4)diff` carries
  `// cast`. Modulo-then-`as i32` results (`justifiedContain`, fspec.rs:430,436) are bounded
  < alignment so non-lossy.
- **Wrapping.** Clean (covered by `contained_by_no_panic_on_wrap_w6s4`). The `uintb`-lineage
  arithmetic in `subsumesDefinition`/`containedBy`/`intersects`/`justifiedContain`/`getContainer`/
  `getSlot`/`getAddrBySlot`/`populateResolver`/`checkJoin` uses `wadd`/`wsub`/`%` matching C++
  unsigned wrap. The `int4` multiply `(index * self.alignment)` (fspec.rs:640) and
  `(*slotnum * self.alignment)` (fspec.rs:619) would panic on debug overflow where C++ int wraps,
  but are unreachable with realistic slot/alignment values (latent, not a finding).
- **Comparator totality.** Clean (was F1, now fixed — see above). `EffectRecord::compare_by_address`
  (fspec.rs:1642) delegates to `Address::cmp` (space-index then offset); for real EffectRecord
  ranges (never null/max sentinel space) this matches C++ `compareByAddress` exactly
  (fspec.hh:1761-1766). `fixed_position_compare` (fspec.rs:1181) is total: its (-1,-1) arm
  delegates to the now-total `cmp` (verified by `fixed_position_compare_is_total_order_w6s4`).
- **Iteration-order provenance.** Clean. No `HashMap`/`HashSet` (grep clean). The resolver is a
  BTree-backed `rangemap::RangeMap` (deterministic; `find`/`find_begin`/`find_end`/`iter_between`
  match the C++ multiset-by-`last`-then-`position` order). `populate_resolver` preserves C++
  insertion `position` order (fspec.rs:2282-2306); `selectUnreferenceEntry`/`assignAddressFallback`/
  `buildTrialMap`/`fillinMap` walk `Vec<ParamEntry>` / `&[ParamEntry]` in index order = C++
  `list<ParamEntry>` insertion order.
- **Off-by-one / do-while / reverse iterators.** Clean. `findEntryByStorage` reverse walk
  (`(0..len).rev()`), `justifiedContain`/`getContainer` join reverse loops (`(0..num_pieces).rev()`),
  `which_trial` (fspec.rs:1288), `getNumUsed` early-break. `separateSections` faithfully keeps the
  C++ `nextSection > size()` (not `>=`) check (fspec.rs:2635) — see LOSS-081.
- **Erase-while-iterating.** Clean. `deleteUnusedTrials`/`splitTrial`/`joinTrial`/`freePlaceholderSlot`
  are all collect-into-new-`Vec` rebuilds matching C++ verbatim, including the slot-shift
  bookkeeping (verified by `split_then_join_round_trips_slots_w6s4`).
- **Exception -> Result partial-state parity.** Clean. `resolveJoin`/`resolveOverlap`/`orderWithinGroup`/
  `splitTrial`/`joinTrial`/`assignMap`/`separateSections`/`fillinMap` throw points map to `?`/`Err`
  before any later mutation; the partial state on the error path matches (half-built entry / trial
  vector is discarded by the caller on `Err`, same as C++ where the half-built object is abandoned).
  `joinTrial` size-mismatch `Err` verified in `split_then_join_round_trips_slots_w6s4`.
- **(wave add) Datatype/ParamEntry/ParamTrial sort orders + cache determinism.** ParamEntry/ParamTrial
  sort orders covered above (now total). `Datatype::compare`/`TypeFactory` cache iteration are out
  of this item's scope (fspec.cc:2268+ ProtoModel and dtype.rs are separate items); not reviewed here.

## Re-derived (output-determining, confirmed faithful this round)

- `justifiedContain` / `getContainer` / `assumedExtension` / `containedBy` / `intersects` /
  `contains` / `subsumesDefinition` — endian/alignment/join arithmetic matches C++ (fspec.cc:62-596).
- `getSlot` / `getAddrBySlot` reverse-stack + float-ext index math (fspec.rs:550-649) — confirmed by
  `get_slot_reverse_stack_mapping_w6s4` (slot side) and the round-1 `getAddrBySlot` test (addr side).
- `characterizeAsParam` second-pass window (fspec.rs:2012-2024): the port re-derives the window from
  `find_begin(loc)..find_end(loc+size-1)` instead of continuing from the first `find()`'s end
  iterator. Re-derived from the C++ rangemap `find`/`find_end` semantics (rangemap.hh:332-401):
  the re-scanned first-window entries were ALL already tested for `is_exclusion() && contained_by`
  in pass 1 (which would have early-returned `ContainedBy` before pass 2), so the re-scan cannot
  change the Containment code. RESULT-equivalent in the empty-first-window case too (C++ scans the
  same `[iter1, find_end)`). Benign; covered by the two `characterize_*` tests.
- `buildTrialMap` / `separateSections` / `forceExclusionGroup` / `markBestInactive` /
  `markGroupNoUse` / `forceNoUse` / `forceInactiveChain` / `fillinMap` family / `assignMap` family /
  `assignAddressFallback` / `checkJoin` / `checkSplit` / `findEntry` / `selectUnreferenceEntry` —
  all re-derived line-for-line against fspec.cc:684-1844, faithful. The null-entry-deref sites C++
  reaches only via non-null trials are guarded in Rust with `.expect(...)`/`else 0`; unreachable on
  the real path (null-entry trials are `markNoUse`d -> `isDefinitelyNotUsed` -> skipped before deref,
  and sort to the END so the `i-1` prev-slotGroup in `forceInactiveChain` is never null).
- `ParamEntry::seed` builder faithfully mirrors the C++ `decode` resolution tail
  (`alignment==size->0`, numslots, reverse-justify, reverse-stack, resolveFirst->resolveJoin->
  resolveOverlap). `decode` itself is the W4 SEAM (returns Err).

## findings
  (none blocking; F1 resolved)

  - Observation O1 (minor, non-blocking): bare `as` casts without `// cast:` comments at fspec.rs
    393/395/446/447/498/661/1291/1298/2436/2537/2589 (index/loop-position conversions, e.g.
    `vdata.size as i32`, `i as i32`, `(hitlist.len() as i32)`). All non-lossy or routine
    index/bound conversions, consistent with the surrounding `as i64 as u64` sign-extension idiom.
    Cast-comment hygiene only; no faithfulness divergence. Matches the round-1 treatment.

## adversarial tests
(rust/crates/kuna-decomp/tests/verify_w6_s4_fspec_1.rs — round-1 set committed f158d5c,
 round-2 additions committed ecf8837; all 12 PASS)

Round-2 additions (this review):
  - `cmp_is_total_strict_weak_order_exhaustive_w6s4` — exhaustive reflexivity / antisymmetry /
    transitivity audit of `ParamTrial::cmp` over a 7-element mixed population; FAILS against the
    round-1 defect, PASSES on the fix.
  - `fixed_position_compare_is_total_order_w6s4` — totality of `fixedPositionCompare` over mixed
    fixed/unfixed + null-entry trials (the (-1,-1) -> cmp delegation path); FAILS against round-1 defect.
  - `split_then_join_round_trips_slots_w6s4` — `splitTrial`/`joinTrial` slot bookkeeping round trip
    (1,2,3 -> split slot2 -> 1,2,3,4 -> join -> 1,2,3) + slotbase + size-mismatch `Err` parity.
  - `get_slot_reverse_stack_mapping_w6s4` — reverse-stack `getSlot` index map (the inverse of the
    round-1 `getAddrBySlot` test).

Round-1 set (re-run green this round):
  - `two_null_entry_trials_are_equivalent_not_both_greater_w6s4` (was the REJECT evidence; now PASSES),
    `sort_keeps_all_null_trials_after_real_ones_w6s4`, `sort_stress_null_vs_real_invariant_w6s4`,
    `characterize_contained_by_via_second_pass_w6s4`, `characterize_nonempty_first_window_rescan_is_benign_w6s4`,
    `get_addr_by_slot_reverse_stack_index_w6s4`, `justified_contain_be_aligned_boundary_w6s4`,
    `contained_by_no_panic_on_wrap_w6s4`.

## losses
  - LOSS-081 (informational): `separateSections` preserves the upstream C++ `nextSection > size()`
    (not `>=`) bound. On malformed input where `nextSection == resourceStart.size()`, C++ silently
    reads `resourceStart[size()]` (OOB) while Rust panics on the index. Faithful to the check as
    written; unreachable with valid compiler specs (`resourceStart` always carries the `numgroup`
    sentinel pushed in `decode`, fspec.cc:1504).

## verdict rationale
The single round-1 blocker (F1, comparator non-totality) is fixed with a faithful strict-weak-order
translation, independently re-derived and proven by an exhaustive comparator audit that fails against
the old code and passes against the new. No new findings; every output-determining surface in scope
(containment matrices, resolver, slot/justification arithmetic, trial rebuilds, fillin/assign chains,
characterizeAsParam) re-derives faithfully against fspec.cc ~1-2267. ACCEPT.
