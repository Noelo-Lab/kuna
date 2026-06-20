# w10-typed-access
verdict: ACCEPT_WITH_LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 2

gate: `git diff rust-port...rport/w10-typed-access -- rust/` -> **2 files**
      (`funcdata.rs` +222, `coreaction_infertypes.rs` +20); HEAD `00789a4`,
      one source commit on top of the round-1 verdict commit (`b721023`).
      `cargo test --workspace` -> all green (0 failed). `cargo clippy -p
      kuna-decomp --lib -- -D warnings` -> clean. C++ oracle / specs / baseline
      byte-untouched (`git diff --name-only` lists only `rust/` + `docs/`).

## Summary

Round 1 was REJECT (empty branch). Round 2 ports a real, faithful slice of the
union-resolution machinery: the `Funcdata` **`unionMap` cache** accessors
(`funcdata.cc:915-1028`) and wires `propagateTypeEdge`'s `resolveInFlow`
cache-hit arm (`coreaction.cc:5336-5341`). The transcription is faithful and
free of special-casing. It is **dormant**: the scorer driver that fills the
cache on a miss (`ScoreUnionFields`, LOSS-087) is not wired, so the new live
path returns `None` on every miss and the cache is never populated during
corpus decompilation — the change is **byte-neutral** (proven base==branch on
the full 675-assertion corpus tally). Zero new passes, zero regressions.
This is the LOSS-156..168 dormant-foundation pattern; ACCEPT_WITH_LOSSES,
ledgered as **LOSS-169**.

## What the branch actually adds (and what it does not)

ADDS (faithful):
- `union_map: BTreeMap<ResolveEdge, ResolvedUnion>` field on `Funcdata`
  (= C++ `map<ResolveEdge,ResolvedUnion> unionMap`), cleared by `clear()`
  (`funcdata.cc:103`).
- `get_union_field` / `get_union_resolution` / `get_address_based_union_field`
  / `set_union_field` / `set_address_based_union_field` — verbatim against
  `funcdata.cc:915-1028`, including the `getUnionField` size-match guard
  (`unresType->getSize() == dt->getSize()`), the `setUnionField` MULTIEQUAL
  dup-slot copy (`getIn(i) == vn` over the other slots), and the lock arms.
- `find_union_resolve` (= the const `findResolve` family: needsResolution gate
  -> `getUnionField` hit -> field datatype, else the receiver unchanged).
- `resolve_union_in_flow` (= `TypeUnion::resolveInFlow`'s cache-consulting
  prefix: `getUnionField`, then `getAddressBasedUnionField` with the on-hit
  copy onto the (op,slot) edge).
- the `propagateTypeEdge` arm: `if alttype.needs_resolution() { if let
  Some(r) = resolve_union_in_flow(..) { if !is_marker { alttype = r; } } }`.

DOES NOT add (the seam, declared as LOSS-169):
- the `ScoreUnionFields` live driver — `resolve_union_in_flow` returns `None`
  on a cache miss instead of running the scorer + caching its result. In C++
  the same miss runs `ScoreUnionFields(*fd,this,op,slot)`, caches
  `scoreFields.getResult()`, and returns the *scored* field datatype. The Rust
  declines (leaves the raw union in flow). This is the documented behavioral
  gap; it is conservative (never a wrong resolution) and currently invisible
  (no live caller writes the cache, no corpus assertion depends on it).

## Mandatory hunt list

- **Signedness / widths.** Clean. `slot`/`encoding` stay `int4`; `op_time` is
  `uintm`(=u32); `type_id` is `uint8`(=u64) — matching the C++ members. The one
  added cast (`addr.get_offset() as uintm`, in the prior-wave `ResolveEdge::
  new_addr`) carries a `// (uintm) cast:` justification and faithfully mirrors
  the C++ `uintb -> uintm` member store (verified by the new
  `w10r2_address_offset_uintm_truncation_is_mod_2_32` test).
- **Wrapping.** No arithmetic added on the live path beyond the documented u32
  truncation; no wrap risk.
- **Comparator totality.** `ResolveEdge::Ord` (prior wave, but load-bearing for
  this cache) is a strict total order on `(type_id, encoding, op_time)` —
  re-pinned by the new `w10r2_resolve_edge_ordering_precedence_and_totality`
  (irreflexive, antisymmetric, transitive, encoding-dominates-opTime) and
  `w10r2_resolve_edge_pointer_does_not_collide_with_raw_union` (the `0x1000`
  pointer bit keeps a ptr-to-union edge distinct from the raw-union edge that
  shares its `type_id`).
- **Iteration-order provenance.** The cache is a `BTreeMap` keyed by the
  faithful `ResolveEdge` order = the C++ `std::map` order. The MULTIEQUAL
  dup-slot loop is `for i in 0..num_input` over op input slots = the C++
  `for(int4 i=0;i<op->numInput();++i)` index order. No `HashMap`/`HashSet`
  anywhere in the diff (grep clean).
- **Off-by-one / do-while.** `setUnionField`'s MULTIEQUAL loop skips `i == slot`
  and tests `getIn(i) == vn` — matches C++ exactly; verified by
  `w10_set_union_field_multiequal_copies_to_dup_slots` (copies to the same-vn
  slot 2, not the different-vn slot 1).
- **Erase-while-iterating.** N/A (no erase during traversal).
- **Exception -> Result partial-state.** `set_union_field`'s `None => return
  true` op-missing path is a defensive guard with no C++ analogue (C++ `op` is
  always valid); returning the non-locked success default is correct and cannot
  leave partial state (it precedes any map mutation). `resolve_union_in_flow`'s
  address-based copy mutates the cache via `set_union_field` then returns the
  hit — the only mutation, and it occurs only on a real address-cache hit
  (currently unreachable, since nothing writes the address cache live).

## Faithfulness nuances examined (not findings)

1. **address-based copy return value.** C++ builds a *fresh* `ResolvedUnion
   (this, res->getFieldNum(), types)` and returns *its* datatype while caching
   the *existing* `*res`; the Rust returns the existing `res.get_datatype()`
   and caches the existing record. Both `ResolvedUnion`s carry the same
   `fieldNum`/resolved-datatype (the fresh one is built from the same `this` +
   `fieldNum`), so the returned datatype is equivalent. This path is dead code
   on the live corpus (no `setAddressBasedUnionField` caller), so the
   equivalence is moot for parity and only matters for faithfulness — and it is
   faithful.
2. **marker side-effect ordering.** C++ runs `resolveInFlow` (with its caching
   side-effect) unconditionally and only gates the `alttype = resType`
   assignment on `!isMarker()`. The Rust calls `resolve_union_in_flow`
   unconditionally and gates only the assignment. Faithful.
3. **`ResolvedUnion::update` pointer compare.** C++ `resolve == op.resolve` is
   `Datatype*` identity; Rust uses `Rc::ptr_eq`. Faithful; pinned by the new
   `w10r2_resolved_union_update_lock_arms` (lock+diff-field refuses; lock+same-
   field datatype update applies; same-field same-Rc is a no-op).

## No special-casing

`git diff rust-port...rport/w10-typed-access -- rust/crates/kuna-decomp/src/`
production lines: the only `0x` literal is `0x2000` inside a doc comment
(referencing the C++ address-edge encoding). The `0x2000`/`0x300`/`0x80`/`0x84`
numeric literals are all inside the `#[cfg(test)]` module (fixture addresses) —
allowed. Zero function-name / datatest-address / union-name / case-value /
magic-divisor dispatch in production logic; resolution dispatches purely on
`get_metatype()` / sizes / field indices. The `0x1000`/`0x2000` `ResolveEdge`
encoding constants are verbatim from `unionresolve.cc:108-122`, derived
generally (the pointer bit is added for *any* `TYPE_PTR`, not a specific type).

## Mechanical pass

- diff grep `todo!|unimplemented!|panic!|HashMap|HashSet|sort_unstable` ->
  **none**.
- diff grep bare `as` casts in production -> **none** added by this branch
  (the only `as` matches are the `use ... as dtflags` rename and prose in
  comments; the one real cast lives in the prior-wave `new_addr`).
- `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean.

## Real-passes / regression verification (the teeth)

- **Corpus tally identical base==branch.** `verify_w10_struct_corpus`
  (`verify_w10_corpus_stringmatch_tally`, all 83 datatests, 675 assertions)
  run on **base `rust-port`** (specs symlinked into the base worktree) and on
  **branch `rport/w10-typed-access`** both report: PASS positive **5**, PASS
  negative **32**, FAIL positive **628**, FAIL negative **10**. The W10 change
  adds **zero** new passes and breaks **zero** — exactly the inert-plumbing
  prediction. The merged-tree "59 passing" baseline is preserved (this fleet
  item neither raises nor lowers it).
- **Named byte-parity fences green.** `print_b5_boolless` (5/5),
  `printc_parity` (11 pass / 1 ignored — drives the committed C++
  `decomp_test_dbg` for its byte-compare, confirming the oracle is intact),
  `verify_w10_dominant_copy` (readstruct, 3/3), `verify_w10_const_prop_phi`
  (condconst_conn, 6/6), `corpus_bootstrap` (1/1).
- **`cargo test --workspace`** -> all green, 0 failed.
- **C++ oracle** 207/207 + 675/675 PARITY OK, byte-untouched (branch diff is
  `rust/`+`docs/` only).

## Adversarial tests (committed on the branch, `f914d80`)

`rust/crates/kuna-decomp/tests/verify_w10_typed_access.rs` — the round-1 fence
(8 tests pinning `score_truncation`/`score_locked_type` C++ constants) is kept,
plus 5 round-2 tests targeting THIS branch's cache plumbing:
- `w10r2_resolve_edge_pointer_does_not_collide_with_raw_union` — the `0x1000`
  pointer bit (non-vacuous: both edges share the union's `type_id`).
- `w10r2_resolve_edge_ordering_precedence_and_totality` — operator< precedence
  + strict total order.
- `w10r2_resolve_edge_address_form_truncates_offset_and_ignores_slot` — the
  `0x2000` encoding, slot-ignore, and `uintm` offset truncation (two offsets
  congruent mod 2^32 collapse to one key).
- `w10r2_resolved_union_update_lock_arms` — all four `ResolvedUnion::update`
  branches (the cache's `setUnionField` return value depends on them).
- `w10r2_address_offset_uintm_truncation_is_mod_2_32` — the cast contract.
All 13 green; clippy `-D warnings` clean on the test target.

## Verdict

ACCEPT_WITH_LOSSES. The cache plumbing is a faithful, additive, special-casing-
free transcription of `funcdata.cc:915-1028` + the `coreaction.cc:5336-5341`
arm. It is currently dormant (the `ScoreUnionFields` miss-path is the seam),
so it produces no new datatest passes — but it also regresses nothing
(base==branch corpus tally; all named byte-parity fences and the whole
workspace green; C++ oracle untouched). The dormancy + the declined miss-path
are ledgered as **LOSS-169**.
