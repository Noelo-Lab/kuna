# 08 — Structuring

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p8_structure
```

This phase turns the basic-block CFG into a tree of high-level constructs —
if/else, loops, switches, and the residual `goto`s — and then spends a family
of passes driving the goto count and the structure shape toward the source.
The folder holds **two structuring engines and one pass family**: Ghidra's
`CollapseStructure` (§8.1, the ported default-lineage engine, still the
fallback), the angr Phoenix/SAILR **region structurer** (§8.2, the primary
path since DIV-12), and the **goto-quality passes** (§8.3), almost all
angr-derived (SAILR; Basque et al., USENIX Security 2024). §8.4 covers the
quality *signal* — what kuna can measure about a structuring, and what it
deliberately does not yet do with the measurement. This is the chapter with
the heaviest angr provenance: untagged prose is the Ghidra port; everything
option-gated below is registered with `source_decompiler = "angr"` in
`decompiler/crates/kuna-decomp/phases.toml` and tagged `(angr)` here. Option
defaults, tiers, symptoms, and flip guidance for every option named below
live in the generated catalog ([docs/options.md](../options.md)); this
chapter names only the divergence record that flipped a default
(DIV-10/12/14/17/18/23/25 in `docs/divergences.md`).

Structuring operates on a dedicated arena: `sblocks`, a `BlockCopy` mirror of
the real basic blocks seeded by
`decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs
(ActionBlockStructure)`. Both engines collapse that mirror; neither mutates
p-code, with one deliberate exception — a branch-condition orientation flip
(`negateCondition`, needed whenever a schema requires the true edge on the
other side) is recorded during collapse and its data-flow half (the CBRANCH
`boolean_flip`/`fallthru_true` toggle) is realized afterwards by the driver (`blockaction.rs (CollapseStructure::take_pending_flips)` →
`Funcdata::block_basic_negate_lastop`, and identically for the region
structurer). The phase registry rows for everything here are the P8 sections
of `phases.toml`: the `schema-precedence` subphase (the latent rule-order
assertion §8.1 realizes), `goto-quality` (the option-gated pass family),
`readability-rewrites` (branch polarity), and `goto-quality-acceptance`
(§8.4).

## 8.1 Collapse structuring

The engine is `decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs
(CollapseStructure)`, the line-faithful port of Ghidra's structurer
(parenthetical provenance: `decompiler/cpp/blockaction.{cc,hh}`). The simple
case: repeatedly scan the live components and collapse the first sub-graph
that matches a code-structure schema, until one node remains. The schema
cascade in `blockaction.rs (CollapseStructure::collapse_internal)` is
**output-determining** — when several structurings are possible, the rule
order decides which one wins and therefore which edges are left over to
become gotos — so it is transcribed in exactly the upstream order:

1. `rule_block_goto` — wrap an edge already *marked* unstructured: a marked
   switch edge becomes a `BlockMultiGoto`, a marked 2-out branch a
   `BlockIf`-goto (negating the condition if the goto is not the true edge),
   a marked single out a `BlockGoto` leaf.
2. `rule_block_cat` — concatenate: a single-out block whose successor is
   single-in merges into a `BlockList`, extended while the tail stays
   single-in/single-out.
3. `rule_block_proper_if` — if-then: a 2-out condition whose one clause is
   single-in/single-out and rejoins the other successor.
4. `rule_block_if_else` — if/else: both clauses single-in/single-out,
   reconverging on the same block.
5. `rule_block_while_do` — while-do: condition with one edge looping back.
6. `rule_block_do_while` — do-while: a body whose own out-edge is the
   back-edge.
7. `rule_block_inf_loop` — a single-out block falling into itself.
8. `rule_block_switch` — a switch-out head whose cases share a determinable
   exit block (a loop-back, multi-in, or multi-out successor, else the common
   successor of all single-out cases), with no goto touching the exit;
   `blockaction.rs (CollapseStructure::check_switch_skips)` first converts a
   non-default case edge that jumps straight to the exit into a goto when a
   default exists elsewhere (the `case N: goto done;` skip shape).

Two rules are deliberately **deferred to a second scan** that runs only when
the cascade reaches a fixpoint, because firing them early robs better rules
of their match: `rule_block_if_no_exit` (an if whose clause has no exit at
all — a terminal arm) and `rule_case_fallthru` (a case falling through into
another case). One rule is deliberately **not in the cascade at all**:
`rule_block_or` (`&&`/`||` short-circuit folding into a `BlockCondition`) is
run to a fixpoint *before* everything else by `blockaction.rs
(CollapseStructure::collapse_conditions)`, so cascading guards fold into one
compound condition before the if-rules can consume them separately.

**Goto selection (the pathological case).** When the cascade stalls with more
than one component live, `blockaction.rs (CollapseStructure::select_goto)`
marks one edge unstructured and the cascade retries. Which edge matters
enormously for output quality, and the machinery is layered:

- *Loops first, innermost first.* `blockaction.rs (LoopBody)` records every
  natural-loop body up front (identical heads merged, bodies ordered deepest
  first), because gotos must be resolved inside the innermost loop before the
  containing loop can collapse. The candidate list survives graph mutation as
  `blockaction.rs (FloatingEdge)` records — an edge is remembered by its
  endpoints' current collapsed representatives, not by index.
- *The trace.* `blockaction.rs (TraceDAG)` traces every path from the scope's
  roots (the innermost live loop's head, with the loop's exit blocks masked;
  or the whole DAG when no loop remains), retiring branch points as their
  paths merge. When traces stall, `blockaction.rs (TraceDAG::select_bad_edge)`
  scores each blocked trace (`blockaction.rs (BadEdgeScore)`) and removes the
  least-structurable one, appending it to the **likelygoto** list. The final
  comparison (`BadEdgeScore` `compare_final`) prefers, in order: fewest
  sibling edges into the same exit; a destination with no exits of its own
  (a terminal); a greater distance between the conflicting branch points; a
  deeper branch point. After the trace, the loop's own exit edges are
  appended (`blockaction.rs (LoopBody::emit_likely_edges)`).
- *Consumption.* Each `select_goto` call takes the next still-live likely
  edge and marks it (`set_goto_branch`); `blockaction.rs
  (CollapseStructure::update_loop_body)` regenerates the list only when the
  current loop is exhausted or dissolved.

**Failure mode.** If no likely edge is live, no loop remains, and the final
whole-DAG trace produced nothing, `blockaction.rs
(CollapseStructure::clip_extra_roots)` tries the last unblocking move —
marking as gotos the cross-over edges out of a region reachable only from an
*extra* root component (a second entry point); if even that finds nothing
the engine errors ("Could not finish collapsing block structure"), which
`ActionBlockStructure` degrades to "no change made" rather than aborting the
decompilation — the printer then falls back to flat block output for the
function.

**When this path still runs.** Since DIV-12 the region structurer (§8.2) is
the default S8 engine, and on the entire reducible corpus the two produce
byte-identical output (675/675 datatests verified at the flip).
`CollapseStructure` still runs in exactly three situations: `option
regionstructure off` (forcing the upstream path unconditionally); the region
structurer reports non-convergence — an irreducible/multi-entry knot it
declines or its hang-guard trips — and `ActionBlockStructure` re-seeds
`sblocks` and falls through; or the region structurer's own cyclic schemas
decline a loop with `regionlooprefine` off. The fallback guarantees the
region path is never *worse* than upstream.

**The rest of the file.** The same module carries the structuring-adjacent
actions from upstream: `blockaction.rs (ActionNodeJoin)` /
`(ConditionalJoin)` rejoin a split conditional (two blocks computing the same
branch condition merge into one decision point); `blockaction.rs
(ActionReturnSplit)` is upstream's goto-driven epilogue splitter — it
duplicates a shared RETURN block only when structuring left a **goto** into
it (the Ghidra analog of what angr calls `ReturnDuplicatorLow`; the gotoless
cases are §8.3's business); `blockaction.rs (ActionPreferComplement)` picks
between symmetric if/else structurings by removing negations;
`blockaction.rs (ActionStructureTransform)` and `(ActionFinalStructure)` do
the print-prep — for-loop rerolling, switch-case label assignment and
sorting, the §8.3 `scopeBreak` hook, `mark_unstructured` (labelling every
goto target so the printer emits `label_N:`), and `mark_label_bump_up` (a
loop-head label is hoisted above the loop rather than into its condition).
`blockaction.rs (ActionNormalizeBranches)` is transcribed control-flow with
its mutating half still stubbed (recorded in `docs/rust-port/losses.md`).

**(angr) `ActionBranchFlip`** also lives here (`blockaction.rs
(ActionBranchFlip)`), scheduled on the *final* structured tree right after
the second `ActionPreferComplement` pass
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`). Pattern: a 3-component `if/else` whose guard is the
negated / equality-to-zero form — exactly the flip class the upstream
`preferComplement` leaves alone
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::branch_flip_complement)`: `split_flip_in_place_test` result `1`,
i.e. `==` / `== 0` / a constant-operand `<`/`<=`, where `preferComplement`
only takes result `0`, the flips that *remove* negation). Transform: flip the
condition in place with the proven flip machinery and swap the two arms, so
`if (x == 0) {A} else {B}` reads as the positive `if (x != 0) {B} else {A}` —
angr-SAILR condition-polarity linearity. Each flip logs a `branchflip:`
warning at the guard's CBRANCH. Failure mode: anything that is not a clean
3-component if/else with a flippable split point is left alone (result `0`
is `preferComplement`'s job; result `2` cannot be flipped). Option
`branchflip`, default flipped on by DIV-14 (which lists the datatest files
that pin the pre-flip polarity via per-test opt-outs); registered at the
`readability-rewrites` subphase in `phases.toml`.

## 8.2 The region structurer (angr)

`decompiler/crates/kuna-decomp/src/p8_structure/region_structurer.rs` is the
port of angr's Phoenix/SAILR structurer (parenthetical provenance: angr
`analyses/decompiler/structuring/{recursive_structurer,phoenix,sailr}.py`),
grown increment by increment (the archived plan is
`docs/history/region-structurer-roadmap.md`) and made the default S8 engine
by DIV-12 under `option regionstructure`.

**Input.** `region_structurer.rs (run_region_structurer)` first runs the
chapter-07 region identifier fresh over the real CFG
(`decompiler/crates/kuna-decomp/src/p7_regions/kuna_regionid.rs
(KunaRegionIdentifier)`) and consumes two things from it: the deterministic
node order (tie-breaking for edge virtualization) and — when
`regionlooprefine` is on — the `kuna_regionid.rs
(KunaRegionIdentifier::cyclic_loops)` projection: each loop's refined body
and exit frontier keyed by block start address, which absorbs
dominated switch-case successors that a purely structural natural-loop walk
on the collapsing graph misses. Identification failure is non-fatal: the
structurer still runs on the `sblocks` topology alone. The structuring
itself operates on the same seeded `sblocks` mirror as §8.1, with the same
precomputed switch maps and complexity set `ActionBlockStructure` builds for
`CollapseStructure`, so the switch-finalization and rendering paths
downstream are unchanged.

**The schema loop.** `region_structurer.rs (RegionStructurer::structure)`
first folds short-circuit conditions to a fixpoint
(`match_acyclic_short_circuit_conditions`, the mirror of `ruleBlockOr` —
run up front for exactly the reason §8.1 runs `collapse_conditions` first),
then iterates until one component remains, each round trying in order:

1. `region_structurer.rs (RegionStructurer::match_acyclic_switch_cases)` —
   a resolved jump-table head folds natively into a `BlockSwitch` carrying
   the same `CaseOrder` descriptors the Ghidra path produces; runs first so
   a nested switch resolves before its surrounding structure.
2. `match_acyclic_sequence` — the single-pred/single-succ chain collapse
   (angr `_match_acyclic_sequence`, extended like `ruleBlockCat`).
3. `region_structurer.rs (RegionStructurer::match_acyclic_ite)` — the
   structural ITE: `try_if_else` (both clauses single-in/single-out,
   reconverging) and `try_if_then_true_clause` (the true-edge if-then, which
   needs no condition negation). The false-clause if-then — the arm that
   *would* need the data-flow negate — is deliberately left to the
   virtualize fallback (honest-partial).
4. `match_pending_inf_loop` — completes a switch-bodied loop fold queued by
   the cyclic pass below.
5. `region_structurer.rs (RegionStructurer::match_cyclic_schemas)` — loops,
   innermost first: `try_fold_loop` folds a loop whose body has already
   collapsed to a single block with at most one structural exit into a
   `BlockWhileDo`/`BlockDoWhile`/`BlockInfLoop`; under `regionlooprefine`,
   `try_fold_switch_loop` handles the `while (getopt(...)) switch (c) {…}`
   shape (case→head continues kept structural so the switch folds with the
   head as its exit), and `refine_loop_edges` handles everything else (see
   below).
6. `rule_block_goto` — wrap edges already marked unstructured, exactly as
   §8.1's rule 1.
7. `region_structurer.rs (RegionStructurer::virtualize_one_edge)` — the last
   resort: mark one structured edge as a goto (below). Because this always
   removes an edge, the loop converges; the only "stuck" exit is a component
   set with no virtualizable edge at all.

A hang-guard caps the rounds at `2*n^2 + 64` (`region_structurer.rs
(round_cap)`) so a mis-port fails cleanly instead of spinning; the cap, a
stuck knot, or a schema error all make `structure()` report non-convergence,
upon which `ActionBlockStructure` re-seeds and falls back to
`CollapseStructure` (§8.1) — the structurer never aborts the run.

**Edge virtualization and its ordering.** When forced to emit a goto, the
choice of edge is the whole quality game. Candidates are every structured,
non-self out-edge of a live component (edges inside a mid-fold switch-loop
body are protected and only offered when nothing else remains, so the
switch-loop schema is not shattered by its own fallback). The default
ordering (`region_structurer.rs
(RegionStructurer::order_virtualizable_edges)`) applies SAILR's H1 — fewest
siblings into the destination — then H3 — prefer an edge to a simple return
block — then a deterministic address-keyed base order. **(angr) `option
regionedgeorder`** upgrades this to the full SAILR machinery
(`region_structurer.rs (RegionStructurer::order_virtualizable_edges_sailr)`,
the port of `phoenix._last_resort_refinement` +
`sailr._order_virtualizable_edges`): build a snapshot graph over the live
components, compute forward immediate dominators, and bucket the candidates —
*crossing* edges (neither endpoint dominates the other) are virtualized
before *secondary* edges (destination dominates source); a forward tree edge
is never chosen. Within the winning bucket (`sailr_order_within_bucket`): H1;
then H2 — remove each candidate, recompute post-dominators, and keep the
edges whose removal yields the *most* post-dominators (most-linear result) —
bounded by `region_structurer.rs (POSTDOM_MAX_EDGES)` = 10 candidate edges
and `(POSTDOM_MAX_GRAPH_SIZE)` = 50 nodes, exactly angr's caps, because
post-dominator recomputation per edge is super-linear; then H3; then the
post-order `node_seq` tiebreak. `regionedgeorder` only changes *which* edge
is cut when the structurer is already forced to virtualize, so reducible
code is byte-identical either way; it remains an opt-in (no DIV has flipped
it — see the catalog).

**Loop refinement (angr, `option regionlooprefine`, flipped on by DIV-14).**
The base cyclic schemas fold only the clean shape: single entry, one
structural exit, one back-edge. Real loops have secondary exits (error
paths), secondary latches, and mid-body entries. `region_structurer.rs
(RegionStructurer::refine_loop_edges)` — the kuna analog of angr's
`_refine_loop_successors_to_guarded_successors` / `force_loop_single_exit` —
virtualizes exactly the abnormal edges so the loop *becomes* the clean shape,
in a strict order, one class per round so the acyclic schemas can re-collapse
in between: mid-body entry edges first (make the loop single-entry); then all
but one latch (keep the deepest as the structural back-edge — the rest become
`continue`); then all but one exit (keep the edge to the *normal* exit — the
region identifier's frontier target with the lowest address, or the
most-targeted exit without RI data — the rest become `break`). The
virtualized exit gotos are lowered to `break;` by §8.3's `scopeBreak` and the
latch gotos to `continue;` by loop construction, so the refined loop renders
as a structured `while`/`do-while`/`for` with break/continue instead of
falling back to `CollapseStructure`'s goto spray. Multi-entry heads are
handled only when the region identifier confirms a real loop (the extra
entries are virtualized); a structural false positive — a high-in-degree
merge the natural-loop walk mistook for a head — is bailed on
(`LoopRefineOutcome::Irreducible`) and left to the acyclic schemas. Because
refinement only touches loops the base schemas already declined, reducible
code is byte-identical with the flag off or on.

**Invariant.** The region structurer is byte-identical to `CollapseStructure`
across the full datatest corpus (DIV-12's verification) and structurally
cannot do worse: any function it cannot finish is re-structured by the
upstream engine from a clean re-seed.

## 8.3 The goto-quality pass family (angr)

Everything in this section is an option-gated, SAILR-derived pass that runs
around the structuring engines to remove residual gotos or recover the
source's return/branch shape. They come in two placement groups, visible in
the schedule (`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`):

- **CFG splitters** in fullloop's `returnsplit` group (`returndup`,
  `earlyreturn`, `switchreturn`): they duplicate real p-code via
  `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
  (Funcdata::node_split)` — the same machinery `ActionReturnSplit` drives —
  *before* final structuring, so the split returns flow through merge/naming
  and are structured normally on the next iteration.
- **Print-tree rewrites** after `ActionFinalStructure` (`gotoreduce`,
  `taildup`, `ifelseflatten`, `crossjumprevert`, `dedupitetail`,
  `iteregion`, in that registration order): they edit only the structured
  `sblocks` tree (fresh `BlockCopy` leaves referencing the *same* underlying
  basic blocks, or `list`/`parent` splices, or a print mark) — no p-code is
  cloned, SSA/def-use is untouched, and the printer re-emits the same ops in
  the new layout.

Two more run inside §8.1's machinery: `branchflip` (described there) and
`loopbreak_recovery` (inside `ActionFinalStructure`, below). All of them
share the safety stance: exact structural preconditions, hard caps, and
decline-on-any-doubt — there is no post-hoc validation to catch a bad edit
(§8.4).

### gotoreduce — return-tail duplication (angr `ReturnDuplicator`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_gotoreduce.rs
(ActionGotoReduce)`. *Pattern:* a `BlockIf` rendering as `if (cond) goto T`
where `T` is a single-successor basic-block chain ending in `return` —
`CollapseStructure` has no schema that duplicates a shared multi-predecessor
return tail, so one in-edge always renders as a goto. *Transform:* inline a
print-tree copy of the tail into the `if` arm
(`kuna_inline_return_tail`), clearing `T`'s label when this was its last
unstructured in-edge — the goto becomes a structured early return.
*Bounds/failure:* the chain is capped at `kuna_gotoreduce.rs
(MAX_TAIL_BLOCKS)` = 3 blocks and `(MAX_TAIL_OPS)` = 8 ops, and **any**
`CALL`/`CALLIND`/`CALLOTHER`/`STORE` in the tail declines the match (a
duplicated side effect would be unsafe to print twice) — the call-bearing
return tail is `taildup`'s case. Flipped on by DIV-14 (changes zero
datatests).

### taildup — return tails with calls (angr `ReturnDuplicatorLow`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_taildup.rs
(ActionTailDup)`. *Pattern:* the `free(p); return;` epilogue — a return-tail
`goto` target that *contains a call*, which `gotoreduce` rejects and
`crossjumprevert` (non-return tails only) declines. Two source shapes are
matched: the bare `BlockIf` if-goto, and the `BlockGoto` leaf `{ stmts; goto
T }` (the goto at the end of a multi-statement arm). *Transform:* the same
print-tree tail inlining, with the matching surgery per shape. Because the
tail ends in `return`, the duplicated call is reached on exactly one
structured path — no convergence proof is needed. *Bounds/failure:* ≤ 3
blocks, ≤ 12 printed ops, at most `kuna_taildup.rs (MAX_TAIL_CALLS)` = 2
calls (angr `ReturnDuplicatorBase.max_calls_in_regions = 2` — the budget
that defines this pass), `STORE` always declines. Flipped on by DIV-14.

### crossjumprevert — un-doing compiler cross-jumping (angr `CrossJumpReverter`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_crossjumpreverter.rs
(ActionCrossJumpReverter)`. *Pattern:* compilers condense identical tail code
reached from several places into one block (Irreducible Statement
Condensing), leaving all but one predecessor to arrive by `goto` — a
`BlockGoto` leaf whose target is a small *non-return* block that falls
through to a unique successor `S`. *Transform:* duplicate the one tail block
inline and drop the goto — but only under the **convergence precondition**
(`kuna_crossjumpreverter.rs (converges_on)`): the structured fall-through
after the goto's parent (`decompiler/crates/kuna-decomp/src/substrate/block.rs
(BlockGraph::next_flow_after)`) must be exactly `S`, because unlike a return
tail this path stays open — duplicating it is only semantics-preserving if
both routes provably rejoin at `S`. *Bounds/failure:* exactly one tail block
(angr duplicates the single `goto_target` node), ≤ 8 printed ops, ≤ 1 call
(`MAX_TAIL_CALLS`, angr `max_call_dup`), no `STORE`; declines a tail ending
in `return` (that is the gotoreduce/taildup family) and anything whose
convergence cannot be proven. Flipped on by DIV-14.

### ifelseflatten — guard-clause flattening (angr `IfElseFlattener`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_ifelseflatten.rs
(ActionIfElseFlatten)`. *Pattern:* a full 3-component `BlockIf` whose true
clause is statement-terminating but which still carries an explicit `else` —
the residue the structurer or fallback could not invert away. *Transform:*
unlink the else clause and splice it into the parent list as the `if`'s
immediate follower: `if (c) { ...; return X; } else { body }` becomes
`if (c) { ...; return X; } body` — the guard-clause de-indent. *The
termination test is deliberately strict* (`kuna_ifelseflatten.rs
(clause_terminates)`, angr's `is_statement_terminating`): only a `return` or
a no-return call terminates; a trailing `goto`/`break`/`continue` does
**not** (its target may rejoin the after-if flow, so dropping the else would
be unsound), and a nested conditional that might fall through on one arm is
conservatively non-terminating. *Failure:* declines when the else clause
itself terminates (angr's exact dual test), when the if carries its own
goto, or when the parent cannot host the moved sibling. Flipped on by
DIV-14.

### dedupitetail — merging over-duplicated arms (angr ITE region dedup)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_dedupitetail.rs
(ActionDedupIteTail)`. The **inverse** of the duplication passes: here the
collapse-based structurer *cloned* a shared continuation into both arms of an
`if/else` (the compiler emitted two physical copies and structuring kept
both), and the win is merging them back. *Pattern:* a 3-component `BlockIf`
whose arms begin (prefix) or end (suffix) with runs of *leaf* blocks that
emit **identical C** — same printed-op sequence, constants matched by value,
named storage by location, call targets by resolved callee. *Transform:*
hoist the shared run out of the `if` — a prefix before it (unconditionally
sound: both arms executed it first regardless of the condition), a suffix
after it, the latter only when each arm's divergent middle provably falls
through to that suffix (`kuna_dedupitetail.rs (plan_suffix_merge)`).
*Bounds/failure:* only flat leaves (never a nested if/loop, whose hoisting
would change control flow), ≤ `kuna_dedupitetail.rs (MAX_HOISTED_LEAVES)` = 8
leaves and ≤ `(MAX_LEAF_OPS)` = 64 ops per equivalence check, exact-or-decline
on any mismatch, and a leaf carrying a label/goto is never hoisted. Flipped
on by DIV-14.

### iteregion — `?:` recovery (angr `ITERegionConverter`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs
(ActionIteRegion)`. *Pattern:* the assignment diamond a source ternary
compiles to — a 3-component `BlockIf` with no goto, whose condition leaf's
only printed op is the `CBRANCH` and whose two arms are each a single `COPY`
to the same storage. *Transform:* **print-only** — set the `kuna_iteregion`
addl-flag on the matched CBRANCH; the S9 printer
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`, via
`kuna_iteregion.rs (match_ite_assignment)`) re-derives the match and emits
`v = ( c ) ? A : B;` in place of the `if/else` — the ternary emit is the one
genuinely new piece (upstream printc has no ternary operator). No p-code is
touched; the arm COPYs stay live so later uses render unchanged. *Failure:*
declines any deviation — multi-statement arms, different destinations, a
side effect riding the condition. **`iteexpr` (option, default-off)** broadens
`single_assign_arm` from a plain `COPY` arm to any single **computed** pure-value
op (`v = *p`, `v = b + 5` — a `LOAD`/`INT_*`/`PTR*`/`CAST`/`SUBPIECE`, whose
second-level operands are single-use implied varnodes), rejecting only
side-effecting/control ops; the print-only render (`op_push_ir`) already handles
arbitrary arm expressions. This matches angr's aggressive `?:` recovery (angr emits
~8× more ternaries than kuna on decbench O0). It is a **readability** change only:
Joern gives a `?:` and its equivalent `if/else` the identical CFG, so GED is
unaffected — hence `iteexpr` ships default-off (a `--mode aggressive` member), not a
DIV flip. This one is an explicit **runtime
choice**: an explicit source `if/else` compiles to the same bytes, so the
rewrite matches the source only when the source used a ternary; DIV-17
flipped it on (ablation net-positive) and documents flipping it off per
function.

### earlyreturn — per-edge const-guard peeling (angr `ReturnDuplicatorHigh`, narrowed)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_earlyreturn.rs
(ActionEarlyReturn)`. *Pattern:* the leading argument-validity guard — a
shared RETURN block computing `v = MULTIEQUAL(#K /*guard arm*/, <var> /*body
arm*/); return v`, a **mixed** phi that whole-block gates structurally cannot
touch. Without it kuna renders the inverted diamond `if (guard) { entire
body } else { v = K; } return v;`. *Transform:* compute the phi input slots
that resolve to constants (`funcdata_block.rs
(Funcdata::earlyreturn_const_edges)`; slot *i* aligns with in-edge *i*) and
`node_split` **only those edges** (`funcdata_block.rs
(Funcdata::const_return_peel)`), leaving the variable body return merged.
Each peeled arm becomes its own `return K` predecessor; downstream
`branchflip` orients the guard and `ifelseflatten` drops the else, yielding
the source's `if (c) return K; ...body...; return <var>;`. *Bounds/failure:*
the block must pass the same bare-epilogue filter as `ActionReturnSplit`
(`funcdata_block.rs (Funcdata::return_split_is_splittable)` — only
MULTIEQUAL/COPY/RETURN over constant/annotation/non-free inputs, so calls and
stores are never cloned); the last remaining edge is never split; caps
`kuna_earlyreturn.rs (MAX_EARLYRETURN_INEDGES)` = 16 predecessors,
`(MAX_EARLYRETURN_SPLITS)` = 32 splits/function; each hoist logs an
`earlyreturn:` warning. Because it only ever peels a **constant** arm it
cannot reproduce `returndup`'s variable-return regression (below). Flipped
on by DIV-23 (ablation: +47 perfect matches, −576 summed GED, 158:54
improved:regressed over 508 binaries).

### switchreturn — the wide multi-way extension (angr `ReturnDuplicatorHigh`, switch-phi)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_switchreturn.rs
(ActionSwitchReturn)`. *Pattern:* the enum-dispatch switch whose every case
assigns a constant and breaks to one exit — `v = MULTIEQUAL(#K0, #K1, …);
return v` with one in-edge per case. `earlyreturn` **already** recovers this
for switches whose merge block has at most 16 predecessors — its
`MAX_EARLYRETURN_INEDGES` cap, read from the code — so a 17-case dispatch
was left merged. *Transform:* the identical const peel
(`funcdata_block.rs (Funcdata::switchreturn_apply)` delegates to the same
`const_return_peel` core) with the caps lifted to `kuna_switchreturn.rs
(MAX_SWITCHRETURN_INEDGES)` = 256 and `(MAX_SWITCHRETURN_SPLITS)` = 256, so
every constant case ends in its own `return K`. *Bounds/failure:* inherits
every earlyreturn safeguard (splittable filter, never the last edge,
const-arms only — a non-const default/body arm stays merged); logs
`switchreturn:` per hoist. Registered right after `earlyreturn` so the
narrower pass consumes the ≤16-edge diamonds first; when both are on,
switchreturn's incremental effect is precisely the wide tables. Flipped on
by DIV-25 (incremental ablation on top of earlyreturn: +2 perfect, −107
summed GED, 3:0 improved:regressed, zero regressions).

### loopbreak_recovery — `goto` → `break;` (Ghidra `scopeBreak`, angr-surfaced)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_loopbreak_recovery.rs
(kuna_scope_break)`. The mechanism is the faithful port of upstream Ghidra's
`BlockGraph::scopeBreak` — which upstream runs unconditionally but kuna's
port had left stubbed; the gap was found and the pass registered through the
angr comparison (record `docs/features/1after909-doit-73591e/`, hence the
angr provenance row). *Pattern:* after collapse, an edge that leaves a loop
for the loop's single successor (or a switch case exiting straight to the
switch successor) is the semantic `break;` — without this pass it rendered
as `goto label_N;` plus a synthesized label. *Transform:* one recursive walk
of the structured tree carrying the current fall-through and the innermost
enclosing loop's successor; every `BlockGoto`/if-goto/case-exit whose target
equals the latter is retagged `f_goto_goto → f_break_goto`, which the printer
emits as `break;`. It runs inside `ActionFinalStructure` strictly between
`finalizePrinting` and `mark_unstructured`, so the retagged target is never
label-marked and the dead label disappears (`continue` needs no handling
here — loop construction already tags back-edge gotos). *Failure:* an edge
whose target is not exactly the innermost loop successor is left a real
goto. Flipped on by DIV-10.

### returndup — shipped, then reverted (angr `ReturnDuplicatorHigh`; history)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_returndup.rs
(ActionReturnDup)` is the whole-block gotoless return duplicator: every
shared bare-epilogue RETURN block passing the splittable filter *and* whose
whole return value resolves to a constant (`funcdata_block.rs
(Funcdata::returndup_is_const_ret)` — a literal, COPY chain, or
phi-of-constants; never a free variable) is split into per-predecessor
returns, capped at `kuna_returndup.rs (MAX_RETURNDUP_INEDGES)` = 16 /
`(MAX_RETURNDUP_SPLITS)` = 64. It exists to break `rule_block_or`'s
comma-folded guard merges (`if ((A||B) && (v=f(..), C)) …` with one trailing
return) back into the source's early-return guards. **History (DIV-18):** it
shipped default-on and was **reverted the same cycle** — the decbench re-run
measured the then-unselective version (which also split `return <variable>`
epilogues) firing 21,768 times across 550 binaries and regressing the
aggregate GED-perfect count by ~976: on the majority of shared epilogues the
*source* used the merged form, so broad duplication was churn, not recovery.
The pass survives as an opt-in runtime choice with the const-return gate
now in place; its durable descendants are the per-edge narrowings that
target only what the source provably wrote as early returns — `earlyreturn`
(DIV-23) and `switchreturn` (DIV-25). The lesson is recorded in each of
their gates: never peel a variable-return share.

## 8.4 The quality signal

What kuna can *measure* about a structuring today is the goto census. The
structured tree is walked by `decompiler/crates/kuna-decomp/src/substrate/block.rs
(BlockGraph::kuna_count_quality)`, tallying `BlockGoto` nodes — split into
nodes and *printed* gotos, because a `BlockGoto` whose target is exactly the
next block in normal flow prints nothing (`block.rs
(BlockGraph::goto_prints)`, which asks `next_flow_after`) — plus
`BlockMultiGoto` virtual edges and `BlockIf` if-goto edges still tagged as
real gotos. The console command `quality`
(`decompiler/crates/kuna-console/src/kuna_console.rs (IfcKunaQuality)`,
surfaced through `decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::kuna_quality_counts)`) reports the census for the current
function — basic-block count, goto nodes (and printed), multi-goto edges,
if-goto edges, and the unstructured total — and refuses politely when the
function has no structured blocks yet. Alongside the census, every §8.3 pass
logs its firings as warning comments in the output (`earlyreturn:`,
`switchreturn:`, `branchflip:`, `crossjumprevert:`, `ifelseflatten:` …), so
a diff of the rendered C always shows *which* quality pass produced it.

What kuna deliberately does **not** have is speculate-validate-rollback:
angr's `StructuringOptimizationPass` wraps every de-optimization in an
edit → re-structure → measure → restore-on-regression loop
(`prevent_new_gotos`, `strictly_less_gotos` et al.); kuna's passes are all
greedy accept-only, with the correctness burden carried up front by their
structural preconditions and caps. This is a recorded, intentional gap: the
`goto-quality-acceptance` subphase row in
`decompiler/crates/kuna-decomp/phases.toml` registers the accept-or-rollback
decision as LATENT, with the `quality` command as its observable half, and
the archived phase-model study (`docs/history/stage-model.md`, mechanism
"speculate-validate-rollback") names porting it as future work. In practice
the validation loop runs *outside* the engine instead: the default flips in
this chapter were each gated on a measured decbench GED ablation
(net-positive for DIV-17/23/25; the DIV-18 revert is the same oracle saying
no), so the accept/rollback decision is made per-default at the corpus
level, not per-function at decompile time.
