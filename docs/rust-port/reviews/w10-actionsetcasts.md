# w10-actionsetcasts
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 1
gate: `cargo test --workspace` (worktree /tmp/w10-actionsetcasts) -> all green, 0 failed
      (kuna-decomp alone: 2545 passed / 0 failed);
      `cargo clippy --lib -p kuna-decomp` -> clean (no warnings);
      `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK

## Scope

Branch `rport/w10-actionsetcasts` is **1 commit** (fdf41e0), Rust-only diff
(`git diff --name-only rust-port...HEAD` touches only `rust/`). It does **NOT**
close the `ActionSetCasts` keystone the item is named for. It lands the
**prerequisite** only: the `Funcdata` union-field resolution cache
(`decompiler/cpp/funcdata.cc:915-1115`).

Files:
- NEW `rust/crates/kuna-decomp/src/funcdata_union.rs` (358 lines): the nine
  `unionMap` accessors `get_union_field` / `get_union_resolution` /
  `get_address_based_union_field` / `set_union_field` /
  `set_address_based_union_field` / `update_union_field` / `force_facing_type` /
  `inherit_union_field` / `inherit_union_field_ptr`.
- `funcdata.rs`: adds the `union_map: BTreeMap<ResolveEdge, ResolvedUnion>` field
  + its `clear()` (the `unionMap.clear()` at funcdata.cc:90, previously a SEAM
  note).
- `lib.rs`: `pub mod funcdata_union;`.
- `coreaction_cleanup.rs`: comment-only — updates the `ActionSetCasts::apply`
  SEAM note to record the prereq landed; the body still `return 0` (no
  CAST/PTRSUB op emitted).

The prompt's keystone surface (`ActionSetCasts::apply`/`castInput`/`castOutput`/
`checkPointerIssues`/`tryResolutionAdjustment`/`resolveUnion`/`insertPtrsubZero`,
the `CastStrategy` getInputCast/getOutputToken, the per-op typeop cast/propagate,
the printc cast/field rendering) is **NOT in this diff** and is NOT claimed. The
`apply` body is byte-for-byte the same no-op it was on `rust-port`. Verified
accordingly: this is a foundation-first landing of the persistence layer the
eventual cast/union-resolution pass will key on.

NB on the ledger: LOSS-167 in `losses.md` describes these same accessors at a
`funcdata.rs:1351-1572` location. That code is **not present on the current
`rust-port` base** (`git show rust-port:.../funcdata.rs` has only seam-note
*comments* for `unionMap`, no field, no accessors; `funcdata_union.rs` does not
exist there). So this branch is the **genuine first landing** of the accessors,
not a duplicate — it just lives in a dedicated file rather than inline.

## Verdict rationale

FAITHFUL transcription, ZERO special-casing, ZERO regression, ZERO new parity
(the keystone is still seamed) -> ACCEPT-WITH-LOSSES (a dormant-foundation loss).

### Faithfulness (line-by-line vs funcdata.cc:915-1115)
- `get_union_field` (cc:915-929): edge build, `find`, the `unresType->getSize()
  == dt->getSize()` size guard, null on miss — exact.
- `get_union_resolution` (cc:939-950): same lookup WITHOUT the size guard — exact.
- `get_address_based_union_field` (cc:959-968): address-edge form — exact.
- `set_union_field` (cc:979-1004): `emplace` -> `BTreeMap::entry`
  (vacant=insert / occupied = the `!res.second` arm running `update`, returning
  `!isLocked()` when update is false); the `code()==MULTIEQUAL && slot>=0`
  duplicate-input-slot fan-out copying the resolution to every other input slot
  holding the same Varnode — exact. The Rust snapshots the duplicate slot list
  before the mutable re-entry (the C++ holds a raw `PcodeOp*` and reads
  `op->getIn(i)` throughout); the snapshot is the same edge sequence in the same
  order, so the observable result is identical.
- `set_address_based_union_field` (cc:1015-1028): the locked-rejects /
  overwrite-else arms — exact.
- `update_union_field` (cc:1038-1049): `setResolve` on hit, false on miss — exact.
- `force_facing_type` (cc:1057-1069): `baseType` derived from the ORIGINAL
  unresType (TYPE_PTR -> getPtrTo) BEFORE the pointer-rel reassignment, the
  `isPointerRel()` -> `getTypePointer(size, baseType, wordSize)` conversion, then
  `ResolvedUnion(unresType,fieldNum,*glb->types)` + `setUnionField` — exact
  ordering (the non-pointer-rel PTR case computes `baseType` but never uses it,
  matching the C++ dead-store).
- `inherit_union_field` (cc:1079-1091): the `slot<0 && oldOp->isMarker()` ->
  `slot=0` rewrite, find-or-`-1`, copy via `setUnionField`, return the field
  number. The Rust captures `field_num` from a clone BEFORE the set whereas C++
  reads `(*iter).second.getFieldNum()` AFTER — these diverge ONLY if the set
  mutated that exact map entry, which requires the write edge to equal the read
  edge; in that case the value inserted equals the entry's own value, so
  `update()` compares equal and returns false (no mutation), leaving the field
  number identical. PROVABLY equivalent.
- `inherit_union_field_ptr` (cc:1101-1115): `getDepend(0)`, the same marker
  rewrite, find-or-`-1`, `ResolvedUnion(unresPtr,fieldNum,*glb->types)` +
  `setUnionField`, return the inherited field — exact.
- The prereq value types it keys on (`ResolveEdge::operator<` total order, the
  0x1000/0x2000 encoding, `ResolvedUnion::update` lock arms) live in
  `unionresolve.rs` on the base (prior wave, w6-s5-unionresolve) and were
  re-checked: faithful.
- `clear()` placement: `union_map.clear()` runs after `clear_active_output()`
  and before `clear_blocks()`, matching funcdata.cc:84-103.

### No special-casing
Grepped the full diff. Zero function-name / address / register-name / type-name
hardcoding. The only literals are `0x1000`/`0x2000` (the verbatim ResolveEdge
pointer/address encoding bits) and the `-1`/`0` field-number sentinels — all
straight from C++. The two "HashMap"/"as" grep hits in `funcdata_union.rs` are
both inside doc comments (no real HashMap, no real `as` cast in the file). The
keyed map is a `BTreeMap` (HashMap is clippy-banned and would lose the ordered
`find`/`emplace` the C++ `std::map` relies on) — correct choice.

### No regression (all re-run on the worktree)
- C++ oracle UNTOUCHED (Rust-only diff): 207/207 unit + 675/675 datatest, PARITY
  OK.
- boolless B5 byte-parity unregressed: `print_b5_boolless` 11/11 (the committed
  C++ B5 oracle byte-compare + the anti-special-casing tripwires fire and pass).
- readstruct/condconst path unregressed: `verify_w10_typed_access` 8/8 (the W10
  typed-access parity fence), and the broader w10 suite green.
- Whole kuna-decomp suite: 2545 passed / 0 failed (>> the 182 floor; none of the
  prior passes regressed).
- `cargo clippy --lib -p kuna-decomp`: clean.
- Mechanical: no `todo!`/`unimplemented!`/`unreachable!`/`panic!` in
  `funcdata_union.rs`; no `HashMap`/`HashSet`; no uncommented `as` cast.

### No fabricated passes
There are NO new passes to fabricate — `apply` still returns 0 and the comment
honestly says so. The porter's in-tree test (`verify_w10_funcdata_union_cache.rs`,
6 tests) exercises real accessor behavior, but one arm
(`w10_union_get_field_size_match_and_mismatch`, lines 132-135) is a **tautology**
(`assert!(x.is_none() || x.is_some())`) — it punts on actually proving the size
guard MISSES. Not a fabricated pass of production code, but a toothless
assertion.

### Verifier adversarial tests (committed to the porter branch, 8060803)
The required path `rust/crates/kuna-decomp/tests/verify_w10_actionsetcasts.rs`
was MISSING from the branch (the porter committed
`verify_w10_funcdata_union_cache.rs` instead). I authored **5 two-sided,
mutation-checked** fences at the required path and committed them:
1. `w10_get_union_field_size_guard_rejects_but_resolution_still_hits` — closes
   the porter's tautology: a size-8 union whose field-0 resolve dt is size 4 ⇒
   the edge key HITS but `get_union_field` returns None (size guard) while
   `get_union_resolution` returns Some (proves the miss was the guard, not a
   missing edge).
2. `w10_multiequal_output_edge_does_not_fan_out` — slot == -1 ⇒ no fan-out
   (the `slot >= 0` guard).
3. `w10_multiequal_fanout_is_op_local` — fan-out does not bleed to a different
   MULTIEQUAL with a different opTime.
4. `w10_unlocked_overwrite_changes_field_number` — the `update()==true` arm flips
   the cached field; the same-field re-set is a no-op that still returns true.
5. `w10_inherit_marker_rewrites_write_slot_to_zero` — `slot<0 && isMarker` lands
   the inherited resolution on slot 0, not the -1 write edge.

Mutation-verified: deleting the size guard fails (1); dropping the `slot >= 0`
guard fails (2) (it then `get_in(-1)` panics — exactly the bug the guard
prevents). All 5 pass on the unmodified source.

## Losses
- LOSS-172 (this review): the `ActionSetCasts` keystone is NOT closed; only the
  union-cache persistence layer landed. Dormant foundation, zero new parity,
  fail-closed. See `docs/rust-port/losses.md`.
