# RSP/Spacebase Keystone — Sequenced Implementation Plan

Design pass (read-only) 2026-06-16. The dominant remaining M3 lever (~187/381 failing
assertions across 26/83 datatest files). This is a **coupled multi-layer landing**: each
layer is individually correct/ported but yields +0 in isolation; the only thing that makes
them net-safe is landing them together with the jump-table co-requisite that prevents the
**switchind regression**.

## Corrected root cause (supersedes the stale "propagateSpacebaseRef" diagnosis)

`propagateSpacebaseRef` is **already fully ported + wired** (`coreaction_infertypes.rs:1356`,
called `:1582`) — NOT the blocker. The actual blocker (LOSS-201/148): the `"jumptable"`
ActionDatabase group (`coreaction.cc:5694`) **excludes `typerecovery`** but **includes
`stackptrflow`**. When `ActionExtraPopSetup` inserts a per-call `INT_ADD(RSP, extrapop)`,
jump-table recovery clones the function (`truncatedFlow`) and runs the `"jumptable"` group,
which has no InferTypes to clean the spacebase op and relies entirely on
`ActionStackPtrFlow::analyzeExtraPop`'s StackSolver. That solver's INDIRECT-companion equation
(`coreaction.cc:226-228`) reads `fc->getExtraPop()`, written by **`setEffectiveExtraPop` — the
one piece dropped in the Rust port** (LOSS-148, `coreaction_stackptr.rs:685-694`). Without it
the switch index's stack-relative offset is mis-recovered and switchind loses case-label
recovery.

## Layer dependency graph

- **L0** `ActionExtraPopSetup::apply` (trigger; STUBBED `coreaction_protos.rs:470-511`) — inserts per-call `INT_ADD/INDIRECT(RSP)`. C++ `coreaction.cc:1452-1482`.
- **L1** `setEffectiveExtraPop` write in `analyzeExtraPop` (THE SWITCHIND GATE; GAP `coreaction_stackptr.rs:685-694`, LOSS-148). C++ `coreaction.cc:321` + StackSolver `:226-228`.
- **L2** `propagateSpacebaseRef` — ALREADY LIVE (`coreaction_infertypes.rs:1356/1582`). No change.
- **L3** `ActionDeadCode` consume-sweep — ALREADY LIVE (`coreaction_render.rs:2124`, BRANCHIND consume `:2209`). Kills the dead retaddr store + RSP residue once L0/L1/L4 make the flow clean. No change.
- **L4** `ActionRestrictLocal::apply` (STUB `coreaction_protos.rs:1141-1166`) + `ScopeLocal::markNotMapped`/`isUnaffectedStorage` (NOT in `varmap.rs`, LOSS-109/145). C++ `coreaction.cc:2003-2059` + `varmap.cc`. The `//rsp` UNAFFECTED-marking + prologue/epilogue `RSP=RSP+N` elimination + return-addr-store unmapping.
- **L5** restructureVarnode tail: `markUnaliased`/`checkUnaliasedReturn`/`nolocalalias` (absent/stubbed `funcdata_spacebase.rs:636-639`, LOSS-145/147). C++ `varmap.cc:1256-1286/1332/414-428` + `funcdata_varnode.cc:999-1001`. cast-output-typing (`xunknown1*`->`float4(*)[1]`) + inner-PTRSUB struct-member typing. **COLLIDES with cast-plane/printc** — sequence LAST.
- **L6** for-loop reroll — ALREADY PORTED + INERT (`funcdata_block.rs:196` `has_overflow_syntax` bail). AUTO-ACTIVATES once L0-L5 strip the RSP noise so `bb_is_complex` drops. No code.

**Minimal net-safe atomic landing = L0 + L1.** ← **DISPROVEN — see CORRECTION below.**

## CORRECTION (Wave A failed 2026-06-16, wxfjsp3lu — instrument-proven)

L0+L1 alone is **NOT** net-safe — landing it regresses switchind (`switch(v1)`→`switch(0x100058)`,
`if(v1<=10)`→`if(1)`). The porter instrumented and found: **L1 (`setEffectiveExtraPop`) never
fires for switchind** — `analyze_extra_pop` (`coreaction_stackptr.rs`) **early-returns on the
known `defaultfp` extrapop** before reaching the solve loop, so the StackSolver companion
equation the plan hinged on is never exercised. The 13/16 switchind *assertion count* is
unchanged but the full rendered C is not byte-identical (the switch index degrades to a raw
constant). L0's 7× `INT_ADD(RSP,8)` per-call ops are inserted (confirmed via KUNA_L0DBG) but
nothing neutralizes them in the jumptable clone before BRANCHIND emulation.

**The TRUE keystone (must port BEFORE L0):** the C++ pass in the `"jumptable"` ActionDatabase
group (`coreaction.cc:5694`, excludes typerecovery, includes stackptrflow) that **neutralizes
the per-call `INT_ADD(RSP,8)` before BRANCHIND index emulation** — for the *known-extrapop*
path (which bypasses `analyzeExtraPop`'s solve loop). It is NOT `analyzeExtraPop`/L1. Locate it
(likely the StackSolver's main propagation in `ActionStackPtrFlow`, or a spacebase
simplification in the jumptable group), port it, prove `KUNA_DUMP print raw` keeps
`switch s0xffffffffffffffec:4(i)` (not `switch #0x100058`), THEN land L0+L1 on top. The faithful
(compiling) L0+L1 transcription is saved at `w10-rsp-waveA-L0L1-wip.patch` for the retry.

**Revised minimal landing = [jumptable INT_ADD consumer] + L0 + L1**, atomic.

## Sequenced waves

- **Wave A = L0+L1** (atomic). Owns `coreaction_protos.rs` (ActionExtraPopSetup only) + `coreaction_stackptr.rs` (the `set_effective_extra_pop` wire). Expected +0 substrate; UNBLOCKS B/C/D. **HARD GATE: switchind byte-identical** (the 13 Switch-Indirect assertions #1-7,#9-14) + `verify_w10_jts_chain` + `verify_w10_extrapop_jumptable_noregress` green. *[LAUNCHED 2026-06-16 as wxfjsp3lu.]*
- **Wave B = L4**. Owns `varmap.rs` (`mark_not_mapped`/`is_unaffected_storage`) + `coreaction_protos.rs` (ActionRestrictLocal un-stub). Removes `//rsp` spurious locals + prologue/epilogue. Flips No-for-loop + stack-string/spill (~14).
- **Wave C = L5**. Owns `funcdata_spacebase.rs` (restructure tail, `nolocalalias`) + `varmap.rs` (markUnaliased/checkUnaliasedReturn). **COLLISION ZONE** — read-only against `coreaction_casts.rs`/`printc.rs`/merge; gate on cast goldens. LARGEST yield: Concat stack arm (18+7), Piece-Structure (6), Partial-splitting (20+11), `float4(*)[1]`/struct-array stack vars.
- **Wave D = L6** (verification only). For-loop cluster auto-activates (~12: forloop1/loaditer/varused/withskip/thruspecial). `noforloop_*` negatives must stay while-loops.

## Top risk + gate

**The switchind regression (HARD).** L0 alone regresses jump-table index recovery. Mitigation: (1) L0 and L1 MUST be the same atomic wave; (2) pin with `verify_w10_extrapop_jumptable_noregress` (exact 13-assertion set) + `..._switchmulti_first_assertion_survives`; (3) byte-diff the switch cluster base-vs-branch; (4) bisection probe if it regresses — `KUNA_DUMP=1` on the switchind partial clone, confirm the StackSolver clean-solved `INT_ADD(RSP,8)` (companion-paired) not an unsolved INDIRECT / `65535` 'unable to track', and `get_op_from_const(op.get_in(1).get_addr())` resolves to the cloned op via `get_call_specs_index`. Secondary: L5's cast-plane collision — sequence last, own only `funcdata_spacebase.rs`/`varmap.rs`.

## Critical files
- `coreaction_protos.rs` (L0 :470-511; L4 :1141-1166)
- `coreaction_stackptr.rs` (L1 :685-694 — the switchind gate)
- `varmap.rs` (L4 markNotMapped/isUnaffectedStorage; L5 markUnaliased/checkUnaliasedReturn)
- `funcdata_spacebase.rs` (L5 restructure tail + nolocalalias :636-639)
- `coreaction_render.rs` (L3 deadcode_apply :2124 — verify only) ; `coreaction_infertypes.rs:1356` (L2 live) ; `funcdata_block.rs:196` (L6 inert→auto)
