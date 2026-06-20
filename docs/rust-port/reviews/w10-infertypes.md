# w10-infertypes
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier, round 1)
date: 2026-06-13
item: rport/w10-infertypes (ActionInferTypes type-lattice; closes LOSS-131 roadmap step 4 / the LOSS-137 `undefined1`->`uint1` residual)
gate: `cd rust && cargo test --workspace` -> 0 failures (every crate green; the 51s end-to-end suite + 1601 lib unit + 11 boolless tests all pass)
oracle: `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK (C++ oracle UNTOUCHED)
clippy: `cargo clippy -p kuna-decomp --lib` -> 0 warnings (the new engine file is clippy-clean; pre-existing doc-nit warnings live only in unrelated verify_w3/w4/w5 test files)

## Scope reviewed
- C++: coreaction.cc:5234-5672 (ActionInferTypes — propagationDebug/buildLocaltypes/
  writeBack/propagateTypeEdge/PropagationState/propagateOneType/propagateRef/
  propagateSpacebaseRef/canonicalReturnOp/propagateAcrossReturns/apply);
  varnode.cc:919 getLocalType, :475 updateType; typeop.cc propagateType for all 21
  overriding TypeOp classes (enumerated below).
- Rust diff: `git diff rust-port...rport/w10-infertypes -- rust/` (1001 lines): the new
  `coreaction_infertypes.rs` (709 lines) + the `ActionInferTypes::apply` wrapper wire-up
  in `coreaction_render.rs` + the Rc-shared TypeFactory in `architecture.rs`/`seams.rs` +
  the `temp_type` field on `varnode.rs` + the strengthened boolless byte-parity test.

## The central question — is the `uint1` REAL? — answered YES (proven)
The boolless `print C` now FULLY byte-matches the C++ B5 oracle (`uint1 boolless(void)` /
`uint1 v1; // acc`), with the former `undefined1->uint1` normalization REMOVED and the test
now asserting EXACT oracle equality + `contains("uint1 v1;")` + `!contains("undefined1")`.
Three independent proofs the `uint1` is genuine type propagation, not a hack:
1. **No hardcoded type name in the engine.** grep of `coreaction_infertypes.rs` + the whole
   `src/` analysis path for `"uint1"`/`"undefined1"` literals -> the ONLY `"uint1"` is
   `architecture.rs:856` `set_core_type("uint1", 1, TYPE_UINT, false)` (the legitimate core-
   type-table registration, exactly upstream Ghidra coretypes). The engine emits NO type-name
   string; the printer renders `uint1` only because the varnode carries the interned size-1
   `TYPE_UINT` base.
2. **Causality experiment.** Env-gating an early `return false` into `run_infer_types`
   collapsed boolless back to `undefined1 boolless(void)` / `undefined1 v1;` (the strengthened
   test FAILED with `undefined1` survived); restoring the engine -> `uint1` returns and the
   test passes. The `uint1` is produced ONLY by the engine running over the def-use graph.
3. **Metatype ground truth.** The recovered varnode carries `get_metatype()==TYPE_UINT &&
   get_size()==1` (a string hack would leave `TYPE_UNKNOWN`), and the lattice is monotone:
   a fresh `run_infer_types` pass only refines `(UNKNOWN,1)->(UINT,1)`, never regresses a
   recovered UINT (verifier tests V1/V2 below). The byte-parity is REAL.

## No special-casing — confirmed (REJECT triggers all absent)
grep of the diff for boolless/function-name/address/`uint1`/`dat_52`/`undefined1`/special-case:
every hit is in the TEST file (assertions ON the output) — NONE in the engine. The engine
dispatches purely on `op.code()` + varnode metatype/flags; `verify_w10_infertypes_no_uint1_acc_
leak_into_condconst` proves a different function/arch (condconst, x86-16) does NOT acquire the
`uint1 // acc` artifact, so the result is data-driven, not a constant the engine emits.

## Faithfulness — the lattice control flow is a line-by-line transcription
- `apply` wrapper (coreaction_render.rs): `hasTypeRecoveryStarted` gate, the localcount>=7
  ceiling, `setTypeRecoveryExceeded` + localcount bump, the writeBack-only localcount bump
  (NOT a data-flow `count` change) — all match coreaction.cc:5630-5672.
- `build_localtypes`/`get_local_type` == buildLocaltypes (5262) + getLocalType (varnode.cc:919):
  typelock early-out, def->outputTypeLocal + stopsTypePropagation->blockup, the descend
  inputTypeLocal typeOrder-min, setStopUpPropagation/setTempType.
- `propagate_type_edge` == propagateTypeEdge (5328): inslot==outslot backtrack guard, the BOOL-
  only-propagate `getNZMask()>1` guard, typelock, `stopsUpPropagation && outslot>=0`, annotation,
  the `0>typeOrder` accept + `!isMark` return.
- `propagate_one_type` + the inner PState == propagateOneType (5428) + PropagationState
  ctor/step/valid (5371-5416) — VERBATIM, including the "step before push_back" order and the
  mark-on-push / clear-on-pop visit-once discipline.
- `propagate_across_returns`/`canonical_return_op` == 5598/5567: output-lock early-out, the
  typeOrder-best canonical RETURN, the size/`isBool && NZMask>1`/already-propagated guards.
- `write_back` == writeBack (5297) + updateType (varnode.cc:475): the `Rc::ptr_eq`/typelock
  identity check; the `high->typeDirty()` is relocated from updateType into write_back (the
  only update_type caller needing it) — equivalent side effect.
- The 5 firing propagateType arms (COPY/MULTIEQUAL :412/1953, INDIRECT :2007, the 6 compares
  via propagateAcrossCompare :965, INT_ADD :1183) match their C++ bodies, including the
  spacebase->code-pointer construction.

## Mandatory hunt list
- **Signedness:** clean. `off as i64` (line 452) feeds `AddrSpace::address_to_byte_int(i64,u32)`
  whose C++ counterpart `addressToByteInt(int8,uint4)` takes a *signed* int8 and is called with a
  `uintb offset` — the reinterpret matches C++'s implicit uintb->int8 conversion. (F2: this bare
  cast lacks the protocol-required `// cast:` comment — style only, behavior correct.)
- **Integer widths:** clean. `int4`/sizes flow through typed accessors; no promotion surprises.
- **Wrapping:** clean. No arithmetic on uintb-lineage values in the engine except the cast above;
  `address_to_byte_int` is a pre-existing audited helper.
- **Comparator totality:** clean. `type_order` (pre-existing, dtype.rs:1077) returns a total int4
  order; no float partial_cmp anywhere in the engine.
- **Iteration-order provenance:** clean. `iter_loc()` walks `loc_tree: BTreeMap<LocKey,VarnodeId>`
  (mirrors C++ VarnodeLocSet std::set order); `iter_code(CPUI_RETURN)` walks the ordered
  `returnlist` (C++ beginOp/endOp); `descend_iter()` is the insertion-ordered descend list
  (C++ `list<PcodeOp*>`). NO HashMap/HashSet in the ported logic.
- **Off-by-one / do-while / reverse iterators:** clean. The propagateOneType DFS is a `while let`
  over a Vec stack faithfully reproducing the C++ `while(!state.empty())`; step() is verbatim.
- **Erase-while-iterating:** N/A. The engine mutates only temp types + mark bits during the walk;
  the descend/loc lists are not mutated, and the collect-then-iterate (`descend_iter().collect()`)
  is order-preserving.
- **Exception->Result partial-state:** clean. No `?`-mid-mutation; the `unwrap_or`/`continue`
  guards decline rather than throw, matching the C++ null-return decline semantics. All `.unwrap()`
  are guarded (post-`valid()`, post-`Some(...)`, or non-empty-stack `last_mut()`).

## Mechanical pass
- `coreaction_infertypes.rs`: NO todo!/unimplemented!/HashMap/HashSet/sort_unstable. One bare
  `as` cast (F2, above). `cargo clippy -p kuna-decomp --lib` is 0-warnings.
- `cargo test --workspace`: all crates green; C++ oracle 207/207 + 675/675 PARITY OK untouched.

## findings
  - F1 (loss, accepted -> LOSS-138): 11 of 21 C++ propagateType overrides decline
    unconditionally (LOAD/STORE/INT_XOR/INT_AND/INT_OR/PIECE/SUBPIECE/PTRADD/PTRSUB/SEGMENT/NEW),
    and the relptr-STRUCT bare-pointer compare arm, propagateAddPointer command analysis, union
    resolveInFlow, the type-locked-SymbolEntry seed, and propagateSpacebaseRef/propagateRef/
    applyTypeRecommendations are stubbed. Each is a faithful "no propagation along this edge"
    (= the C++ null-return) and ALL return null on the integer corpus boolless targets, so they
    do not fire on the gate; they are output-affecting on enum/float/pointer/composite/load-store
    programs. Conservative + documented, not silently wrong.
       cpp: decompiler/cpp/typeop.cc:488,559,1424,1457,1490,2076,2163,2270,2368,2433,2503; :974-984; :1217-1270
       rust: rust/crates/kuna-decomp/src/coreaction_infertypes.rs:298-353 (`_ => None`), :374-382, :430-465, :89, :703-706
  - F2 (nit): the module docstring (coreaction_infertypes.rs:31-35) OVERSTATES coverage —
    "ZEXT/SEXT, the non-pointer arithmetic ... is transcribed". ZEXT/SEXT do not override
    propagateType in C++ (their absence is vacuously faithful), but INT_XOR/INT_AND/INT_OR DO
    override (enum/float propagation) and are NOT transcribed. The doc should claim the 5 firing
    arms, not "every comparison + the non-pointer arithmetic". Also the `off as i64` cast at :452
    lacks the protocol `// cast:` justification. Both are documentation/style; the CODE behavior
    is correct and matches C++. Recorded for cleanup in LOSS-138 restoration criteria.

## adversarial tests (3 added by the verifier; land with the verdict)
  rust/crates/kuna-decomp/tests/print_b5_boolless.rs:
  - verify_w10_infertypes_uint1_is_real_metatype_not_string — the recovered `uint1` carries a
    genuine TYPE_UINT/size-1 *metatype* cluster (>=2 varnodes) with the interned `uint1` name;
    a hardcoded string would leave TYPE_UNKNOWN. (Caught and corrected an over-strict initial
    assertion that flagged boolless's legitimately-UNKNOWN intermediate — confirmed harmless.)
  - verify_w10_infertypes_is_settled_fixpoint_refining_the_seed — the lattice is MONOTONE: an
    extra `run_infer_types` pass may only refine (UNKNOWN->UINT, observed at one varnode in this
    partial slice since ActionInferTypes does not bump the data-flow count), NEVER regresses a
    recovered UINT, never changes a size, and the >=2 UINT-1 results persist. (This test's
    investigation directly PROVED the engine's monotonicity and exposed that the byte-parity
    `uint1` is stable.)
  - verify_w10_infertypes_no_uint1_acc_leak_into_condconst — a different function/arch does NOT
    acquire boolless's `uint1 // acc` artifact (anti-special-casing / data-driven proof).

## losses
LOSS-138 (appended this round) — the incomplete propagateType op coverage + the deep-surface
declines. The boolless gate (full B5 byte-parity, `uint1` genuinely inferred) is MET; the
C++ oracle is untouched (675/675 PARITY OK); the un-seam roadmap step 4 (LOSS-131) is closed
and the LOSS-137 `undefined1`->`uint1` residual is resolved.
