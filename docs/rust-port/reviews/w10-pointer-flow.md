# w10-pointer-flow
verdict: ACCEPT
verifier: verifier-agent (Opus 4.8 1M) — Round 2
date: 2026-06-14
gate: `cd /tmp/kuna-ptr/rust && cargo test --workspace` -> **GREEN** (143 `test result: ok` binaries, 0 failed across the workspace; `verify_w10_struct_corpus` 16/16 incl. all three adversarial `w10_ptr_flow_*` tests; `print_b5_boolless` 11 pass incl. `w10_boolless_full_byte_parity_modulo_type_inference`). `cargo clippy -p kuna-decomp -- -D warnings` -> **clean** (exit 0); the branch's own test file `verify_w10_struct_corpus` is clippy-clean under `-D warnings` too (the test-file warnings seen are pre-existing in sibling W3/W5 test files, not this branch). `cargo build --workspace` -> clean. Branch touches ONLY `rust/` (`git diff --name-only rust-port...HEAD | grep -v ^rust/` = empty) so `decompiler/`/`specs/`/`baseline.json` are untouched; the C++ oracle is intact — re-ran `python -m kuna.run_tests --all --baseline docs/baseline.json` in the MAIN tree -> **207/207 unit + 675/675 datatest assertions, PARITY OK**.

## Round-2 disposition (the two REJECT-relevant Round-1 findings are FIXED)

**F1 (the REJECT trigger — store operand inversion): FIXED and verified.**
The repair (`12572ce`) swaps the two pushes in `op_store_ir` (printc.rs:2615-2624)
so the pointer is pushed FIRST (LHS) and the value SECOND (RHS), matching the
direct-recursion engine's push-order contract (`op_binary_ir` pushes in0-then-in1;
`emit_expression_ir` pushes the assignment output first) — the inverse of the C++
LIFO `nodepend` order, exactly as C++ `opStore` (printc.cc:520-538) pushes
in2(value) then in1(pointer) so the LIFO reversal makes the pointer the LHS.
Independently re-rendered divopt via the Rust engine: **64** `a0[N] = <expr>;`
array-LHS stores, **0** inverted `<expr> = a0[N];` stores, **0** surviving
functional `STORE(`, **0** surviving functional `LOAD(`, **0** raw `RDI` leaks.
This matches the C++ oracle direction for divopt (`divopt.xml` stringmatches:
`*divu = *divu / 81;`, `divu[1] = divu[1] / 89;`, … — array/pointer access on the
LHS of `=`). The member/array access is REAL: produced by genuine PTRADD/PTRSUB
from real pointer-type propagation (`*a0`, `a0[1]`, `a0[0xb]` all appear, driven
by `RulePtrArith`->`AddTreeState`), not a print-time special case.

**F2 (uint4 width): FIXED and verified.**
`biggest_non_mult_coeff` is now declared `uint4` (addtreestate.rs:67) and every
store narrows to the low 32 bits: `vncoeff = (sval<0) ? sval.wrapping_neg() as
uint4 : sval as uint4` (matching C++ `(uint4)-sval`/`(uint4)sval`,
ruleaction.cc:6169) and `if tree_coeff > biggest_non_mult_coeff as uintb {
biggest_non_mult_coeff = tree_coeff as uint4 }` (matching the C++
uint8->uint4 narrowing store, ruleaction.cc:6182-6183/6234-6235). The
`has_matching_sub_type` `array_hint` param is now `uint4` too, and the
`elSizeBefore == arrayHint` comparison correctly zero-extends the uint4 to int8
(`array_hint as int8`) exactly as the C++ promotion does. Faithful.

**F3 (int4 narrowing in opUndoPtradd multiply): remains, NON-BLOCKING (unchanged).**
`op_undo_ptradd` uses the untruncated `mult_off` (uintb) in the multiply
(funcdata_op.rs:498/516) where C++ uses the int4-narrowed `multSize`
(funcdata_op.cc:583/590); it correctly uses the int4-narrowed `mult_size` for the
`==1` guard. They differ only when the PTRADD element-size constant exceeds int4
range — never for a legitimate datatype byte size; faithful on the corpus.
Flagged for completeness, not a loss.

## Round-2 hunt list (re-verified independently)
- **Signedness**: clean. `slot`/`1-slot` and `for s in 0..num_input` (int4) in
  `RulePtrArith::apply_op` are bounded to {0,1}/numInput as in C++; the `el_size
  == array_hint as int8` comparison zero-extends a non-negative uint4 (faithful to
  the C++ uint4->int8 promotion); the `sval<0` sign tests match C++.
- **Integer widths**: F2 now matched (uint4). The `as int4`/`as uintb` casts on
  align-size/byte-size lineages are faithful to C++ implicit conversions.
- **Wrapping**: clean. `wrapping_mul`/`wrapping_add`/`wrapping_sub`/`wrapping_neg`
  on every uintb-lineage op; `sval.wrapping_neg()` faithful to C++ `-sval`.
- **Comparator totality**: n/a (no comparator added).
- **Iteration-order provenance**: clean. **No HashMap/HashSet anywhere in the
  diff** (grep over the full `rust-port...HEAD` diff: NONE). `calc_nz_mask` uses a
  `Vec` LIFO `opstack`/`worklist` (`.last()`/`.pop()`/`.push()`) matching C++
  `vector::back()/pop_back()`; the alive-op iteration is `iter_alive()` (list
  order); descend reads are `descend_iter()` matching `beginDescend()..endDescend()`.
- **Off-by-one / do-while**: clean. `calc_nz_mask`'s C++ `do {…} while(!opstack.
  empty())` is reproduced by `while let Some(&(op,slot)) = opstack.last()` —
  the root is pushed before the loop so the body runs at-least-once, identical to
  the do-while (which can only differ on an empty-on-entry stack, never the case).
  The slot advance (`oldslot = slot; top.1 += 1`) and the MULTIEQUAL loop-edge clip
  (`is_loop_in(oldslot)`) are faithful.
- **Erase-while-iterating**: clean. The descend-list re-reads in `RulePushPtr`/
  `evaluate_pointer_expression` collect-then-iterate; no mutate-during-traversal.
- **Exception->Result partial-state**: faithful. `get_sub_type`/`find_truncation`
  `.ok().flatten()` decline edges return the C++ null edges; the `op_ptrsub_ir`
  out-of-bounds / non-pointer arms fall to `op_func_ir` (functional render) instead
  of the C++ `throw` (output stays parseable) — a render-robustness choice, the
  corpus never reaches it.
- **Bare `as` casts**: 37 in addtreestate.rs, 3 with `// cast:` (the F2 fix sites).
  Each sampled bare cast is faithful to a C++ implicit conversion; the missing
  `// cast:` comments are a LOW/non-blocking protocol nit (same disposition as the
  sibling type-prop/printc reviews), not an unfaithfulness.
- **No special-casing**: clean. Grepped the full source diff for function-name /
  address / type-name / struct-field hardcoding: NONE. The only `field_0x{…}`
  literal is the faithful C++ default-field-name format (`s << "field_0x" << hex
  << suboff`, printc.cc). `a0`/`divu` names come from genuine parameter/symbol
  binding (sibling W10 items), not this branch.

## Round-2 re-verification of the un-stubbed supporting rules
- `RulePtraddUndo::apply_op` (ruleaction_6.rs): now a faithful transcription of
  C++ `RulePtraddUndo::applyOp` (ruleaction.cc) — same `getPtrTo()->getAlignSize()
  == addressToByteInt(size, wordSize)` size guard, same `(!isConstant)||(offset!=0)`
  index guard, then `opUndoPtradd(op,false); return 1`. The W6 `seam_op_undo_ptradd`
  Err-stub is removed.
- `set_opcode`/`set_opcode_seam` (ruleaction_1.rs/_4.rs): now resolve through the
  canonical `typeop::type_op_for(opc)` table (general, all opcodes) rather than the
  flag-less `TypeOp::new(opc,0,..)` stub, so created ops carry their real eval-type
  bit and `RuleCollapseConstants` can fold `SUB(mask,0)`. General, not special-cased.
- `op_ptrsub_ir` / `op_ptradd_ir` / `op_load_ir`: faithful transcriptions of C++
  `opPtrsub`/`opPtradd`/`opLoad` (printc.cc:900-1160/507-518), including the
  `&( ).name` / `( )->name` / `( )[0]` / `*( )` member/array/deref table and the
  push-order inversion. The SPACEBASE (W4) and UNION (W8) arms fall through to a
  functional render (off-corpus seams), correctly.

---

# w10-pointer-flow (Round 1 — superseded by Round 2 above)
verdict: REJECT
verifier: verifier-agent (Opus 4.8 1M) — Round 1
date: 2026-06-14
gate: `cd /tmp/kuna-ptr/rust && cargo test --workspace` -> **RED** (`verify_w10_struct_corpus` binary: 11 pass / 2 fail before the verifier's tests; with the verifier's adversarial `w10_ptr_flow_*` tests, 11/3). Two pre-failing tests + one new failing adversarial test all trace to **one** branch-introduced bug (F1, the `op_store_ir` operand inversion). `cargo clippy -p kuna-decomp -- -D warnings` -> **clean**. `cargo build --workspace` -> clean. Branch touches only `rust/` (15 files; `decompiler/`/`specs/`/`baseline.json` untouched — verified by `git diff --name-only rust-port...HEAD | grep -v ^rust/` = empty), so the C++ oracle 675/675 PARITY OK is preserved by construction. boolless byte-parity: **preserved** (`print_b5_boolless` 11/11, incl. `w10_boolless_full_byte_parity_modulo_type_inference`).

## Summary
The pointer-flow port is, in the analysis half, an unusually faithful
statement-for-statement transcription of the C++ (`AddTreeState`, `RulePtrArith`,
`RuleStructOffset0`, `RulePushPtr`, `RulePtraddUndo`, `Funcdata::opUndoPtradd`/
`collapseIntMultMult`/`newOpBefore`/`calcNZMask`). The PTRADD/PTRSUB creation and
the mask folding are real and correct; the member/array access is REAL (genuine
PTRADD/PTRSUB from `RulePtrArith` driven by real pointer-type propagation, NOT a
print-time special case — two of the verifier's three adversarial tests prove the
LOAD side renders `a0[N]` / `*(a0 + ..)` and that no functional `LOAD(..,a0)`
survives). The branch is a large net improvement: the corpus goes 0/13 -> 11/13.

But the **print half has a faithfulness bug that fails the gate**: `op_store_ir`
renders the STORE assignment with its operands **inverted** — `<value> = a0[N];`
instead of the C++ oracle's `a0[N] = <value>;`. This is a REJECT trigger
(unfaithful render that regresses against the oracle on a real corpus function),
independent of how much else is correct.

## F1 (REJECT trigger): `op_store_ir` inverts the assignment operands
The PrintC direct-recursion RPN engine renders in **push order** (first push =
leftmost operand), which is the **inverse** of the C++ LIFO `nodepend` push order.
The branch's own `op_binary_ir` and `emit_expression_ir` both honor this by
inverting (push in0-then-in1 where C++ pushes in1-then-in0; push the assignment
output/LHS first). `op_store_ir` does **not** invert: it pushes `val` (slot 2)
first, then `ptr` (slot 1) — copying the C++ source push order literally — so the
pointer/array access lands on the RIGHT of `=`.

- cpp: decompiler/cpp/printc.cc:520-538 (`opStore`: `pushVn(in2/value)` then
  `pushVn(in1/pointer)`; the LIFO reversal makes the **pointer** the LHS so the
  store reads `pointer = value`).
- rust: rust/crates/kuna-decomp/src/printc.rs:1381-1384 (pushes `val` then `ptr`
  in the SAME order — under the push-order engine this renders `value = pointer`).
- evidence: `verify_w10_r2_divopt_stores_through_rdi_straightline`
  (`verify_w10_struct_corpus.rs:545`) — oracle wants straight-line `p[N] = …`
  (decomp_test_dbg scores divopt 34/34); the branch emits every store reversed,
  e.g. `SUB(ZEXT(a0[1]) * ZEXT(0x70..),8) + (…) >> 6 = a0[1];`. The verifier's
  `w10_ptr_flow_store_renders_pointer_on_lhs_of_assignment` finds **105** inverted
  stores in the divopt corpus and **0** correct `a0[N] = …` forms.
- fix: swap the two pushes (and their mods) so the pointer goes first:
  `push_vn_ir_m(ptr, m)` then `push_vn_ir_m(val, mods)`. (opLoad is single-operand
  and is correct; opPtradd/opPtrsub already invert correctly.)

## F2 (medium, width): `biggest_non_mult_coeff` is `uintb`, C++ is `uint4`
C++ `AddTreeState::biggestNonMultCoeff` is **`uint4`** (ruleaction.hh:54). Every
assignment to it in C++ **truncates to 32 bits** before storing:
`biggestNonMultCoeff = (uint4)sval` (ruleaction.cc:6169) and
`biggestNonMultCoeff = treeCoeff` (a `uint8 -> uint4` narrowing, 6182-6183 /
6234-6235). The Rust field is `uintb` and stores the **full 64-bit** value at all
three sites (no truncation). Downstream this value is read as `arrayHint != 0` /
`biggestNonMultCoeff != 0` and as the array element-size hint in
`hasMatchingSubType` (`elSizeBefore == arrayHint`). For any non-multiple
coefficient whose magnitude exceeds `0xFFFFFFFF`, C++ keeps only its low 32 bits
(possibly collapsing to 0) while Rust keeps the whole value — diverging both the
STRUCT negative-offset arm of `calcSubtype` (`biggestNonMultCoeff != 0 &&
multsum == 0`) and the array-element-size match. divopt is full of 64-bit
reciprocal-division constants, so this is not obviously unreachable; the porter
neither matched the width nor documented the divergence.

- cpp: decompiler/cpp/ruleaction.hh:54 (`uint4 biggestNonMultCoeff;`);
  ruleaction.cc:6169, 6182-6183, 6234-6235 (the `(uint4)` / uint8->uint4 stores).
- rust: rust/crates/kuna-decomp/src/addtreestate.rs:67 (`biggest_non_mult_coeff:
  uintb`), :307-309, :321-322, :372-373 (full-width stores, no `as u32`).
- fix: declare the field `uint4` (and the `array_hint` param of
  `has_matching_sub_type`), or truncate at each store (`(… ) as u32 as uintb`),
  matching the C++ low-32-bits semantics; add a coefficient-> 2^32 test.

## F3 (low, width — non-blocking): `op_undo_ptradd` multiply uses untruncated offset
C++ `opUndoPtradd` computes `newVal = multSize * offVn->getOffset()` where
`multSize` is `int4` (narrowed from `multVn->getOffset()` at funcdata_op.cc:583).
The Rust uses the **untruncated** `mult_off` (uintb) in the multiply
(funcdata_op.rs:995: `mult_off.wrapping_mul(off_off)`), although it correctly uses
the int4-narrowed `mult_size` for the `== 1` guard. They differ only when the
PTRADD element-size constant exceeds int4 range — never for a legitimate datatype
byte size. Faithful on the corpus; flagged for completeness.
- cpp: decompiler/cpp/funcdata_op.cc:583,590; rust: funcdata_op.rs:979,995.

## Non-finding: `verify_w10_symbol_naming_readstruct_body_uses_param_names_not_registers`
This second pre-failing corpus test is **obsoleted by correct behavior**, not a
defect of this branch. Its regex requires `LOAD([0-9]+,a0` (the OLD functional
LOAD rendering). This branch correctly now renders that LOAD as `*(a0 + (a1 + 2 +
a2) * 4)` (a non-PTRADD/PTRSUB address -> explicit deref, faithful to C++ opLoad).
The body still binds and uses `a0`. This is a stale assertion belonging to the
sibling `w10-symbol-naming` item that the pointer-flow improvement supersedes; it
is NOT counted as an F-finding here (the rendering is the C++-faithful one), but it
does contribute to the RED gate and must be reconciled when this item is re-landed.

## NO SPECIAL-CASING
**CLEAN.** A literal-extraction pass over the whole diff finds zero
function-name / address / type-name / struct-field hardcoding. Every string in the
added code is a `.expect()` panic message, a C++-comment quote, or the
**computed** default-field name `format!("field_0x{suboff_bytes:x}")` (faithful to
C++ printc's `field_0x%x` for unnamed struct offsets — derived from the offset, not
a hardcoded field). No `0x`-address compares, no `get_name()==` gates. The
member/array access comes entirely from genuine pointer-type propagation +
PTRADD/PTRSUB creation matching the C++; the verifier's two passing LOAD tests
confirm it is real, not fabricated for a specific function's `ptr->array[i]`.

## Member/array access is REAL / datatest parity
The LOAD-side access IS real: `w10_ptr_flow_load_renders_array_member_access_not_functional_load`
and `w10_ptr_flow_load_explicit_deref_keeps_base_inside_star` both pass — `a0[N]`
comes from a genuine PTRADD/PTRSUB built by `RulePtrArith`/`RuleStructOffset0` over
the real `AddTreeState` analysis, and no functional `LOAD(..,a0)` survives. No NEW
positive datatest pass is claimed by this branch (the gate is the corpus-render
binary, not the datatest count); the boolless byte-parity test still passes, so the
`set_opcode_seam -> type_op_for` change (the real `opflags`/eval-type table in
double.rs / prefersplit.rs / ruleaction_1/4) did not regress boolless. Execution
failures: the corpus binary goes 0/13 -> 11/13 (the pipeline now reaches rendering
on functions that previously panicked), so the exec-failure count **improves**; the
2 (now 3-with-verifier) remaining failures are render-correctness, not execution.

## hunt list
- **Signedness**: clean. `slot`/`preslot`/`dec_slot` int4 vs container sizes;
  `1 - slot`/`1 - dec_slot` with slot in {0,1}; `offsetbytes < 0 || >= getSize()`
  comparisons reproduce the C++ int8-vs-int4 (sign-extended) compares.
- **Integer widths**: **F2** (`biggest_non_mult_coeff` uintb vs C++ uint4 — real),
  **F3** (`op_undo_ptradd` multiply untruncated — low). `size`/`ptrsize`/`movesize`
  int4/int8 mappings otherwise faithful; `sign_extend(val as i64, sz*8-1)` matches
  the C++ `sign_extend(uintb, ...)` family (same usage the sibling type-prop review
  validated).
- **Wrapping**: clean. `wrapping_mul`/`wrapping_add`/`wrapping_sub`/`wrapping_neg`
  on every `uintb`-lineage op (multsum/nonmultsum/offset/correct/val), matching C++
  silent unsigned wrap; `uintb_negate(correct-1, ptrsize)` faithful.
- **Comparator totality**: n/a (no comparator added).
- **Iteration-order provenance**: clean. **No HashMap/HashSet** in the diff (grep
  NONE). `multiple`/`coeff`/`nonmult` are `Vec` in insertion order = C++ `vector`;
  the `calc_nz_mask` DFS/worklist is a deterministic graph walk with a LIFO worklist
  matching C++ `worklist.back()/pop_back()`; the `descend_iter()` reads match the
  C++ `beginDescend()..endDescend()` list order.
- **Off-by-one / do-while / reverse iterators**: clean. `calc_nz_mask`'s C++
  `do…while(!opstack.empty())` is reproduced by `while let Some = opstack.last()`
  (stack non-empty on entry); `RulePushPtr::duplicateNeed`/`apply_op` re-read the
  front of a body-rewired descend list each pass — the Rust `loop { … next() else
  break }` visits the same elements (the C++ do-while's at-least-once is only
  reachable with a non-empty descend list, which both forms require).
- **Erase-while-iterating**: clean. `op_destroy`/`op_set_input` rewrites happen
  after the per-iteration `descend_iter().next()` re-read, matching the C++
  re-`beginDescend()` idiom; no collect-then-mutate hazard.
- **Exception->Result partial-state**: faithful. The `get_sub_type`/`find_truncation`
  `.ok().flatten()` decline points return the C++ null edges; the `valid=false`
  early returns leave the same partial `AddTreeState` the C++ leaves.
- **Bare `as` casts**: many in `addtreestate.rs` lack `// cast:` (protocol nit);
  each is faithful to a C++ implicit conversion. Low/non-blocking (same disposition
  as the sibling type-prop review's F2).

## findings
- F1 (HIGH, REJECT trigger): `op_store_ir` renders the STORE assignment inverted
  (`value = ptr[N]` instead of `ptr[N] = value`) — the direct-recursion engine
  renders in push order but opStore pushes value-then-pointer (C++ source order)
  instead of inverting like every other binary op.
       cpp: decompiler/cpp/printc.cc:520-538
       rust: rust/crates/kuna-decomp/src/printc.rs:1381-1384
- F2 (MEDIUM, width): `biggest_non_mult_coeff` typed `uintb` and stored full-width
  where C++ `biggestNonMultCoeff` is `uint4` and every store truncates to 32 bits.
       cpp: decompiler/cpp/ruleaction.hh:54; ruleaction.cc:6169,6182-6183,6234-6235
       rust: rust/crates/kuna-decomp/src/addtreestate.rs:67,307-309,321-322,372-373
- F3 (LOW, width — non-blocking): `op_undo_ptradd` multiply uses the untruncated
  uintb offset where C++ uses the int4-narrowed `multSize`.
       cpp: decompiler/cpp/funcdata_op.cc:583,590
       rust: rust/crates/kuna-decomp/src/funcdata_op.rs:979,995
- OBS (non-finding): `verify_w10_symbol_naming_readstruct_body...` is obsoleted by
  this branch's correct `*(a0 + ..)` LOAD rendering (its regex wants the old
  functional `LOAD(..,a0)`); reconcile the sibling-item test on re-land.

## adversarial tests
Committed to the worktree (`/tmp/kuna-ptr`, commit `e0a0f21`,
`rust/crates/kuna-decomp/tests/verify_w10_struct_corpus.rs`, prefix `w10_ptr_flow_`):
- `w10_ptr_flow_store_renders_pointer_on_lhs_of_assignment` — **FAILS** on the
  branch (the F1 divergence trace): 105 inverted `… = a0[N];` stores, 0 correct
  `a0[N] = …`. This is the REJECT's required failing test.
- `w10_ptr_flow_load_renders_array_member_access_not_functional_load` — passes:
  the LOAD member/array access is REAL (`a0[N]`, no functional `LOAD(..,a0[`).
- `w10_ptr_flow_load_explicit_deref_keeps_base_inside_star` — passes: a non-PTRADD
  LOAD renders `*(a0 + ..)` / `a0[..]` with the base inside, never functional.

losses: none. F1 is a fixable unfaithfulness (a 2-line operand swap), not an
accepted divergence; F2/F3 are width-fidelity findings, not accepted losses. No
`losses.md` entry is appended (REJECT -> item returns to `todo` with these findings
as the work order; the first fix is F1, then F2, then reconcile the obsoleted
sibling test).
