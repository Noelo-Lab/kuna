# w5-s3-subflow
verdict: ACCEPT-WITH-LOSSES
verifier: subagent (independent; did not read porter notes)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (23 suites, 0 failed; lib 438 + verify_w5_s3_subflow 5 + all others green)

## Scope

Port commit `e892c53` ports `SubvariableFlow` (the sub-variable replacement
*discovery* engine) + its six trigger rules (`subvar_and`, `subvar_subpiece`,
`subvar_compzero`, `subvar_shift`, `subvar_zext`, `subvar_sext`) from
`decompiler/cpp/subflow.{cc,hh}`.  `SplitFlow`/`RuleSplitFlow`/`LaneDivide`
(TransformManager-derived), `SplitDatatype`/`RuleSplit{Copy,Load,Store}`/
`RuleDumptyHumpLate`, and `SubfloatFlow`/`RuleSubfloatConvert` are out of scope
/ SEAM(W5-transform) and are correctly omitted.

blob-sha drift: NONE — `git hash-object subflow.cc` = `d1e17ee…` and `subflow.hh`
= `1c87c94…` match the checklist `cpp_blob_sha` exactly.  (port-audit crate does
not exist in the tree yet; drift verified by hand.)

## Hunt list

- Signedness — CLEAN.  `int4`/`uintb` operands map faithfully.  The mixed
  `sa >= sizeof(uintb)*8` (`int4` vs `size_t`) comparisons are reproduced as
  `sa as u64 >= size_of::<uintb>() as u64 * 8`, which matches the C++ unsigned
  promotion (a negative `sa` becomes huge and triggers the guard, as in C++).
- Integer widths — CLEAN.  `(int4)getOffset()` truncations preserved;
  `flowsize`/`bitsize`/`sa` arithmetic stays i32 as in C++.  `bitmask as uintb`
  = C++ `(uintb)bitmask`.
- Wrapping — see F1 (LOSS-064).  All *guarded* shifts (the trace functions, which
  carry the `sa >= sizeof(uintb)*8` early-out) use bare operators safely; the
  *unguarded* sites (RuleSubvarShift, backward SUBPIECE/PIECE, forward INT_MULT
  `&~7`) are the only wrap-prone ones and are pre-existing C++ UB.  No
  `wrapping_*` that should be `checked_*`.
- Comparator totality — N/A (no `operator<`/sort/BTree comparator ported here;
  `varmap` keys on `VarnodeId`, an integer newtype).
- Iteration-order provenance — CLEAN.  `worklist` is LIFO (`Vec::push`/`pop`) =
  C++ `vector` `push_back`/`back`+`pop_back`.  `traceForward`/`traceForwardSext`
  iterate a `descend_snapshot(rvn_vn)` (Vec, insertion-order) reproducing the C++
  `beginDescend()..endDescend()` list order; the descend list is NOT mutated
  during the trace (mutations are placeholder-only, deferred to `doReplacement`),
  so the snapshot is equivalent.  `getRepeatSlot` is fed `&descend[..idx]` — the
  exact `[beginDescend, iter)` prefix the C++ counts over (op.cc:97).  `doTrace`'s
  mark-clear walks `varmap.keys()` (BTreeMap) but clearMark is idempotent and the
  key-set equals the C++ marked-vn set, so order is irrelevant.  No `HashMap`/
  `HashSet` anywhere in the ported logic.
- Off-by-one / do-while / reverse iterators — CLEAN.  No `do{}while`; the
  `RuleSubvarAnd` `cmask >>= 8` scan is a `loop { if … break }` that matches the
  C++ `while(cmask!=0)` (the C++ `>>=8` happens before the loop, then inside —
  reproduced exactly).  `createOp`'s "already-defined" early-out, `createOpDown`'s
  `while(input.size()<=slot) push(None)` slot-grow, and `createLink`'s slot==-1
  output wiring all match.
- Erase-while-iterating — N/A here (no erase during the trace; `addPush` does a
  front-insert `patchlist.insert(0,…)` = C++ `push_front`, order preserved).
- Exception -> Result partial-state — CLEAN.  Seam `Err`s propagate via `?`.  The
  `try*` seams that mutate sibling state (`tryReturnPull`'s cross-RETURN loop) are
  gated *whole* (Err before any worklist/patchlist write), so no partial sibling
  state.  `do_trace`'s mark set is cleared symmetrically even on the seam path.
- AND-rule pitfalls (transposed conditions / swapped slots / inverted returns /
  missing early-outs) — CLEAN.  Re-derived rule-by-rule: `doesOrSet`/`doesAndClear`
  index selection, the forward INT_AND mask-extract terminal, the bitsize==1
  boolean-movement `booldir` flip under INT_EQUAL, the `1-slot` other-side picks,
  the backward INT_ADD `mask==1 -> XOR` split, the `tryCallReturnPush` `mask&1`/
  `bitsize<8` guards, and the `CPUI_BOOL_*` forward case deliberately NOT bumping
  hcount (subflow.cc:632-639) are all faithful.

## Mechanical pass

- grep: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`.  The only
  bare `as`-into-`uintb` cast (`bitmask as uintb`, :1837) is C++ `(uintb)bitmask`;
  all other `as` are width conversions (`as int4`/`as u64`/`as usize`/`as uint4`).
- `cargo clippy -p kuna-decomp` (and `--tests` for the new file) — clean (no
  warnings on subflow.rs or verify_w5_s3_subflow.rs).
- port-audit — crate absent from the tree; blob-sha drift checked by hand (none).

## Adversarial tests (added, committed to branch as 99646c7)

`rust/crates/kuna-decomp/tests/verify_w5_s3_subflow.rs` (5 tests, all pass):
- `w5s3_subvar_shift_amount_64_panics_where_cpp_is_masked_ub_f1` — pins F1: a
  1-byte INT_RIGHT with constant shift 64 debug-panics (`mask >> 64`) where C++
  returns 0.
- `w5s3_subvar_shift_small_amount_no_panic_returns_zero` — control isolating F1 to
  the shift width.
- `w5s3_forward_int_less_non_aggressive_bails_aggressive_proceeds` — the
  `!aggressive && (nz|mask)!=mask` compare gate.
- `w5s3_backward_int_add_single_bit_input_flag_guard` — backward INT_ADD `mask==1`
  -> XOR adder + the 1-bit-input-flag rejection.
- `w5s3_forward_call_descendant_surfaces_w4_seam` — a CALL(slot=1) descendant
  surfaces the W4 `tryCallPull` seam as `Err` out of `do_trace`.

## Findings

- F1 (minor): unguarded bare `<<`/`>>` in `RuleSubvarShift::applyOp` and the
  backward SUBPIECE/PIECE + forward INT_MULT(`&~7`) shift sites.  Debug-panics on
  a shift count >= width / negative (e.g. a 1-byte INT_RIGHT shift of 64) where
  C++ is x86-masked UB and returns a value.  Output-identical on every
  well-defined input (shift < width); not output-affecting on valid p-code.  Same
  family as LOSS-055.  -> LOSS-064.  Not a blocker.
     cpp: decompiler/cpp/subflow.cc:1694
     rust: rust/crates/kuna-decomp/src/subflow.rs:2254

All other divergences are declared cross-wave seams, ledgered as losses:
- LOSS-062: `doReplacement` + all six rules seamed to no-op (W3-funcdata/W6
  mutation); discovery (`doTrace`) fully executable.
- LOSS-063: `tryCallPull`/`tryReturnPull`/`tryCallReturnPush`/`tryInt2FloatPull`/
  `isZeroExtended` seamed inside `doTrace` (W4/W6/W3-varnode) — CALL/RETURN/
  INT2FLOAT/DIV/REM flows cannot be discovered.
- LOSS-065: `RuleSubvarSext::reset` hard-codes `isaggressive=false` vs C++
  `aggressive_ext_trim` (W4 arch seam; matches the C++ default).

## Why ACCEPT-WITH-LOSSES (no blocker stands)

The discovery engine — the load-bearing, output-determining half (worklist push
order, descendant visit order, per-opcode trace/create/patch structure, the bit
helpers, the patch builders, the constructor flowsize derivation) — is a faithful
statement-for-statement transcription; re-derived against the C++ with the hunt
list and confirmed by both the in-module suite and the new adversarial tests.
The seamed halves (`doReplacement` and the W4/W6 `try*`/`isZeroExtended` edges)
follow the established LOSS-053/054/057 seam pattern (structure transcribed,
mutation deferred, no invented behavior, no observable partial state at this
boundary).  The single code-level finding (F1) is a debug-only panic on inputs
where C++ is itself UB, identical on all valid inputs — a minor loss, not a
blocker.

losses: LOSS-062, LOSS-063, LOSS-064, LOSS-065
