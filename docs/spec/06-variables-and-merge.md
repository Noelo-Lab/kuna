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
   to force merge of op"). *(kuna)* Where upstream lets that error abort the
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
more than `max_implied_ref` = 2 descendants
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`; a PTRSUB off the spacebase lifts the cap to
1000000 so stack addresses always fold). A multi-reader implied candidate is
then re-examined for *term duplication*
(`coreaction_cleanup.rs (process_multiplier)`, `(multiple_interaction)`):
inlining it would print its whole expression at every use, so if the
duplicated terminal count exceeds `max_term_duplication` = 2 (same anchor), or
two marked candidates interact through a bool/extension/PTRADD op, it is
forced explicit. `coreaction_cleanup.rs (ActionMarkImplied)` then walks each
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
policy of chapter [09](09-emission.md). Three scheduled bodies are documented
inert stubs in the live tree — `coreaction_cleanup.rs (ActionMarkIndirectOnly)`
and `(ActionHideShadow)` apply no change, and `(ActionMergeMultiEntry)` is
wired to the real engine (`merge.rs (Merge::merge_multi_entry)`) but its
multi-entry-symbol source (`funcdata_merge.rs
(MergeContext::multi_entry_symbols)`) returns empty pending the symbol-scope
layer.

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
explicit: false negatives over reordering bugs. When the predicate passes, the output falls
through to the ordinary implied machinery of §6.1 — the fold itself is just
`if (timespec_cmp(...) <= -1)` emerging from the printer's normal recursion.
`off` restores the upstream always-spill form byte-for-byte; four datatest
files pin that form via per-test opt-outs (DIV-14, `docs/history.md`).
Provenance: `docs/features/call-return-variable-folding-dcde82/record.json`
(ablation: 5 upstream assertions change; measured speed delta −3.2%).

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
