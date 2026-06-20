# w3-ir-varnode
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (15 in-module + 6 adversarial; 0 failed)

## Scope

C++ in scope: `decompiler/cpp/varnode.cc`, `decompiler/cpp/varnode.hh`
(blob shas match checklist: varnode.cc f444a66…, varnode.hh aa2166a… — no drift).
Branch `worktree-wf_434ccb01-75c-1`; diff touches `varnode.rs` (+2579),
`seams.rs` (arena key newtypes + HighVariable/Cover placeholders), `dtype.rs`
(minimal Datatype + verbatim `type_metatype`).

This is the W3 IR data-model port: `Varnode` + `VarnodeBank` as two
`Funcdata`-owned-equivalent stores. Per ADR 0001 `def`→`Option<OpId>`,
`descend`→`SmallVec<[OpId;4]>`, the cached `set::iterator`s → stored `LocKey`/
`DefKey`; per ADR 0002 `loc_tree`/`def_tree` are `BTreeMap<Key,VarnodeId>` whose
key `Ord` transcribes `VarnodeCompareLocDef`/`DefLoc` field-by-field, and the
C++ `searchvn` template is replaced by constructed bound keys (`LocProbe`/
`DefProbe`) converted to `Bound` per start/end position.

## Hunt list

- **Signedness.** Clean. The `(input|written)` flag class arithmetic
  `(f1-1)<(f2-1)` is done in `uint4` with explicit `wsub(1)` (FlagClass::cmp,
  varnode.rs:189-194); `0u32.wsub(1)==u32::MAX` reproduces "free sorts last".
  `int4` size/`uintb` offset mixes in `contains`/`intersects`/
  `characterizeOverlap` use `size as i64 as u64` = the C++ int4→uintb promotion
  (sign-extend then reinterpret); positive sizes coincide with `as u64`.
- **Integer widths.** `uintm=u32`, `int4=i32`, `uintb=u64`, `uint4=u32`,
  `int2=i16`, `uint2=u16` per ADR 0003. `uniqid += s` (uintm += int4) ported as
  `uniqid.wadd(s as uintm)` = C++ usual-arithmetic-conversion. `numVarnodes`
  returns `loc_tree.len() as int4` matching C++ `loc_tree.size()` (int4).
- **Wrapping.** Clean. `create_index++`/`uniqid += s` use `wadd`; geometry uses
  `wadd`/`wsub` throughout (`a+size`, `b+op.size`, `off+s-1`). The only un-
  wrapped arithmetic is `self.size - 1` / `s + 1` on a small positive byte count
  (panics only at `i32::MIN/MAX`, unreachable for a byte size — matches C++ which
  would wrap at the same impossible input). Not a finding.
- **Comparator totality.** `LocKey::cmp`/`DefKey::cmp` are a faithful
  transcription, **including** the C++'s own non-transitivity: `seqnum_step`
  returns `Equal` when two def seqnums share `uniq` but differ in `pc` (SeqNum's
  uniq-only `!=`), while ordering on full `(pc,uniq)` when `uniq` differs — the
  identical latent hazard `std::set<Varnode*,Comparator>` carries. It is inert
  under the IR invariant that `uniq` (op create-time) is globally unique per op,
  which the bank's allocation maintains. Documented in the porter's `seqnum_step`
  doc-comment and re-derived here; NOT a regression (the order would only break
  on the same input the C++ breaks on). The exhaustive in-module
  `golden_varnodesort_comparators_exhaustive` (>100k pairs) cross-checks both
  keys against a literal re-transcription of varnode.cc:34-79.
- **Iteration-order provenance.** Every iteration is `BTreeMap::range`/`.values()`
  over `loc_tree`/`def_tree` keyed by the transcribed comparators — order is the
  std::set key order, reproduced. No `HashMap`/`HashSet` anywhere in ported
  logic. `descend` is a `SmallVec` with C++ `push_back` order. Verified by the
  adversarial `def_tree_is_seqnum_dominated_not_address` (loc tree by addr, def
  tree by seqnum — opposite orders) and `flag_class_wrap_free_sorts_last`.
- **Off-by-one / do-while / reverse iterators / lower_bound vs upper_bound.**
  The start-vs-end membership of each C++ `set::iterator` is modelled
  explicitly: a probe is `lower_bound`/`upper_bound` and becomes
  `Included`/`Excluded` differently as a start (>=k / >k) vs an end (<k / <=k)
  (LocProbe/DefProbe `into_start`/`into_end`, varnode.rs:387-438). The `--iter`
  step-back in `findCoveringInput`/`hasInputIntersection` is `range((Unbounded,
  begin.into_end())).next_back()` = largest element < the lower_bound key =
  exactly `--iter`. The `endLoc(Address)` `getHighest()`-wrap and the
  `find_covered_input` `end==highest` wrap are both ported; the latter's wrap
  branch is covered by the new `find_covered_input_address_wrap_branch` test.
- **Erase-while-iterating.** The C++ `replace`'s `tmpiter = iter++; erase(tmpiter)`
  idiom is NOT in scope here — `replace` rewires op inputs and is delegated to a
  `ReplaceReads` caller callback (SEAM W3 funcdata). `eraseDescend`'s C++
  "find-then-erase, walk-off-end-if-absent (UB)" is ported as
  `position(...).expect(...)` (ADR 0004 panic on the UB case) then `remove(pos)`;
  order preserved (verified by `descend_push_back_and_boundary_erase`, incl.
  first/last erase).
- **Exception -> Result partial-state parity.** The `LowlevelError` throw sites
  (`addDescend` multi-descendant, `destroy` integrated, `setInput`/`setDef`
  not-free/constant, `beginDef(written,addr)` "contiguous written AND addressed")
  are all `Err` returned BEFORE any tree mutation — same partial state as the C++
  (which throws before erasing). `set_input`/`set_def` validate (`is_free`/
  `is_constant`) before the `erase` pair, matching C++ order.
- **SeqNum/create_index allocation-order fidelity (wave-specific).** `create`,
  `createUnique`, `createDef`, `createDefUnique` all draw `create_index` from the
  single bank counter with post-increment (`vn.create_index = self.create_index;
  self.create_index = wadd(1)`), exactly as C++ `create_index++`. `clear` resets
  `uniqid=uniqbase` and `create_index=0`. Verified by `create_index_monotonic`
  (in-module) and `create_unique_accumulates_and_clear_resets` (adversarial).
- **Intrusive-list splice-order fidelity (ADR 0001).** `descend` is the only list
  and it is a value `SmallVec`, not a spliced intrusive list, so there is no
  cross-list splice in this item; push_back/erase-by-value order is preserved.

## Findings

- F1 (minor / deferral, accepted as LOSS-029): five **pure** `VarnodeBank`
  tree-query methods are unported and carry no SEAM marker, unlike every other
  deferral in the diff: `beginLoc(AddrSpace*)`, `endLoc(AddrSpace*)`,
  `beginLoc(const Address&)`, `endLoc(const Address&)`, and `overlapLoc`. They
  read only `loc_tree` + `AddrSpaceManager::getNextSpaceInOrder`/`getHighest` —
  no op/type/symbol/cover dependency — so they are droppable now, but are absent.
  No in-tree consumer exists yet (op/block/funcdata are 1-line stubs), so the
  gate is unaffected; the gap becomes load-bearing when heritage/funcdata first
  iterates varnodes by address space. `loneDescend` is also unported but is a
  one-liner reconstructible from the ported `num_descend`/`descend_iter`, so it
  is not separately tracked.
       cpp: decompiler/cpp/varnode.cc:1579,1590,1601,1612,1810
       rust: rust/crates/kuna-decomp/src/varnode.rs (no `iter_loc_space`/`iter_loc_addr`/`overlap_loc`)

No blockers or majors. The op-graph / type-resolution / symbol / cover / print /
encode methods (`copyShadow`, `findSubpieceShadow`, `findPieceShadow`,
`partialCopyShadow`, `isConstantExtended`, `isEventualConstant`, `getLocalType`,
`termOrder`, `getUsePoint`, `getType*Facing`, `updateCover`/`calcCover`,
`encode`, `printRaw*`/`printInfo`/`printCover`, the Symbol setters,
`replaceInHigh`, `clearSymbolLinks`) genuinely require later-wave subsystems
(op arena W3-op, TypeFactory W6, HighVariable/Cover W7) and are correctly out of
scope for the IR data-model; most carry SEAM annotations, the remainder are
unreachable without those subsystems.

## Adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w3_ir_varnode.rs` (6 integration
tests over the public `VarnodeBank` API, exercising the real comparator+BTreeMap
path), committed `2bafc80` "rport/w3-ir-varnode: verifier adversarial tests":
  - `def_tree_is_seqnum_dominated_not_address` — loc tree by address, def tree by
    def seqnum, with the two orders deliberately opposite (the core of the
    LocDef vs DefLoc comparator split).
  - `flag_class_wrap_free_sorts_last` — `(f1-1)<(f2-1)`: input<written<free at
    one address, inserted scrambled.
  - `find_covered_input_address_wrap_branch` — 1-byte space (highest 0xFF),
    size-1 query at 0xFF hits `end==highest` (the branch the in-module
    `input_finders` test does not reach).
  - `create_unique_accumulates_and_clear_resets` — uniqid accumulation by size +
    uniqid/create_index reset on `clear`.
  - `xref_writtens_distinct_by_seqnum_but_equal_inputs_unify` — dedup keyed on
    the full comparator (writtens with same addr/size but distinct seqnum stay
    separate; equal inputs unify with the replace callback firing once).
  - `descend_push_back_and_boundary_erase` — push_back order + first/last
    erase-by-value.

Mechanical pass: `cargo clippy -p kuna-decomp -- -D warnings` clean; no
`todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the diff; all
bare `as` casts are `int4→u64` C++-promotion casts (`as i64 as u64`),
`len() as int4`, or `as uintm`/`as u64`/`as i64` size casts — each matching a
specific C++ implicit conversion. (`port-audit` not run: no such crate present in
the workspace; blob-sha drift checked directly — matches.)

## Losses

LOSS-029 (five pure VarnodeBank tree-query methods + overlapLoc unported, no SEAM
marker; no in-tree consumer yet) — appended to the main-tree losses.md.
