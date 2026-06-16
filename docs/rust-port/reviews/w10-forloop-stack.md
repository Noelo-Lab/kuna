# w10-forloop-stack
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8)
date: 2026-06-16
gate: branch `rport/w10-forloop-stack` (HEAD 0c45569 + verifier-test commit 0bf68b7)
  - rust `decomp_test_dbg` full corpus (branch worktree) -> 675 applied / 293 passing
  - rust `decomp_test_dbg` full corpus (merge-base 9f86ea0) -> 675 applied / 293 passing
  - passing-SET diff base->branch: `comm -23` EMPTY (0 regressions), `comm -13` EMPTY (0 gains) -> BYTE-IDENTICAL
  - boolless (2) / struct (14) / condconst (37) datatest assertions: byte-identical base==branch
  - `cargo test --workspace` -> green (0 failed); `cargo test -p kuna-decomp --lib` -> 1712 passed / 0 failed (incl. 3 new verifier tests)
  - `cargo clippy -p kuna-decomp -p kuna-console --lib` -> clean
  - C++ oracle `decomp_test_dbg` datatests -> 675/675 PARITY OK (specs/cpp/datatests/baseline byte-untouched; Rust-only diff)

## Summary

The change ports the upstream whiledo->for reroll end to end: `BlockWhileDo::finalTransform`
(block.cc:3405) + `finalizePrinting` (block.cc:3454) and their helpers
(`findLoopVariable`/`findInitializer`/`testTerminal`/`testIterateForm`), the op-move
predicates `PcodeOp::isMoveable` (op.cc:201) and `Funcdata::moveRespectingCover`
(funcdata_op.cc:1498) with `HighVariable::markExpression` (variable.cc:885), and
`PrintC::emitForLoop` (printc.cc:3106). It wires these into the existing
`ActionStructureTransform`/`ActionFinalStructure` hooks (previously no-ops, declared
losses) and carries the `analyze_for_loops` gate through the seam.

It is a faithful, complete transcription. It is also INERT on the current corpus: it
adds **0 new datatest passes and 0 regressions** (the passing set is byte-identical
base==branch). No corpus datatest asserts a literal `for (`; on the one function C++
rerolls (`loopcomment`'s `forcomment` region) the rust IR has not yet had the upstream
stack-var/HighVariable-recovery passes, so `findLoopVariable`/`testTerminal` correctly
decline and the loop still renders as `while`. This is a faithful PARTIAL with zero
regression -> ACCEPT-WITH-LOSSES (LOSS-197).

## Faithfulness verdict (the prompt's four axes)

1. FAITHFUL transcription, not a one-function heuristic: every cited C++ method is
   ported statement-for-statement; verified line-by-line against block.cc/op.cc/
   funcdata_op.cc/variable.cc/printc.cc at the recorded blob. B0 / pipeline order
   UNCHANGED (only the two existing action `apply` bodies gained the hook calls).
2. NO SPECIAL-CASING: grep over every added executable line found ZERO
   function-name / address / offset / magic-number hardcoding (only opcode enums and
   the `for` keyword id). No HashMap/HashSet/sort_unstable/todo!/unimplemented!/bare-as.
3. New PASSES: there are NONE (feature dormant). Confirmed via live KUNA_DUMP that the
   C++ oracle emits `for (v5 = 0; ...; v5 = v5 + 1)` on `loopcomment` where the rust
   engine still emits `while` — an honest unported-prerequisite gap, NOT a fabricated
   pass and NOT a regression. The reroll correctly emits 0 FALSE for-loops (all
   `noforloop_*` guard assertions stay passing).
4. NO REGRESSION: passing SET byte-identical; boolless/struct/condconst byte-identical;
   whole-suite 293 (== the >=293 floor, flat); C++ oracle 675/675 untouched; no reserved
   file touched (the printc.rs edit is the loop-emit arm, NOT the pushPartialSymbol /
   member-access arms; variable.rs/funcdata_varnode.rs/unionresolve.rs/dtype.rs absent).

## Mandatory hunt list

- Signedness: CLEAN. All index arithmetic is `int4`(i32) vs `int4`: `num_input()->int4`,
  `get_in(int4)`, `get_out_rev_index(int4)->int4`, `ind >= num_in`, `count == 3`,
  `1 - slot` over {0,1}. Matches the `-Wno-sign-compare` C++ exactly.
- Integer widths: CLEAN. No int4*int4->intb promotions in this surface; no width games.
- Wrapping: CLEAN. No `uintb`-lineage arithmetic in the ported logic (pure
  block/op/varnode graph walks + flag tests + slot indices).
- Comparator totality: N/A. No `operator<`/sort ported here (StackEqn::compare /
  StackSolver are NOT in this diff — they pre-exist in coreaction_stackptr.rs, untouched).
- Iteration-order provenance: whiledo-node scan is `slotmap` dense-index order
  (deterministic, not hash-randomized). Per-whiledo transforms touch only their own
  node's fields + their own disjoint basic blocks, so the flat scan ~= the C++ tree
  recursion FOR DISJOINT LOOPS. Latent caveat: for NESTED whiledos the slotmap order is
  not guaranteed outer-first (the C++ recursion is); currently unobservable (dormant).
  No HashMap/HashSet in ported logic.
- Off-by-one / do-while / reverse: `findLoopVariable`'s `path[4]`+`count==3` bound is
  reproduced exactly (Vec cap 4, `count=len-1`, guard before push, read-then-increment
  slot). `isMoveable`'s C++ `do{++biter;...}while(biter!=point)` at-least-once forward
  walk is a faithful `loop{advance; body; if==point break}` — point's own output IS
  processed by the tied-overlap check, matching C++.
- Erase-while-iterating: N/A for traversal; the op moves use `op_uninsert` +
  `op_insert_after` (== C++ `opUninsert`/`opInsertAfter`), gated by `allow_op_moves` as
  in C++. No collect-then-remove / retain divergence.
- Exception->Result partial-state: the for-loop helpers don't throw; they return
  Option/bail and unwind the whiledo node fields (`set_iterate_op(None)` etc.) exactly
  where C++ sets `iterateOp = 0` / `initializeOp = 0`. Partial-state parity holds.

## Findings

- F1 (low / latent, NON-blocking): `struct_last_op` (block.rs, the `FlowBlock::lastOp`
  virtual) handles Basic/List/Condition/If but returns `None` for BlockCopy/BlockGoto/
  BlockMultiGoto, where C++ recurses (`copy->lastOp()` / `getBlock(0)->lastOp()`).
       cpp: decompiler/cpp/block.hh:546,576,604 (BlockCopy/BlockGoto/BlockMultiGoto lastOp)
       rust: rust/crates/kuna-decomp/src/block.rs:1355 (`struct_last_op`, the `_ => None` arm)
  Effect: a whiledo body/condition wrapped in a Copy/Goto would make the transform bail
  (a CONSERVATIVE false-negative -- never a false reroll). Unobservable now (feature
  dormant: passing set byte-identical). Cited in LOSS-197 restoration criterion.

## Adversarial tests (committed to the branch, commit 0bf68b7)

- `vfy_forloopstack_op_is_moveable_load_is_special_but_allowed` — LOAD movingLoad path
  is allowed (vs STORE rejected); the porter only tested STORE-not-moveable.
- `vfy_forloopstack_moving_load_blocked_by_intervening_store` — a moving LOAD is blocked
  by an intervening STORE (op.cc:257-259).
- `vfy_forloopstack_op_is_moveable_point_must_follow_this` — the Rust `None => return
  false` bail when the forward block walk never reaches an earlier point (replacing the
  C++ do-while's past-the-end UB); no panic, no UB.
(All 3 pass; they pin the most fragile transcribed predicate. The reroll's CFG-level
helpers `findLoopVariable`/`testTerminal` are not directly unit-testable without a full
HighVariable-bearing CFG; they are pinned end-to-end by the dormancy + zero-false-reroll
evidence and the byte-identical `noforloop_*` guard set.)

## Losses

- LOSS-197 (faithful PARTIAL): the reroll is INERT on the corpus (0 new passes,
  blocked by unported stack-var/HighVariable-recovery prerequisites, demonstrated by the
  C++-vs-rust `loopcomment` divergence) + the F1 `struct_last_op` Copy/Goto coverage gap.
  Zero observable regression. Restoration criterion recorded in losses.md.
