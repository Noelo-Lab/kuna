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
stack (or a fresh input) instead (`heritage.rs (op_from_const)`).

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

Two upstream sub-cases are deliberately not implemented (kuna): the
partial-range output overlap (`tryOutputOverlapGuard`) and partial-range input
overlap (`guardCallOverlappingInput`) branches do nothing — a sub-register
slice of a return or argument register at a call simply gets no guard (the
whole-register corpus never reaches them). `heritage.rs
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
(`heritage.rs (Heritage::reprocess_free_stores)`). Two upstream refinements are
not ported (kuna): the value-set analysis that narrows a guard to a
`[min,max,step]` window (`analyzeNewLoadGuards` /
`LoadGuard::establish_range` / `finalize_range`) — so kuna guards are never
range-locked and every consumer sees the maximally conservative whole-space
range — and the `highPtrPossible` alias path inside
`heritage.rs (Heritage::guard)` is structurally disabled (its condition is
constant false; the `guard_stores` body behind it is an explicit unreached
stub, and `guard_loads` a second, silent no-op behind the same constant). The guards' main consumer is the merge tier's untied-call intersection
test (chapter 06); `RuleIndirectCollapse`'s store-guard branch reads them too.

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
order through a resumable cursor that survives op deletion (§0.3), and for each
op walks its opcode's rule list in registration order
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
`readonlypropagate` is set, expands *volatile* access into its user-op form,
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
