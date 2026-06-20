# for-loop cluster COMPLETE — the 5-fix chain (w10-forloop-chain) — ACCEPT, +15

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-forloop-chain` @ `cec0805`, base
11cce78/517). Verdict: **ACCEPT**. The session's largest single win — resolves the dominant
convergent root (LOSS-235/156/233) after ~11 progressive diagnoses.

## The convergent root, finally cracked

The for-loop cluster + Copy trim render byte-identical to C++ once five faithful fixes land. The
root was misdiagnosed five times (RSP keystone → L4/L5 spill promotion → heritage forwarding timing
→ S7 structureReset → finally setCopyImmed) — each blocked wave sharpening it one layer.

## The 5 fixes (file:line anchors)

1. **RuleEarlyRemoval seam** (`ruleaction_1.rs:131`): was a `SEAM(heritage)` returning 0 for every
   deadcode space → rust fired 0 earlyremoval in oppool1 (C++ ~20), leaving the cond block at 12
   ops/complex (vs C++ 5/not-complex) → overflow → for-loop bailed. Wired to
   `Funcdata::dead_removal_allowed_seen` (funcdata.rs).
2. **setCopyImmed** (the LOSS-233 root, `ruleaction_3.rs:1943`): `RulePropagateCopy` skipped C++
   `op->setCopyImmed(i)` (ruleaction.cc:3972). Wired the existing `set_immed_copy_edge`/`immed_copy`
   infra → `ActionDoNothing` DELAYS the loop-exit block removal → the exit COPY stays in its own
   block (byte-identical to C++) instead of hoisting into the cond block. NOT flow.rs CFG construction.
3. **fd_sblock_last_op Copy-arm** (`funcdata_block.rs`): `BlockGraph::struct_last_op` lacked the
   `BlockKind::Copy` arm (C++ block.hh:546); threaded the sblocks→bblocks Copy hop into
   `whiledo_final_transform`.
4. **HighVariable purge on op-destroy** (`funcdata_op.rs:1928` op_destroy → `funcdata_varnode.rs:895`
   destroy_varnode, port of ~Varnode varnode.cc:629: `high->remove(this)` + delete-if-unattached via
   variable.rs `high_remove_member`/`is_unattached`/`erase`). Stops the switchloop naming crash (a
   destroyed vn was left in its HighVariable inst list → `vn_name_view` derefed a freed member).
5. **flipInPlace/negatetoken render** (`printc.rs`): ported the `opCbranch` `checkPrintNegation` arm
   (printc.cc:578) — when `boolean_flip` is set and the condition is token-flippable, absorb the `!`
   via the `negatetoken` modifier → `condconst_conn` renders `y != 10` (byte-identical) not `!(y==10)`.

Plus re-pinned 6 stale tests (4 for-loop verifiers + earlyremoval/cse unit tests) that pinned the
old broken while/seam behavior.

## Effect

517 → **532** (+15). Gained: Copy trim #1/#3/#6/#8, For-loop #1, For-loop thru special #1, For-loop
var used #1, For-loop with skip #1/#3, Inline target #4, No-for-loop global call #1, No-for-loop
iterator used #1, Partial union #3, Pointer Compare #1, Relative pointers #8.

## Gate (re-run at integration onto 517)

- `cargo test --workspace --no-fail-fast`: 0 failures.
- datatests: `[675, 532]`; passing-set diff vs base 517: **regressed-set EMPTY** (gained exactly the 15).
- switch cluster: switchind 16/16, switchmulti 9/9, switchhide 3/4 — held. oracle **PARITY OK**.

## Why this is a fence-discipline landmark

The prior wave measured +15 with regressed-set EMPTY *on the datatest oracle* but FAILED the
`--no-fail-fast` fence (5 byte-parity regressions the stringmatch oracle missed). The complete chain
clears both. This is the clearest proof of why the fence (not the datatest count alone) is the gate.

## No special-casing

All five fixes are faithful C++ transcriptions; no name/address/value hardcoding.
