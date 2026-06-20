# Verdict: rport/w10-subpiece-cast — ACCEPT-WITH-LOSSES

**Item:** `PrintC::opSubpiece` truncation-as-cast rendering
**Branch:** `rport/w10-subpiece-cast` (`00b31fe`, on top of `rust-port` @ `582e067`)
**Verifier round:** 2 (Round 1 was REJECT — see history below)
**Diff scope:** `rust/crates/kuna-decomp/src/printc.rs` (+109) and
`rust/crates/kuna-decomp/src/cast/tests.rs` (+91). No other files.

## Verdict

**ACCEPT-WITH-LOSSES.** This Round-2 diff does exactly what the Round-1 verdict
prescribed: it **defers** (un-wires) the cast render so the `condconstsub`
regression is avoided, and it commits **5** adversarial tests (the gate requires
>=3). The transcription is faithful, free of special-casing, touches no reserved
file, and produces **zero** observable change to live output — the whole-suite
passing SET is byte-for-byte identical to base. The only "loss" is that the two
corpus wins (Bitfields #4, Union #28) are deferred until the unported upstream
dataflow elimination pass lands. Recorded as **LOSS-198**.

This is the same shape as LOSS-193 (w10-bitfield-activate) and LOSS-197
(w10-forloop-stack): a faithful, complete transcription held dormant on an
unported prerequisite — a faithful PARTIAL with zero regression.

## Hunt list

- **Signedness:** clean. The decision uses no signed/unsigned comparison; the
  in/out metatype checks are enum equality. The offset is `uintb`-lineage cast
  to `uint4` (matching C++).
- **Integer widths:** clean. `offset = in1->get_offset() as uint4` matches C++
  `(uint4)op->getIn(1)->getOffset()`; the `as uint4` carries a justification
  comment (printc.rs:2991-2992).
- **Wrapping:** N/A. No arithmetic on the SUBPIECE path — only a width-narrowing
  cast of the constant offset, which is the intended C++ truncation.
- **Comparator totality:** N/A (no comparator ported).
- **Iteration-order provenance:** N/A (no container iteration in the added code).
- **Off-by-one / do-while / reverse iterators:** N/A.
- **Erase-while-iterating:** N/A.
- **Exception -> Result partial-state:** clean. Both bail paths
  (`cast_strategy_for(arch) == None`, `subpiece_cast_facing == None`) degrade to
  `op_func_ir` via `?`-propagation / early `return`, matching the pre-cast
  dispatch default. No partial RPN state is left.

## Evidence

### Faithfulness — PASS

`op_subpiece_ir` (printc.rs:2949) is a symmetric transcription of
`PrintC::opSubpiece` (printc.cc:892-897):

```text
  if (castStrategy->isSubpieceCast(op->getOut()->getHighTypeDefFacing(),
                                   op->getIn(0)->getHighTypeReadFacing(op),
                                   (uint4)op->getIn(1)->getOffset()))
    opTypeCast(op);
  else
    opFunc(op);
```

- The cast TARGET TYPE flows from the **output** varnode's def-facing high type:
  `op_type_cast_ir` (printc.rs:2806) reads `out.get_type_def_facing()`, exactly
  C++ `opTypeCast`'s `getOut()->getHighTypeDefFacing()`. Not hardcoded, not
  opcode-keyed, not tuned to one case. (This is the same `op_type_cast_ir`
  already used and verified by the merged SEXT/ZEXT cast-render path.)
- `is_subpiece_cast` (cast.rs:849) is the pre-existing W8 port of cast.cc:411-432,
  verified line-by-line: `offset!=0 -> false`; in-metatype gate
  {INT,UINT,UNKNOWN,PTR,PARTIALSTRUCT,PARTIALUNION}; out-metatype gate
  {INT,UINT,UNKNOWN,PTR,FLOAT}; the PTR-in far->near pointer arm. This wave
  REUSES it, does not re-implement it.
- The offset is `in1->get_offset() as uint4` (printc.rs:2993), matching C++
  `(uint4)...getOffset()`.
- The `doesSpecialPrinting()` structured-field branch (printc.cc:866-890) is
  honestly declared a fall-through-to-functional SEAM (printc.rs doc), not faked.

### No special-casing — PASS

Greps of the added EXECUTABLE lines for `0x` / `== N` / `!= N` / `field5` /
`longField` / `CPUI_` / `"SUB"` / function-name literals: clean. The only matches
are in doc-comments DESCRIBING the corpus boundaries (Bitfields #4, Union #28,
condconstsub) — none in code. No size/offset/opcode-operand/function-name
hardcoding. No `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/
`unwrap`/`panic` in the added code (it uses `?` Option-propagation).

### New passes are REAL parity / NO REGRESSION — PASS (zero-delta, by construction)

The cast arm is **UNWIRED**: there is no live `CPUI_SUBPIECE =>` arm in
`push_op_ir`'s match (printc.rs:2552-2566 is the documenting comment only).
SUBPIECE falls through to `op_emit_kind` -> `OpEmitKind::Custom` -> `op_func_ir`,
so it renders functional `SUB(x,off)` exactly as on base. `op_subpiece_ir` /
`subpiece_cast_facing` are `#[allow(dead_code)]`.

Full corpus re-run (83 datatest files, 675 assertions) with the same built `.sla`,
Rust `decomp_test_dbg`, base (`582e067`) vs branch (`00b31fe`):

```
base   (rust-port @ 582e067): Total tests applied = 675, Total passing tests = 298
branch (00b31fe):             Total tests applied = 675, Total passing tests = 298
```

Passing-SET diff (full 675-line result set, byte-sorted):

```
diff base_set branch_set  ->  IDENTICAL (no line differs)
comm -13 (new passes):   EMPTY
comm -23 (lost passes):  EMPTY
```

So the live output is byte-for-byte unchanged: **0 regressions, 0 new passes.**
`Bitfields #4`, `Union #28`, and `Modified conditional constant #4` all sit at the
same status on both sides. The condconstsub negative assertion (`\(int4\)ptr`
min=0/max=0, condconstsub.xml:22) stays PASSING because the cast is not wired —
the Round-1 regression is gone. boolless / readstruct / condconst(sub) bodies are
byte-identical (covered by the identical full-set diff and the green corpus
self-test). Inspected a real SUBPIECE case: the offset-0 `void* -> int4` SUBPIECE
on the condconstsub path renders functional `SUB(...)`, matching the absence of
`(int4)ptr` the oracle requires.

### Other gates

- **Reserved files** (blockaction.rs, coreaction_stackptr.rs, funcdata.rs
  stack-ptr helpers — the concurrent for-loop/stack wave): **NOT touched.** Diff
  is printc.rs + cast/tests.rs only. PASS.
- **`cargo test --workspace`:** green, 0 failed across all crates (the 1709-test
  kuna-decomp lib suite includes the 5 new tests). PASS.
- **`cargo clippy -p kuna-decomp --lib`:** clean (0 warnings on the shipped
  library, despite the `#[allow(dead_code)]` seam). PASS. (The `--tests` run
  surfaces only pre-existing doc-list / literal-bool warnings in unrelated W3-W10
  test files, none in this diff's added code.)
- **C++ oracle 207/207 + 675/675 PARITY OK:** the diff is `rust/`-only, so the
  C++ source tree is byte-identical to base. Confirmed via
  `python -m kuna.run_tests --all --baseline docs/baseline.json` -> PARITY OK
  (207/207 unit, 675/675 datatest). PASS.
- **>=3 adversarial tests committed:** PASS (5 added, all passing).

### Why dormant rather than wired (the loss)

In condconstsub.xml an `override flow ... callreturn` makes C++ recover the
call's return value (`return v1;`) and ELIMINATE the offset-0 `void* -> int4`
SUBPIECE of the pointer BEFORE `opSubpiece` is reached (the conditional-constant /
return-value-merge elimination). The Rust engine has not ported that pass, so the
phantom SUBPIECE survives to print. `is_subpiece_cast(int4, void*, 0)` returns the
SAME `true` as the genuine wins (PTR-in/INT-out arm, cast.cc:423-431) — there is
**no faithful printer-level discriminator** between the phantom and a real
truncation. Wiring the cast would emit `(int4)ptr` where the oracle emits nothing,
regressing `Modified conditional constant #4`. The fix belongs in the missing
dataflow pass; deferring the wiring is the correct, faithful choice.

## Findings

None blocking. The deferral is faithful and ledgered.

- F1 (info): the cast render is ported-but-unwired, delivering 0 net corpus
  passes this wave. Tracked as LOSS-198 with an explicit restoration criterion.
  cpp: decompiler/cpp/printc.cc:892-897
  rust: rust/crates/kuna-decomp/src/printc.rs:2949 (`op_subpiece_ir`,
        `#[allow(dead_code)]`)

## Adversarial tests added (5, all passing)

In `rust/crates/kuna-decomp/src/cast/tests.rs`, module
`w10_subpiece_cast_decision`:
- `win_bitfields4_uint4_to_uint1_offset0_casts` — uint4->uint1 offset 0 -> cast.
- `win_union28_int8_to_int4_offset0_casts` — int8->int4 offset 0 -> cast.
- `condconstsub_voidptr_to_int4_offset0_also_casts_so_printer_cannot_discriminate`
  — void*->int4 offset 0 -> cast, asserts the verdict EQUALS the real wins (the
  no-discriminator proof, the crux of the BLOCKED status).
- `nonzero_offset_extraction_is_functional_not_cast` — offset 4 / offset 1 ->
  functional (the `opFunc` arm).
- `struct_output_metatype_falls_through_to_functional` — STRUCT out-metatype ->
  functional fallthrough.

## Losses

LOSS-198 (appended to docs/rust-port/losses.md): faithful PARTIAL — the
`opSubpiece` cast decision is ported but held unwired to avoid the condconstsub
regression; the two corpus wins are deferred to the dataflow-elimination pass.

## Round-1 history

Round 1 (`6502bc1`) REJECTed because the cast was WIRED (regressing `Modified
conditional constant #4`, +2/-1 net) AND committed zero adversarial tests. Round 2
un-wires the cast (restoring that assertion, byte-identical set) and adds 5 tests
— precisely the path the Round-1 verdict laid out for ACCEPT-WITH-LOSSES.
