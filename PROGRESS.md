# kuna Progress Log

## Session (2026-06-17b) — rust-port W10: 400 → 408/675; ActionReturnSplit + Convert #17

**Two porter-committed survivor waves integrated via the main loop (+8 → 408).** Both waves'
independent verify phases died on a transient API 529 throttle; their ports were committed
before the death, so they were recovered and re-gated in the main loop (full per-assertion
passing-set diff, fence catcher, switch-no-regression, oracle PARITY, C++ untouched).
- **ActionReturnSplit + Funcdata::nodeSplit (+7 → 407)** — return-block duplication so the
  structurer recovers per-arm `return <expr>;`. switchmulti 1/9 → 8/9 (Switch-Multi
  #2/#4/#5/#6/#7/#8/#9; #3 pre-existing). Merge `7339946`; review
  `reviews/w10-returnsplit-nodesplit.md`. Gate: cargo test 3666/0, `[675,407]`, regressed-set
  EMPTY, PARITY OK.
- **Convert #17 — equate survives the size-4 DynamicHash fold (+1 → 408)** — `recv_signed(L'a')`
  now byte-identical to the oracle. The remaining convert fails (#2/#6/#10/#14) are a separate
  pre-existing negative-constant rendering seam. Merge `71f495a`; review
  `reviews/w10-convert17-equate-rebind.md`. Gate: cargo test 3669/0, `[675,408]`, regressed-set
  EMPTY, PARITY OK.

Server throttle finding: 7 concurrent heavy waves tripped a rate-limit/529; ~3–4 is the real
ceiling. Dead waves to relaunch when stable: namespace (+3), enum4 (+1), De Morgan (+5–7),
and the RSP-harvest &v1-render layer (L4/L5 branch @ `7f1f4df`, CORRECTION-10).

## Session (2026-06-16d/17) — rust-port W10: 307 → 400/675; RSP KEYSTONE + switchind 16/16

**RSP #8 guard-fold LANDED (+3 → 400): switchind now FULLY recovers 16/16.** Enabled
`fold_in_guards` + **wired `removeBranch`** (the W3-block seam — the `if(1)` constant-CBRANCH
collapse) so the bound-check guard folds into the switch as `default:`. Flips Switch-Indirect #8
(default:), If/Switch #2, Switch-Hide #2; regressed-set EMPTY, cargo test 3663/0, oracle PARITY.
The `verify_w5_s3_coreaction_early` update is a STRENGTHENING (it now verifies removeBranch
actually folds the 2 constant CBRANCHes — was a seam no-op pin). #8's independent verify died on
a transient API 529; gate re-run + spot-checked clean in the main loop. **RSP HARVEST STATUS:**
keystone (+13) + switchind-16/16 (+3) landed; the spacebase-render chain (L4/L5 substrate
@ 7f1f4df + restructureVarnode stack-frame typing → the //rsp-local removal + the noforloop/
stackstring/partialsplit/piecestruct/stackspill STACK arms) is IN PROGRESS — its wave died twice
on API 529 (degraded); substrate preserved, relaunch when the API recovers. Remaining RSP-harvest
seams: ActionReturnSplit/nodeSplit (switchmulti #2-9 structuring) + the render-chain stack-frame
flips. Shelves to re-activate post-render-chain: longdouble-x87, base_explicit-v2, forloop-reroll.


**RSP/SPACEBASE KEYSTONE CRACKED + INTEGRATED (+13 → 397, independently verified).** The
dominant deferred keystone — the deepest, most cross-cutting subsystem in the port (~190
assertions) — solved through EIGHT live-engine corrections (CORRECTION-1..8 in
w10-rsp-keystone-plan.md). The breakthrough: it was NEVER the action pipeline 5 prior attempts
assumed — it was a **dead-code bug**. Three roots, each instrument-proven:
- **ROOT-A:** the cspec `<unaffected>/<killedbycall>/<returnaddress>` effect blocks were NEVER
  parsed — `push_effect` was dead code, effectlist always empty, so `hasEffect(RSP)` returned
  `unknown_effect` not `unaffected`. The decompiler literally didn't know RSP was preserved
  across calls. Fixed in architecture.rs::decode_default_proto → RSP effecttype 4→1 → stack
  slot −0xc not −0x14.
- **ROOT-B:** the `&val` call-arg (PTRSUB(RSP,−0xc) in RDI) wasn't recovered as an active input
  trial — `check_call_double_use` stub + `createPlaceholder`/`opStackLoad` W4 seams + an
  index-based `ActionActiveParam` refactor (take_call_specs emptied qlst).
- **Type-collision:** the 8-byte `&val` shadowed the mapped 4-byte `int4 val` — fixed by
  spacebase-aware `propagate_add_in2_out` (TypePointer::downChain→TypeSpacebase::getSubType) so
  PTRSUB types int4* + a const-sibling decl-skip in printc.
switchind 13→15 (#15 `get_value_byref(&val)` no cast, #16 `switch(val)` slot −0xc — GREEN);
the 13 gained span Concat-split/For-loop/Partial-splitting/Ptr-to-array/Switch-Indirect/Union.
Gate: cargo test --workspace 3659/0, regressed-set EMPTY (strict superset, both-engine diff),
4 render fences re-certified justified-residue, oracle PARITY, B0 unchanged. **REMAINING:**
#8 `default:` = the guard-fold `if(1)`-collapse seam (CORRECTION-8, fold_in_guards ported-inert)
→ switchind 16/16; then L4/L5 + the auto-activating shelves (longdouble-x87/base_explicit-v2/
forloop-reroll) open the rest of the stack-frame cluster.

## Session (2026-06-16d/17) — rust-port W10: keystone grind 307 → 384/675; pivot to dedicated RSP

**enum-truncation (+1 → 384, ACCEPT):** TypeOpSubpiece::propagateType propagates the enum
type through the sub-byte field extract — Enum #3 `ptr->flagfield & (FLAG_20|FLAG_8|FLAG_4)`.
Enum #4 (heritage load-resizing) + Convert #17 (DynamicHash size-sensitivity) are separate
deeper seams. **longdouble-x87 (+0, SHELVED, ACCEPT_WITH_LOSSES):** PART 1 succeeded — the
float10 CALL-arg CONCAT is now built (`writeLongDouble(ldarr,...CONCAT(z,...,x))`, arg present)
+ base_explicit-v2 folded in — but the verdicts don't flip: the float10 stack slot reassembles
from sub-pieces instead of a single addrtied input = **stack-parameter SSA coherence = RSP
heritage/ScopeLocal restructuring**. So the longdouble cluster is RSP-gated. Branch retained as
a shelf, ready for when RSP lands.

**INFLECTION (2026-06-17): the tractable tail is mostly exhausted.** The last several waves each
flip +1 then reveal a DEEPER gate (longdouble->RSP stack-param, enum#4->heritage load-resize,
convert#17->DynamicHash size, retstruct->dynamic-naming, packstruct->CALL-output typing). The
remaining big value (~190 assertions) is concentrated in the **RSP/spacebase keystone**. Pivoting
to a DEDICATED RSP effort: deep investigation of the two concrete leads — (a) the ExtraPopSetup
spacebase varnode resolving to REGISTER-space (offset 32) not the stack slot (RSP A' finding),
(b) the stack-param SSA sub-piece reassembly vs single addrtied input (longdouble-x87 finding) —
likely a shared root in the stack-spacebase/ScopeLocal/MapState restructuring. Housekeeping done:
~82 stale worktrees + 78 merged branches pruned; 3 held shelves + running retained.


**Convert-B1+B2 LANDED (+25 net → 383, the session's biggest single integration,
ACCEPT_WITH_LOSSES):** the real Convert root was upstream of the dynamic hash —
`ActionDefaultParams` (a W4 seam) never copied a known callee's locked FuncProto into the
call site (coreaction.cc:2385), so a CALL-arg constant recovered at full register size 8
instead of the param size 4, diverging the dynamic-hash constant-fold. B1 fixes that
(callee-proto -> call-arg typing) + B2 (copy-elim honoring Varnode::mapped) + the prereq.
Convert 16/17 + **broad collateral from the now-correct CALL-arg typing**: Concat #1/#3/#4,
Concat-split #1, Pointer-to-array #5/#8/#10, Union #10/#11/#29, Bitfields #23 x2,
Partial-splitting #7, Intermediate-pointers #10. Rigorous gate clean (cargo test --workspace
3649/0, regressed-set EMPTY, switch 17/17, B0 untouched, oracle PARITY). Convert #17 (wide-char
L'a' through size-4 truncation) is the lone remaining seam (= the enum-truncation seam).
**call-arg-piece (+13 off 358) overlaps Convert** (same callee-proto root via ActionInputPrototype)
— integrating for its non-redundant Pointer-to-array #1/#2/#3. base_explicit-v2 shelf (f2c3e72)
still gated: float10 is TYPE_FLOAT so RulePieceStructure can't build its CALL-arg CONCAT.

**f0-flag-v2 LANDED (+9 → 358, ACCEPT_WITH_LOSSES):** mark_output_storage_addr_tied lifts the
marker-write un-tie out of the output_locked gate (so output-UNLOCKED transient return registers
un-tie -> render IMPLIED) + a LOSS-206 ScopeLocal-ownership gate so legit recovered locals stay
tied. doublemove #1/#4 now `return glob1+glob1;` (byte-identical oracle) + collateral If/Switch
#3/#5/#6, lzcount #1, mixfloat #2/#6, ModuloAlt #1. partialmerge #3 HOLDS; legit ties (global-RAM,
boolless, readpartial) stay tied. Rigorous gate clean: cargo test --workspace 3646/0, byte-parity
guards 3/3+19/19 (FULL counts — the multi-vN probe ADAPTED readstruct->loopcomment since the un-tie
made readstruct render more oracle-faithful, NO assertion dropped), regressed-set EMPTY. Held
base_explicit (the PIECE/ZEXT pair, now unblocked by the un-tie) left held for a careful re-base.
IN FLIGHT: Convert-B1+B2 (+17). NEXT: base_explicit-v2 (re-implement the PIECE fall-through on the
current base -> broader implied-float PIECE cluster, longdouble subset).


**printc-decl-render LANDED (+1 → 349, ACCEPT_WITH_LOSSES):** all 3 printc seams now render
byte-correct (concat shared `foo v1;` decl-collapse; twodim global-decl skip; checkAddressOfCast
`&c`) — net +1 (Pointer-to-array #7); the other cluster assertions are gated on deeper seams
(struct-PARAM by-value split [LOSS-215], shared-array-read v1-temp, function-arg ptr typing).
Render fidelity improved across 8 stems, regressed-set EMPTY. PATTERN NOTE: render fixes are
flipping fewer datatest assertions than diagnosed because the stringmatch assertions check more
than the one fixed line — the remaining clusters are increasingly fragmented across multiple
seams. IN FLIGHT: f0-flag-v2 (+8). NEXT (dedicated): Convert-B1+B2 (+17, the CALL-arg constant
size-4 fold + copy-elim-mapped, on the 5685b39 prereq).


**Enum render LANDED (+1 → 348, ACCEPT):** PrintC::pushEnumConstant + the TYPE_ENUM dispatch
arm (flag-OR constant render) — Enum Reading #5 (`(FLAG_100000|FLAG_800)`). #3/#4 turned out
gated on a DIFFERENT seam (enum-through-truncation: an 8-byte enum read truncated by sub-byte
SUBPIECE strips the enum type from the AND — TypeOpSubpiece::propagateType, queued); #1/#2
RSP-stack. Freed printc for the decl-render follow-on. IN FLIGHT: f0-flag-v2 (+8), printc-decl-
render (concat 15->18 decl-collapse + twodim/threedim #1/#2 global-mis-scoping/address-of-cast).


**struct-return-v2 LANDED (+11 → 347, the session's biggest single integration):** the
RETURN type-seed re-pin (RulePieceStructure splits register-returned structs on the
post-ActionConstantPtr fixpoint) + linkProtoPartial field-binding (funcdata_varnode.cc:1153 +
establishGroupSymbolOffset variable.cc:623). concat **5/18 → 15/18** (register + edx:eax join
+ zerofield bodies); concatreturn now byte-identical to the oracle (`v1.B=retp2; v1.A=retp1;
return v1;` — root keeps v1, pieces render v1.A/v1.B). Passed the full rigorous gate
(`cargo test --workspace` 3631/0, regressed-set EMPTY, oracle PARITY). This validated the
whole mergepiece→split→field-binding chain, and FREED coreaction_cleanup/coreaction_infertypes/
funcdata_varnode/variable — unblocking f0-flag-v2 (+8) and Convert-B1 (+17). Remaining struct
seams: concat 15→18 (printc declaration-collapse for proto-partial pieces — printc-gated),
retstruct register (`tmp` dynamic-symbol naming), packstructaccess (CALL-output type recovery).


**spacebase-PTRSUB cast LANDED (+3 → 336, ACCEPT_WITH_LOSSES):** ActionSetCasts no longer
wraps the strip-array spacebase-PTRSUB output in a spurious `(int4 *[5])` cast — twodim now
renders `myarray[i][j]`; flips Three-dim #1/#2 + Pointer-to-array. Passed the full rigorous
gate (`cargo test --workspace` 3626/0, regressed-set EMPTY, switch byte-identical, oracle
PARITY). The remaining twodim/threedim #1/#2 + ptrtoarray #7/#10 need a printc decl-render
fix (global mis-scoping `undefined20 myarray[3]` + address-of-cast collapse) — queued
follow-on (gated on the Enum wave freeing printc). Convert turned out gated on CALL-arg
constant SIZE (not the hash, which is faithful) — shelved its prereq (5685b39), queued as a
B1+B2 wave. IN FLIGHT: struct-return-v2 (+20-30), Enum (+3, printc). QUEUED: f0-flag-v2 (+8),
Convert-B1+B2 (+17), printc-decl-render (twodim/threedim #1/#2), Heap-string (+7, W6-gated).


**Recent integrations + two honest walls (rust-port b7a1293 = 333):** float-typeclass
(+4 → 331), mergepiece (prereq, 331 held), **type-seed (+2 → 333)** all integrated clean
(byte-parity 3/3+19/19, switch 6/6 byte-identical, oracle PARITY OK). Two big levers hit
walls that disproved their design premises — valuable failures, each leaving a precise
next-step:
- **RSP Wave A FAILED** (L0+L1 regresses switchind): the porter instrument-proved that L1
  (`setEffectiveExtraPop`) never fires for switchind (`analyze_extra_pop` early-returns on
  known `defaultfp` extrapop). The TRUE keystone is a different `jumptable`-group pass that
  neutralizes the per-call `INT_ADD(RSP,8)` before BRANCHIND emulation — must port it FIRST.
  Plan corrected in `w10-rsp-keystone-plan.md`; faithful L0+L1 transcription saved as a WIP
  patch.
- **struct-return REVERTED** (+1 on its branch, but 2 of its own tests fail on the integrated
  tree): the CONCAT→PIECE split fired off `ed09d28` but **type-seed's type-flow changes break
  it** on the post-type-seed tree (a real cross-wave interaction). The +20-30 cluster also needs
  `linkProtoPartial` (`funcdata_varnode.cc:1153` + `establishGroupSymbolOffset`) to bind split
  pieces to the root struct symbol so printc renders `v1.A`/`v1.B`. Re-land rebased on type-seed
  + the field-binding finisher.
- **f0-flag-untie** (register f0 over-marked `addrtied|mapped`; clean standalone +2 doublemove)
  IN FLIGHT.
Held branches (each paired to a pending unlock, files untouched in main so non-stale):
forloop-reroll, base_explicit, struct-return.

**RSP DEFERRED (CORRECTION-3, `1596ad6`) — the decisive meta-lesson of the session.** Wave A′
landed L0 (ActionExtraPopSetup un-stub, faithful) and the datatest count rose to **340 (+7)** —
but the rigorous finalize gate proved that was MASKING: the +7 came from concatsplit/forloop
(unrelated), while `cargo test --workspace` caught **8 real fence regressions** (switchind STILL
13/16 — `switch(0x100058)`; nanops spurious param; forloop lost 64-bit lift) that the loose
datatest stringmatch oracle hides. Root cause: the inserted `INT_ADD` targets the **register-space
spacebase (offset 32), not the stack slot**, so RuleAddMultCollapse can't fold it; L0 is genuinely
COUPLED (needs propagateSpacebaseRef-consume + spacebase-deadcode atomically, exactly as the
original deferred-stub comment predicted). RSP is now thrice-disproven and deferred to a dedicated
multi-wave effort (lead: the register-vs-stack spacebase bug). **TWO META-LESSONS (logged):
(1) read-only design passes repeatedly get the MECHANISM wrong — the port attempt with the live
engine is the real test; (2) the datatest count alone masks regressions — every gate MUST run
`cargo test --workspace` (the fence tests). Pivoted to the tractable clusters.** IN FLIGHT:
struct-return-v2 (+20-30, split re-pin + linkProtoPartial), spacebase-PTRSUB cast (+6-8, the
type-seed cast follow-on). QUEUED: f0-flag-v2 (+8, coreaction_cleanup un-tie + LOSS-206 gate).



Five verified waves carried the corpus from 307 to 327 passing (now **6.8× the M2 entry
of 48**), all under the standard gate (cargo test --workspace green, clippy -D, byte-parity
3/3 + 19/19, C++ oracle 207/207 + 675/675 PARITY OK byte-untouched, B0 unchanged, monotonic
passing):

- **global-persist2 (+7)** — `Scope::add_range` global-RAM seeding + `GlobalQuery.owned`
  ram persist, co-landing the previously-held `revisit-ssa` guard-calls heritage pass
  (the earlier proto-E/revisit `heritage.rs` conflict resolved here).
- **bitfield tail / ScoreUnionFields** intermediate waves — union-field scoring +
  bitfield pull/insert render arms.
- **proto-unlock (keystone #2, substrate +1)** — Stage A `mark_output_storage_addr_tied`
  AL sub-register **un-tie** (restores Promotion-on-compare #2, Return-Structure #3);
  Stage B `decode_join_addr` + `processJoins` building the **x86win/x86gcc prototype
  model** from the cspec `<addr space="join" piece1=.. piece2=../>` pentry (faithful per
  space.cc:539 JoinSpace::decodeAttributes). The model now **builds** — the full
  Return-Structure/mixfloat/longdouble render is gated on the float-typeclass +
  SplitDatatype struct-split on top of it. HEAD **a4bcd0f**.
- **float-typeclass (+4 → 331, `258b015`, ACCEPT_WITH_LOSSES)** — the proto-model
  payoff: float pentry typeclass separation + `hidden_return` ModelRule + float10
  stack model. Flips **Return-Value-Input-Register** (`returnbig` struct-return renders
  byte-exact). mixfloat's float8 *render* is diagnosed as a downstream **merge/type-
  inference** artifact (XMM0 in/out overlap), NOT an fspec gap (fspec typeclass proven
  correct — XMM1 `a2` already types `float8`); long-double struct-split stays RSP-gated.

**Two probe waves (+0, cheap intelligence) + two design specs:** the arrays probe proved
AddTreeState is already correct — nested-array failures are a **type-seed** gap (global
const `0x601060` never typed `&myarray:int4[3][5]`), specced as `w10-typeseed-constptr`
(un-stub `ActionConstantPtr` + `Funcdata::spacebase_constant` + `architecture`
`cache_addr_space_properties`/`infer_ptr_spaces`; ~+4–8; must run after the funcdata/
database files free up). The double-precision probe proved RuleDouble* IR is byte-identical
— the gap is `ActionMarkExplicit::base_explicit` marking wide-float PIECE temps explicit;
specced as INDEPENDENT (owns only `coreaction_cleanup.rs:470-588` + a `find_root` helper;
~+4–10), launching now.

**mergepiece-dynsym integrated (`ed09d28`, ACCEPT_WITH_LOSSES, +0-net but PREREQUISITE,
331 held monotonic):** MergeContext piece-read-back + ActionDynamicSymbols dynamic-hash;
removes spurious `glob1.a = glob1.a` self-assigns (partial_restore `return glob1.a + 7;`
now byte-exact via the *real* piece-suppression collapse). **KEY REFRAME (design pass):**
SplitDatatype is *already fully ported* (subflow.rs split_copy/build_out_concats/
get_value_datatype + RuleSplit* live, ACCEPTed in w10-splitdatatype.md) — so the
**retstruct/concat/concatsplit/packstructaccess cluster (~+20–30)** was gated precisely on
mergepiece's piece-read-back *collapse*, NOT on a SplitDatatype stub and NOT on RSP (the
register EAX:EDX / PIECE returns are join, not stack). With mergepiece now integrated, the
**struct-return finish wave** (`subflow/merge/printc`, verify-and-finish the CONCAT→per-field
collapse) is the single biggest remaining lever — launched off `ed09d28`. Three disjoint
productive fronts now run: base_explicit (`coreaction_cleanup`), type-seed
(`coreaction_render/funcdata/architecture`), struct-return (`subflow/merge/printc`).

KEYSTONE MAP (the remaining ~348 gate on ~5 named keystones): **#1 RSP/spacebase**
(ExtraPopSetup + propagateSpacebaseRef per-CALL dead-code — deep, paused, gates ~half the
remainder); **#2 proto model** (substrate landed; float-typeclass + struct-split are the
payoff, IN FLIGHT as `w10-float-typeclass` for mixfloat+longdouble); **#4 mergepiece-dynsym**
(MergeContext piece-read-back + ActionDynamicSymbols dyn-hash naming — IN FLIGHT);
**#5 SplitDatatype struct-typing** (struct-return CONCAT split, partly RSP-gated).

## Session (2026-06-16c) — rust-port W10: for-loop reroll merged (INERT), 307/675 held

Merged the held `rport/w10-forloop-reroll` (while->for conversion: `emit_for_loop` +
`finalize_forloop_transform`/`finalize_forloop_printing` + `funcdata_block`/`funcdata_op`)
onto the RSP-spacebase-rendered tree (`f621eee`).  The merge is BYTE-SAFE and INERT —
**307/675 held, ZERO assertions lost/gained, switch cluster IDENTICAL, boolless/readstruct/
condconst_conn byte-identical to the C++ oracle, B0 unchanged, 3,491 Rust tests green,
production-lib clippy -D clean, C++ oracle 207/207 + 675/675 PARITY OK.**

DIAGNOSIS (corrects the stage premise): the for-loop reroll does NOT activate, because the
for-loop datatests are NOT RSP-clean.  The spacebase-typing keystone (13ed4c3) cleaned the
*additive* `RSP + const` form (ptrtoarray's first function), but the for-loop files carry a
DIFFERENT, un-eliminated RSP pattern — per-CALL dead return-address stores
(`*(xunknown1 *)&v3[0xff..f8] = <retaddr>;`) + the `v3 = &v3[...]` stack-ptr adjust +
`undefined8 v2/v3; // rsp` locals.  This is the work of the **DEFERRED**
`ActionExtraPopSetup::apply` (coreaction_protos.rs:470, deferred at 10fc2ab): its per-call
INT_ADD/INDIRECT spacebase op is only net-safe once `propagateSpacebaseRef` + the
spacebase-store dead-code land WITH it.  Enabling it ALONE regresses jump-table index
recovery (`switchind` — committed `verify_w10_jts_chain` tests), which the stage's hard
constraint forbids.  So the reroll's `has_overflow_syntax()` guard correctly still declines.
Proven via the faithful `decomp_test_dbg`+`KUNA_DUMP=1` harness; forloop1 C++ oracle target
`for (v1 = 0; v1 < max; v1 = v1 + 1) { sub_400430(0x400820); }` vs Rust `while( true ) { if
(max <= v1) break; ...dead rsp chain...; v1 = v1 + 1; }`.  NEXT KEYSTONE: co-land
ActionExtraPopSetup + propagateSpacebaseRef per-CALL spacebase dead-code (a type-plane wave,
not a render stage) — the reroll then auto-activates the whole For-loop cluster.  Added 5
adversarial verifier tests (`verify_w10_spacebase_render.rs`) pinning the inert-merge state +
switch-no-regression + protected-file byte-parity, written to flip green on activation.

## Session (2026-06-16b) — rust-port W10 → 307/675

Waves since 290: MIPS dead-CALLOTHER (+3), union value-members (+5, HighVariable
union-type retention), RSP/spacebase substrate (propagateSpacebaseRef + spacebase
deadcode — 8 files transiently clean; the ExtraPop count-mover deferred, jumptable
conflict), **pushPartialSymbol (+9: symbol-mapped struct/union members glob.intfield /
val.mydouble / val.u.myint 2-deep walk — high-risk broad render landed monotonic).**
rust-port 77ced72 = **675 applied / 307 passing / 0 exec-failures.** 3,5xx Rust tests
green; clippy -D clean; boolless/readstruct/condconst_conn byte-identical; B0 unchanged;
**C++ oracle 207/207 + 675/675 PARITY OK, byte-untouched.** RSP CHAIN ROOT proven (4
diagnoses): the RSP input varnode must type TYPE_PTR->TYPE_SPACEBASE so
propagateSpacebaseRef fires (cleans the 15 dirty-RSP files; the HELD for-loop reroll then
auto-activates). IN FLIGHT: spacebase-input typing (the RSP keystone), union-truncation
(resolveTruncation: val.c/globvar.b.bval1).

## Session (2026-06-16) — rust-port W10 → 290/675, ZERO exec-failures (full corpus applies)

The whole datatest corpus now decodes end-to-end under the Rust engine. Waves since 242:
SBORROW signed-compare normalization (+3), **float cluster +21** (fillinReadOnly
readonly-constant fold + float-lane getNameRepresentative -> all 14 Float-print + casts +
long-double), union cluster +5 (ScoreUnionFields::run + printc .field arm), **implied-var
/ dominant-COPY collapse +18** (broad: floatcast tail + bitfields + union + deindirect),
**indproto** (getTypeCode(PrototypePieces) nested function-pointer parse -> the LAST
exec-failure closed). **rust-port d852f06 = 675 applied / 290 passing / 0 exec-failures.**
3,5xx Rust tests green; clippy -D clean; boolless/readstruct/condconst_conn byte-identical;
B0 list-action unchanged; **C++ oracle 207/207 + 675/675 PARITY OK, byte-untouched.**
IN FLIGHT: union value-members (~30; HighVariable union-type retention + pushPartialSymbol),
MIPS dead-CALLOTHER elimination. Remaining: bitfield tail, longdouble, for-loops, the
smaller held branches.

## Session (2026-06-15d) — rust-port W10 → 242/672, only 1 exec-failure left

Continued the verified-wave grind. Bitfield subsystem (2 waves: BitFieldPull/Insert +
6 rules + TypeStruct::findMatchingBitField + ZPULL/SPULL/INSERT + expression + printc
pushBitfield, then RuleInsertAbsorb/PullAbsorb) -> Bitfields 18/62 (LE `increment`
byte-identical). **Symbol/global-naming consolidation (+20):** names mapped globals
(dat_->globalfree/glob1), folds in the held global-persist branch, flips Conditional
Constant #6-9 + Conditional Multi #1-4 + Display Format (displayformat.xml now FULLY
byte-identical 4/4, drops out of exec-failures) + Global cross. **rust-port b5277eb =
672 applied / 242 passing / 1 exec-failure (only indproto).** 3,512 Rust tests green;
clippy -D clean; boolless/readstruct/condconst_conn byte-identical; B0 list-action
unchanged; **C++ oracle 207/207 + 675/675 PARITY OK, byte-untouched.** IN FLIGHT: float
cluster (~48; fillinReadOnly readonly-constant fold + float-lane getNameRepresentative —
precisely mapped, casts already byte-exact), SBORROW signed-compare normalization.

## Session (2026-06-15c) — rust-port W10 type-plane chain + M4 reached → 204/668

Drove the type-plane keystone CHAIN to completion via orchestrated waves (all Opus,
worktrees, porter→independent-verifier→repair, single-integrator additive merges). Each
keystone was +0 in isolation but unblocked the next; the verifier protocol gated
*diagnoses* as well as fixes (three "failed" waves on one regression each produced a
sharper proven root-cause instead of a fabricated flip).

- **ActionSetCasts cast plane** (active) → **TransformManager::apply materialization
  keystone (+6:** broad SUB(x,0)/ZEXT collapse via live SubvariableFlow/Split/Subfloat —
  compare/conditional/inlining/status; inline.xml 3→6) → **LaneDivide** (XMM lane noise
  retired) → **printc cast-notation render** ((float8)a byte-exact, +1) →
  **cast-strategy** (+6: Concat/Intermediate-ptr/Long-double extension-cast routing) →
  **HighVariable naming** (register/local vN coalescing; Dead-Volatile/Signed-byte/
  No-for-loop) → **harness LOSS-007 regex faithfulness** (\< \> identity-escapes).
- **Promotion #2 saga (3 waves, all honest):** merge_indirect FAILED + re-diagnosed
  (float blocker = global-persist, not lane-coalescing); char* FAILED + re-diagnosed
  (cause = action-pipeline seam, not types); seam-abort wave found the real fix (a
  RuleTrivialArith name-vs-group clone-gate) — restored Promotion #2, naming branch clean
  (+4 / 0 regression). Integrated at rust-port 8e0cc4a.
- **M4 REACHED:** W11 object-crate ELF loader (`kuna.decompile --engine rust <real.elf>`
  decompiles a REAL ELF; `main` byte-identical to cpp) + W11 pipeline binding
  (`KUNA_ENGINE=rust` flows through the compare/sweep/rank harness — real kuna-rust-vs-cpp
  comparison row).

**State (rust-port 8e0cc4a): 204/668 passing, 2 exec-failures** (displayformat decodes
now via global-persist held branch; indproto only true ERROR). M1 ✓ M2 ✓ **M4 ✓** M3 in
progress. 3,453 Rust tests green; clippy -D clean; boolless/readstruct/condconst_conn
byte-identical; **C++ oracle 207/207 + 675/675 PARITY OK, byte-untouched.** IN FLIGHT:
bitfield full subsystem (~62), symbol/global-naming consolidation (displayformat +
condconst-globals + float, folds in global-persist).

## Session (2026-06-15b) — rust-port W10 second fleet wave → 182/668 datatest parity

Continued the parallel-fleet method (Workflow tool, all Opus 4.8, isolated worktrees,
porter→independent-verifier→repair, single-integrator additive-union merge). After the
first integration (59/549, rust-port cd44e73), ran two more fleets + a switch-chain:

- **Switch-chain** (`rport/w10-jts-chain`, ACCEPT_W_LOSSES): JumpBasic index-range model
  (emulation-driven over the W10 EmulateFunction foundation) + cloneOp/truncatedFlow +
  stageJumpTable + install → ActionSwitchNorm + BlockSwitch + emitBlockSwitch. The 6
  switch files now decode+render; **switchind's switch/case matches the C++ B5 oracle
  exactly** (real recovered case labels). +14 passing in-branch.
- **Depth fleet** (4 waves): **div/mod +99** (RuleDivOpt reciprocal-multiply recon +
  RuleSelectCse + a RuleDoubleShift group-filter bug — all 68 div/mod assertions pass);
  **console-family** (8/10 LOSS-119 exec-failures closed, +71 applied/+7 passing);
  **concat-piece** (RulePieceStructure ported faithfully, +0 — W4/W8 seams);
  **typed-access +0 with the key M3 diagnosis**: the ~150-assertion typed-access cluster
  (union/bitfield/float-cast) is ALL blocked on `ActionSetCasts::apply` being a no-op
  stub (coreaction_cleanup.rs) — the W8 cast-insertion keystone. Every typed access
  renders as raw p-code because no CAST/PTRSUB ops are inserted. Correctly returned ZERO
  code + a precise keystone diagnosis (LOSS-166) instead of faking passes.
- Two deep clusters' porters failed honestly (string char-array folding; typed-access
  first pass) — diagnosed as depth, not transient, and quarantined for dedicated waves.

**Integrated jump (rust-port b2c49b4):** exec-failure files **16 → 2** (only
displayformat + indproto), assertions APPLIED **549 → 668** (+119), assertions PASSING
**59 → 182** (+123 — nearly 4× the session-start 48). 3,375 Rust tests green; clippy -D
clean; boolless/readstruct/condconst_conn still byte-identical; **C++ oracle 207/207 +
675/675 PARITY OK, byte-untouched.**

**Follow-on waves (rust-port e7028b2, 668/187/2):** input-param recovery (register→`a0`,
landed structurally) + ActionSetCasts union-cache (prerequisites, +0) integrated at
37c2450; inline body clone (**+5**, 182→187) + displayformat infra integrated at e7028b2.
The casts+param waves jointly DIAGNOSED the M3 endgame keystone: the ~150-assertion
typed-access cluster + a0-typing + the SUB/CONCAT corpus noise are all gated by the **S6
HighVariable Merge engine** being seamed (ActionMergeType/Required/Adjacent + merge.rs
copyShadow/block-surgery + the HighVariable type-facing graph). Keystone wave
`w10-merge-casts` (2-stage: Merge engine → ActionSetCasts) IN FLIGHT. Sequencing: hold
the type-dependent clusters (union/bitfield/float/string/indproto) until the keystone
lands — they share its files and would collide; fan them out as a fleet immediately after.

**Next M3 levers:** (1) `ActionSetCasts` W8 cast-insertion keystone — the single biggest
unlock (~150 typed-access assertions: union 33, bitfields 31+31, float/cast ~48);
(2) input-parameter recovery (register-input → named `a0`; unblocks ModuloAlt + Return
Value Input Register + float bodies); (3) the last 2 exec-failures (displayformat
buildDynamicSymbol, indproto getTypeCode-with-prototype); (4) string char-array folding.

## Session (2026-06-15) — rust-port W10 parallel un-seam fleets → 59/549 datatest parity

Pivoted from serial single-lever un-seams to **orchestrated parallel fleets** (Workflow
tool, all agents Opus 4.8, isolated worktrees, porter→independent-verifier→repair). Two
big fleets + a jump-table foundation landed this session:

- **JumpTable foundation** (`rport/w10-jumptable-switch`, ACCEPT_WITH_LOSSES, merged
  03c4cef): ported `EmulateFunction` (the syntax-tree value-flow emulator) + read-only
  load-image-value + op-behavior plumbing — the keystone the JumpBasic index-range model
  drives through. +5 tests; count unchanged (the end-to-end switch recovery chain is 6
  subsystems deep — model/cloneOp/stageJumpTable/switchOver/BlockSwitch/emit — now in
  flight as `rport/w10-jts-chain`).
- **6-class exec-failure fleet** (all 6 ACCEPT/ACCEPT_WITH_LOSSES): float-family,
  inline-inject, partial-types, rel-pointer, struct-return, const-prop-phi. Integrated
  cleanly (all-additive shared-file unions) at **rust-port cd44e73**.

**Integrated jump (the biggest single step so far):** exec-failure files **28 → 16**,
assertions APPLIED **456 → 549** (+93), assertions PASSING **48 → 59** (+11). 3,336 Rust
tests green; clippy -D warnings clean; boolless/readstruct/condconst_conn still
byte-identical; **C++ oracle 207/207 + 675/675 PARITY OK, byte-untouched.** The honest
−1 (struct-return retspecial) is more-correct-but-count-down: faithful hidden-return
recovery loses a coincidental degraded `return;` match (LOSS-131 signature, accepted).

**M3 map (from the 490 applied-but-failing assertions, histogrammed):** the gap is now
per-feature seams, not the keystone (28 feature groups already byte-match). Biggest
independent levers: div/mod strength-reduction (~99), typed-access (~150: union 33,
bitfields 31+31, float/cast ~48), concat/piece (~29), + the 10 non-switch exec-failures
(console command family / LOSS-119). **In flight:** `wi287re9j` (switch-chain) +
`wyi68lmpv` (4-wave depth fleet: divmod / typed-access / concat-piece / console-family).



Addressed the w10-stackvar-promotion REJECT (review F0-F7). Corrected the overstated
"whole chain complete" claim and made the chain faithful + tested:

- **F1 (chain link 3 was a stub):** ported `ActionStackPtrFlow` in full — new
  `coreaction_stackptr.rs` realizes `checkClog`/`repair`/`adjustLoad`/`isStackRelative`
  (the LOAD->COPY clog fold) and the `StackSolver` + `analyzeExtraPop` linear solve
  (INT_ADD rewrite of the solved stack-pointer producers). The action's `apply` now
  runs `check_clog` then `analyzeExtraPop` per `coreaction.cc:496-512` (was a no-op
  seam). The only gated pieces are the call-spec `setEffectiveExtraPop` write and the
  IOP->FuncCallSpecs lookup, which bind to the W6/W7 proto-recovery surface the merged
  tree does not build (`numCalls()==0`) — exactly the C++ `fc==0` guess branch; the
  in-scope INT_ADD/clog rewrites run regardless. (LOSS: extra-pop propagation into call
  specs.)
- **F2 (no positive tests + false comment):** added 8 positive unit tests — the clog
  LOAD->COPY fold, `is_stack_relative` recognize/reject, `StackSolver` add-chain solve
  and indirect-guess solve, and the `Funcdata::spacebase` mark (input SP -> spacebase;
  free varnode skipped). Removed the false "exercised end-to-end by the datatest corpus"
  comment on `loadvarnode_seam_noop`.
- **F3/F4 (PIECE/SUBPIECE faithfulness):** `MapState::isReadActive` now filters the
  same-storage PIECE slot (was `PIECE => true`); `gatherVarnodes` gained the explicit
  PIECE (two-COPY per-slot hints) and SUBPIECE (same-storage-truncation filter) cases.
- **F7 (SET-addrtied invariant):** restored the verbatim C++ `syncVarnodesWithSymbol`
  mask (addrtied/addrforce CLEAR-but-never-SET); the kuna pre-tie of address-tied stack
  storage is now a SEPARATE documented step (mirroring `setSymbolEntry`) before the mask.

Result: the stack location still promotes to a named, typed local (`int4 i [4]; //
stack - 0x18` with `i[1]` access in noforloop_alias; `loopvar`/`pchar` in the others) —
genuine spacebase promotion, no special-casing. Datatest positive (min>=1) full-
assertion passes hold at 24/425 (6 positive); flipping the whole multi-statement
assertions needs the out-of-scope proto-recovery + for-loop-structuring items (the
lingering `RSP = RSP + ...`, missing params, and `SUB(0xffffffff,0)`/`BOOL_NEGATE`
noise are those seams, not stack promotion).

**State: 3,197 Rust tests green (+8 new positive chain tests); clippy -D warnings clean;
C++ oracle 207/207 + 675/675 PARITY OK, untouched.**

## Session (2026-06-14) — rust-port W10 un-seam: stack-var promotion chain (named locals)

Closed the whole stack-variable promotion chain (1)->(5), all unblocked by the
SpacebaseSpace keystone: `ActionSpacebase`->`Funcdata::spacebase` marks the RSP input
spacebase + types it as a pointer; `RuleLoad/StoreVarnode::checkSpacebase`
(correctSpacebase/vnSpacebase + `getSpaceBySpacebase`/`getSpaceFromConst`) folds
`LOAD/STORE(RSP+off)` into a `(stack,off)` COPY; `ScopeLocal::restructureVarnode`
gathers over the LIVE IR (gatherVarnodes/gatherOpen with the AliasChecker
gatherAdditiveBase/gatherOffset seam realized + gatherSymbols) and restructures into
Symbols; `syncVarnodesWithSymbols`/`syncVarnodesWithSymbol` paint mapped|addrtied + the
recovered type; and the naming pass + printer render the mapped Symbol name (incl.
array-member `name[idx]` access and `int4 i [4]; // stack - 0x18` array declarations).
Also closed the console IR-rebuild gap (`decompile` rebuilds the Funcdata, dropping the
`map addr` symbols; now carried across via `mapped_symbol_specs`/`seed_mapped_symbols`).

Result: stack locals that were raw `STORE/LOAD(RSP+off)` / `Stackffffffff...` unnamed
locations now promote to NAMED locals across functions — `loopvar`/`count`
(forloop_loaditer), `pchar` (pointercmp), `int4 i [4]; // stack - 0x18` with `i[1]`
array access (noforloop_alias) — byte-matching the C++ B5 oracle declarations. Datatest
positive (min>=1) full-assertion passes hold at 24/425 (no regression); the remaining
gap to flipping those whole multi-statement assertions is the downstream for-loop
structuring / CALL-arg rendering / raw-stack-ptr alias annotation seams.

**State: 3,186 Rust tests green; clippy -D warnings clean; C++ oracle 675/675 PARITY OK,
untouched. Stack-var promotion chain functionally complete (named locals render).**

## Session (2026-06-14) — rust-port W10 un-seam chain cont.: stack-frame keystone, 24/425

Continued the horizontal parity grind (each wave: measure vs stage golden, un-seam ONE
layer faithfully, independent verify rejecting special-casing, all tests green, C++
oracle untouched at 675/675). Un-seams this stretch: refinement+loops (heritage
placeMultiequals refinement + loop emitters; +7 files reach printer), pspec-context
(x86:64 was decoding as 16-bit real mode — a correctness bug; now 64-bit, generic
cspec context_data parse), ScopeLocal (LOSS-109 partial restoration; getScopeLocal
8->0 errors), and the SpacebaseSpace KEYSTONE (LOSS-012: the stack space was never
created; cspec <stackpointer> decode + Architecture::add_spacebase into the single
manager + Database::adjust_caches). Result: datatest assertions applied 357->425,
passing 20->24, with +3 genuine min=1 positive matches (Global cross, Pointer to
array, Return Value Input Register).

**State: the port is COMPLETE + FUNCTIONALLY PROVEN (boolless byte-identical to
Ghidra). 3,186 Rust tests green; C++ oracle 675/675 PARITY OK, untouched. Integrated
datatest parity: 24/425 applied (most of 83 files now decompile).**

HONEST REMAINING-SCOPE ASSESSMENT: reaching M3 (675/675) is a substantial multi-
session tail. The un-seam chain keeps revealing that each "lever" is a CHAIN of
seams: e.g. stack-var promotion = SpacebaseSpace (done) -> ActionSpacebase ->
heritage spacebase-pointer recovery (spacebaseConstant) -> ActionStackPtrFlow ->
ScopeLocal::restructureVarnode gather -> syncVarnodesWithSymbols. Several such chains
remain (stack-var promotion, switch/JumpTable recovery, union types, per-datatest
body convergence). The methodology is proven and the blockers are all enumerated
(LOSS-131/135/137 family + the named seams_remaining in each W10 review) — which is
the well-characterized state suited to either continued orchestrated waves or the
autonomous pipeline (built, dry-run-proven; tools/pipeline PIPELINE_MODE=port).

NEXT (highest-leverage): finish the stack-var promotion chain (ActionSpacebase ->
spacebaseConstant -> ActionStackPtrFlow -> restructureVarnode) — unblocks named
locals across many functions; then JumpTable recovery (switches); then per-function
convergence. ~150 checklist port-items closed of 202 + the W10 analysis-unseam set.


## Session (2026-06-13 cont.) — rust-port W10: boolless BYTE-IDENTICAL to the C++ oracle

The vertical slice is COMPLETE. After two more un-seams (merge/naming/output-storage,
then ActionInferTypes type-lattice), boolless print C is now BYTE-FOR-BYTE identical
to the C++ Ghidra oracle:

    uint1 boolless(void)
    { uint1 v1; // acc
      v1 = dat_52; if (dat_52 <= 10) { v1 = 1; } return v1; }

This is the first FULLY-byte-parity function — the entire pipeline proven end-to-end:
lift -> flow -> heritage(SSA) -> deadcode -> simplify -> proto-recovery -> merge
(HighVariables) -> naming -> block-structuring -> type-inference -> printc -> C
identical to Ghidra. Every pass faithfully ported + integrated; the uint1 came from
real type propagation (verified non-hardcoded). The infertypes wave closed a THIRD
glb-reach seam (TypeFactory wasn't shared into glb — same shape as LOSS-132's dual
manager + the OpBehavior-table bug). 3,170 Rust tests green; C++ oracle 675/675
PARITY OK untouched.

The port is now FUNCTIONALLY PROVEN (not just structurally complete). Strategy shifts
from vertical (one function's full path) to HORIZONTAL expansion. Highest-leverage
next blocker (identified): heritage placeMultiequals refinement + loop-structuring
gates ~42 datatest files from reaching the printer at all (divopt etc. emit the
'structuring declined' shell); plus a loader 'Bytes not mapped' gap on several x86-64
tests. The type lattice + all passes are in place, so closing the structuring/loop
blocker should light up many functions' assertions at once. Then triage each function
class against its stage golden toward M3 (675/675).


## Session (2026-06-13 cont.) — rust-port W10 un-seam chain: first real datatest parity

Continuing the analysis-body un-seam grind (LOSS-131), each wave gated on the W0
stage-boundary goldens, faithful + general (verifier rejects special-casing), all
tests green, C++ oracle untouched (675/675). Progress this session:
- heritage: ported the DRIVER (W5 had only substrate); B3 oracle-exact. Closed the
  KEYSTONE LOSS-132 (dual AddrSpaceManager -> single Rc<AddrSpaceManager>): heritage
  now reaches the real lifted varnodes (boolless 37/37).
- deadcode + simplify: boolless 25->9 ops; closed a 2nd dual-Architecture bug (the
  OpBehavior table wasn't shared into glb); ActionDeadCode + constant folding real.
- proto-recovery: real fspec::FuncProto on Funcdata + active-trial RETURN recovery;
  boolless 9->7 ops, RETURN(#0x0,ACC) EXACTLY the C++ B4 -> boolless IR == oracle.
- structuring + printc-body: ActionBlockStructure seeds sblocks + the printc
  statement-body driver emits REAL structured C (if/body/return); generalizes across
  6+ functions. First time the Rust engine emits structured C, not a shell.
- node-join + present-compare: closed ConditionalJoin::execute; confirmed boolless's
  'dat_52 <= 10' comes from real IR INT_LESSEQUAL (ActionPresentCompareForm, GH-558).

**MILESTONE: the first REAL positive datatest assertion passes under --engine rust**
('Boolean thru Less-than #1' = 'if (dat_52 <= 10)'), verified non-vacuous (from real
IR, not a printer hack). boolless now prints `void boolless(void){ if (dat_52 <= 10)
{ ACC = 1; } return ACC; }` vs oracle `uint1 boolless(void){ uint1 v1; v1 = dat_52;
if (dat_52 <= 10) { v1 = 1; } return v1; }`. Remaining boolless gap: merge/naming
(ACC->v1) + output-type (void->uint1). 3,166 Rust tests green.

Each un-seam keeps exposing+fixing latent integration bugs the per-item gates missed
(dual manager, unshared OpBehavior table, condexe loop bound) — only the live
lift->analyze->print path reveals them. Methodology proven: measure vs stage golden,
un-seam one layer faithfully, name the next blocker. Path to M3: merge/naming +
types -> boolless full byte-parity -> horizontal expansion across 83 files.


## Session (2026-06-13) — rust-port W10: keystone closed (single AddrSpaceManager); heritage oracle-exact

The W10 analysis-body grind began and immediately found + closed THE keystone
architectural blocker. The vertical-slice attempt (one agent, boolless full match)
correctly FAILED — too interconnected for one task — so it was decomposed per-pass.
The first, ActionHeritage, revealed that W5 had ported only the heritage SUBSTRATE
(the driver was unimplemented_seam); that agent ported the full driver faithfully,
proved correct SSA on a diamond CFG, and pinned the real root cause: LOSS-132, the
DUAL AddrSpaceManager — lifted varnodes live in the engine's manager while
Funcdata.glb carried a stripped ir-boundary manager, so every analysis pass reached
0/N varnodes (the cause of the whole-pipeline no-op).

A focused keystone agent UNIFIED the managers to the C++ single-manager model
(Rc<AddrSpaceManager>; engine owns it, iop/fspec inserted into it, shared as glb).
Result: **heritage now reaches the real varnodes — boolless B3 post-heritage IR is
oracle-exact (37/37 varnodes, phi placement exact, ZERO rust-only identities);
lzcount 14/14, stackspill 20/20.** The unblock also exposed+fixed real downstream
bugs (a latent || short-circuit panic, heritage normalizeWriteSize). 3,153 Rust
tests green; C++ oracle untouched (675/675 PARITY OK).

VALIDATED METHODOLOGY: per-un-seam progress is measured by the W0 stage-boundary
goldens (B2/B3/B4/B5 vs the C++ IR at each stage), NOT the datatest count — which is
a LAGGING metric that only rises when a full vertical slice closes (it moved 21->18
as functions reached deeper-real IR and hit the next seam). The remaining M3 path is
the dependency-ordered un-seam chain, each gated on its stage boundary: heritage
(B3) DONE; next ActionDeadCode+simplify, then merge+types, then ActionBlockStructure
(B4 structured blocks), then printc (B5) -> datatests pass; then horizontally expand
across the 83 files looping to PARITY OK (M3). Multi-session by nature.

Cumulative: ~150/207 checklist items; the engine is complete + running + now
operating on REAL SSA. Next: continue the un-seam chain (deadcode/simplify) or hand
to the autonomous pipeline with this validated per-stage-goldens gating.


## Session (2026-06-13) — rust-port W9 + M2: the Rust decompiler RUNS end-to-end

W9 console ports (interface, ifacedecomp 4.4k command set, grammar, kuna stage
commands, codedata, datatest runner; rulecompile/unify deferred — zero oracle
exposure) all verified, then the M2 integration: the engine-glue the modular port
deferred — Architecture now owns + exposes TypeFactory/PrintC/LoadImage/Context/
proto-models, impls ArchOptionContext (the `option` command mutates the real arch),
runs the init/spec pipeline, and a decompile_drive (decompile_func -> universalAction
perform -> print_c). The decomp_dbg/decomp_test_dbg bins are wired to it.

**M2-RUN ACHIEVED: `KUNA_ENGINE=rust run_tests --datatests` executes end-to-end** —
the Rust decomp_test_dbg runs the corpus, run_tests.py parses it cleanly. Engine glue
proven: 5/5 architectures emit structurally-sane C; 3/675 assertions PASS baseline
(7/83 files complete). The C++ oracle stays 675/675 PARITY OK (untouched).

Honest gap to M3 (LOSS-130): the dominant blocker is the PrintC BODY emitter (the
op-emitter methods exist but aren't driven by a block-graph statement walk — only the
signature shell emits), then parse_C grammar store-writes (44 files' setup) +
parse_machaddr/Scope symbol mutation (27 files), then the C-text grind. Gate: 3,119
Rust tests, 0 failures. Cumulative: 143/202 items. Next: W10 parity grind — PrintC
body + C-decl/symbol setup, then loop-until-PARITY-OK.


## Session (2026-06-13) — rust-port W8: print stack + the full pipeline (B0 byte-equal)

All 7 W8 items + the allowlist closure verified. The Rust engine is now a COMPLETE
decompiler: SLEIGH lift -> flow -> SSA -> the 252-pass simplification/recovery/
structuring pipeline -> C output. Delivered: printlanguage + CastStrategy, the
3.9k-LOC printc C emitter, prettyprint's line-breaking engine (validated byte-for-
byte against the real Ghidra emit engine), string/naming/arraynotation, the S9 render
actions, the signature/paramid pack, and the KEYSTONE universalAction assembly.

**B0 keystone: the materialized 'decompile' action tree is BYTE-EQUAL to the C++
decomp_dbg `list action` dump.** The B0 gate caught 10 scattered passes that slipped
wave batching (7 subflow SplitDatatype rules, 2 constseq string rules,
RuleStackProbeLoop — which I'd omitted from the W7 file list — and
ActionLowerSwitchInstall); the closure ported all 10, drove UNPORTED_ALLOWLIST to
EMPTY, and re-pinned the 3 secondary verifier tests to the oracle (firstpass +
all-groups dumps matched first try: 252 leaves + 7 headers). Pass STRUCTURE +
registration order are now complete and order-exact; several pass BODIES still
decline at W4/W6 seams (SplitDatatype mutation, string-copy deep bodies, lowered-
switch CFG surgery) — the M2/M3 grind exercises them against real datatests.

Gate: workspace 2,885 tests, 0 failures, 35 ignored; lib clippy clean (LOSS-120
tracks test-file --all-targets debt); oracle untouched. Cumulative: 128/202 items.
Next: W9 — console + ifacedecomp + grammar + the datatest runner -> M2 (the 83
datatests running end-to-end under --engine rust).


## Session (2026-06-13) — rust-port W7: storage + structuring; M1 name-parity 207/207

All 11 W7 items verified: variable/cover (HighVariable + Cover, wiring Varnode::high
into Funcdata), merge (HighVariable formation), varmap (stack-layout core; ScopeLocal
deferred), dynamic + kuna_stackguard, blockaction (control-flow structuring — the
goto-quality engine), the kuna region digraph -> RegionIdentifier re-host ->
loweredswitch, coreaction-cleanup, testkunaregion (3 tests), and the M1-closure
diagnostic.

**M1 name-parity: port_audit 207/207 PORT COMPLETE.** The M1-closure agent re-verified
the ~32 ignored unit-test bodies against their C++ sources and corrected the blocker
analysis (LOSS-094-UPDATE): they are NOT W7-blocked but need the W8 print/cast
machinery (testtypes casts) and the W9 C-declaration grammar + spec-loaded
prototype-model registry (testfuncproto/testparamstore). Each stub now carries an
inline DEFER(Wn). M1 BODY-parity re-targeted to a new w9-m1-bodies closure item
(gate: zero #[ignore] in the 8 unit suites). Honest body count today: 175/207
executing, 32 deferred-and-anchored, 1 float DBL_MAX edge (LOSS-025).

Gate: workspace 2,629 tests, 0 failures, 35 ignored; clippy clean; oracle untouched.
Cumulative: 120/201 items. Losses through LOSS-108. Next: W8 (print stack + the
universalAction assembly that wires all ported rules/actions into the real pipeline).


## Session (2026-06-13) — rust-port W6: type system + parameter recovery (M1 name-parity)

All 16 W6 items verified (5 landed pre-limit, 11 relaunched on restored Opus credits
after a weekly-limit interruption — zero work lost, failed roots just re-ran with
deps already merged). Delivered: the full fspec chain (ParamEntry/ParamList ->
ProtoModel/FuncProto -> FuncCallSpecs), the Datatype type system (base total orders
-> struct/union/enum -> TypeFactory structural-identity tree), the canonical
per-opcode typeop inst table (replacing every per-wave local seam resolver),
modelrules, unionresolve, rangeutil (CircleRange + ValueSetSolver), double-precision,
bitfield, constseq, the S4 prototype actions, the kuna returnpair/memset/compareform
pack, and the unit-test harness. LOSS-050 restored (type-1).

**M1 status (honest): name-parity 204/207** (the 3 testkunaregion tests land with the
W7 kuna_regionid port). **Body-parity 174/207**: ~35 unit-test bodies are #[ignore]d
stubs blocked on W7 (HighVariable/merge/Funcdata-high-types) + still-seamed FuncProto
trials (LOSS-094). M1 is therefore a W6+W7 milestone; the W7 M1-gate must un-ignore
all stubs + port testkunaregion + assert port_audit 207/207 with zero ignores.

Gate: workspace 2,384 tests, 0 failures, 35 ignored; clippy clean; oracle untouched.
Cumulative: 109/200 items. Losses through LOSS-094. Next: W7 (S6/S7 + M1 closure).


## Session (2026-06-12) — rust-port W5: the rules mountain + SSA + B2 structural parity

All 16 W5 items verified (15 wave + closure pair): the full dtype interface, 136
ruleaction rule classes in 8 batches, heritage (SSA: dominance-frontier phi
placement + rename), subflow, condexe + expression equality machinery, transform,
7 option-gated kuna rules, coreaction-early, then the closure: deferred funcdata
helpers (replaceLessequal incl. the kuna provenance anchor, distributeIntMultAdd,
CSE machinery, opSetInput constant re-duplication guard) + rule un-deferrals, and
the lift-diff B2 STRUCTURAL differential (Rust FlowInfo + early actions vs C++
break-at-heritage snapshots: >=8 corpus cases structurally equal, exclusions
justified per-test, an over-broad exclusion explicitly rejected to avoid false
passes). Cross-wave effect: w4x-flow-linkage retroactively activated deferred rule
bodies (RuleOrCompare/RuleLzcountShiftBool now complete their C++ rewrites; four
seam pins flipped to full structural assertions). LOSS-077 closed; ledger at
LOSS-080. Gate: 1,849 tests, 0 failures, clippy clean. Cumulative: 93/200 items.
Next: W6 — fspec + type system + rangeutil + the remaining 150 unit tests = M1.


## Session (2026-06-12) — rust-port W4: framework wave (engine, Architecture, P0 registry)

All eight W4 items verified: the Action/Rule engine (processOp dispatch transcribed
exactly; SchedNode derivation proven identical to clone(grouplist)), options DB,
the 4.5k-LOC symbol database over W1's rangemap, cpool/callgraph, Override+
CommentDatabase (P0 stores), Architecture (clean ACCEPT — implemented the W3 seams,
opened kuna-sleigh's emission API closing LOSS-031, proved injection end-to-end),
the XML/Sleigh/Raw front-ends (corpus bootstrap across 8 architectures with opcode
sequences checked against lift fixtures), and the kuna P0 pack: stages.toml +
build.rs codegen + the catalog JSON emitter BYTE-COMPARED against the live C++
'stage catalog' output — the LLM control surface preserved to the byte (39/40/90/22
table rows).

Gate: workspace tests green (1,135 at the 7/8 mark + frontends), clippy clean,
losses through LOSS-052. Carried-forward blocker found by the frontends gate: W3
left Funcdata::opSetOutput/newVarnodeOut/newCodeRef seamed (split-borrow), so real
FlowInfo cannot link op outputs yet — targeted fix agent dispatched; required
before the W5/W6 B2-grade differentials. W5 (dtype expansion + heritage + 8 rule
batches + subflow + kuna rules) running concurrently.


## Session (2026-06-12) — rust-port W3: core IR complete (arenas, Funcdata, FlowInfo, jumptables)

All ten W3 items ACCEPTed (one transient API failure mid-wave; chain relaunched, no
work lost). The decompiler's heart now exists in Rust: VarnodeBank/PcodeOpBank
slotmap arenas with the C++ comparators pinned by an exhaustive >100k-pair
cross-check, the three intrusive op lists with splice-order fidelity, the full
FlowBlock hierarchy + CHK dominators, Funcdata with its mutation API (SeqNum/
create_index allocation order transcribed statement-for-statement), FlowInfo
generating ops through the W2 Sleigh engine (validated against the lift-fixture
corpus), jumptable recovery models, and the five kuna flow hooks (GH-8817/9191/
8471/6930/6882) as logic ports awaiting their W4 Rule wrappers.

Gate: workspace 873 tests, 0 failures, clippy clean. Losses LOSS-029..034 recorded
(W4/W6/W7/W8 seam deferrals — typed, anchored, and verifier-confirmed; the seam
inventory IS the W4/W7 work plan). Cumulative: 54/207 unit tests, 53/200 checklist
items closed. Next: W4 framework wave (action engine, options, database,
Architecture + front-ends, kuna stage registry with stages.toml codegen).


## Session (2026-06-11) — rust-port W2: SLEIGH runtime ported; Rust lifts byte-identical p-code

All ten W2 items ACCEPTed (two relaunched on Opus after a session-limit failure):
translate (restoring LOSS-012 join/spacebase/register arms in kuna-base), context,
pattern, symbol, semantics, pcodeparse (hand parser; found UB-5: upstream keyword
table violates its sorted-search contract), loadimage(+xml), emulate, sleigh-core
(slaformat/SleighBase/Sleigh engine incl. ParserWalker, delay slots, label
resolution), and testfloatemu (31 tests; 54/207 total).

**Parity teeth: `golden lift` fixtures from the C++ oracle — 16 corpus images, 16
distinct SLEIGH languages, 1,171 instructions — replay through the Rust engine
text-exactly (16/16 green), including MIPS/SPARC delay slots, x86-16 context
painting, and LOSS-015 spaceid normalization.** Workspace gate: 564 tests, 0
failures, clippy clean. Losses LOSS-017..028 appended along the way (notable:
ContextCache re-fetch semantics, join-table placement, crossbuild lazy-disassembly
deferral). Oracle re-verified by the fixture agent: PARITY OK + stages 154/154.
Next: W3 core IR (varnode/op/block/funcdata arenas per ADR 0001).


## Session (2026-06-11) — rust-port W1: foundation crates ported and verified

Seven porter agents (isolated worktrees, dependency-ordered fork-after-merge) +
eight independent verifiers (hunt-list protocol, adversarial tests, repair loop):
**all eight W1 checklist items ACCEPTed** — foundation (error/types/partmap/
rangemap), xml (hand parser, 6,485-case differential parity vs the bison oracle),
float+multiprecision (+ cfmt %g emulation), util (crc32/flate2/filemanage),
marshal+space+address (combined branch; packed marshaling bit-exact), opcodes/
pcoderaw/opbehavior (UB-1 fixed with compile-time assert; UB-2 → EvaluationError),
and the first 23 ported unit tests (testmarshal 18 + testmultiprec 5, exact names).

Gates: cargo build/test/clippy --workspace green — **246 Rust tests, 0 failed**;
golden suites all pass (float.csv 3,607 rows, opbehavior.csv 20,227 rows,
addrsort.csv); port_audit suite gate exit 0; C++ oracle untouched by merges.
Losses ledger grew LOSS-010..016 (incl. the miniz-vs-zlib deflate byte divergence
with refined restoration gate, Translate-dependent deferral LOSS-012, UB-2
decision LOSS-014); upstream bugs UB-3 (xml convertCharRef overflow) and UB-4
(rangemap::erase dangling sub-range) recorded. Unit-test progress: 23/207.
Next: W2 (sleigh-wave start + type system) per docs/rust-port/checklist.json.

## Session (2026-06-10) — rust-port W0: verification machinery + workspace (branch `rust-port`)

Started the full Rust port (plan: `docs/rust-port/plan.md`; user decisions: grind to
full parity across sessions, pipeline port-mode to be launched, SLEIGH compiler
deferred — C++ `sleigh_opt` stays the `.slaspec→.sla` builder). Strategy: parallel
cargo workspace under `rust/`, C++ tree untouched as the oracle, Rust bins speak the
same console surface so all oracles verify the port unchanged via `--engine rust`.

**W0 landed (5 concurrent agents, all gates green):**
- `rust/` workspace: 6 crates (kuna-base/num/sleigh/decomp/console/harness), bins
  named `decomp_dbg`/`decomp_test_dbg`, clippy HashMap/HashSet ban proven live,
  7 ADRs (`docs/rust-port/adr/`), `make rust` / `make rust-test` targets.
- Engine switch: `kuna/paths.py` honors `KUNA_ENGINE=rust` (+`KUNA_RUST_PROFILE`);
  `--engine {cpp,rust}` on run_tests/decompile/catalog; default path byte-identical.
  `kuna/port_audit.py`: TEST↔#[test] 1:1 name audit (207 C++ tests inventoried).
- Golden vectors: `decompiler/cpp/kuna_goldengen.{cc,hh}` (`golden
  opbehavior|float|addrsort`, zero upstream edits) + `tools/rust-port/gen_vectors.py`
  → `tests/golden/vectors/` (25k lines, deterministic, run-twice-diff-empty). Two
  latent upstream bugs found and documented (`docs/rust-port/upstream-bugs.md`).
- Goldens harness: `kuna/goldens.py` — all 131 corpus XMLs (83 datatests + 48 stage
  tests) replay standalone in decomp_dbg via raw `<binaryimage>` + `load file`;
  boundaries B0/B2/B3/B4/B5 = 1095 snapshot files (gitignored), 0 unreplayable;
  cpp-vs-cpp determinism 131/131 identical; `compare` exits 1 on first divergence.
  B3 anchors at `break start paramdouble` (LOSS-008), varnode-tree dumps normalized
  for heap addresses (LOSS-009).
- Backlog: `docs/rust-port/checklist.json` — 200 items (91 port + 91 paired verify +
  18 infra/gates), 182,926 LOC scope, blob-sha pinned, validator + `--verify-sha`
  green. Verifier protocol in `docs/rust-port/verification.md`; losses ledger seeded
  LOSS-001..009.

**Oracle parity re-verified after relink: 207/207 unit, 675/675 datatest, PARITY OK.**
Next: W1 foundation port (xml/marshal/float/multiprec/space/address → 54 unit tests)
+ pipeline port mode.

## Session (2026-06-10) — port angr's RegionIdentifier (S7 region observability)

Ported angr's region identification analysis (`region_identifier.py` 1349 LOC +
`graph_region.py` + `utils/doms.py` + the five `utils/graph.py` helpers) to C++ as a
faithful, **analysis-only** S7 artifact — the first standalone, inspectable region
tree in kuna (previously implicit inside `ActionBlockStructure`'s collapse engine).

**Mechanism.** Two new kuna-owned file pairs, zero upstream anchor edits:
- `kuna_regiongraph.{hh,cc}` — deterministic mutable digraph (`KunaRegionNode`/
  `KunaRegionGraph`, global `(addr, creation-ident)` order replacing Python dict
  insertion order), DFS back edges, deterministic postorder, quasi-topological sort
  (iterative Tarjan + `_append_scc` + panic mode), `subgraph_between_nodes` (per-succ
  `has_path` → one reverse BFS), CHK immediate dominators, and the full
  `IncrementalDominators` port (lazy dominance frontiers + `graph_updated` patching).
- `kuna_regionid.{hh,cc}` — `KunaGraphRegion` (head/graph/successors/
  graph_with_successors/full_graph/cyclic), `KunaRegionVisitor` (recursive block
  walker), `KunaRegionIdentifier` (supergraph chain-merging with angr `MultiNode`,
  cyclic phase: back-edge headers + 3-stage loop refinement + cyclic abstraction;
  acyclic phase: postdom-tree climb + dominance-frontier `_check_region` + iterative
  collapse), plus console commands. Builds its private graph from
  `Funcdata::getBasicBlocks()` (read-only) or synthetic addr nodes (test mode).
  Analysis-only divergences (AIL statement mutation omitted: loop-exit jump
  insertion, `force_loop_single_exit` guarded successors) marked `(kuna)` in-source
  and documented in `docs/regions.md`.

**Exposure.** Console: `region tree` / `region blocks` / `region walk` (recompute
per call, bulk stream); `stage map region tree` → S7 / `loop-refinement` (options
stay LATENT; the commands are the observable half — exposure string updated).
Python/LLM: `python -m kuna.decompile BIN FUNC --regions` (second `openfile write`;
library returns `(c, regions)`). Three `surfaceTable` rows; **no** settable/option/
ElementId (nothing flips, output unchanged → no DIV entry; `catalog --check` clean).

**Verification.**
- Unit tests `decompiler/unittests/testkunaregion.cc` (kuna-owned in vendored dir,
  UPSTREAM.md row): ports of angr `test_region_identifier_0/1` (top region == 2
  nodes) + a kuna loop case. 204→207, baseline re-saved (+3 keys only).
- Datatest `tests/stages/kuna-regions.xml` (gh8724 bytes as loop+diamond CFG): cyclic
  region at 0x3e0, root at entry, walker exactly-once, `stage map` routing. Stages
  baseline 145→150 (+5 keys only).
- Adversarial fidelity review (8-agent workflow, all 4 algorithm sections vs the
  Python line-by-line): 2 confirmed findings, both fixed — 64-bit guard caps (old
  `int4 2*n*n` overflowed/insta-threw at n≥32768 where angr completes) and
  carry-aware `_sort_edge` compare (Python sums are arbitrary-precision); 2 findings
  refuted (dead-code guards). All other methods verified faithful.
- Stress: 158 + 141 real functions (bomb/awk/calc/loops/switch binaries) — 100%
  region trees, zero errors; the 141 ran under `-DKUNA_REGIONID_DEBUG`, which
  recomputes dominators+frontiers from scratch after **every** incremental update
  (`verify()`, port of `_debug_check`) — zero divergence.
- Gates: 207/207 unit + 675/675 datatests PARITY OK; stages 150/150 PARITY OK;
  catalog OK; `--regions` CLI smoke on fauxware.

Docs: new `docs/regions.md` (commands, stable dump grammar, walker API, divergences);
rows/notes in `docs/stage-mapping.md`, `docs/stages.md`, `docs/stage-implementation.md`,
`tests/stages/README.md`, `UPSTREAM.md`; counts in `CLAUDE.md`/`kuna/run_tests.py`.

## Session (2026-06-09) — ite-region-converter-missing-5db28e (option `stackguard`)

**Opportunity.** angr `test_ite_region_converter_missing_break_statement::authenticate`
(`binaries/tests/x86_64/ite_region_converter_missing_breaks`, x86-64 GCC,
`-fstack-protector`). angr is shorter and goto-free (loc 21 vs 40, **gotos 0 vs 1**,
labels 0 vs 1).

**Why angr was better.** The function is compiled with the glibc stack protector: it
saves the TLS canary at entry (`canary = *(fs:0x28)`) and at every exit reloads fs:0x28
and runs `if (canary != reload) __stack_chk_fail(); return v;`. That canary check is a
**single shared return point**, so a return deep inside the loop (the `Authenticated!`
path) cannot return directly — Ghidra's structurer routes it through `goto label_<addr>`
(+ a `label_<addr>:`). angr's `StackCanarySimplifier` strips the canary epilogue, after
which the tail is a bare `return v` that `ActionReturnSplit` duplicates into each
predecessor, killing the goto. (angr also folds the trailing `if(c) v=1 else v=0` into a
ternary `c ? 1 : 0`; Ghidra's C printer has **no ternary operator**, so that rendering is
out of scope — the canary strip is the reproducible win.)

**Mechanism.** New `kuna_stackguard.{cc,hh}` adds `ActionStripStackGuard`
(ElementId 4021), run in `actfullloop` immediately before `ActionReturnSplit`. Detection
is **purely structural** (kuna's BFD console loader doesn't resolve the `__stack_chk_fail@plt`
stub to its name — it renders as `sub_<addr>`): a CBRANCH whose `INT_EQUAL`/`INT_NOTEQUAL`
boolean has BOTH operands deriving from a `LOAD` of `<base>+0x28`, with a handler `CALL`
on the corrupted-canary branch. The corrupted branch is removed with the stock
`Funcdata::removeBranch` (CBRANCH→fall-through, MULTIEQUALs patched) and
`removeUnreachableBlocks` collects the orphaned `__stack_chk_fail` block; the dead canary
load/store/compare are reaped by the next deadcode pass. Self-gates once no handler
call remains. Anchors (all `(kuna)`): `architecture.{hh,cc}` flag `strip_stack_guard`,
`options.cc` registration, `coreaction.cc` action, `kuna_stages.cc` settable+surface rows.

**Ablation / default.** With the option default-ON, **3** upstream datatest assertions
change (Multi-size return #1–3 — real canary-bearing functions). Stripping deletes real
instructions, so the option is kept **default-OFF (opt-in)**; default output stays
byte-identical (PARITY OK, no DIV entry). On `authenticate` with `option stackguard on`:
the `goto`/`label`, the `__stack_chk_fail` call, and one `if` disappear, and the deep
match path becomes `return 1;` directly. Test:
`tests/stages/ghangr-ite-region-converter-missing-5db28e.xml` (two-pass: off proves the
bug, on proves the fix); demo `tools/ite-region-converter-missing-5db28e_angr_demo.sh`.

## Session (2026-06-09) — continuous angr-inspired feature pipeline (`kuna.pipeline`)

Turned the hand-driven "study angr, find where it's better, port one improvement at a
time" methodology (the `loweredswitch`/`namestyle` work) into an observable, mostly
autonomous loop that opens one PR per feature.

**Provenance schema (prep).** Added three self-describing fields to `KunaSettable`
(`kuna_stages.hh`) and emitted them in `kunaEmitSettableJson` + `catalog.py`:
`source_decompiler` (`angr`/`ghidra-upstream`/…), `inspiration` (concrete origin), and
`change_kind` (`correctness-fix`|`presentation-default`|`structure-recovery`|`opt-in-tool`
— the field an LLM reads to judge default-ON eligibility). Migrated all 21 catalog rows;
extended `tests/stages/kuna-catalog.xml` (+4 assertions). So every pipeline feature is
fully discoverable via `python -m kuna.catalog --json` without reading C++.

**Pipeline (`kuna/pipeline/`, `tools/pipeline/`).**
- `reference/` — a `ReferenceDecompiler` seam (angr first; reko/BN/IDA are future adapters);
  `_angr_runner.py` runs under the angr venv (9.2.213, `KUNA_PIPELINE_ANGR_PYTHON`).
- `worklist.py` — AST-parses angr's `test_decompiler.py` into 221 `(binary, function)`
  targets (captures `arch`/`load_debug`/custom-options).
- `compare.py` + `sweep.py` + `rank.py` — reference-vs-kuna structural comparison
  (gotos/labels/switch/loops/loc), ranked into `docs/pipeline/{opportunities.json,matrix.md}`.
  First x86 sweep: **185 functions, 83 where angr is structurally better** (dominated by
  goto/label structuring — uncovered in kuna).
- `state.py`/`status.py` — flock-guarded worker inventory + claims + `--watch` live view.
- `select.py` — pick the next unclaimed, highest-score gap.
- `tools/pipeline/{worker_prompt.md,worker.sh,run.sh,open_pr.sh,install_gh.sh}` — each
  feature is implemented by a headless `claude -p` session in its own git worktree, gated
  by the `loweredswitch` recipe + `catalog --check` + PARITY OK; PRs on `feat/angr-<slug>`
  (branch push via SSH, `gh pr create` with a compare-URL fallback).

**Policy.** A feature ships default-ON only if its full ablation changes **0/675** upstream
assertions, else default-OFF opt-in — the loop never re-pins `docs/baseline.json`. Each PR
carries a self-documenting `docs/features/<slug>/` bundle (analysis, angr-vs-kuna, plan,
`record.json` incl. the resumable session id) so a reviewer can resume the Claude session.

**Gates green for the prep:** 204/204 unit + 675/675 datatests **PARITY OK**; stages
**145/145**; `catalog --check` OK; `docs/assertions.md` + `docs/baseline-stages.json`
regenerated. See `docs/pipeline.md`.

## Session (2026-06-09) — angr-style default naming (`option namestyle`, DIV-5)

Re-skinned the decompiler's default output to read like the angr decompiler, behind a
single master toggle `option namestyle angr|ghidra` (default `angr`; `ghidra` restores
upstream naming byte-for-byte).

**What changed (default output):**
- locals/temps/`extraout_*`/`unaff_*`/`in_*` → `v1, v2, v3 …`; arguments → `a0, a1 …`.
- global data (`<type>Ram<addr>`, volatile annotations) → `dat_<addr>` (lowercase, no
  leading zeros, no `0x`); unnamed functions `func_0x…` → `sub_<addr>`; labels
  `code_r0x…` → `label_<addr>`.
- each **local declaration** gets a trailing source-location comment: `// rax` (register)
  or `// stack - 0x10` (frame-relative). Real/recommended names are preserved.

**Mechanism.** New `kuna_naming.{hh,cc}` (`OptionNameStyle`, helpers, ElementId 4020) +
a per-`Architecture` flag `name_style_angr` (default on). Naming intercepted in
`Scope::buildDefaultName` (aN/dat_/vN) and the `persist` arm of
`ScopeInternal::buildVariableName`; functions in `Architecture::nameFunction`,
`PrintC::genericFunctionName`, `FspecSpace::printRaw`; labels in `PrintC::emitLabel`;
the comment in `PrintC::emitVarDeclStatement`; volatile data in `PrintC::pushAnnotation`.
The cross-function default-name guard (`ActionNameVars::makeRec`) now uses
`kunaIsGeneratedName` (recognises `param_N` + `aN`/`vN`).

**Tests.** Re-pinned **185/675** datatest assertions across 50 files to the new default
names (+ `pointerrel.xml` script `rename`/`retype` names), and **25** stages assertions
across 17 `gh*.xml` files — all via parallel sub-agent workflows that dump each test's new
output (`KUNA_DUMP=1`) and verify zero failures per file. New `tests/stages/namestyle.xml`
decompiles a loop (unnamed-helper call + global write) once at the default and once under
`option namestyle ghidra`, pinning both the angr names and the restored Ghidra names — the
"it goes away when you flip the scheme" proof. **PARITY OK: 204/204 unit + 675/675
datatests; stages 141/141; catalog OK.** `docs/divergences.md` DIV-5, `docs/baseline-stages.json`
→ 141 keys, `docs/assertions.md` regenerated, `UPSTREAM.md` *Divergence* updated.

## Session (2026-06-08) — port angr's LoweredSwitchSimplifier (`option loweredswitch`)

Implemented a port of angr's `LoweredSwitchSimplifier` (SAILR, USENIX Security 2024):
reconstruct a GCC-lowered comparison cascade (a binary-search `if/else` tree over one
variable) back into a C `switch`. Validated on coreutils `fmt`/`main`.

**Result.** With `option loweredswitch on`, `fmt`/`main` renders `switch(iVar5)` with **all
9 getopt cases + `default`** (0x70 'p', 0x74 't', 0x75 'u', 0x77 'w', 0x73 's', 0x67 'g',
0x63 'c', and the two negative options -0x83/-0x82), correctly nested inside the getopt
`while (... != -1)` loop — matching angr's expected output for
`test_reverting_switch_clustering_and_lowering_fmt_main`. **DIV-4 default-on** and still
**PARITY OK: 204/204 unit + 675/675 datatests, catalog OK** — see "Default-on flip" below.
New committed testcase `tests/stages/ghangr-loweredswitch.xml` (7 assertions, pass-1 sets the
option off; `docs/baseline-stages.json` → 128 keys).

**Default-on flip (DIV-4).** Flipping `recover_lowered_switch` on naively regressed **10/675**
upstream assertions: the pass also converted *hand-written* linear `if/else-if` chains
(`elseif.xml` ×6, `copytrim.xml` ×3, `partialunion.xml` ×1) to switches. Discriminator added:
**require the GCC binary-search structure** — the cascade must contain ≥1 range/`jle` split
(`sawRange` in `recoverCascade`). A purely linear equality chain is a hand-written `if/else-if`
and is left alone; a compiler-lowered switch has the balanced range-split tree. With the guard
the ablation is **0/675 changed** and `fmt`/`main` still recovers. `option loweredswitch off`
restores the upstream rendering. (`docs/divergences.md` DIV-4.)

**Methodology extended to all of angr's SwitchLowering testcases.** Ran every angr
`LoweredSwitchSimplifier` testcase through kuna (default-on) and recorded the outcome:

| angr testcase | binary | kuna result |
|---|---|---|
| `..._fmt_main` | `fmt` (PIE) | ✅ **full** — `switch` + 9 cases + default. Committed: `ghangr-loweredswitch.xml`. |
| `..._mv_o2_main` | `mv_-O2` (PIE) | ✅ **partial** — `switch` + 9 explicit cases + default; 3 special-return values (110, −131, −130) fold into the default's getopt loop. Committed: `ghangr-loweredswitch-mv.xml`. |
| `..._stat_human_fstype_*` | `stat.o` | ⚠️ **capped** — a 68-case sparse binary-search tree; the synthesized switch makes Ghidra's structurer emit an unstructured (`t_multigoto`) switch and abort with *"Case block has become detached from switch"*. Now guarded by the **≤16-case fail-safe cap**, so it falls back to the if-chain (no crash). |
| `..._cksum_digest_print_filename` | `cksum-digest.o` | ⚪ **n/a** — the cascade is a *linear* equality chain (10/13/92), which the binary-search guard correctly skips; the raw `.o` also has unapplied relocations. |
| `..._filename_unescape` | `b2sum-digest.o` | ⚪ **n/a** — not a ≥3-case single-variable cascade in Ghidra's SSA view (the chars are reloaded); ON==OFF. |
| `..._cat_main` (+ `_no_endpoint_dup`) | `cat.o` | ⚪ **n/a** — `main` is in `.text.startup` at section-offset 0 and collides with `.text` at vaddr 0 when kuna loads the raw `.o`, so the wrong function decompiles; also a real `.rodata` jump table. |

**Two findings worth recording.**
1. **Harness limitation, not a feature limitation:** kuna's `.o`-loading / bytechunk path does
   not apply ELF relocations or resolve section overlaps, so the four relocatable-`.o` angr
   testcases (`cksum`/`b2sum`/`cat`/`stat`) cannot be reproduced as committed kuna datatests the
   way the linked PIEs (`fmt`, `mv`) can. Applying relocations on `.o` load is the prerequisite
   to porting them.
2. **Structurer ceiling:** very large synthesized switches (e.g. `stat`'s 68-case tree) exceed
   Ghidra's `CollapseStructure` ability to form a clean `BlockSwitch` and abort the function. The
   ≤16-case cap keeps the default-on pass fail-safe; lifting it needs structurer robustness work
   (handle a `t_multigoto` switch block / detached case gracefully). All angr SAILR examples are
   ≤11 cases, so the cap does not affect them.

Committed second testcase: `tests/stages/ghangr-loweredswitch-mv.xml` (`mv_-O2`/main, PIE; 6
assertions; `docs/baseline-stages.json` → 134 keys). The general/native angr switch tests
(`test_decompiling_switch0/1/2`, `..._abnormal_switch_case_*`, etc.) are **real jump-table**
switches recovered by Ghidra's existing `BRANCHIND` machinery, not comparison-cascade lowering,
so they are out of scope for `loweredswitch`.

**The architectural problem (what made this hard).** angr edits its *structured region
graph* (an S7 artifact) and emits a `SwitchCase` node. **Ghidra has no region-graph edit
hook**: a `BlockSwitch` is only ever produced by `CollapseStructure::ruleBlockSwitch` from a
`BlockBasic` flagged `f_switch_out`, which is set only for a `CPUI_BRANCHIND`-terminated
block backed by a `JumpTable` — i.e. a Ghidra `switch` is fundamentally an **S2 artifact**
recovered from a *lifted indirect jump*. When the compiler lowered the switch to comparisons
there is no BRANCHIND. So the port **synthesizes the S2 artifact** (a BRANCHIND + a
hand-built, pre-labelled JumpTable) from the control-flow pattern and lets the existing
structurer + printer emit the switch. This **inverts the usual S1→S2 dependency** (lifting
produces the BRANCHIND that S2 explains; here a late pass produces it from a CFG pattern) —
a stage-model feedback edge realized via the existing **restart (mechanism c)**:
- `ActionLowerSwitchDetect` (S2 `switch-model`, placed after `ActionSwitchNorm`): on the
  simplified CFG, cluster the cascade (Varnode-identity, range-node traversal, advancing past
  the `-1` EOF sentinel guard), record it in a survives-`clear()` sticky side table, request a
  restart.
- `ActionLowerSwitchInstall` (before `ActionHeritage`, gated to the pre-SSA window by
  `getHeritagePass()==0`): replay the recorded surgery — head CBRANCH → BRANCHIND(V), rewire
  out-edges to the case targets + default, build the labelled JumpTable — so the following
  heritage rebuilds phi over the corrected CFG. **No MULTIEQUAL/phi hand-patching** (the whole
  reason for the restart-based, pre-SSA strategy).

**Phase ideas this stressed / broke (documented).**
- The model assumed S2 jump-table recovery is *downstream* of S1 lifting. This pass makes S2
  the producer of a BRANCHIND that lifting never emitted — the S1→S2 inversion above. It is
  the first kuna divergence that *creates* a CFG artifact rather than gating an existing
  decision.
- It is the first kuna pass that performs **CFG surgery** (op replacement + edge rewiring +
  block removal). All prior divergences gate a rule/flag/label. This required the restart
  vehicle to stay SSA-safe.

**Limitations / what was not possible without more work (honest report):**
1. **Post-SSA CFG surgery is effectively off-limits.** Doing the rewrite directly on the
   simplified (post-heritage) CFG would require hand-repairing MULTIEQUAL input slots across
   multi-edge additions and block deletions — the exact bug class the restart avoids. The
   restart costs one extra decompilation pass per function.
2. **`maxrestarts == 1` for the `universal` group** (coreaction.cc). A function that *also*
   needs a genuine multi-stage-jump restart in the same invocation would lose ours (the switch
   then stays an if-chain). Because the hint is sticky, a second decompilation recovers it;
   raising the budget to 2 is a one-line global change if co-occurrence proves common.
3. **`JumpTable::foldInGuards` is called unconditionally** even for a fully-labelled table, so
   a synthetic table needs a non-null model. It must be a **non-override** model
   (`JumpModelTrivial`): `clearJumpTables()` *preserves* override tables across a restart's
   `clear()`, and `followFlow` then re-`switchOver`s the stale table and segfaults. Required a
   1-line `(kuna)` `JumpTable::kunaSetTrivialModel` member (no public model setter existed).
4. **MVP scope (conservative, fires on fmt/main):** single switch variable; pure
   comparison-spine blocks only (case bodies/default fail the purity check and bound the
   cascade as leaves); distinct case targets; ≥3 cases / ≥2 distinct targets; the `-1` EOF
   sentinel is special-cased as the loop guard. angr's Type-A "extra statements before the
   comparison" handling and shared-target (fall-through) cases are not yet supported.
5. **Bytechunk regression test needs the binary's neighbours mapped + named** to bound main's
   flow (it falls through past its own end after a `call error`), exactly as `read symbols`
   does for the whole ELF; only `main` is decompiled.

Anchor edits (all `(kuna)`-marked, in `UPSTREAM.md` *Divergence*): `funcdata.hh`,
`jumptable.hh`, `architecture.{hh,cc}`, `options.cc`, `coreaction.cc`, `kuna_stages.cc`,
`kuna_restartlog.{hh,cc}`. New files: `kuna_loweredswitch.{hh,cc}` (ElementId 4019).

---

## Session goals (2026-06-08) — extend stage-model fixes to ALL remaining reproduced PHADE issues

- [x] Triage every remaining reproduced PHADE issue (46) — reproduce in `decomp_dbg`,
      bucket as stage-exposure / spec-fix / not-viable, localize to a stage. Durable ledger
      in `docs/issue-coverage.md`.
- [x] Fix every viable stage-exposure and spec-fix via per-issue TDD (failing testcase →
      gated fix → catalog/parity/test-stages gates → one commit each). PARITY OK and
      `catalog --check` green at every commit.
- [x] Flip the ablation-clean fixes default-on (DIV-3) after the full-suite ablation.
- [x] Update living docs + ledger; one testcase per fixed issue in `tests/stages/`.

### Results (2026-06-08)

- **26 of 46 issues fixed** (one commit each, full parity 204/204 + 675/675 at every commit):
  - **16 via SLEIGH spec-fix**: 9001 (HCS12 BRN), 1243 (8051 ADDC carry), 7418/7139/8790/9184
    (x86 rel16 mask / disp16 sign / prefetch reg-form / FMA YMM256), 5897/8456/8391/7890
    (ARM cpy→mov / MRS-banked / Thumb CDE mrrc / VFP vcvt), 1951+8844 (avr8 branch wrap),
    4788 (V850 RH850 Bcond), 6389 (8085 undoc), 7451 (RISC-V Zfa fli.s), 6904 (PPC e500 mr).
  - **10 via option-gated stage-exposure** (ElementIds 4010–4018): 1276+8777 `flagcompare`,
    9191 `switchmodbound`, 8017 `stackprobeloop`, 8467 `dynamichashmax`, 8500 `stackalias`,
    8724 `arraystride`, 6882 `sparcstructret`, 9203 `condexeplace`, 9218 `inputvarnodeadjust`.
- **DIV-3**: six corpus-clean non-destructive options flipped default-on (ablation: 0/675
  upstream assertions changed) — `flagcompare`, `stackprobeloop`, `dynamichashmax`,
  `arraystride`, `condexeplace`, `inputvarnodeadjust`. The three destructive ones
  (`switchmodbound`, `stackalias`, `sparcstructret`) stay opt-in. (`docs/divergences.md`.)
- **13 not-viable** (precise reasons in the ledger): 809, 1708, 2033, 6333, 6342, 6664, 6682,
  6723, 6836, 7377, 8028, 8694, 8794 — plus 3 reclassified spec→not-viable (766, 5666, 7332).
- **2 already-correct**: 3847 (FYL2X) and 3723 (ARM `udf`, misfiled as AArch64).
- **2 deferred** (attempted, non-convergent): 6674 (V850 free-register switch — distinct from
  the x86 LOAD-table case `switchmodbound` fixes), 6858 (stripped-PIE main — distinct from
  8017's gcc probe loop; full-function testcase aborts the harness).
- **Tooling**: `KUNA_DUMP` env var on the test harness (echoes captured console output;
  default-off, zero behavior change) — the triage/repro substrate for raw-byte issues.
- **Method**: read-only triage workflow + worktree-isolated implementation workflows
  (spec-fix and stage-exposure lanes), then serial replay on `main` with the binding
  parity/catalog/test-stages gate per commit. `make` parallelism pinned to `NJOBS=4`
  (an unbounded `-j` OOM-killed a mid-session build).
- `tests/stages/` grew 48 → 121 assertions; all 46 remaining PHADE issues are now terminal.

## Session goals (2026-06-06/07) — stage-model physicalization: split the decompiler into stages, fix 10 PHADE issues

- [x] Address each stage: implement the model physically (registry + console + assertions
      + observability); per-stage changelog in `docs/stage-implementation.md`
- [x] Verify all testcases pass after every change (PARITY OK at every one of the
      session's commits; `make test-stages` grown 4 → 43 assertions)
- [x] Critique the stages empirically (`docs/stage-critique.md`: per-issue stage-fit,
      ablation matrix, §13-navigation score, per-stage verdicts)
- [x] Fix + reproduce ≥10 PHADE issues through the stage model (10 fixed: GH-2786, 8471,
      6930, 6990, 1282, 7190, 8817, 8913, 9230, 1537)

## Results (2026-06-06/07)

- **Infrastructure** (each component a commit, PARITY OK gated): `kuna_stages` (stage
  registry: group→stage map, 40-row sub-stage catalog, surface routing — queryable via
  `stage list/map/status`); `kuna_console` (self-registering `IfaceKunaCapability` —
  console commands with ZERO upstream anchor edits); `kuna_assert` (`kassert
  <stage> <substage> ...` typed assertion API over Override/SymbolDB/proto/options, with
  computed minimal rewind scopes REPORTED per stage-model §12); `kuna_restartlog`
  (mechanism-c restart reasons recorded at all 5 trigger sites; the switchmulti
  multistage restart is now visible); `pipeline` (reduced-pipeline sub-queries,
  mechanism c′ at the console); `quality` (goto-count metric — the measurement half of
  roadmap #4). `docs/stage-mapping.md` §0 remaps all 115 files to P0/S1–S9.
- **10 issues fixed through the model**, all reproduced+fixed in `tests/stages/` (each
  testcase asserts bug-under-default AND fix-under-assertion): 1 default-flip (GH-2786,
  invalid `--x` C; DIV-1, 0/675 upstream churn), 8 option-gated exposures (S2..S5,
  options `thumbfuncptr inferfuncentry returnpair booleanmask ovlesssimplify
  v850indirectbranch addcarrychain memsetrecover`, ElementIds 4002–4009), and 1 pure
  ROUTING fix (GH-1537 fixed by GH-9230's option with zero new code — exposed decisions
  generalize). Plus `V850.cspec` `ctbp`→`CTBP` case fix (V850 could not load at all in
  the case-sensitive C++ decompiler).
- **Method**: 12-issue triage + 6 hard implementations ran as parallel worktree-isolated
  subagent workflows; every fix gated on a default-off Architecture flag
  (`if (!flag) return 0;`) so upstream parity is structural, not tested-for. Worktree
  diffs replayed sequentially into master with per-issue gates.
- **Key empirical findings** (full critique in `docs/stage-critique.md`): symptom-stage ≠
  decision-stage (GH-8471/6930 look S9, live in S5 — the model's strongest validation);
  ablation matrix separates text-only (S9) from analysis-bearing (Band B) flips, with
  `returnpair single` breaking exactly the 3 upstream tests that need the pair (the
  definition of an assertion-worthy decision); GH-8748 negative result — else-if tail
  duplication is blocked by un-clonable live-out INDIRECT phis, scoping what
  quality-gated structuring (roadmap #4) actually requires.
- **Honest assessment**: defaults are byte-identical to upstream except DIV-1
  (`docs/divergences.md`); 9 of 10 fixes are opt-in assertions, so nothing is "fixed"
  for a user who never sets an option — the deliverable is named, tested, durable
  decision surfaces (and one real bug fix). S6/S7 attracted no issues (dataset bias);
  GH-28 deferred (repro diverges between debug/test harness builds); GH-8748 partial.
- **Addendum (2026-06-07, user decision)**: **DIV-2** — the eight non-destructive fixes
  are now ON BY DEFAULT (compareform=original, arraynotation, thumbfuncptr,
  inferfuncentry, booleanmask, addcarrychain, ovlesssimplify, memsetrecover with a new
  >=2-COPYs/>=16-bytes fill guard). `returnpair single` and `v850indirectbranch` stay
  opt-in (ablation-proven destructive). 22/675 upstream datatest assertions re-pinned
  in place to the new renderings (all compareform/arraynotation text forms; 15 files,
  updated+verified by parallel subagents); both baselines regenerated; the nine
  affected stage testcases inverted (default = fix, `option ... off` = old behavior).
  Full record: `docs/divergences.md` DIV-2. Gates: 204/204 + 675/675 PARITY OK (kuna
  oracle) + test-stages 43/43.
- **Engineering hazard fixed**: `OptionDatabase::registerOption` keys on
  `ElementId::find(name)` → options without registered ElementIds silently collide on
  `ELEM_UNKNOWN`; all kuna options now allocate 4000+ ElementIds.

## Session goals (2026-06-06) — stage-model prototype: fix a real issue via finer-grained stages

- [x] Pick a reproducible, dataset-backed Ghidra issue convertible to a kuna testcase
      (chose **GH-558** from `~/github/PHADE/issue_dataset`: 58 reproduced issues triaged
      by an 18-agent workflow; GH-558 = pure representation policy, zero plain-bug risk)
- [x] New testcase family for stage-model issue testcases: `tests/stages/` +
      `make test-stages` + `docs/baseline-stages.json` (upstream baseline untouched)
- [x] Map the issue to the stage model and implement the fix as finer-grained stages
- [x] Verify: testcases pass *because of* the stage split; upstream PARITY OK throughout
- [x] Writeup: `docs/prototypes/gh558.md`

## Results (2026-06-06)

- **First intentional divergence from upstream** (authorized): vendored-file edits are
  now permitted; tracked in `UPSTREAM.md` *Divergence*. kuna decompiler additions live
  in `decompiler/cpp/kuna_*.{cc,hh}` (auto-linked via the upstream Makefile's wildcard).
- **GH-558 fix 1 — `compareform`:** the `V <= c => V < c+1` canonicalization split into
  `canonicalcompare` (Band B, named group) + `presentcompare` (`ActionPresentCompareForm`
  at the S8→S9 boundary), connected by a provenance bit set in the single primitive
  `Funcdata::replaceLessequal`. P0 assertion: `option compareform canonical|original`.
- **GH-558 fix 2 — `arraynotation`:** standalone `PTRADD` rendering (`base + index` vs
  `&base[index]`) exposed as `option arraynotation on|off` in `PrintC::opPtradd`.
- **Key discovery:** the canonicalization decision fires from *three* sites across two
  stages — the rule pool AND the structuring-time branch-flips (`opFlipInPlaceExecute`/
  `opNormalizeFlip`); per-rule toggles can't reach it, and a cleanup-pool inverse runs
  too early. The working placement (after the last flip pass, before prototype/cast
  fixation) shows the model's S8→S9 boundary is a physical program point.
- **Experiments:** forcing `compareform original` globally changes 12/675 upstream
  assertions (pure text); removing `canonicalcompare` changes a *different* 9/675
  (analysis effects) — empirical evidence the two sub-stages are distinct decisions.
- **Honest assessment:** no decompiler behavior was "fixed" — defaults are byte-identical
  (PARITY OK 204/204 + 675/675 at every milestone); the issue is resolved purely by
  exposing decisions. Caveat recorded: assertions are currently function/global-grain;
  per-op anchors (DynamicHash) are future work.
- **Tests:** `make test-stages` → 4/4 (`gh558-compareform.xml`, `gh558-arraynotation.xml`
  — the latter uses the issue's own attached binary; before/after `p->r + iVar2` →
  `&p->r[iVar2]`).

## Session goals (2026-06-05)

- [x] Find the Ghidra decompiler testcases (deep-decompiler only) and create a way to run them
- [x] Identify the C/C++ decompiler source in Ghidra and how to extract it
- [x] Extract it into kuna and wrap it in a simple Python interface (binary + function → C)
- [x] Port the testcases to the stripped-down decompiler
- [x] Record a procedure for porting future upstream Ghidra decompiler changes
- [x] Map decompiler source files to STAGES.md (document mismatches/new stages)

## Results

- **Tests:** `decompiler/unittests/` (204 C++ unit tests) + `decompiler/datatests/` (83 XML
  regression tests → 675 assertions). Java GUI tests excluded. Harness = upstream
  `decomp_test_dbg`, wrapped by `kuna.run_tests` (parses unit/stderr + data/stdout, parity diff).
- **Parity:** kuna build reproduces the pristine-ghidra baseline EXACTLY — 204/204 unit,
  675/675 datatest assertions, identical pass set. `docs/baseline.json` is the recorded oracle;
  `python -m kuna.run_tests --baseline docs/baseline.json` → **PARITY OK**.
- **Wrapper:** `python -m kuna.decompile <bin> <func|0xaddr> [--addr]` drives `decomp_dbg`
  (`load file`/`read symbols`/`load function|addr`/`decompile`/`print C`); C captured via
  `openfile write` so prompts never leak in. Verified on a compiled binary by name and address.
- **Upstream sync:** `tools/sync_upstream.py` (diff vendored paths → rewrite prefixes → git apply
  → bump GHIDRA_REV). Validated clean forward-apply + non-mutating dry-run on two historical revs.
- **Stage mapping:** `STAGE_MAPPING.md` places all 115 .cc exactly once across the 19 stages +
  infrastructure, anchored to `universalAction`. Documented gaps: stages 06 Dataflow, 10 Memory,
  12 Aggregate Types, 15 Interproc have no dedicated module (emergent / folded into others).

## Build note (libbfd without root)

`decomp_dbg`/`decomp_opt`/`decomp_test_dbg` link libbfd. The proper prerequisite is
`sudo apt-get install -y binutils-dev`. This session had no sudo, so libbfd was fetched into
`./.bfdlocal` via `tools/fetch_bfd.sh` and the build run as `make BFD_PREFIX=$(pwd)/.bfdlocal`
(the Makefile injects it through the upstream `ADDITIONAL_FLAGS`/`BFDLIB` without editing
upstream files). `sleigh_opt` and all 148 `.sla` build without libbfd.

## Log

### 2026-06-05 — All 18 PORT_PROBLEMS fixed
- sync_upstream.py hardened: --no-renames + core.quotepath=false diff, hard-fail
  (PatchRewriteError) on quoted headers / rename-copy / binary markers, dry-run is
  truly side-effect free, write_ghidra_rev verifies its substitution, analyze()
  reports vendored-only kuna paths, all-skipped diffs short-circuit cleanly,
  .kuna_sync.patch gitignored. Verified end-to-end on synthetic repos (boundary
  renames, unicode, new processor module) + real-data reverse-sync dry-run.
- Makefile: .NOTPARALLEL (P8 race), jobserver-aware SUBJOBS (P9), nproc fallback
  (P11), libbfd.so located by wildcard not triplet guess + validated in check-deps
  and fetch_bfd.sh (P10), make test builds binaries/specs only when missing (P12).
- run_tests.py: indented/`Execution failed` error forms parsed + deduped and force
  nonzero exit (P13), 128+signal exit on signal-killed harness (P14), duplicate
  test names disambiguated with " @dupN" in the pass keyset (P15; baseline
  regenerated -> 879 keys = 204 unit + 675 data, PARITY OK re-verified), run()
  rejects names with mode='all' (P16). decompile.py: SLEIGHHOME-vs--s comment (P17).
- Docs: UPSTREAM.md apply-strategy paragraph rewritten to match reality (P18),
  CLAUDE.md sync paragraph aligned. Resolution table appended to PORT_PROBLEMS.md.

### 2026-06-05 — Post-port audit (PORT_PROBLEMS.md)
- Full audit of the port for translation bugs: blob-hash sweep of all 1039 vendored
  files vs upstream @ GHIDRA_REV (byte-identical, modes too), fresh clean rebuild
  (148/148 specs), 204/204 + 675/675 PARITY OK, e2e decompile (name/addr/stripped +
  error paths), every decompile.py console command and run_tests.py regex verified
  against vendored harness sources, STAGE_MAPPING 115/115 + line anchors verified.
- **Vendored translation is clean — zero content bugs.** 18 issues found in kuna-owned
  glue, all adversarially verified and recorded in `PORT_PROBLEMS.md`: 2 major (both
  future-sync risks in sync_upstream.py: --dry-run mutates UPSTREAM.md on the empty-diff
  path; boundary-crossing renames corrupt/abort a sync), 13 minor, 3 info. None affect
  current build/test/decompile correctness.

### 2026-06-05 — Session start, exploration & plan
- Located decompiler source: `ghidra/Ghidra/Features/Decompiler/src/decompile/`
  (cpp: 115 .cc / 114 .hh; upstream Makefile builds `decomp_dbg`, `decomp_test_dbg`,
  `sleigh_dbg`, `ghidra_dbg`, libs). Pure C++11; deps: libbfd (console/test binaries
  only), libz; bison/flex outputs are committed upstream.
- Located the deep-decompiler-only tests: `unittests/` (7 files, 204 TEST()s, pure C++)
  and `datatests/` (83 XML regression tests with embedded binaries + regex assertions).
  Java-side tests (`src/test.slow/java/...`) are GUI/integration — explicitly out of scope.
  Harness: `decomp_test_dbg [-sleighpath D] [-path DIR] [unittests|datatests [names]]`,
  exit code = #failures.
- Key runtime fact: no compiled `.sla` exist in the repo — specs must be compiled with
  the `sleigh` binary (`sleigh_opt -a <dir>`).
- Plan approved: vendor decompile/{cpp,unittests,datatests} + ALL processor spec modules,
  zero-modification policy, top-level Makefile driver, Python package (`kuna`),
  `tools/sync_upstream.py`, STAGE_MAPPING.md. Upstream rev: cef869af04c474...1644.
- Scaffolding written (.gitignore, README.md, UPSTREAM.md, this file).
- Baseline build of the pristine tree started in /tmp/kuna-baseline (parity oracle).
