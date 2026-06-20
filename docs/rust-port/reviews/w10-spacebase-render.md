# w10-spacebase-render
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8), round 1
date: 2026-06-16
gate: `cargo test -p kuna-decomp` (worktree /tmp/kuna-spacebaserender) -> OK
      (1722 lib + 8 integration tests pass, 0 failed); clippy -p kuna-decomp --lib clean;
      rust-engine datatests 307/675 (passing SET byte-identical to base 13ed4c3);
      C++ oracle 675/675 PARITY OK (untouched).

## Scope

The branch `rport/w10-spacebase-render` (bc5074c) does two things:
1. Adds the `PrintC::opPtrsub` TYPE_SPACEBASE arm (printc.cc:1081-1121) + the
   `ActionNameVars::linkSpacebaseSymbol` (coreaction.cc:3005) /
   `Funcdata::linkSymbolReference` (funcdata_varnode.cc:1214) action that parks the
   Symbol on the offset-constant high — the `&name` render payoff for the W10
   RSP-input spacebase typing.
2. Merges the held `rport/w10-forloop-reroll` (BlockWhileDo::finalTransform/
   finalizePrinting + emitForLoop + isMoveable/moveRespectingCover/markExpression)
   onto the spacebase-typed tree.

Only `rust/` + `PROGRESS.md` touched. No C++ source, no specs, no reserved file.

## The honest stage outcome (verified, not taken on the porter's word)

The brief's premise was that the for-loop files would go RSP-clean and for-loops
would form (new passes: forloop1, ptrtoarray). **That did NOT happen.** The
for-loop reroll is INERT: `For-loop #1` stays FAIL on the rust engine (C++ oracle
passes it), and `ptrtoarray`/`Pointer to array #1-#10` stay FAIL on BOTH base and
branch (the array-spacebase render + param typing are not landed). The blocker is
the deferred `ActionExtraPopSetup::apply` (LOSS-199/201) — the for-loop bodies are
still RSP-dirty, so `BlockWhileDo::finalTransform`'s `has_overflow_syntax()` guard
correctly declines. The porter's own integration tests (a1/a1b/a2) document this
truthfully rather than fabricating a `for (` header.

What the stage DID deliver: the SPACEBASE arm fires and renders `&name` for
MAPPED stack/global symbols — verified on 5 datatests whose render moved toward
the C++ form base->branch (noforloop_alias `i`, partialmerge `&hilo_stack`,
partialsplit `&stackfoo`/`&stackmy`/`&stackconst`, varcross `local_array`,
ptrtoarray). Crucially this added ZERO new passes AND ZERO regressions: the sorted
`Success --` set is byte-identical base(13ed4c3)==branch(bc5074c), 307==307.

## No-regression evidence (the CRITICAL constraint)

- Rust passing SET byte-identical base->branch (`diff` of sorted Success sets
  empty in both directions) — zero regressions, zero new passes.
- Switch/jumptable verdicts IDENTICAL base->branch (Switch Indirect #1-#14,
  Switch Multi/Loop/Hide, If/Switch — full grep diff empty).
- No-for-loop negatives IDENTICAL base->branch; none became a `for` (grep `for (`
  on each of the 5 changed renders == 0).
- boolless / nestedoffset(readstruct) / condconst(condconst_conn) renders
  BYTE-IDENTICAL rust==cpp (the protected files).
- No NEW `&$$undef` leak from the SPACEBASE arm (corpus's single pre-existing one
  in forloop_thruspecial is unchanged on both base and branch).
- C++ oracle 675/675 PARITY OK, no C++ source touched (B0 byte-equal).
- Reserved files (unionresolve_run.rs, dtype.rs) NOT in the diff.

## Mandatory hunt list

- Signedness: CLEAN. Every new comparison (`ind >= num_in`, `num_in <= slot`,
  `order <= point_order`, `overlap >= 0`) is int4-vs-int4; `1 - slot` over a {0,1}
  int4 matches C++.
- Integer widths: CLEAN. `sym_off as int8` is an int4->int8 widening matching C++
  `int4 off` promoted into `pushPartialSymbol`'s int8; no narrowing.
- Wrapping: CLEAN. No unsigned arithmetic on uintb-lineage values in the new code;
  `calc_mask`/offset reads are unchanged helpers.
- Comparator totality: N/A (no new comparator).
- Iteration-order provenance: `link_spacebase_symbol` uses `iter_loc()` (loc_tree
  order, == C++ beginLoc..endLoc, const space first); per-spacebase-varnode
  independence makes the C++ two-pass vs Rust one-pass split immaterial. No
  HashMap/HashSet. `find_loop_variable`/`high_mark_expression`/`testIterateForm`
  use explicit path-stack DFS matching the C++ `PcodeOpNode path[]`/`vector` order.
- Off-by-one / do-while: `op_is_moveable` faithfully ports the C++
  `do{++biter;...}while(biter!=point)` at-least-once forward walk, with a
  DELIBERATE `None => return false` bail for the C++ UB when `point` precedes
  `this` (pinned by `vfy_op_is_moveable_point_must_follow_this`).
  `op_move_respecting_cover` ports the do-while as `loop{...; if cur==last break}`
  (at-least-once preserved). `struct_last_op` matches lastOp() per subtype
  (basic/ls/condition/if) exactly.
- Erase-while-iterating: N/A (descend list snapshotted before the &mut walk; no
  erase during traversal).
- Exception->Result: `link_symbol_reference` returns `false` where C++ throws
  "Unable to generate proper address from spacebase" (an unresolvable address) —
  benign (no Symbol attached, functional fallback), and where C++ `entry==0`
  returns null.

## findings

- F1 (low, faithfulness/inert): `whiledo_final_transform` clears `iterate_op`/
  `initialize_op` (`set_iterate_op(None)`) in the `!allow_op_moves` early-return
  arms; C++ `BlockWhileDo::finalTransform` returns there WITHOUT clearing, leaving
  the member set (re-assigned by `findLoopVariable` that call). Behavioral
  divergence only when a for-loop forms with an unmovable iterate/init op under
  the second (`allowOpMoves=false`) pass — UNREACHABLE while the reroll is inert
  (verified: 0 for-loops form). Recorded LOSS-203 (4).
       cpp: decompiler/cpp/block.cc:3430-3432, 3443-3446
       rust: rust/crates/kuna-decomp/src/funcdata_block.rs (whiledo_final_transform, the `if !allow_op_moves` arms)
- F2 (low, faithfulness/latent): `finalize_forloop_transform`/`_printing` flat-scan
  whiledo nodes in arena-id order rather than the C++ `BlockGraph::finalTransform`
  inner-before-outer recursion order. Order-insensitive for the per-node-independent
  reroll on flat loops; a latent nested-for-loop gap, unreachable while inert.
  Documented by the porter. Recorded LOSS-203 (4).
       cpp: decompiler/cpp/block.cc:1356-1366 (recursion), 3408
       rust: rust/crates/kuna-decomp/src/funcdata_block.rs (finalize_forloop_transform flat scan)
- F3 (low, faithfulness/markup-only): the `off!=0` partial path passes `in1` (the
  offset constant) as the markup-vn to `push_partial_symbol_ir` where C++
  `pushPartialSymbol(symbol,off,0,(Varnode*)0,op,-1,false)` passes a null vn.
  Rendered TEXT identical (markup key only); unexercised by the corpus (no mapped
  composite reaches the `off!=0` arm). Recorded LOSS-203 (3).
       cpp: decompiler/cpp/printc.cc:1116
       rust: rust/crates/kuna-decomp/src/printc.rs (SPACEBASE arm, push_partial_symbol_ir call, `in1.unwrap_or_default()`)
- F4 (info, deliberate loss): `link_symbol_reference` returns `false` for an
  `is_name_undefined` Symbol where C++ always returns the Symbol; an unmapped
  auto-local renders functional `PTRSUB(vN,off)` instead of C++ `&vN`/`&stackNN`.
  Deliberate (the kuna namerec rename can't rename undefined composites; binding
  `$$undefNN` would leak `&$$undefNN`). No new `&$$undef` leak verified. LOSS-203 (1).
       cpp: decompiler/cpp/funcdata_varnode.cc:1232, coreaction.cc:3015/3088
       rust: rust/crates/kuna-decomp/src/funcdata_varnode.rs (link_symbol_reference, `if info.is_name_undefined { return false }`)
- F5 (info, test-quality): the porter's `a4_protected_files_byte_identical_rust_vs_cpp`
  keys on stems "readstruct"/"condconst_conn" which are NOT datatest files, so
  `dump_print_c` returns None and the test silently checks only `boolless`. Not a
  port bug. The verifier added `vfy_r1_real_protected_files_byte_identical` over
  real stems (boolless/nestedoffset/condconst) to cover the intent.
       rust: rust/crates/kuna-decomp/tests/verify_w10_spacebase_render.rs:280-292
- F6 (info, latent panic): `op_move_respecting_cover` does `op_next_op(cur_op).expect(..)`
  — would panic if `op` is the block's last op when a for-loop forms. Unreachable
  while inert; matches the C++ assumption of a following branch op.
       cpp: decompiler/cpp/funcdata_op.cc:1517
       rust: rust/crates/kuna-decomp/src/funcdata_op.rs (op_move_respecting_cover loop, `op_next_op(...).expect`)

## Faithfulness checklist (line-by-line against C++)

- opPtrsub SPACEBASE arm (printc.cc:1081-1121): FAITHFUL — TYPE_ARRAY/TYPE_CODE
  valueon/arrayvalue toggles, `!valueon`->addressof / arrayvalue->subscript,
  off==0->bare name vs off!=0->partial, trailing `[0]`. The `symbol==0`->
  `pushUnnamedLocation` branch is intentionally NOT reproduced (LOSS-203 (1)).
- linkSpacebaseSymbol (coreaction.cc:3005): FAITHFUL — `(!const && !input) return`,
  PTRSUB-only descend, `getIn(1)` offVn. The `isNameUndefined()->namerec` arm
  omitted (kuna namerec model, LOSS-203 (1)).
- linkSymbolReference (funcdata_varnode.cc:1214): FAITHFUL — loneDescend, ptype
  TYPE_PTR, TypeSpacebase TYPE_SPACEBASE, getAddress, queryContainer, off arith.
- finalTransform/findLoopVariable/findInitializer/testTerminal/testIterateForm
  (block.cc:3212-3475): FAITHFUL transcription (path[4]/count==3 bound, slot
  post-increment, head/tail config, marker/moveable guards). See F1/F2.
- isMoveable (op.cc:201): FAITHFUL — special/LOAD/STORE/INDIRECT/CALL switch arms,
  crossCalls computation, tiedList overlap (both directions), do-while-at-least-once.
- moveRespectingCover (funcdata_op.cc:1498) + markExpression (variable.cc:885):
  FAITHFUL — CAST-prev handling, symmetric mark set/clear, call(1)/load(2) retVal.
- emit_for_loop (printc.cc:3106) + emit_block_while_do dispatch (printc.cc:3150):
  FAITHFUL keyword/paren/semicolon/comma-separate/brace sequence; reachable only
  when an iterate op is recorded (never on the corpus).
- struct_last_op (block.cc lastOp per subtype): FAITHFUL basic/ls/condition/if.

## adversarial tests (committed with the verdict)

Verifier (round 1), in rust/crates/kuna-decomp/tests/verify_w10_spacebase_render.rs:
- vfy_r1_real_protected_files_byte_identical — boolless/nestedoffset/condconst
  byte-identical rust==cpp on REAL stems (fixes F5's vacuous check).
- vfy_r1_spacebase_arm_emits_addressof_name_for_mapped_local — partialsplit emits
  `&stackfoo`/`&stackmy`/`&stackconst` AND stays selective (unmapped slot keeps
  functional `PTRSUB(v1,...)`).
- vfy_r1_no_new_addressof_undef_leak_from_spacebase_arm — no `&$$undef` in
  partialmerge/partialsplit/varcross/noforloop_alias/stackstring.

(Porter's a1/a1b/a2/a3/a4 + 8 op_is_moveable unit/vfy tests also pass.)

## losses

LOSS-203 (new, this round): SPACEBASE-arm `is_name_undefined` suppression +
markup-vn (F3/F4) + the merged-but-inert reroll. Cross-references LOSS-197/200
(reroll inertness) and LOSS-199/201 (deferred ActionExtraPopSetup blocking the
RSP cleanup). ptrtoarray header typing remains LOSS-150-class (unchanged).
