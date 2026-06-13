# w6-s5-type-1
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-12
round: 1
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (938 lib + all integration
      suites, 0 failed across 42 result lines). The item's own
      `verify_w6_s5_type_1.rs` is 7/7 green.
blob-sha: type.cc 9014a307b1c222be9414705e2151d5f50b29a23b (matches recorded),
          type.hh 0bafaef240058a69328f71a2ea632c661efef7ef (matches recorded) —
          no drift.

## Scope

Datatype base + scalar/pointer/array bodies from type.cc ~1-1722:
`compare`/`compareDependency` (base + TypePointer/TypeArray overrides),
`getSubType`, `getHoleSize`, `nearestArrayedComponent{Forward,Backward}`,
`isPtrsubMatching` (+ `testForArraySlack`), `findCompatibleResolve`, and the
LOSS-050 restoration of the base `resolveInFlow`/`findResolve` "return this".

## hunt list

- **Signedness**: clean. The two `submeta < op.submeta` / `wordsize < op_wordsize`
  comparisons are deliberate: `submeta` is a unit-discriminant enum compared by
  value (see below), `wordsize` is `uint4`/`u32` on both sides (C++ unsigned
  compare faithfully reproduced, dtype.rs:1137-1139, 1315-1316). The `op.size -
  size` finishers are signed `int4`/`i32` on both sides.
- **Integer widths**: clean. `int4`->`i32`, `int8`->`i64`, `uint4`->`u32`,
  `uint8`->`u64` throughout. `addressToByteInt(val,ws)=val*ws` (`int8 * uint4`)
  ports to `val.wmul(ws as i64)` (space.rs:1834) — a wrapping multiply matching the
  C++ `int8` wrap. The `getSubType` truncate `min = size - trunc->getSize()` is an
  `int4-int4` then widened to `int8` on both sides (dtype.rs:1382-1383); cannot
  realistically overflow (bigendian truncate => size >= trunc.size by
  construction).
- **Wrapping**: the size finishers use `wrapping_sub` (dtype.rs:1281,1322,1337,
  1354) matching C++ `op.size - size`. The two `wmul` byte conversions wrap. See
  OBS-1/OBS-2 for two bare non-wrapping `+`/`-` on paths that cannot realistically
  overflow (and one is currently unreachable) — non-blocking.
- **Comparator totality**: clean — and re-derived. THE load-bearing fact: the C++
  `submeta < op.submeta` compares the *integer* enum value (SUB_PARTIALUNION=0 ..
  SUB_VOID=23). The Rust `sub_metatype` (dtype.rs:142) derives `Ord` but declares
  variants in *descending* discriminant order. I empirically confirmed (twice, with
  a discriminant/declaration-order disagreement case) that Rust's derived `Ord` on
  a fieldless enum compares the *explicit discriminants*, NOT declaration position
  — so the Rust `<` matches the C++ integer order exactly. Pinned for all 24x24
  pairs by `w6_type1_submeta_ord_matches_cpp_integer_value`. Reflexivity +
  antisymmetry + strict-transitivity audited over a mixed corpus for both
  comparators (`w6_type1_compare_and_dependency_total_order_audit`). The
  address-identity tie-breaks (`compare_dependency_ptr`, pointer-space `Rc::ptr_eq`)
  are internally consistent within a run, faithfully mirroring C++ raw-pointer
  comparison (non-deterministic across runs in C++ too).
- **Iteration-order provenance**: N/A — no container iteration in this item (all
  bodies are scalar/recursive). No HashMap/HashSet (grep clean).
- **Off-by-one / do-while / reverse iterators**: clean. `getSubType` array bound
  `off >= size` (dtype.rs:1397), pointer truncate window `off >= min && off < min +
  trunc.size` (1387), `nearestArrayedComponentBackward` `off <= size` vs `> size`
  branch (1488) all transcribed exactly. No do-while / reverse-iterator idioms.
  Boundaries pinned by `w6_type1_get_sub_type_boundaries` and
  `w6_type1_nearest_arrayed_backward_branch`.
- **Erase-while-iterating**: N/A — no mutation/erase in this item.
- **Exception -> Result partial-state parity**: clean. The C++ overrides here have
  no `throw`; the SEAM(W6) `Err`s are pure deferrals (returned before any mutation —
  these methods are read-only / construct nothing). The `isPtrsubMatching` STRUCT
  branch propagates the struct-getSubType SEAM via `?` (struct getSubType is genuine
  type-2 W6), so it Errs rather than returning a wrong `true` — conservative and
  correct; pinned by the in-module `is_ptrsub_matching_routing` test.

## Re-derivation of the wave-specific risks

- **Datatype::compare/compareDependency total-order fidelity**: verified. The base
  body (submeta -> size finisher) and both TypePointer/TypeArray overrides match
  type.cc:216-237, 1074-1108, 1363-1384 line-for-line including the *order* of the
  tie-breaks (compare: base-step then wordsize then space then ptrto-recursion with
  decremented level; compareDependency: submeta then ptr/array-identity then
  wordsize then space then op.size-size — note compareDependency does NOT call the
  base size compare up front, faithfully transcribed). The `level<0` short-circuit
  to id-compare is reproduced (dtype.rs:1144-1150, 1159-1165).
- **TypePointer vs TypePointerRel cast safety**: re-derived the submeta partition.
  Plain TypePointer carries SUB_PTR(6)/SUB_PTR_STRUCT(4) (type.cc:1178-1183);
  TypePointerRel carries SUB_PTRREL(5)/SUB_PTRREL_UNK(7) (type.cc:3029,
  type.hh:1033). Disjoint => after the matching-submeta step the C++ `(TypePointer
  *)&op` cast is always a real plain pointer, so `as_plain_pointer()` returning
  `None` (the defensive invariant Err) is unreachable in correct usage. The Rust
  models `TypePointerRel` as a separate `DatatypeKind` and the plain-pointer
  overrides only match `DatatypeKind::Pointer`. Sound.
- **TypeFactory cache iteration determinism**: out of scope for this item (the
  factory tree/caches are W6 type-2/type-3, correctly SEAM'd).

## Mechanical pass

- grep: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in dtype.rs.
  Bare `as` casts in the new bodies are all `as int8`/`as usize`/`Rc::as_ptr`
  width/representation casts (i32->i64, addr->usize) — lossless; the remaining `as`
  greps were doc-comment prose.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean (the protocol-mandated form).
  Note: `cargo clippy --tests` surfaces lints in *pre-existing, unrelated* test
  files (heritage.rs:1602, verify_w4_fw_architecture_r2.rs, verify_w3_*,
  verify_w5_s3_rules_7.rs) — none in dtype.rs or verify_w6_s5_type_1.rs.
- port-audit: the `port-audit` crate does not exist in this workspace; blob-sha
  drift checked manually (above) — no drift.

## LOSS-050 restoration

LOSS-050 (the base `resolveInFlow`/`findResolve` "return this" deferral) is
*restored* by this item, exactly per its documented restoration path: the receiver
becomes `self: &Rc<Datatype>` and hands back `Rc::clone(self)` for every kind
without a union override (dtype.rs:1651-1682, 1693-1720). The pointer-to-union /
array / type-2-structured paths stay SEAM(W6) (they still need Funcdata). The
previously-pinning test `w5r2_resolve_in_flow_base_is_seamed_loss_f2` was correctly
flipped to assert the restored identity (Rc::ptr_eq of receiver). No production
caller exists (grep: only test callers), so the `&self`->`self: &Rc<...>` signature
change is safe; the gate confirms all callers compile. This closes a prior accepted
loss — it is NOT a new loss. (No `losses.md` append needed.)

## findings

(none rising to blocker/major/minor)

observations (non-blocking, no fix required for ACCEPT):
  - OBS-1: `isPtrsubMatching` STRUCT/`None` branch uses bare `extra += new_off2`
       (dtype.rs:1583) where C++ `extra += newoff` is `int8 += int8` (wraps). Rust
       `+=` panics on debug overflow. This path is currently UNREACHABLE (a STRUCT
       ptrto's getSubType is type-2 SEAM, so the `?` Errs before this branch), and
       offsets are small in realistic PTRSUB use. Mirror-of-C++ wrap risk is
       theoretical.
       cpp: decompiler/cpp/type.cc:1298
       rust: rust/crates/kuna-decomp/src/dtype.rs:1583
  - OBS-2: `getSubType` truncate `min = (self.size - trunc.get_size())` is a bare
       i32 subtraction (dtype.rs:1383); cannot underflow because bigendian truncate
       => size >= trunc.size by construction, same as C++.
       cpp: decompiler/cpp/type.cc:1064
       rust: rust/crates/kuna-decomp/src/dtype.rs:1383

## adversarial tests added

`rust/crates/kuna-decomp/tests/verify_w6_s5_type_1.rs` (7 tests, all green):
  - w6_type1_submeta_ord_matches_cpp_integer_value (the load-bearing Ord/discriminant fact, 24x24)
  - w6_type1_compare_size_finisher_is_raw_signed_diff
  - w6_type1_pointer_space_tiebreak_total_and_antisymmetric
  - w6_type1_compare_and_dependency_total_order_audit (reflexive/antisymmetric/transitive)
  - w6_type1_get_sub_type_boundaries (array off==0/size-1/size + bigendian truncate window)
  - w6_type1_is_ptrsub_matching_array_and_union_branches (vs the array's OWN align-size)
  - w6_type1_nearest_arrayed_backward_branch

One test initially failed (`is_ptrsub_matching` array multiplier vs align-size) — a
verifier test-author error, not a port bug: the C++ compares against
`ptrto->getAlignSize()` (the array object's align-size), not the element's. The
port is correct; the test was corrected to the array's align-size boundary.

losses: none (LOSS-050 restored, not a new loss).
