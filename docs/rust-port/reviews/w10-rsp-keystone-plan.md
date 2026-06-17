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

## CORRECTION-2 (deep empirical re-diagnosis 2026-06-16, supersedes CORRECTION)

The "missing jumptable INT_ADD consumer" hypothesis is ALSO wrong. The consumer is
**`RuleAddMultCollapse`** (`ruleaction_3.rs:2082`, + `RuleCollectTerms` `ruleaction_1.rs:152`),
in the `analysis` group which IS in the `"jumptable"` group (`action.rs:1589-1595`, byte-identical
to C++) — **already ported and correct**. ExtraPopSetup's per-call `INT_ADD(RSP,8)` is the
algebraic inverse of the SLEIGH CALL's retaddr push `RSP−8`; `RuleAddMultCollapse` folds
`(RSP:2−8)+8 → RSP:2`. Proven from the committed goldens `tests/golden/snapshots/cpp/switchind/
000-B3.txt`/`000-B4.txt` (C++ B4: `switch s0xfffffffffffffff4:4`, slot −0xc).

**switchind is currently 13/16, NOT 16/16.** With ExtraPopSetup STUBBED (no `+8`), the index
LOAD resolves to `0xc + (RSP:2−8) = RSP_in − 0x14` → a spurious `uint4 v1; // stack − 0x14`;
the jump table still recovers structurally (13 assertions) but #15 (`get_value_byref(&val)`) and
#16 (`switch(val)`) FAIL. (The `w10-extrapop-jumptable.md` review mis-stated #15/#16 as passing.)
So landing L0 **correctly** FLIPS #15/#16 green (slot −0x14 → −0xc) → switchind 16/16 (+2/+3),
it does NOT "regress switchind."

**The Wave-A failure** (`switch #0x100058`) = the `+8`/`−8` fold did NOT fire in the clone: an
**L0 op-insertion / heritage-ordering** problem (the `+8` op, `opInsertAfter(op, fc->getOp())`,
must heritage onto the post-call RSP SSA value so `RuleAddMultCollapse` sees `(RSP−8)+8`), NOT a
missing pass. **L1 (`setEffectiveExtraPop`) is irrelevant to switchind** (analyze_extra_pop
early-returns on known defaultfp extrapop; it only covers `extrapop_unknown` INDIRECT calls) —
keep it faithful-but-inert, drop it from the critical path.

**Corrected gate** (NOT byte-identity — L0 legitimately changes the rendered C by fixing slots):
switchind 13→16 (the 13 hold AND #15/#16 flip green; B4 shows `switch s0x..f4:4` not
`switch #0x100058` not −0x14); all other switch datatests + the 333 suite MONOTONIC (none
regress, may improve). **Wave A′ = L0 (WIP patch) + diagnose/repair the L0 op-insertion ordering
so the existing fold fires.** Smaller + better-understood than the plan assumed.

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

## CORRECTION-4 (live-engine deep investigation 2026-06-17, wifc7lawm — DEFINITIVE)

Instrumented (KUNA_RSPDBG probes + cpp/rust `decomp_dbg print raw` + KUNA_DUMP) — supersedes
CORRECTION-3's "INT_ADD targets register-space" guess (that was wrong; `register:0x20` IS the
RSP register and is correct).

**THE SHARED ROOT (confirmed, both leads):** the per-call **extrapop=8** pop (cspec `__stdcall`
extrapop=8) is unmodeled, so the SLEIGH CALL retaddr push (RSP−8, cspec stackshift=8) is never
canceled → **every stack slot is skewed by exactly 8** (Rust −0x14 vs C++ −0xc). Lead A
(switchind `&val`/switch-index lost) and Lead B (longdouble float10 stack-arg dropped/split)
both depend on the coherent stack frame that only ExtraPopSetup + its downstream cleanup produce.

**THE ATOMIC LANDING = L0 + L3** (single wave; L1/L4/L5 are not the critical-path blocker):
- **L0** un-stub `ActionExtraPopSetup::apply` (coreaction_protos.rs; faithful body exists behind
  `KUNA_L0=1` + the `w10-rsp-waveA-L0L1-wip.patch`). Inserts `INT_ADD(RSP@register:0x20,+8)` per
  known-extrapop call. Proven correct in isolation but **NET-NEGATIVE alone** (switchind degrades
  to `switch #0x100058`/`if(1)` — exactly the RSP A' failure).
- **L3 (THE MISSING CO-REQUISITE — NOT the spacebase setup):** `ActionDeadCode` consume-sweep
  (coreaction_render.rs `deadcode_apply` :1552) must remove the dead retaddr store + the folded
  `+8` residue **inside the `"jumptable"`-group clone** (truncatedFlow; action.rs:1589 group is
  byte-identical to coreaction.cc:5694). **Instrument the truncatedFlow clone body: confirm
  `INT_ADD(RSP,8)` is gone and the BRANCHIND index varnode is `s0x..ec`-relative (a slot) NOT a
  const, BEFORE reverse-emulation.** This is where every prior attempt must focus.
- **NET-SAFETY:** L0 raises the slot −0x14→−0xc (correct); only L3 stops the jumptable clone from
  const-folding the index. They MUST land together (L0-alone net-negative; L3-alone +0). Gate:
  switchind 13→16 (#8 default:/#15 `get_value_byref(&val)`/#16 `switch(val)` flip; B4
  `switch s0x..f4:4` slot −0xc), all switch + the 384-suite monotonic.
- **L1** (analyze_extra_pop coreaction_stackptr.rs:650 early-returns on known defaultfp extrapop —
  byte-faithful to coreaction.cc:282) is faithful-but-inert; NOT on the critical path.
- **L4/L5** (ActionRestrictLocal un-stub + restructureVarnode tail) sequence AFTER L0+L3 for the
  `&val` PTRSUB-arg + `int4 val // stack - 0xc` render.

## CORRECTION-5 (L0+L3 attempt 2026-06-17, wpeihboit — the definitive scope)

L0+L3 (CORRECTION-4's "jumptable-clone deadcode") is ALSO disproven by direct attempt:
switchind stays 13/16, L3 is not the missing piece. The instrumented attempt establishes the
TRUE coupled set (5 layers, atomic):

- **L0** un-stub ActionExtraPopSetup (coreaction_protos.rs; WIP patch). Inserts the cancelling
  per-call `INT_ADD(RSP,+8)`. **NOT separably landable**: its "+6 datatest (Concat-split +
  For-loop-thru-special), regressed-set EMPTY" is the DATATEST COUNT MASKING the switchind
  *render* fence regression (switch index degrades to `switch #0x100058`) — confirmed the
  count-vs-fence lesson again. L0 only nets positive WITH the downstream cleanup below.
- **heritage stack-INDIRECT guard** (`heritage.rs:1514`, `guard_calls` non-persist
  unknown_effect INDIRECT for addr-tied ranges — currently gated). Make it C++-faithful
  (heritage.cc:1514). **Un-gating ALONE is net −15** — needs the INDIRECT-collapse/cover + L4/L5
  cleanup to be net-safe.
- **`resolveSpacebaseRelative` call chain**: ported (`fspec.rs:6668`) but **UNCALLED**; the call
  site is `ruleaction_4.rs:409`. Required for `tryreg=true` → #15 `get_value_byref(&val)` + the
  proper stack offset.
- **L4** ActionRestrictLocal un-stub (coreaction_protos.rs) + **L5** restructureVarnode tail —
  the clean `int4 val; switch(val)` local render once the index stays symbolic.

**VERDICT: RSP is a genuinely cross-cutting multi-layer coupled keystone** (heritage +
jumptable-clone + action pipeline + varmap/ScopeLocal + fspec) requiring an ATOMIC 5-layer
landing — net-safe only as a whole (L0 +6/render-break, guard −15, the rest +N). It is NOT
crackable in a single wave; it needs a dedicated focused multi-wave session that lands all 5
layers together against the switchind-16/16 gate. Gate unchanged: switchind 13→16 (#8/#15/#16
flip, B4 slot −0xc), all switch + the suite monotonic, fences (not just the datatest count).

## CORRECTION-6 (5-layer attempt 2026-06-17, way4ufw8y — THE BREAKTHROUGH, disproves the 5-layer premise)

The blocker is NOT the action pipeline (L0/heritage/resolveSpacebase/L4/L5). It is TWO
precisely-instrumented gaps no prior attempt found:

**ROOT-A (THE TRUE ROOT — NOW FIXED, net-zero-safe, in branch rport/w10-rsp-5layer-atomic
@ 0983561):** the Rust port never parsed the cspec `<unaffected>/<killedbycall>/<returnaddress>`
effect blocks into `ProtoModel::effectlist` — `push_effect` was **DEAD CODE** (never called),
effectlist always empty, so `FuncProto::hasEffect(register:0x20 / RSP)` returned
`unknown_effect(4)` instead of `unaffected(1)`. Fixed in `architecture.rs::decode_default_proto`
+ new `decode_effect_block` (faithful to ProtoModel::decode). **Instrument-proven RSP effecttype
4→1; this is what makes the slot resolve to −0xc not −0x14** (with L0+heritage+this, switchind B4
= `int4 val; // stack - 0xc; switch(val)`, #16 GREEN). *This is why every prior attempt failed —
they patched the action pipeline while the proto effectlist was silently empty.* Also landed
net-zero-safe in 0983561: `ScopeLocal::markUnaliased` (database.rs/varmap.rs) + the
`resolveSpacebaseRelative` call-site wire (ruleaction_4.rs).

**ROOT-B (THE SINGLE REMAINING BLOCKER for #15/#8 + switchind 16/16):** the call
INPUT-ACTIVE argument recovery does NOT pass `&val = PTRSUB(RSP_in,−0xc)` (in register RDI) to
`get_value_byref`. The binary does `lea rdi,[rsp+0xc]` before the call; the decompiler must
register RDI as an **active input trial** (`heritage.rs guardCalls is_input_active` →
RDI `characterize_as_input_param` yields `ContainsJustified` for get_value_byref's recovered
proto) so RDI=&val survives as the call argument. Currently RDI is dead-coded → &val absent →
(a) #15 fails, (b) the −0xc alias is unstable in the AliasChecker → markUnaliased mis-marks the
val slot nolocalalias → heritage guard collapses → index const-folds.

**The genuine FINAL layer = the call input-active argument recovery** (ActionActiveParam /
`is_input_active` trial for the RDI stack-pointer arg) — a DISTINCT parameter-recovery-trial
subsystem, tractable but not in any prior plan. Instrument-verified order: effectlist →
markUnaliased → L0 → heritage-ungate gives switchind 14/16 (#16 green) but −6 (heritage INDIRECTs
over other locals need nolocalalias AND the &val alias stable); **ONLY the RDI &val recovery
closes both at once → switchind 16/16.** NEXT = ROOT-B + L0 + heritage-ungate on the 0983561
substrate.

## CORRECTION-7 (ROOT-B attempt 2026-06-17, wv4xgtvsw — switchind 14/16 reached; ONE blocker left)

ROOT-B IS the call input-active trial subsystem, and it reached **switchind 14/16** (#16
`switch(val)` GREEN, slot −0xc, `&val` recovered, **+8 datatests**: Concat-split #2-6,
For-loop-thru-special #2, Indirect-prototype #3, Switch-Indirect #16) — but a render FENCE
regressed (loopcomment duplicate vN decls), so it reverted to the 0983561 substrate (the
count-vs-fence lesson, AGAIN). The substrate's input-active branch (guardCalls is_input_active +
characterize_as_input_param) ALREADY works (RDI→ContainsJustified, trial registered). ROOT-B is
**three coupled gaps** (all verified necessary):

1. **`Funcdata::check_call_double_use`** (funcdata_varnode.rs ~:1910) is a STUB returning `false`
   (C++ funcdata_varnode.cc:1802) — THIS dead-codes RDI=&val (RDI flows into get_value_byref AND
   the casefunc* calls; the non-matching calls hit check_call_double_use → stub `false` →
   markInactive → trial not `used` → &val dropped). Port faithfully (getSlot/getCallSpecs/
   getEntryAddress/getTrialForInputVarnode/isAlternatePathValid; add const `active_input()` to
   FuncCallSpecs, use get_call_specs_index(op)+get_call_specs(idx)).
2. **`FuncCallSpecs::createPlaceholder`** (fspec.rs ~:6615) + `Funcdata::opStackLoad`/
   `createStackRef`/`newSpacebasePtr` (funcdata_op.rs) — stubbed W4 seams; port (createStackRef =
   INT_ADD(spacebase, byteToAddress(off)); opStackLoad = LOAD; newSpacebasePtr at getSpacebase(0);
   SEGMENTOP arm unreached on flat stacks — faithful seam comment). Wire createPlaceholder into
   func_link_input's `if (spacebase != 0)` tail (else getSpacebaseOffset stays offset_unknown →
   tryregister=false for the val slot).
3. **ARCHITECTURE FIX — ActionActiveParam::apply + check_input_trial_use MUST be INDEX-BASED**
   (keep call specs on data.qlst), NOT `take_call_specs` (the take empties data.qlst → 
   get_call_specs_index(op) returns None → check_call_double_use can never find the other call's
   spec). Iterate `for idx in 0..data.num_calls()`, mutate via get_call_specs_mut(idx)
   .get_active_input(); for ancestor_op_use clone the trial then write back. Single-spec
   take/restore ONLY for finalize (no cross-call lookup). ParamTrial is Clone.

Plus re-apply L0 (ActionExtraPopSetup, w10-rsp-waveA-L0L1-wip.patch) + L1 (setEffectiveExtraPop)
+ heritage non-persist INDIRECT un-gate (heritage.rs ~:1510, drop `persist_range &&`). L0 alone
const-folds the index BECAUSE &val isn't passed; ROOT-B's &val recovery is what stops the fold.

**THE FINAL BLOCKER to 16/16 (#15 cast + #8 default) — the next target:** an **int8-vs-int4
DUPLICATE-HIGHVARIABLE TYPE COLLISION** in the cast/merge plane. The recovered `&val` arg (RDI,
8-byte register pointer) makes the decompiler synthesize a SECOND 8-byte HighVariable `int8 val`
shadowing the mapped 4-byte `int4 val` (// stack −0xc). So &val types as int8* not int4* →
ActionSetCasts inserts `(int4 *)` (breaks #15: `get_value_byref((int4 *)&val)` vs oracle
`get_value_byref(&val)`), and `(uint4)val<=10` keeps the bound-check from folding into `default:`
(#8). **The mapped int4-val type-lock must DOMINATE so PTRSUB(RSP,−0xc) types as int4*** —
backward type-prop through TypeOpPtrsub / restructureVarnode unification (the L5 cast-plane). The
SAME pathology corrupts loopcomment (3× `v2` decls int8/int8/int4) — caught by
verify_w10_hvnaming...gapless. Fix the unification → #15/#8 + loopcomment all close → 16/16.

STALE FENCES to update on relanding (pre-keystone residue now correctly cleaned): forloop1
`// rsp` → a1b; nanops now recovers BOTH float8 params (oracle `void nanops(float8,float8)`);
partialsplit `stackother` → `&stackother`; forloop_varused `// rsp` proxy → `(uint8)` cast proxy.

## CORRECTION-8 (final RSP wave 2026-06-17, whezwcpck — KEYSTONE CRACKED: switchind 15/16, +13)

ROOT-A + ROOT-B + the int8-vs-int4 type-collision are ALL SOLVED. switchind 13→15 (#15
`get_value_byref(&val)` NO cast + #16 `switch(val)` slot −0xc both GREEN), datatests 384→397
(**+13**, regressed-set EMPTY), loopcomment CLEAN, cargo test green, oracle PARITY. State on
branch rport/w10-rsp-5layer-atomic @ bf4ca27 (pending keystone-verify → integration).

**PART 2 (type-collision) SOLVED by two C++-faithful changes (re-apply on any reland):**
- (A) `coreaction_infertypes.rs propagate_add_in2_out`: when the down_chain `cur`'s ptr_to is
  TYPE_SPACEBASE, resolve via `data.spacebase_get_sub_type(&sbptrto, type_offset)` +
  `get_type_pointer_strip_array` (scope-aware — reproducing C++ TypePointer::downChain's
  `ptrto->getSubType` → TypeSpacebase::getSubType, type.cc:1248) INSTEAD of the generic
  `tlst.down_chain` (which can't reach the symbol scope → PTRSUB(RSP,−0xc) propagated as bare
  int8* and forced casts). Now the PTRSUB output types int4* during inference → cast_output
  token==high → no cast.
- (B) `printc.rs emit_local_var_decls`: skip declaring a constant-only HighVariable that has a
  storage-sibling of the same name (≥1 non-constant instance). The WHOLE-SIBLING guard is
  load-bearing — do NOT use bare all_constant (it drops the const-only `&c` stack array in
  passPtrToArray which has no storage sibling; t2b_local_frame_spacebase_symbol_still_declared
  catches that).

**THE ONLY REMAINING PIECE for switchind 16/16 = #8 `default:`** (a SEPARATE structuring seam;
#8 also fails on baseline 384, so 15/16 is NOT a regression): the `if ((uint4)val <= 10)` guard
must fold into the switch as `default:`. The fold machinery is ALREADY PORTED but inert
(jumptable.rs `JumpBasicModel::fold_in_one_guard` #[allow(dead_code)], funcdata.rs
`block_no_intervening_statement`, funcdata_block.rs `push_branch`; `install_switch_defaults`
already marks the default edge from get_default_block). Enabling `fold_in_guards` (return the
real result not Ok(false)) folds the guard BUT leaves an `if(1)` constant-CBRANCH residue — C++
relies on a SUBSEQUENT condexe/deadcode pass to collapse it; the Rust pipeline doesn't
re-simplify after ActionSwitchNorm, so `if(1)` survives, the switch never absorbs `default:`,
AND Switch-Multi #1 regresses. **TO CLOSE #8:** after fold_in_guards sets the default + constant
predicate, run a condexe/RuleConditionalMove/deadcode re-pass over the folded block to collapse
the `if(1)` (or convert the else-branch CBRANCH to an unconditional BRANCH in-place, like the
pos==nout push_branch arm). Then enable fold_in_guards. Commit only at switchind 16/16 + Switch
Multi #1 still green + regressed-EMPTY.

**The 4 updated render fences (jts_chain/spacebase_render/input_params/struct_corpus) are
justified POST-KEYSTONE residue-updates** (float10-arg reconstruction, nanops 2nd float8 param,
partialsplit &stackother, forloop //rsp cleanup) — each documented inline + being independently
re-certified against the oracle by the keystone-verify.
