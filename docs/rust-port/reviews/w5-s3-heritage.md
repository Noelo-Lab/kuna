# w5-s3-heritage
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (463 tests pass incl. 11 new adversarial; 0 failed)

## Scope

Partial port of `decompiler/cpp/heritage.{cc,hh}` (3.2k LOC). The **realized**
surface is the self-contained SSA-construction *engine*:

- `LocationMap` disjoint-cover (`add`/`find`/`findPass`/`erase`), `TaskList`,
  `PriorityQueue`, `HeritageInfo`, `LoadGuard` predicate state;
- the augmented-dominator-tree phi-node placement core
  (`buildADT`/`visitIncr`/`calcMultiequals`);
- `bumpDeadcodeDelay` (with the kuna restart-recorder anchor) and the
  dead-code-delay bookkeeping (`numHeritagePasses`/`seenDeadCode`/
  `getDeadCodeDelay`/`setDeadCodeDelay`/`deadRemovalAllowed[Seen]`);
- `buildInfoList`/`clearInfoList`/`clear`/`forceRestructure` and accessors.

The **data-flow mutation** surface (`rename`/`renameRecurse`,
`placeMultiequals`, the `heritage` pass driver, all `guard*`/`collect`/
`refinement`/`processJoins`, and the load-guard value-set methods) is a
documented seam (`unimplemented_seam` panic). It depends on `Funcdata`
primitives absent from the merged tree (`setInputVarnode`, `newVarnodeOut`/
`newUniqueOut`, the single-address `beginLoc(addr)` range) and the W4/W6
subsystems. This is **LOSS-054** (ledgered per the LOSS-035/036/037/042 seam
precedent). Verified the named primitives are genuinely absent, so the seams
are blocked, not lazily skipped.

cpp_blob_sha verified against the in-tree source (no drift):
heritage.cc = 7e79c40e…, heritage.hh = 804c96a5… — both match the checklist.

## Re-derivation (rule-by-rule against the C++ oracle)

- `LocationMap::add` (heritage.cc:34) — re-derived the full iterator walk:
  `lower_bound(addr)`-then-step-back, the `++iter` skip when the leading
  candidate does not overlap (cc:40-41), the first-block contain/partial branch
  (cc:45-57), and the multi-element swallow `while` loop (cc:58-66). The Rust
  re-queries `range(addr..).next()` each iteration; proved this coincides with
  the C++ `erase(iter++)` successor in every branch (the leading element after
  the `++iter` logic always has key >= addr in the no-first-overlap case, and
  the erased key == addr in the overlap case). The pass-min carry and the
  intersect codes (0/1/2) match exactly.
- `buildADT` (cc:2317) — transcribed: up-edge accumulation into `b[]`/`t[]`,
  the reverse `a[]`/`z[]` recurrence + boundary marking, `z[0]=-1`, the `z[]`
  boundary-ancestor pass, and the `while(j<k){augment[k].push(v); k=z[k]}`
  idom-dominance loop. `k` stays >= 1 inside the loop (`j<k`, `j>=0`), so the
  `z[k as usize]` index never goes negative. Iteration order (forward over
  blocks, then over domchild, then over in-edges; reverse for the recurrence)
  is preserved.
- `visitIncr`/`calcMultiequals` (cc:2395/2440) — the `merged_node`/`mark_node`/
  `boundary_node` flag protocol, the early `break` on the first augment edge
  whose idom is not a strict ancestor, the dom-child recursion guard, the
  start-block seeding at `flags[0]`/`depth[0]`, and the final mark-clear all
  match. `pq.insert(v, depth[k])` ordering preserved.
- `PriorityQueue` (cc:142) — `reset` early-return (`curdepth==-1 &&
  maxdepth==len-1`), LIFO `push_back`/`pop_back` per depth, the `extract` drain
  walking `curdepth` down past empty stacks to the -1 sentinel: faithful.
- `bumpDeadcodeDelay` (cc:2572) — the space-type gate (IPTR_PROCESSOR ||
  IPTR_SPACEBASE), the `getDelay()==getDeadcodeDelay()` guard, the install-once
  / suppress-on-revisit split, and both restart-recorder calls match. The
  `Override`/`RestartLog` are explicit `&mut` params (a documented W7 seam; the
  exception-free path has no partial-state concern).
- `HeritageInfo::reset` — the intentionally-omitted `deadcodedelay = delay`
  (override-preserving) is reproduced with the same comment.

## Mandatory hunt list

- **Signedness**: clean. The only signed comparisons in realized logic are
  `z[i] > a[i]+1`, `getIndex() < j`/`j_q`, and `while j<k` — all int4 vs int4,
  matching the C++; the `-Wno-sign-compare` loop bounds (`j<domchild.size()`,
  `k<sizeIn()`) became `0..len`/`0..size_in` (usize/int4-domain `Range`s).
- **Integer widths**: clean. The SSA core is small graph-index int4 only; no
  `int4*int4`-into-intb promotions. The only `uintb` site is
  `LoadGuard::is_guarded`'s offset `<`/`>` (no arithmetic).
- **Wrapping**: clean. No `uintb` arithmetic in realized logic;
  `LocationMap`/`TaskList` size math is int4 byte-distance from the (verified)
  `Address::overlap`.
- **Comparator totality**: n/a — the ordered containers key on `Address`
  (`BTreeMap`), whose `Ord` is the verified kuna-base total order; no float or
  custom comparator ported here.
- **Iteration-order provenance**: clean. `LocationMap.themap`/`VariableStack`
  are `BTreeMap<Address,…>` (== C++ `map<Address,…>` key order); `domchild`/
  `augment`/`merge`/`flags`/`depth` are `Vec` index-keyed (== C++ `vector`);
  `disjoint` is a `Vec` in insertion order (== C++ `list`, only mid-list-iter
  callers are seamed). **No HashMap/HashSet anywhere.**
- **Off-by-one / do-while / reverse iterators**: clean. The reverse recurrence
  `for(i=size-1;i>=0;--i)` is `for i in (0..size).rev()`. `PriorityQueue`'s
  inner drain is a `while`+break preserving the do-once semantics. No
  `lower_bound`/`upper_bound` boundary slip (`find`/`findPass` use
  `range(..=addr).next_back()` == C++ `upper_bound`-then-decrement).
- **Erase-while-iterating**: the `LocationMap::add` swallow loop and `find`/
  `findPass` are the erase/decrement idioms; re-derived (above) that the
  re-query reproduces the C++ post-erase successor in every branch. Pinned by
  `location_map_add_swallow_carries_global_min_pass` and
  `..._skips_nonoverlapping_leading_then_unions_later`.
- **Exception -> Result parity**: clean. The two throwing methods
  (`numHeritagePasses`, `setDeadCodeDelay`) became `KunaResult` with the same
  guard condition and no mid-mutation partial state (the check precedes any
  write). All other deferred mutators panic loudly rather than `?`-return.

## Mechanical pass

- `cargo run -p port-audit -- --item w5-s3-heritage`: **n/a** — `port-audit` is
  not yet in the workspace (a tooling seam). Blob-sha drift checked by hand
  (matches, above).
- grep of heritage.rs: `HashMap`/`HashSet` = 0; `todo!`/`unimplemented!` = 0;
  `sort_unstable` = 1 (a test helper, not ported logic); bare `as` casts are
  all int4<->usize index-domain conversions on proven-non-negative values,
  consistent with the merged-file convention (dtype.rs/block.rs carry the same
  unannotated index casts — `// cast:` is reserved for truncation/sign casts).
- `cargo clippy -p kuna-decomp -- -D warnings` (the protocol's lib gate):
  **clean**.

## Adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w5_s3_heritage.rs` (11 tests, all
green; committed on the branch at 37086fe):
`location_map_add_skips_nonoverlapping_leading_then_unions_later`,
`location_map_add_between_elements_is_fresh_disjoint`,
`location_map_add_swallow_carries_global_min_pass`,
`location_map_find_pass_end_boundary_exclusive`,
`tasklist_add_only_extends_last_not_earlier`,
`priority_queue_drains_across_empty_intermediate_depths`,
`phi_placement_three_way_join_single_phi`,
`phi_placement_three_way_join_two_writes_still_single_phi`,
`phi_placement_stacked_diamonds_first_write_no_phi_at_dominated_join`,
`phi_placement_stacked_diamonds_mid_write_phi_at_second_join_only`,
`bump_deadcode_delay_spacebase_installs_once_then_suppresses`.

Note: my initial stacked-diamond test asserted phis at BOTH joins; it failed,
and on re-derivation the **port was correct** — block 3 *strictly* dominates 6
(idom(6)=3), so 6 is not in DF(3) and the iterated frontier of a write in the
first diamond is `{3}` only. The corrected test pair pins both the "no phi at a
strictly-dominated join" case and the genuine "phi at the second join only"
case (write in block 4). This is positive evidence the phi-placement core is
not over-placing.

## Findings

- F1 (minor, non-blocking): the porter's in-module test at
  `rust/crates/kuna-decomp/src/heritage.rs:1602` asserts
  `h.infolist.is_empty() || h.infolist.iter().all(|i| !i.is_heritaged() || true)`
  — the `|| true` makes the predicate a tautology (clippy `logic_bug`). The
  assertion is vacuous and would fail `cargo clippy --all-targets -D warnings`.
  Not a correctness defect in ported logic (test-quality only); does not gate
  acceptance. Recommend replacing with a substantive post-`clear` invariant
  (e.g. `get_pass()==0` and `merge_points().is_empty()`, which already hold).

No major or blocker findings. The realized algorithmic core is a faithful
transcription of the C++ oracle.

## losses

LOSS-054 (the deferred data-flow mutation surface — `rename`/`placeMultiequals`/
`heritage` driver/guards, all seam-marked, all named primitives confirmed
absent from the merged tree).
