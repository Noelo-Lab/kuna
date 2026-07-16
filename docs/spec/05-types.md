# 05 — Types

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p5_types
  - decompiler/crates/kuna-decomp/src/substrate/dtype.rs
```

Phase 5 computes the **fact fabric** over the SSA graph: a data-type on every
live Varnode, plus the value facts the other phases consume — circular value
ranges, non-zero masks, and consume bits. None of it runs as a standalone
stage. The type-inference and constant-pointer actions sit *inside* `mainloop`
(00-overview §0.6), between heritage and the structuring tail, and iterate to
mutual quiescence with SSA simplification (chapter 03), prototype recovery
(chapter 04), and the variable model (chapter 06) — the Band-B fixpoint. The
exact slot order is `decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`: dead-code (consume bits) → non-zero masks → type inference →
the `stackstall` rule pools → block structure → constant-pointer recovery → the
`oppool2` pointer-arithmetic pool.

Option defaults, tiers, and flip guidance for every option named below live in
the generated catalog ([docs/options.md](../options.md)); the rows are defined
in `decompiler/crates/kuna-decomp/phases.toml` and the type-phase default
divergences are DIV-2 in `docs/divergences.md`. Untagged prose is the
Ghidra-derived port; `(kuna)` marks kuna-original passes (each named with the
upstream GH issue that inspired it, per its `phases.toml` row).

## 5.1 Type representation

The type *data model* lives in the substrate —
`decompiler/crates/kuna-decomp/src/substrate/dtype.rs` — not under `p5_types`,
because it is shared IR: every Varnode carries a `Datatype` from the moment it
is created, and every phase from lift to emission reads it. What lives in
`p5_types` is the *inference* — the passes that decide which type a Varnode
carries.

**The metatype lattice.** Every type reduces to one of 18 meta-types
(`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (type_metatype)`),
transcribed with explicit discriminants because **the numeric order is the
specificity order**: lower is more specific, from `TYPE_PARTIALUNION` (0)
through struct/enum/array/pointer/float down to `TYPE_UNKNOWN` (15),
`TYPE_SPACEBASE` (16) and `TYPE_VOID` (17). Propagation and cast decisions
never compare metatypes directly; they go through the 24-value refinement
`sub_metatype` (same file), which splits e.g. `TYPE_PTR` into
plain/relative/into-struct pointer ranks and `TYPE_INT` into
char/unicode/enum/plain ranks. `Datatype::type_order` — the single gate the
inference engine uses (§5.2) — resolves to `Datatype::compare` with a recursion
budget of 10 levels; past the budget, identity falls back to the interned type
id, so comparison of deep recursive structures terminates. A separate
`type_order_formal` de-prioritizes partial unions and `bool` when *choosing a
declared type* (a value that merely behaved boolean should not out-compete a
real integer type). The third enum, `type_class`, is not a lattice at all: it
classifies types for parameter-storage assignment (general/float/pointer/
hidden-return/vector, plus four architecture-specific classes) and belongs to chapter 04's prototype models.

**One struct, no inheritance.** The C++ `Datatype` hierarchy
(`TypePointer`/`TypeArray`/`TypeStruct`/…) becomes a single `Datatype` struct
carrying the shared members (id, size, flags, name, metatype, submeta,
alignment) plus a `DatatypeKind` payload enum
(`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (DatatypeKind)`) with one
variant per C++ subclass; methods match on the kind to reproduce virtual
dispatch. The variants worth knowing:

- `Struct` carries ordered `TypeField`s plus a separate `TypeBitField` list for
  sub-byte fields (§5.6); `Union` carries fields that all start at offset 0 and
  is never accessed directly — every read/write goes through resolution (§5.4).
- `Enum` carries the value→name map; rendering a constant as an OR of enum
  names is an emission concern (`EnumRepresentation`, same file).
- The `Partial*` variants (`PartialStruct`/`PartialUnion`/`PartialEnum`) stand
  for a byte-slice of a container — the type a Varnode gets when it holds only
  part of a struct/union/enum — each carrying the container, the byte offset,
  and a `stripped` plain type to fall back on when a formal type is required.
- `PointerRel` is a pointer *into the middle* of a container (parent + offset),
  ranked as a distinct pointer sub-metatype so a mid-struct pointer never
  unifies silently with a plain pointer to the same field type.
- `Spacebase` treats an entire address space as one struct whose "fields" are
  the symbols mapped in it. This is the pivot type of both constant-pointer
  recovery (§5.2) and stack-frame typing: a `TYPE_PTR` to `TYPE_SPACEBASE` is
  "pointer into the frame/globals", and member lookup on it is symbol-table
  lookup.
- `Code` optionally carries a full `FuncProto`, so a function-pointer call
  through it can type its arguments.

**The factory.** All types are interned:
`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (TypeFactoryImpl)` is the
per-architecture container behind the `TypeFactory` trait handle. Every
constructor (`get_base`, `get_type_pointer`, `get_type_array`, …) builds a
candidate and de-duplicates it through a `BTreeSet` ordered by
`compare_dependency`-then-id (`TreeKey` in the same file) — the C++
`DatatypeCompare` semantics, which is why structurally identical types are
pointer-equal and `Rc::ptr_eq` is a valid fast-path everywhere. A 9×8 cache
matrix holds the atomic types (sizes 0–8 × the eight meta-types from
`TYPE_FLOAT` up), with special slots for 10- and 16-byte floats and the char
types. Two policy knobs are read from the compiler spec's data-organization:
the primitive sizes/alignments, and `max_basetype_size` — a `get_base` request
larger than it does not invent a giant integer, it returns an **array of
unknown bytes** of the right size, which is the honest statement of what is
known. `get_exact_piece` answers "what type is the size-N slice at offset K of
this type" (the symbol-piece seed of §5.2); `concretize` maps residual
`TYPE_UNKNOWN` onto concrete integer types where a formal type is forced.

The interning tree's order is a dependency *comparison*, not a dependency
*ordering*: `compare_dependency` ranks by sub-metatype then **descending**
size, so a struct contained by value sorts *after* the struct that contains
it — walking the tree front to back is not definition-before-use. The
explicit fix is `decompiler/crates/kuna-decomp/src/substrate/dtype.rs
(TypeFactoryImpl::dependent_order)`, the C++
`TypeFactory::dependentOrder`/`orderRecurse` port: a postorder DFS over each
type's typedef base (`get_typedef`) then its component sub-types
(`get_depend`), marked on `Rc` identity so a pointer cycle
(`struct A { struct B *b; }` / `struct B { struct A *a; }`) terminates with
each type listed exactly once. Its consumer is emission — chapter
[09](09-emission.md) §9.7's `doc_type_definitions` walks the list front to
back — and the two inline unit tests (`dependent_order_nested_struct`,
`dependent_order_pointer_cycle`) pin both facts: the raw tree order really is
container-first, and the DFS reorders it.

## 5.2 Inference

Type recovery is **off** for the entire first `fullloop` iteration — early
simplification should not chase types that heritage and prototype recovery are
about to invalidate. The arming switch is
`decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionStartTypes)` at the tail of `fullloop`: it flips the function's
type-recovery flag and reports a change, forcing at least one more `fullloop`
pass with inference live.

**One pass of the engine.**
`decompiler/crates/kuna-decomp/src/p5_types/coreaction_infertypes.rs
(run_infer_types)` executes the bounded bidirectional lattice in four steps:

1. **Seed** (`build_localtypes`): every live Varnode gets a *temporary* type
   from purely local evidence. A type-locked Varnode is its own seed (and a
   hard wall — nothing propagates over it). A Varnode covered by a type-locked
   symbol gets the exact byte-slice of the symbol's type
   (`decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs
   (build_localtype_seed)` → the factory's `get_exact_piece`). Everything else
   asks its defining op and each reading op for a suggestion
   (`output_type_local` / `input_type_local`, same module) and keeps the most
   specific by `type_order`. The suggestions come from the per-opcode table
   `decompiler/crates/kuna-decomp/src/p5_types/typeop.rs (type_op_info)` — the
   port of the C++ `TypeOp` `inst[]` registry, which bundles each opcode's
   p-code property flags, display attributes, and local input/output
   meta-types — with two live upgrades: a CALL/CALLIND whose resolved callee
   prototype is committed suggests the callee's real return/parameter types,
   which is how a typed argument reaches the caller's stack Varnode.
2. **Propagate** (`propagate_one_type`): from each seeded Varnode, a DFS walks
   the def-use graph in both directions, at each edge asking the op's transfer
   function for the outgoing type (`propagate_type`). A pushed type is adopted
   only if it is **strictly more specific** than the target's current temporary
   (`0 > type_order`), and each Varnode is expanded at most once per walk (a
   mark bit), so a single pass is linear-ish and cannot ping-pong.
3. **Returns** (`propagate_across_returns`): unless the output prototype is
   locked, the most specific temporary among all `RETURN` value inputs is
   re-seeded onto the other returns, so one well-typed exit types them all.
4. **Write-back** (`write_back`): temporaries become permanent Varnode types;
   every change dirties the owning HighVariable (chapter 06 recomputes lazily)
   and marks the pass "changed".

The transfer functions are the heart. COPY/MULTIEQUAL/INDIRECT propagate
identity (input↔output only); the unsigned comparisons propagate input↔input;
the *signed* comparisons propagate only `TYPE_INT` ("only propagate signed
things" — a pointer compared signed must not become signed); `INT_ADD`
propagates a pointer across the add while accounting for the constant offset;
LOAD/STORE convert between pointer and pointee (`propagate_to_pointer` /
`propagate_from_pointer`); PTRADD/PTRSUB walk the pointed-to composite through
`TypePointer::downChain` (`propagate_add_in2_out`) so array/member arithmetic
yields field pointers; XOR/AND propagate only enums and float
sign-manipulation idioms, OR only enums; PIECE/SUBPIECE map between a composite
and its byte-slices (producing `Partial*` types); SEGMENT resizes pointers;
NEW takes its type from the constant pool. Everything else refuses —
propagating through an opcode with no sound transfer is how type garbage
spreads. A `TYPE_BOOL` additionally refuses to land on any Varnode whose
non-zero mask (§5.3) admits values above 1. When the incoming type is a union
(or pointer-to-union), the edge first resolves a concrete facet via
`decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs
(Funcdata::resolve_in_flow)` (§5.4) — except across MULTIEQUAL/INDIRECT
markers, where the unresolved union flows on so one phi input cannot lock the
facet for all of them.

The S5→S6 feedback lives at the end of the pass: `propagate_spacebase_ref`
finds the stack-pointer input and pushes recovered pointer types (e.g. a
`mystruct *` argument) *through* spacebase arithmetic onto the addressed
stack-frame Varnodes (`propagate_ref`), which is what turns "pointer typed as
`mystruct *`" into "stack local declared `mystruct`". The pass opens by
committing any pending stack-symbol type recommendations
(`decompiler/crates/kuna-decomp/src/p6_variables/funcdata_spacebase.rs
(apply_type_recommendations)`).

**The bounded pass count.** The wrapper
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionInferTypes)` runs the engine once per `mainloop` iteration and counts
only pass runs where `write_back` reported a change. The counter is capped at
**7 passes** per (re)start: on the 7th it stops, records
`set_type_recovery_exceeded`, and the upstream "Type propagation algorithm not
settling" condition holds. The ceiling exists because the lattice itself is
monotone but the *environment* is not — rule-pool rewrites between passes can
keep presenting new ops, and a pathological int↔pointer disagreement can
alternate forever. Failure mode when the ceiling hits: types freeze at the
last settled state, and downstream pointer-arithmetic rewrites self-type their
new ops directly — `decompiler/crates/kuna-decomp/src/substrate/addtreestate.rs
(assign_propagated_type)` fires whenever `is_type_recovery_exceeded` holds —
since no further inference pass will visit them. The budget is compile-time and
deliberately latent (the `solver-budget` row in
`decompiler/crates/kuna-decomp/phases.toml`, strength HINT).

**The casting boundary.** Inference annotates; it never converts. Where the
final Varnode type disagrees with what an op requires, nothing in phase 5
reconciles it — the disagreement survives to
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionSetCasts)` in the one-shot tail, which renders an explicit cast
(chapter 09). So the symptom of a lost propagation is a spurious `(int *)`
cast in the output, never wrong data-flow.

**Constant pointers.** The other typerecovery action in `mainloop`,
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionConstantPtr)`, turns bare constants into symbol references. Decision
rule for the simple case: a constant with a single reader, already typed as a
pointer (or used where only a pointer makes sense), whose value resolves to a
mapped global symbol, is rewritten to reference that symbol via the space's
spacebase (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(spacebase_constant)`) — and the symbol's type then seeds the next inference
pass. The machinery around it is all about *not* firing: the action is gated
on type recovery having started and capped at 4 passes per function; the space
is chosen by `select_infer_space` (an explicit pointer type's space attribute
wins; otherwise the architecture's inferable spaces, tie-broken by scanning
forward for a LOAD/STORE that names the space); usage arms (call arguments,
comparisons, PIECE, COPY, plain INT_ADD) are additionally gated by the P0
`inferconstptr` option; the value must lie inside the space's pointer bounds;
and a constant whose bit pattern has fewer than 3 bit-transitions is rejected
as a probable flag/mask (`p9_emit/coreaction_render.rs (is_pointer)` — the ActionConstantPtr file), because turning `0x100000`
into a pointer corrupts every function that uses it as a bit. On acceptance
the symbol lookup requires an exact hit unless the target is a character
array (string constants may point mid-string).

Two (kuna) escapes hook exactly here, both shipped default-on (DIV-2,
`decompiler/crates/kuna-decomp/phases.toml`):

- **(kuna GH-6930)** [`inferfuncentry`](../options.md): the
  bit-transitions rejection is skipped when the constant resolves *exactly* to
  a known function entry
  (`decompiler/crates/kuna-decomp/src/p5_types/kuna_inferfuncentry.rs
  (kuna_is_function_entry)`, wired in `coreaction_render.rs
  (kuna_const_is_function_entry)`) — a function placed at a power-of-two image
  base is a single-bit value, but it is still a function. Ordinary data
  constants never match, so flag semantics are preserved.
- **(kuna GH-8471)** [`thumbfuncptr`](../options.md): a Thumb function pointer
  is `fn|1`; constant-pointer recovery produces `PTRSUB(fn) + 1`, and the
  simplification rule that normally deletes out-of-bounds PTRSUBs would
  collapse it back to a hex literal. The guard
  `decompiler/crates/kuna-decomp/src/p5_types/kuna_thumbfuncptr.rs
  (kuna_preserve_thumb_funcptr)` — consulted by
  `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs
  (RulePtrsubUndo)` — keeps the PTRSUB when the base resolves through the
  spacebase to a `TYPE_CODE` symbol and the leftover offset fits inside the
  architecture's `funcptr_align` mode bits. Inert on architectures with no
  alignment encoding (`funcptr_align == 0`).

Type facts are *consumed* back into the graph by the typerecovery rules: the
`oppool2` pool (`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_5.rs
(RulePushPtr, RuleStructOffset0, RulePtrArith)`) materializes PTRADD/PTRSUB
member access from pointer types, and `ruleaction_6.rs (RulePtraddUndo,
RulePtrsubUndo)` in the main pool reverts them when the types they were built
from degrade — the visible ping that the 7-pass ceiling exists to bound.

## 5.3 Ranges & consume bits

The rest of the S5 fact fabric (the framing derives from the study in
`docs/history/stage-model.md` §7.3; every claim below is re-verified against
the Rust).

**Circular ranges.**
`decompiler/crates/kuna-decomp/src/p5_types/rangeutil.rs (CircleRange)` is the
value domain: a half-open interval `[left, right)` on the circle of integers
mod 2^n, with an optional power-of-two step. The circle (rather than a plain
interval) is what lets one object exactly represent "signed negative", a
wrapped `x-c < k` guard, or a strided jump-table index. The per-opcode
`pull_back_*` operators invert an op's effect on a range (given the output
range, what was the input range), and `push_forward_*` run it forward. The
live consumers are (a) jump-table recovery — the guard analysis in
`decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs (GuardRecord,
circlerange_pull_back)` pulls the branch condition back to the switch variable
to bound the case count (chapter 02), and (b) the boolean-expression melding
rule `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs
(RuleRangeMeld)`, which pulls two comparison ranges back to a common Varnode
and intersects/unions them into one comparison.

**The value-set solver.** On top of `CircleRange` sits an abstract
interpretation layer (`ValueSet`, `ValueSetSolver`, same file): a
Bourdoncle-style weak topological ordering over the SSA constraint graph, with
widening to force loop convergence. The shipped strategies are read from the
code: `WidenerFull` widens at iteration **2** (snapping the unstable bound to a
"landmark" — typically the loop-guard constant — or its complement) and gives
up to full range at iteration **5**; `WidenerNone` freezes whatever has been
reached by iteration **3**. The reason for the two-stage schedule: one cheap
guess (the landmark) is usually exactly the loop bound, and if it is not, more
iteration is wasted work. Honest port status: the ordering, widening, and
range arithmetic are fully ported and self-tested, but the binding of value
sets to live Varnodes (constraint generation, and the LoadGuard range
refinement at `decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs
(LoadGuard)`) is a documented stub — guarded LOAD/STORE ranges stay at their
conservative defaults, so the failure mode is over-wide alias guards (extra
heritage conservatism), never a wrong range.

**Non-zero masks.** `decompiler/crates/kuna-decomp/src/substrate/
funcdata_varnode.rs (Funcdata::calc_nz_mask)`, driven once per `mainloop` pass
by `decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionNonzeroMask)`, computes for every Varnode the mask of bits that can
possibly be non-zero: a forward DFS in post-order over the def-use graph, then
a fixpoint re-pass across the MULTIEQUAL loop edges the DFS clipped. The mask
feeds the boolean gate in §5.2, sub-variable flow (chapter 03), and dozens of
simplification rules.

**Consume bits.** The dual analysis, backwards:
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionDeadCode)` — group `deadcode`, classified P5 in
`decompiler/crates/kuna-decomp/phases.toml` because its artifact is a bit-level
fact even though its effect is deletion. Starting from ops whose effects are
observable (stores, calls, returns, branches), it pushes a per-Varnode
*consumed-bits* mask backwards through each defining op's transfer
(`dc_push_consumed` worklist), so e.g. a SUBPIECE consumes only the bytes it
extracts. Anything whose consume mask ends up empty is dead and is deleted —
this is the pass that destroys the lift-time ops phase 2 created. Its
pathological-case machinery lives in chapter 03's heritage: deletion is
suppressed for an address space still inside its *dead-code delay* window, and
a free Varnode reappearing at an already-heritaged address bumps the delay and
restarts (00-overview §0.7). Failure mode: consuming too little deletes a
computation the binary needed (the classic symptom is a wrong parameter list
feeding chapter 04's trial pruning); consuming too much merely leaves clutter
for later pools.

## 5.4 Union resolution

A union-typed value has no single correct field — each *access* has a correct
field. The artifact is therefore per-edge: `ResolvedUnion` records the winning
facet, keyed by `ResolveEdge` (type id + op/address encoding + op sequence
number, with the C++ map's exact total order —
`decompiler/crates/kuna-decomp/src/p5_types/unionresolve.rs (ResolveEdge)`),
cached per function in the `union_map`
(`decompiler/crates/kuna-decomp/src/p5_types/funcdata_union.rs
(Funcdata::get_union_field, Funcdata::set_union_field)`).

Resolution triggers wherever a `needs_resolution` type crosses an edge: during
propagation (§5.2), during cast planning, and in the printer's facing-type
lookups. The dispatch is
`decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs
(Funcdata::resolve_in_flow)`; its decision rule for the simple case, in order:
(1) the per-edge cache; (2) an operator-asserted facet — a `map unionfacet`
DynamicHash symbol, consulted through
`funcdata_union.rs (Funcdata::get_address_based_union_field)`, which is the P0
override surface for this whole section (the `aggregate-union` row in
`decompiler/crates/kuna-decomp/phases.toml`); (3) the scoring engine.

**Facet trials and scoring.**
`decompiler/crates/kuna-decomp/src/p5_types/unionresolve_run.rs
(ScoreUnionFields)` fits every candidate field (plus "the union as a whole",
score index 0 / field −1) against the surrounding data-flow. From the access
edge it launches one *trial* per candidate and walks outward level by level —
down through readers, up through definitions — scoring each op it meets with
per-opcode metatype fit tables
(`decompiler/crates/kuna-decomp/src/p5_types/unionresolve.rs
(score_trial_down_pure, score_trial_up_pure)`): a pointer flowing into a LOAD
scores well, an enum flowing into FLOAT_ADD scores badly. Trials stop and
score terminally at type locks (`score_locked_type`), locked
parameters/returns (`score_parameter`, `score_return_type`), truncations
(`score_truncation`), and constants (`score_constant_fit` — including a
"looks like a pointer" bounds test). The budgets are read from the code
(`unionresolve.rs`): at most **6** levels (`MAX_PASSES`), no new level once
**256** trials have run (`THRESHOLD`), hard stop at **1024** trials
(`MAX_TRIALS`) — a union inside a big expression web must not turn one
resolution into a whole-function analysis. The winner is the highest total
(`compute_best_index`; strict `>`, ties keep the earliest field, whole-union
first). Double-counting is prevented by a visited set keyed on the (Varnode, candidate-field score index) pair (`unionresolve_run.rs (VisitMark)`), so one Varnode is scored at most once per candidate facet.

Failure mode: scoring is heuristic, and a wrong facet renders the wrong member
access on every edge that shares the cached resolution; because the cache is
per-edge, one early bad pick does not poison other accesses, and the
`map unionfacet` assertion is the surgical override. The scoring weights
themselves are deliberately latent (no option) — matching upstream, where they
are compile-time.

## 5.5 Double precision

Compilers split a 2N-byte value into two N-byte registers; the IR then shows
every operation twice (lo half, hi half with carry), glued by SUBPIECE/PIECE.
`decompiler/crates/kuna-decomp/src/p5_types/double.rs` recovers the logical
whole. All four driver rules ride the main `oppool1` / `doubleload` groups in
the schedule (`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`).

**Marking.** `double.rs (RuleDoubleIn)` fires on SUBPIECE. Decision rule
(`attempt_marking`): the SUBPIECE must truncate *exactly the top half* of a
whole that is credibly one logical value — a type-locked primitive input, or
the output of an arithmetic/floating-point op — and a companion SUBPIECE of
the bottom half must exist. Both pieces get precision marks (`precis_lo` /
`precis_hi`). The producing-op restriction exists because for logical ops
there is no way to tell whether the whole was ever "one value"; marking a
coincidental pair would fuse unrelated variables.

**Pairing and rewrite.** Once marked, `double.rs (SplitVarnode)` describes the
hi/lo pair (plus the whole, when one exists — `whole_list` re-finds it from
the SUBPIECE fan-out). Each application pushes the logical operation **one
level** through the graph: `double.rs (apply_rule_in)` walks the pair's
readers and dispatches to the per-opcode `*Form` matcher families — `AddForm`/
`SubForm` for the add/carry and sub/borrow cascades, `LogicalForm` for
paired AND/OR/XOR, `Equal1Form`/`Equal2Form`/`Equal3Form` for the three
equality shapes, `LessThreeWay`/`LessConstForm` for the compare cascades,
`ShiftForm`, `MultForm` (the three-multiply 2N×2N pattern), `PhiForm` for
paired MULTIEQUALs, `IndirectForm`, and `CopyForceForm`. Every form runs a
full `verify` of the exact two-halves pattern — same operand order, same
tie-breakers as upstream — **before** mutating anything, then replaces the
half-ops with the single whole-width op; the halves die by consume-bit
analysis (§5.3). One level per rule firing means the repeat-applied pool
unzips an arbitrarily long cascade pass by pass.

`double.rs (RuleDoubleOut)` runs the same forms anchored at a PIECE (the value
is *built* from halves rather than split into them), merging two persistent
input halves into one logical input via `combine_input_varnodes`.
`RuleDoubleLoad` / `RuleDoubleStore` (same file) fuse two adjacent half-width
LOADs/STOREs into one whole-width access — requiring address contiguity in
the right endian order and proving no interfering write between the two ops.

**When it wins/loses.** It wins when the compiler's lowering kept the standard
shapes: the output shows one 2N-bit variable with ordinary arithmetic. It
declines — silently and safely — when the marking guards fail, when a
consumer shape matches no form, when the function still has unreachable
blocks (`RuleDoubleIn` waits, since dead code fakes patterns), or when the
whole would exceed 8 bytes as a constant (the `uintb` precision bound, `double.rs
(SIZEOF_UINTB)`). The failure rendering is not wrong code but *unfused* code:
`CONCAT`/`SUB` pseudo-ops and doubled arithmetic in the output. Because every
form verifies before rewriting, a wrong fuse is designed out rather than
detected after.

## 5.6 kuna extensions & the late rewrite families

The remaining `p5_types` passes are the late aggregate rewrites: they run in
the 22-rule `cleanup` pool *after* `fullloop` exits (00-overview §0.6), when
types and symbols are final enough to justify rewriting memory idioms.
Defaults below are stated from `decompiler/crates/kuna-decomp/phases.toml`.

**Constant sequences** (`decompiler/crates/kuna-decomp/src/p5_types/constseq.rs`).
Pattern: code writes a string one character at a time — a run of constant
COPYs into a stack/global char array, or constant STOREs through a heap
pointer. The shared base `constseq.rs (ArraySequence)` owns the discipline:
gather the sibling writes in the same block, keep the maximal window with no
interfering LOAD/STORE/CALL between members (`check_interference`), and
assemble the constants into one byte array by offset with endian-correct
unpacking, a single NUL allowed, contiguity required, and at least **4**
elements (`ArraySequence::MINIMUM_SEQUENCE_LENGTH`; upper bound 0x20000). The
two drivers are `constseq.rs (RuleStringCopy)` — COPY-into-array, requiring
the destination be an address-tied char array backed by a symbol container —
and `constseq.rs (RuleStringStore)` — STORE-through-pointer
(`HeapSequence`), which reconstructs the base pointer and per-store offsets
through the PTRADD/ADD forest. Rewrite: the run collapses to one
`memcpy`/`strncpy`/`wcsncpy` builtin CALLOTHER whose source is an internal
string the printer renders as a quoted literal. Failure mode: a guard miss
(interference, gap, non-printable bytes) declines and the per-element
assignments simply remain; the interference check exists because moving all
the writes to one call site is only sound if nothing observed the array
half-written.

**(kuna GH-9230/1537) Constant fill —**
[`memsetrecover`](../options.md)**, default on** (DIV-2).
`decompiler/crates/kuna-decomp/src/p5_types/kuna_memsetsequence.rs
(RuleMemsetCopy)` extends the same machinery to runs that spell no string: an
unrolled or SIMD `memset`/`bzero` otherwise renders as dozens of
`buf[i] = '\0';` stores. It reuses the string driver's collection
(`constseq.rs (StringSequence)` `build_for_fill`) and applies the fill test
`kuna_memsetsequence.rs (detect_fill_run)`: sorted by offset, the COPYs must
tile a contiguous region with one repeated fill byte, with **at least 2 COPYs
and a 16-byte minimum footprint** — the guard that keeps a lone string NUL
terminator from being claimed as a memset (the Stack-string ablation in
DIV-2). Rewrite: one `builtin_memset(dest, value, count)` CALLOTHER; teardown
shares the string path's COPY removal. Off restores the per-element stores.

**Bitfields** (`decompiler/crates/kuna-decomp/src/p5_types/bitfield.rs`).
Pattern: a struct with sub-byte fields is accessed through shift/mask soup on
a byte container. The six `cleanup` rules fire only when the container's type
*has* declared bitfields (`Datatype::has_bitfields` — the triples collected
from `TypeBitField` in `dtype.rs`): `RuleBitFieldStore`/`RuleBitFieldOut`
trace backward from a store (or mapped write) through the OR/AND/SHIFT web and
re-express it as explicit `INSERT` ops per field
(`decompiler/crates/kuna-decomp/src/p5_types/bitfield/insert.rs
(BitFieldInsertTransform)`); `RuleBitFieldLoad`/`RuleBitFieldIn` trace forward
from a load and re-express the extractions as sign/zero `PULL` ops
(`bitfield/pull.rs (BitFieldPullTransform)`); `RulePullAbsorb`/
`RuleInsertAbsorb` (`bitfield/absorb.rs`) then consolidate a shared byte
container so each field renders as its own `ptr->field = …` statement. All
geometry runs through the endian-aware `bitfield.rs (BitRange)` value type —
bit numbering is where big/little endian diverge, and getting it wrong scrambles
adjacent fields. Failure mode: any trace step the transform cannot prove
(e.g. a masked value escaping to an op outside the recognized web) declines
before mutation, leaving the raw shift/mask expressions in the output; the
type is never consulted speculatively, so untyped code is untouched.

**Preferred splits**
(`decompiler/crates/kuna-decomp/src/p5_types/prefersplit.rs
(PreferSplitManager)`). The inverse of §5.5: some processors keep two logical
values in one physical register (SIMD halves), and the spec can declare a
`<prefersplit>` table of storage+offset records. Wherever the whole register
appears as the single producer/consumer of a COPY/PIECE/SUBPIECE/LOAD/STORE/
INT_ZEXT, the manager rewrites that op into two piece-ops (each opcode has a
paired `test*`/`split*` guard, and the op-insertion order is transcribed
exactly because it is output-determining); a second sweep (`split_additional`)
cleans up temporaries the first sweep exposed. Honest port status: the
transforms are ported and unit-tested, but the pass-0 heritage hook that
drives them (`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs
(Heritage::heritage)`) is a documented stub — inert for every architecture
without split records, which is the entire current test surface, so no live
output depends on it yet.
