# w10-rel-pointer
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier, round 1
date: 2026-06-15
gate: `cargo test --workspace` -> ALL GREEN (3284+ tests, 0 failed); `cargo clippy -p kuna-decomp --lib` / `-p kuna-console --lib` -> clean; C++ oracle `decomp_test_dbg datatests` -> 675/675 PARITY OK (untouched, Rust-only diff)

## Scope

One commit (2840d01): closes the `getPtrInto` PointerRel seam, ports the
`TypePointerRel::getPtrToFromParent` walk as a `TypeFactory` default body, wires
the `IfcPointerSetting` (`pointer setting <n> <s> offset <o>` / `space <spc>`)
console command, and adds the `pushTypePointerRel` (`ADJ(...)`) rendering arm to
`PrintC::opPtrsub`. 5 Rust files changed (3 src + 2 test); **zero C++/spec
changes**.

## Faithfulness (line-by-line vs C++ in scope)

- **`Datatype::get_ptr_into` PointerRel arm** (dtype.rs:1148) vs
  `TypePointerRel::getPtrInto` (type.cc:3060-3070): STRUCT/UNION ptrto →
  `(ptrto, 0)`, else `(parent, offset)`. Exact. ✓
- **`TypeFactory::get_ptr_to_from_parent`** (dtype.rs:3540) vs
  `getPtrToFromParent` (type.cc:3157-3171): `off > 0` arm runs the do-while
  `base = getSubType(curoff,&curoff)` walk, null result → `getBase(1,
  TYPE_UNKNOWN)`; the `off <= 0` else arm → `getBase(1,TYPE_UNKNOWN)`. The Rust
  `loop`+`break` preserves the C++ do-while's at-least-once + dual exit
  (`curoff != 0 && base != 0`). Exact. ✓
- **`push_type_pointer_rel_ir`** (printc.rs:2824) vs `pushTypePointerRel`
  (printc.hh:372-377): `pushOp(&function_call)` + `Atom("ADJ", optoken,
  funcname_color, op)`. `FUNCTION_CALL` OpToken matches the C++ `{"(",")",2,66,
  false,postsurround,0,10}`; token "ADJ" = `typePointerRelToken`. Exact. ✓
- **The 9 `if is_rel { push_type_pointer_rel_ir(op) }` insertions** in
  `op_ptrsub_ir` (printc.rs:2889-3017) line up 1:1 with the 9 C++ `if (ptrel)
  pushTypePointerRel(op)` sites (printc.cc:985-1157), each placed after the
  structural op and before `pushVn`. Exact ordering. ✓
- **`IfcPointerSetting::execute`** (ifacedecomp.rs:2099) vs ifacedecomp.cc:3051-
  3099: conf-check-first → "No load image present"; the name/base-type/setting
  eof-guarded parse error ladder; offset arm (struct base-type check,
  `getPtrToFromParent`, default-data-space `getTypePointerRel`); space arm
  (`findByName`, `getSpaceByName`, `getTypePointerWithSpace`); the
  `"Unknown pointer setting"` else; the success echo. Faithful. ✓

## No special-casing

Production diff has only the two C++ command keyword literals (`"offset"`,
`"space"`) — faithful transcriptions of `if (setting == "offset")`. Zero
function-name / address / case-value / type-name hardcoding (grep over the
non-comment added lines confirms). The `100`/`"a".."d"` literals are in tests
only.

## Mandatory hunt list

- Signedness: clean — `off:int4 > 0` matches C++; `off as int8` widen lossless.
- Integer widths: clean — `curoff:int8` matches `int8 curoff`; `getSubType`
  newoff is int8 both sides; `v as int4` bounded by `v <= int4::MAX`.
- Wrapping: clean — no new arithmetic; the opPtrsub `suboff` math is pre-existing
  (the diff added only `push_type_pointer_rel_ir` calls, no arithmetic lines).
- Comparator totality: N/A (no comparators added).
- Iteration-order: clean — the do-while walks a struct's fields via index
  (`get_field_iter`), deterministic; no HashMap/HashSet anywhere in the diff.
- Off-by-one / do-while: THE fragile spot — `getPtrToFromParent`'s do-while.
  Verified faithful (at-least-once + null-fallback + multi-iteration drill) by
  adversarial tests AT1/AT2.
- Erase-while-iterating / exception→Result partial-state: N/A.

## Mechanical pass

- grep added lines: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`; all `.unwrap()` are in tests.
- bare `as`: 4 in production (`v as int4` [bounded], `get_addr_size()/
  get_word_size() as int4` [C++ returns int4], `off as int8` [lossless widen]) —
  all correct, no-comment style consistent with dtype.rs prevailing practice
  (5 `// cast:` notes vs 103 `as` in the file; comments reserved for lossy/
  non-obvious casts). Minor observation, not a finding.
- clippy `-p kuna-decomp --lib` / `-p kuna-console --lib`: clean.
- port-audit: N/A — `w10-rel-pointer` is a `w10-infra-parity-grind` sub-branch,
  not a formal checklist item with a recorded `cpp_blob_sha`; C++ untouched.

## Byte-parity invariants

boolless / readstruct / condconst_conn tests all green and byte-identical
(`w10_boolless_full_byte_parity_modulo_type_inference`, the
`w10_dc_condconst_conn_*` / `w10_eob_condconst_conn_*` suite,
`verify_w10_symbol_naming_readstruct_body_*` all pass). Full workspace
`cargo test` 0 failed. C++ oracle 675/675 PARITY OK (no C++ changed).

## findings

  - F1 (loss, not a blocker): the headline `ADJ(...)` relptr-rendering parity is
    NOT realized end-to-end and the 9-site rendering arm is DORMANT (untested).
    The pointerrel.xml script reaches `retype v4 myptroff ptrrel`, which hits the
    unported "engine integration not yet ported: parse_type + Scope::retypeSymbol"
    seam, so the Rust harness reports "Error: Did not apply tests in
    pointerrel.xml" and 0/8 "Relative pointers" assertions are applied (the C++
    oracle passes all 8). No Rust test drives `op_ptrsub_ir` with a
    `TypePointerRel`-typed varnode, so the 9 `push_type_pointer_rel_ir`
    insertions are transcribed-but-unexercised. The console command itself DOES
    execute end-to-end ("Successfully created pointer: myptroff"), and
    `getPtrInto`/`getPtrToFromParent` ARE exercised by unit + verifier tests.
    -> cited as LOSS-163. Dormant-downstream foundation, the
       LOSS-156/158/159/160/162 pattern. Faithful leaf, deferred capability.
       cpp: decompiler/cpp/printc.cc:985-1157, decompiler/datatests/pointerrel.xml
       rust: rust/crates/kuna-decomp/src/printc.rs:2889-3017 (op_ptrsub_ir relptr
             insertions, unexercised); the `retype`/`parse_type` engine seam upstream

## adversarial tests

`rust/crates/kuna-decomp/tests/verify_w10_rel_pointer.rs` (3 tests, committed to
the branch as be7d0db; mutation-checked — dropping the `getPtrInto` UNION arm
makes `w10rel_get_ptr_into_struct_union_and_scalar_arms` fail):
  - `w10rel_get_ptr_to_from_parent_hole_offset_is_unknown1_not_base` — positive
    hole/escaping/0/negative offsets all → `getBase(1,TYPE_UNKNOWN)`, never the
    original container (the do-while at-least-once + null-fallback boundary).
  - `w10rel_get_ptr_to_from_parent_nested_struct_drills_to_leaf` — nested struct
    forces the do-while to iterate (`curoff != 0`) into the inner leaf field;
    exact-hit returns the nested struct, mid-field → unknown1.
  - `w10rel_get_ptr_into_struct_union_and_scalar_arms` — STRUCT→(ptrto,0),
    UNION→(ptrto,0), scalar→(parent,offset), plain Pointer→(ptrto,0).

## losses

LOSS-163 (the dormant end-to-end `ADJ(...)` rendering + untested
`op_ptrsub_ir` relptr arm; gated on the `retype`/`parse_type` engine seam).
