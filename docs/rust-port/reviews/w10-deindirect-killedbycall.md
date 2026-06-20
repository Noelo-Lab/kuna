# call-return recovery: killedbycall + ActionDeindirect (w10-deindirect) — ACCEPT, +5

Wave chain (2026-06-17) integrated at rust-port (merge of `rport/w10-deindirect` @ `e693201`,
which carries the substrate merge `3cdab57` of `rport/w10-killedbycall` @ b72bfea). Verdict:
**ACCEPT**. Resolves LOSS-230 (the killedbycall + deindirect halves).

## Chain (two waves)

This is the two-wave chain documented across LOSS-230 + its corrections:
1. **killedbycall heritage seam** (substrate b72bfea, +3/-1 alone): `Heritage::guard_calls`
   output-active trial + KILLEDBYCALL INDIRECT-creation (heritage.cc:1470-1526) + un-stubbed
   `ActionFuncLink::init_active_output` (coreaction.cc:1611). Recovers call return outputs. Its
   lone -1 (Deindirect Output #2) was caused by the separate ActionDeindirect stub.
2. **ActionDeindirect** (this wave): removes that regression by deindirecting the CALLIND to a
   known-proto CALL before output recovery.

## ActionDeindirect roots + fix

- `coreaction_render.rs` `ActionDeindirect::apply` (was stub returning 0): iterates CALLIND ops,
  traces in0 through COPYs, resolves the target, calls `deindirect`.
- `seams.rs` `Architecture::query_function` — exact `TYPE_CODE` function-entry hit off the frozen
  `GlobalQuery` (C++ `Scope::queryFunction` database.cc:1292) — NO `database.rs` edit (avoided the
  concurrent stacklocal-typing wave).
- `fspec.rs` `FuncCallSpecs::deindirect` (was SEAM stub): `newVarnodeCallSpecs` + `opSetInput` +
  `opSetOpcode CALLIND→CALL` + `insertIndirectOverride`, then `commit_new_inputs/outputs`
  (fspec.cc:5155/5206) + `build_param` (5010) + `transfer_locked_output`/`...Param` — merges the
  two killed-output trial pieces into one clean RAX return-value Varnode.
- `flow.rs` `setup_callind_specs` — consumes `applyIndirect` (flow.cc:729) for the restart path.

## Effect

468 → 473 (+5). Gained: Conditional Add #1, Conditional Subpiece #1, Deindirect Output #3
(bonus), Modified conditional constant #2/#3. Deindirect Output #2 recovered (the substrate's -1
gone). The printc.rs:3810 SUBPIECE-cast arm was NOT enabled (still available as a follow-up for
Union #8/#14/#28 + Bitfields #4, now that the IR root is fixed).

## Gate (re-run at integration onto 468)

- `cargo test --workspace` (debug): 0 failures (the two fspec deindirect unit tests now exercise
  the real IR rewrite).
- datatests: `[675, 473]`; passing-set diff vs base 468: **regressed-set EMPTY** (gained the 5).
- switch cluster 8/16/3 unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining (separate seams)

Deindirect Output #1 (locked-output `int4 *` type not propagated to the output HighVariable — an
InferTypes seam) and Deindirect #2 (the `hasTypeRecoveryStarted` function-pointer-proto `forceSet`
arm, coreaction.cc:1274-1293) were already failing in base-468.

## No special-casing

Faithful transcription of ActionDeindirect/FuncCallSpecs::deindirect/queryFunction; no
name/address/value hardcoding.
