# w6-s5-typeop
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> lib 1024/1024 pass; typeop unit 11/11 pass; verify_w6_s5_typeop 5/5 pass. One UNRELATED failure: `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` requires compiled `.sla` SLEIGH artifacts (gitignored, absent in this worktree); the typeop commit (d4aace8) touches no specs/corpus code. NOT a blocker for this item.

## Scope

Item commit d4aace8 changes exactly two files: `rust/crates/kuna-decomp/src/typeop.rs`
(+1157) and `src/typeop/tests.rs` (+778). The large `dtype.rs`/`verify_w6_s5_type_3.rs`
deltas in `git diff rust-port...HEAD` come from a *merged sibling branch*
(`worktree-wf_91be7ffe-767-2`, item w6-s5-type-3) and are out of scope here.

The port is the canonical `inst[]` table: per-op-code `(opflags, addlflags, name,
class, metaout, metain)` transcribed from each `TypeOp*` constructor, plus
`registerInstructions`, `selectJavaOperators`, `floatSignManipulation`, the addlflags
predicates, and the size-only `getInputLocal`/`getOutputLocal` overrides. The full
per-op `propagateType`/`getInputCast`/`getOutputToken` surface and
`propagateToPointer`/`propagateFromPointer` are explicitly seam-noted for W8 (they need
CastStrategy/TypePointerRel/getTypePartialEnum, unported at the W6-partial boundary).

## Re-derivation

Every registered op-code's flag word, name, class, and metatypes were re-derived from a
third independent transcription of the `typeop.cc` constructors (grep of every
`opflags =`/`addlflags =` assignment and every `TypeOpBinary/Unary/Func(t,...)`
delegation, typeop.cc:391-2623) and matched against the production `type_op_info` match.
All 73 entries match. Verified independently:

- `addlflags` enum bits (typeop.hh:42-47): inherits_sign=1, inherits_sign_zero=2,
  shift_op=4, arithmetic_op=8, logical_op=0x10, floatingpoint_op=0x20 — match.
- `pcodeop_flags` bits (op.hh:73-104) vs the Rust `pf::` consts — all match.
- Opcode enum: FLOAT_LESSEQUAL=44, **gap at 45**, FLOAT_NAN=46, CPUI_MAX=75
  (opcodes.hh:91,93,131) — Rust `register_instructions` builds `vec![None; 75]` and
  indexes by `opc as usize`, leaving slots 0 and 45 null exactly as C++
  `inst.insert(end,CPUI_MAX,0)` does.
- Base-class defaults `getBase(size, TYPE_UNKNOWN)` and the
  Binary/Unary/Func `getBase(size, metaout/metain)` (typeop.cc:262-376) — match.
- Size-only overrides PTRADD/PTRSUB (INT both ways), shift slot-1 (getBaseNoChar INT),
  CPOOLREF in (INT), CBRANCH slot-1 (BOOL), INSERT (<=1 UNKNOWN), ZPULL/SPULL (slot-0
  UNKNOWN) — match (typeop.cc:611-621,1512-1608,2234-2320,2467-2471,2537-2596).

## hunt list

- Signedness: clean. Only `slot == 0/1`, `slot <= 1` (int4 vs literal), matching C++
  `int4 slot`. Enum-discriminant `as usize` index casts are all non-negative.
- Integer widths: clean. int4=i32/uint4=u32/uintb=u64/intb=i64 match; `floatSignManipulation`
  keeps (int4 size, uintb offset) exactly as C++ `cvn->getSize()`/`getOffset()`.
- Wrapping: clean. `floatSignManipulation` uses only `>>` and `^` on u64 — cannot wrap;
  no add/sub/mul on any uintb-lineage value in this module.
- Comparator totality: N/A — no comparators or sorts in typeop.
- Iteration-order provenance: clean. `register_instructions` iterates the fixed slice
  `REGISTERED_OPCODES` and assigns `inst[opc as usize]`, mirroring C++'s explicit
  `inst[CPUI_*]=new ...` block; no HashMap/HashSet. (v1 pins the exact null shape
  independently of REGISTERED_OPCODES.)
- Off-by-one / do-while / reverse iterators: clean. vec length == CPUI_MAX; slot-45 gap
  preserved; no do-while or reverse iteration.
- Erase-while-iterating: N/A.
- Exception->Result parity: clean. `get_output_local`/`get_input_local` return
  `KunaResult`, `?`-propagating `get_base[_no_char]` errors (pure lookups — no partial
  state). `type_op_info(CPUI_MAX)` panics, matching the null `inst[CPUI_MAX]`
  (programming-bug, not a runtime path).
- Wave-specific (Datatype::compare / ParamEntry/ProtoModel sort / TypeFactory cache):
  N/A — typeop has no comparator or factory-cache logic; it only *calls* `get_base`.

## Mechanical pass

- greps over typeop.rs / typeop/tests.rs: no `todo!`/`unimplemented!`/`HashMap`/
  `HashSet`/`sort_unstable`/`partial_cmp`. Single `panic!` is the CPUI_MAX guard
  (matches null inst[CPUI_MAX]). Bare `as usize` casts are all OpCode-enum->index for
  array lookup (the C++ `inst[opc]` analogue) — non-lossy, non-negative discriminants.
- `cargo clippy -p kuna-decomp -- -D warnings` (the protocol's required lib invocation):
  CLEAN. (Under the broader `--tests` mode there are pre-existing pedantic lints in
  *other* items' test files — verify_w4_fw_architecture_r2.rs, verify_w5_s3_rules_7.rs,
  dtype.rs:5714 — plus a glob-import false-positive on typeop/tests.rs:12
  `use type_metatype::*` which IS used by the b!/t! macros; none affect correctness and
  none are in the protocol's required invocation. Noted as N1.)

## findings

- N1 (nit): `typeop/tests.rs:12 use type_metatype::*` flags as unused only under
  `clippy --tests` (the glob is consumed by the b!/t! macros). Cosmetic; lib clippy is
  clean. No fix required.
- N2 (nit): the `as usize` index casts in register_instructions/select_java carry no
  `// cast:` justification comment (verification.md asks for one). These are
  enum-discriminant->array-index, semantically the C++ `inst[opc]`; non-lossy. Cosmetic.

No blockers, no majors. The W8 seams (CBRANCH slot-0 code pointer; CALL/CALLIND/
CALLOTHER/RETURN/INDIRECT prototype-aware getInputLocal/getOutputLocal; CPOOLREF
getOutputLocal; propagateType bodies) are documented in-source and are the wave-level
W6-partial boundary, not faithful-port defects within this item's declared scope. v4
pins the current CBRANCH slot-0 inherited-UNKNOWN behavior so any future regression is
visible.

## adversarial tests

`rust/crates/kuna-decomp/tests/verify_w6_s5_typeop.rs` (5 tests, all pass):
- v1_register_instructions_null_shape_is_exactly_the_opcode_gap — re-derives null/non-null
  per slot from `OpCode::from_i32` (the opcode enum), independent of REGISTERED_OPCODES;
  catches off-by-one in that constant or a transposed `opc as usize` index.
- v2_float_sign_manipulation_is_width_specific — per-width (1/2/4/8) ABS/NEG recognition
  + negative cases (8-byte mask must not match at narrower widths; cross-op AND/XOR
  confusion). Targets the integer-width hunt item the porter's size-8-only test missed.
- v3_select_java_operators_blast_radius — snapshots the whole table, toggles Java, and
  asserts exactly the six C++-touched op-codes change (opflags/addlflags/class never), +
  a full Java->C round-trip.
- v4_local_type_overrides_metatype_rederived — re-derives every getInputLocal/
  getOutputLocal override metatype straight from the C++ bodies, including the documented
  CBRANCH slot-0 W8 seam.
- v5_typeopclass_shape_sanity — guards the TypeOpClass per shape (PIECE/SUBPIECE are Func
  despite the binary opflag; PTRADD/PTRSUB are Base).

losses: none
