# 03 — SSA & simplification

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p3_dataflow
```

This phase owns the **definition web**: the SSA linkage over the op-graph
(heritage — phi placement, renaming, call/return/load/store guards, the
dead-definition gate) and the **simplification fixpoint** that runs over it (the
rule pools, sub-variable flow, conditional-execution collapse, conditional
constants, and the kuna peephole rewrites). Nothing here runs as a standalone
stage: every pass in this chapter is a member of `mainloop`/`stackstall` or the
post-fullloop cleanup, scheduled and repeated exactly as §0.6 describes — SSA is
rebuilt incrementally each mainloop iteration and the pools re-fire between
rebuilds, until Band B reaches mutual quiescence.

Option metadata (defaults, tiers, symptoms, flip guidance) for every option
named below lives in the generated catalog ([docs/options.md](../options.md));
the rows are defined in `decompiler/crates/kuna-decomp/phases.toml` and the
default-divergence measurements are DIV-2/DIV-3 in `docs/history.md`.

## 3.1 Heritage

`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs (Heritage)` is the
SSA construction engine — the port of the upstream `heritage.cc`. It is owned by
the function (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::op_heritage_with_deadline)`) and driven once per mainloop iteration
by `decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionHeritage)`. SSA is therefore built over **multiple passes**, not once: a
*free* Varnode (a value not yet linked to a defining op) becomes *heritaged*
when some pass collects its address range, and each pass increments the engine's
`pass` counter that everything else in this section keys on.

**Per-space staging.** Each address space carries a
`heritage.rs (HeritageInfo)`: a `delay` (how many passes to wait before
heritaging the space at all) and a `deadcodedelay` (how many passes to wait
before dead code may be removed there), both seeded from the processor spec's
per-space values. The registers heritage on pass 0; the stack space is typically
delayed one pass so that indirect references through the not-yet-renamed stack
pointer have a chance to materialize as located varnodes first (the
`heritage-staging` row in `decompiler/crates/kuna-decomp/phases.toml` — latent,
no user assertion). A space whose `delay` has not elapsed is skipped for the
round.

**Address-range worklists.** The unit of work is an address range, not a
varnode. Two disjoint-cover maps drive each pass
(`heritage.rs (LocationMap::add)`): `globaldisjoint` accumulates every range
ever heritaged (with the pass number it first appeared in), and `disjoint` holds
this pass's todo list. Adding a range returns an intersect code — `0` all-new,
`1` partially overlapping an older range, `2` wholly contained in one — and the
driver (`heritage.rs (Heritage::heritage)`) files the range under
`new_addresses`/`old_addresses` flags accordingly. That classification is
load-bearing twice over: only ranges with new addresses get call/return guards
(below), and an *old* overlap is the trigger for the dead-code-delay machinery.
`add` also hands back the size of the element the range ended up inside, which is
the other half of the C++ iterator its callers read, so classifying a range costs
one lookup rather than an add followed by a second search for the same entry; and
the walk that finds the candidate element asks for the *predecessor* of the range
first, which answers both the "step back from `lower_bound`" and the "already at
`begin()`" cases in one descent. Every varnode of every heritaged space is
re-offered to this map on every pass, so it is the most-called map in the phase.

**The simple case.** For each disjoint range, `heritage.rs (Heritage::collect)`
partitions the range's varnodes into reads (free), writes (defined), and
inputs. It walks the loc-tree's bounded half-open `[start,end)` slice in
location order (a wrapped end runs to the current space's end), rather than
scanning every varnode. Writes smaller than the range are widened through a
PIECE concatenation (`normalize_write_size`), reads smaller than the range are
served by a SUBPIECE (`normalize_read_size`), and input holes are filled and
concatenated (`guard_input`). Phi placement then runs the Bilardi–Pingali
augmented-dominator-tree algorithm (`heritage.rs (Heritage::build_adt)`,
`heritage.rs (Heritage::calc_multiequals)`) with a depth-keyed, LIFO-within-depth
priority queue (`heritage.rs (PriorityQueue)`) — the queue order decides
MULTIEQUAL placement order and is therefore observable output — and
`heritage.rs (Heritage::place_multiequals)` inserts a MULTIEQUAL with one free
input per in-edge at the head of every merge block. Renaming is the classic
Cytron et al. dominator-tree stack walk
(`heritage.rs (Heritage::rename_recurse)`): reads take the top of the
per-address `VariableStack`, writes push, and the walk pops on exit. A read
whose stack is *empty* has no reaching definition — it is materialized as a
formal **input varnode** of the function; this is how registers read before
being written become parameters-in-waiting for phase 04. One carve-out: an
INDIRECT and the op it wraps happen "at the same time", so an op whose renamed
read would resolve to its *own* INDIRECT output takes the next value down the
stack (or a fresh input) instead (`heritage.rs (op_from_const)`). After a block's
own ops are renamed the walk fills the in-edge slots of each successor's leading
MULTIEQUALs; it reads only that leading run (phi ops are always at the head of a
block and the walk stops at the first op that is not one), so it collects the run
off the block's intrusive op list rather than materializing every op of every
successor once per CFG edge per pass.

**Materializing an input over existing pieces (kuna, DIV-50).** The input a
stack-empty read materializes may land on storage that already holds input
varnodes. Upstream refuses that outright — `Funcdata::set_input_varnode` raises
`Overlapping input varnodes` and the function is abandoned with no body at all.
The reachable case is `guard_input`'s own residue: it tiles a partially-input
range with input pieces, marks each piece *write-masked* so `collect` stops
seeing them, and represents the range by the PIECE concatenation instead. When
the rule pools later fold that PIECE away and a new free read of the full range
arrives on a subsequent pass, the read is asking for exactly the value those
pieces still hold. `kuna_inputtile.rs (new_tiled_input)` therefore
completes the tiling (creating an input for any gap, as `guard_input` does) and
folds it into one full-size input with
`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs (Funcdata::combine_input_varnodes)`, which
destroys the pieces, rewrites each concatenating PIECE into a COPY, and repoints
every other reader at a SUBPIECE of the new whole. Only write-masked pieces
fully contained in the request are folded — a write-masked varnode is never
pushed onto a `VariableStack`, so no stack can be left holding a destroyed id —
and any other overlap still raises the upstream error.

**Phi-range granularity (refinement).** When a range is bigger than 4 bytes and
no single write covers it (`size > 4 && maxwritesize < size`), the range is
split at every varnode boundary observed inside it before phis are placed
(`heritage.rs (Heritage::refinement)`): ranges over 1024 bytes are never
refined, and a 1-byte/3-byte adjacent split is healed back to 4
(`remove13_refinement`). Refinement rewrites the disjoint covers (local and
global) in place and re-enters the walk at the first partition. Its inverse
exists too: when a *larger* range arrives over addresses already heritaged at a
smaller size, the stale MULTIEQUAL/INDIRECT/return-COPY markers from the earlier
pass are deleted and the old outputs re-derived as SUBPIECEs of the new full
range (`heritage.rs (Heritage::remove_revisited_markers)`).

**Call and return guards.** For ranges with new addresses, data-flow across
call sites is made explicit before renaming
(`heritage.rs (Heritage::guard_calls)`). Each call spec is asked what effect the
call has on the (callee-translated) range (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncProto::has_effect)`):

- *unknown effect* or *return address* → an INDIRECT op re-defines the range
  across the call, so its lifetime honestly spans the call site; if the range
  is address-tied the INDIRECT output is `addrforce`d (kept alive against
  dead-code) — the alias guard a call casts over memory it might touch through
  a pointer;
- *killed by call* → an INDIRECT *creation* (a definition from nothing) whose
  output is the potential return value, registered as an output trial when the
  call's output recovery is active;
- input-active call → a fresh varnode at the range is appended to the CALL as a
  tentative argument and an input trial registered — this is where register and
  stack arguments physically join the call op (chapter 04 judges the trials);
- a callee returning a struct into locked stack storage materializes the
  delayed CALL output and SUBPIECEs/PIECEs it into the range
  (`heritage.rs (Heritage::try_output_stack_guard)`).

**Narrowing the killed set to the callee's own writes.** A `<killedbycall>`
block in a compiler spec is a statement about the *convention*, not about any
particular callee, and there are callees the convention does not describe. The
one that costs a reader most is the i386 get-PC thunk gcc emits for every PIE
that reaches a global: `mov ebx,[esp]; ret`, four bytes that write `EBX` and
nothing else, called in place of a convention-abiding call precisely because it
is cheaper. `x86gcc.cspec` puts `ECX` and `EDX` in `<killedbycall>`, so a value
the caller loaded into `EDX` before that call becomes an INDIRECT creation and
every later read of it prints as a local the function never assigns.

Under `option calleepreserves` the call guard consults the callee's own
instructions instead. The evidence is the bounded body walk chapter 04 already
takes for the call-output seam (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_rustabi.rs
(probe_callee_return_writes)`): from the callee's entry it follows fall-through
and resolved machine branch targets, ends a path at a `RETURN`, and declares
itself *incomplete* — proving nothing — at a nested call, an unresolved
`BRANCHIND`, an undecodable instruction, or its instruction budget. A complete
walk that records no write to the range downgrades *killed by call* to
*unaffected* for that one call, so no INDIRECT is planted and the caller's value
flows across (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleepreserves.rs
(callee_preserves_range)`). The output-active arm is skipped for the same range:
a register the callee provably never writes cannot be carrying its return value
either, so registering an output trial for it would put the clobber straight
back.

Absence of a recorded write is not on its own enough to act on, and the second
half of the test is what keeps the rule off a body that is not really a body. A
summary with no writes at all is the maximal claim — *every* register survives
this call — drawn from the weakest possible reading, and a one-byte `ret` is
what a stub, a placeholder and a misidentified entry all decode to. So the
callee must also have written a register the model itself marks `<unaffected>`,
excluding the stack pointer, which every `RET` writes
(`kuna_calleepreserves.rs (body_departs_from_convention)`). That is the
signature of the hand-rolled helper the rule exists for — the get-PC thunk's
`EBX` is callee-saved, so the convention is already not a description of it —
and it is a positive finding rather than an absence. Only a processor-space
range is ever answered, because a callee's memory writes are `STORE`s through an
address the walk cannot follow; only *killed by call* is downgraded, never
promoted; and a prototype carrying its own effect-record override has had a
deliberate statement made about it and is left alone.

**Partial-range call overlap.** A heritaged range can be strictly *larger* than
the ABI storage it contains — the characterization is `ContainedBy` rather than
`ContainsJustified`, so none of the whole-range arms above apply. This is
routine on x86-64: SLEIGH models `PXOR`, `POR`, `PAND`, `MOVDQA`, `MOVDQU`,
`MOVQ`-to-xmm and `ORPD` as a single 128-bit write to the whole XMM register, so
the range is never partitioned by refinement (whose gate is `size > 4 && maxw <
size`) and the 8-byte parameter and return entries inside it are invisible.
Under `option calloverlap` two dedicated guards recover them.

On the input side (`heritage.rs (Heritage::guard_call_overlapping_input)`), the
biggest input entry contained in the range is located, its address translated
from the callee's to the caller's perspective, and a SUBPIECE inserted before
the CALL that truncates a fresh whole-range varnode down to that entry; the
truncated varnode is registered as an input trial and appended to the CALL.
Chapter 04's trial machinery then judges it exactly as it judges a whole-register
trial — the guard *proposes* storage, it does not assert an argument.

On the output side (`heritage.rs (Heritage::try_output_overlap_guard)` and
`Heritage::guard_output_overlap`), the biggest contained return entry becomes an
INDIRECT *creation* at the call, and the bytes of the range on either side of it
become further INDIRECT creations that are PIECEd back around it, so the range
as a whole still has a definition at the call while the return entry alone
carries the output trial. When that succeeds the range's effect is downgraded to
*unaffected* so no second guard fires over the same bytes. Note this is not the
same construction as the locked-stack-output case above: the register form makes
every piece an indirect creation, where the stack form pulls the flanking pieces
off a value that already existed before the call.

The level selects how much of that runs: at `off` both branches are inert and a
partial-range slice of an argument or return register at a call gets no guard,
which is what kuna shipped before the option — the observable symptom is a call
rendered with missing arguments and a return value read from a stale pre-call
definition. At `in` only the input guard runs, which recovers the argument but
leaves the return value stale; at `full` both run, which is upstream Ghidra's
behavior.

`heritage.rs
(Heritage::guard_returns)` symmetrically appends output-trial varnodes to every
live RETURN when the range overlaps the recovered return storage (truncating
via SUBPIECE when the range is bigger, `guard_returns_overlapping`), and — for
*persist* ranges (globals) — inserts an `addrforce` COPY of the range before
each RETURN (`return_copy`), which is precisely what keeps a global store's
def-chain alive through dead-code elimination so `glob = ...` survives to the
output.

**LOAD/STORE guards.** Ranges in the stack space can be aliased by indexed
LOADs/STOREs (`stack[i]`). Once per space per function
(`heritage.rs (Heritage::discover_indexed_stack_pointers)`), the engine walks
the stack-pointer input's descendant tree — accumulating constant `INT_ADD`
offsets, passing through COPY/INDIRECT/SEGMENTOP, and flagging any traversal of
a *non-constant* add or a MULTIEQUAL — and records a guard
(`heritage.rs (LoadGuard)`) for every LOAD/STORE reached on a flagged path,
marking the op `spacebase_ptr`. A guard is born covering the **entire space**
(`LoadGuard::set`: minimum 0, maximum the space's highest offset). A STORE
whose pointer is still a free varnode cannot be classified yet: it is
conservatively marked and queued (`heritage.rs (Heritage::protect_free_stores)`),
and after the pass completes the discovery re-runs and strips the spurious
INDIRECTs from any STORE that turned out not to need a guard
(`heritage.rs (Heritage::reprocess_free_stores)`).

After renaming completes each pass, the value-set analysis narrows every newly
discovered guard to a real `[min,max,step]` window
(`heritage.rs (Heritage::analyze_new_load_guards)`, gated by
`option loadguardrange`, default on): the guards' pointer Varnodes become the
sinks of a `ValueSetSolver` system (the solver itself — constraint
generation, weak topological ordering, widening — is chapter
[05](05-types.md)'s machinery in
`decompiler/crates/kuna-decomp/src/p5_types/rangeutil.rs (ValueSetSolver)`),
one cheap `WidenerNone` solve seeds each guard
(`heritage.rs (LoadGuard::establish_range)`: minimum from the stable range
bound or the pointer base, step recorded only when the partial analysis shows
consistent iteration), and if any guard is still unresolved a full
`WidenerFull` solve finalizes it (`heritage.rs (LoadGuard::finalize_range)`:
a converged range of size in `(1, 0xffffff)` locks the guard —
`analysis_state == 2` — with `highind`-grade min/max/step; a range that wraps
past the stack parameters falls back to the whole space). A range-locked
store guard is what chapter [06](06-variables-and-merge.md)'s
`MapState::addGuard` loops turn into a real array index bound, and the
narrowed windows also shrink the merge tier's untied-call intersection test
to the addresses the op can actually touch. With the option off, guards keep
the maximally conservative whole-space window and are never range-locked —
the pre-port behavior. One upstream path remains unported: the
`highPtrPossible` alias path inside `heritage.rs (Heritage::guard)` is
structurally disabled (its condition is constant false; the `guard_stores`
body behind it is an explicit unreached stub, and `guard_loads` a second,
silent no-op behind the same constant — so the load-guard COPY sinks that
`Heritage::handle_new_load_copies` would mark address-forced are never
created, and it takes its faithful empty early return). The guards' main
consumer is the merge tier's untied-call intersection test (chapter 06);
`RuleIndirectCollapse`'s store-guard branch reads them too.

**The dead-code delay machinery and the dead-definition gate.** Dead-code
removal is only *allowed* in a space once heritage there is past the space's
dead-code delay: `heritage.rs (Heritage::dead_removal_allowed)` is the gate
(`pass > deadcodedelay`), consumed by
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs (ActionDeadCode)`
and by `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs
(RuleEarlyRemoval)` — the checked variant (`dead_removal_allowed_seen`) also
records that removal has now *happened* (`deadremoved`). The reason the gate
exists: a free varnode can surface in pass N+1 at an address already heritaged
in pass N — most commonly a stack location whose aliasing access only became
visible after the stack pointer renamed — and if dead code was already removed
there, its defining stores may be gone. When the driver detects exactly that
(an old-range overlap, `deadremoved > 0`), it fires
`heritage.rs (Heritage::bump_deadcode_delay)`: install `deadcodedelay + 1` for
the space as a **persistent Override**
(`decompiler/crates/kuna-decomp/src/p0_knowledge/overrides.rs
(Override::insert_deadcode_delay)` — it survives `Funcdata::clear`), set the
restart-pending flag, and let the outer drive re-flow the function (§0.6); the
restarted run re-applies the persisted delay to the fresh per-space info before
its first pass (`funcdata.rs (Funcdata::op_heritage_with_deadline)`), so dead
code now waits one pass longer and the aliased store survives. The bump is
self-limiting: if the Override already carries a delay for the space, the bump
is suppressed rather than re-requested — that suppression is what makes the
restart converge instead of looping. The bump machinery records both events into a
throwaway per-call `RestartLog`
(`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_restartlog.rs
(RestartLog)`) that is dropped on return — diagnostic plumbing not yet wired to
the Architecture-owned log — and neither fires during a jump-table
sub-decompilation (the `is_jumptable_recovery_on` guards at the call sites —
the sub-query must not mutate P0, §0.7). The console `deadcode delay` command
exists but is an unwired stub (`kuna-console/src/ifacedecomp.rs
(IfcDeadcodedelay)` returns engine-unavailable); the only live writer of the
Override is `Heritage::bump_deadcode_delay`.

**Free-varnode failure mode.** After `remove_revisited_markers`, a free read
being guarded must have exactly one reader; a free varnode with multiple reads
is an IR invariant violation and `heritage.rs (Heritage::guard)` deliberately
panics carrying the upstream error text ("kuna heritage: Free varnode with
multiple reads") — the
drive catches it at the per-function boundary and degrades to that function's
error record, exactly the route the C++ `LowlevelError` takes. (The port
history briefly downgraded this throw to a skip; with call-argument def-chains
kept alive by dead-code marking it fires zero times across the corpus, and the
faithful throw is restored.)

Two kuna extensions ride on the pass boundary: the per-function watchdog
deadline is probed at each address-space iteration (§0.6 — a stripped-binary
non-convergence spends its time inside heritage, so the pass bails here rather
than at the next action boundary; the abandoned partial pass is never
rendered), and after every pass `ActionHeritage` runs the lowered-switch input
repair (`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::kuna_repair_lowered_switch_inputs)`), which re-points a synthetic
lowered-switch BRANCHIND whose input heritage normalized away (chapter 02). The
repair accepts written, input, *or heritage-known* varnodes as healthy — the
last category is what ended the condconst-vs-repair tug-of-war that once kept
mainloop reporting one change forever on certain stripped binaries
(`tests/hang-repro/README.md`).

## 3.2 The rule pools

A `decompiler/crates/kuna-decomp/src/infra/action.rs (Rule)` is a stateless
pattern→rewrite unit: `get_op_list` declares the opcodes it can fire on
(defaulting to *all* opcodes), and `apply_op(op, data)` either returns 0 (no
match — every guard along the way simply declines) or performs its whole
rewrite and returns 1. Rules are owned by an
`decompiler/crates/kuna-decomp/src/infra/action.rs (ActionPool)`, which indexes
them at registration into a flat per-opcode table (`perop`, insertion order
preserved). One pool sweep visits every op in the function in sequence-number
order through a resumable cursor that survives op deletion (§0.3) — it is
recorded as the last *consumed* `SeqNum`, so the next op is the first optree key
strictly greater than it, which stays valid when the op it named is destroyed.
Resolving that key is an optree search, and the sweep is the decompiler's
innermost loop, so the advance resolves the successor's id and the cursor read
returns it rather than searching a second time for the op the advance already
found; the memo is dropped on every `apply` exit, so a resumed or interleaved
sweep re-searches. For each op the sweep walks its opcode's rule list in
registration order
(`action.rs (ActionPool::process_op)`): disabled rules are skipped (the
upstream `option togglerule` surface writes that per-rule flag), a rule that
fires bumps the pool's change count, a rule that kills the op ends the walk,
and a rule that *changes the op's opcode* rewinds the walk to index 0 of the
new opcode's list — rules see each other's effects mid-op, and that rewind
order is part of the observable output (§0.6). A rule that mutates without
returning 1 is an invariant violation the pool reports as an engine error
message rather than silently absorbing. The **local fixpoint** comes from the
scheduler, not the pool: every pool node carries the repeat flag, so
`action.rs (Action::perform)` re-sweeps the whole function until a sweep makes
no change. There is no bound on the number of sweeps — quiescence is the
contract, and the only backstop against a rule pair that feeds itself forever
is the (kuna) cooperative deadline probed every 1024 op-visits
(`action.rs (POOL_DEADLINE_STRIDE)`, §0.6); exactly one such oscillation has
occurred in kuna's history (the lowered-switch repair, §3.1), and it presented
as mainloop reporting one change per iteration for good.

Three pools exist in the `decompile` tree
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`): **oppool1**, 141 registered rules, sits inside the
`stackstall` repeat-group in mainloop — the main simplification bag;
**oppool2**, 5 rules (`RulePushPtr`, `RuleStructOffset0`, `RulePtrArith`,
`RuleLoadVarnode`, `RuleStoreVarnode`), runs after block structuring in
mainloop's tail — the pointer-arithmetic and stack-variable forms that need
type recovery started and a stable block structure; and the **cleanup** pool,
22 rules, runs once-per-drive after fullloop exits — presentation-form
rewrites that must not perturb the analysis fixpoint. The architecture may
append CPU-specific rules to oppool1 (`universalaction.rs (build_universal_action)`
takes `extra_pool_rules`); the engine currently always passes an empty list
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(Architecture::build_action)`).

The upstream rule set is ported across eight files in C++ definition order —
`ruleaction.cc` split at class boundaries. The map, by dominant theme (named
rules are representative, not exhaustive; a rule's registration row in
`universalaction.rs (universal_sched)` is the authority for its pool and
group):

| File | Theme | Representative rules |
|---|---|---|
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs` | dead-op pruning, term ordering, bit-mask algebra, SUBPIECE motion through phis/INDIRECTs | `RuleEarlyRemoval` (the all-opcode dead-op reaper, gated by §3.1's dead-definition gate), `RuleCollectTerms`, `RuleAndMask`/`RuleShiftBitops`, `RulePullsubMulti`/`RulePushMulti`, `RuleIntLessEqual` (§3.5 compareform), `RuleRangeMeld`, `RulePiece2Zext` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_2.rs` | logical ops through extensions/pieces, double-op fusion, zext elimination | `RuleAndCommute`, `RuleAndCompare`, `RuleDoubleShift`, `RuleConcatShift`, `RuleLeftRight`, `RuleZextEliminate`, `RuleBooleanUndistribute`, `RuleFloatRange` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_3.rs` | boolean normalization, phi/INDIRECT collapse, constant folding, reassociation | `RuleMultiCollapse`, `RuleIndirectCollapse`, `RuleCollapseConstants` (the OpBehavior constant evaluator), `RulePropagateCopy`, `RuleAddMultCollapse`, `RuleSborrow`, `RuleShift2Mult` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_4.rs` | the SUBPIECE/ZEXT/CONCAT commuting family, piece reassembly, stack-var promotion | `RuleSubCommute`, `RuleConcatZext`, `RuleSubCancel`, `RuleHumptyDumpty`/`RuleDumptyHump`, `RuleLoadVarnode`/`RuleStoreVarnode` (oppool2, group `stackvars`), `RuleSwitchSingle`, `RuleCondNegate` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_5.rs` | comparisons against extremal constants, equation solving, the pointer-recovery trio | `RuleLess2Zero`, `RuleSLess2Zero`, `RuleEqual2Constant`, and oppool2's `RulePtrArith`/`RuleStructOffset0`/`RulePushPtr` (all no-ops until `ActionStartTypes` flips `has_type_recovery_started` — chapter 05 owns what they build) |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs` | pointer-op undo, division strength-reduction inversion, cleanup arithmetic | `RulePtraddUndo`/`RulePtrsubUndo`, `RuleDivOpt`/`RuleDivTermAdd`/`RuleSubNormal` (recover `/`, `%` from magic-number multiplies), cleanup-pool `RuleMultNegOne`/`RuleAddUnsigned`/`RuleSubRight`/`RulePieceStructure` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_7.rs` | signed div/mod idioms, segments, pointer flow, predication, float compares | `RuleSignDiv2`, `RuleSignMod2nOpt`, `RuleModOpt`, `RuleSegment`, `RulePtrFlow`, `RuleConditionalMove` (group `conditionalexe`), `RuleFloatCast`, `RuleIgnoreNan` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs` | int↔float conversion recovery, bit-counting booleans, float sign ops, compare splitting | `RuleUnsigned2Float`, `RuleThreeWayCompare`, `RulePopcountBoolXor`, `RuleLzcountShiftBool`, `RuleFloatSign`, `RuleOrCompare`, `RuleFuncPtrEncoding`, cleanup-pool `RuleExpandLoad` |

**Keeping a frame store that only a marker still reads** (`option tiedstorekeep`,
default on). `RulePropagateCopy` rewrites a reader of a `COPY` output to read the
`COPY`'s input instead. When the reader is an ordinary op that is pure gain: the
value is the same, and the `COPY` stays alive for whoever else reads its
location. When the reader is a **marker** — an `INDIRECT` guarding an
address-tied range across a call (§3.1), or a `MULTIEQUAL` at a join — it is
not, because markers never print. Once the marker has swallowed the last
remaining reader, an address-tied `COPY` has no descendants at all and is reaped
as dead, and with it goes the only statement that said where the location's
value came from. `Merge` normally conceals that (chapter 06): it merges the
source's HighVariable into the tied location's, so both print under one name and
the store reads as an assignment to that name. When the merge is DECLINED —
covers intersect — nothing repairs it, and the local's last printed assignment
is whatever preceded the store, typically its initialiser. Upstream already
refuses the propagation when the `COPY` output is `addrforce` ("don't propagate
if we are keeping the `COPY` anyway"), but `addrforce` is set only on heritage's
own guard outputs (§3.1), never on an ordinary frame store. kuna widens that
refusal by one case: the marker is about to take the **last** reader of a
non-`persist` address-tied `COPY` whose input is not itself address-tied and
whose value comes from a call — a `CALL`/`CALLIND`/`CALLOTHER` output, or the
`INDIRECT` that carries the return register across the call site before chapter
04's output promotion rewrites it. Propagating there buys nothing, since the
marker is invisible either way, and costs the store. Every other propagation is
untouched, including into a marker that is not the last reader, out of a
constant, out of a same-location copy, and into any marker over a `persist`
global — a global already has heritage's persist `RETURN-COPY` (§3.1) keeping
its last store printed, so the brake has nothing to add there. `option
tiedstorekeep off` restores upstream's behavior exactly.

**Retyping an op mid-rule.** A rule that rewrites an op in place usually changes
its op-code, and the op-code is not just a tag: `set_opcode` caches the
op-code's *property word* (`unary`/`binary`/`booloutput`/`commutative`/`marker`/
… ) into the op's flags, and every later guard — `is_bool_output`,
`is_commutative`, the pool's eval-type dispatch — reads it back off the op. The
upstream `Funcdata::opSetOpcode` therefore takes a bare op-code and looks up the
architecture's singleton property record (`glb->inst[opc]`); kuna's
`Funcdata::op_set_opcode` takes the already-resolved record, so each rule file
resolves it at the call site. Every one of those call sites goes through the
single canonical port of that table,
`decompiler/crates/kuna-decomp/src/p5_types/typeop.rs (seam_type_op_for)`, whose
per-op-code rows are transcribed field-for-field from the upstream `typeop.cc`
constructors. The seam is **total**: the table's `match` carries no wildcard arm
(so a new op-code cannot enter the enum without the compiler demanding its row),
every registered op-code answers with real property bits, and the one value with
no upstream record — the `CPUI_MAX` sentinel, which is not an operation — yields
a property-less skeleton rather than aborting. This totality is load-bearing
rather than cosmetic: the rule files previously each kept their own partial
whitelist of "op-codes this batch emits" with a `panic!` default arm, and the
copies drifted apart, so a rule that legitimately produced an op-code its file
had not enumerated (`INT_SRIGHT` out of `RuleBitUndistribute`, or a
`FLOAT_INT2FLOAT`/`FLOAT_LESS`/`FLOAT_ADD` phi collapsing through
`RuleMultiCollapse`) unwound the entire decompilation — the caller saw one error
record and no C at all for that function.

**Lowering a non-least-significant truncation.** A SUBPIECE carries a byte
offset, and only the offset-0 form has a C spelling: it is a cast. Every other
offset is a p-code-level slice with no operator in the language, so the printer
falls back to rendering the operation itself — `SUB81(v,7)`, an identifier no
emitted header declares. `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs
(RuleSubRight)` (cleanup pool, registered `subright`) is what keeps that
fallback unreachable in practice: it rewrites `sub(V,c)` into
`sub(V >> c*8, 0)`, synthesizing an `INT_RIGHT` by `c*8` bits ahead of the
SUBPIECE and zeroing the SUBPIECE's offset, so the result prints as the cast of
a shift — the ordinary arithmetic the source wrote. The shift's temporary is
typed `TYPE_UINT` at the input's width so the shift renders unsigned. Three
cases decline. `c == 0` is already least-significant and needs nothing. A
truncation whose input carries a composite (struct/union/array) read-facing
type is marked for special printing instead and rendered as a field extraction
(`sym._2_1_`), because there the slice *is* the source-level operation. And
when output and input are both address-tied and overlap at exactly `c`, the
SUBPIECE is a storage marker that
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionCopyMarker)` will convert,
so rewriting it would destroy the partial-symbol rendering. One refinement
folds a level away: if the SUBPIECE takes the *high* end of its input
(`outsize + c == insize`) and its **only** reader is an `INT_RIGHT`/`INT_SRIGHT`
by a constant, the two shifts are lumped into one — the reader becomes the
least-significant SUBPIECE and the synthesized shift carries `c*8` plus the
reader's amount (so a 4-byte-offset SUBPIECE feeding `>> 5` becomes a single
`>> 0x25`). A lumped `INT_RIGHT` whose combined amount reaches the input width
would evaluate to zero and is declined outright; the arithmetic form clamps to
the sign bit instead, since that is a sign extraction.

Rules registered in the pools but implemented elsewhere: the sub-variable
triggers and split rules (§3.3, `subflow.rs`), `RuleOrPredicate` (§3.4,
`condexe.rs`), the kuna gated rules (§3.5), the double-precision family
(`decompiler/crates/kuna-decomp/src/p5_types/double.rs`, chapter 05), the
constant-sequence and bit-field cleanup rules
(`decompiler/crates/kuna-decomp/src/p5_types/constseq.rs`,
`decompiler/crates/kuna-decomp/src/p5_types/bitfield.rs`, chapter 05), and the
stack-probe-loop phi resolver
(`decompiler/crates/kuna-decomp/src/p2_lift/kuna_stackprobeloop.rs`, chapter
02). A note on the files themselves: their module headers still carry the
port-wave `STUB(...)` inventory from the mid-port merge; the live registration
and rule bodies are complete (the tree's action listing is byte-equal to the
C++ oracle dump, §0.6) — trust the code, not the header prose.

**Erasing the ISA-mode / alignment encoding on an indirect-call target.** On
processors that steal the low bits of a function pointer, the *instruction*
clears them before branching, so SLEIGH lifts the clear as a real p-code
`INT_AND` feeding the `CALLIND` target: ARM/Thumb `blx` goes through
`BXWritePC`, whose body is `local tmp = addr & 0xfffffffe`, and MIPS `jalr`
through `JXWritePC`, whose body is `tmp = -2 & addr`. That AND is machine
bookkeeping, not program semantics — the source performs no bit-clear — and
leaving it in place costs twice: the emitted C asserts an operation the program
never performs, and the mask stands between the `CALLIND` and its pointer
operand, so the pointer-to-code data-type never back-propagates onto the LOAD
that fetched the callee. A masked call renders as
`(*(code *)(*(uint4 *)(p + 0x44) & 0xfffffffe))(p)` where the un-masked form is
`(**(code **)(p + 0x44))(p)`.

`RuleFuncPtrEncoding` erases it. The width of the encoding is **not** a kuna
policy: it is declared per compiler spec by `<funcptr align="N"/>`, decoded into
`Architecture::funcptr_align` as the bit position of `N`'s first set bit
(chapter 00, the P0 knowledge plane), and read live by the rule. The rule fires
only on an exact match — the constant mask must equal `calc_mask(size) & (~0 <<
funcptr_align)`, i.e. all ones above the encoded bits — and rewrites the `INT_AND`
to a `COPY`, which is transparent, so any other reader of the masked value keeps
seeing it. A cspec that declares no `<funcptr>` leaves `funcptr_align == 0` and
the rule is inert, which is why x86/x86-64 keep every `& 0xfffffffe` their
programs really compute. The vendored specs declare `align="2"` (one mode bit)
for the four ARM cspecs, the nine MIPS cspecs, the four Loongarch cspecs and
8051, and `align="4"` (two word-alignment bits) for the five AARCH64 cspecs;
AARCH64's own `blr` masks nothing, so there the rule only fires on a mask the
program itself wrote. `funcptr_align` has two other live readers — the jump-table
model (chapter 02) and the `thumbfuncptr` const-pointer preservation (chapter 05,
§5) — and the three do not interact: this rule only ever removes an `INT_AND`
that a `CALLIND` consumes.

## 3.3 Sub-variable flow

`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs (SubvariableFlow)`
shrinks a logical value out of a larger container: given a *root* varnode and a
bit-mask identifying where the small value lives, it traces the value's flow
forward and backward through the data-flow graph, builds a parallel shadow
graph of placeholder varnodes/ops plus a patch list, and only if the **entire**
flow is expressible at the smaller size commits the rewrite
(`subflow.rs (SubvariableFlow::do_replacement)`) — replacing the wide ops with
logically-sized ones. It is all-or-nothing by construction: any placeholder the
trace cannot legalize aborts the whole transform with no IR change (marks are
cleared, `subflow.rs (SubvariableFlow::do_trace)`).

Six trigger rules in oppool1 (group `subvar`) seed it from ops that *prove* a
smaller logical value exists: `RuleSubvarAnd` (INT_AND by a low mask),
`RuleSubvarSubpiece` (SUBPIECE), `RuleSubvarCompZero` (INT_EQUAL/INT_NOTEQUAL
against a masked constant), `RuleSubvarShift` (INT_RIGHT bringing high bits
down), `RuleSubvarZext`, and `RuleSubvarSext` (the last arming the
sign-extension-invariant mode). The mask's bit-span picks the logical size
(`subflow.rs (SubvariableFlow::new)`): 1/2/3/4 bytes, 8 only when the caller
passes `big`, anything else — including a zero mask or a span over 64 bits —
constructs an invalid engine that traces nothing.

**When it refuses** (`subflow.rs (SubvariableFlow::set_replacement)`), roughly
in decision order (the constant-sext check actually sits in the constant arm
first; the sext size-mismatched-input refusal is bypassed in aggressive mode;
both type-lock refusals exempt `TYPE_PARTIALSTRUCT`): a varnode already visited with a *different* mask (two
inconsistent claims about where the logical value sits); any **free** varnode
(untraceable flow); an `addrforce` varnode of the wrong size (its full
container is pinned live); under sign-extension restrictions, a constant that
does not equal the sign-extension of its masked low part, and any
size-mismatched input or persistent varnode (their high bits cannot be assumed
to be extension); outside flag-sized traces (logical size ≥ 8 bits), a varnode
whose *consumed* bits extend beyond the mask — unless the caller is in
aggressive mode — because outside consumption means the container is probably
one real variable, not a packing; a type-locked varnode whose locked size
differs from the flow size; and for function inputs, no sub-byte flags and no
mask that is not anchored at bit 0 (either would fabricate an input register
slice the ABI cannot name). Terminal ops (CALL/RETURN/BRANCHIND boundaries) do
not refuse but *patch*: the trace records a pull/push patch at the boundary
(`try_call_pull`/`try_return_pull`/`try_switch_pull`/`try_call_return_push`),
and `do_trace` additionally refuses to commit when **zero pull points** were
found — a rewrite whose small value never actually escapes the shadow graph
would churn the IR for no output gain.

Three sibling engines share the file. `subflow.rs (SplitFlow)` (trigger
`RuleSplitFlow`, oppool1) splits a double-sized value into hi/lo lanes through
the `decompiler/crates/kuna-decomp/src/substrate/transform.rs
(TransformManager)` machinery when a SUBPIECE proves the halves live separate
lives. `subflow.rs (SubfloatFlow)` (trigger `RuleSubfloatConvert`, group
`floatprecision`) does the same for a float value carried in a wider float
container, converting constant encodings between formats along the way.
`subflow.rs (SplitDatatype)` (triggers `RuleSplitCopy`/`RuleSplitLoad`/
`RuleSplitStore`, cleanup pool) splits a whole-struct COPY/LOAD/STORE into
per-field transfers using recovered types — described with the type system in
chapter 05, as is lane division (`ActionLaneDivide` in stackstall, over
`subflow.rs (LaneDivide)` (built over `transform.rs (TransformManager)`)).

**Which copies the split declines** (`subflow.rs
(SplitDatatype::test_copy_constraints)`). Upstream refuses a COPY whose input is
a function input, whose input and output are address-tied at the *same* address
(the identity copy a heritage guard leaves behind), or whose input is the lone
output of a LOAD (handled by the LOAD split instead). kuna adds one more (DIV-55):
a COPY whose **output Varnode is read-only** — an address the load image reported
inside a non-writable section — is never split. A store into a read-only range is
not something the program performs, and the split is what makes such a copy
*visible*: whole, its input and output share a HighVariable and
`decompiler/crates/kuna-decomp/src/p6_variables/merge.rs
(Merge::mark_internal_copies)` marks it non-printing; split into one COPY per
array element, the pieces land in different HighVariables and P9 prints a block of
per-byte assignments into a `.rodata` string literal. The copies that reach the
gate in that shape are the `return_copy` guards of §3.1 after a block clone has
rewritten them: `substrate/funcdata_block.rs (CloneBlockOps::build_op_clone)`
copies only the upstream flag subset, which does not carry `return_copy`, and
`CloneBlockOps::patch_inputs` re-inputs the clone from a fresh COPY, so neither
the same-address test nor the flag can recognize the clone for what it is. The
read-only output test does, and it is the property that actually matters. The
same invariant is what `substrate/funcdata_varnode.rs (Funcdata::fillin_read_only)`
warns about (`Read-only address (ram,X) is written`) when `readonlypropagate` is
on; declining the split does not depend on that option.

## 3.4 Conditional execution

`decompiler/crates/kuna-decomp/src/p3_dataflow/condexe.rs
(ActionConditionalExe)` (mainloop tail) removes a CBRANCH that re-tests a
condition an earlier block already decided. The candidate — the *iblock* — must
satisfy the two-block merge condition
(`condexe.rs (ConditionalExecution::verify)`), all read-only tests:

1. the iblock has exactly 2 in-edges and 2 out-edges and ends in a CBRANCH
   (`test_iblock`);
2. both in-paths, walked backward through any chain of single-in/single-out
   blocks, reach the **same** *initblock*, itself two-exit — so the iblock is
   purely a re-join of one earlier decision (`find_init_pre`);
3. the initblock also ends in a CBRANCH, and the two branch conditions are
   provably identical or complementary —
   `decompiler/crates/kuna-decomp/src/substrate/expression.rs
   (BooleanExpressionMatch::verify_condition)` matches the boolean expressions
   structurally (complement flips which path is "true");
4. every op in the iblock other than its branch is removable or movable
   (`test_removability`): no call, no flow-break, no LOAD/STORE/INDIRECT, no
   address-tied output; a MULTIEQUAL's readers must each tolerate the phi being
   pulled back into the predecessors (`test_multi_read` — a RETURN reader only
   in value position, an in-iblock reader only if COPY/SUBPIECE).

If verification passes, `condexe.rs (ConditionalExecution::execute)` rewires
the data-flow — each iblock op's output is replaced per consuming block, with
pulled-back MULTIEQUALs materialized in the post-blocks as needed
(`do_replacement`/`get_new_multi`) — deletes the iblock's ops in reverse order,
and splices the block out of the graph
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::remove_from_flow_split)`). The action loops over all blocks until a
full round makes no change, and refuses to run at all while unreachable blocks
exist. One kuna conservatism: the per-space "has heritage run yet" array the
removability test consults is hard-wired to *false*
(`condexe.rs (ConditionalExecution::build_heritage_array)` — a port seam never
re-wired to the live `Funcdata::num_heritage_passes`), so an iblock op whose
output has **no readers** is always refused rather than trusted once its space
is heritaged; strictly conservative relative to upstream (a collapse is missed,
never wrongly taken). `condexe.rs (RuleOrPredicate)` (oppool1, group
`conditionalexe`) handles the value-form of the same redundancy: an INT_OR
(or INT_XOR) where one operand is provably zero along the path that reaches it (the
`MultiPredicate` zero-slot analysis) collapses to a COPY of the other operand.

**Conditional constants.** `decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionConditionalConst)` (mainloop tail, wrapper over
`decompiler/crates/kuna-decomp/src/p3_dataflow/condconst.rs (condconst_apply)`)
propagates the knowledge a CBRANCH creates: after `x == k` branches, `x` *is*
`k` on one out-edge (and a raw boolean is 0/1 down its two edges). Every read
of the varnode dominated by the constant edge is rewritten to the constant
(`condconst.rs (propagate_constant)`), constants are pushed through ops whose
other inputs are constant by direct evaluation (`condconst.rs (push_constant)`),
and — the phi case — a MULTIEQUAL input arriving on the constant edge is
replaced by a freshly-placed constant COPY in the edge's predecessor block,
but only when excising that edge leaves no alternate data-flow path rejoining
the original value (`condconst.rs (handle_phi_nodes)`; multiple disconnected
edges that flow together downstream get one shared placement).

A block whose last op is a CBRANCH is read as a two-way branch, so
`condconst.rs (condconst_apply)` skips one carrying fewer than two out-edges rather
than indexing off the end of its edge list. The `funcboundflow` truncation used to
produce one; the guard keeps a malformed graph from killing the process before the
pass that malformed it can be identified, but it does not by itself make the emitted
C right.

(kuna) **condexeplace** — GH-9203: that materialized COPY could land inside a
*loop* predecessor block, re-executing a supposedly loop-invariant `= 0` every
iteration and malforming the do/while. Under the gate,
`condconst.rs (handle_phi_nodes)` declines the placement when the predecessor
has a loop in-edge and leaves the phi edge untouched. Settable
`condexeplace` (`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_condexeplace.rs`
owns the option surface; the gate itself is the guarded block in
`handle_phi_nodes`); shipped default **on** per
`decompiler/crates/kuna-decomp/phases.toml` (DIV-3 — corpus-neutral, 0 of 675
assertions changed); `option condexeplace off` restores the upstream placement.
Catalog: [docs/options.md](../options.md).

## 3.5 kuna peephole rewrites

Six kuna-added transforms live beside the upstream rules, each resolving an
open upstream issue (the sanctioned `(kuna)`-tag exception: their
`phases.toml` rows record `ghidra-upstream` as lineage because an upstream
*issue*, not upstream code, specified them — the GH number is the row's
`issue`). All share one wiring pattern: the rule is registered with its own baked-in
enable flag off (the pool still dispatches it), so each `apply_op` defers
per-op to the live gate on the per-function architecture snapshot (e.g. `kuna_booleanmask.rs (RuleBoolSignShift::apply_op)` testing
`fold_boolean_mask`) — which makes them subject to the flag-copy hazard of
§0.5 — and every gate's engine default is set in
`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`, mirrored by the `default` column of
`decompiler/crates/kuna-decomp/phases.toml` (the source quoted below; the
DIV-2/DIV-3 rows of `docs/history.md` carry the ablation evidence). With a
gate off, the rule returns 0 unconditionally and output is byte-identical to
upstream. Full option metadata: [docs/options.md](../options.md).

**addcarrychain** (GH-8913) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_addcarrychain.rs
(RuleAddCarryChain)`, oppool1, fires on PIECE. Pattern: the reassembly of an
8-bit carry-chained add, `PIECE(hi, lo)` where `lo = INT_ADD(a, b)` and
`hi = INT_ADD(hipart, carry)` with `carry` the carry of `(a, b)` — either a raw
INT_CARRY or its const-folded `INT_LESSEQUAL((-b) & mask, a)` form, matched
through CAST/COPY chains. Rewrite: one wide `INT_ADD(PIECE(hipart, b),
ZEXT(a))`, recovering the single 16-bit addition the 6502-class ADC pair
implements. Settable `addcarrychain`, shipped default **on** (DIV-2).

**booleanmask** (GH-1282) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_booleanmask.rs
(RuleBoolSignShift)`, oppool1, fires on INT_SRIGHT. Pattern:
`(b << k) s>> k` with the same non-byte-aligned `k` on both shifts (the
byte-aligned case already belongs to `RuleLeftRight`), where the pre-shift
value's known-nonzero mask fits entirely below the shifted-out bits — i.e. `b`
is a boolean being smeared across the word. Rewrite: `INT_2COMP(b)` (`0 - b`,
giving 0 or all-ones), which the surrounding compare rules then clean to a
plain boolean test. Settable `booleanmask`, shipped default **on** (DIV-2).

**simdlane** (repipe `simd-constant-string-initializer`) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_simdlane.rs
(RuleSimdShuffleLane)`, oppool1, fires on SUBPIECE. *Pattern:* a one-byte lane
read of a byte-shuffle user op whose mask is constant — `SUBPIECE(CALLOTHER
pshufb(src, m), k)` with `m` a constant Varnode. `pshufb` has no p-code
semantics (the x86 SLEIGH spec models it as an opaque CALLOTHER over a 16-byte
value), so after `ActionLaneDivide` splits the vector consumers into byte lanes
every lane read is a SUBPIECE of something nothing downstream can see through,
and neither `RuleSubExtComm`/`RuleSubZext` nor copy propagation can collapse
them. *Rewrite:* the instruction is a pure permutation with an exact per-lane
definition once the mask is known, `dst[i] = (m[i] & 0x80) ? 0 : src[m[i] &
(N-1)]`, so the lane read becomes `SUBPIECE(src, m[k] & (N-1))`, or a COPY of
the constant `0` for a zeroing mask byte. It is an identity, not a heuristic.
Once every lane read is re-anchored on the source the CALLOTHER loses its last
reader; for the standard byte-broadcast idiom (`pxor xmm2,xmm2; pshufb
xmm0,xmm2` — an all-zero mask) all sixteen lanes resolve to `SUBPIECE(src, 0)`
and collapse into one value. *Bounds/failure:* only a user op the architecture
registered under a shuffle name (`kuna_simdlane.rs (SHUFFLE_USEROP_NAMES)` =
`pshufb`, `vpshufb`; the ids are resolved once per program in
`Architecture::build_arch_handle` and carried on the `ArchContext`, since a Rule
cannot reach the userop table); only the three-input form whose two operands and
output all have the vector width; only widths 8 (MMX) and 16 (SSE); only a
ONE-BYTE lane read, because a wider SUBPIECE of a shuffle is a concatenation of
lanes and not another SUBPIECE. A mask wider than eight bytes does not fit a
`uintb` offset and is accepted only at value `0`, where the offset IS the whole
value and every lane byte is provably zero — the broadcast mask, and the only
wide constant mask the engine constructs. Settable `simdlane`, shipped default
**on**.

**flagcompare** (GH-1276 / GH-8777) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_flagcompare.rs`, two rules
under one gate, for architectures that model condition flags as explicit bits.
`RuleBoolSignLess` (fires on INT_SLESS): a boolean shifted into the sign bit
and tested with `s< 0` — where the operand's nonzero mask is exactly the bit
landing in the sign position — becomes `b != 0`. `RuleSborrowGe` (fires on
BOOL_AND/BOOL_OR): the `N == V` signed-comparison idiom — the
XNOR of the result sign of `V - K` with `SBORROW(V, K)`, in either its
AND-of-ORs or OR-of-ANDs lowering — becomes `INT_SLESSEQUAL(K, V)` (`V >= K`
as the source wrote it). Settable `flagcompare`, shipped default **on**
(DIV-3).

**ovlesssimplify** (GH-7190) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_ovlesssimplify.rs
(RuleOvLessSimplify)`, oppool1, fires on INT_NOTEQUAL. Pattern: the explicit
S/OV-flag signed-less-than computation (V850-style),
`NE(SLESS(V+K, 0), BOOL_AND(signtest, SLESS(-1, V+K)))` — the sign flag XORed
with the overflow test spelled out in p-code. Rewrite: `INT_SLESS(V, -K)`.
Settable `ovlesssimplify`, shipped default **on** (DIV-2).

**compareform** (GH-558) — not a pool peephole but the canonicalization
round-trip for `<=`. The analysis wants one canonical compare form, so
`decompiler/crates/kuna-decomp/src/substrate/funcdata_op.rs
(Funcdata::replace_lessequal)` rewrites `V <= c` into `V < c+1` (and
`c-1 < V` from `c <= V`), with overflow guards, from exactly three sites: the
pool rule `ruleaction_1.rs (RuleIntLessEqual)` — carried in its own group
`canonicalcompare`, enabled in every root variant — and the two branch-flip
primitives in `funcdata_op.rs` (`op_normalize_flip` and the flip-in-place
path). Each rewrite stamps a provenance bit on the op
(`canonical_lessequal`). At the very end of the drive — after structuring's
last flips, before prototype/cast/naming fixation —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_compareform.rs
(ActionPresentCompareForm)` (group `presentcompare`, `decompile` variant only)
inverts every still-marked op back to the source `<=` form, re-validating the
shape from scratch so an op reshaped by a later transform is simply left
alone. Settable `compareform canonical|original`, shipped default
**original** (restore `<=`; DIV-2 — the flip re-pinned 12 of 675 datatest
assertions); `option compareform canonical` leaves the analysis form standing,
reproducing upstream Ghidra's rendering.

**arraystride** (GH-8724) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_arraystride.rs
(RuleArrayStride)`, oppool1, fires on MULTIEQUAL. Pattern: a strength-reduced
array walk — a loop-header offset accumulator
`acc = MULTIEQUAL(#0, acc + STRIDE)` (STRIDE constant, neither 0 nor 1) with a
sibling unit-step counter phi `cnt = MULTIEQUAL(#0, cnt + 1)` in the *same*
block, lining up edge-for-edge. Rewrite: every other use of `acc` is replaced
by `INT_MULT(cnt, STRIDE)`, re-exposing `cnt` as the array index so the
pointer rules and the emitter can render `arr[i]` instead of
`iVar += 0x414`. Settable `arraystride`, shipped default **on** (DIV-3).

## 3.6 Early passes

`decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs` holds the
setup and maintenance actions the schedule interleaves around heritage and the
pools (§0.6 places them; this section says what each computes).

**Setup one-shots** (in the restart group's prologue or at phase switches):
`ActionStart`/`ActionStop` are no-ops in kuna — the start/stop bookkeeping the
C++ did there happens in the drive, which follows flow before the tree runs
(§0.6) — and exist so the tree's listing stays oracle-identical.
`ActionConstbase` injects a `COPY #val` at the entry block for every *tracked
register* the context database pins to a constant at this function's address
(the console `set track` surface). `ActionStartTypes` flips the function's
type-recovery flag — the gate the oppool2 pointer rules and
`ActionInferTypes` key on (chapter 05) — and `ActionStartCleanUp` marks the
transition into the cleanup phase. `ActionNormalizeSetup` (normalize variant
only) strips prototype locks for the normalization style.

**Per-iteration maintenance** (mainloop): `ActionSpacebase` marks
stack-pointer varnodes and their types ahead of heritage; `ActionHeritage`
drives §3.1; `ActionNonzeroMask` recomputes the known-zero-bits fact
(`Funcdata::calc_nz_mask`) that dozens of rules consult (§3.5's booleanmask
and flagcompare among them); `ActionVarnodeProps` applies storage-derived
properties — after the first heritage pass it releases the `autolivehold`
pins (except on values still LOADed through a constant/read-only pointer),
replaces *read-only* storage with its image constant when
`readonlypropagate` is set — or, with that program-wide switch off, when the
varnode lies in one of the loader's `dynrelocs` ranges, the `PT_GNU_RELRO`-frozen
dynamic-relocation slots whose value the linker itself computed (§1.2), which is
what turns a call through a relocated GOT slot back into a named call — expands
*volatile* access into its user-op form,
and folds to zero any varnode whose consumed bits and nonzero mask are
disjoint (skipping constants and COPYs of nonzero constants, which would
recurse).

**Block-graph cleanup** (mainloop tail): `ActionUnreachable` deletes blocks
flow cannot reach (`Funcdata::remove_unreachable_blocks`); `ActionDoNothing`
(repeat-apply) and `ActionLateDoNothing` splice out empty do-nothing blocks
early and late; `ActionRedundBranch` removes a branch whose target join adds
nothing (the redundant-join splice); and `ActionDeterminedBranch` converts a
CBRANCH whose condition has simplified to a constant into an unconditional
branch, severing the dead edge
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::remove_branch)`) — this is the in-loop feedback edge by which a
constant-propagation result (P5 facts) edits the P2 control-flow artifact
without any restart (§0.7): the next mainloop iteration simply re-heritages
the smaller graph.
