# w10-splitdatatype
verdict: ACCEPT
verifier: opus-4.8 (independent)
date: 2026-06-16
gate: `cd rust && cargo test -p kuna-decomp` -> 2696 passed / 0 failed (incl. 3 new adversarial tests); `cargo test --workspace` -> 3556 passed / 0 failed / 37 ignored; `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean; C++ oracle `kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK (untouched — diff has zero C++ files)

## Scope

Ports the `SplitDatatype` graph-mutation build path (subflow.cc 2098-2954):
`RootPointer::{backUpPointer,find,duplicateToTemp,freePointerChain}`,
`SplitDatatype::{testCopyConstraints,generateConstants,buildInConstants,
buildInSubpieces,buildOutVarnodes,buildOutConcats,buildPointers,
isArithmeticInput,isArithmeticOutput,splitCopy,splitLoad,splitStore,
getValueDatatype}`, the three `RuleSplit{Copy,Load,Store}::applyOp` bodies, and
`Funcdata::buildCopyTemp` (funcdata_op.cc:1167).  Removes the prior W6 seam stubs
(`split_datatype_seam`, `split_copy_facing_seam`).  Carries
`split_datatype_config` through the Architecture seam (default struct|array|
pointer).  4 files, all under `rust/`.

## Faithfulness (transcription verified line-for-line against the C++)

- `buildCopyTemp` vs funcdata_op.cc:1170-1218: descend scan for a COPY-to-
  IPTR_INTERNAL non-type-locked output; same-parent seq-order test; cross-block
  `findCommonBlock` 3-way (point ancestor / other ancestor / neither->fresh COPY
  at common->getStop + opInsertEnd); fresh-COPY-before-point fallback;
  totalReplace+opDestroy of the superseded otherOp.  Exact.
- `backUpPointer` vs 2098-2134: PTRSUB/INT_ADD/PTRADD const-in1 + COPY(off=0)
  arms; parent-metatype STRUCT/ARRAY gate with the impliedBase escape
  `(opc!=PTRADD && opc!=COPY) || parent!=impliedBase` (identity, `Rc::ptr_eq`);
  PTRADD `off *= in2`; `addressToByteInt`; `baseOffset += off`.  Exact.
- `find` vs 2144-2180: PARTIALSTRUCT->parent, ARRAY->base sets impliedBase; the
  `ptrTo != valueType` (identity) one-hop backup with the `impliedBase!=0 ->
  false` short-circuit; the 3-iteration containing-struct/array backup loop with
  the addrTied/loneDescend==null break.  resolver.addResolution sites replaced by
  documented no-ops (every ResolveCache method short-circuits on
  `!needsResolution()`; the non-union corpus never resolves) — faithful.
- `splitCopy` vs 2728-2761, `splitLoad` vs 2770-2815, `splitStore` vs 2823-2913:
  constraint/compatibility/arithmetic guards, the LOAD-into-lone-COPY deferral,
  the splitStore retry-without-LOAD path (`dataTypePieces.clear()` then re-test),
  the duplicateToTemp-when-root-addrTied, the "preserve original STORE for
  INDIRECTs, convert to first smaller STORE" opSetInput(0..2) + opInsertAfter
  chain, and the freePointerChain teardown order.  Exact.
- `generateConstants` vs 2413-2469: ZEXT(c)/PIECE(c1,c2) detection, hi/lo/losize
  extraction, the `dt->getSize() > sizeof(uintb)` clear+false bail, and the shift
  arithmetic `sa>=losize ? hi>>(sa-losize) : lo>>sa*8 (| hi<<(losize-sa)*8)` —
  including upstream's `hi>>(sa-losize)` with no `*8` (transcribed verbatim, not
  "fixed").  Confirmed by adversarial test (0x3344 / 0x1122 little-endian split).
- `buildOutConcats` vs 2552-2606: setProtoPartial marking, the BE/LE PIECE-stack
  build (MSB in0 / LSB in1), the `i<=0 break` post-concat little-endian loop,
  setPartialRoot + opSetOutput + registerProtoPartialRoot.  The `len()==1`
  out-of-bounds is unreachable (callers guarantee `dataTypePieces.size()>1`).
- `buildPointers` vs 2620-2683: the `do{}while(tmpType->getSize() >
  matchType->getSize())` ported as `loop{...; if <= break}` (at-least-once
  preserved); the array-offset `curOff%size` (+size if neg) arm; PTRADD
  (tmpType==resType || ARRAY) vs PTRSUB branch; `byteToAddressInt`;
  getTypePointerStripArray.  Exact.
- `getValueDatatype` vs 2925-2953: TYPE_PTR guard, isPointerRel->parent/byteOffset
  vs ptrTo/0, the `alignSize<size` INT/UINT/BOOL/FLOAT/PTR getTypeArray arm, the
  STRUCT/ARRAY getExactPiece arm (nullable, `.ok().flatten()`).  Exact.

## Mandatory hunt list

- Signedness: clean.  `sa`/`losize`/`curOff` int4/int8 throughout; the BE `sa`
  could go negative only for out-of-range pieces that the corpus never produces
  (C++ would UB-shift; Rust would debug-panic) — same realistic domain, faithful.
- Integer widths: clean.  size->int8 widenings (`get_size() as int8`,
  `base_offset as int8`) are non-lossy and match the C++ `int8` arithmetic.
- Wrapping: clean.  The two value-shifts use `wrapping_mul(8)` / `wrapping_shl`,
  matching C++ silent-wrap on the `sa*8` / `(losize-sa)*8` group shifts; the
  pointer/index arithmetic is bounded.
- Comparator totality: N/A (no operator< ported).
- Iteration order: clean.  Every loop is over `dataTypePieces` (Vec, insertion
  index) or `descend_iter()` (the varnode descend list, insertion order, snapshot
  to Vec before the &mut borrow — same elements, same order); no HashMap/HashSet.
- Off-by-one / do-while / reverse iters: clean.  `buildPointers` do-while kept
  at-least-once; `buildOutConcats` `i<=0 break` post-body; `find` 3-iter loop.
- Erase-while-iterating: clean.  `freePointerChain` walks first->pointer
  destroying defs after reading getIn(0); `buildCopyTemp` totalReplace+opDestroy
  of otherOp after the new out is wired — same sequencing as C++.
- Exception->Result partial-state: clean.  `?` points are pre-mutation
  allocations (new_op/new_constant) whose failure is a true KunaError; the
  Ok(false) early-returns leave `dataTypePieces` as C++ leaves it (splitStore's
  explicit `clear()` on the retry path is reproduced).

## Special-casing audit

NO special-casing.  Greps over the added lines for field-name / type-name /
offset / function-name literals and for non-message string literals: zero hits.
All string literals are panic/error text; all numeric literals are opcode
arities (new_op 1/2/3), input slots (get_in(0..2)), the `4`-byte SUBPIECE offset
const (== C++ `data.newConstant(4, off)`), and the `8`=`sizeof(uintb)` bound.

## Reserved-file / oracle / no-regression

- Reserved files (printc.rs, funcdata_varnode.rs, blockaction.rs, src/block.rs):
  NONE touched.  Diff = architecture.rs, funcdata_op.rs, seams.rs, subflow.rs.
- C++ oracle: zero C++/.cc/.hh files in the diff -> decomp_test_dbg definitionally
  untouched; re-ran from main tree, 675/675 PARITY OK.
- Whole passing SET: base rust-port = 3553 pass / 0 fail; feature = 3556 pass / 0
  fail.  The set strictly GREW by exactly the 3 verifier tests; no prior test
  dropped or flipped.  Well above the 312 floor.
- boolless / readstruct / condconst_conn: the kuna-harness corpus self-test +
  lift-diff structural differential (which back those byte-identical fixtures)
  are green (49+15+3+... all pass); split rules run pre-typing and these
  fixtures are unchanged.

## Findings

- F1 (cosmetic, non-blocking): two pre-existing tests carry now-stale doc
  comments claiming a seam that this wave removed —
  `split_load_store_get_value_datatype_is_seam_none` (subflow.rs ~6713) and
  `rule_split_copy_is_seam_gated` (~6730).  Both still PASS with valid
  assertions: the first returns None via the real TYPE_PTR guard (untyped input
  varnode), the second returns 0 via the real metatype gate.  Only the comments
  are outdated; no behavioral regression.  Not a reject reason; flagged for a
  later comment cleanup.
       cpp: decompiler/cpp/subflow.cc:2925, :2962 (now-live paths)
       rust: rust/crates/kuna-decomp/src/subflow.rs:6713, :6730

## Adversarial tests (committed to the branch, 27b566d)

- `w10_splitdt_test_copy_constraints_ladder` — the testCopyConstraints boolean
  ladder: input-Varnode reject; written non-LOAD input accept; LOAD-into-lone-
  COPY reject (deferred to splitCopy).
- `w10_splitdt_generate_constants_zext_and_oversize_bail` — generateConstants
  ZEXT(0x11223344:4)->8 little-endian split yields exactly 0x3344 / 0x1122 (the
  `lo>>sa*8` shift), and a 16-byte piece (>sizeof(uintb)) clears the accumulator
  and returns false.
- `w10_splitdt_get_value_datatype_rejects_non_pointer` — getValueDatatype returns
  None when the read-facing type is a plain int (the TYPE_PTR guard).

All 3 pass.

losses: none (faithful, complete, zero regression).
