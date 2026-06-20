# w10-condexe-structure

verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-14
branch: rport/w10-condexe-structure (HEAD 2ec86b6; verifier tests a69ec93)

## Scope (as built, not as framed)

The verifier task framing named "condexe hoist + the &&/|| CollapseStructure
condition-merge + emitBlockCondition". The actual diff
(`git diff rust-port...rport/w10-condexe-structure -- rust/`) touches ONLY three
files and contains NEITHER a condexe hoist NOR a CollapseStructure condition-merge
change:

- `rust/crates/kuna-decomp/src/printc.rs` — adds `PrintC::emit_block_condition`
  and routes `BlockType::Condition` to it (previously fell through to the flat
  component-block default arm).
- `rust/crates/kuna-decomp/src/block.rs` — adds `FlowBlock::get_condition_opcode`
  (`BlockCondition::getOpcode`).
- `rust/crates/kuna-decomp/src/funcdata_block.rs` — closes the `spliceBlockBasic`
  trailing-branch-destruction seam: `return Err(SEAM W3-op)` -> `self.op_destroy(jumpop)`
  (the C++ `funcdata_block.cc:942 opDestroy(jumpop)`); plus a test-helper
  `mark_alive` and an updated test.

`condexe.cc` (ConditionalExecution) and `blockaction.cc` (CollapseStructure
`ruleBlockOr`, the genuine &&/|| merge) were ported in EARLIER waves and are
unchanged by this diff. This review verifies what is actually here: the
`emitBlockCondition` render + the `spliceBlockBasic` seam closure.

## gate

- `cargo test --workspace` (in /tmp/kuna-cxs) -> ALL GREEN (145 ok result groups,
  0 failed), including the 5 new verifier adversarial tests.
- `cargo clippy -p kuna-decomp -- -D warnings` (the documented CI gate, non-test)
  -> clean.
  (`cargo clippy -p kuna-decomp --tests -- -D warnings` has 19 errors, ALL in
  pre-existing files — double.rs, heritage.rs, kuna_regionid.rs, prefersplit/tests.rs,
  printc/tests.rs, universalaction/tests.rs; 52 such errors on the base rust-port
  tree, i.e. pre-existing, none in the diff or in the new test file.)
- C++ oracle parity (`python -m kuna.run_tests --all --baseline docs/baseline.json`)
  -> unittests 207/207, datatests 675/675, PARITY OK — UNTOUCHED (diff is Rust-only;
  no decompiler/cpp, specs/, or docs/baseline.json change).
- printc_parity byte-match: base == branch (both 8/9 decompiled, 0/8 byte-match,
  same 1 failing fixture `gp`) — NO parity or execution-failure regression.

## Faithfulness (by inspection — see "central finding" for why execution can't confirm)

`emit_block_condition` vs C++ `PrintC::emitBlockCondition` (printc.cc:2985-3019):
- `no_branch` arm: `beginBlock(getBlock(0))` / emit block0 / `endBlock` / return —
  matches (Rust `begin_block(0)`; the `0` blockref is the established port-wide
  convention for every `emit_block_*`, only used by markup emitters, irrelevant to
  the plaintext gate).
- `only_branch || comma_separate` arm: `openParen(id)` / emit block0 / `pushMod` /
  `unsetMod(only_branch)` / `setMod(comma_separate)` / build ReversePolish
  {op=null, visited=1, tok=&boolean_and|or} / `emitOp` / `openParen(id2)` /
  emit block1 / `closeParen(id2)` / `popMod` / `closeParen(id)` — the Rust
  reproduces this exact sequence and mod ordering.
- opcode->operator: C++ `if getOpcode()==CPUI_BOOL_AND {boolean_and} else {boolean_or}`;
  Rust `if opc==CPUI_BOOL_AND {BOOLEAN_AND} else {BOOLEAN_OR}` with
  `get_condition_opcode().unwrap_or(CPUI_BOOL_AND)` (the unwrap_or default is
  unreachable on a real Condition node, which always carries an opcode).
- The token constants match the C++ exactly: BOOLEAN_AND `("&&","",2,22,...)`,
  BOOLEAN_OR `("||","",2,18,...)` == printc.cc:54/56. `emit_op` Binary arm
  (visited==1 -> spaces/tagOp(print1)/spaces) == printlanguage.cc:336-341.
- The `emit_op` id2-persist tail (printc.rs:1444) is a no-op for Binary tokens
  (only Postsurround/Presurround mutate id2), so the local non-pushed `pol` is safe.

`spliceBlockBasic` op_destroy seam vs C++ funcdata_block.cc:927-961:
- `op_destroy` (funcdata_op.rs:1119, ported earlier) faithfully mirrors
  `opDestroy` (funcdata_op.cc:203): for a BRANCH op (no output) it takes the
  parent-removal arm `mark_dead` + `removeOp` == C++ `markDead` + `removeOp`.
  (`op_unset_output` substitutes for the unported `destroyVarnode` — a pre-existing
  W3-varnode seam, irrelevant for a no-output BRANCH.)
- The test-helper `mark_alive` addition is CORRECT: `create_at` starts an op dead;
  `op_destroy -> mark_dead` does `alivelist.erase`, which requires the op to be on
  the alive list (as block ops are in the real pipeline). Test-only fixture fix.

## hunt list

- Signedness: clean — no signed/unsigned comparison in the diff (the only `==` is
  `opc == CPUI_BOOL_AND`, an enum compare).
- Integer widths: clean — no integer arithmetic introduced.
- Wrapping: clean — no arithmetic.
- Comparator totality: n/a — no comparator.
- Iteration-order provenance: clean — `emit_block_condition` indexes `get_block(0/1)`
  (fixed); `spliceBlockBasic`'s op-move loop is unchanged by this diff (only the
  branch-destroy line changed).
- Off-by-one / do-while / reverse iterators: clean — no loops added in production.
- Erase-while-iterating: the `op_destroy` now runs BEFORE the outbl op-move loop
  (matching C++ line 942 before 952); it removes only the tail BRANCH of `bl`,
  not an element being iterated. Clean.
- Exception -> Result partial-state parity: the seam previously RETURNED Err mid-method
  (before any mutation); now it `op_destroy`s and continues, exactly as C++ falls
  through. The remaining Err arm (MULTIEQUAL head) is unchanged and matches
  C++ `throw LowlevelError("Splicing block with MULTIEQUAL")`. Clean.

## Mechanical pass

- diff grep: no `todo!`, `unimplemented!`, `HashMap`, `HashSet`, `sort_unstable`,
  no bare `as` casts in the production hunks.
- NO special-casing: no function-name or address hardcode anywhere in the diff
  (the only literals are synthetic test addresses 0x1000-0x100c and the
  `CPUI_BOOL_AND` opcode dispatch). REJECT condition not met.
- NO mis-merge: boolless's single `if (dat_52 <= 10)` is NOT structured into a
  BlockCondition and renders as a plain `if` — `print_b5_boolless.rs`'s
  `w10_boolless_full_byte_parity_modulo_type_inference` (exact byte-equality to
  the C++ B5 oracle) PASSES on the branch, and verifier AT3 confirms
  `count_condition_blocks(boolless)==0`. boolless and readstruct
  (`verify_w10_struct_corpus.rs`, 16 tests) remain byte-/structure-identical.

## CENTRAL FINDING — emit_block_condition is unreached dead code (LOSS-159)

A corpus-wide scan (committed `#[ignore]`d probe
`probe_scan_corpus_for_any_condition_node`) decompiled all 167 decompilable
datatest functions through the Rust pipeline and found ZERO `BlockType::Condition`
nodes. The Rust engine does not currently structure any corpus function into a
`&&`/`||` BlockCondition (e.g. ccmp/orcompare, which the C++ oracle renders as
`if (ptr[1] == 0x3c && val <= 9)` / `if (a == 10 || b == 0x14)`, are rendered by
the Rust engine as un-collapsed boolean arithmetic `v3 = v2 | ZEXT(..)` — a
PRE-EXISTING analysis-completeness gap, NOT this diff). The `||` that does appear
in renders (nan's `NAN(..) || NAN(..)`) comes from the DATA-FLOW expression
printer (CPUI_BOOL_OR PcodeOp -> BOOLEAN_OR token), not the control-flow
`emit_block_condition`. So the new render is a faithful-by-inspection transcription
that cannot be confirmed by execution and is byte-neutral on every gate today.
This matches the established W10-unseam pattern (LOSS-156/157/158: faithful ports
held dormant in the degraded pipeline). Accepted as LOSS-159.

## findings

- F1 (LOSS, accepted): `emit_block_condition` is unreached dead code in the
  current Rust pipeline (no corpus function builds a Condition node). Faithful by
  inspection; unverifiable by execution; byte-neutral. -> LOSS-159.
     cpp: decompiler/cpp/printc.cc:2985 (emitBlockCondition)
     rust: rust/crates/kuna-decomp/src/printc.rs (emit_block_condition + the
           BlockType::Condition dispatch arm)
- F2 (informational, no action): the `spliceBlockBasic` seam closure is correct
  and exercised by the updated unit test `splice_block_basic_destroys_trailing_branch`;
  `op_destroy`'s `destroyVarnode`->`op_unset_output` substitution is a pre-existing
  W3-varnode seam (already in losses), unchanged here.
     cpp: decompiler/cpp/funcdata_block.cc:942
     rust: rust/crates/kuna-decomp/src/funcdata_block.rs:433

## adversarial tests (committed a69ec93, rust/crates/kuna-decomp/tests/verify_w10_condexe_structure_adversarial.rs)

- at1_emitblockcondition_is_currently_unreached_dead_code — the central finding,
  a tripwire that flips when structuring lands.
- at2_any_condition_opcode_is_a_boolean_glue — BOOL_AND/BOOL_OR invariant (load-
  bearing once Conditions appear).
- at3_single_condition_stays_plain_if_no_merge — boolless NOT mis-merged.
- at4_condition_render_is_engine_driven_per_function — cross-function token purity.
- at_obs_ccmp_and_structuring_is_a_known_gap_not_this_diff — pins the pre-existing
  ccmp `&&`-structuring gap + records the C++ oracle target.
- probe_scan_corpus_for_any_condition_node (#[ignore]) — 167 funcs -> 0 Condition
  nodes (the evidence behind at1).

## losses

LOSS-159 (this review).
