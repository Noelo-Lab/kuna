# RuleConditionalMove + CloneBlockOps (w10-conditionalmove) — ACCEPT, +3

Wave (2026-06-18) integrated at rust-port (merge `b4f435d` of `rport/w10-conditionalmove` @ `9fb2930`,
base 09bd629/509). Verdict: **ACCEPT**. Closes the RuleConditionalMove half of LOSS-074.

## What landed (a long-standing W4 seam)

`RuleConditionalMove` (`ruleaction_7.rs:1883`) was a no-op SEAM (matched MULTIEQUAL+CBRANCH topology
then `return 0`). Ported the full C++ rule (ruleaction.cc:9292-9405) + its CloneBlockOps prerequisites:
- `funcdata_op.rs:2153` `Funcdata::op_bool_negate` (C++ funcdata_op.cc:560).
- `funcdata_block.rs:2659` `CloneBlockOps::clone_expression` (C++ funcdata_block.cc:1043) on the
  pre-existing build_op_clone/build_varnode_output/patch_inputs shell, + `Funcdata::clone_expression`
  wrapper.
- `ruleaction_7.rs` `gather_expression`/`construct_bool` + the full `apply_op` rewrite (both
  `boolcond ? trueval : falseval` and `boolcond || differentcond` variations, bool-const arms,
  path0istrue/boolean-flip/needcomplement).

It pulls the boolean expression out of a conditionally-executed block, collapsing the flag-chain
MULTIEQUAL into BOOL_AND/BOOL_OR so downstream rules (RuleSborrow, RuleFloatRange, RuleIgnoreNan) fire.

## Effect

509 → 512. Gained: Ccmp test #2/#3 (the CCMP flag-chain → `val <= 9`), NaN operations #1. condmove #7
(LOSS-234 mergeAddrTied) + the NaN tail (the `&& 1` BOOL_AND-const-true post-pool issue, LOSS-074
REFINEMENT) have separate deeper roots.

## Gate (re-run at integration onto 509)

- `cargo test --workspace --no-fail-fast`: 0 failures.
- datatests: `[675, 512]`; passing-set diff vs base 509: **regressed-set EMPTY** (gained the 3).
- switch cluster switchind 16/16, switchmulti 9/9 unchanged; oracle **PARITY OK**; C++ byte-untouched.

## No special-casing

Faithful transcription of RuleConditionalMove + CloneBlockOps::cloneExpression + opBoolNegate; no
name/address/value hardcoding. The CloneBlockOps expression cloner is now available for future
conditional-move/select work.

## Integration note

The merge initially landed on a detached HEAD (the main tree had checked out an agent branch); caught
via the pre-merge `git branch --show-current` check and recovered by fast-forwarding rust-port to the
(correct, gated) merge commit. Lesson reinforced: verify the active branch before every integrate.
