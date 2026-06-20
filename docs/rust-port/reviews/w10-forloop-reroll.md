# w10-forloop-reroll
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Opus 4.8 [1m])
date: 2026-06-16
gate: `cargo test --workspace` -> green (kuna-decomp lib 1718 passed / 0 failed incl. 3 new adversarial); `cargo clippy -p kuna-decomp --lib` -> clean; C++ oracle `run_tests --datatests` -> 675/675 PARITY OK (untouched); rust-engine datatests -> 298/675 (passing SET byte-identical to rust-port base); B0 `universalaction_listing` -> 1 passed (pass order unchanged)

## Summary

`rport/w10-forloop-reroll` (`c497cc3`) is a **faithful, complete transcription** of
Ghidra's BlockWhileDo while->for reroll (block.cc:3212-3475, op.cc:201,
funcdata_op.cc:1498, variable.cc:885, printc.cc:3106/3150) wired into the existing
`ActionStructureTransform`/`ActionFinalStructure` hooks. The port is correct, has
ZERO special-casing, touches NO reserved file, does NOT reorder the pipeline, leaves
the C++ oracle untouched, and produces ZERO regressions. It also produces ZERO new
passes: the reroll is INERT on the current corpus (the structurer emits
overflow-syntax `while(true){...break}` loops, so `finalTransform`'s
`has_overflow_syntax()` guard always declines). This inertness — and the identical
faithful transcription — is ALREADY recorded as **LOSS-197** (recorded by the sibling
`w10-forloop-stack` verifier; this branch's production logic is byte-identical to that
already-accepted branch modulo rustfmt wrapping + test names + comment wording).

The commit message HONESTLY self-discloses the inertness ("the reroll is correct but
currently INERT ... rust datatests 675 applied / 298 passing (0 regressions, 0 gains,
passing set identical)"). No passes were fabricated.

## Mandatory hunt list

- **Signedness**: clean. All index/slot/count arithmetic is `int4` vs `int4`
  (`num_input()->int4`, `get_in(int4)`, `get_out_rev_index(int4)->int4`, `1 - slot`
  over {0,1}, `path[count].slot` is int4). Matches the `-Wno-sign-compare` C++ exactly;
  no signed/unsigned comparison introduced.
- **Integer widths**: clean. No promotion-sensitive arithmetic; offsets/orders compared
  as their native int4. No bare `as` casts in the diff (grep clean).
- **Wrapping**: clean. The reroll does no `uintb` arithmetic; the only constant built is
  in code not added by this diff. `get_seq_num().get_order()` comparisons are plain.
- **Comparator totality**: N/A — no comparator ported here.
- **Iteration-order provenance**: `finalize_forloop_transform`/`finalize_forloop_printing`
  walk `sblocks.arena.iter()` (a `slotmap::SlotMap`, dense-index/insertion order,
  deterministic — NOT a HashMap). C++ recurses the structured `list` (insertion order)
  inner-before-outer. The Rust flat scan order can differ from C++'s inner-before-outer
  for NESTED whiledos. Per-node processing is independent (each node sets its own
  iterate_op/loop_def and moves ops only within its own loop blocks), so the result is
  order-insensitive for the cases tested; a nested-loop op-move interaction is a latent
  theoretical order gap, currently UNREACHABLE because the feature is inert
  (`has_overflow_syntax()` always bails). Recorded under LOSS-197's "inert" envelope.
  In `find_loop_variable`/`while_test_iterate_form`/`high_mark_expression` the path
  vectors reproduce the C++ fixed `path[4]` / `vector<PcodeOpNode>` DFS order exactly.
- **Off-by-one / do-while / reverse iterators**: clean.
  `find_loop_variable`'s fixed-depth `path[4]` walk is reproduced via
  `Vec::with_capacity(4)` with `count = len-1` and `if count == 3 { continue }` BEFORE
  push (max 4 entries, matching the C++ array bound); `path[count].slot++` read-then-
  increment is `let ind = .1; .1 += 1`. `isMoveable`'s C++
  `do { ++biter; ...; } while(biter != point)` at-least-once forward walk is
  `loop { advance; body; if biter == point break }` (point's own output IS processed by
  the tied-overlap check, matching C++). `moveRespectingCover`'s
  `do { nextOp; ...; } while(curOp != lastOp)` is `loop { next_op; ...; cur_op = next_op;
  if cur_op == last_op break }` — at-least-once preserved, `cur_op = next_op` applied
  before the terminal compare, matching C++. `findInitializer`'s `lastOp()==0` check
  precedes the `sizeOut()!=1` check in both.
- **Erase-while-iterating**: N/A — no container erasure during traversal.
- **Exception -> Result partial-state parity**: clean. The C++ methods do not throw;
  every early-out (`return`/`return (PcodeOp*)0`) maps to a Rust `return`/`None`/early
  `set_*(None)`. The `set_initialize_op(None)`/`set_iterate_op(None)` "turn it off"
  side effects on decline are reproduced at the same points (block.cc:3440 / 3464 /
  finalizePrinting). `find_initializer`'s success-only `initializeOp = res` side effect
  (and the last-chance call relying on it) is preserved.

## Faithfulness anchors (spot-verified against C++)

- `finalize_forloop_transform`/`whiledo_final_transform` vs block.cc:1356/3405 —
  overflow-guard, front-leaf head, body-lastOp, tail single-out-to-head, CBRANCH cond,
  branch-stepback, findLoopVariable, iterate-op move (allow_op_moves gate),
  findInitializer + isMoveable + initialize-op move. Faithful.
- `find_loop_variable` vs block.cc:3212 — exact path[4] DFS, MULTIEQUAL-in-head /
  itvn-written / iterate-in-tail / non-marker / isMoveable gates, `(possibleIterate, defOp)`
  return mapping to `(iterate_op, loop_def)`. Faithful.
- `find_initializer` vs block.cc:3271; `while_test_terminal` vs block.cc:3304;
  `while_test_iterate_form` vs block.cc:3335; `whiledo_finalize_printing` vs block.cc:3454
  — all statement-faithful (COPY/notPrinted unwrap, explicit/printed gates,
  moveRespectingCover, high-equality DFS truncating at explicit/annotation,
  last-chance-initializer, opMarkNonPrinting). Faithful.
- `op_is_moveable` vs op.cc:201; `op_move_respecting_cover` vs funcdata_op.cc:1498;
  `high_mark_expression` vs variable.cc:885 — special-op switch arms, crossCalls model,
  tiedList overlap, COPY/CAST-only crossing, mark set/clear symmetry, call(1)/load(2)
  ret_val bits. Faithful. One DELIBERATE safety divergence: the C++ `do-while` that
  derefs past end() when `point` precedes `this` (UB) is replaced by a
  `None => return false` bail — a strictly-safe guard for an input C++ never validly
  receives (pinned by the porter's `vfy_op_is_moveable_point_must_follow_this`).
- `emit_for_loop` vs printc.cc:3106 + the `emit_block_while_do` for/while dispatch vs
  printc.cc:3156 — keyword/paren/semicolon/comma-separate/brace sequence matches; the
  iterator `if let Some` guard is always-taken (only reached when iterate_op.is_some()).
  The `emitAnyLabelStatement`/`emitCommentBlockTree` omissions are the SAME pre-existing
  simplification as the plain-while arm (printc-body loss envelope).
- `analyze_for_loops` gate: seam default `true` matches C++ architecture.cc:1449
  default-on; the real arch's value is carried onto the seam (architecture.rs:748), so
  `option analyzeforloops off` reaches the gate. Faithful.

## Mechanical pass

- grep of added executable lines for function-name / address / offset / magic-number
  hardcoding: CLEAN (only literals are opcode enums + the `for` keyword id).
- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`partial_cmp`/bare-`as`:
  none in the diff.
- reserved files (coreaction_stackptr.rs, coreaction_infertypes.rs, deadcode,
  funcdata_spacebase.rs, jumptable.rs): NONE touched. Changed files are
  architecture.rs, block.rs, blockaction.rs, funcdata_block.rs, funcdata_op.rs,
  funcdata_op/tests.rs, printc.rs, seams.rs.
- B0 / pipeline order: unchanged — only the BODIES of the existing
  `ActionStructureTransform`/`ActionFinalStructure` hooks gained the for-loop calls
  (previously no-ops); `universalaction_listing` gate green.

## End-to-end parity (re-run)

- forloop1 (oracle asserts `for (... v1 = 0; v1 < max; v1 = v1 + 1)`): C++ oracle 1/1;
  Rust engine **0/1** — renders `while( true ) { if (a0 <= v1) break; ... v1 = v1 + 1; }`
  (overflow syntax). IDENTICAL on rust-port base and this branch. The reroll never fires
  (every loop in the for-loop corpus structures with `f_whiledo_overflow`).
- No-for-loop negatives (noforloop_alias / noforloop_globcall / noforloop_iterused):
  STAY while-loops on both base and branch (no negative became a `for`); their partial
  rust-engine failures are pre-existing render gaps (overflow-while + HighVariable
  naming), byte-identical base->branch.
- Whole-suite rust-engine datatests: 298/675 on BOTH base (`10fc2ab`) and branch
  (`c497cc3`); the failing SET (377 entries) is byte-identical (`diff` empty). No
  regression, no gain. boolless 2/2 unchanged; printc_parity / print_b5_boolless green.

## findings

  - F1 (info, non-blocking): the for-loop reroll is INERT end-to-end — 0 new datatest
    passes (forloop1 stays 0/1 on the rust engine, byte-identical to base). The blocker
    is the structurer emitting overflow-syntax while-loops (raw RSP stack-store ops keep
    BlockBasic::isComplex true), not this transcription. Already recorded as LOSS-197.
       cpp: decompiler/cpp/block.cc:3410 (`if (hasOverflowSyntax()) return;`)
       rust: rust/crates/kuna-decomp/src/funcdata_block.rs whiledo_final_transform (`has_overflow_syntax()` bail)
  - F2 (info, non-blocking): `struct_last_op` handles Basic/List/Condition/If but returns
    `None` for BlockCopy/BlockGoto/BlockMultiGoto, where C++ `lastOp()` recurses. A
    conservative false-negative (never a false reroll); unobservable while inert. Covered
    by LOSS-197 restoration criterion.
       cpp: decompiler/cpp/block.cc:2348/3008/3064/3167 (`lastOp` virtuals)
       rust: rust/crates/kuna-decomp/src/block.rs struct_last_op
  - F3 (info, non-blocking): whiledo-node iteration is slotmap-index (insertion) order,
    not C++ inner-before-outer recursion order. Order-insensitive for the per-node-
    independent reroll on the cases exercised; a latent nested-loop op-move ordering gap,
    unreachable while inert.
       cpp: decompiler/cpp/block.cc:1356 (`BlockGraph::finalTransform` list recursion)
       rust: rust/crates/kuna-decomp/src/funcdata_block.rs finalize_forloop_transform (arena scan)

## adversarial tests (added with this verdict, in funcdata_op/tests.rs)

  - vfy_w10_forloop_normal_op_crosses_indirect_lets_through — pins op.cc:267-270
    (INDIRECT in the let-through set; a normal op crosses it).
  - vfy_w10_forloop_normal_op_crosses_call_when_crosscalls — pins op.cc:271-275
    (a fully-untied normal op, crossCalls=true, may cross a CALL).
  - vfy_w10_forloop_unknown_special_op_blocks_move — pins op.cc:276-277
    (a CALLOTHER, not in any allowed arm, blocks the move via the default reject).

  These target the `op_is_moveable` special-op switch arms the porter's 8 tests left
  uncovered. All 3 pass on the branch.

## losses

  - LOSS-197 (w10-forloop-stack / w10-forloop-reroll): the whiledo->for reroll is a
    faithful, complete transcription but INERT on the current corpus (the structurer
    emits overflow-syntax loops; prerequisite stack-var / HighVariable-recovery passes
    are unported). Its rust-anchors are this diff's files verbatim. This branch's
    production logic is byte-identical to the already-accepted `w10-forloop-stack` modulo
    rustfmt/comment/test-name cosmetics. F1/F2/F3 fall within its envelope.
