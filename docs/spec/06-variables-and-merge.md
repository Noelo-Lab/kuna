# 06 — Variables & merge

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p6_variables
```

This phase turns SSA values into *human variables*: it decides which Varnodes
are the same source-level variable (`HighVariable` formation, §6.1), carves the
stack frame into named locals (§6.2), gives storage-less values a durable
identity (§6.3), and — because none of these decisions may be revisited by
simplification — runs its committing half strictly *after* the Band-B fixpoint
(§6.5). Per the schedule (00-overview §0.6) the phase is split in two: the
*preparatory* members run inside `mainloop` and co-evolve with SSA and types
(`ActionRestrictLocal`, `ActionDynamicMapping`, `ActionRestructureVarnode`,
and `stackstall`'s `ActionStackPtrFlow` plus the `stackvars` rules), while the
*committing* merge phalanx (`ActionAssignHigh` through `ActionCopyMarker`) is a
one-shot tail after `fullloop` exits — the exact order is the pass tree in
`decompiler/crates/kuna-decomp/src/infra/universalaction.rs (universal_sched)`.

Option defaults and flip guidance for every option named below live in the
generated catalog ([docs/options.md](../options.md)); the rows are defined in
`decompiler/crates/kuna-decomp/phases.toml` and the intentional
default-divergences are DIV-3/14 in `docs/history.md`.

> Scope note: several actions of this phase live outside the `p6_variables`
> folder for file-lineage reasons — `ActionRestructureVarnode`,
> `ActionMappedLocalSync`, `ActionDynamicMapping`, `ActionDynamicSymbols`, and
> the `ActionStackPtrFlow` wrapper are in
> `decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs`, and
> `ActionRestrictLocal` is in
> `decompiler/crates/kuna-decomp/src/p4_calls/coreaction_protos.rs`. Conversely
> `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_facing.rs` (the
> "facing type" a merged HighVariable presents at each read/write) is consumed
> by the cast placement of chapter [09 — Emission](09-emission.md).

## 6.1 HighVariables & Cover

**The artifact.** A `decompiler/crates/kuna-decomp/src/p6_variables/variable.rs
(HighVariable)` is an ordered list of member Varnodes that will render as one
named variable. Its derived state — boolean properties, data-type, name
representative, cover — is *lazily* recomputed under dirty bits
(`variable.rs (high_flags)`): any merge or membership change marks
`flagsdirty|typedirty|coverdirty|…` and the next accessor re-derives from the
members. This object-grain invalidation is P6's own feedback mechanism — no
pass re-runs when a HighVariable changes, the next reader just pays the
recompute. The type of the whole is the most specialized member type
(`variable.rs (HighVariable::get_type_representative)`, type-locked members
win), and the naming storage is chosen by a fixed preference ladder
(`variable.rs (HighVariable::compare_name)`: name-locked > unaffected >
persistent > input > address-tied > … > earliest-defined). HighVariables live
in a per-function arena (`variable.rs (HighVariableBank)`), created for every
Varnode by `ActionAssignHigh` (`Funcdata::set_high_level`) at the head of the
phalanx; overlapping storage pieces of one symbol are tied together by the
`variable.rs (VariableGroup)`/`(VariablePiece)` model so a 4-byte read of an
8-byte slot stays a *piece* of the same group rather than a rival variable.

**Cover.** A variable can only absorb another if their live ranges do not
collide, and the live range is the
`decompiler/crates/kuna-decomp/src/p6_variables/cover.rs (Cover)`: per basic
block, one interval from a start point to a stop point in the block's op order
(sentinels for block-begin/block-end/function-input). Interval comparison
returns a three-level verdict — 0 no overlap, 1 boundary-touch only, 2 real
interval overlap (`cover.rs (CoverBlock::intersect)`); boundary touches are
*allowed* in a merge (one variable dying exactly where the other is defined is
the normal copy chain). A whole-cover test walks both block maps in order and
reports the strongest per-block verdict (`cover.rs (Cover::intersect)`,
`(Cover::intersect_list)` at level 2 for the candidate blocks).

A Cover is built from *two* kinds of point and is only correct when both are
supplied. `cover.rs (Cover::add_def_point)` resets it to the single point where
the Varnode is written; `cover.rs (Cover::add_ref_point_for)` then extends it
back from each read through the predecessors until it meets that write
(`cover.rs (Cover::add_ref_recurse)`). A cover built from the def point alone is
a *point*, not a range, and every containment question asked of it answers "no" —
so any caller that means to ask "is anything written between these two program
points" must add the ref point. Two such ranges are built on the fly rather than
from a Varnode's own reads: the single-read cover
`funcdata.rs (Funcdata::build_single_read_cover)` that
`merge.rs (Merge::eliminate_intersect)` uses to decide whether one read crosses
an intervening write at the same address, and the dominance range
`funcdata.rs (Funcdata::build_copy_pair_range)` used by the redundant-COPY test
in the phalanx tail below.

The reads a Cover walks are not only the Varnode's own: `cover.rs
(Cover::rebuild)` follows every read whose output is *implied* and keeps walking
that value's reads, transitively. An inlined expression is printed where its
outermost explicit consumer is printed, so an operand of an implied expression is
live all the way to that point, and its Cover must say so. This is why marking a
Varnode implied invalidates its operands' Covers rather than only its own (§ the
explicit/implied pass below), and why a Varnode `coverdirty` is forwarded to the
owning HighVariable — a member whose Cover went stale makes the whole variable's
cover stale, and `variable.rs (HighIntersectTest::update_high)` refuses to
recompute a variable it believes is clean.

That forward is owed on every path that dirties a member, including the one where
the Cover object is *replaced* rather than invalidated. Re-pointing an op's output
drops the old output's Cover entirely
(`funcdata_op.rs (Funcdata::op_unset_output)`), and the next
`funcdata.rs (Funcdata::set_varnode_properties)` allocates a fresh, empty one. A
Varnode that is already a member of a settled variable therefore re-enters that
variable carrying no live range at all, and unless the allocation dirties the
high, the empty range is what the intersection test reads: the member is treated
as live nowhere and an overlapping merge is accepted. This bites the trim COPYs
`merge.rs (Merge::merge_op)` inserts, whose outputs are re-pointed after they have
joined a high — the merged variable then holds two values that are live at the
same instant.

**What makes two Varnodes mergeable.** The gate is a three-rung test ladder in
`decompiler/crates/kuna-decomp/src/p6_variables/merge.rs`, and the rung used
determines how aggressive the merge may be:

- `merge.rs (Merge::merge_test_basic)` — can this Varnode ever be part of a
  multi-member variable: it must carry a cover (not a constant or annotation),
  and not be implied, a proto-partial (CONCAT-tree piece), or the spacebase.
- `merge.rs (Merge::merge_test_required)` — the tests even a *forced* merge
  must pass: two type-locked highs need the identical type; two address-tied
  highs need the same tied address; an input never merges with a persistent
  (global) or into address-tied storage it doesn't own; proto-partials keep
  out of inputs/tied/persistent highs; two pieces of *separate* overlap groups
  only merge if one represents its whole group; and two highs mapped to
  different Symbols (or different offsets of the same Symbol) never merge —
  this last guard is what keeps a dynamic-hash temporary (§6.3) distinct from
  the storage it copies.
- `merge.rs (Merge::merge_test_adjacent)` — additionally: same data-type, no
  two name-locks, no illegal inputs (unless indirect-only), no speculative
  merging across separate overlap groups, and neither Symbol *isolated* — the
  console `isolate` command sets exactly this bit, the operator's HARD "do not
  fuse this variable" assertion.

  Upstream reads both the Symbol and the isolated bit off a *cached*
  `HighVariable::getSymbol()`; kuna's merged tree does not paint SymbolEntries
  onto Varnodes before the merge group runs, so it re-derives the binding with
  the same `findContainer` containment query `linkSymbol` uses
  (`funcdata_merge.rs (bank_symbol, bank_symbol_isolated)`). Re-deriving it is a
  scan of the high's members, and this ladder is the merge's inner loop, so both
  re-derivations answer from a cheaper fact first when one settles them: the
  Symbol lookup only ever accepts an address-tied member and a high's cached flag
  word is the OR of its members', so a clean `addrtied == false`
  (`variable.rs (kuna_addr_tied_if_clean)`) means no member can carry one; and
  the isolated test can only say yes about a scope that has had a Symbol
  isolated, which `ScopeLocal` records as it happens
  (`varmap.rs (ScopeLocal::has_isolated_symbols)`) because
  `set_symbol_isolated` is the only route the `ISOLATE` dispflag has into a
  function-local scope. Neither shortcut changes an answer.
- `merge.rs (Merge::merge_test_speculative)` — additionally: nothing
  persistent, no inputs, nothing address-tied. Purely cosmetic merges never
  touch storage that has an ABI or memory identity.

Passing the ladder still only earns the *attempt*: the actual merge
(`merge.rs (Merge::merge)`) runs the cover-intersection test and refuses on
collision.

**The intersection test** (`merge.rs (MergeIntersect::intersection)`). Results
are cached per HighVariable pair (`variable.rs (HighIntersectTest)`, a
symmetric map keyed on the id pair) because the phalanx asks the same question
many times; a cover-dirty high purges its cached edges first
(`variable.rs (HighIntersectTest::purge_high)`), and a successful merge
*translates* the loser's cached tests onto the winner instead of discarding
them (`variable.rs (HighIntersectTest::move_intersect_tests)` — only
still-valid verdicts survive). The test itself has a refinement rung and a
secondary test:

- Blocks where the two covers overlap at level 2 are re-examined per instance
  pair, and an overlap is *forgiven* when the two Varnodes are copy shadows —
  provably the same value (`merge.rs (MergeIntersect::test_block_intersection)`,
  through `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_merge.rs
  (MergeContext for Funcdata)` to `Varnode::copy_shadow`); overlap-group pieces
  are tested at their relative offsets with partial shadows.
- If the covers are disjoint but exactly one high is address-tied, the
  *untied-call* test runs (`merge.rs
  (MergeIntersect::test_untied_call_intersection)`): a non-persistent tied
  local that has a possible pointer alias (no `nolocalalias`, §6.2) conflicts
  with an untied candidate whose cover crosses any call or guarded STORE — the
  callee could reach the stack home through the alias while the untied value
  is live in it. The crossing set is every CALL plus the still-guarded STOREs
  (`funcdata_merge.rs (MergeContext::populate_affecting_ops)`), built once and
  cached in the test cache's `cover.rs (PcodeOpSet)`.

**Forced merges** (`coreaction_cleanup.rs (ActionMergeRequired)` — all files
in this section under `decompiler/crates/kuna-decomp/src/p6_variables/` unless
pathed). Three passes run in fixed order, and *must* succeed — where the cover
forbids a forced merge, the machinery edits the data flow until it doesn't:

1. `merge.rs (Merge::merge_addr_tied)` — every maximal window of overlapping
   address-tied storage: Varnodes at the same (address, size) are force-merged
   into one high (`merge_range_must`), after `unify_address` /
   `eliminate_intersect` snip genuinely conflicting SSA versions apart.
   The snip (`merge.rs (Merge::snip_reads)`) reroutes the conflicting reads
   through a fresh COPY placed at the value's birth: at the start of the entry
   block for an input Varnode, otherwise directly after the defining op — with
   one carve-out: a value defined by an INDIRECT gets its COPY after the op
   *causing* the effect (the call or store iop-encoded in the INDIRECT's
   input 1; `decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
   (Funcdata::do_snip_reads_insert_point)`), never at the INDIRECT marker
   itself, which sits *before* that op in block order. Without the carve-out
   an out-parameter's post-call copy lands textually above the call that
   fills the slot (upstream `merge.cc:461` "snip must come after OP CAUSING
   EFFECT"; kuna GH-181, where `freecon` then frees the pre-call NULL).
   `Merge::trim_op_output` applies the same INDIRECT rule to its own
   insert-after point. Windows spanning several addresses become
   VariableGroup pieces (`bank_group_with`). *(kuna divergence, documented at
   `merge.rs (Merge::merge_addr_tied)`)*: a forced merge that still intersects
   — a heavily-reused stack slot from compiler stack-coloring — is *skipped*,
   not fatal; upstream's `eliminateIntersect` never fails there, and aborting
   the whole loop would also skip the piece-grouping for every other slot,
   degrading member accesses (`x._4_4_`) into `SUB84/CONCAT44` soup.
2. `merge.rs (Merge::group_partials)` — each registered CONCAT-tree root
   groups its pieces into one VariableGroup so the tree renders as one
   structured value.
3. `merge.rs (Merge::merge_marker)` — every MULTIEQUAL and INDIRECT (except
   indirect-creations) force-merges its output with its inputs
   (`merge.rs (Merge::merge_op)`, INDIRECTs only input 0 via `merge_indirect`).
   When the required tests or the cover forbid a branch, that input is
   *trimmed*: a fresh COPY to a new unique Varnode is inserted just before the
   op (at the end of the corresponding predecessor block for a phi input —
   `merge.rs (Merge::trim_op_input)`), shrinking the input's cover to a point
   so the forced merge goes through. The op itself trims one branch at a time,
   re-testing after each, and as a last resort trims its own output; if the
   required tests *still* fail after all trims, the sweep errors out ("Unable
   to force merge of op"). *(kuna)* `option tiedphitrim` (§6.4) adds one trim
   upstream never makes: a loop head's direct read of an aliased location
   whose loop values are loaded from memory. *(kuna)* Where upstream lets that error abort the
   whole decompilation, `ActionMergeRequired` swallows it at the action
   boundary and keeps the partially merged state — the failure mode is a
   residual un-fused marker op, not a dead function.

The inserted trim COPYs accumulate in the persistent `Merge::copy_trims`
(`funcdata_merge.rs (Funcdata::with_covermerge)` keeps one `Merge` per
function so they survive between actions). Later,
`coreaction_cleanup.rs (ActionDominantCopy)` runs
`merge.rs (Merge::process_copy_trims)`: a high that received two or more trim
COPYs (tracked by the `copy_in1`/`copy_in2` bits) gets them grouped by source
and replaced with a single COPY hoisted to the common dominator block
(`funcdata_merge.rs (MergeContext::build_dominant_copy)`) — one assignment
instead of one per predecessor.

**Copy and speculative merges.** After the forced merges and explicit/implied
marking (below), three passes fuse variables to reduce clutter:

- `coreaction_cleanup.rs (ActionMergeCopy)` → `merge.rs (Merge::merge_opcode)`
  over COPY: a COPY's output and input merge whenever the required rung and
  the cover allow — still a non-speculative merge (only the required tests
  gate it), and the assignment then disappears (marked internal below).
- `coreaction_cleanup.rs (ActionMergeAdjacent)` →
  `merge.rs (Merge::merge_adjacent)`: for every non-call op, the output merges
  *speculatively* with any same-size input whose *local* op type matches
  (adjacent rung); a speculative merge lands in its own merge class on the
  HighVariable, keeping it separable from the forced core.
- `coreaction_cleanup.rs (ActionMergeType)` →
  `merge.rs (Merge::merge_by_datatype)` over the whole loc-ordered Varnode
  set: highs of the identical data-type are grouped and fed to
  `merge.rs (Merge::merge_linear)` — sorted by earliest cover
  (`merge.rs (compare_high_by_block)`), then each candidate merges into the
  first already-accepted variable it passes the speculative rung and the
  cover test with; failures just keep it separate. This is the pass that
  turns dozens of same-typed temporaries into a handful of reused `vN`s.

**Explicit vs implied** — which values get a named token at all. Between the
required and speculative groups, `coreaction_cleanup.rs (ActionMarkExplicit)`
decides per Varnode via `coreaction_cleanup.rs (base_explicit)`: explicit
(gets its own statement/declaration) if it is an input or marker/call output
(but see `foldcallret`, §6.4), belongs to a multi-instance high, is
address-tied or mapped or a proto-partial, has no or too many readers —
more than `max_implied_ref` descendants, which ships at 2
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`; a PTRSUB off the spacebase lifts the cap to
1000000 so stack addresses always fold). A multi-reader implied candidate is
then re-examined for *term duplication*
(`coreaction_cleanup.rs (process_multiplier)`, `(multiple_interaction)`):
inlining it would print its whole expression at every use, so if the
duplicated terminal count exceeds `max_term_duplication`, which also ships at 2
(same anchor), or two marked candidates interact through a bool/extension/PTRADD
op, it is forced explicit.

Both bounds are settable per run rather than compiled in
(`kuna_impliedrefs.rs (OptionImpliedRefs)`, `(OptionTermDup)`): `option
impliedrefs <n>` writes the descendant bound and `option termdup <n>` the
terminal-term bound, each taking a decimal count from 0 to 1000000 (the value
`base_explicit` already uses as its never-explicit sentinel). The two are
independent halves of one decision — `impliedrefs` bounds how many places an
expression is copied to, `termdup` how large the copied expression may be — and
the shipped 2/2 is the value upstream's own comments call "best, in specific
cases a higher number might be good" and "2 and 3 (4) are reasonable", so the
defaults are byte-identical to the compiled-in behaviour. Raising either trades
declarations for expression size: on coreutils `fmt` -O2 (`decompile-all`, 151
functions) the emitted C declares 369 locals at 2, 366 at 3 and 363 at 4, and
384 at 1 and 630 at 0 in the other direction, where nothing stays implied.
`max_implied_ref` is a program-wide tunable rather than a private constant of
this pass: `funcdata_block.rs (Funcdata::bb_is_complex)` — upstream
`BlockBasic::isComplex`, the OR-clause absorb test of `blockaction.rs
(CollapseStructure::rule_block_or)` — and `kuna_condfold.rs` read the same field
to bound how many printed statements a condition block may carry, so
`impliedrefs` loosens those two structure tests along with the marking.

`coreaction_cleanup.rs (ActionMarkImplied)` then walks each
remaining Varnode's expression tree depth-first and marks it implied unless
inlining would be *semantically* unsafe
(`coreaction_cleanup.rs (check_implied_cover)`): a LOAD whose cover crosses a
STORE into the same space with a possibly-aliasing pointer
(`coreaction_cleanup.rs (is_possible_alias)`, recursion depth 2 — proves
difference only through matching op shapes with distinct constant offsets), a
LOAD or call output whose cover crosses any call, or a defining input whose
high would collide after inflating its cover to the candidate's
(`merge.rs (Merge::inflate_test)`, copy shadows again forgiven). The failure
mode of a wrong "implied" is a value printed at a program point where it no
longer holds — which is why every unsafe case resolves to explicit.

Each accepted marking goes through `merge.rs (Merge::mark_implied)`, which sets
the flag *and* dirties the Cover of every operand of the defining op. That
second half is load-bearing, not bookkeeping: it is what makes the transitive
walk of §6.1 re-run, stretching each operand's live range forward to wherever the
inlined expression will actually be printed. The pass is deliberately scheduled
ahead of the speculative merges for exactly this reason — the copy, adjacent and
datatype merges all decide by cover intersection, and they must see the ranges
that inlining created. Skipping the invalidation leaves two values that are both
live at the printed expression looking disjoint, they merge into one variable,
and the emitted C reads the later value where the binary reads the earlier one.

**Phalanx tail.** `coreaction_cleanup.rs (ActionCopyMarker)` →
`merge.rs (Merge::mark_internal_copies)`: COPYs whose input and output landed
in the same high are marked non-printing, as are PIECE/SUBPIECE ops that
merely reassemble or extract pieces of one VariableGroup at their proper
offsets (their operands forced explicit so the group renders through member
notation); a fully-shadowed COPY output with no readers is silenced too, and
highs with multiple surviving COPY-ins get the redundancy resolved
(`process_high_redundant_copy`). "Redundant" there means strictly dominated and
*unshadowed*: `merge.rs (Merge::check_copy_pair)` accepts a later COPY only when
its block is dominated by an earlier COPY from the same source Varnode **and**
nothing writes the shared HighVariable between the two — the dominance range
`funcdata.rs (Funcdata::build_copy_pair_range)` spans the dominant COPY's write
through to the later COPY's read of that source, and any member write landing
inside it vetoes. Getting that range wrong is directly a wrong-value bug: a
`-O0` epilogue reached by several `return param;` paths puts several COPYs of one
parameter in one variable, and if the reload that follows a call clobbering the
same storage is called redundant and silenced, the emitted C returns the call's
result on a path where the binary returns the parameter. Naming
(`coreaction_cleanup.rs
(ActionNameVars)`) and casts (`ActionSetCasts`) close the phalanx but are
policy of chapter [09](09-emission.md). One scheduled body is still inert in
the live tree: `coreaction_cleanup.rs (ActionMergeMultiEntry)` is wired to the
real engine (`merge.rs (Merge::merge_multi_entry)`) but its multi-entry-symbol
source (`funcdata_merge.rs (MergeContext::multi_entry_symbols)`) returns empty
pending the symbol-scope layer.

**Inputs that are only ever read indirectly** (`kuna_indirectonly.rs
(mark_indirect_only)`, run by `coreaction_cleanup.rs (ActionMarkIndirectOnly)`
one slot ahead of `ActionMergeAdjacent`, option `indirectonly`, default off).
Two of the tests above ask whether a variable's input member is an *illegal*
input — an input Varnode that `ActionDirectWrite` never reached, so no formal
parameter can be responsible for its value: typically the leftover initial
contents of a frame slot the function overwrites before it reads. `merge_test_adjacent`
refuses every speculative merge with such a variable, to keep an uninitialized
value from being printed as a real one, and `variable.rs
(HighVariable::has_name)` will not name an unaffected input whose members are
all of that kind. Both carry an exception for the case where the leftover value
is not really read at all, and the exception needs a Varnode flag that says so.

`kuna_indirectonly.rs (check_indirect_use)` decides it, as a worklist over the
def-set. A reader that is a plain `CPUI_INDIRECT` is the accepting case and the
walk stops there — an INDIRECT is the record that *some other op may have
overwritten this storage*, not a use of the value. An INDIRECT caused by a STORE
is accepted too but the walk continues through its output, because the value
survives that store and where it ends up still matters, and a `CPUI_MULTIEQUAL`
is transparent for the same reason. **Any other reader fails the whole test**,
which is what keeps a slot that is genuinely read out of the flag: one
arithmetic use, one compare, one call argument anywhere in the def-set and the
input stays isolated. `mark_indirect_only` applies that to every illegal input
and sets `indirectonly` on the ones that pass, and the two tests above then take
their exception branch.

The visible effect is the merge, and its soundness depends on which side of the
copy the illegal input is. When the slot is the copy's **destination** the
machine really does store into it, and the merge only moves where the value is
computed: into the slot instead of into a temporary that is then assigned across
— and the assignment, now a COPY inside one variable, is hidden by
`mark_internal_copies`. That direction also has a wrong-output face on the
refusing side: on coreutils `ls` at `-O2` the two 8-byte loads that fill a
16-byte `struct dev_ino` on the frame are printed into two register locals and
never into the slot, and the `hash_delete(…, &di)` that follows reads a slot the
emitted C never wrote.

When the slot is the copy's **source** the direction reverses and the merge is
unsound. The machine loads the slot into a register once and mutates the
register; after the merge the emitted C mutates the slot, so it claims a store
the machine never performs, and if the slot's address escaped earlier — the
usual case, since an escaped object is why the slot's entry value was
INDIRECT-only in the first place — a later call handed that object reads a value
that never existed. Nothing in the merge phalanx catches this. A CPUI_INDIRECT
is attached only where the storage is still live in the SSA, so a slot whose
last read happens before the loop carries no INDIRECT at any call after it, its
cover ends at that read, and `merge_test_required`'s cover intersection has
nothing to intersect. This is upstream behaviour, not a porting gap — stock
Ghidra emits the same fabricated store on the same input — so the flag ships
**off** and kuna's default keeps the register local. Turning it on reproduces
Ghidra's partitioning; leaving it off restores the inert stub exactly, and
`indirectonly` then has no writer and both readers take their more-variables
branch.

**Shadow copies** (`kuna_hideshadow.rs (hide_shadow_copies)` driving
`merge.rs (Merge::hide_shadows)`, scheduled as `coreaction_cleanup.rs
(ActionHideShadow)` immediately ahead of the copy marker, option
`hideshadow`, on by default). Two Varnodes reached from one ancestor through COPYs alone
always hold the same value. When the two copy paths are not nested the merges
above leave them as separate assignments of that value into one variable, and
the C repeats the assignment: a `-O0` short-circuit condition that spills a
parameter on both ways into the body prints `(v3 = a2, v4 = a0, v4 = a0, v5 =
a1, ...)`. The pass walks the written def-set, visits each HighVariable once
(deduped through the bank's mark bit, cleared again on a second pass so no mark
survives), and asks `Merge::hide_shadows` to re-point the later copy's input at
the earlier Varnode, turning ancestor → first → second into one chain. The
COPY's input and output are then in the same high, so the copy marker directly
below silences it. Two conditions bound the re-point, and both are about the
value that will be read at the re-pointed input rather than about the text:
`funcdata.rs (Funcdata::varnode_copy_shadow)` must trace both Varnodes to one
common ancestor through COPYs only, and the surviving Varnode's Cover must
contain the other's definition point *strictly interior* (`cover.rs
(Cover::contain_varnode_def)` returning 1, not a boundary hit), so the value is
demonstrably live and unclobbered where the copy is made to read it. Two copies
on exclusive branches fail the second test and are left alone, which is the
common case — over an instrumented sweep of 40 binaries the Cover test accepted
110 candidate pairs and declined 9,138. The Cover read is preceded by a
`MergeContext::bank_update_cover` refresh because kuna's `vn_cover_ref` is a
plain read where the C++ `Varnode::getCover` rebuilds a dirtied cover on the
spot, and the previous iteration's `op_set_input` is exactly what dirties it.
Off, the action returns without walking anything, which is what the tree did
while the body was a stub.

**Closing out the undefined names** (`kuna_undefname.rs
(finish_undefined_names)`, the tail of `coreaction_cleanup.rs
(name_local_highs_angr)`). A Symbol created with an empty name is given the
placeholder `$$undef` plus eight hex digits (`p0_knowledge/database.rs
(Database::build_undefined_name)`), and upstream that string is unobservable
for two reasons kuna does not inherit: the naming pass renames every
still-undefined Symbol before it returns, and the printer reads the Symbol's
display name *live* at emit time. kuna instead caches the name on the
HighVariable (chapter [09](09-emission.md), "where names bind vs. where they
render"), and the cache is written by `varmap.rs
(ScopeLocal::resolve_default_name_override)`, which only renames the covering
Symbol for a *whole-symbol* cover — the C++ `namerec` gate — and otherwise
returns whatever the Symbol is called at that moment. Location order visits a
partial cover of a stack slot before its whole-cover sibling, so the partial
froze the placeholder while the sibling went on to rename the shared Symbol to
its `vN`: one slot, two identifiers, and a body that reads a name nothing ever
writes. Separately, a Symbol that *no* high covers wholly was never renamed at
all and kept the placeholder in the Symbol table, from where it leaked into the
JSON `variables` array and the project export's stack-frame comments. The tail
closes both: it runs the ported catch-all
(`ScopeLocal::assign_default_names` → `Database::assign_default_names`, the
final statement of upstream's naming pass) and then re-resolves every
HighVariable still holding a placeholder through the same containment query
that bound it, which is kuna's stand-in for the printer's live read. Both steps
run after every `vN` has been handed out, so neither renumbers an existing
local: a Symbol named here takes the highest index in the function, and the
re-resolve consumes no index at all. No `$$undef` string reaches any surface
(DIV-58); an identifier containing `$$` is not valid C under any naming policy,
so this is unconditional rather than a settable.

## 6.2 The stack frame

Stack locals do not exist until this phase builds them; before it, the frame
is anonymous memory reached through the stack pointer. The promotion chain has
three links. First `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_spacebase.rs
(Funcdata::spacebase)` (driven by `ActionSpacebase` in `mainloop`) marks every
Varnode holding the stack-pointer value `spacebase` and types the input
register as a pointer into the stack space. Second, the `stackvars` rules in
`mainloop`'s second op pool, `oppool2` (`RuleLoadVarnode`/`RuleStoreVarnode`, registered in
`decompiler/crates/kuna-decomp/src/infra/universalaction.rs (universal_sched)`)
rewrite `LOAD/STORE` through a spacebase-relative pointer into direct
`(stack, offset)` Varnodes. Third, the stack-pointer *flow* itself is kept
affine by `ActionStackPtrFlow` (§6.5). Everything below consumes the
`(stack, offset)` Varnodes those links produce.

**Keeping non-locals out** (`decompiler/crates/kuna-decomp/src/p4_calls/coreaction_protos.rs
(ActionRestrictLocal)` → `funcdata_spacebase.rs (Funcdata::restrict_local)`,
in `mainloop`). Stack storage that belongs to someone else is marked
*not-mapped* before layout ever runs: the stack homes of locked parameters of
every sub-call (they are the callee's), and saved-register COPYs of unaffected
registers (calling-convention bookkeeping, not a variable). The same
mechanism (`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs
(ScopeLocal::mark_not_mapped)`) later excludes an unaliased return-value
staging slot (`funcdata_spacebase.rs (Funcdata::check_unaliased_return)`).

**Reporting the frame vs. reporting the declarations (`option framelayout`).**
Because the layout is rebuilt from scratch every pass, a slot is only in the
FINAL layout if a stack Varnode for it was still live at the last pass. At -O0
every local is a spill slot, so `RuleLoadVarnode`/`RuleStoreVarnode` convert the
store/load pair into a sized stack COPY, copy-propagation folds it, and dead-code
removal takes the Varnode with it; the slot is then in an early layout and in no
later one. The emitted C is right to drop it — there is no expression left to
declare — but the *frame* still has the slot, and
`decompile_drive.rs (extract_variables)` describes the recovered frame, not the
printed declarations, which is what IDA's stack view and Binary Ninja's variable
list are the analogues of and what decbench's `type_match` metric reads. So
`restructure_varnode` folds each pass's NO_CATEGORY stack symbols into a
per-`Funcdata` union (`funcdata.rs (Funcdata::record_frame_slots)`, first writer
wins because the earliest pass saw the most dataflow standing), and under
`option framelayout` (default ON, DIV-97) `extract_variables` appends every
recorded offset no parameter or surviving local already covers. Two spellings are
normalized on the way out: a slot the type system never committed to is carried
internally as `xunknown1[N]` and would render as `char[N]` — an element type the
recovery never established — so it is reported as the width-only `undefined<N>`;
and Ghidra's internal `$$undefNNNNNNNN` placeholder for an unnamed symbol is
renamed to its stack-view form `local_<hex>`. The union is recorded
unconditionally (one map insert per stack symbol per pass) and read only by
`extract_variables`: no p-code, no emitted C, so neither structure nor
recompilation can move.

**Reporting an uncommitted byte (`option bytehonest`).** The same distinction
applies one type at a time, to the parameters and surviving stack locals of
sections 1 and 2. A one-byte value the type system never committed to is carried
as a size-1 `TYPE_UNKNOWN`, and the default `realtypes` rendering spells that
carrier `char` — the only one-byte C type that reads as a value, and therefore
the right choice for the emitted C. On a machine-readable surface it is the wrong
one: it asserts an element type the recovery never established, where the
program's own type may as well be `_Bool`, `unsigned char`, a one-byte enum or one
byte of a struct nobody split. Under `option bytehonest` (default ON)
`decompiler/crates/kuna-decomp/src/p6_variables/kuna_bytehonest.rs
(exported_type_name)` reports the width instead — `undefined1`, the spelling
Ghidra uses for the same fact and the analogue of IDA's `_BYTE`. The predicate
(`is_uncommitted_byte`) reads the datatype's own metatype and size, never its
spelling, so it behaves identically with `realtypes` off (where the row would
otherwise export kuna's internal `xunknown1`); it is deliberately narrower than
the frame-slot speller above, which also rewrites an array of unknown: size 1
exactly and never an array, because `char[264]` is a buffer whose array-ness is
the honest part of the answer and `undefined264` would drop it. The exported
`size` is the datatype's size, unchanged.

This is the one place where the two surfaces of the same variable disagree on
purpose: the `.c` text says `char a0` and `variables[]` says `undefined1` for the
same parameter. The divergence is the point — the C text is a *rendering* that
must be legal C, while `variables[]` is a *report* of what the recovery
established — so the option changes no p-code and no emitted C, and a consumer
that wants the printer's spelling on both surfaces flips it off.

The JSON use evidence is joined after extraction. A scalar high can be matched by
its exact storage, but an array's emitted uses normally belong to smaller highs for
individual elements or to constants representing the aggregate's base address.
When several highs carry the same printed name, `decompile_drive.rs
(high_matches_stack_variable)` therefore admits a stack Varnode only when its full
byte range is contained by the reported variable, and admits an address constant
only when it names the variable's base. Name plus containment prevents a
fragment from a neighbouring frame object from acquiring the array's lines while
preserving aggregate and element uses.

**RangeHint gathering.** Each `mainloop` pass,
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionRestructureVarnode)` rebuilds the frame layout from scratch:
`funcdata_spacebase.rs (Funcdata::restructure_varnode)` clears the unlocked
auto-recovered stack symbols (a stale early-pass symbol must not outvote the
current Varnodes) and fills a `varmap.rs (MapState)` with typed hints — one
`varmap.rs (RangeHint)` per source:

- `funcdata_spacebase.rs (Funcdata::gather_varnodes)` — a *fixed* hint per
  live stack Varnode, from its current data-type;
- `funcdata_spacebase.rs (Funcdata::gather_open)` — an *open* (array-like)
  hint per pointer into the stack found by the alias checker; if the pointer
  arithmetic had an index Varnode, at least indices [0,3] are assumed
  (`min_items` 3). The same routine then walks the heritage LOAD/STORE
  guards (chapter [03](03-ssa-and-simplification.md)):
  `funcdata_spacebase.rs (Funcdata::add_guard)` turns each guard whose range
  the value-set refinement locked (`option loadguardrange`) into an open hint
  at the guard's minimum with the **real** index bound,
  `highind = ((max - min) + 1) / step - 1` — the only hint source that can
  push an indexed array's extent past the [0,3] fallback (an unrefined or
  step-less guard contributes nothing, and an unlocked-but-stepped one
  contributes the same [0,3] floor). This is what keeps element 4+ of an
  indexed stack array inside the array instead of splitting off as a
  separate, never-assigned scalar when `RangeHint::attempt_join` compares
  distance against `highind`;
- `varmap.rs (MapState::gather_symbols)` — a hint per already-mapped Symbol
  (locked ones carry the `TYPELOCK` flag).

The parameter range from the prototype is subtracted from the analyzed range
up front (`varmap.rs (MapState::new)`), so parameter recovery (chapter 04)
owns that region.

**The layout decision** (`varmap.rs (ScopeLocal::restructure)`). Hints are
sorted by the *output-determining* comparator — signed start, then size
(small first), then range type (`fixed < open < endpoint`), then flags, then
array-index bound; the data-type is deliberately *not* part of the sort key
(`varmap.rs (RangeHint::compare)`), and equal-(start,size,flags) runs collapse
to the most specific type (`varmap.rs (MapState::reconcile_datatypes)`). A
sentinel `endpoint` hint bounds the walk (`varmap.rs (MapState::initialize)`).
Then a single sweep unions intersecting hints (`varmap.rs (RangeHint::merge)`)
under a fixed conflict policy: can the two types *line up* (one is a
component of the other at the right alignment —
`varmap.rs (RangeHint::reconcile)`)? If yes, prefer the type-locked, then the
fixed-over-open, then the larger-fixed, then the more specific type
(`varmap.rs (RangeHint::preferred)`). If no, and neither is locked, the merged
range *concedes*: it becomes a fixed unknown-typed blob of the union size (or
a 1-byte open range if that size isn't 1/2/4/8). Two locked types that cannot
reconcile are the hard failure ("Overlapping forced variable types" — the
user pinned contradictory truths); a lock beaten only by position discards
the other hint entirely. Non-intersecting neighbors may still *join* into one
array when an open hint's element type and alignment continue
(`varmap.rs (RangeHint::attempt_join)`, including absorbing constant-only
COPY ranges). Every surviving range becomes a Symbol in the local scope
(`adjust_fit`/`create_entry`), and `funcdata_spacebase.rs
(Funcdata::sync_varnodes_with_symbols)` paints the resulting
`mapped`/`addrtied`/`addrforce`/`nolocalalias` flags (and, in the final sync
only, data-types) onto the Varnodes. After `fullloop` exits,
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionMappedLocalSync)` runs that final data-type-updating sync once; its
failure mode is tolerance, not an abort — the layout keeps the conceded
unknowns (upstream additionally emits a "Could not reconcile some variable
overlaps" warning header; kuna stubs that diagnostic).

**Terminator absorption** (`option nulterminator`, **opt-in, default off**). An
open hint that `attempt_join` cannot extend ends where the next hint starts, so
`strncpy(buf, s, sizeof(buf) - 1); buf[sizeof(buf) - 1] = 0;` splits into `char
buf[63]` plus a separate `char` assigned 0 and never read: the terminator's
constant index is folded into a direct frame store, and a NUL-scan loop leaves
`loadguardrange` no range to lock. With the option on,
`funcdata_spacebase.rs (Funcdata::gather_varnodes)` records, for every frame
slot a constant COPY writes, whether each such write is a zero that no op reads
directly (`varmap.rs (MapState::note_terminator_store)`), and after
`attempt_join` declines a hint,
`decompiler/crates/kuna-decomp/src/p6_variables/kuna_nulterminator.rs
(close_at_terminator)` closes the open range into a *fixed* range ending one
element after that hint -- when the range is open and unlocked with one- or
two-byte integer elements, the hint is a plain fixed constant-COPY slot of
exactly one element at a whole-element offset whose every write is an unread
zero, the hint after it starts at or past the slot's end, no plain frame
address (an open hint with no index, `varmap.rs
(MapState::swept_address_inside)`) starts strictly inside the array, and
`adjust_fit` keeps the extent. The result is footprint-identical to upstream's
`char[N]` plus the trailing `char`: it merges two adjacent declarations and
never reaches padding or a non-adjacent slot.

**Why it is off by default.** The same frame shape -- an open `char` array
followed by a lone unread zero at the next whole-element offset -- is produced
by the struct-first-member idiom `struct { char name[N]; T field; } u;
strncpy(u.name, s, N); u.field = 0; ...; f(&u);`, where the zero is a sibling
*field* the callee reads through `&u`, not the array's terminator; a spilled
scalar whose value is read from a register is indistinguishable too (its stack
slot is written but never read *from the stack*). There is no sound frame-level
signal that separates a genuine `buf[N-1] = 0` terminator from a following
field, so on by default the pass over-extends a correctly-sized `char name[N]`
to `[N+1]` in ordinary gcc/clang/MSVC output. It is therefore an opt-in tool:
flip it on for a string-heavy target whose stack buffers are known to be
NUL-terminated strings. Off, the layout is upstream's split.
**A pointer walk's buffer and its end bound (kuna `endptrbound`, default on).**
A loop that walks a stack buffer with a pointer stops on the address one past
the buffer's last element, and nothing in the hint sources above says that
address belongs to the buffer. It is a spacebase-relative constant like any
other, so type propagation resolves it to whatever Symbol starts there — the
next local — and the bound renders as that local (`while (p != v2)`). The
comparison then pulls the neighbour's pointer type into the walking pointer,
the open hint the walk contributes takes that type, and the walked bytes are
declared as a scalar of the neighbour's width (`unsigned int v1;` for eight
bytes). The trip count, which is the buffer size, is not recoverable from the
C. (A constant initializer that writes the buffer in wider pieces keeps that
form, `v1._0_4_ = ...`, whichever way the buffer is declared.)

`decompiler/crates/kuna-decomp/src/p6_variables/kuna_endptrbound.rs
(find_pointer_walks)` accepts a walk during `restructure_varnode` only when it
can prove that every dereference through the pointer lands in `[start, end)`.
The candidates are the `MULTIEQUAL`s that some LOAD or STORE reads through,
possibly plus constants. `kuna_endptrbound.rs (walk_loop)` requires the phi to
head a natural loop: every input arriving on an edge from a block the phi's
block dominates (a back edge) is the phi plus one positive constant `step`,
every other input is one stack address `start`, and the loop body is the header
plus every block that reaches a back-edge source without passing the header.
Casts and copies are looked through; an INDIRECT is not, anywhere, because the
call it stands for may have changed the value. `kuna_endptrbound.rs
(pointer_derefs)` then accounts for every use of the pointer: values at a known
constant offset flow through casts, copies and constant additions; each
dereference must fit inside one element (offset `0 <= off` and
`off + width <= step`) and one must be exactly `step` bytes at offset 0;
comparisons read no memory; a value narrower than a pointer, or the pointer's
difference with a stack address, is an integer and is not followed. A variable
offset that reaches a dereference, or the pointer reaching a call, an INDIRECT,
a store as a value, a return, or any other phi, declines the walk.
`kuna_endptrbound.rs (controlling_bounds)` looks for the comparison that bounds
the loop: `t == end`, `t != end`, `t < end` or `end <= t`, where `t` is the
pointer itself or the one value every back edge carries, and `end` is a stack
address (followed through a loop-invariant phi to the address that seeds it).
`kuna_endptrbound.rs (exit_branch)` requires the comparison to reach, through
`BOOL_NEGATE` and copies, the condition of a `CBRANCH` in a loop block that
dominates every back-edge source and whose one out-edge leaves the loop — the
edge taken exactly when `t` reaches `end`, with the branch's boolean flip
honoured — while the other stays inside. When `t` is the pointer itself, every
dereference must also sit in a loop block that branch strictly dominates. By
induction over the back edges the pointer is then `start + i * step` with the
element `i` short of `end` at every dereference, whatever other exits the loop
has. A comparison whose branch stays inside the loop (a midpoint test), leaves it
the wrong way, can be bypassed on the way to a back edge, or feeds no branch at
all proves nothing and is ignored. Every comparison that controls an exit this
way must name the same `end`, at least one must keep the dereferences inside the
walk on its own, and `end` must be at least two and a whole number of steps
beyond `start`.

For each walk, `kuna_endptrbound.rs (coalesce_hints)` runs after gathering and
before the layout decision and replaces every hint that starts inside
`[start, end)` with one array hint of `step`-byte elements, typed by the most
specific `step`-wide scalar hint already in the range (so a `char` walk stays
`char` and constant-sequence recovery still sees a character array) or by the
unknown type of that width. The hint is open, with index evidence through the
last element, so it extends exactly as far as the open hints it replaces would
have: to the next frame reference, which a bound written as the frame address
`end` itself provides. The walk never makes the array longer than the frame
would otherwise have made it. The walk contributes nothing when that would
contradict other evidence: a hint that straddles either edge (an open hint
included), a type-locked Symbol, a fixed hint holding a non-constant value that
is typed or is not exactly one element wide (a separately used variable, or the
range also read as one wider value, as a union's scalar member is), typed
open-hint index evidence inside the range whose element width is not the step
(a byte walk that clears an `int` array the function also indexes as ints keeps
`int v1 [4]` and its indexing), open-hint index evidence reaching past either
edge, a range outside the analyzed window, or another walk that overlaps the
range without one of the two containing the other. Nested walks resolve to the
outermost: the array hint the outer walk leaves carries index evidence that
reaches past the inner walk's edge. Once the layout holds an array
Symbol that covers exactly `[start, end)` — this pass's or a locked one from
debug information — `kuna_endptrbound.rs (rebase_bounds)` rebuilds a compared
address written as `sp + end` into `PTRSUB(sp, start) + (end - start)`, which
renders `&buf[n]`; the array's slack makes the form stable against
`RulePtrsubUndo`, and a bound already in that form is not rebuilt again. The
rewrite is made at the address's defining op when every use of it ends at that
comparison (through casts, copies and phis), so a source-level `end` variable
reads `end = &buf[8]`; otherwise a fresh expression is spliced in front of the
comparison alone, and an address that also means the neighbour elsewhere — the
same register handed to a call as that object — keeps naming the neighbour
there. The value compared never changes, and a rewrite is counted as a change
of `ActionRestructureVarnode` so the next inference pass types it. The rebuilt
`PTRSUB(sp, start) + span` is itself an additive stack reference at `end`, so
the alias gather still hands later layouts an open hint there and the rewrite
never changes the layout that justified it. `option endptrbound off` restores
the neighbour-bound layout.

**Alias blocking.** The `varmap.rs (AliasChecker)` collects every pointer
into the stack by walking additive expressions rooted at the spacebase input
(`funcdata_spacebase.rs (Funcdata::gather_additive_base)`), converts each to a
frame offset, and — the load-bearing heuristic — treats **everything after
the shallowest alias as potentially aliased** (`varmap.rs
(AliasChecker::gather_internal)`: `alias_boundary` is the minimum local-side
offset; `varmap.rs (AliasChecker::has_local_alias)` answers `offset >=
alias_boundary`; offsets below the parameter boundary, default `0x1000000`,
don't count, and on positive-growth stacks the test is disabled entirely —
queued outgoing parameters would always sit "after" the locals). On every
pass but the first (`ActionRestructureVarnode`'s `aliasyes = numpass != 0` —
first-pass aliases are unreliable because the pointer forms haven't
simplified yet), the sorted alias list drives
`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
(Database::mark_unaliased)`: every stack Symbol *not* crossed by an alias
(walked in lock-step with the mapped ranges; an alias more than `0xffff`
bytes back is ignored, and aliases don't propagate through unmapped gaps)
gets the `nolocalalias` flag. That flag is what lets `RuleIndirectCollapse`
delete the per-call INDIRECT effects heritage placed on the local — without
it every call kills every local's value. This is exactly where **`option
aliasblock`** acts (`alias_block_level` on the architecture, default 2): a
*type-locked* Symbol *stops alias propagation* by dtype class (aliases do not travel through it; the locked symbol itself keeps an alias that already reached it) — level 0
never, level 1 locked structs, level 2 (the default) locked structs and
arrays, level 3 every locked Symbol. The trade is explicit: raising the level
trusts the user's types over the pointer analysis; lowering it keeps
stores-through-pointers alive at the cost of INDIRECT clutter. Two tail
refinements: a return value staged in an unaliased stack slot is unmapped
rather than promoted to a spurious local (`funcdata_spacebase.rs
(Funcdata::check_unaliased_return)`), and a *zero-offset* use of the raw
stack pointer gets a `PTRSUB(sp, #0)` spliced in so the type system renders
`&local` instead of the bare register (`funcdata_spacebase.rs
(Funcdata::annotate_raw_stack_ptr)`).

**What counts as a pointer escaping (kuna `cookiescramble`, default on).**
The escape sites `gather_additive_base` records are the *non-additive* uses of
a stack-pointer-derived Varnode: the walk follows `COPY`/`INT_ADD`/`PTRADD`/
`PTRSUB`/`INT_SUB` chains and treats every other use as "this address left our
sight". An `INT_XOR` is not an address computation, and MSVC's `/GS` prologue
mixes the raw stack pointer into the frame cookie with one (`mov rax,
[__security_cookie]; xor rax,rsp; mov [rsp+N],rax`). Read as an escape it
records a site at the *bottom* of the frame — the shallowest offset there is —
so `has_local_alias` answers yes for every stack location in the function.
That answer is consumed by the call-site input recovery
(`funcdata_callsite.rs (check_input_trial_use)`, §4): a stack argument trial
whose slot is locally aliased is scored *no-use*, its CALL input is replaced
with a constant `0`, and the argument's computation is dead-code eliminated.
The result is that every stack-passed argument at every call site in a `/GS`
function is dropped — visibly, the variable tail of a `...` prototype never
appears. With `cookiescramble` on, an `INT_XOR` no longer records an escape
site, and the boundary is decided by the genuine address-forming uses. The
exemption applies **only** to the checker the call-site recovery builds
(`Funcdata::build_alias_checker_deferred` / `Funcdata::alias_gather_access`);
the local-layout gather (`gather_open`, which drives the `RangeHint` open
ranges above) always answers upstream, so stack-variable layout is unchanged
either way. The rule is not conditioned on the XOR's second operand — whether
the cookie is loaded or has been folded to an immediate is a property of the
optimizer, not of the aliasing — and its cost is a deliberately masked pointer
(`p ^ mask`, dereferenced after a second `^ mask`), whose base stops counting
as escaped; `option cookiescramble off` restores upstream's answer.

**Name recommendations.** A namelocked-but-NOT-typelocked local never
survives restructure — `clearUnlockedCategory(-1)` removes every non-typelocked
category-less symbol at the pass head — so its *name* survives separately: C++
harvests such symbols into `ScopeLocal::nameRecommend` records and re-applies
them at naming time (`recoverNameRecommendationsForSymbols`, varmap.cc:1050 —
run at the top of `ActionNameVars::apply`).  The kuna port carries the record
type (`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs
(NameRecommend)`) with a list on the scope
(`ScopeLocal::add_recommend_name`/`ScopeLocal::name_recommendations`) and
applies it in the `ActionNameVars` port
(`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(recommended_name_for)`): a high whose name representative matches a record's
storage + size wins the recommended name — the use-address selects the arm
(invalid = address-tied whole, `entry-1` = a function input, else the defining
write's address) — before both the container bind and the `vN` allocator.
A variable whose storage is a HASH rather than an address needs the parallel
list: C++ keeps `dynRecommend` and re-applies it through
`DynamicHash::findVarnode` (varmap.cc:1557-1573).  kuna ports that too
(`ScopeLocal::add_recommend_dynamic` /
`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::kuna_apply_dynamic_recommendations)`): the recorded hash resolves
back to its Varnode, and when that Varnode's high is still unnamed it takes the
recommended name AND a dynamic Symbol carrying the same hash — so the
re-encoded `<localdb>` hands Java a `<mapsym type="dynamic">` it resolves to
the very variable the user renamed.  The hash is computed with the upstream
budget of 8 because Java hardcodes the same (`DynamicHash.java:440`) and a hash
the two sides disagree on cannot round-trip — deliberately NOT kuna's
`dynamichashmax` option, whose value only has to satisfy kuna's own analysis.

PLACEMENT is a real divergence.  Upstream runs the `dynRecommend` loop AFTER
`linkSymbols`, so every high already carries `getSymbol()` and the loop merely
RENAMES an existing Symbol under three guards (`sym == 0`, wrong scope,
`!isNameUndefined`).  kuna's naming pass fuses linking with the `vN` default
assignment into one location-ordered walk, leaving no "after linking, before
defaults" point; the loop therefore runs FIRST and CREATES the dynamic Symbol.
Because no high is named at that moment, the ported per-high guard is vacuous,
so the equivalent guard is applied against the SCOPE instead: a hash landing on
storage the walk is about to bind to a real Symbol — a `function_parameter`, or
any Symbol that already has a defined name — is skipped.  Without it a stale or
shape-shifted host hash could take a parameter's variable, and that high's
`<high symref>` would stop pointing at the parameter.

The lists' only producer today is ghidra-mode's host-`<localdb>` seeding (the
GUI rename persistence loop, chapter [00](00-overview.md)); the standalone
pipeline never adds a record, so both passes are structurally inert there.
The C++ `collectNameRecs` harvest (standalone symbols → records) remains an
unported follow-up.

**The scope wire encode.** The whole local scope marshals out for the
ghidra-mode `decompileAt` response as the `<localdb>` element
(`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs
(ScopeLocal::encode)`, the varmap.cc:462 port): the `main=` stack space and
`lock=` attributes, then the `<scope>` document
(`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs
(Database::encode_scope)`) — the positional `<parent>` + `<rangelist>` pair
(Java's `LocalSymbolMap.decodeScope` skips its first two scope children
blind, so both are always written; the parentless private database writes its
own id), then the `<symbollist>` of `<mapsym>`s in nametree order.  Each
mapsym is `Symbol::encode` (header: name, the UNCONDITIONAL nonzero id, the
lock/storage flag attributes, `cat` and — for a parameter — the slot `index`;
body: the data-type reference) followed by its storage entries
(`SymbolEntry::encode`: a `<hash>` for a dynamic entry, a plain `<addr>`
otherwise, each with its uselimit `<rangelist>`; piece entries are skipped).
Category-0 symbols with exact parameter storage are what the Java rename path
compares against the database (`checkFullCommit`) — a mismatch would turn
every rename into a whole-signature rewrite.  Symbols with no entry, no id,
or a zero-sized type are skipped defensively (each is a Java-side hard throw
that would discard the entire decompile result); none arises through the kuna
creation paths.

## 6.3 Dynamic hashes

A stack local is addressed by its offset, a register by its storage — but a
decompiler-internal temporary or a constant has no storage that survives
re-decompilation, so an assertion attached to it ("this constant is an
equate", "this union access uses field X") needs a different identity. The
identity is a 64-bit hash of the value's *local data-flow neighborhood*
(`decompiler/crates/kuna-decomp/src/p6_variables/dynamic.rs (DynamicHash)`):
starting from the root Varnode, grow the sub-graph one edge layer at a time
(four methods of increasing reach for Varnodes, methods 0–3; three for op/slot
roots, methods 4–6), fold every edge through CRC32 in a deterministic order,
and pack the 32-bit neighborhood hash with the root's op-code, slot, and
method. Op-codes are hashed through a lumping table
(`dynamic.rs (TRANSTABLE)`) so cosmetic rewrites don't change identity:
NOTEQUAL hashes as EQUAL, SUB as ADD, PTRADD/PTRSUB as INT_ADD, CAST is
skipped entirely.

**Uniqueness under repetition** (`dynamic.rs (DynamicHash::unique_hash)`).
Structurally identical code (an unrolled loop) produces colliding hashes at
one address. The resolver cycles the methods, collecting for each the list of
same-address Varnodes with an equal comparable hash, and keeps the *smallest*
collision list not exceeding the budget; a singleton wins immediately. If no
method reaches uniqueness, the champion list's *position* and *total* are
packed into dedicated bit fields of the hash (bits 49–51 and 52–54), making
"the 3rd of 5 identical values at this address" a stable identity. When even
that fails — every method's list overflows the budget — the hash is 0 and the
consumer errors: `decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::build_dynamic_symbol)` aborts the request with "Unable to find
unique hash for varnode".

*(kuna, Ghidra issue GH-8467)* **`option dynamichashmax`** — upstream
hard-codes the collision budget at 8, which dense unrolled AArch64/Go NEON
loops overflow, turning the whole decompilation into that abort. The option
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_dynamichashmax.rs
(DynamicHashMaxOption)`) raises the budget to 16: collision lists of up to 8 still encode exactly in
the 3-bit position/total fields, and a 9–16-entry champion escapes the
"Unable to find unique hash" abort — its overflowing position/total then makes
re-binding fail dormant rather than mis-bind. Default **on** (DIV-3); `off` restores the
upstream budget and the abort. The live gate is
`Architecture::dynamic_hash_maxdup_high`, resolved to 8/16 at the call sites
(e.g. `decompiler/crates/kuna-console/src/ifacedecomp.rs` for the console
`force varnode` path).

**Addressing across re-decompilations.** A dynamic Symbol persists as
(first-use address, hash). On the next run,
`dynamic.rs (DynamicHash::find_varnode)` gathers the Varnodes at that address,
re-computes their hashes with the recorded method, and re-binds on match; an
IR that simplified differently simply fails to match, and the assertion lies
dormant rather than mis-attaching. Two scheduled consumers do the binding:
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionDynamicMapping)` inside `mainloop` — the *early* mapping, which pins
the matched Varnode as mapped storage precisely so copy-propagation cannot
dissolve the value before anyone sees it — and `(ActionDynamicSymbols)` in the
post-fixpoint tail (twice: once inside the merge phalanx, once before naming)
for the final attachment the printer reads. Because kuna's fullloop *does*
collapse the carrier COPY that upstream preserves, `ActionMergeRequired`
re-runs the (idempotent) early mapping right after the forced merges
re-materialize those COPYs (`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionMergeRequired)`), so the Symbol guard of `merge_test_required` (§6.1)
still sees the binding and keeps the dynamic temporary a distinct variable.

## 6.4 kuna extensions

Shipped defaults below are the `phases.toml` settable rows (mirrored into
`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`); module doc-comments in this folder predate the
default flips and may state stale defaults — `phases.toml` is the source of
truth.

**(angr) `option foldcallret` — call-return folding into the use site**
(default **on**, DIV-14). Ghidra forces every call output explicit
(`base_explicit`'s call arm, §6.1), so a call result always spills:
`v5 = timespec_cmp(...); if (v5 <= -1)`. angr folds a single-use call return
into its consumer. The kuna port
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(call_output_foldable)`) relaxes the forced-explicit rule only when the fold
is provably order-safe — marking a call output implied *moves the call's
evaluation* to the use site, so the predicate demands: exactly one descendant
and it is not a phi (no single textual evaluation point); use in the *same
block*, strictly after the call; no op between them that is a call or
touches memory (LOAD/STORE/CALLOTHER — LOAD included because the call may
STORE what the intervening LOAD reads); and no non-marker op between them
that *reads a value the call indirectly writes* — an input defined by an
INDIRECT whose iop input names this call (kuna GH-181: the snipped
out-parameter COPY `Merge::snip_reads` places right after the call would
otherwise have the call text sunk past it, handing it the pre-call value;
marker ops are skipped since a later call's own INDIRECTs chain the earlier
call's versions without any textual evaluation point). Anything else stays
explicit: false negatives over reordering bugs.

A barrier set stated in opcodes is not the whole of "writes something the callee
can read". Heritage promotes a write to a fixed global address into a plain
`CPUI_COPY`, which no opcode test catches, so `int t = f(7); k = 42;
return t + k;` once folded to `k = 0x2a; return f(7) + 0x2a;` — the binary
returns 50, that C reads 91 (kuna GH-657). The barrier set is therefore stated
in storage as well as opcodes
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(op_is_barrier)`): a barrier is a call, a `CPUI_LOAD`/`CPUI_STORE`/
`CPUI_CALLOTHER`, **or** any op whose output varnode is persistent or
address-tied
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(op_writes_tied_storage)`). A global's output is persistent; a frame slot's is
tied to its stack address, and the callee can reach one whenever the frame
address escaped into it — an escaped slot is *not* held back as a `CPUI_STORE`,
heritage promotes it like any other. The cost is declining over a frame slot the
callee could not have reached.

One op with a tied output writes nothing: a `CPUI_COPY` back into the storage
its input already occupies
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(op_is_self_copy)`). `RuleIndirectCollapse` leaves that shape behind — `glob =
COPY glob` — wherever a call turns out not to write a global it carried an
INDIRECT for, and it stores the value that is already there, so it is exempt
(a volatile location is not: there the access itself is the effect).

That span ends at the single use, which is the call's textual home only when the
use op is itself a statement; when the use op's own output is *implied* the
expression keeps travelling and is evaluated wherever that value is finally
consumed. So the call output is re-examined in `ActionMarkImplied`
(`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(check_implied_cover)`), where the descendants-first walk has already classified
the chain below the use and the landing statement is therefore derivable
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(print_chain)`); a landing statement in another block, or one with a write in
between, keeps the call spilled
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(fold_print_point_is_order_safe)`). The ops of that chain are themselves exempt:
each consumes the previous one's value, so the call is evaluated before them in
the folded text exactly as it is in the binary.

Over that second span the question narrows to **writes**
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_callretfold.rs
(op_is_write_barrier)`): a call, a `CPUI_STORE` or `CPUI_CALLOTHER`, or a write
to persistent or address-tied storage. `CPUI_LOAD` is a barrier only up to the
use. Both halves of that split are measured on the 37-binary sweep in
`docs/features/gh657/`. Up to the use a `LOAD` is a hazard this pass can decide —
the call may `STORE` what it reads, and the use is where the folded text lands,
so the `LOAD` really is something the call moved across. Past the use the reads
left in the span are the ones the folded expression is printed *beside*, inside
the single statement the whole implied chain collapses into, so de-folding over
them repositions no statement: counting them costs 117 further changed functions
over the 37-binary sweep, and the shape that dominates is `__ctype_b_loc()`
inlined into the same conditional as the loads it now sits next to. The INDIRECT
half of the predicate stays on the span to the use for the same reason: asking it
past the use changes 31 functions across `grep` O0, `tar` O0 and `ssh` O2 and
de-folds 27, the dominant shape being `dat_33798 = *__errno_location();`, which
is `call 4890; mov (%rax),%eax; mov %eax,0x33798` in `grep` O0 `sub_6c53` — one
call, one load, one store, in exactly that order. `foldcallretphi` asks both
questions in full over the whole distance, for its own folds, because those are
the ones it released; the `foldcallretphi` entry below has that half. Provenance:
`docs/features/gh657/`.

The direct call output may have one descendant even though a derived value
later fans out. For example, `u = (ushort)f()` gives the call one `SUBPIECE`
descendant, while `u` can feed a loop comparison and a post-loop store. If the
ordinary multiplier analysis made `u` implied, the printer would recursively
emit `f()` at both sinks. Before multiplier analysis, kuna therefore walks an
implied expression at each multi-use root back through its defining ops. If it
reaches a call that passed the fold predicate, the derived root remains explicit
and becomes the single textual evaluation point. Already-explicit inputs stop
the walk. The traversal follows only operands that the ordinary multiplier
analysis can print as part of the expression: it skips a LOAD's space input, a
PTRADD's multiplier, and SEGMENTOP metadata.

When the predicate passes, the output falls
through to the ordinary implied machinery of §6.1 — the fold itself is just
`if (timespec_cmp(...) <= -1)` emerging from the printer's normal recursion.
`off` restores the upstream always-spill form byte-for-byte; four datatest
files pin that form via per-test opt-outs (DIV-14, `docs/history.md`).
Provenance: `docs/features/call-return-variable-folding-dcde82/record.json`
(ablation: 5 upstream assertions change; measured speed delta −3.2%).

**(angr) `option foldcallretphi` — folding past the merge phalanx**
(default **on**). `foldcallret` relaxes only the first of two gates. A call
output it lets through is re-examined by `ActionMarkImplied` in
`coreaction_cleanup.rs (check_implied_cover)`, whose third arm
(`merge.rs (Merge::inflate_test)`) forces a value explicit when one of its
operands has another live SSA version of the same HighVariable over the
candidate's internal cover — inlining would then print an operand where a
different version of it holds. Upstream never reaches that arm with a call
output, because Ghidra marks every call output explicit one pass earlier, so it
is only with `foldcallret` on that it sees this shape at all. What it finds is
usually self-inflicted: a call may write any global, so the call carries a
`CPUI_INDIRECT` over every global it might touch, and passing one of those
globals as an argument (`v3 = sub_3700(stdin,v7); v1 = 1; v12 &= v3;`) is enough
to put a second version of the operand's high over the call output's cover. The
colliding version is produced by the very call being moved.

`decompiler/crates/kuna-decomp/src/p6_variables/kuna_foldcallretphi.rs
(conflict_is_self_call_effect)` discounts exactly that case: the rejection is
ignored only when at least one instance of the operand's high collides and
*every* colliding instance is the output of an INDIRECT whose effect op is this
call. The collision is about versions, not order: the folded text performs the
operand read and the call's own write at one point, exactly as the spilled form
does. Three conditions bound the discount itself. First, a high that belongs to
a `VariableGroup` declines: `inflate_test`'s second loop reasons about
overlapping storage rather than versions, and its rejections are never
discounted. Second, a use op that itself reads an INDIRECT effect of the call
declines, since the folded text would otherwise name the operand's high both as
the call's argument (pre-call) and as an operand of the use (post-call). Third,
the call's output has to carry everything the callee returns
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_foldcallretphi.rs
(call_output_is_full_width)`). A locked output is the callee's declared type and
qualifies. An unlocked one qualifies only when no wider return storage contains
it. An unlocked `eax` read out of `rax` does not qualify: the callee may write
all of `rax`, and its own decompilation prints that width (`sort` O2's
`sub_c5b0` is `unsigned long` and ends in `return 0xffffffff;`), so the spilled
local's `int` declaration is the only place the narrowing is written down.
Folding it away turns `v15 = sub_c5b0(stdin), v15 == -1` into
`sub_c5b0(stdin) == -1`, which against that prototype is never true. The
return registers are one place to look, and the model's answer for a wider
integer is the other: the discount asks where the model would return an `int`
of each power-of-two size above the output's, up to `long long`, and declines
when that storage contains the output. That catches the joined pairs the register entries
do not show: `edx:eax` on i386, which `x86gcc.cspec` declares as a join entry
for 5 to 8 bytes, and `r0:r1` on ARM, which is a model rule and not an entry at
all. On i386, `if (v1 != -1)` after `v1 = big(g)` against a `big` kuna prints as
`unsigned long long` and that returns `0xffffffff` in `eax` and 0 in `edx` would
otherwise fold into `big(g) != -1`, which returns the other branch. So on a
32-bit target an undeclared callee's output never folds under this option.
Pairs wider than `long long` (`rdx:rax`, `x0:x1`) are not asked about. They
carry `__int128` or a two-word struct, and counting them would mark every
64-bit output narrowed.

How far the call travels is the rest of this option's question, and it is asked
here rather than borrowed from `foldcallret`
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_foldcallretphi.rs
(print_point_is_order_safe)`): from the call up to the landing statement nothing
may be a barrier — any opcode in `foldcallret`'s first set, or any op writing
persistent or address-tied storage, the self-copy exemption not applied — and
nothing up to and including that statement may read a value the call writes
indirectly. Nor may the call's value reach a `BOOL_AND` or `BOOL_OR` as its
right-hand operand anywhere along the chain the expression travels through
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_foldcallretphi.rs
(chain_reaches_short_circuit_rhs)`). Both ops evaluate both operands, and every
op of the chain can sit in the call's own block, but C prints them as `&&` and
`||`, which skip the right-hand operand when the left one decides. gcc -O2
compiles `r = fflush(stdout); gflag = (r == 0) && (a > 5);` to one block
(`call fflush; test eax,eax; sete dl; cmp ebx,5; setg al; and eax,edx`), and
folding the call prints `gflag = 5 < a0 && !fflush(stdout)`: the binary always
calls `fflush`, the C only when `a0 > 5`. The left-hand operand is always
evaluated, and the printer emits input 0 on the left, so a call that reaches
input 0 (`(ferror(stdin) || 5 < a0)`) still folds. A structured `if (a && b)` built from two blocks is
not affected: there the call sits in the second block and the binary skips it
too. `foldcallret`'s own folds can land in such an operand as well; that is
tracked separately (GH-684) and not changed by this option. Without the
barrier half `betaflight`'s `sub_8051ac4` emits its
`sub_80515b4(dat_200181a4)` *after* an `if (dat_200019cc & 1)` that the binary
evaluates after the call — a call moved past two global reads it may itself
write; the discount is sometimes the only thing that was holding such a call in
place, which is why the guard has to reach the landing statement rather than the
use. Narrowing the INDIRECT half to the landing statement alone lets `ssh` O2
`sub_4fd30` print `sub_3fa80(v2,v3)` after `v5 = v2`, a copy of an escaped stack
slot the binary reloads at `0x4fe98` — GH-181's shape — and adds one such fold on
`ssh` O2 and one on `tar` O0.

This guard is deliberately wider than the two `foldcallret` runs, and stays as it
shipped: the folds it releases are ones the merge machinery was holding, so what
`--option foldcallretphi on` renders differently from `off` is decided exactly as
it was before GH-657. The only thing the option inherits from that fix is what
the fix changes in the default rendering — measured per function on `ssh` O2,
`dpkg` O2, `bash` O2, `tar` O0 and `grep` O0 in `docs/features/gh657/`.

It is on by default. The effect is a call spill removed and its expression
printed at the use: `v31 &= 0xffffffff; if (strcmp(v98,optarg))` in `sort` O2
`main`. Because marking a value
implied re-dirties its operands' covers, a neighbouring value occasionally fails
its own implied test and gains a statement of its own at the position its
defining op already had (the loads `find` O2 `sub_f400` passes to
`__snprintf_chk` become statements just ahead of it) — the conservative direction, and the reason the declaration count falls
by less than the number of folds. What the default buys and costs was measured
on the flip itself. Over `fmt`, `ls`, `sort` and `du` at O0 and O2 (2,918
functions) 30 functions change and the emitted C declares 14 fewer locals and 7
fewer single-def/single-use temporaries. Over those and 24 more stripped binaries
(11,279 functions) the `--json` `variables[]` surface decbench scores keeps the
same (kind, type, stack offset, size, argument index) multiset in every function,
with only `vN` names and line spans moving; every declaration this removes is a
register local, which `variables[]` never carried. Each hunk is a fold, one of
those operand hoists, a renumbering, or the declaration that went with them. A
folded call is never moved past a call, a load, a store through a pointer, a
global write or a branch, and never into an operand of `&&` or `||`. The most it crosses is an assignment to a local whose
address is never taken: a register (`v31 &= 0xffffffff`, `v1 = 0`), or a stack
slot nothing can reach, such as the phantom return-address slot `ssh-add` O2
`sub_9cb0` writes (`v17 = 0xa04c`).

Two costs remain. The first is positional: removing a declaration renumbers the
remaining `vN` locals, and `--assert type vN` / `--assert name vN` address a
variable by that auto-generated name, so a directive written against the pre-flip
output can name a different variable. The second is signedness at equal width:
the width condition keeps every narrowing of an integer up to `long long`, but a full-width unlocked output is
still typed by this function's own reads, so a callee printed `unsigned long`
whose caller spilled its result into a `long` loses that conversion when folded.
It only matters under a relational comparison, a shift or a division, and over
the 32 binaries no folded call of an undeclared callee feeds one; every such use
is a declared library call or already prints a cast (`(long)lseek(..) < 0`).
`--option foldcallretphi off` restores the spilled form byte-for-byte.
Provenance and the corpus sweep:
`docs/features/foldcallretphi/`.

**(kuna, Ghidra issue GH-8500) `option stackalias`** (default **off**,
destructive). The recorded gap: a store through a take-address-of-local
pointer could be dead-coded one heritage round before the aliasing LOAD
resolved, leaving a read of an uninitialized `xStack_*` local. The module
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_stackalias.rs
(StackAliasOption)`) owns only the gate (`Architecture::stack_alias_deadstore`);
the behavioral arm — holding all stack stores live for the round when a
pointer-to-stack LOAD is seen in `ActionDeadCode::lastChanceLoad` — is **not
in the live tree** (recorded at the gate's would-be consumption point in
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(deadcode_apply)`), because the general heritage deadcode-delay restart
(chapter [03](03-ssa-and-simplification.md), edge table in 00-overview §0.7)
was fixed to persist across re-flows and now subsumes the known repro — in
`tests/stages/gh8500-stackalias.xml` the default (off) pass recovers the
store, and the option-on pass is byte-identical. It stays a settable (and deliberately not default-on, DIV-3: as a
global default it would pin genuinely dead stores alive) so the surface and
its catalog row exist for the day a non-subsumed case appears.

**(kuna, Ghidra issue GH-9218) `option inputvarnodeadjust`** (default **on**,
DIV-3). When parameter recovery widens an unjustified input container
(`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionUnjustifiedParams)`), upstream only absorbs earlier inputs that
overlap the container's *low* end; an input overlapping the *high* end (mc68k
`link`/`unlk` overlapping stack params) survives to
`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs
(Funcdata::adjust_input_varnodes)`, which refuses with "Cannot properly
adjust input varnodes" (upstream that throw kills the whole function; kuna's
driver skips the adjustment and leaves the overlapping inputs unrecombined).
The gated forward scan walks the address-ordered inputs *upward*, widening
the container over any input whose tail extends past the container end, so
the adjustment always sees fully-contained inputs. `off` preserves the
upstream refusal.

**(kuna, decbench) `option paramcopyhoist` — the entry-block anchor for an
unmodified parameter's copy-shadow** (default **off**, opt-in). `gcc -O0` gives
every parameter a stack home, and the source idiom `if (p && !*p) p = NULL;`
leaves a MULTIEQUAL at the guard's join whose incoming slots are the raw input
register. `Merge::mergeOp` cannot merge an input-register HighVariable with the
address-tied stack-slot HighVariable, so `Merge::trimOpInput` snips each
offending slot into a COPY and places it at the *tail of that slot's predecessor
block* — and that COPY is what prints as `vN = aM;`. For the **first** parameter
the guard's join predecessors intersect at the entry block, so
`Merge::buildDominantCopy`'s `findCommonBlock` lands the copy there and it
renders with the other spills; for every later parameter the intersection is the
previous guard's join, so the copy sinks below that guard and splits the
source's single entry block in two. Upstream Ghidra sinks it identically, so the
hoist is a divergence and ships behind an option, not as a fix. Two producing
sub-cases both route here: two-or-more trimmed slots (later collapsed by
`buildDominantCopy`) and exactly one trimmed slot (no dominant-copy pass at all,
so the single `opInsertEnd` is final).

When on, `decompiler/crates/kuna-decomp/src/p6_variables/kuna_paramcopyhoist.rs
(ActionParamCopyHoist)` relocates the COPY to the end of the entry block. It is
the **last** action in `universal_sched`, and that position is load-bearing:
taking the decision inside `Merge::trimOpInput` defeats the trim's own purpose
(the widened Cover fails `mergeOp`'s test, `mergeOp` falls through to
`trimOpOutput`, and `markInternalCopies` then hides both resulting COPYs, so the
assignment disappears from the emitted C), and running the move any earlier than
`ActionFinalStructure` perturbs the P8 duplication/dedup passes. Running last,
the only observable effect is which basic block's statement list holds the COPY.

A candidate must be a printing COPY of a Varnode occupying a **formal
parameter's** storage (`Varnode::isInput` alone also admits globals and
read-before-written stack slots), at least one of whose reads is a MULTIEQUAL
and all of whose reads are MULTIEQUALs or INDIRECTs — the INDIRECT-only case is
`Merge::mergeIndirect`'s call-adjacent snip, a different Cover shape left alone.
Legality is `buildDominantCopy`'s own Cover test re-run against the hypothetical
hoisted placement and the **final** HighVariable: `b_cover` over the high's other
instances (skipping copy-shadows of the same root) versus an `a_cover` whose def
point is taken at the *start* of the entry block, rejecting on
`Cover::intersect > 1`; the def-point choice over-approximates, so the test errs
toward rejecting. Finally, a high with more than one candidate is skipped
entirely — the Cover test compares each move against where the *other*
definitions sit today, so two definitions of one variable can both be admitted
even though, once both have moved, the second kills the first on every path.

**(kuna, GH-468) `option tiedphitrim` — a loop that reads memory does not
store what it reads** (default **on**, DIV-182). A HighVariable that holds an
address-tied instance *is* that location, so every other instance's definition
prints as an assignment to it. `merge.rs (Merge::merge_op)` forces a
MULTIEQUAL's output and inputs into one HighVariable and trims an input only when
the required tests or the Covers forbid the merge. A loop that is entered with a
direct read of an aliased slot and then walks memory has no such conflict: the
entry read is dead once the loop starts, so upstream folds the loop variable into
the slot and every value the loop loads prints as a store into it. MSVC `/O2`
guards `for (i = 0; buf[i]; i++)` by reading `buf[0]` from the frame and latches
through `[RSP+RAX+0x40]`, which prints `while (v10[0]) { ...; v10[0] = v10[v6]; }`;
gcc and clang `-O2` carry the byte in a register and print `v2 = *v1;` into the
buffer's first byte; a walk over a global list prints the global head being
advanced. Upstream Ghidra 12.1 prints the same stores. When the buffer is read
after the loop the C is wrong, not only misleading: an in-place lowercase loop
followed by `strcmp(buf, name)` compares an empty string in the C, because every
iteration stored into `buf[0]`.

`decompiler/crates/kuna-decomp/src/p6_variables/kuna_tiedphitrim.rs
(tied_slots)` runs inside `merge_op` after upstream's own trims. It fires on a
MULTIEQUAL whose output is not address-tied when every address-tied input reads
one location a pointer can observe and arrives from outside the loop, and some
input arrives on a back edge and is read by a LOAD. A pointer can observe a
global, or a frame local below the parameter area that is not proven unaliased
(`Varnode::has_no_local_alias`); the function's entry value of either is left
alone. An edge is a back edge when the MULTIEQUAL's block dominates its
predecessor, and the LOAD may be reached through COPYs and MULTIEQUALs. Each
address-tied input then gets upstream's trim COPY on its entering edge, so the
loop reads the location once into its own variable. At an if/else join, and in a
loop whose value is computed rather than loaded, the merged variable is usually
the source's own local (`if (err) result = 0; return result;` after
`get(&result)`), so those keep upstream's merge.

The trim takes the loop-head read out of the location's HighVariable, and that
read is what kept everything live in the loop out of the location. Two rules keep
that exclusion. First, the trim declines when the location is written inside the
loop, or written anywhere with a value that comes from inside it (a backward walk
over the written value's definitions that stops at address-tied values). Without
this, `c = buf[0]; while (c) { x = c + acc + 1; ...; c = buf[n - 1]; } buf[0] = x;`
would let `x` merge into `buf[0]`, printing `buf[0] = x` inside the loop, where the
loop's first `buf[n - 1]` reads it back. The loop blocks are those the head
dominates and that reach the head again. Second, `Merge` records each trimmed loop
variable with its location, and `merge.rs (Merge::merge)` and
`merge.rs (Merge::merge_test)` test every merge into that location as if the
location's variable still held the loop variable
(`kuna_tiedphitrim.rs (intersects)`). No address-tied variable merges with the
loop variable, at the trimmed location or any other: upstream's required test
never merges two address-tied variables, and the loop variable used to be one.
Without that, `buf[1] = c;` after a loop over `buf` would fold the loop variable
into `buf[1]` through `merge_opcode`, printing the store before the loop, where
the loop's first reload of `buf[1]` reads it back. Nothing whose Cover intersects
the loop variable merges into its own location either. Every other merge is the
one upstream makes, so the only change from upstream is that the loop variable's
own definitions print as that variable instead of as stores.

The trim never removes an operation and refusing a merge never removes one either,
so a live store into the location keeps its statement. `off` is upstream's merge
exactly.

**(kuna) `option dynamichashmax`** — §6.3.

## 6.5 Cleanup

**What runs after the Band-B fixpoint.** When `fullloop` reports quiescence,
the one-shot tail runs in tree order (`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`): the final symbol sync (`ActionMappedLocalSync`, §6.2),
then `decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionStartCleanUp)` — which snapshots the Varnode creation index
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::start_clean_up)`) so later passes can tell cleanup-born values from
analyzed ones — then the 22-rule `cleanup` pool (a repeat-pool: it still runs
to its own local fixpoint, just outside the analysis loop), then structuring
normalization, then the merge phalanx of §6.1, prototype fixation, naming,
casts, and final structure.

**Why the cleanup pool must not run inside the fixpoint.** Its rules trade
analyzability for readability, and several are literal inverses of `oppool1`
normalizations: the main pool canonicalizes `V - W` into `V + W*-1`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_3.rs
(RuleSub2Add)`) so one ADD-shaped matcher set covers all additive flow, and
the cleanup pool converts it back (`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs
(Rule2Comp2Sub)`, plus `RuleMultNegOne` for the `*-1` residue). Inside one
pool the pair would oscillate and the repeat-group would never quiesce;
outside, each direction fires exactly once per function. The same one-way
logic covers the rest of the pool (registered in `universal_sched`): the
`splitcopy`/`splitpointer` and `bitfields` rules lower whole-value COPYs and
LOAD/STOREs into field accesses the analysis would otherwise have to reason
through, and the `constsequence` string/memset builders collapse instruction
sequences the rule pools deliberately kept elementwise.

**Why the merge phalanx must not run inside the fixpoint.** Every phalanx
action is `rule_onceperfunc` — it commits, it does not iterate. Three facts
force the placement: (1) HighVariables do not exist until `ActionAssignHigh`
turns them on, and downstream surfaces hard-refuse earlier
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::build_dynamic_symbol)` errors "Cannot create dynamic symbols until
decompile has completed"); (2) a Cover is only meaningful over a stable CFG
and stable SSA — any further heritage pass or block edit would silently
invalidate every merge the intersection cache blessed; (3) the trim COPYs the
forced merges insert (§6.1) are exactly the kind of copy `RulePropagateCopy`
exists to delete — inside the loop the pool would collapse them and re-fuse
what the merge just separated (the live tree documents precisely this
interaction at `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionMergeRequired)`, which has to re-bind dynamic symbols because the
fullloop already collapsed the upstream-preserved carrier COPY once).

**The exception that proves the rule.** One resident of this folder runs
*inside* the fixpoint: `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_stackptr.rs`
(the realized body of `ActionStackPtrFlow`, scheduled in `stackstall`). It is
analysis, not commitment — the stack-variable promotion of §6.2 only works
while the stack pointer remains a tractable affine expression, so this pass
repairs the two flows that break that invariant as soon as they appear:
a *clog* (`coreaction_stackptr.rs (check_clog)`: `SP = SP + *(SP+k)` — an
`alloca`-style adjustment loaded from the stack; when the matching constant
STORE is found the LOAD becomes a COPY), and unresolved stack adjustments
across calls (`coreaction_stackptr.rs (StackSolver)`: a linear system over
every spacebase reference — one variable per reference, sentinel 65535 for
"unsolved" — whose solution rewrites the INDIRECT survivors into explicit
`INT_ADD SP, #c`; an underdetermined call contributes the guessed equation
`extrapop = 4`). Once the solve succeeds the action latches
(`analysis_finished`) until the next restart resets it. Its failure mode is
honest: an unsolvable system leaves the INDIRECTs in place, the affected
frame offsets never promote, and the function keeps raw pointer arithmetic
where locals should be.

The guessed equation is where kuna diverges (option `calleepop`, default on).
Upstream's constant `extrapop = 4` states that the callee pops none of its
arguments, and under a compiler spec whose `<default_proto>` carries
`extrapop="unknown"` — `x86win.cspec`'s `__stdcall` is the one that matters —
that is wrong for most calls, cumulatively: the solve latches its answer into
`INT_ADD sp, #c` at every reference, so each unpopped argument run displaces
every stack slot after it. A slot whose address is taken before a call and read
after it then splits into two variables, the read half acquires no reaching
definition, and the emitted C computes over a constant where the callee's bytes
should be. With the option on the guess is raised for the one family whose
convention the platform fixes — an **imported** callee, which is `__stdcall` by
the Win32 ABI and which kuna can already identify because `peimportcall` paints
`Varnode::externref` over the Import Address Table. An internal callee keeps
upstream's guess. For an import the argument bytes are counted off the caller's
own push run
(`decompiler/crates/kuna-decomp/src/p6_variables/kuna_calleepop.rs
(guess_extra_pop)`): back from the return-address slot in pointer-sized steps,
stopping at the first push that stores a register's own input Varnode (a
prologue callee-save) or at a step that is no push at all (a bare `sub esp,n`).
A caller that raises the stack pointer past that whole run afterwards is
performing a `__cdecl` cleanup, and the guess falls back to `4`; the height is
measured as the topmost pushed slot reachable from the call rather than as an
`add esp,#k` op, because the normalization re-bases a following push run onto
the call's own result and leaves the cleanup Varnode dead. The reading stays a
*guess* — it is consulted only where the exact equations leave the variable
free — and a more faithful frame can expose weaknesses further down: an
outgoing-argument slot that lands inside the caller's `localrange` once the
frame is the right size is scored no-use by `checkInputTrialUse` (§4.4) and the
argument is dropped, which is visible on deep-frame MSVC CRT helpers.
