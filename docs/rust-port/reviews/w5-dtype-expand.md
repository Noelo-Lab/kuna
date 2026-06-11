# w5-dtype-expand
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-11
round: 2
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (602 passed, 0 failed;
      the item's own `verify_w5_dtype_expand.rs` is 9/9 green — 4 round-1 + 5
      round-2 adversarial tests).
branch: worktree-wf_a6862008-41f-1 @ 11541ed

## Scope

The `type.hh` `Datatype`/`TypeFactory` **interface** surface only (enums, flag
bits, base-class query/property accessors, the metatype helper free functions,
the subclass payloads as `DatatypeKind`, and the `TypeFactory` handle trait).
`type.cc` logic is W6; the porter additionally implemented the *trivial* `type.cc`
bodies that read only stored fields (the metatype helpers, base
`compare`/`compareDependency`, `isPrimitiveWhole`, the inline
`getDisplayFormat`/`getUnsizedId`/`typeOrderFormal`/`isPieceStructured`) and
routed every heavy override to `Err(KunaError::lowlevel("SEAM(W6) ..."))`.

## Round-1 -> round-2 disposition

Round 1 REJECTed on **F1** (major): `DatatypeKind::Spacebase` was routed to the
base `compare`/`compareDependency` body, silently returning `Ok(0)` for two
spacebases with distinct `localframe` (a non-total/wrong-result comparator),
where C++ `TypeSpacebase` *overrides* both (type.hh:821-822). **This is fixed in
round 2**: `compare` (dtype.rs:1100-1114) and `compare_dependency`
(dtype.rs:1133-1145) now match only `Base | Void | Unknown` to the base body and
route `Spacebase` (and every other overriding subclass) to the `_ => Err(SEAM)`
arm. Independently re-confirmed: TypeBase/TypeVoid/TypeUnknown declare NO
`compare`/`compareDependency` override in type.hh, so the base-body grouping is
exactly correct. The round-1 adversarial tests now pass (the `Ok(0)` panic branch
is unreachable; the seam `Err` is the faithful interface answer).

Round-1 **F2** (minor) — base `resolveInFlow`/`findResolve` C++ `return this`,
port returns `Err(SEAM(W6))` — is retained as a documented seam. The round-1 work
order allowed "implement OR record as an explicit loss"; the porter kept the seam
with an inline `LOSS (F2)` annotation. The verifier accepts it as **LOSS-050**
(appended to docs/rust-port/losses.md), making the deferral a ledgered loss.

## Re-derivation (rule-by-rule, against type.hh + the trivial type.cc bodies)

- `type_metatype`/`sub_metatype`/`type_class` discriminants — byte-for-byte vs.
  type.hh:80-142 (re-derived all 17/23/9 values).
- `Datatype::base2sub[18]` — matches type.cc:23-27 exactly (all 18 re-derived).
- flag bits (anonymous enum, type.hh:171-189) — exact, incl. `force_format`
  `0x7000`, `has_bitfields 0x40000`, and the `>>12` display-format extraction.
- base accessors (isCoreType/isCharPrint/isEnumType/isASCII/isUTF16/isUTF32/
  isVariableLength/isOpaqueString/isPointerToArray/isPointerRel/isFormalPointerRel/
  hasStripped/isIncomplete/needsResolution/hasWarning/hasBitfields/
  inheritForPointer/inheritForPartial/getDisplayFormat/getMetatype/getSubMeta/
  getId/getSize/getAlignSize/getAlignment/getName/getDisplayName/getTypedef) —
  all exact bit-tests/field reads.
- `metatype2string`/`string2metatype`/`string2typeclass`/`metatype2typeclass`
  (type.cc:242-436) — exact, incl. the upstream `"partenum"`-emitted-but-not-
  decodable asymmetry ('p' branch omits it), the known-first-char + fallthrough ->
  throw structure, and 'u'/"unknown" -> TYPECLASS_GENERAL.
- base `compare`/`compareDependency` (type.cc:216-237) — exact: submeta sign, then
  `op.size - size` (the port uses `wrapping_sub`, faithfully reproducing the C++
  signed-`int4` subtraction; sizes are small/non-negative in practice).
- `isPrimitiveWhole` (type.cc:505-518), `isPieceStructured` (`metatype <=
  TYPE_ARRAY`), `typeOrderFormal` (partialunion/bool deemphasis, then compare,10),
  `getDisplayFormat`, `getUnsizedId` (variable-length -> SEAM, else id) — exact.
- override partitions independently re-derived against type.hh's virtual map and
  confirmed correct:
    * `compare`/`compareDependency`: Pointer/Array/Struct/Union/Enum/Code/
      PartialStruct/PartialUnion/PartialEnum/PointerRel/Spacebase -> SEAM; rest base.
    * `getSubType`: Pointer/Array/Struct/Spacebase/PartialStruct/Code -> SEAM;
      Union (its override is COMMENTED OUT, type.hh:619) and Enum (no override) ->
      base `(None, off)`. The port routes Union/Enum to the base arm — correct.
    * `findCompatibleResolve`: the FIVE overriders (Pointer/Array/Struct/Union/
      PartialUnion, type.hh:294/488/522/608/637/714) -> SEAM; rest base `-1`.
    * `isPtrsubMatching`: Pointer/PointerRel -> SEAM; rest base `false`.
    * `getStripped`: PartialEnum/PartialStruct/PartialUnion/PointerRel return their
      cached `stripped` (type.hh:660/682/711/761); PartialEnum's getPartialBase
      returns `parent` while the other two return `container` — matched exactly.
    * `numDepend`/`getDepend`: PointerRel inherits TypePointer's `1`/`ptrto` (it
      does not override) — the port returns 1/ptrto. PartialUnion::numDepend is a
      W6 SEAM (returns 0); no implemented method (isPrimitiveWhole only recurses for
      ARRAY/STRUCT) depends on it, so no live corruption.

## Hunt list

- signedness: clean. `op.size - size` signed both sides; `submeta < op.submeta`
  uses enum `Ord` == discriminant order; `metatype <= TYPE_ARRAY` via `as_i32()`.
- integer widths: clean. int4->i32, uint4->u32, uint8->u64, int8->i64; `m as usize`
  index into BASE2SUB[18] bounded (metatype 0..17).
- wrapping: clean (faithful). base compare `op.size.wrapping_sub(self.size)`
  reproduces the C++ signed-`int4` subtraction; cannot panic; same 2's-complement
  result. (A `checked_sub` would have been a "fix" the C++ does not do.)
- comparator totality: clean (F1 FIXED). Spacebase no longer routes to the base
  body; the only *implemented* comparator path (Base/Void/Unknown) reads only
  submeta/size and is a total order. Every overrider SEAMs uniformly.
- iteration-order provenance: clean. The only ported container is
  `DatatypeKind::Enum { namemap: BTreeMap<u64,String> }`, reproducing the C++
  `map<uintb,string>` key order; struct/union `field` are `Vec` (insertion order).
  No `HashMap`/`HashSet` anywhere in the item's code.
- off-by-one / do-while / reverse iterators: clean (none in an interface port).
- erase-while-iterating: clean (none).
- exception -> Result partial-state parity: helper throws map to `Err` faithfully;
  `metatype2string` is exhaustive over the closed enum (the C++ `default: throw` is
  unreachable in Rust). F2 (base resolveInFlow/findResolve) is the one place the
  port errors where C++ returns `this` — accepted as LOSS-050.

## findings

  - F1 (major) — RESOLVED in round 2. `Datatype::compare`/`compareDependency` no
       longer route `DatatypeKind::Spacebase` to the base body; it falls to the
       `_ => Err(SEAM(W6))` arm, matching the type.hh:821-822 override. Verified by
       `w5r2_spacebase_compare_routes_to_seam_err` and the two round-1 spacebase
       tests (now green via the Err branch).
       cpp: decompiler/cpp/type.hh:821-822 ; decompiler/cpp/type.cc:3498-3514
       rust: rust/crates/kuna-decomp/src/dtype.rs:1100-1114, :1133-1145

  - F2 (minor) — ACCEPTED AS LOSS-050. Base `Datatype::resolveInFlow`
       (type.cc:578-582) and `findResolve` (type.cc:590-594) return `this` for all
       non-union types; the port returns `Err(SEAM(W6))` for every kind. No
       W5/interface-stage caller reaches it (these are S5/W6+ type-propagation
       surfaces), so it is not output-affecting at this item's boundary. Recorded
       as LOSS-050 with W6 restoration criteria.
       cpp: decompiler/cpp/type.cc:578-594
       rust: rust/crates/kuna-decomp/src/dtype.rs:1253 (resolve_in_flow), :1264
             (find_resolve)

  - F3 (nit, non-blocking) — a handful of bare `as` casts read enum discriminants
       (`self as i32` in `as_i32`) without a `// cast:` comment. All are
       self-evidently safe (closed enums, discriminants 0..23); the
       container-index casts (`m as usize`, `field.len() as int4`, `index as
       usize`) DO carry `// cast:` justifications. Cosmetic only; no behavior.
       rust: rust/crates/kuna-decomp/src/dtype.rs:112, :185

## Mechanical pass

- grep (item code): no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`
  in `dtype.rs`. The only `panic!`s are in the verifier test (divergence traces).
- bare `as`: container-index casts justified with `// cast:`; enum-discriminant
  casts are F3 (cosmetic).
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean.
- `cargo clippy -p kuna-decomp --test verify_w5_dtype_expand -- -D warnings`: clean
  (the verifier converted a round-1 `single_match` in the test to `if let`).
  (Note: a crate-wide `cargo clippy --tests -- -D warnings` still fails, but solely
  in the unrelated `verify_w4_fw_options.rs` test — a different item, out of scope.)
- port-audit: the `port-audit` crate is not present in this workspace; the item
  gate `cargo test -p kuna-decomp` is the authoritative check and is green.

## adversarial tests

In `rust/crates/kuna-decomp/tests/verify_w5_dtype_expand.rs` (branch @ 11541ed):

Round 1 (prior verifier, retained, now green via the F1 fix):
  - `w5_dtype_spacebase_compare_must_not_silently_equal_distinct_frames`
  - `w5_dtype_spacebase_compare_dependency_distinguishes_localframe`
  - `w5_dtype_base_compare_submeta_then_size_subtraction`
  - `w5_dtype_string2metatype_boundary_and_fallthrough`

Round 2 (this verifier, added):
  - `w5r2_spacebase_compare_routes_to_seam_err` — pins the F1 fix (Spacebase
    compare/compareDependency must Err, never Ok(0)).
  - `w5r2_find_compatible_resolve_override_partition` — pins the 5-overrider
    partition vs. base `-1` for Code/PartialEnum/Void/plain-int.
  - `w5r2_get_sub_type_union_enum_are_base_not_seam` — pins Union (commented-out
    override) and Enum (no override) -> base `(None, off)`, Struct -> SEAM.
  - `w5r2_pointer_rel_inherits_pointer_depend_and_ptrsub_seams` — pins PointerRel
    inheriting TypePointer's numDepend/getDepend, and the ptrsub/getPtrInto seams.
  - `w5r2_resolve_in_flow_base_is_seamed_loss_f2` — pins the LOSS-050 seam Err.

All 9 pass.

## losses

LOSS-050 (appended to docs/rust-port/losses.md): base
`Datatype::resolveInFlow`/`findResolve` "return this" identity is W6-deferred —
the port SEAMs them to `Err(SEAM(W6))` for every kind. Cited for this
ACCEPT-WITH-LOSSES.
