# w10-float-family
verdict: ACCEPT
verifier: opus-4.8-1m (independent verifier, round 1)
date: 2026-06-15
branch: rport/w10-float-family @ 2b4371f
gate: cargo test --workspace -> 3287 passed / 0 failed (was 3279 + 8 new verifier tests);
      cargo clippy -p kuna-decomp --lib -- -D warnings -> clean;
      C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` ->
      207/207 unit + 675/675 datatest, PARITY OK (untouched — branch makes zero C++ changes).

## Scope

The diff (78 lines, two files, only `rust/`) closes one printer seam:
`PrintC::opFloatInt2Float` (printc.cc:850) + its helper
`TypeOpFloatInt2Float::absorbZext` (typeop.cc:1874). FLOAT_INT2FLOAT now renders
as a `(floatN)input` type-cast (not a functional `FLOAT_INT2FLOAT(x)`),
absorbing an implied INT_ZEXT on its input. Dispatch is routed via the public
`op_emit_kind` table cell (FLOAT_INT2FLOAT -> Custom) into the new
`op_float_int2float_ir` arm.

C++ blob shas at review (rust-port tree): typeop.cc 025b8430, printc.cc c776ae0a,
float.cc d4632819, coreaction.cc 28f6157f.

## Faithfulness (transcription, not a shortcut)

- `absorb_zext` (printc.rs:3238) is a line-faithful transcription of
  `TypeOpFloatInt2Float::absorbZext` (typeop.cc:1874): `in0.isWritten() &&
  in0.isImplied()` then `def.code() == CPUI_INT_ZEXT` -> return the ZEXT, else
  None. `is_written`/`is_implied` flag accessors verified against varnode.hh
  282/236 (same flag bits).
- `op_float_int2float_ir` (printc.rs:2469) matches opFloatInt2Float: pick
  `zextOp.getIn(0) ?? op.getIn(0)`; if `!option_nocasts` push TYPECAST + the
  output's def-facing type, then pushVn(vn0). The `option_nocasts` suppression
  arm is present and faithful (printc.cc:856).
- The cast type uses `get_type_def_facing()`; the *HighVariable* indirection of
  C++ `getHighTypeDefFacing()` (varnode.cc:670, `high->getType()`) is a
  pre-existing crate-wide W8 seam (varnode.rs:586 returns the varnode's own
  `type_`, same as every other `*_facing` accessor in the merged tree) — NOT a
  shortcut introduced here. Consistent with the rest of the printer.
- `push_cast_type` (printc.rs:2498) reuses the already-reviewed `declarator_parts`
  (pub(crate)) to emit the base-type front; for a scalar `floatN` (typestack
  size 1) C++ `pushType`->`pushTypeStart(noident)` emits exactly the display
  name, which `declarator_parts` reproduces as `("float8","")`. Faithful for the
  only form an int->float cast produces (a scalar float).

## No special-casing

Grepped the full diff: NO function-name / address / magic-value / type-name
hardcoding. The only `floatN`/`float8` literals are in doc comments and in MY
test oracle. The change is one opcode-keyed table cell + opcode-keyed dispatch;
the routing decision (`op_emit_kind`) is a pure function of the OpCode with no
per-value branch. The `(floatN)` text is derived from the varnode's datatype,
not a literal. PASS.

## Mandatory hunt list

- Signedness: n/a — no signed/unsigned comparisons in the ported logic (pure
  pointer-chasing op->in0->def->code). clean.
- Integer widths: n/a — no integer arithmetic. clean.
- Wrapping: n/a — no arithmetic. clean.
- Comparator totality: n/a — no comparators added. clean.
- Iteration-order provenance: n/a — no container iteration added (the only loop,
  `declarator_parts`, is pre-existing and walks a base-up modifier stack by
  index, not a HashMap). No HashMap/HashSet/sort_unstable in the diff. clean.
- Off-by-one / do-while / reverse iterators: n/a — no loops. clean.
- Erase-while-iterating: n/a — no mutation of containers. clean.
- Exception -> Result partial-state: the ported fns are pure reads; `?`/`Option`
  combinators only short-circuit to None (no partial mutation). C++ `absorbZext`
  derefs `getDef()` unguarded after `isWritten()`; Rust uses `get_def()?` —
  identical on well-formed IR (written varnode always has a def), more defensive
  on malformed IR. Benign, noted as F2.

## Mechanical pass

- grep diff for todo!/unimplemented!/HashMap/HashSet/sort_unstable/bare `as`:
  none in the diff (the one `as_ffi() as usize` is the pre-existing `vn_key`
  helper, NOT in the diff). All `unwrap()`s in the new code are in MY test
  scaffolding (matching funcdata_op/tests.rs style); the ported logic has no
  unwrap/expect/panic.
- `cargo run -p port-audit`: port-audit is not a package in this workspace
  (protocol artifact absent here) — not run; substituted with the full hunt-list
  greps above.
- clippy -p kuna-decomp --lib -D warnings: clean.

## Findings

- F1 (info): cast type uses the varnode's own `type_` via `get_type_def_facing`,
  not the HighVariable `high->getType()` of C++ `getHighTypeDefFacing`. This is
  the documented crate-wide W8 HighVariable seam, identical to every other
  `*_facing` accessor; NOT introduced by this diff. No output divergence in the
  merged (no-HighVariable) tree.
       cpp: decompiler/cpp/varnode.cc:670
       rust: rust/crates/kuna-decomp/src/varnode.rs:586
- F2 (info, benign): `vn0` fallback differs only on malformed IR. C++
  `zextOp ? zextOp->getIn(0) : op->getIn(0)` would pass a null in0 to pushVn for
  an INT_ZEXT with zero inputs; Rust `.and_then(get_in(0)).or(in0)` falls back to
  the original in0. Unreachable on well-formed IR (a real INT_ZEXT always has
  in0). Rust is strictly more defensive; identical observable behavior.
       cpp: decompiler/cpp/printc.cc:854
       rust: rust/crates/kuna-decomp/src/printc.rs:2473

## Recovery is REAL (live in the corpus, not a decode shortcut)

The new `(floatN)` arm is NOT dormant — it fires end-to-end on real corpus
decompiles. Driving the committed Rust `decomp_test_dbg` (KUNA_ENGINE=rust,
KUNA_DUMP=1) over the float-cast datatests produces genuine int->float casts
from the new arm:
  - floatconv.xml -> `v1 = (float4)(fval + 0xfffffff0);`, `v1 = (float8)RAX;`
  - mixfloatint.xml -> `(float8)EDX`, `(float8)R9D`, `(float4)EDX` (6 casts)
  - union_datatype.xml -> 1 cast.
These `(floatN)var` forms match the C++ oracle's FLOAT_INT2FLOAT rendering
(C++ `(float8)a1` <-> Rust `(float8)EDX`/`(float8)R9D` for the same conversion).
The functions are NOT byte-identical to the oracle, but that residual is the
PRE-EXISTING raw-IR gap (LOSS-130: register names, `SUB(...)`/`CONCAT(...)`
functional forms, no clean variable recovery), NOT this item — the
FLOAT_INT2FLOAT-specific cast text the diff added is correct and matches. So
this item ADVANCES parity on the float corpus; it introduces no new dormant
foundation (contrast the LOSS-156/163 pattern).

The dispatch + absorption logic was additionally mutation-tested:
- dropping the `is_implied()` half of the C++ AND -> `absorb_zext_negative_not_implied` FAILS.
- broadening the opcode check to also accept INT_SEXT -> `absorb_zext_negative_wrong_opcode_intsext` FAILS.
Both mutations are caught; the original passes. The tests exercise the actual
ported `absorb_zext` over a hand-built Funcdata (INT_ZEXT def -> written+implied
varnode -> FLOAT_INT2FLOAT in0), not a fixture string.

(The INT_ZEXT-absorption path — `(floatN)` over an unsigned source — is the one
sub-feature the corpus does not yet clearly exercise in a type-recovered form;
it is covered by the in-crate adversarial tests over a hand-built ZEXT, and the
absorption is line-faithful to typeop.cc:1874.)

## Prior byte-parity functions (regression guards, all green)

- boolless: `print_b5_boolless` (11 passed) + `verify_w10_emptyblock_orform::
  w10_eob_boolless_not_perturbed_byte_identical` — byte-identical to C++ B5 oracle.
- condconst_conn: `verify_w10_emptyblock_orform::
  w10_eob_condconst_conn_forms_andand_byte_identical` — byte-identical.
- readstruct / struct corpus: `verify_w10_struct_corpus` (16 passed).
- printc_parity gate: 1 passed.

## Adversarial tests added (8, committed with the verdict)

In-crate (reach the PRIVATE ported logic), `src/printc/tests.rs::w10_float_family`:
  - absorb_zext_positive_implied_written_intzext
  - absorb_zext_negative_not_implied        (mutation-caught: drops isImplied)
  - absorb_zext_negative_wrong_opcode_intsext (mutation-caught: INT_SEXT != INT_ZEXT)
  - absorb_zext_negative_free_input          (isWritten false -> getDef never reached)
  - cast_type_text_scalar_float_is_display_name  ((float8)/(float4) text, no stray modifiers)
Integration (public dispatch surface), `tests/verify_w10_float_family.rs`:
  - w10ff_float_int2float_routes_to_custom
  - w10ff_sibling_float_ops_keep_their_kinds  (anti-special-casing on the table)
  - w10ff_emit_kind_is_total_no_panics

losses: none.
