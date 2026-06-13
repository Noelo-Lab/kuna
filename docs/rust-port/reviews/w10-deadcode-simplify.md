# w10-deadcode-simplify
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-13
item: w10-unseam-heritage-deadcode (deadcode half — ActionDeadCode + simplify rule bodies)

## Gate
- `cd /tmp/kuna-dc/rust && cargo test --workspace` -> **3157 passed, 0 failed** (8 crates).
- kuna-decomp lib: **1598 passed** (was 1595; +3 verifier adversarial tests).
- B4 gate `tests/deadcode_b4.rs::boolless_b4_deadcode_reduces_toward_oracle` -> **ok** (9 alive ops, down from 21 raw-lifted; no INT_MULT survives; RETURN survives).
- `cargo clippy -p kuna-decomp` (production) -> **0 warnings**. (`--tests`: 39 warnings, ALL in pre-existing other-wave files; none in coreaction_render / deadcode_b4 / ruleaction_3 / the new tests.)
- C++ oracle untouched: `git diff` touches ZERO files under decompiler/ or specs/. `python -m kuna.run_tests --all --baseline docs/baseline.json` -> **207/207 unit, 675/675 datatest, PARITY OK**.

## Faithfulness (the four un-seamed bodies, line-by-line vs C++)
- **ActionDeadCode::apply** (coreaction.cc:4146-4290) -> `deadcode_apply` (coreaction_render.rs): the 6-phase structure is exact — clear-consume + addrforce drop; pre-live seed of non-heritaged deadcode spaces; per-alive-op seeding with the EXACT call / non-assignment(RETURN/BRANCHIND/other) / assignment branch tree (incl. CALL+assignment getting BOTH holdOutput and autolive-output pushes, and NOT the autolive-inputs branch); propagate fixpoint; lastChanceLoad re-seed; the two-phase sweep (vacflag captured before clear, opDestroy vs opUnsetOutput on call, neverConsumed on consume==0). Returns 0 per C++ (change not signaled via count). FAITHFUL.
- **pushConsumed** (3714) -> `dc_push_consumed`: `(val|getConsume())&calc_mask(size)`, vacuous early-return, list-mark, written-only worklist push. The flag-set/push reorder vs C++ is functionally identical. FAITHFUL.
- **propagateConsumed** (3734-3962) -> `dc_propagate_consumed`: every per-opcode transfer arm transcribed — INT_MULT (leastsigbit_set/coveringmask), ADD/SUB, SUBPIECE, PIECE, INDIRECT (outer push applied; inner indirect-source seamed — LOSS-135), COPY/NEGATE, XOR/OR, AND(const), MULTIEQUAL, ZEXT, SEXT(signbit), LEFT/RIGHT(const & non-const), the four compares (NZMask), INSERT, ZPULL/SPULL, POPCOUNT/LZCOUNT, CALL/CALLIND (no-op), FLOAT_INT2FLOAT, default. The `usize_bits` constant = sizeof(uintb) = 8; `8*usize_bits` = 64 bit-width — matches every `8*sizeof(uintb)` site. FAITHFUL.
- **neverConsumed / gatherConsumedReturn / lastChanceLoad / isEventualConstant / clearDeadVarnodes** -> all transcribed verbatim against varnode.cc:873 / coreaction.cc:3971/4033/4064 / funcdata_varnode.cc:850.
- **RuleCollapseConstants::applyOp** (ruleaction.cc:3873) + **PcodeOp::collapse** (op.cc:473) -> `apply_op` + `dc_collapse`: the isCollapsible guard, opMarkNoCollapse-on-error, getConstant->newVarnode->opRemoveInput(high-to-low,keep slot0)->opSetInput(0)->opSetOpcode(COPY)->return 1; the unary/binary OpBehavior dispatch with markedInput always-false (getSymbolEntry W4 seam). FAITHFUL.
- Supporting un-seams (condexe re-read-getSize() loop fix, structureReset->forceRestructure, remove_from_code_list null-opcode guard, op_mark_no_collapse, OpBehavior table wiring) — each is a correct seam-resolution matching its C++ anchor.

## NO SPECIAL-CASING — clean
Grepped the full diff for boolless / "main" / address / function-name / `== "..."` branches in production code: the ONLY `boolless` occurrences are the test fixture name + a comment. ActionDeadCode dispatches PURELY on `op->code()` (the C++ switch); RuleCollapseConstants on isCollapsible + eval-type. No name/address/fixture branch anywhere in the algorithm. The op-count reduction is REAL SSA simplification, not a shortcut.

## Op-count reduction is REAL (spot-checked vs C++ B4)
boolless post-pipeline: 9 alive ops {INT_LESS, INT_NOTEQUAL, 2xCBRANCH, 2xLOAD, INT_ADD, PIECE, RETURN}. The raw lift (21 ops) had constant INT_MULT products (`1*0xff` etc.); RuleCollapseConstants folded them to constants and the deadcode sweep removed the now-dead products — **no INT_MULT survives** (asserted + observed). The remaining 9->7 gap to the C++ B4 is merge/proto-recovery/structuring (MULTIEQUAL, COPY normalization), correctly reported as the remaining layer, NOT closed here. The reduction direction matches the C++ oracle.

## hunt list
- Signedness: clean. `outc > b` (SEXT), `sz > usize_bits` (8) etc. are unsigned/positive-int4 compares matching the -Wno-sign-compare C++; no int4-vs-size loop hazard (loops use captured num_input/loc-vecs).
- Integer widths: clean. uintb->u64, int4->i32; `16*size-1` (POPCOUNT) computed in u64 vs C++ int4 — identical for valid small sizes.
- Wrapping: clean. SUBPIECE `wrapping_shl((sz*8))` guarded sz<8 (==plain shl); LEFT/RIGHT/PIECE shift amounts all guarded <64; verified no debug-panic on valid lifted IR. (Note: INSERT/ZPULL/SPULL `1<<getIn(k).off` would debug-panic for an offset>=64 where C++ is UB — unreachable for valid synthetic ops; F2 below.)
- Comparator totality: n/a (no new comparators; the BTree key orders are pre-existing).
- Iteration-order provenance: clean. spaces via `manage.get_space(i)` index order (== C++); `iter_loc`/`loc_space_ids` walk BTreeMap<LocKey> (== VarnodeLocSet std::set); `iter_alive`/`iter_code` walk intrusive insertion-order lists (== C++ list<PcodeOp*>); `gatherConsumedReturn` order-independent (OR). NO HashMap/HashSet in ported logic.
- Off-by-one/do-while/reverse: clean. opRemoveInput high->low keeping slot0 matches C++; condexe loop re-reads getSize() each step (the porter's fix; correct — hoisting over-indexes a shrinking graph).
- Erase-while-iterating: clean. deadcode sweep + neverConsumed + clearDeadVarnodes all collect-then-mutate, reproducing the C++ advance-before-delete visit set/order.
- Exception->Result partial-state: clean. dc_collapse Err -> opMarkNoCollapse (== C++ catch); no mid-mutation throw in the deadcode body.

## findings
- F1 (minor / accepted, LOSS-135): ActionDeadCode's four sub-seams — markConsumedParameters (numCalls==0 no-op), CPUI_INDIRECT indirect-source mark (getOpFromConst seam; outer backward push IS applied), CPUI_BRANCHIND switch-var mask (findJumpTable==None -> mask=~0), holdStackAliasStores (option default-off) — take the conservative C++-default value, not a recovered one. Only the skipped INDIRECT indirect-source mark is a behavioral divergence (a potential over-prune of one held op), unreachable on the B4 fixture. Appended as LOSS-135.
       cpp: decompiler/cpp/coreaction.cc:3808-3821,4205-4213,4233-4235,4104-4144
       rust: rust/crates/kuna-decomp/src/coreaction_render.rs (deadcode_apply / dc_propagate_consumed CPUI_INDIRECT arm)
- F2 (low, latent): INSERT/ZPULL/SPULL/INT_LEFT-large `1u64 << off` / shift-by-`off` panics in Rust debug if a constant offset >= 64, where C++ `1 << off` is UB (in practice wraps/0). Not reachable for well-formed lifted IR (these offsets are small bit positions); matches C++ for all valid inputs. No change required; flagged for the record.
       cpp: decompiler/cpp/coreaction.cc:3917-3933
       rust: rust/crates/kuna-decomp/src/coreaction_render.rs (CPUI_INSERT / CPUI_ZPULL arms)
- F3 (nit): the new deadcode `as i32`/`as u64`/`as u32` casts lack the `// cast:` justification comment the protocol requests. Each is provably in-range (traced above); consistent with the crate's existing inconsistent cast-comment usage. Not a blocker.
       rust: rust/crates/kuna-decomp/src/coreaction_render.rs (dc_propagate_consumed shift/offset casts)

## adversarial tests (added, land with the verdict, all green)
In rust/crates/kuna-decomp/src/coreaction_render/tests.rs (white-box, reach the private dc_* fns):
- `dc_subpiece_consume_shift_matches_cpp` — SUBPIECE backward consume `a = outc<<(sz*8)` (the wrapping-shift port), `b = ~0` for the truncation const. Pins the shift does not panic and equals the plain `<<` for in-range sz.
- `dc_int_and_const_intersects_mask` — INT_AND(var,const) pushes `outc&const` to the variable input, `outc` to the const; pins the const path + mask intersection with no sign contamination.
- `dc_push_consumed_accumulates_clamps_and_gates_worklist` — pushConsumed OR-accumulate + `&calc_mask(size)` clamp + the free-vs-written worklist gate + the vacuous early-return no-double-enqueue. (This test surfaced and confirmed the C++ `consumed = ~0` Varnode-init invariant: a faithful port detail, not a bug.)

## losses
- LOSS-070 (pre-existing): RuleCollapseConstants `op->collapse`/getConstant/collapseConstantSymbol/opMarkNoCollapse — this wave CLOSES the collapse + getConstant + opMarkNoCollapse half (markedInput/collapseConstantSymbol stays the W4 symbol seam, faithful no-op with no symbols).
- LOSS-132 (pre-existing keystone): single-manager already closed upstream of this branch (B3 oracle-exact); this wave builds on it.
- LOSS-135 (appended): the four ActionDeadCode sub-seams (F1).
