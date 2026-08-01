# 02 — Lift & flow recovery

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p2_lift
```

This phase turns an entry address into a function: raw p-code ops for every
reachable instruction, a basic-block graph over them, a call-spec record per
call site, and — the centerpiece — a recovered `JumpTable` for every indirect
branch that is really a switch. The driver is
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (follow_flow_on_fd)`
(the C++ `Funcdata::followFlow`): it constructs a
`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs (FlowInfo)` over the fresh
`Funcdata`, applies the architecture's flow options (default
`error_toomanyinstructions`, instruction budget 100000 —
`decompiler/crates/kuna-decomp/src/infra/architecture.rs (reset_defaults_internal)`),
runs op generation + block generation, maps each recovered jump table's
addresses onto block out-edges (`JumpTable::switch_over`), and finally computes
the dominator tree (`structure_reset`) the SSA phase requires. `FlowInfo`
reaches the architecture only through the
`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs (FlowEnvironment)` trait; the
live implementation is
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (ArchFlowEnv)`.

Option defaults and flip guidance for every option named below live in the
generated catalog ([docs/options.md](../options.md)); the rows are defined in
`decompiler/crates/kuna-decomp/phases.toml` and the intentional
default-divergences are DIV-3/4/13/14 in `docs/history.md`.

> Scope note: `decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs
> (Funcdata::resolve_in_flow)` is, despite its name, the union-field
> `resolveInFlow`/`resolveTruncation` dispatch — type-recovery machinery
> described in chapter [05 — Types](05-types.md). It lives in this folder only
> for file-lineage reasons and plays no part in flow recovery.

## 2.1 Instruction following

**Two-phase model.** `flow.rs (FlowInfo::generate_ops)` produces every raw
p-code op into the op bank's *dead list*, instruction by instruction;
`flow.rs (FlowInfo::generate_blocks)` later organizes them into basic blocks
(§2.2). The simple case is a worklist: pop an address off the fall-through
stack, decode one instruction through the SLEIGH translator
(`flow.rs (FlowInfo::process_instruction)`), record it in the `visited` map
(address → first-op sequence number + byte length), classify the emitted ops'
control flow (`flow.rs (FlowInfo::xref_control_flow)`), and push the
fall-through successor plus any branch targets (`flow.rs
(FlowInfo::new_address)`). RETURN and BRANCH end fall-through; CALL and CALLIND
are treated as fall-through ops; a BRANCHIND is parked on `tablelist` for
jump-table recovery (§2.3). A branch whose target is a *constant* is a branch
internal to the instruction's own p-code (`flow.rs (FlowInfo::find_rel_target)`);
ops beyond the deepest internal branch time are dead and deleted
(`delete_remaining_ops`). The op-creation and classification order here is
observable — it fixes the SeqNum allocation every later phase keys on.

**Decode scratch storage.** Every SLEIGH translation checks out a parser
context from the engine-local pool
(`decompiler/crates/kuna-sleigh/src/sleigh.rs (Sleigh::checkout_context)`).
Checkout resets the parse state, addresses, context words, commit records, and
root node; each child node is reset before it is allocated. The state arena and
its allocation capacities are retained across instructions. Simultaneously
live main-instruction, `inst_next2`, and delay-slot resolutions hold distinct
contexts, and a guard returns each context after successful translation or an
error. This is scratch reuse only: instruction results and addresses are not
cached, and the bytes and painted context are resolved on every translation.

**Decode-error policy** (`flow.rs (FlowInfo::handle_decode_error)`). An
unimplemented instruction is, per the flags, treated as a NOP
(`ignore_unimplemented`), re-thrown (`error_unimplemented`), or replaced by an
*artificial halt* that truncates flow at that point. Undecodable bytes (bad
data) halt-truncate or throw; flow past the instruction budget throws by
default (`error_toomanyinstructions`); a branch outside the flow bounds is
recorded on the `unprocessed` list (`flow.rs (FlowInfo::new_address)`, via `handle_out_of_bounds` for the warn/throw policy); flow into the
middle of an already-decoded instruction is a *reinterpretation* (warn, or
throw under `error_reinterpreted`). An artificial halt
(`flow.rs (FlowInfo::artificial_halt)`) is a synthesized RETURN annotated with
its cause (`unimplemented`/`badinstruction`/`noreturn`/`missing`), so the CFG
always terminates cleanly and the printer can attribute the truncation.

**Call sites.** Each CALL gets a `FuncCallSpecs` bound to the op
(`flow.rs (FlowInfo::setup_call_specs)`, shared body `build_call_specs`): the
callee entry is resolved through the symbol table (`FlowEnvironment::query_call`),
the call op's input 0 is replaced by an *fspec annotation* Varnode carrying the
spec's identity, per-call prototype overrides are applied before the query
(`override prototype`), and a declared callee prototype is copied onto the spec
(kuna performs the C++ `ActionDefaultParams`-time copy here, excluding inline
calls, with identical observable result — the analysis path locks callee
signatures before flow runs). A CALLIND keeps its computed target as input 0
(`setup_callind_specs`) unless a previous decompilation pass planted an
indirect override (de-indirection), which converts it to a direct CALL before
the spec is built. `flow.rs (FlowInfo::check_for_flow_modification)` then
applies the callee's flow effects: an *inline* callee queues the op for
injection; a *no-return* callee gets an artificial halt planted directly after
the call plus the `"Subroutine does not return"` warning, so flow never runs
off past the call (§2.4 covers who supplies the no-return facts).

**Inlining** (`flow.rs (FlowInfo::inline_sub_function)`). The callee's p-code
is generated by a nested `FlowInfo` over a fresh callee `Funcdata`
(`inline_flow`), then woven in by one of two models. The *EZ model* — the
callee is a straight-line leaf (no call or branch ops, `check_ez_model`) —
clones the body re-addressed to the call site and deletes the call: the inline
is invisible to addressing. The *hard model* clones the body at its original
addresses, replaces each callee RETURN with a BRANCH to the op after the call
site, and rewrites the CALL itself into a BRANCH to the callee entry; it is
refused (`test_hard_inline_restrictions`) when there is no op to return to or
the return address equals the call address. A recursion set
(`inline_recursion`, forwarded into nested flows) stops a function from being
inlined into itself; the failure mode of every refusal is the same — the call
stays a call — with a per-cause warning (`"Could not inline here"` for
recursion; distinct no-fallthrough / return-address messages from
`test_hard_inline_restrictions`; a missing callee body refuses silently).

**P-code injection.** Three substitution kinds run from a queue drained during op
generation (`flow.rs (FlowInfo::inject_pcode)`); the user-op and call-fixup kinds
share one weave (`flow.rs (FlowInfo::do_injection)`), while inlining uses its own
clone weave (above): emit the payload's p-code at the dead-list
tail, classify its control flow, optionally mark it *incidental copy*, splice
it after the original op, repoint the target map, and destroy the original op.

Classification queues an op the moment it is decoded, so the drain must run once
per flow-discovery round or the ops queued by a later round are dropped: the
queue is drained after the initial fall-thru phase (`generate_ops`) and again
after every jump-table round (§2.3), and a drain clears the queue. Which round
found a block therefore has no bearing on whether its injections are applied —
a spec-declared eraser such as ARM's `setISAMode` `<callotherfixup>` removes the
op uniformly, whether the block was reached by fall-thru or only through a
recovered switch table.

- **Injection library.** `decompiler/crates/kuna-decomp/src/p2_lift/pcodeinject.rs
  (PcodeInjectLibraryBase)` holds the payloads — `<callfixup>`,
  `<callotherfixup>`, and executable-p-code snippets — decoded from the
  compiler/processor specs; their SLEIGH source bodies are compiled to p-code
  templates by `decompiler/crates/kuna-decomp/src/p2_lift/inject_sleigh.rs
  (parse_inject)` and emitted at a concrete address through
  `inject_sleigh.rs (SleighInjectEngine)`.
- **User ops.** `decompiler/crates/kuna-decomp/src/p2_lift/userop.rs
  (UserOpManage)` manages the CALLOTHER black-box ops (unspecialized, datatype,
  volatile, segment, jump-assist, injected). A CALLOTHER whose user op is
  *injected* is queued during classification and replaced by its
  callother-fixup body (`flow.rs (FlowInfo::inject_user_op)`) — e.g. the ARM and
  MIPS `setISAMode` no-op, which dead-code elimination then removes. A user op
  with no declared fixup is not injected and survives as a black box the printer
  renders as a call; that is the intended rendering for unimplemented semantics
  (ARM `DataMemoryBarrier`, the coprocessor family), and only a *declared* fixup
  makes disappearance the correct outcome.
- **Call fixups.** A call spec carrying an inject id has its CALL/CALLIND
  replaced by the named call-fixup payload
  (`flow.rs (FlowInfo::inject_sub_function)`); the payload's parameter shift is
  transferred to the call spec created inside the woven body, and a nested call
  to the same fixup entry is cycle-broken (it must not re-inject).

## 2.2 CFG construction

Blocks are built only after *all* ops exist — the deferred second phase
(`flow.rs (FlowInfo::generate_blocks)`). First every referenced-but-undecoded
address (out-of-bounds targets) gets an artificial halt so branches always have
a landing op (`fillin_branch_stubs`). Then `collect_edges` walks the dead list
pairing branch ops with their target ops — a BRANCHIND contributes one edge per
recovered jump-table entry, deduplicated, and contributes *no* edges when no
table was recovered (the partial-flow "assume no branches out" rule);
`split_basic` cuts the list into blocks at the `startbasic` marks planted
during classification; `connect_basic` materializes the edges. If the entry
block acquired an in-edge (a loop back to the function start), a fresh empty
entry block is prepended so the entry is always in-degree 0. Jump-table
recovery runs *between* the phases (§2.3) precisely because it needs blocks and
SSA over a function whose blocks are not built yet — it gets them on a clone.

The same machinery serves the restart loop: when a late pass requests a
restart, `decompile_drive.rs (run_pipeline)` re-follows flow on the cleared
`Funcdata` (`refollow_flow`; per-function overrides survive the clear, recovered
jump tables do not — they are re-recovered) and re-runs the action pipeline, at
most 8 cross-flow restarts before keeping the last analyzed IR.

**(angr) Tail-call jumps — `option tailcalljump`, default on (DIV-14),
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_tailcalljump.rs
(kuna_is_tail_call_branch)`.** At `-O2` a "call X then return" tail compiles to
a direct `jmp X`. Decision rule: a `CPUI_BRANCH` whose direct machine-address target is the
entry of a *known function* (including a PLT thunk) that is not the current
function's own entry is a tail call. The rewrite lives in the BRANCH arm of
`flow.rs (FlowInfo::xref_control_flow)`: the BRANCH becomes a CALL with a full
call spec, an artificial RETURN is planted after it (unless the callee is
no-return, whose halt was already planted), and a
`tailcalljump: recovered tail call` warning makes the introduced call
attributable. Without it the follower walks *into* the PLT thunk, whose body is
an indirect GOT jump; jump-table recovery fails on it and the function renders
a `(*dat_...)(...)` computed call with a `"Treating indirect jump as call"`
warning. Two datatests (Long double #1/#2) opt out per-test.

**(kuna) Stack-probe loops — `option stackprobeloop`, default on (DIV-3),
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_stackprobeloop.rs
(RuleStackProbeLoop)` (from Ghidra issues GH-8017/6858).** gcc's
stack-clash/`-fstack-check` prologue probes a large frame one page at a time,
leaving the post-loop stack pointer as a self-referential phi
(`PHI = PHI - page`) the spacebase tracker cannot resolve — every post-loop
local renders as `&pxVar[-0x1000]` noise and argument stores at unmatched
offsets vanish from calls. The rule (it runs in the simplification pools but is
stack-pointer normalization, so it is specified here) matches the exact shape —
a two-input stack-pointer `MULTIEQUAL` whose back edge subtracts the page
constant and whose loop exit compares against a stack-relative limit — and
rewrites the phi to the value the exit comparison pins:
`INT_ADD(SP_in, limit_const - page)`. Inert on functions without a probe loop.

**(kuna) V850 indirect branch — `option v850indirectbranch`, default off,
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_v850indbranch.rs
(kuna_is_v850_indirect_jmp)` (from Ghidra issue GH-8817).** The V850 SLEIGH
spec lifts `jmp [reg]` to a CALLIND, so a compiler switch dispatch never
reaches jump-table recovery (which is gated on BRANCHIND). When on, a CALLIND
through a named non-PC hardware register is reclassified to BRANCHIND at the
top of `xref_control_flow`. Kept opt-in per program because the same pattern is
a genuine computed call on other architectures.

## 2.3 Jump tables & switch recovery

A switch exists in the output only if a BRANCHIND op carries a recovered
`decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs (JumpTable)` — a map
from index values to code targets, attached to the op, with per-target case
labels. Everything in this section exists to manufacture, verify, or rescue
that artifact; when all of it fails the BRANCHIND is demoted to a CALLIND and
the switch (and any loop containing it) is destroyed.

### The recovery stage: a reduced sub-decompilation at flow time

The BRANCHINDs parked on `tablelist` during op generation are recovered before
block building, in a loop that re-fills flow from each new table's targets
(`flow.rs (FlowInfo::generate_ops_with_jumptables)`). Each round of that loop
ends by draining the pending p-code injections (§2.1) — the newly reached blocks
queue their own, and an injected body can itself introduce indirect branches, so
the loop re-runs while `tablelist` is non-empty. The address computation
feeding a raw BRANCHIND is unusable as lifted — it must be simplified first, but
the function has no blocks or SSA yet. So each attempt runs as a **reduced-tree
sub-decompilation on a clone**
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::stage_jump_table)`, C++ `stageJumpTable`): the raw ops and existing
tables are cloned into a partial `Funcdata` (`"@@jumprecovery"`), its blocks are
built against the parent flow's `visited` snapshot, and the reduced
`"jumptable"` action set — heritage plus the simplification core, no
structuring — is run over it (`decompile_drive.rs (run_jumptable_pipeline)`).
The model is then recovered against the *partial*'s BRANCHIND and the finished
table is written back keyed to the real op. Two pre-checks bound the attempt:
`funcdata_block.rs (Funcdata::early_jump_table_fail)` backtracks up to 8 ops
through value-preserving arithmetic looking for a computation the recovery can
never emulate -- but its only failing arm (the uninjected-CALLOTHER
classification) is a stubbed loss, so in live kuna the check always passes, and `funcdata_block.rs (Funcdata::test_for_return_address)`
recognizes a BRANCHIND whose input chains back to the saved return address —
that is a return, not a switch (`RecoveryMode::FailReturn`).

Failure demotes: `flow.rs (FlowInfo::truncate_indirect_jump)` rewrites the
BRANCHIND to a RETURN (`fail_return`, warning `"Treating indirect jump as
return"`) or to a CALLIND with an artificial RETURN after it (`fail_normal`,
warning `"Treating indirect jump as call"`; `fail_thunk` silently; a
CALLOTHER-computed target additionally marks the halt no-return) (unreachable today: nothing produces the CALLOTHER failure, see above). This is the
failure mode every rescue below is fighting for: one unbounded table turns a
switch into an opaque computed call.

### The JumpBasic model

The base recovery (`jumptable.rs (JumpBasicModel::recover_model_basic)`) is
Ghidra's JumpBasic: derive a *normalized switch variable* plus a *value range*
such that emulating the address computation for each in-range value enumerates
the case targets.

1. **Path meld.** Starting from the BRANCHIND input, walk the defining
   expressions backwards, depth-first, pruning at calls/phis/constant-free ops
   (`jumptable.rs (JumpBasic::find_determining_varnodes)`). The
   `jumptable.rs (PathMeld)` intersects all paths into the sequence of Varnodes
   *common to every path* — the candidates for the switch variable — ordered
   from the branch backwards.
2. **Guards.** `jumptable.rs (JumpBasicModel::analyze_guards)` walks up the
   CFG from the branch through at most 2 dominating CBRANCHes; each guard's
   branch condition is pulled back through at most 2 defining ops
   (`jumptable.rs (circlerange_pull_back)`, the op-coupled `CircleRange`
   pull-back, non-zero-mask-refined) into a `jumptable.rs (GuardRecord)`: a
   circular value range the guarded Varnode must lie in for control to reach
   the switch. A guard applies to a candidate if they are literally the same
   Varnode, quasi-copies of one base value, duplicate calculations, or loads of
   the same location (`jumptable.rs (GuardRecord::value_match)`).
3. **Range.** For each meld candidate, `jumptable.rs
   (JumpBasicModel::calc_range)` seeds a range from what the Varnode itself
   proves (a constant; a boolean output = {0,1}; an AND-mask bound + power-of-2
   stride from the non-zero mask) and intersects every matching guard range;
   ranges still larger than 0x10000 are assumed positive.
   `find_smallest_normal` picks the candidate with the smallest reaching range
   as the normalized variable — refusing a 1-byte, 256-value candidate unless a
   table LOAD lies between it and the branch (a bare byte is not evidence of a
   switch). One special case: if the meld is a single read-only Varnode, the
   "switch" is a jump through a read-only pointer; its value is read from the
   load image and the table has one entry.
4. **Accept or rescue.** If the chosen range exceeds `max_jumptable_size`
   (1024, `architecture.rs (reset_defaults_internal)`), the four kuna bound
   extensions below get one chance each, in order; if none installs a bound the
   model is declined, model 2 is tried, and then recovery fails with
   `"Could not recover jumptable ... Too many branches"`
   (`jumptable.rs (JumpTable::recover_addresses)`).
5. **Enumeration.** `jumptable.rs (JumpBasicModel::build_addresses_basic)`
   emulates the meld path once per in-range value on the one-path syntax-tree
   emulator (`decompiler/crates/kuna-decomp/src/p2_lift/kuna_emulatefunction.rs
   (EmulateFunction::emulate_path)`), masking each result by the architecture's
   function-pointer alignment; with `option jumpload` the table LOADs are also
   recorded as `jumptable.rs (LoadTable)` entries. A sanity pass
   (`jumptable.rs (JumpTable::sanity_check)`) truncates the table at the first
   null target or far target (> 0xffff from the first) with no loaded data
   behind it, rejects it outright if the *first* entry is bad, and classifies a
   1-entry table whose target is null or > 0xffff from the branch as a thunk
   (`"Likely thunk"`). A BRANCHIND sitting behind an already-collapsed constant
   guard is marked *partial* — recovered as far as flow allows, revisited by
   `jumptable.rs (JumpTable::recover_multistage)` (re-recover, restoring the
   saved model and addresses on failure).

**Model 2 — the default-path variant** (`jumptable.rs
(JumpBasicModel::recover_model2)`, C++ `JumpBasic2`). Some compilers merge the
out-of-range path back into the switch by loading a constant "default" target
into the same variable: the failed model-1 meld ends at a 2-input MULTIEQUAL,
one input a COPY of a constant. Model 2 re-runs the model-1 analysis restricted
to the non-constant path and iterates the range *plus* that one extra value
(`jumptable.rs (JumpValuesRangeDefault)`), so the default becomes an explicit
last entry; a dominance check (`check_normal_dominance`) decides whether the
normalization walk can proceed past the join.

**What is deliberately absent.** The CALLOTHER-assisted `JumpAssisted` model
(the `jumpassist` user-op family) and the manual `JumpBasicOverride` model are
unported shells: `jumptable.rs (JumpTable::set_override)` and the
`<basicoverride>` arm of `jumptable.rs (JumpTable::decode)` return errors, and
`recover_model` walks only JumpBasic/JumpBasic2 (Trivial exists only as the label-time fallback). Likewise upstream's
multistage *restart* accounting — persisting a table whose size disagrees at
`matchModel` time and restarting the whole function — is a recorded loss: kuna
keeps the flow-recovered addresses instead (`jumptable.rs
(JumpTable::match_model)`).

### The late check: labels, normalization folding, guard folding

A table recovered mid-simplification may disagree with fully-simplified
dataflow, so the model is re-derived late, against the finished function, by
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionSwitchNorm)`: for each unlabelled table, `match_model` saves the
flow-time model and recovers a fresh instance (preferring a variable whose
range size matches the known table size), then
`jumptable.rs (JumpTable::recover_labels)` computes the *case labels* by
reverse-emulating the normalization chain from the normalized variable back to
the unnormalized one (`jumptable.rs (JumpBasicModel::backup2_switch)`, exact
inversion of at most 1 add/sub and 1 extension per the table's caps); a
non-reversible value labels `NO_LABEL` (rendered as the default). If no model
can be recovered at all but addresses exist from flow, a trivial model labels
the targets by index (`jumptable.rs (JumpModelTrivial)` — each target labeled
with its own address; table size = the block's out-edge count). `fold_in_normalization` then re-points the BRANCHIND
input at the unnormalized variable — the whole address computation becomes dead
code and the header renders `switch(V)` — and records how many bits of `V` the
switch actually consumes. Finally `jumptable.rs
(JumpBasicModel::fold_in_one_guard)` folds each surviving guard CBRANCH into
the switch: its out-of-range edge becomes the switch's *default* edge (adding
the target as a new label-less destination, or marking an existing destination
as default and collapsing the CBRANCH to a constant predicate); a fold clears
the structuring so the new edge is re-structured, and the constant-predicate
residue is severed on the re-run by `ActionDeterminedBranch`. Before
structuring, any table still without a default marks its most-targeted
out-edge as the default (`decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs
(ActionBlockStructure)` via `funcdata_block.rs (Funcdata::install_switch_defaults)`).

### (angr) Lowered-cascade recovery — `option loweredswitch`, default on (DIV-4)

GCC lowers a dense switch over a small variable into a balanced binary-search
tree of compares — no BRANCHIND, so no switch, and Ghidra (like stock angr)
renders a deep if/else chain.
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitch.rs` (port of
SAILR's `LoweredSwitchSimplifier`) detects the cascade and *manufactures* the
artifact. The two halves straddle a restart because the CFG surgery must not
strand phi state:

- **Detect** (`kuna_loweredswitch.rs (ActionLowerSwitchDetect)`) runs late —
  scheduled directly after `ActionSwitchNorm` — on the fully simplified CFG,
  and only reads: it collects the pure-compare blocks, canonicalizes each
  compared variable, groups by variable and takes the most-compared one, finds
  the cascade head (skipping a leading getopt-style `V == -1` sentinel guard),
  and walks the compare tree carrying angr's binary-search interval
  (`kuna_loweredswitch.rs (recover_cascade)`) to collect case→target pairs and
  default votes. Acceptance is deliberately narrow: at least 3 cases and 2
  distinct targets, at most 16 cases, at least one *range* node (a purely
  linear equality chain is a hand-written if/else-if — without this guard the
  flip regressed 10 upstream assertions, DIV-4), exactly one independent
  default *sink* (candidates that flow into another candidate are paths into a
  shared default, counted by a bounded CFG walk — a hand-written cascade whose
  arms land on independent bodies keeps every arm a sink and is declined), and
  the variable must live in register/stack storage. A hit is recorded in a
  **restart-surviving side store** (keyed by function identity, addresses only
  — no IR handles; the store lives on the Action, outliving the `clear()`) and
  a restart is requested.
- **Install** (`kuna_loweredswitch.rs (ActionLowerSwitchInstall)`) runs on the
  restart, scheduled before `ActionHeritage` and gated to heritage pass 0 —
  the pre-SSA window, where edge surgery needs no phi patching.
  `funcdata_block.rs (Funcdata::kuna_install_lowered_switch)` replaces the
  cascade head's CBRANCH with a synthetic `BRANCHIND(V)`, rewires its
  out-edges to the case targets plus default, pushes a hand-built, pre-labelled
  `JumpTable` (signed labels recorded when the recovered variable is signed)
  carrying a `JumpModelTrivial`, and sweeps the orphaned compare spine via
  unreachable-block removal. Heritage then rebuilds SSA over the corrected CFG
  and the ordinary structurer/printer emit the switch.

One repair hook closes the loop: heritage may widen the synthetic BRANCHIND's
storage read and null its input, so `funcdata_block.rs
(Funcdata::kuna_repair_lowered_switch_inputs)` (driven from
`decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs`) re-points
it at the live reaching SSA def of the recorded storage — accepting a
heritage-known input (written, function input, *or constant*) as healthy. The
constant case is load-bearing: `ActionConditionalConst` may legitimately prove
the switch variable constant on a guarded edge, and classifying that constant
as broken made repair and cond-const toggle the same input forever — the fixed
infinite-loop hang on stripped openssh/bash binaries (`tests/hang-repro/`).

### Bound extensions: rescuing an unboundable table

Four gated extensions run, in this order, only when JumpBasic's range exceeds
the table cap; each installs a `[0, N)` bound on a chosen index Varnode and
hands back to the enumeration core — at most re-deriving the path meld and
seeding loop-invariant values (`switchsharedcase`); the enumeration core itself
is never changed, only what counts as a bound.

- **(kuna) `switchmodbound`, default off (from Ghidra issue GH-9191) —**
  `jumptable.rs (JumpBasicModel::kuna_try_modulo_bound_table)` (option shell:
  `decompiler/crates/kuna-decomp/src/p2_lift/kuna_switchmodbound.rs`). Accepts
  an *in-band* bound: the meld path from the BRANCHIND, through realigning ops
  and exactly one table LOAD, reaches an `INT_REM`/`INT_SREM` by a constant `N`
  or an `INT_AND` with a contiguous low mask (`bound = mask+1`); the index is
  re-bound to `[0, N)`, `N ∈ [2, max_jumptable_size]` (default 1024), starting emulation at the already-
  reduced modulo result. Opt-in: on a program whose indirect jump genuinely has
  no modulo bound it can over-bound an unrelated computed jump.
- **(angr) `switchguardbound`, default off —**
  `jumptable.rs (JumpBasicModel::kuna_try_guard_bound_table)` (option shell:
  `decompiler/crates/kuna-decomp/src/p2_lift/kuna_switchguardbound.rs`).
  Accepts an *out-of-band* CBRANCH range guard that guard analysis missed —
  the GCC `sub LOW; ja DEFAULT` dispatch where, on the early partial-function
  run, the guard is still unsimplified x86 flag arithmetic (the pull-back
  extracts no bound) and the index is spilled to the stack between test and
  load (so the guarded Varnode never value-matches). Rather than
  pattern-matching comparison constants, `scan_guard_tree` *evaluates* the
  guard's boolean as a function of a candidate meld index (sibling meld values
  resolved through a linear-offset map) for `v = 0, 1, …` and takes `N` = the
  first value whose routing flips from `v = 0`'s. Opt-in for the same
  over-bounding reason: the guard↔index correspondence is asserted across a
  memory round-trip dataflow cannot prove.
- **(angr) `switchsharedcase`, default on (DIV-14) —**
  `jumptable.rs (JumpBasicModel::kuna_try_loop_carried_guard_table)` (option
  shell: `decompiler/crates/kuna-decomp/src/p2_lift/kuna_switchsharedcase.rs`).
  Rescues the GCC PIC relative-offset table (`target = base +
  sext(load4(base + idx*4))`) whose `lea .rodata` base register is set *before*
  a getopt-style loop while the BRANCHIND sits inside it: the base reaches the
  jump through a loop-header phi, so the meld collapses to the final add and
  the index guard never bounds anything. The walk rebuilds a clean single path
  from the BRANCHIND down to a guard-bounded load index, identifies the base as
  the unique read-only-image constant reachable through the COPY/phi tree, and
  re-runs normalization with the base pre-seeded into the emulator
  (`kuna_emulatefunction.rs (EmulateFunction::seed_varnode_value)`). Slower on
  exactly the functions it rescues; declines restore the saved model.
- **(angr) `switchmultipred`, default on (DIV-14) —**
  `jumptable.rs (JumpBasicModel::kuna_try_multipred_guard_table)` (no
  dedicated module; the option row lives in
  `decompiler/crates/kuna-decomp/phases.toml`). Rescues the dispatch whose
  bound guard is duplicated — "unrolled" — across *multiple* predecessors of
  the BRANCHIND block, the per-path indices meeting in a MULTIEQUAL (angr's
  "abnormal switch case", e.g. an MSVC memmove small-count tail). The same gate
  also arms the upstream-faithful `jumptable.rs
  (JumpBasicModel::check_unrolled_guard)` inside guard analysis (a no-op when
  off), whose lockstep walk only fires when the *same* guard is duplicated on
  every path; when the per-path guards are semantically different (entry
  `count <= 16` vs back-copy `count & 7 != 0`) the fallback evaluates each
  predecessor's guard as a function of its MULTIEQUAL input (same
  first-routing-flip evaluation as `switchguardbound`, trampoline blocks
  peeled up to 4 deep) and re-binds the table to the *union* — the max — of
  the per-path prefixes.

**(angr) `unrolledguard`, default off** — despite the name, not a guard
analysis: a partial-flow tolerance in `flow.rs (FlowInfo::collect_edges)` for
the MSVC optimized-memcpy shape where several *interleaved* tables' case bodies
are only reachable as one another's case targets. kuna recovers tables one at
a time, each in a fresh partial clone that re-clones already-recovered
siblings; the clone's edge collection then hits a sibling case body that was
never decoded into *this* partial's `visited` and throws `"Could not find op at
target address"`, demoting a recoverable dispatch. With the gate on, an
unresolvable recovered-table case-target edge inside a recovery clone is
skipped instead (the same "assume no branches out" shape as the no-table
path). Opt-in because on a truly malformed table it would mask a real missing
target instead of declining.

## 2.4 No-return at lift time

The mechanism is §2.1's halt plant: if `check_for_flow_modification` believes a
call never returns, an artificial noreturn RETURN lands right after the CALL
and flow stops. The *facts* come from
`decompile_drive.rs (ArchFlowEnv::query_call_no_return)` as an OR of three
sources, checked in order:

1. the resolved callee symbol's no-return flag — set by `option noreturn`, by a
   declared prototype, or by the analysis-tier passes (`noreturn_known`, and
   the call-graph fixpoint `noreturn_propagate`) described in chapter
   [01 — Program preparation](01-program-prep.md);
2. **(angr) `noreturn_externmatch`, default on (DIV-13)** —
   `decompiler/crates/kuna-decomp/src/p2_lift/kuna_noreturn_externmatch.rs
   (is_known_noreturn_name)`: the callee *display name* matched against the
   same vendored list the analysis tier uses
   (`decompiler/crates/kuna-analysis/data/ElfFunctionsThatDoNotReturn`,
   build-time-included so the two matchers cannot drift), with the same
   all-leading-underscore strip, global/`std`-only namespace guard, and
   trailing-`*` wildcard-prefix support;
3. **(angr) `noreturn_extern`, default on (DIV-14)** —
   `decompiler/crates/kuna-decomp/src/p2_lift/kuna_noreturnextern.rs
   (matches_noreturn_extern_name)`: an exact-match check against a closed,
   hard-coded ELF no-return name set with the same namespace guard.

Both name matchers exist for the case the address-keyed analysis pass
structurally cannot reach: in an ET_REL `.o`, `__stack_chk_fail` is an
undefined extern — no definition, no address, no PLT — so no address-keyed fact
is ever emitted, and without the match flow runs off the function's end into
inter-function alignment padding (`00 00` decoding as `add byte ptr [rax], al`),
swallowing neighbour functions in garbage. On a normal dynamically-linked ELF
the proto flag is already set and the OR is a no-op. This *removes code* by
design — the fall-through past a matched call is dropped as unreachable — so
the match surface is kept deliberately narrow (exact names, closed lists, no
class methods): the failure mode of a false positive is truncating live code
after a returning callee that happens to share a listed name.

## 2.5 Arch quirks

**(kuna) SPARC struct return — `option sparcstructret`, default off,
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_sparcstructret.rs
(kuna_is_sparc_struct_ret_trap)` (from Ghidra issue GH-6882).** The SPARC ABI
plants an `unimp <structsize>` word after a call to a struct-returning
function; the SLEIGH spec lifts it to an `IllegalInstructionTrap` CALLOTHER
feeding a BRANCHIND, which jump-table recovery can never resolve — so the
function loses its tail to a non-returning CALLIND. The predicate, consulted in
the BRANCHIND arm of `xref_control_flow`, identifies the idiom *positionally*
(pre-SSA the input is not def-linked): walk backwards over the dead list within
the same instruction looking for a CALLOTHER whose user op is named
`IllegalInstructionTrap`. On a match the BRANCHIND is destroyed and the
instruction falls through. Kept opt-in per program: globally it would convert a
*real* trap into silent fall-through on other targets.

**Emulate-function hooks.** `kuna_emulatefunction.rs (EmulateFunction)` is the
lightweight emulator behind every address enumeration in §2.3: a memory state
keyed by Varnode, constants read off the IR, RAM/register reads pulled from the
load image, and exactly *one* execution path — MULTIEQUAL inputs are selected
by which block the previous op came from, LOADs optionally collected as table
records, and any CALL/CALLOTHER is ignored while a nested branch op aborts the
path (the meld guarantees straight-line evaluation). Its one (kuna) extension
is the pre-seeding hook (`seed_varnode_value`) that lets `switchsharedcase`
inject the loop-carried table base — a register value that exists in no load
image — before each path walk.
