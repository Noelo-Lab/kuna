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
(DIV-10/12/14/17/18/23/25 in `docs/history.md`).

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

**`guardarm` — which arm becomes the `if` clause.** `rule_block_if_no_exit`
folds the condition block and *one* terminal out-arm into a `BlockIf`; the
other arm survives as that node's fall-through, so the choice decides which
half of the function is nested inside the `if` and which is written flat
after it. Upstream walks the two out-edges in index order and takes the first
eligible one. Usually only one arm qualifies (in-degree 1, out-degree 0, a
decision edge) and there is no choice to make — but when the condition guards
a fatal call and the other side is the function's own `return`, **both** arms
qualify and the index order decides. Index order carries no source
information here: `negateCondition`'s `swapEdges` re-orients the block between
the two `collapse_all` runs the action pool performs, so the orientation the
rule sees is not the orientation the branch was assembled with, and the
`i == 0` arm is as likely to be the `return` as the fatal call. When it is the
`return`, kuna emits `if (ok) return v; fatal();` where the source, IDA and
Ghidra all write `if (!ok) fatal(); return v;` — the condition negated, the
body re-parented one level deeper, and the fatal call demoted to the
function's trailing fall-through.

`guardarm` breaks **only that tie**, and by code layout: of the two eligible
arms, the one whose first leaf lies earlier in the address space becomes the
clause. It takes `off`, `auto` and `on`; `auto`, the default, applies the
layout only in a function chapter 03's detector calls unoptimized
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_unoptimized.rs (detect)`,
read here through `is_unoptimized`), `on` in every function and `off` in
none. An unoptimizing compiler emits blocks in source order, so its layout is
the author's choice; an optimizing one moves cold paths and shares tails, so
elsewhere the earlier arm says nothing about the source. On the decbench
corpus the layout applied everywhere wins 279 and loses 58 exact matches at O0
but loses 67 against 24 at O2; under `auto` the O2 output is unchanged. An unoptimized compiler emits the taken
clause of `if (c) A; B;` in front of `B`, and emits the `then` side of an
`if/else` in front of the `else` side, so the earlier arm is the source's
`if` body in both shapes. A block with a single eligible arm is untouched, so
the rule's behavior is unchanged everywhere the choice was already forced.
The predicate deliberately does **not** consult the artificial-halt
(`op_mark_halt`) flag: preferring the no-return arm gets `openssh scp
xcalloc` and `dpkg-query control_list` right but inverts `coreutils
make-prime-list xalloc`, whose source really is `if (p) return p;
fprintf(...); exit(1);` — layout order gets all three right, because it is
reading what the compiler recorded about the source rather than guessing from
the callee.

**`loopcondhoist` — which *block* the deferred scan folds first.** The deferred
scan walks the live components in order and folds the first one whose terminal
arm qualifies, then restarts the whole cascade from the top. A head-tested loop
whose exit arm is a `return` is such a component — once `ActionReturnSplit` (or
`returndup`) has given that return in-degree 1, `rule_block_if_no_exit` matches
the loop head — and the head sits ahead of its own body in component order. So
the head folds to `if (!C) return X;` and drops to one out-edge, at which point
`rule_block_while_do` can never match it again: the loop is emitted as
`while( true ) { if (!C) return X; BODY; }` where the source and IDA write
`while (C) { BODY }`. Nothing later recovers it; the head test is now a
statement, not a condition.

The block carrying the loop's `break` is a candidate in the *same* scan, and
folding it first is strictly better. Its clause is the loop's follower, which
the rule already requires be reached from nowhere else (`size_in() == 1`), so
absorbing it into the break arm is semantics-preserving by the rule's own
precondition; the body then collapses to a single back-edge clause and the head
test hoists into the `while`/`for` header on the next cascade pass with no new
machinery and no relocated statement. `loopcondhoist on` (default-off opt-in)
therefore gives the non-heads one pass of the scan to themselves — skipping
components that are live loop heads, resolved through the collapsed graph the
same way `LoopBody::update` resolves a recorded head — and falls back to the
unrestricted upstream pass only when that finds nothing. A function with no
loop-head candidate is byte-identical, and the scan still terminates on the same
fixpoint because each fold strictly reduces the component count.

**`loopcontinue` — preserve secondary latches before greedy loop folding.**
A single-entry natural loop can have several back-edges to the same head:
error retries and early continues, followed by the ordinary end-of-body latch.
The collapse schemas can consume each retry as a smaller loop before their
fallback considers virtualizing any edge, producing nested infinite loops for
one source loop. With `loopcontinue on` (the default),
`kuna_loopcontinue.rs (refine_latches)` runs on the freshly seeded collapse
mirror, including after a region-structurer fallback. It requires one ordinary
entry, at least two back-edges, and domination of every latch by the head.
A head with an exit outside the natural-loop body, a switch anywhere in that
body, another loop head in that body, or a conditional retained latch is
excluded, preserving existing head-tested, bottom-tested, and switch-loop
schemas. The highest reverse-postorder latch stays structural; the others are marked as gotos before schema matching.
Every body node must have no outside predecessor except the head, and any
irreducible edge touching the body rejects refinement: dominators are computed
with those edges masked and alone cannot establish single entry.
No p-code, statement, or destination changes.

`kuna_loopcontinue.rs (recover_continues)` runs before `mark_unstructured`.
The value `auto` (the default) adds the record of jump statements in
unoptimized code described under `loopcontinue auto` in §8.3.
A goto or if-goto becomes `continue` only when its target's first leaf is the
first leaf of its innermost `BlockInfLoop`, or the continue point of its
innermost conditional loop. Entering a nested loop resets that scope. A
conditional loop's `continue;` evaluates a test or an iterate statement rather
than restarting the first leaf, so `kuna_loopcontinue.rs (continue_points)`
gives each one the leaf a `continue;` resumes at, or none. A while-do resumes
at the front leaf of its condition block, unless it prints as a `for` (an
iterate op), whose `continue;` would run the iterate statement first; in the
overflow form the condition block prints first inside `while( true ) { ... }`,
which is where the jump lands too. A do-while resumes at the last leaf of its
body only when that leaf prints nothing but the loop test: every op besides
the branch is not printed, a marker, or has an implied output, and only when
the output language's do-while runs its test on `continue;`: C's `do..while`
and Go's `for` post statement do, while Rust prints a do-while as
`loop { ..; if !(c) { break; } }`, where `continue;` would skip the test, so a
Rust do-while keeps the goto (the `do_while_continue` flag the architecture
derives from the output language). A switch
between the jump and the loop does not matter, since C's `continue` skips
switches. A jump into any other leaf, including the latch of a `while( true )`
loop, keeps its goto. `off` restores the original collapse precedence and
rendering.

**(angr) `condfold` — folding across a *complex* sibling.** `rule_block_or`
requires the sibling condition block (upstream's `orblock`, the right operand
of the prospective `&&`/`||`) to be *non-complex*: `substrate/funcdata_block.rs
(Funcdata::bb_is_complex)` counts the block's statements — the trailing branch
counts as one — and anything past two declines the fold. In practice that means
the sibling must be a bare test. When the compiler parked even one extra
statement in front of the second test (a spill, an address computation, a call
whose result *is* the test), upstream refuses to fold. Two shapes pay for that,
and they are the two proposals this option closes:

* a **crossing goto** — the second condition becomes its own block that
  branches back into the first arm's clause, where angr renders one compound
  condition. On coreutils `tr::build_spec_list` that is literally `result_idx =
  &v9; if (es_match(es_00,v11,'=')) goto label_40242e;`.
* a **goto + label into a guard cascade's shared body** — three permission
  guards whose arms reconverge cannot fold pairwise, so one of the two edges
  into the body has to be virtualized. On lighttpd `server.c::main` (angr
  corpus binary `newbury`) kuna emits 1 goto / 1 label where angr emits 0 / 0.

angr does not decline on either. `phoenix._match_acyclic_short_circuit_conditions`
(types a–d) checks `_is_single_statement_block` and, on a multi-statement
sibling, wraps the operand in an AIL `MultiStatementExpression` — a C **comma
expression** — under the `_should_use_multistmtexprs` policy (`MAX_ONE_CALL`:
at most one call and at most `_multistmtexpr_stmt_threshold`, default 5,
non-label statements), refusing outright on a non-terminal conditional jump or
any unconditional jump. kuna's `comma_separate` printer modifier *is*
`MultiStatementExpression`; kuna's `bb_is_complex` *is*
`_is_single_statement_block`.

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_condfold.rs` ports exactly
that relaxation and nothing else. It widens the single `is_complex(orblock)`
decline in both engines: a complex sibling is accepted when **either** of two
admission rules takes it. The predicate is their union — the two rules were
derived independently against the two shapes above and neither subsumes the
other.

**The shared width budget.** Every admitted sibling first has to fit the
policy level's **printed-width budget**: `kuna_condfold.rs (printed_shape)`
walks the block's live op list with the *same* skip rules the printer applies
(`p9_emit/printc.rs (PrintC::emit_basic_block_ops)`: not-printed ops, SSA
markers, ops whose output varnode is implied, and the bare `CPUI_BRANCH`) and
counts what is left, the terminal `CPUI_CBRANCH` included because it renders as
the chain's last element. That count must not exceed five at `on` or nine at
`wide`. `printed_shape` also refuses outright on a block that does not end in a
`CPUI_CBRANCH`, or in which a non-terminal branch op survives the skips — angr's
own refusal, and a `goto` is not an expression. Both admission rules are then
applied on top of that one measure, so the level is the only thing that changes
how wide an operand either rule will let through.

**Rule A — bounded prefix.** The sibling is a `BlockCopy` of one `BlockBasic`
that `kuna_condfold.rs (prefix_eligible)` approves: within the width budget,
with at most one *statement-root* call, and carrying no comment — because
`emit_basic_block_ops` suppresses `emit_comment_group` under `comma_separate`
and folding would silently delete the comment. A
`BlockList`/`BlockIf`/`BlockCondition` sibling is never taken by this rule — such
an operand could render braces, multiple lines, or a label inside the
parentheses, i.e. invalid C.

**Rule B — statement shape.** `kuna_condfold.rs (shape_score)` applies an
explicit allowlist on top of the same width budget: every op must be a marker
(never printed), an op with an output varnode, a void `CALL`/`CALLIND`, a
`STORE`, or the single terminal `CBRANCH`. A `RETURN`, a second branch, a
`BRANCHIND`, a `CALLOTHER` (userop rendering can be multi-line or
special-cased) or a no-return call declines the block outright, as does a
comment of any type. Per block it allows at most two conservatively-scored
statements and two calls — counting *every* call, unlike Rule A.

Those two *scored* statements are counted with the same
`Varnode::calc_explicit` approximation `bb_is_complex` uses, rather than by
reading `Varnode::is_explicit()`, which is not yet meaningful when structuring
runs. **That score is an approximation of printed width and is not one.** On
`tr::build_spec_list`'s third-term sibling it reads 2 for a block that renders
seven comma elements; while it was Rule B's only length cap, `on` emitted the
option's widest operands and `wide` could not tell itself apart from `on` on any
named target. The score is retained only because the cross-leaf caps below are
expressed in it.

Rule B additionally admits a **nested `BlockCondition`** as the sibling, which
Rule A never does, and that is what lets a guard *cascade* fold: `newbury::main`
folds its second and third guards first and then takes the resulting Condition
as the sibling of the first. `printc.rs (PrintC::emit_block_condition)` emits
*both* sub-blocks of a nested Condition under `comma_separate`, so every leaf
must itself be Rule-B admissible (the recursion in `blockaction.rs
(CollapseStructure::is_shape_foldable)`), and the fold site enforces two
cross-leaf caps — at most four condition leaves and at most four scored
statements summed over them — because `collapse_conditions` is a fixpoint loop
with no natural bound. A leaf admitted only by Rule A is deliberately *not*
admissible as a nested leaf: a Rule-A block can hold up to nine printed
statements, which the cross-leaf caps have no honest way to charge, so a Rule-A
block may only ever be the direct operand of one fold.

**Which rule fires, and what separates them.** The disjuncts are tried in
order — `blockaction.rs (CollapseStructure::condfold_ok)` asks Rule A first — so
a sibling both rules accept folds through Rule A, and the two properties the
rules share can never tell them apart: the printed-width budget is enforced by
both on the same measure, and the comment guard declines both. Two properties
are Rule B's alone. One is the nested `BlockCondition` operand above. The other
is the call cap, and unlike the width budget it does **not** move with the
policy level: Rule A charges at most one statement-root call at `on` *and* at
`wide`, where Rule B allows two. A guard block holding two calls that each print
as their own comma element is therefore outside Rule A at every level and inside
Rule B. That is the shape of the middle guard in `tests/stages/ghangr-condfold-ruleb.xml`
(`su_log(); x = su_euid(); if (x != wu)`, where the failure arm consumes `x` so
the second call's output is not implied and prints as its own element): both of
its folds are Rule B's, in both engines, and the second one's sibling is a
nested `BlockCondition`, so the witness covers both exclusive properties in one
run. Removing Rule B reverts its second pass to its first.
`tests/stages/ghangr-condfold.xml` is the Rule A witness, and
`tests/stages/ghangr-condfold-newbury.xml` — the guard-cascade *shape* witness —
folds through Rule A too, because at `wide` Rule A's budget also admits its
guard blocks.

Both are observable. `KUNA_CONDFOLD_DEBUG=1` traces every per-block admission
verdict (with its decline reason and both of Rule B's measures) and every
relaxed fold, naming the engine that took it and the rule that admitted it;
`KUNA_CONDFOLD_DEBUG=ops` additionally lists each op `printed_shape` charges,
the terminal `CBRANCH` included, so the listing is exactly as long as the width
the same walk reports and a measurement can be reconciled element by element.

Two guards live at the gate rather than in either predicate. The pre-existing
`!is_interior_goto_target` decline is never relaxed, because `printc.rs
(PrintC::emit_block_copy)` emits the label statement *first* and a labelled
sibling would print `label_x:` inside an expression. And a node produced by a
relaxed fold is force-marked complex, because `BlockCondition::isComplex`
delegates to sub-block 0 and would otherwise report the *left* operand's
trivial verdict for a node whose right operand is a comma chain — letting
`rule_block_while_do` hoist the whole chain into a `while(...)` header. That
marking is also what keeps Rule B's expression-size caps reachable: a relaxed
result always re-enters the admission check instead of slipping past it as
non-complex. The caps nonetheless bound the relaxation only at each site where
it is *exercised*; they are not a global bound on the final condition's width,
because a purely upstream-legal fold (both operands non-complex) can still
extend a cascade on the left without consulting them, exactly as it can with
the option off.

**What the width budget bounds, stated exactly.** Per admitted leaf, the comma
chain `printed_shape` measures is at most five elements at `on` and nine at
`wide`. Measured across 2827 functions in seven binaries, the widest comma
operand the option *creates* is five elements at `on` and seven at `wide`; the
widest operand present in the emitted C at `on` is six, in `dd::human_readable`,
and that one is already there with the option **off** — Ghidra's own
`bb_is_complex` budget has the same score-versus-width slack, and refusing to
inherit it is the whole point of this measure. Three things the budget does not
bound. It is a per-leaf bound, so a Rule-B cascade of up to four leaves has no
bound on their summed printed width (the only cross-leaf cap is expressed in the
weak score). It is taken on the op list as it stands at structuring time, so a
later pass can still add or drop an op — on `tr::build_spec_list` the measure
reads six to nine across the structuring passes and the operand finally emits
eight elements. And where a call's stack-effect ops are still live at
structuring time (an unprototyped callee with no frame recovery), it
over-counts: the stage witness `tests/stages/ghangr-condfold-newbury.xml`
measures six for a guard block whose operand finally emits none, which is why
that witness needs `wide` although the real `newbury::main` closes at `on`.
Over-counting only ever declines a fold, so the error is in the safe
direction.

Nothing moves. `substrate/block.rs (BlockGraph::new_block_condition)`
re-parents two existing structuring nodes; no p-code op is moved, reordered,
cloned, or deleted, and the absorbed statements are printed by `printc.rs
(PrintC::emit_block_condition)` inside the right operand under the existing
`comma_separate` modifier. Soundness rests on a precondition `rule_block_or`
already enforces and this option never weakens: the sibling is single-in, so
its statements cannot execute on a path where the compound condition is not
evaluated; C's short-circuit rule evaluates the right operand exactly when
control used to fall into the sibling; and the comma operator is a sequence
point, so the prefix still runs before the test. The only synthesized operation
is `negate_condition_rec`, a boolean sense flip on an already-computed
condition, which is order-neutral. Consequently the concern raised in the
second proposal — that a de Morgan inversion cannot reorder predicates which
call `getuid`/`geteuid`/`getegid`/`getgid` — does not arise: the operands are
never swapped, only relabelled, and no purity analysis is needed. Every guard
listed above is a rendering-validity or readability guard, not a semantic one.

Rule A's call cap deserves a precise statement, because it is *not* an exact
match for angr's `MAX_ONE_CALL`. The eligibility walk mirrors the printer, and
the printer's implied-output skip necessarily runs **before** the call test —
an implied call is not a statement at all — so the cap counts only calls
printed as their own comma-chain element. A call whose result is inlined into
the sibling's own condition is not charged, and a folded operand may therefore
render more than one call (`mv -O2 copy_internal` at `wide` renders
`cached_umask(...)`, counted, alongside `fchmod(...)`, implied into the test
and not counted). Keeping the statement-root semantics is a deliberate choice:
the cap exists to bound the printed comma chain, the fold moves no p-code, and
the sequencing argument above is independent of the call count, so the looser
count cannot produce wrong C — it only admits a denser operand than angr would.
Rule B, which charges every call op, is the tighter half of the union.

Option `condfold`, default **off**, three-valued: `on` uses angr's statement
threshold (width budget 5, and Rule A additionally 1 statement-root call) and
`wide` raises the width budget to 9 for **both** rules; Rule A's call cap, Rule
B's shape allowlist and scored caps, the cross-leaf caps and every soundness or
rendering guard are identical at both levels. `wide` exists because kuna's
*printed* statement granularity is finer than angr's AIL for the same block —
argument recovery may leave operand computations as separate assignments where
angr renders one call statement, and an address-tied stack variable prints copy
shadows the AIL never materializes — so a block angr folds as one statement can
print as seven in kuna. Both levels apply to **both** engines (`blockaction.rs
(CollapseStructure::rule_block_or)` and `region_structurer.rs
(RegionStructurer::try_block_or)`, which are verbatim copies of each other);
implementing only one would no-op on functions where the other converges. Off,
the precompute is skipped and every disjunct is dead, so output is
byte-identical to upstream.

Two effects are accepted rather than fixed. The option is **not a monotone goto
reducer**: folding changes which edges the structurer can subsequently match,
so an individual function can gain a goto even where the aggregate falls
sharply. And a **later-produced advisory comment can be lost**: both rules
decline a block that already carries a comment, but the check only sees the
comments buffered in `Funcdata` at structuring time, while `comma_separate`
suppresses `emit_comment_group` for the whole operand — so an advisory emitted
by a pass that runs after structuring can still vanish (`newbury::main` emits
four `WARNING:` lines at `off` and three at `on`). Those are kuna
annotations, never a correctness signal, and the emitted C is unaffected.

**`condstmts` — a cap on what a folded operand prints before its test.** The
opposite pressure: Ghidra's own non-complex budget can let a block fold whose
operand then prints a long comma chain. `bb_is_complex` scores a block with the
`calc_explicit` approximation while the function is structured inside the main
loop, before `ActionAssignHigh`; values that merging later makes explicit — a
load kept in a callee-saved register across a call, a copy into a merged
variable — are free in that score and cost one comma element each in the
emitted C. The structure is not rebuilt after `ActionMarkImplied` (the final
`ActionBlockStructure` returns at once when the tree already exists), so the
fold stands. The witness is four byte loads feeding a compare against a call's
result: it scores as one statement and prints as
`(v1 = *a2, v2 = a2[1], v3 = a2[2], v4 = a2[3], f(a0) == ...)`.

With `option condstmts N`, `p8_structure/kuna_condstmts.rs (record_hits)`
runs at that final `ActionBlockStructure`, where implied flags are final. It
walks the tree for the comma-printed positions — every leaf under the right
operand of a `BlockCondition` and under the condition of a `BlockWhileDo`
without overflow syntax — and measures each leaf with `kuna_condfold.rs
(printed_shape)`, the printer's own skip rules. A leaf printing more than `N`
statements before its terminal `CBRANCH` has its block start recorded on the
`Funcdata`. `infra/decompile_drive.rs
(decompile_func_full_with_override_dyn_prefollowed)` then analyzes that
function again from scratch with the recorded starts as a seed (kept across
`Funcdata::clear`, so an in-pipeline restart keeps it), at most twice, keeping
the last successful result. During the re-analysis
`kuna_condstmts.rs (complex_blocks)` adds every seeded block to both engines'
complex set, so `rule_block_or`/`try_block_or` decline the fold and
`rule_block_while_do` gives the loop overflow syntax. Re-analysing, rather than
restructuring in place at the final pass, is deliberate: by then
`ActionPreferComplement` and the structurers' own branch negations have already
rewritten `CBRANCH` polarity for the first tree, and a fresh collapse over those
ops re-orients unrelated `if`s and loses unrelated folds. The cost is a second
analysis of each affected function, and only those. Once highs exist (a tree
rebuilt after a late structure reset), `complex_blocks` also includes every
block that measures over the cap directly. The cap also bounds `condfold`:
`kuna_condstmts.rs (condfold_budget)` passes `min(condfold budget, N + 1)` to
`compute_condfold_sets`, since that budget counts the terminal `CBRANCH`.

The cap only removes folds, and can therefore only add statements and, where a
declined fold leaves two edges into one block, a goto. Default `3`: 0/675
datatest assertions change, and over 3,293 functions in six x86-64 binaries two
functions change (one of them gains two gotos) at no measurable cost outside
the re-analysed functions. At `2`, eight functions change and gotos rise by
four, but the affected functions are large enough that the second analysis
costs +8% (grep) and +26% (libselinux) of whole-binary time. `off` makes
`record_hits`, the seed and the budget clamp inert, so the output is
byte-identical to the uncapped form. Witness: `tests/stages/kuna-condstmts.xml`.

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
branch condition merge into one decision point). Both merge discovery and
exit-edge repair inspect every MULTIEQUAL in each exit block. Stack analysis
can replace an earlier stack-pointer phi with an INT_ADD in place, leaving
other live phis behind it. Rejoining two predecessor edges must still merge
their two values and remove exactly one input from each later phi, preserving
unrelated predecessors and their input order. Ordinary exit operations retain
their operands and block; a phi that has only one input after the repair becomes
a COPY. `blockaction.rs
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
A for-loop header's initializer and iterator are each a statement moved out
of the body or the pre-loop block into the header, so the move must not
reorder them against a printed statement. When the PHI's input is a hidden
COPY, the statement the header prints is the one feeding that COPY, not the
COPY itself; `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::while_test_terminal)` therefore requires that statement to move,
respecting cover, up to the COPY as well. A call, or a statement whose result a
printed statement between them reads (`n = v & 1;` ahead of a loop over
`v = strtoul(...)`), keeps its place, and the header goes without it.
Upstream sorts the arms of a switch by label, and the `default:` arm sorts under
whatever label its table entry carries, so it prints first or between two cases.
An arm whose code ends in a call that does not return prints nothing that stops
it, so the arm printed after it reads as the code it falls into — `default:
die(...);` followed by `case 1: return "[*]";` shows an edge the program does not
have, and a reader takes it at its word. After the sort,
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_switchcaseorder.rs
(order_cases)` keeps the label order and every fall-through chain together and
moves the arms that would read wrong: the group holding `default:` after the
other arms (the order authors write), and an arm whose last basic block has no
successor and ends in a call rather than a return
(`kuna_switchcaseorder.rs (ends_unstopped)`) after all of those. Only the last of
several such arms can end the switch, so each of the others is printed with a
`break;` after its call (as in the `usage(0); break;` of a getopt loop). This is
a correctness fix with no option: no order it changes printed anything the
program does.
`blockaction.rs (ActionNormalizeBranches)` remains a non-mutating compatibility
action in the `jumptable` and `normalize` groups. It preserves its registration
and initialized-root check, but does not traverse branches or allocate unused
flip lists. Branch opcodes, operands, flags and edge order remain unchanged;
removing this discarded analysis does not enable branch normalization.

**The label invariant.** An emitted `goto label_X;` without a `label_X:` in
the same function is not C, so the set of blocks
`decompiler/crates/kuna-decomp/src/substrate/block.rs
(BlockGraph::mark_unstructured)` labels must cover every goto the printer
emits. It marks the front leaf of a `BlockGoto`'s target, a `BlockIf`'s goto
target, and each `BlockSwitch` case arm whose edge stayed unstructured.
Upstream marks a `BlockGoto`'s target only when `gotoPrints()` holds — the
target is not the block flow reaches next — because upstream's
`PrintC::emitBlockGoto` suppresses the goto under the same test. kuna's
printer (`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs
(PrintC::emit_block_goto)`) emits the trailing goto for every target it has,
so the mark here is deliberately wider than upstream's: gating it left the
printed `goto label_X;` unlabelled whenever its target happened to be the
next block in flow. The redundant goto itself is kept rather than suppressed,
because dropping a statement is a structuring decision, not a label fix.

**Final component ordering.** Structuring does not always end with a single
top-level component: an irreducible knot, a multi-entry loop, or a switch whose
arms could not be folded leaves the root `sblocks` graph holding several
siblings, and the printer emits them in list order
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`, a plain walk of the
component list). Collapse order is not print order — the collapse appends each
new composite at the end of the parent's list, so the component that happens to
collapse last lands last no matter where its code lives in the function. The
first step of `blockaction.rs (ActionFinalStructure)` therefore re-sorts the
root's components into their final order
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::sblocks_order_blocks)` →
`decompiler/crates/kuna-decomp/src/substrate/block.rs
(BlockGraph::order_blocks)`): the component containing the function entry
first, components that end in a RETURN last, everything else by ascending
block index — the reverse-post-order number that `block.rs
(BlockGraph::add_block)` keeps at the minimum over a composite's members, so a
composite sorts by its earliest constituent block. The sort applies to the root
graph only; nested components already carry their schema's own order. It is a
printing decision, not a transform: the same statements are emitted, only the
order of the top-level pieces changes, and both engines (§8.1 and §8.2) reach it
because both hand the same root list to `ActionFinalStructure`. Without it, a
function whose structuring left more than one component can open in the middle
of its body and reach its real entry only after an unconditional `goto`, i.e.
emit the entry as unreachable dead code. Ties are broken stably, so the
several RETURN-terminated components the ordering rule treats as equal keep the
relative order the collapse produced instead of an arbitrary one.

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
`readability-rewrites` subphase in `phases.toml`. Invariant: the flip is a
*pure* arm swap, so the multiset of leaf components reachable under the
`BlockIf` is identical before and after it — a debug-build assertion in
`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::block_if_flip_negated_guard)` checks exactly that, and polices the
whole arm-rearranging family against a dropped or re-parented component. What
the swap *does* change is which arm the printer sees in the else slot; when
that arm is an `if` whose condition component carries statements, the `else if`
collapse governs the result, and its ownership rule is chapter 09's
*pending-brace ownership* (a nested frame that steals the collapse hoists the
arm's body onto the then-path).

## 8.2 The region structurer (angr)

`decompiler/crates/kuna-decomp/src/p8_structure/region_structurer.rs` is the
port of angr's Phoenix/SAILR structurer (parenthetical provenance: angr
`analyses/decompiler/structuring/{recursive_structurer,phoenix,sailr}.py`),
grown increment by increment (the archived plan is
the retired region-structurer roadmap, git history) and made the default S8 engine
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
7. The switch schema once more, splitting: a switch held back only because a
   continue case and its exit lead to different blocks has those continue
   cases made gotos (the one-successor rule at the end of this section).
8. `region_structurer.rs (RegionStructurer::virtualize_one_edge)` — the last
   resort: mark one structured edge as a goto (below). Because this always
   removes an edge, the loop converges; the only "stuck" exit is a component
   set with no virtualizable edge at all.

A hang-guard caps the rounds at `2*n^2 + 64` (`region_structurer.rs
(round_cap)`) so a mis-port fails cleanly instead of spinning; the cap, a
stuck knot, a schema error, or a finished tree that holds a statement where a
condition belongs (`kuna_switchcond.rs (misplaced_condition)`, below) all make
`structure()` report non-convergence, upon which `ActionBlockStructure`
re-seeds and falls back to `CollapseStructure` (§8.1).

The condition-folding and schema loops also check the current function's time
and memory budget before each round. Exhaustion stops the attempt, and
`ActionBlockStructure` returns without re-seeding or invoking the fallback
structurer. The drive then reports the usual per-function budget error instead
of printing the unfinished graph; later functions in a batch still run. With no
budget, these checks do not change the folding order or output.

A doomed attempt is also cut short. Every schema except the switch-loop
absorb merges components along edges, and no step adds an edge, so a live
component left with no in- or out-edges (typically after `rule_block_goto`
removes the last edge into a block whose own exits are already gone) can
never join the rest. Each round therefore first asks `region_structurer.rs
(RegionStructurer::has_stranded_component)` whether such a component exists
while others remain, and reports non-convergence at once if so. The one
exemption is a component the address-keyed absorb could still collect: its
address lies in a pending switch-loop body, or another live component shares
it (a later body set could then name it). A declined attempt is discarded
whole, so stopping early changes no output; it only skips rounds. That matters
on a large goto-heavy function, which declines once per `structure_reset` the
CFG passes take: Bash's `declare_internal` declines 18 times in one `kuna
decompile`, and each attempt strands a component within its first ~100 of
~800 rounds.

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
is never chosen. Candidate dominance queries use an iterative dominator-tree
interval index (`p8_structure/kuna_domindex.rs`) built once per immutable
snapshot. Subtree containment replaces repeated parent-chain walks; absent
nodes retain the chain walk's self-dominance behavior. The index is discarded
with the snapshot, so graph changes cannot leave stale intervals. Interval
slots are arrays indexed by node arena id (each snapshot has a fresh pool, so
ids are dense): building costs one immediate-dominator lookup per node plus
linear array work, and each query is two array reads independent of
dominator-tree depth, so the index is cheaper than the chain walk even on the
shallow trees of ordinary functions. Candidate order and ranking are unchanged.
Within the winning bucket (`sailr_order_within_bucket`): H1;
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
- **Print-tree rewrites** after `ActionFinalStructure` (`gotoreduce`'s
  two passes, `ifelseflatten`, `crossjumprevert`, `dedupitetail`, `tailform`,
  `iteregion`, `iteboolean`, in that registration order): they edit only the structured
  `sblocks` tree (fresh `BlockCopy` leaves referencing the *same* underlying
  basic blocks, or `list`/`parent` splices, or a print mark) — no p-code is
  cloned, SSA/def-use is untouched, and the printer re-emits the same ops in
  the new layout.

A CFG split must leave every path performing the same writes, the same number
of times. The p-code half of `node_split`,
`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs (CloneBlockOps)`,
copies two kinds of op that write nothing but name a global as their output.
Upstream's clone let copy propagation turn both into stores (kuna #1355):

- *The return guard.* A RETURN block carries one `Heritage::guardReturns`
  `glob = glob` COPY per persistent global (§3.1). It is flagged `return_copy`,
  and that flag is what keeps `RulePropagateCopy` off it. Upstream's
  `buildOpClone` mask (`funcdata_block.cc:982`) drops the flag, so the clone's
  guard took the value the function last stored and printed a second
  `glob = <value>;` on the cloned path, after the real store had already run.
  `CloneBlockOps::build_op_clone` keeps the flag.
- *The phi.* A return block reached on paths that leave a global with different
  values starts with a MULTIEQUAL for it. Upstream's `patchInputs` turns the
  clone's copy of that phi, and the original once one edge is left, into
  `glob = COPY glob` from the incoming value. Propagation then turns that into a
  second store of what the predecessor stored. When the predecessor's own store
  had no other reader, the store was dropped and the copy printed it at the
  return instead, which reorders the function's stores.
  `CloneBlockOps::settle_one_input_phi` settles every phi the split leaves with
  one input. A phi from a global to the same global writes nothing: its readers
  take the incoming Varnode and the COPY is destroyed, in the clone and in the
  original block alike. A copy left in the original is no safer than one in the
  clone: the next split of that block clones it as a plain COPY, and
  `ActionVarnodeProps` replaces a known-zero COPY output with the constant 0, so
  the guard that read it printed `glob = 0;` a second time (bash `sh_getopt`,
  scp `getopt_internal`). A guard that reads a one-input phi through a temporary
  (`ConditionalExecution` builds those) takes the incoming Varnode when that is
  its own global.

What the split cannot recover is where a store sat once copy propagation folded
its value into the phi (`RulePropagateCopy` lets a non-constant value into a
MULTIEQUAL, and the store then dies). The clone's copy of that phi is the store,
at the start of the clone, after every store the predecessor kept; several such
stores print in the order the phis are cloned. That order is set by
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_clonephiorder.rs (order_global_phis)`,
always on: the phis on globals are cloned after the others and in address
order, which is how kuna printed most of these stores before the clone kept its
guards' flag. Block order would be the reverse of heritage's placement
(heritage inserts each phi at the block start in ascending address order, so
one pass's phis end up in descending address order). Only the clone is reordered: the original block keeps its op order,
which merging also reads (reordering its phis too printed bash
`get_urandom32`'s seed store inside the loop it leaves). Neither order comes
from the binary. Over 140 corpus binaries the two
orders differ in 119 store runs; of the 34 the disassembly decides, on matches
the binary in 21 and off in 9.

A `store; cmp; cmov; ret` function (clang and gcc, -O1 and -O2) hit the first
through `returndup`, `earlyreturn` and `switchreturn` alike, because SLEIGH lifts
the CMOV as a branch around one move, which leaves the RET with two in-edges. A
`-O0` function whose paths all jump to one `leave; ret` hit the second.
`ActionReturnSplit` clones through the same helper. Keeping the flag and
settling the phis is a strict fix with no option.

Two more run inside §8.1's machinery: `branchflip` (described there) and
`loopbreak_recovery` (inside `ActionFinalStructure`, below). All of them
share the safety stance: exact structural preconditions, hard caps, and
decline-on-any-doubt — there is no post-hoc validation to catch a bad edit
(§8.4).

### gotoreduce — return-tail duplication (angr `ReturnDuplicator`, `ReturnDuplicatorLow`)

`CollapseStructure` has no schema that duplicates a shared multi-predecessor
return tail, so one in-edge always renders as a goto. `gotoreduce` removes
that goto by inlining a print-tree copy of the tail into its source, clearing
the target's label when this was its last unstructured in-edge, so the goto
becomes a structured early return. The option's value says which return tails
qualify, and each tier is its own pass:

- `return` runs
  `decompiler/crates/kuna-decomp/src/p8_structure/kuna_gotoreduce.rs
  (ActionGotoReduce)` alone. *Pattern:* a `BlockIf` rendering as
  `if (cond) goto T` where `T` is a single-successor basic-block chain ending
  in `return`. *Transform:* the tail is inlined into the `if` arm
  (`kuna_inline_return_tail`). *Bounds/failure:* the chain is capped at
  `kuna_gotoreduce.rs (MAX_TAIL_BLOCKS)` = 3 blocks and `(MAX_TAIL_OPS)` = 8
  ops, and **any** `CALL`/`CALLIND`/`CALLOTHER`/`STORE` in the tail declines
  the match.
- `on`, the default, runs that pass and then
  `decompiler/crates/kuna-decomp/src/p8_structure/kuna_taildup.rs
  (ActionTailDup)` for the return tail that *contains a call* — the
  `free(p); return;` epilogue, which the call-free pass rejects and
  `crossjumprevert` (non-return tails only) declines. Two source shapes are
  matched: the bare `BlockIf` if-goto, and the `BlockGoto` leaf `{ stmts; goto
  T }` (the goto at the end of a multi-statement arm), each with the matching
  surgery. Because the tail ends in `return`, the duplicated call is reached on
  exactly one structured path, so no convergence proof is needed and the call
  runs as often as before. *Bounds/failure:* ≤ 3 blocks, ≤ 12 printed ops, at
  most `kuna_taildup.rs (MAX_TAIL_CALLS)` = 2 calls (angr
  `ReturnDuplicatorBase.max_calls_in_regions = 2` — a budget in calls rather
  than a veto), `STORE` always declines. The op count includes the casts
  chapter 09's `ActionSetCasts` inserted, since that pass runs first, so a cast
  policy that prints fewer casts (`castarith`) can bring a tail within the
  bound. Each duplication is logged (`taildup:`).
- `off` runs neither and keeps the upstream goto/label rendering.

`return` is the setting for the C `goto out;` idiom, where many early-exit
checks share one cleanup epilogue: `on` gives every check its own copy of the
cleanup call, which the source does not have, while `return` keeps the
epilogue behind its label and still turns the call-free return tails into
early returns. Both tiers were flipped on by DIV-14 (changing zero datatests).

### Releasing a label the duplication passes converted away

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_gotolabel.rs`. The three
passes that turn a `goto` into a duplicated tail (`gotoreduce`'s two passes and
`crossjumprevert`) each end by clearing `f_unstructured_targ` on the target
they converted, so a label whose last goto is gone does not linger. That is a
*release*, and it is only correct when no other carrier still jumps there, so
the surviving-carrier census is shared and covers all four carriers a goto can
ride: a `BlockGoto`, a `BlockIf` goto, a `BlockSwitch` case arm whose edge
stayed unstructured, and a `BlockMultiGoto`'s virtual edges. A census that saw
only the first two released the label of a block a switch arm still jumped to,
and the arm printed `goto label_X;` into a function with no `label_X:`
(`coreutils/sort` O2 `main`: `default: goto label_661d;`). The comparison is
on **front leaves**, not on the blocks the carriers name, because the label
lives on the target's front leaf and two carriers can reach one leaf through
different enclosing nodes — a `BlockList` and the `BlockCopy` it opens with.
The census is only ever read to *decline* a release, so it can keep a label
the printer would otherwise drop but can never remove one.

### retsplitglobal — a bound on cloning a global-writing epilogue

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_retsplitglobal.rs`, read by
`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::return_split_is_splittable)` — the predicate BOTH return splitters
consult, upstream `ActionReturnSplit` and kuna's `ActionReturnDup`
(`returndup`), which is why `option returndup off` does not change the shapes
this option governs. *Pattern:* upstream's `isSplittable` (`blockaction.cc:2242`)
asks whether a multi-predecessor RETURN block is a *bare* epilogue, cheap enough
to give every predecessor its own private copy. It admits only MULTIEQUAL, COPY
and RETURN and, for COPY, checks only that no INPUT is free; it never looks at
the COPY's OUTPUT. Upstream that is harmless, because the epilogue COPYs it was
written for move registers, which die at the return. A COPY into a *persistent*
Varnode is a store to a global the program can observe, and it is what P9 prints
as `dat_<addr> = ...`. *Transform:* count the block's observable global stores
and decline the split past `kuna_retsplitglobal.rs
(MAX_CLONED_GLOBAL_STORES)` = 8, so the epilogue stays shared. *A bound, not a
veto, is the design*, and it matches this phase's other duplication passes
(`crossjumprevert` and `dedupitetail` cap a duplicated leaf run at 8 printed
ops, `gotoreduce on` a return tail with a call at 12): a two-store epilogue is cheap to clone and reads better
cloned, because the merged form needs a phi per global and can print a global's
own value back into it on a path where the binary stores nothing; a 72-store
epilogue across three predecessors is 216 statements. *Bounds/failure:* only
COPY is counted (a MULTIEQUAL writing a persistent Varnode is a phi, not a
write — `node_split` rewires phis rather than duplicating an effect — and RETURN
writes nothing); `Heritage::guardReturns`' synthetic `glob = glob` self-copies
are excluded two ways, either sufficient, by the `return_copy` flag and by input
and output naming the same storage, because those are liveness markers present
before every RETURN in every function that has a global and counting them would
price a bare epilogue by how many globals the whole *program* has. Declining is
one-directional: it can only make a split not happen, so it cannot create a
duplication or change what the surviving merged block does. Settable
`retsplitglobal`, shipped default **on**.

### retsplitlimit — a bound on what one return split may clone

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_retsplitlimit.rs
(split_is_declined)`, read by `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::return_split_apply)` once a return block's split edges are known.
*Pattern:* `node_split` clones the whole return block once per split edge, and
a return block carries, besides its epilogue, one `Heritage::guardReturns`
`glob = glob` COPY per global the function reads — the liveness markers
`retsplitglobal` deliberately does not count. A huge function that reads
thousands of globals and reaches one return from hundreds of gotos clones
millions of ops, which every later pass and the variable merge walk: serde-hkx's
194 KB deserializer (GH-1034) has two return blocks of 4,524 ops reached by 674
and 658 gotos, 6.0 million cloned ops (8.75 million ops in the function right
after the split, 7.19 million of them guard COPYs). *Transform:* when the
block's op count times its split edges exceeds the limit (`retsplitlimit <n>`,
default 2^20 = 1,048,576 cloned ops), none of that block's edges are split and
its gotos stay; `off` restores the unlimited split. *Bounds:* the test is per
block and in cloned ops, so a block under the limit splits exactly as upstream
and the C changes only in a function that would have cloned more, where those
paths print a `goto` to the shared return instead of their own `return`. Like
`retsplitglobal` it can only decline a split. The kuna const-return peelers that
also call `node_split` (`returndup`, `earlyreturn`, `switchreturn`) do not
consult the limit; their own in-edge caps bound them. Settable `retsplitlimit`,
shipped default **1048576**: the costliest return split in ordinary code is a
few thousand cloned ops, so the default declines only in functions like
serde-hkx's two largest (the one above, and a 174 KB one whose 3,665-op return
block is reached by 673 gotos).

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
in `return` (that is `gotoreduce`'s case) and anything whose
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

The hoist frees tree nodes: each dropped else-arm copy, and any arm list that
is left with one component and unwrapped to it. A goto, switch case or
multi-goto edge elsewhere in the tree can still name one of those nodes. A
goto from one arm into the start of the other names that whole arm. Before a
node is freed, every branch that names it is pointed at the node that took its
place (`decompiler/crates/kuna-decomp/src/substrate/block.rs
(kuna_free_replaced)`). An unwrapped arm hands over to its sole component,
which starts at the same leaf, so the goto keeps its label and lands on the
same statement. A dropped copy hands over to the copy that was kept. Before
this, the goto kept the freed node's id and printing its label panicked.

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
function. In an unoptimized function the default `auto` value replaces the
guess with what the binary records (see *iteregion auto* below).

### iteboolean — short-circuit `0`/`1` re-rolling (decbench)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteboolean.rs
(ActionIteBoolean)`. *Pattern:* the boolean **materialization** diamond a source
`x = a && b;` compiles to at `-O0` — a 3-component `BlockIf` with no goto whose
two arms are each a single `COPY` of the constants `1` and `0` (in either order)
to the same integral storage, and whose condition component is either a clean
`CBRANCH` leaf or the folded short-circuit `BlockCondition` chain. *Transform:*
**print-only** — set the `kuna_iteboolean` addl-flag on the condition's terminal
`CBRANCH`; the S9 printer (via `kuna_iteboolean.rs (match_ite_boolean)`)
re-derives the match and emits the single statement `v = ( c );` — or
`v = !( c );` when the `0` arm is the true arm — in place of the `if/else`.

*Why it is a structuring decision and not a P3 one.* The rule that normally
collapses this is P3 `RuleConditionalMove`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_7.rs
(RuleConditionalMove)`), which folds a `MULTIEQUAL(0,1)` into `zext(cond)`. It
fires on the one-test shape, but requires each MULTIEQUAL input block to be the
CBRANCH root itself or a **single-predecessor** pass-through — upstream Ghidra's
`if (inblock->sizeIn() != 1) return 0` (`ruleaction.cc:9427`), ported verbatim. A
short-circuit `&&`/`||` chain gives the constant arm two or more predecessors, so
it bails, and the bail is *right*: no single dominating CBRANCH boolean stands
for the whole chain, and hoisting a guarded operand out of its branch (reading
`p[1]` when `p` may be null) would evaluate it unconditionally. The re-roll is
only expressible after structuring, where the chain has become one
`BlockCondition` that C's own `&&`/`||` renders with short-circuit semantics —
so the condition is emitted by the *same* `ONLY_BRANCH` renderer that produced
the `if (...)` header, and evaluation order, short-circuiting and any
comma-expression side effects riding the condition are preserved verbatim.

*Failure:* declines a labelled or multi-statement arm, a computed (non-constant)
arm, two arms writing different storage or the same constant, a pointer/float/
aggregate destination (which would lose the constant's own rendering), an `if`
carrying its own goto, and a condition component that is anything other than a
`CBRANCH`-terminated leaf or a `BlockCondition` tree of such leaves (capped at
`kuna_iteboolean.rs (MAX_CONDITION_DEPTH)` = 16). Like `iteregion` this is a
**runtime choice** — a source that really wrote `if (c) x = 1; else x = 0;`
compiles to the same bytes — flipped on by DIV-51.

### itecondlist — matching a diamond whose condition was concatenated (decbench)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_itecondlist.rs
(cond_list_tail)`. Not a pass of its own: a one-predicate widening of the two
matchers above, shared by both because both bottom out in the same leaf helper.

*The gap.* `kuna_iteregion.rs (leaf_bblock)` descends a `BlockList` of **one**
component but returns `None` for a list of two or more, and both matchers reach
the condition component through it. Meanwhile the collapse structurer
concatenates a just-collapsed predecessor together with the following condition
block into a two-component `BlockList`, and *that list* becomes component 0 of
the next `BlockIf`. A run of structurally identical diamonds therefore alternates
between the bare-leaf shape and the concatenated shape, and only the bare-leaf
ones are re-rolled: over a chain of N diamonds kuna recovers exactly `ceil(N/2)`
(measured for N = 1..8: 1 1 2 2 3 3 4 4). The tell is that *which* diamonds are
missed depends on the **neighbouring** structure rather than on the diamond
itself — an immediately preceding structured `if` suppresses the very next one.
Three source-identical `x = a||b||c||d;` call arguments in bash
`shell_initialize` came out as one re-rolled boolean assignment and two explicit
diamonds.

*Transform.* In the **condition position only**, descend a `BlockList` to its
last component before applying the usual leaf test — `iteregion`'s
`cond_cbranch`, and `iteboolean`'s `BlockCondition` gate (which is where the
descent must sit: `cond_terminal_cbranch`'s leaf arm is only ever reached for the
individual clauses of an already-identified chain). The arms keep the strict
single-statement `leaf_bblock` test, the labelled-goto-target guard still applies
to the descended leaf, and the match struct keeps the **outer** list as its
`cond_block` so the printer still emits the leading components.

*Why the tail is the right component, and why it is sound.* The rewrite is
print-only and the printer already renders this exact shape.
`PrintC::emit_block_if` emits the condition component twice — once under
`NO_BRANCH` (its statements) and once under `ONLY_BRANCH` (its branch condition)
— and `PrintC::emit_block_ls` emits every component in order under `NO_BRANCH`
but **only the last** under `ONLY_BRANCH`. So the leading components already
print exactly once, as ordinary statements ahead of the `if` header, and
`emit_block_if_ite`/`emit_block_if_bool` perform the same two emissions on the
same block. Folding only the tail therefore cannot disturb, duplicate or drop
anything that precedes it: it is the tolerance `cond_cbranch` already documents
for leading statements *inside* one basic block, spelled across a block
boundary. The descent is deliberately restricted to `BlockList` and excludes
`BlockGraph`, whose emitter (`PrintC::emit_block_graph`) ignores `ONLY_BRANCH`
and emits every component — descending one would make the emitters print the
whole graph twice.

*Failure:* everything the two matchers already decline, unchanged. It is a
strictly narrower match than `iteregion`/`iteboolean` themselves and runs
whenever they do (DIV-56), so their options are the only ones that reach it.

### iteregion auto — the conditional values an unoptimized function was written with

`iteregion` folds a diamond by its shape, and in optimized code the shape is
all there is: `x = c ? a : b;` and `if (c) x = a; else x = b;` compile to the
same branch and the same merge. Unoptimized code keeps them apart. An
unoptimizing compiler finishes every statement with a store to the variable it
assigns and evaluates an expression in scratch registers, so a conditional
value loads a register in each arm and is stored (or passed, or tested) once
after the join, while the `if`/`else` statement stores in each arm. After the
stack is heritaged both are one phi over the arms, and the merge decides the
printed shape: two inputs that interfere get a copy in each arm and print as
the diamond, an input that shares storage with the result leaves an `if`
without an `else` (`if (c) b = a;` for `x = c ? a : b;`).

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_o0select.rs (record)`
reads the raw graph at the first heritage pass of a function the detector of
chapter 03 (`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_unoptimized.rs
(is_unoptimized)`) calls unoptimized, when registers are in SSA form and the
stack is not. For every block that ends in a CBRANCH whose two successors each
have one successor, the same join:

- if each arm is entered only from the branch and the join holds a phi of a
  temporary register whose inputs are defined in the two arms and whose value
  is read, the branch is a *conditional value*, whatever else the arms do
  (`c ? i++ : j` also stores). A temporary is a register the calling
  convention does not preserve, or a preserved one whose only read is a copy:
  the compiler parks an argument there to carry it across a call before the
  one it belongs to, while a `register` variable in a preserved register is
  read wherever the source names it. A read is a store, a computation or a
  test of the value; a copy into another register that is then read; and a
  call, when the arms do nothing but set that register (an argument evaluated
  in place) or a copy moved the value into the argument register, because an
  unprototyped call also reads whatever a statement or an earlier call left in
  a scratch register. Only the low part of a
  partial-register piece carries the value, since the pieces heritage builds
  around a narrow write would otherwise make every overwritten register look
  read. A phi read only by the RETURN is a returned value and is left to
  `returndup auto` (`return c ? a : b;` compiles like two returns);
- otherwise, if both arms write memory (a STORE, or a direct write to a frame
  or global address; the return address a call instruction pushes does not
  count), the branch is a *statement*.

Each decided branch is recorded by the address of its CBRANCH, which every
later copy of the branch keeps. `kuna_iteregion.rs (match_ite_assignment)`
asks the record for the CBRANCH of the `if` it matches: a recorded statement
never folds, and `kuna_iteboolean.rs (match_ite_boolean)` declines it too; a
recorded conditional value folds with computed arms (what `iteexpr` does on
request), with an arm that is a call whose result is the value, with arms that
write the same high variable through different storage, and as an `if`
without an `else`, which prints `v = c ? A : v;`. A conditional evaluates only
the arm it selects, as the branch did, so the fold moves each arm's
computation into the expression unchanged. Unrecorded branches, and every
branch of an optimized function, keep the shape rule; `option iteregion on`
restores it everywhere.

clang does not separate the two forms: at `-O0` it keeps any value that
crosses a branch in a stack slot, so a conditional value is stored in each arm
like the statement. The record is not read in an image the container says
clang built (`decompiler/crates/kuna-analysis/src/loader/kuna_spillselects.rs
(built_by_clang)`: a `clang version` line in an ELF `.comment`, which the C
runtime's objects fill with a GCC line in every image, or any Mach-O image).

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

### gotonext — jumps to where control goes anyway (Ghidra `gotoPrints`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_gotonext.rs
(goto_rewrites)`, read by the C printer
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs (PrintC::emit_function_body)`).
Upstream prints a `BlockGoto`'s jump only when `BlockGoto::gotoPrints`
holds: the jump's target is not the block that `nextFlowAfter` says runs
next. kuna's printer printed every such jump, so a switch case falling into
the next case, and an `if` arm whose last statement jumps to the statement
after the `if`, printed `goto label_X;` directly above `label_X:`. Passes that
edit the structured tree after `scopeBreak` also leave jumps to the statement
after the innermost loop or switch, which is what `break;` does.

*Transform:* the census walks the final structured tree once per function,
at the top of body emission, so it sees the tree after every pass that edits
it. It uses `BlockGraph::next_printed_flow_after`, which is `nextFlowAfter`
with one addition: a switch case whose body prints no `break;` (it is not
an exit of the switch) falls into the next case even when the body is not a
`BlockGoto`. Then:

- a `BlockGoto` whose target's front leaf is the next flow prints no jump;
- a `BlockGoto`, `if (c) goto` or bare-goto switch arm of type
  `f_goto_goto` whose target's front leaf is the next flow after the
  innermost loop or switch whose body holds it prints `break;` (only while
  `loopbreak_recovery` is on, the option that owns `scopeBreak`);
- a label prints only if a jump that still prints `goto` reaches it. The
  label carriers are the ones `mark_unstructured` marks, and a
  `BlockMultiGoto` edge always keeps its label, so a printed `goto` never
  loses its label.

The census only changes how a jump is spelled, never where control goes:
each rewrite needs the target to be exactly the block the C text reaches
next. It applies to the C switch form only, because a Rust `match` arm does
not fall through and a Rust `break` leaves the loop, not the match.

*Case order:* `BlockSwitch::finalizePrinting` sorts the cases by label and
places each fall-thru chain (a case that is a `BlockGoto` to another case)
directly before the case it jumps to. `grab_case_basic` reads the chained
case's switch out-index from sblocks edges. A case wrapped in its own
`BlockGoto` has lost that in-edge, and the remaining sblocks indices can be
stale, so a chain was missed or pointed at the case itself; the sort then
printed that case first under label 0. With the option on,
`Funcdata::chain_fallthru_cases` re-derives each chain by matching the jump's
target leaf against the case blocks before the sort, and the jump between
the two cases is then a fall-through the census leaves out. Off restores
both the printing of every jump and the recorded chains.

### switchmerge — one switch for a search tree of jump tables

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_switchmerge.rs
(switch_plans)`, read by the C printer
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs (PrintC::emit_block_switch_c)`)
and by the goto census. A compiler lowers a sparse source `switch` into a
search tree of jump tables on one selector: a table's range guard sends the
values it does not cover to the next table, and the table's holes go to the
source's `default:` code. The structurer gives each table its own
`BlockSwitch`, nested in the `default:` arm of the one before it, and prints
each hole as a `case N:` of an arm that only jumps to the code `default:`
runs.

*Transform:* at the top of body emission, for each C switch on the final
structured tree, outer switches first, the plan lists the arms to print:

- When the `default:` arm starts with a `BlockSwitch` on the same selector
  (the same varnode or the same HighVariable), that switch's arms take the
  place of the `default:` arm. The inner switch must print nothing before its
  `switch` (only implied ops in its header block), carry no label, and list no
  value the merged switch already lists. If the inner switch is the whole arm,
  its own `default:` becomes the merged one; it
  must have no jump-table entry that goes to its exit, and the arm must leave
  the switch or not leave at all, so a `break;` in it still goes to the same
  place. If the inner switch is the first block of a list (also inside a
  `BlockGoto`), it must have no `default:` arm, no arm that leaves it and no
  `break;` that leaves it; its unlisted values reach the rest of the list,
  which stays the `default:` arm and prints from the next block.
- A switch that has a jump-table entry going straight to its exit is not
  merged into: that value prints no label, and in the merged switch it would
  take the `default:` arm.
- Then, when `default:` is code, an arm that is only a `goto` to the first
  block of that code prints no labels. When `default:` is itself such a
  `goto` and another arm holds the code, the `default:` arm and the jumps are
  left out and the arm holding the code prints `default:` in place of its
  labels.
- An arm that is only a `goto` or a `break;` also prints no labels when every
  value it lists reaches its target through the `default:` code before that
  code prints a statement. For each value the walk starts at the basic block
  the `default:` arm prints first and decides each branch with the value: a
  `CBRANCH` whose condition the printer inlines from the selector and
  constants (`==`, `!=`, ranges, masks, casts; evaluated with the ops'
  `OpBehavior`), or a `BRANCHIND` on the selector, whose out-edge is the
  entry labelled with the value or else the default edge (none when the
  table has an unrecovered label or a label on two edges). It stops with no
  drop at a block that prints a statement other than its branch, a block it
  has seen, a back edge (which could reach a `switch` whose printed labels
  no longer match its table), a test of anything else, or after 256 blocks. A varnode is the
  selector when it is the switch's selector or an explicit varnode of the
  same HighVariable: the walk passes no printed statement, so no assignment
  to that variable runs between the `switch` and the test. Holes of a table
  whose `default:` is an `==` chain or a range test that leads to the next
  table go away this way.
- After that, while the `default:` arm (plain code, not a jump) starts with
  an `if (v == c) S` that has no else, whose condition block prints only its
  branch and whose test is `INT_EQUAL`/`INT_NOTEQUAL` of the selector and a
  constant `c` that no arm lists, with the then-branch reached exactly for
  `c`, the plan prints `case c: S` right before `default:` and `default:`
  prints the rest of its list. `S` runs for `c` and then falls into the rest,
  as after the `if`. The arm before `default:` must not fall into it, and a
  second `case` follows only an `S` whose printed code cannot run past its
  end (the rule below). When
  nothing is left of a last `default:` arm, it is not printed.
- Last, innermost first, an `if` with an else whose two branches each print
  as a switch on the same selector (a switch not merged elsewhere, or an
  `if` this rule already prints as one) prints as one switch: the first
  branch's arms, then the second's. Every label of each branch must route
  from the condition block to that branch under the value walk above, the
  label sets must be disjoint with the same size and sign, no owner switch
  may have a table entry to its exit, every header must print nothing, and
  the condition block and both branches must carry no label. The defaults
  must do the same: both absent or `break;`, or the same jump; the second's
  is left out. The first branch's last arm and the arm before the second
  branch's default must not fall through. The second switch and the
  switches merged into it are hosted by the first, so `break;` still leaves
  to the block after the `if`.

An arm moves or is left out only when the arm printed before it cannot run
into it. That is read from the printed code, not the case's structured edges:
the arm before must end in a `break;`, in a jump that prints, or in a leaf
whose basic block has no successor. A jump to the arm after it in the
switch's own order prints as nothing, and a tail the cross-jump reverter
duplicated into the arm carries no edges; both run into the next arm. The
census reads the plan: a left-out jump keeps no label alive, and
the target of a `break;` inside a merged switch is the block after the switch
that is printed. The plan only changes which arm a value is printed under,
never which code runs for it. It applies to the C switch form only.

### looptest — the test at the top of a rotated loop

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_looptest.rs
(loop_tests)`, read by the C printer
(`decompiler/crates/kuna-decomp/src/p9_emit/printc.rs (PrintC::emit_block_do_while_c)`,
`PrintC::emit_block_if`). An optimizing compiler rotates a loop: `while (c) body`
becomes `if (c) do body while (c);` (gcc's loop header copying), and the `if`
disappears when the compiler proves `c` on entry. The structurer recovers the
`BlockDoWhile` that is in the binary, so a loop the source tests at the top
prints as a guarded do-while. Under `option looptest proven` (the default) the
plan, made at the top of body emission like `switchmerge`'s, marks the
do-while loops that print as `while (c) { body }` and the guard `if` nodes that
print without their test. The p-code is not touched.

*Top test.* The latch is the last leaf of the loop body and ends in the
`CBRANCH`; the header must have one entry edge and one back edge (from the
latch), and no goto may target it. Moving the test to the top is the same
program when the condition reads, at the top of iteration k+1, what it read at
the bottom of iteration k. So every leaf of the printed condition (following
implied ops, which must call nothing and store nothing) that is defined inside
the loop must be the latch input of a header MULTIEQUAL in the MULTIEQUAL's own
HighVariable, or a varnode that input is a COPY of. A COPY source in another
HighVariable of the same type prints as the MULTIEQUAL's variable while the
condition is printed (`do { q = *p; p = q; } while (q)` prints `while (p)`).
An implied expression of the condition that computes a latch input again --
the same pure ops over the same varnodes, reading no memory, such as `&v2[1]`
beside `v1 = &v2[1]` -- has that input's value, so it prints as the carried
variable while the condition is printed, and the entry test reads that
variable's entry value in its place. A condition that reads any other body
value (`do c = getc(); while (c != '\n')`) keeps the bottom test, and so does a
loop whose carried variable has an instance in the stack frame in a function
the unoptimized-code detector
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_unoptimized.rs`) flags: there
every counter lives in the frame and a bottom test is the source's own. In an
optimized function a counter in the frame is one whose address is taken, and
its loop is tested like any other.

A loop whose body prints nothing -- one block whose only printed op is its
`CBRANCH` -- is the condition's own loop: `do {} while (c);` and
`while (c) {}` evaluate `c` the same number of times for every input, calls in
`c` included, so it prints top-tested whatever `c` reads and without an entry
proof (`while (poll_busy()) {}`).

*Entry test.* The first top test must agree with the binary, which runs the
body once before testing. The entry test is the condition with each carried
value replaced by its entry value (the MULTIEQUAL input from the entry edge),
built as an expression over the varnodes' defining ops and put in a normal
form: COPY and CAST are transparent, constants fold, commutative operands are
ordered, comparisons become `==`, unsigned `<` or signed `<` with a constant
on the right and a negation flag (`0 < n` and `n != 0` meet, unsigned
`x < 1` is `x == 0`), `x == x + n` is `n == 0`, `p == &p[n]` is `n == 0`
(C pointer arithmetic), and a zero test sees through a zero or sign extension
and a multiply or shift by a constant that cannot wrap the extended value.
Comparisons and boolean ops of constants fold, and two sums of the same
non-constant terms (through `+`, `-` and `PTRADD`) are equal exactly when their
constants are, modulo the word: `p != &p->v[0x1d]` and `sp - 0x168 !=
sp - 0x108` fold to true. Two LOADs are equal when they are the same op, or
when their addresses are equal and both read the memory the loop header sees:
a LOAD the printed condition evaluates, one in pure blocks between the guard
and the loop, or one met walking back from the guard's branch -- through the
guard block and on into each block that is the only way into the one after
it -- before any call or side effect, where a STORE in between is passed when
its bytes and the LOAD's are disjoint offsets of the same base value (`p->nalloc
= 0` between a load of `p->num` and the loop). The loop prints top-tested when the
entry test folds to true (`i = 0; do … while (i < 16)`), when it equals the
condition of an edge that dominates the header and whose target has that edge
as its only way in (no LOADs in this case), or when it equals the condition of
a guard. Implication is not enough: with a signed count, `n > 0` and
`i != n` disagree for negative `n`, so the latch is not printed as it stands.

*An ordering test.* A rotated counted loop usually tests `i != n` at the
bottom behind a guard `n > 0`. When none of the proofs above holds, the plan
tries the latch's `==`/`!=` comparison as an ordering test. One side must be a
carried value `x` whose latch value is the MULTIEQUAL output plus a constant
`s` (`INT_ADD`, `INT_SUB`, or `PTRADD` by constant index and element size,
through COPYs), the other an invariant bound `b` (defined before the loop, or
an implied pure expression of such values with no LOAD). If `x` starts strictly
on the near side of `b` (`x_entry < b` for `s > 0`, `b < x_entry` for
`s < 0`) and `b - x_entry` is an exact multiple of `s`, then `x` takes the
values `x_entry, x_entry + s, ..., b` and stops there: `x != b` and `x < b`
(`x > b`) agree on every value the top test sees, and no step can wrap, since
each value lies between `x_entry` and `b`. A step of 1 needs no more; otherwise
the bound must be `&x_entry[n]` with `s` the element size (C pointer
arithmetic, also when `x_entry` is itself `&p[i]`), or `x_entry + n * c` /
`x_entry + (n << j)` with `c` or `2^j` a multiple of a power-of-two `s` (the
difference is then a multiple of `s` modulo the word, hence exactly). The entry test of the ordering test, `x_entry < b`
compared with the same signedness, must then equal the guard (which is dropped
under the rules below) or a dominating condition, through the same normal
form, which also orders pointers by their index (`p < &p[n]` is `0 < n`, and
`&p[i]` against `&p[j]` over one element size is `i` against `j`, C pointer
arithmetic; the same rule lets `&p[16] != &p[32]` fold to true as a constant
entry test) and compares an extended value with a constant at its own width.
The signedness is the one C will use: both operands must print with types of
the same class, signed integers, unsigned integers or pointers, read
from each high's declaration (the `signedness` rounding included) or, for an
expression, its own type; a constant bound must lie in `[0, 2^31)`. The
printer then spells that one comparison op with the ordering token (`<`, `>`,
or `>=`/`<=` for an `==` exit test, which its negation turns into the loop
test) while it prints the top test. `wrapend`-style bounds such as
`a + (n - 1) + 1` in 32 bits are far past `a` at `n == 0`, so their entry test
is not the guard's and the loop stays a do-while.

*The guard.* Walking back from the header's entry edge over straight-line
blocks reaches the guard, a block ending in a `CBRANCH` with one edge toward
the loop. When its condition (with the edge's polarity) equals the entry test,
the guard is left out if dropping it keeps the program: its printed condition
calls nothing, and the blocks between it and the loop (`pre`) hold no call,
STORE or LOAD (a LOAD the guard protects must not run when the loop is
skipped). Without the guard, the skip path runs `pre`, the top test (false,
since it is the guard's), and the loop's exit path, instead of what it ran
before; that is the same program when both reach the same place with the same
effects and the same values, and the loop then ran zero times, so every carried
variable still holds its entry value. The check evaluates the exit side that
way: each value it reads from the loop must be a carried variable in its own
HighVariable (a varnode the body computes and nothing carries has no value
after zero iterations), read as its entry value, and on the skip side a
failed `a != k` guard lets `a` read as `k`. The skip and exit paths qualify in
these shapes. They are the same block (the skip edge reaches the loop's exit).
Both end in a return: two bare returns with equal values, the exit's read at
zero trips (`if (b > 1) { x = a; ... return x; } return a;`), or two blocks
whose calls, stores and printed statements pair up in order with the same
callee, the same storage for any write to memory and equal operands, and equal
return values (`return report(head, n)` against `return report(NULL, 0)`; a
constant compares equal to one of another width when both extensions agree).
Or they meet at a join `J`: the skip block `S` (if any) flows into `J`, entered
only from the guard, and has no call, store or write to memory, so dropping it
loses nothing; the exit block `E` (if any) flows into `J`, entered only from
the latch, and has no call, store, LOAD or read of memory, so running it on the
skip path adds nothing; and every MULTIEQUAL in `J` receives equal values over
the skip edge and over the exit edge at zero trips (`if (b < end) { n = 0; do
… } else n = 0;`). A join is also how a write in `pre` to a variable that lives
on is checked when the skip edge reaches the exit block itself: a value the
code after the loop reads on the skip path reaches it through a MULTIEQUAL
there, since variable merging never lets `pre` overwrite a value that is live
across it. The tree shapes, checked on the flattened lists around the guard
`if`: `if (G) { pre; do … while (f); E }` and `if (G) { pre; do … while (f);
E } else { S }` (either arm) print the condition's statements and the loop's
arm; `if (G) { pre; do … while (f); return K; } return K';` prints the
condition's statements and the body (the trailing return is hidden, and a void
function's elided trailing return moves to the loop's); and `if (!G) return K';
pre; do … while (f); return K;` prints only the condition's statements. A
guard that is an `else` arm, or whose shape is anything else, stays, and then
the loop stays a do-while too: a source author does not write a test twice.

`option looptest binary` prints every loop where the binary tests it.
`assume` also prints the test at the top for a loop that passes the top-test
rule but has no entry proof (including one whose guard stays); it is wrong
when the source was a do-while whose first test can fail. The plan applies to
the C do-while form only.

### returndup — the whole-block duplicator (angr `ReturnDuplicatorHigh`)

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_returndup.rs
(ActionReturnDup)` is the whole-block gotoless return duplicator: every
shared bare-epilogue RETURN block passing the splittable filter *and* whose
whole return value resolves to a constant (`funcdata_block.rs
(Funcdata::returndup_is_const_ret)` — a literal, COPY chain, or
phi-of-constants; never a free variable) is split into per-predecessor
returns, capped at `kuna_returndup.rs (MAX_RETURNDUP_INEDGES)` = 16 /
`(MAX_RETURNDUP_SPLITS)` = 64. It exists to break `rule_block_or`'s
comma-folded guard merges (`if ((A||B) && (v=f(..), C)) …` with one trailing
return) back into the source's early-return guards.

**History — shipped, reverted, and re-shipped.** DIV-18 first made it a
default and it was **reverted the same cycle**: the decbench re-run measured
the then-*unselective* version (which also split `return <variable>`
epilogues) firing 21,768 times across 550 binaries and regressing the
aggregate GED-perfect count by ~976. On a majority of shared epilogues the
*source* used the merged form, so broad duplication was churn, not recovery.
The const-return gate (`returndup_is_const_ret`) was added afterwards and
drops the over-eager variable-return splits, which is what the −976 figure
had been measuring. Re-ablated with the gate in place over 536 decbench
slices / 52,862 functions — the same binary in both arms, the 46,005
functions the pass never touches scoring identically — the selective pass is
**+417 GED-perfect and −7,756 aggregate GED** (608 perfects created against
191 destroyed), net-positive in every one of nine partitions tested (split
count, void versus value return, distinct return values, return count,
presence of a `switch`, all-returns-identical, body size, project, opt
level). **DIV-54 therefore makes it a default again, superseding the DIV-18
revert.** One subpopulation is net-negative — a split that de-structures a
loop, so a `for` header becomes `while( true )` plus an early return (321
firings, −257 GED). A gate declining a shared RETURN whose predecessor sits
on a CFG cycle was built and scored at **+2 GED-perfect / −209 aggregate
GED** and was *not* kept: the predicate is far broader than the cell it
targets and eats into the profitable loop firings that pay for it.

The pass stays a per-run runtime choice (`option returndup off` restores the
merged upstream shape byte for byte) precisely because the merged and
split forms compile to the *same* object code, so nothing in the binary
distinguishes them. Its per-edge narrowings — `earlyreturn` (DIV-23) and
`switchreturn` (DIV-25) — remain separate options: they reach the mixed
const/variable diamonds this whole-block gate rejects outright.

### returndup calls — one call per path for a cross-jumped call tail

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_returndupcalls.rs
(ActionReturnDupCalls)` runs in the `returnsplit` group right after
`returndup`, only at the value `option returndup calls` (not a default). gcc
`-Os` cross-jumps the tails of paths that end in the same call: each path sets
the arguments and jumps to one `call f; ...; ret` block, so the call reads
MULTIEQUALs and prints as `if (c) { a = b; k = 2; } return f(a, k);`. The pass
splits such a block once per predecessor with
`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::node_split_calls)`, the `node_split` clone with CALL and INDIRECT
cloning enabled: each cloned call gets its own copy of the original call spec
(`FuncCallSpecs::clone_for_op`, a fresh fspec handle), and a cloned INDIRECT's
iop input names the cloned call. Each path then prints its own call:
`if (c) return f(b, 2); return f(a, 1);`.

A block qualifies when it has no successors, two to four distinct
predecessors, at most 16 ops, exactly one direct CALL, an argument of that call
computed (within the block) from a MULTIEQUAL, and no INDIRECT that still
creates the call's output (prototype recovery has not settled the output yet;
a later fullloop pass retries). Each predecessor runs exactly one copy, so the
split preserves behaviour. It is a value of `returndup` rather than its
default because the source often wrote the merged form as a ternary argument
(`lua_pushstring(L, isint ? "integer" : "float")`), and the binary cannot tell
the two apart.

### orchain — the split that must not happen (decbench)

`returndup` is right about a *guard clause* and wrong about the *operand chain
of a short-circuit expression*, and the two are the same CFG up to one
question: do the predecessors of the shared epilogue still need to share it?
`rule_block_or` (§8.1) fuses `bl -> {orblock, clauseblock}` with
`orblock -> {clauseblock, X}`, and the fold's entire precondition is that
**shared out-target**. Give each predecessor a private return and there is
nothing left to match — and because `ActionReturnDup` runs in fullloop's
`returnsplit` group while `collapse_conditions` runs later, the fold is lost
for good. One source `return a || (b && (c || d));` then prints as four
constant-return guards plus a trailing return (`iproute2 ip::sci_complete`,
GED 21 against 0 for the merged form), and `iteboolean` never sees the 0/1
select diamond either.

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_orchain.rs
(shortcircuit_shared_targets)` is the gate for exactly that shape, and it is a
gate on the pass above rather than a pass of its own. It replays
`collapse_conditions` read-only on the **bblocks** CFG before the first split,
mirroring `ruleBlockOr`'s own admission test rather than approximating it: a
condition node starts as one two-out basic block, and two nodes fuse when the
sibling has exactly one in-edge, has two out-edges, is not a switch block, is
not `BlockBasic::isComplex` (the same `funcdata_block.rs
(Funcdata::bb_is_complex)` verdict the rule reads), and shares one out-target
with the head, whose other target is not the head. Each fuse records the
shared target and rewrites the head's out-targets, so a chain collapses
inside-out as the real fixpoint does. Three upstream tests are omitted because
they cannot hold at this point in the schedule: no goto has been elected yet
(`select_goto` runs after `collapse_conditions`), and a back edge into a
single-in-edge block would make that block unreachable from the entry. The
predicate then also protects a block joining two or more **single-exit arms
that are themselves recorded fold targets** — before the constants are
propagated into the epilogue phi, a chain's arms are one bare `v = K` block
each and the RETURN sits one edge further on, which is the graph the *first*
`returndup` invocation of the fullloop sees. `returndup_apply` declines the
whole function's splits when any of its own candidates is a recorded target,
which is the counterfactual the corpus measurement priced.

**Measurement.** DIV-54 flipped `returndup` on evidence from O2 and
O2-noinline only. Extended to all three levels (795 slices, 85,195 scored
functions) the pass is +640 GED-perfect corpus-wide but **-192 at -O0**, and
one sub-shape — the split that costs a short-circuit fold — carries -665 of
that O0 harm while costing +15 / +20 of the +832 the optimized levels gain.
The gate implemented here reproduces the `returndup off` body byte for byte on
94% of its firings and recognises 2,933 of the 2,951 functions the post-hoc
"a short-circuit was lost" signal marks at O0, at 126 further firings the
signal does not mark. Swept on the same corpus it is **+611 GED-perfect at O0
for -13 at O2 and -15 at O2-noinline — +583 net and -967 aggregate GED**, 781
functions moved to an exact structural match against 198 moved off, which is
92% of the post-hoc ceiling at a *smaller* optimized-level cost than the
ceiling itself. **DIV-69 makes it a default** (rather than a preset member:
`returndup` is a shipped default in every mode, and 45% of the corpus sits over
the 500 KiB `auto` threshold where a preset-only gate would never run). It is
part of `returndup` itself: the merged and split forms compile to the same
object code, and where the source did write the cascade the gate is wrong
(`coreutils factor`, GED 0 → 12), which `option returndup off` leaves merged as
upstream does.
`docs/decbench/returndup-o0-measurement.md` and
`docs/decbench/returndup-regression-triage.md` hold the corpus tables.

### rettriangle — the join of a one-block `if`

A shared constant-return block whose two predecessors are the arms of a
one-block `if` is no guard either. The condition `C` goes straight to the
return `R` or through one block `T` that is entered only from `C` and runs
into `R`; splitting `R` prints `if (c) f(); return 0;` as
`if (!c) return 0; f(); return 0;`, with one statement more and the return
written twice, where IDA and Ghidra print the `if`. The common source of the
shared block is a stack-protected function: once `stackguard` strips the canary
check, every exit reaches one bare `return 0`. With `option rettriangle on`,
`Funcdata::returndup_apply` drops such a block from its candidates
(`decompiler/crates/kuna-decomp/src/p8_structure/kuna_rettriangle.rs
(is_triangle_join)`): exactly two predecessors, one a two-way block whose other
successor is the second predecessor, which has that block as its only
predecessor and `R` as its only successor. Every other candidate of the
function still splits, so the option can only make `returndup` fire less, and
it is inert with `returndup off`.

The same shape is also a source guard clause, `if (!ok) return 0; ...;
return 0;`, which the option prints as `if (ok) { ... } return 0;`, so the binary
cannot tell the two apart. Over a decbench sample (12 projects, 126 slices,
`scripts.decbench.optsweep`) it moves O0 GED 31,382 → 31,219 and GED-perfect
+56 (64 functions to a perfect match, 8 off, McNemar z = +6.6), and O2 GED
69,032 → 69,092 with GED-perfect +1; most of the O2 cost is in tar's pax
decoders, whose source is the one-block `if` the option prints. Because the
O2 aggregate rises, the option ships off and is a member of the `aggressive`
preset, which `--mode auto` selects under 500 KiB.
`tests/stages/kuna-rettriangle.xml` decompiles gcc -O0 functions with the
default and with the option on, and
`kuna-cli/tests/rettriangle_round_trip.rs` compiles the printed C of gcc
-O0/-O2 and clang -O0 builds beside the source and checks each function
returns and calls what the source does.

### returndup auto — the returns an unoptimized function was written with

`returndup`, `earlyreturn`, `rettriangle` and `tailform` choose where returns go
by rules, because in optimized code the binary does not record the source's
return statements. Unoptimized code does. An unoptimizing compiler gives a
function one epilogue and compiles every `return` into code of its own: the
value is put in the return register and control jumps to the epilogue or falls
into it. `if (c) { S; } return K;` therefore has one block that sets `K`, the
epilogue itself, reached from the condition and from `S`; `if (!c) return K;
S; return K;` has two, each ending its own path; and `v = K; if (c) { S;
v = K2; } return v;` loads the frame slot once in the epilogue. Simplification
erases the difference: the value-setting blocks become empty and are removed,
and every form ends as one RETURN block with a phi over its predecessors.

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_o0returns.rs (record)`
reads the raw graph at the first heritage pass, in a function the detector of
chapter 03 (`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_unoptimized.rs
(is_unoptimized)`) calls unoptimized and whose only RETURN is reached from two
to 64 edges. The join is the RETURN's block, or the stack-protector check in
front of it (a two-way block whose other successor is a dead end that makes a
call). Each in-edge of the join belongs to a statement:

- a predecessor that only computes a value on its way to the join (no store,
  call or decision) is one statement for every edge that enters it, so
  `if (a || b) return 1;` is one return reached twice;
- in a function that returns a value, every other predecessor ends in a return
  of its own, unless the join computes the returned register itself (the
  `mov eax, K` or frame load in front of `leave`), in which case all its
  in-edges share that one statement;
- in a void function `return;` is a `nop` (and a jump when it is not last),
  and kuna's flow gives a `nop` no p-code. A jump-only predecessor is a
  `return;` only when a branch into it lands on the `nop` in front of it; a
  bare jump is a `break` or `goto`. Every other path is told apart by the
  instruction it lands on: the target of a branch, or for a fall-through the
  first branch target between its last instruction and the join, and a run of
  landings with no p-code between them counts as the last one. Paths that land
  on the same address share a statement.

The record is carried by the edges themselves: bits 16–30 of each edge label
hold a tag naming the statement (separately for the valued and the void
reading, since the return type is not known yet), set on the join's in-edges
and on the in-edges of value-only predecessors. When a block is removed its
in-edges are re-pointed at its successor with their labels
(`BlockGraph::remove_from_flow`), so the tags reach the RETURN block's in-edges
after simplification; edge flags are only ever cleared by mask. Which reading
applies, and whether the join computed the returned register, is decided when
`returndup` runs, against the recovered output storage.

At `returndup` time `kuna_o0returns.rs (split)` groups the RETURN block's
in-edges by tag and gives each group its own copy of the epilogue with
`node_split` (a group of several edges is moved onto its copy only when the
block has no phi), the largest group keeping the original; a group's value may
be a constant or a variable. A RETURN block still behind the stack-protector
join, reached by one untagged edge from it, is spliced into the join first
(`kuna_o0returns.rs (joined)`), and an epilogue may be split when it holds
arithmetic the simplifier hoisted out of the returns (`p + MULTIEQUAL(i, i +
2)`), as long as it reads no memory and makes no call (`kuna_o0returns.rs
(pure_epilogue)`). When every RETURN block of the function is one
recorded statement the function is *decided*, and the rule-based passes stand
down: `returndup`'s const gate, short-circuit gate and `rettriangle`, and the
`earlyreturn`/`switchreturn` peels, skip recorded blocks (`recorded`), and
`tailform` leaves the function's tails as the record placed them, except that
`if (c) { S; return; } return;` is re-spelled as the guard `if (!c) return; S;`
— void returns are elided, and without the re-spelling the recorded explicit
`return;` would vanish. A block whose in-edges lost their tags (a later pass
made a new edge) is not recorded and the rules apply to it as before.

Two source forms are the same code at `-O0` and the record picks one:
`return c ? a : b;` compiles exactly like `if (c) return a; return b;` and is
printed as the second (merging non-constant returns instead was measured worse,
195 functions lost against 9 gained), and a `switch` whose `default: break;`
is the last statement emits the same `nop` as a `return;`. Optimized functions
are not recorded (the next section covers what `auto` does there); `option
returndup on` restores the rules everywhere, and `option returndup record` keeps
the record but not the optimized-function shapes.

### returndup auto in optimized functions — one guard per `||`, one return per arm

An optimizing compiler emits the same code for `if (a) return K; if (b) return
K;` and `if (a || b) return K;`, and for `if (c) v = f(); else v = g(); return
v;` and `if (c) return f(); return g();`. Which one structuring recovers is
decided by the return layout — whether the paths share a return block, and
whether the splitters above give each path its own — not by the author, and
measured against the sources it is the wrong one far more often than not: where
kuna printed one return fed by assignments in both arms of the last `if`, the
source wrote that form for 5 of 204 functions; where it printed adjacent guards
with the same return, the source had a `||` guard in 241 of 626 and separate
guards in 160. In functions the detector of chapter 03 does not call
unoptimized and that do not address their frame through a frame pointer (the
detector's own evidence, cached at the first heritage pass: small `-O0`
functions it misses keep one, optimizing compilers rarely do), `auto` re-spells
both
(`decompiler/crates/kuna-decomp/src/p8_structure/kuna_retshape.rs`).

**Guards.** `kuna_retshape.rs (ActionRetShape)` runs on the final tree right
before `tailform`. In every statement list (each `Ls`/`Graph` no enclosing list
flattens, with the condition prefixes of its `if`s hoisted the way `tailform`
hoists them, print-neutrally and only when a fold is decided) it folds a run of
adjacent guards `if (a) return K; if (b) return K;` into one `if (a || b)
return K;`: both arms must print only a bare return of the same value (void,
equal constants, one named variable, or the same expression varnode), the
second guard must print nothing in front of its test and carry no label, and
neither may be a guard the `looptest` plan drops. The folded condition is a new
`BlockCondition` over the two existing conditions, left-nested as
`rule_block_or` builds a chain, and the second return leaf leaves the tree. Each
leaf keeps its own `CBRANCH` polarity — it already printed the test that enters
its clause — so the `||` evaluates the same tests in the same order and runs the
same `return` on the same paths. Two adjacent tests that both call something
stay two guards when the return is void or a constant (authors check one call's
result per guard: across the corpus sources such joins were written as `||` 30%
of the time for void and 41% for constants, against 49-78% when at most one
test calls).

**Merged returns.** `kuna_retshape.rs (arm_returns)` adds candidates to
`returndup` (`Funcdata::returndup_apply`, before the short-circuit gate): a
RETURN block whose returned value is a phi of its in-edges, perhaps through
casts, extensions or a `SUBPIECE`, and which holds nothing else but the
return's own `glob = glob` guard copies (`value_tail`: no load, store, call,
branch or `INDIRECT`; at most 16 ops). The split is the same `node_split` the
constant duplication uses, so each arm runs its own copy of a value-only block
and returns the value it computed. On ARM the phi is often extended in the
return block (`sxth r0, r0` before `pop {pc}`), which the bare-epilogue filter
rejects; on x86 a stack-protector check joins the paths, and once
`kuna_stackguard` has stripped it the phi sits in a block that only runs into a
one-predecessor RETURN, which is spliced into it first (the same move as
`kuna_o0returns.rs (joined)`), after every other check has passed.

Which merges split is the judgement. Every in-edge must end an arm (one
out-edge), so a one-armed update `if (c) v += k; return v;` keeps its merge. An
arm entered from several places qualifies only when its entries join one `if`
(`if_join`: a two-way block and the one block it branches to, or two blocks
entered only from the same two-way block) or fold into one short-circuit chain
(`kuna_orchain.rs (folded_in_degree)`, the fold replay above, leaves it one
in-edge): any other arm, such as a cleanup block several failure paths and a
`switch` reach, becomes a terminal block with several entries once it returns
on its own, and the structurer needs a `goto` for it. The one exception to the
arm rule is a one-block `if` (`fresh_triangle`): `v = a; if (c) v = b; return
v;` splits when `b` does not read `a` (through copies), which is the source's
`if (!c) return a; return b;`, while the update keeps its merge.

Measured on the decbench corpus against `record` (main 068372109), `auto` makes
305 more O2-noinline functions structurally exact and 23 fewer (+204 on one
half of the projects, +78 on the other), and no function gains a `goto`; the
guard fold alone is +60 (63 / 3) and the merged-return split +222 (242 / 20).
The losses are sources that wrote the other form: separate guards each checking
a call's result, `status = f(); ... return status;`, a ternary, a one-armed
assignment. The two shapes never touch a function the detector calls
unoptimized or one with a frame pointer; at -O0 the functions both tests let
through (mostly firmware built with optimization) move 61 functions to an exact
match and 8 off (+5 and +48 on the two halves); without the frame-pointer test
two small openssh functions in nine binaries were lost. Each is a print-tree edit or a split of a value-only block, so both
print the same program.

### loopcontinue auto — the jump statements an unoptimized function was written with

The short-circuit fold (`try_block_or` in the region structurer, `rule_block_or`
in `CollapseStructure`) joins two tests whenever they share a successor, so a
loop body that opens with `if (c == NULL || c->type != 7) continue; if
(c->addr == NULL) continue; if (match(...)) { ... }` prints as one `&&` chain
around the body, and a `break;` test right after a loop test folds into the
loop condition. In optimized code nothing else is left to go by. Unoptimized
code keeps a marker: gcc `-O0` compiles a bare `continue;`, `break;` or `goto`
that is the whole arm of an `if` into a block of its own, so that a debugger
can stop on its line, while a structured `if` branches straight to its join.
The block is a `nop` the test branches to, followed by a jump to the
statement's target or falling into it (an `||` group sends every test to the
same `nop`); a `continue;` whose target is a `while` loop's test is instead
`jcc L; jmp T; L:`, a jump-only block the last test of the group falls into,
gcc sending the group's earlier tests straight to `T`. A jump at the end of a
non-empty block (`{ f(); continue; }`) and an empty statement leave no marker,
and a `continue;` that is the last statement of the body leaves no branch at
all. kuna's flow gives a `nop` no p-code, so the marker is a branch that lands
before the first operation of its destination block.

`decompiler/crates/kuna-decomp/src/p8_structure/kuna_o0jumps.rs (record)` reads
the raw graph at the first heritage pass of a function the detector of chapter
03 (`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_unoptimized.rs
(is_unoptimized)`) calls unoptimized. A statement is a jump-only block entered
on the `nop` in front of it, a run of `nop`s (at most eight bytes) a branch
lands on and that falls into its target (keyed by its address), or the
jump-only block of the `while` form together with the tests that jump straight
to its target and run only into the test before it. Every edge into a
jump-only statement belongs to it, and a fall-through that runs over a landing
in the gap before its block belongs to that landing (a void function's closing
brace is a `nop` both a jump and a fall-through reach). The record is carried
by the edges: bits 10–15 of each edge label hold a statement id (1 to 62, in
address order; a statement past that is not recorded), and
`BlockGraph::remove_from_flow` keeps an edge's label when it re-points the
edge past a removed block, so the ids reach the structurer after
simplification, and `build_copy_from` copies them into the structured graph.

The fold asks `kuna_o0jumps.rs (may_join)` before it joins two tests: two
edges into the same block join when they carry the same id or neither carries
one; otherwise they stay apart when that block is a loop latch (the block
holding a `for` loop's step or a `do` loop's test, a `continue;`) or a block a
loop exits to (a `break;`). Edges into a loop head join as before: the loop
rules need a head's in-edges folded, and the `while` form's forwarding makes
the record incomplete there. A `goto` the source wrote does not keep tests
apart either. The loop rules then see the source's tests one by one: a
`continue;` group prints nested, and a `break;` test stays a second exit that
`refine_loop_edges` virtualizes and `scopeBreak` prints as `break`.

`ActionReturnSplit` gives every goto into a RETURN block its own copy of the
block, a `break` included, which would turn the source's one `return` after a
loop into a second return inside it. In a function whose returns `returndup
auto` placed (`kuna_o0returns.rs (decided)`), `kuna_o0jumps.rs (keeps_return)`
leaves such an in-edge alone when it carries a statement id and its
structured goto leaves its innermost loop for the block after that loop.

After structuring, `kuna_o0jumps.rs (print_continues)` (run by
`ActionFinalStructure` right after `recover_continues`) prints the recorded
`continue;` statements: in a while-do or do-while whose `continue;` resumes at
a known leaf (`kuna_loopcontinue.rs (continue_points)`), the body's last `if
(c) { S }` before that leaf, whose condition reaches the leaf's block only
through edges that carry a statement id, becomes `if (!c) continue; S`, and
the rewrite repeats on `S`'s own last statement. Control reaches the same
place either way. In a function with a record `continue_points` also gives a
`for` loop the leaf that holds its iterate statement when that leaf prints
nothing else, so a jump to it prints as `continue;` too.

Keeping tests apart can leave the structurer a loop it cannot fold, and then
it falls back to gotos (two tests whose different `break`s both enter the next
loop's head, say). `ActionBlockStructure` therefore structures a recorded
function with the record and, when the fold refused a join and the result
prints a goto that is not a loop's `break` or an infinite loop's `continue`
(`kuna_o0jumps.rs (gotos)`), structures it again without the record, from the
same branch orientation (`orientation`, `restore_orientation`), keeping the
form with fewer gotos. Optimized functions are untouched, and `option
loopcontinue on` restores the shape rule everywhere.

### tailform — the form of a function-tail `if`

`if (c) { S } return K;` and `if (!c) return K; S; return K;` are the same
program at the end of a function, and the binary does not say which one the
author wrote. Structuring recovers whichever the compiler's return layout
implies: a short body ending in a sibling call gets its own exit (a tail jump)
and reads as a guard clause, a long body shares the epilogue and reads as a
wrapped block. Authors do the opposite — over the decbench void-tail sites the
source wraps 86% of one-line bodies and 29% of bodies over eight lines — so the
inherited form is wrong more often than chance.
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_tailform.rs (ActionTailForm)`
runs after `dedupitetail` and re-spells the tail. By default (`option tailform
model`) saved weights over the site's features choose the form, as described
below; under `option tailform on` or `<n>` the printed size of the conditional
part decides, with `<n>` lines as the threshold (4 for `on`). Both make the
same rewrites, shown here with the size rule:

- **wrap.** The last top-level `if (c) return K;` whose remaining statements
  print at most `<n>` lines becomes `if (!c) { ... }` followed by the final
  `return K;`. `K` must be the same on both returns: void, two constants of
  equal size and value, or one SSA varnode, so the value returned when `c`
  holds cannot change. A tail with a goto label, or in a valued function
  another `return`, is left alone.
- **guard.** A one-arm `if` right before the function's final bare return
  (void, or a constant `K`) whose body prints more than `<n>` lines becomes
  `if (!c) return K;` followed by the body; a valued body must fall through to
  the return.
- **merge.** In a valued function, an arm of that last `if` that ends in the
  same `return K;` the function ends with drops it, so the arm falls through to
  the one return (`if (c) { f(); return 0; } return 0;` prints
  `if (c) f(); return 0;`). In a void function with `voidtailreturn` on, the
  same holds for the `return;` ending an arm of the `if` before the final
  `return;` or of an `if` that is the function's last statement:
  `if (p) { ...; return; }` prints `if (p) { ... }` (O0 +17, 17 gained and none
  lost; O2 unchanged). An arm that is nothing but the return keeps it.
- **no-return.** When the function's top-level statements end in a call that
  does not return, the last one-armed `if (c) { X }` before it whose arm
  leaves — `return E` with any `E`, statements and a return, or a jump —
  becomes `if (!c) { T..; f(); }` followed by `X`, where `T..; f();` are the
  statements after the `if`. Neither path reaches the end of the function, so
  `X` still runs only when `c` holds: `if (0 <= n) return n; xalloc_die();`
  prints `if (0 > n) xalloc_die(); return n;`, as authors write it. The call
  is one flow marks as not returning: the artificial `noreturn` halt planted
  right after it, nothing printed between the two, and the call's address
  carrying the warning the printer shows as `// no-return`, which
  `Funcdata::warning` records (`Funcdata::kuna_noreturn_note`,
  `decompiler/crates/kuna-decomp/src/p8_structure/kuna_tailform/noreturn.rs (noreturn_block)`).
  A flow cut at the next function's entry (`funcboundflow`) plants the same
  halt after a call kuna does not know to be fatal, and does not count.
  The search walks back from the call over other statements and over `if`s
  whose arm falls through (they join the new arm); it stops at an `if` with an
  `else` or a goto, and at one whose arm already ends in a call that does not
  return. This runs before the size rule and has no threshold, for void
  functions too, and the size rule never turns an arm that ends in such a call
  back into a guard.

A guard `if` that the `looptest` plan (§8.3 looptest) drops in front of a loop
belongs to that plan: the pass computes the plan on the tree it is editing
(`kuna_tailform.rs (claimed_guards)`) and does not merge, wrap or guard such an
`if` or give it the no-return form, nor move a tail or body that holds one, so
`while (p) { ... }` is not turned back into
`if (!p) return; do { ... } while (p);`. With
`option looptest binary` there is no plan and the pass decides those tails too.

No edit moves a tail, body or arm that holds a truncated flow
(`kuna_tailform.rs (truncated)`): a decode-failure or `halt` return, or a
no-return halt with no call before it, where flow was cut at the next function's
entry. Such a path does not really end the function.

Lines are counted on the tree (`kuna_tailform.rs (count_lines)`): every
statement the printer would emit, each `if`/loop/`switch`/`case` header, labels
and gotos, never a lone brace. The decision is one function,
`kuna_tailform.rs (decide)`, over a `TailSite` (current form, void or valued,
lines), so a different policy plugs in there; `<void>,<value>` sets separate
thresholds for the two kinds, and `wrap` and `guard` force one form at every
site. In a function whose returns `returndup auto` placed from the binary
(`kuna_o0returns.rs (decided)`, see the previous section) no size or model
decision is made: wrap, guard and the valued merge leave the tree alone, and a
last `if` whose arm ends in its own return before the final bare return is
printed as a guard.

The default, `option tailform model`, decides each site with saved weights
instead of the size test. The site's features
(`decompiler/crates/kuna-decomp/src/p8_structure/kuna_tailform/features.rs (compute)`)
are read off the tree and do not depend on which of the two forms the tree is
in, apart from the form itself: printed lines, top-level statements,
statements, calls, assignments, returns, `if`s and loops of the conditional
part (not counting the returns the rewrite hides); what it ends with and
whether its last call shares a basic block with a return; the condition's kind
in the polarity that enters the part (`x`, `!x`, `==`, `!=`, an ordering,
`&&`, `||`, a bit test, a call), its operation count, and whether it reads a
call result or tests a parameter against zero; the returned value's kind; the
statements, returns, loops and guard clauses before the site; and whether the
binary laid the site out wrapped. Numeric features enter as `sign(x)·ln(1+|x|)`,
kinds one-hot, and the score is one dot product with the `const` table in
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_tailform_weights.rs`; a
positive score wraps. Features are computed only when the rule reads them or the
training dump is on. Two cases keep the threshold: a part that ends in a call
that does not return (the no-return form's, not a decision site), and a
function `kuna_unoptimized` (§3) flags, where the model, trained and measured
there too, lost to the threshold (held out, −10 and −28 functions at O0) while
the binary's layout was right at only 68% of those sites against 74% for the
threshold. An empty part is always wrapped, so every choice the model makes is
one the threshold rule makes for some threshold; a unit test checks this over
generated sites.

The table is generated by `docs/features/tailmodel/train.py` (commands in its
README) from two corpus runs with the decision forced each way and
`KUNA_TAILFORM_DUMP=<dir>` set, which makes the pass append one JSON line per
decided site (binary, function, site, features) to `<dir>/<pid>.jsonl`. A
function is an example when both runs decided the same single site and its
control flow matches the source under exactly one form. On 2,284 such sites from
optimized functions an L2 logistic regression (C = 0.01; `switch` and `goto`
counts left out, too rare to weigh long tails sensibly) picks the matching form
at 77% held out by project, against 73% for the 4-line threshold, and gains
+26/+40 functions (FIT/CHECK) at O2 and +11/+22 at O0 over it; a quarter of the
data already gives 76%. End to end against `on`, the shipped table gains 98 at
O2 (190 gained, 92 lost) and 41 at O0 (94 gained, 53 lost), and a table fitted
on the FIT projects only gains 27 and 32 on the CHECK projects it never saw.

`model` replaced the size rule as the default on 2026-11-25, after
`returndup auto`, `iteregion auto` and `loweredswitch auto` had changed which
functions reach the pass. Re-measured then (main bd07ce279), the shipped table
against `on` moves 100 more functions to an exact match at O2 (190 gained, 90
lost; FIT +38, CHECK +62) and 41 at O0 (94 gained, 53 lost; FIT +14, CHECK
+27). A table refitted on that main (2,276 sites from new forced runs) was no
better where it had not been trained: fitted on one half of the projects and
measured on the other, in both directions, it gains 54 functions at O2 and 41
at O0 together against 53 and 41 for the shipped table's training data fitted
the same way, and on the whole corpus it is 8 and 3 functions worse, so the
table fitted on 2,284 sites of the earlier main stays. The model chooses only
between the pass's two forms, so no function computes anything different:
every changed body holds the same non-control statements under either value.
The lost functions are sources that wrote the form the size rule happened to
pick: mostly a function whose whole body is one short `if (c) { ... }` with
nothing before it, which the model prints as a guard clause, and a guard late
in a chain of guards whose remaining tail the model wraps (openssh's
`sshbuf_free`, one source function linked into twelve binaries, is 12 of the
90 O2 losses). `option tailform on` restores the size rule.

Everything is a print-tree edit on the function's top-level statement list. Its
nested `Ls`/`Graph` containers and the statement prefix of each top-level `if`'s
condition list are first hoisted into one flat list, which prints the same; the
pass declines when a container it would free is labelled or a goto names it. A
wrap frees the guard's return leaf and moves the tail into the `if`; a guard
gives the `if` a fresh `BlockCopy` of the final return's basic block as its arm
and moves the body out after it. The condition is negated in place as
`FlowBlock::negateCondition` does it (`Funcdata::block_basic_negate_lastop` on
each leaf, the AND/OR opcode flipped through a `BlockCondition`), and only when
no other leaf prints the condition's block. No op is cloned, reordered or
removed. When the final `return K;` shares a basic block with the statements
before it — the shrink-wrapped `f(); return 0;` — that leaf is flagged
`F_RET_SKIP` and prints inside the `if` without its return, and a new leaf for
the same block flagged `F_RET_ONLY` prints just the return after it (§9 PrintC).
In a void function with `voidtailreturn` on, the bare `return;` that ends a
path moved to the end of the function is flagged `F_RET_SKIP` the same way.

Measured with decbench's own CFG extractor over the O2-noinline corpus (35,189
functions), the size rule (`on`) moves 549 functions to an exact match with the
source (gained 755, lost 206) and 461 at O0 (gained 717, lost 256) against
`off`.
The threshold was fitted on the even-hash half of the projects and checked on
the odd half (O2 +230 / +319); one to eight lines all help, the wrap side
peaks at four, and one value serves void and constant returns. The lost
functions are sources that wrote the other form. With the option on, `rettriangle`
is worth −5 instead of +73 functions at O2 and +75 instead of +378 at O0: the
pass covers the top-level triangle in both directions, and `rettriangle` keeps
deciding before structuring, including joins nested inside another construct.

The no-return form, measured the same way against the size rule alone (main
8d781e789), moves 266 more functions to an exact match at O2 (gained 276, lost
10; FIT +213, CHECK +53) and 236 more at O0 (gained 261, lost 25; FIT +173,
CHECK +63), re-spelling 544 and 587 sites. Void tails of any length are part of
it (+88 and +84 over leaving them to the size rule), and so are arms with
statements before their return (+141 and +129 over bare returns only). The same
rewrite in nested lists — `if` arms; loop bodies never end in such a call —
measured +28 at O2 but +6 at O0 (CHECK +1, thirteen losses), mostly one gnutls
idiom whose source writes `if (mand) { ...; exit(1); } else return NULL;`, so
the pass keeps to the function's top level. The lost functions are sources that
keep the fatal call last after a guard (coreutils `xalloc`:
`if (p) return p; ...; exit(1);`) and dispatch chains of returns that end in a
fatal call; refusing the form after such a chain cost more than it saved.
`tests/stages/kuna-tailform-noreturn.xml` decompiles gcc -O2 functions ending
in `abort` or `exit` with the option off and on, and
`kuna-cli/tests/tailform_noreturn_round_trip.rs` compiles their printed C beside
the source with `abort` and `exit` turned into a `longjmp`, checking that every
function returns, stores and stops where the source does.
`tests/stages/kuna-tailform-model.xml` decompiles the C runtime's
`__do_global_dtors_aux` under the default, which prints the source's guard
clause, and under `on`, which keeps the four-line tail wrapped.
`tests/stages/kuna-tailform.xml` decompiles gcc -O2 functions with the option
off and on, and `kuna-cli/tests/tailform_round_trip.rs` compiles the printed C
of gcc -O2 and -O0 builds beside the source and checks every function returns
and stores what the source does.

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
the archived phase-model study (git history, mechanism
"speculate-validate-rollback") names porting it as future work. In practice
the validation loop runs *outside* the engine instead: the default flips in
this chapter were each gated on a measured decbench GED ablation
(net-positive for DIV-17/23/25; the DIV-18 revert is the same oracle saying
no), so the accept/rollback decision is made per-default at the corpus
level, not per-function at decompile time.


## 8.5 Outlining: excising a region into a synthesized pseudofunction (`outline`)

Every pass in §8.3 rewrites control flow the compiler produced so it reads like the
source. `outline` does something different in kind: it removes a block set from the
function altogether and emits a **call** to a pseudofunction in its place, reversing
compiler inlining.

```c
if (a1) {                          if (a1)
  v1 = a0 + 7;                       a0 = outlined_0x40100a(a0);
  if (0x33 <= v1)          -->     return a0 + 100;
    v1 = a0 + 4;
  a0 = v1 * 3;
}
return a0 + 100;
```

### Who chooses the region, and why it is not a detector

The region is supplied as the option value, `<fn_entry>:<head>-<exit>`. A human
reading the code, or an LLM agent, chooses it. That is a **measured** decision, not
a missing feature. Two automated detectors were built and scored against DWARF
`DW_TAG_inlined_subroutine` ground truth on the decbench corpus, and both failed:

* The SAILR premise — that a residual `goto` marks where the compiler restructured
  control flow, so block sets that become single-entry when a virtualized edge is
  deleted are candidate inlined bodies — scored **1.03–1.09x enrichment over the
  per-function base rate** at **4.5% recall** across 578 reported regions. That is
  indistinguishable from choosing blocks at random inside the same function.
* Structural enumeration plus scoring did no better. Measured within-function, every
  candidate-indexed feature sat at chance: interface narrowness AUC 0.528, stack-slot
  confinement 0.458 (anti-correlated), repetition refuted outright by the oracle. The
  *ceiling* — the best IoU any candidate in the richest pool achieves, i.e. the bound
  on a perfect scorer — is **0.507, 95% CI [0.474, 0.542]** over addressable instances
  in goto-free functions, with a per-binary median of 0.475.

And **63.8% of inlined instances cannot be addressed by a block-set method at all**:
they span fewer than two whole basic blocks (median instance 33 bytes). Region
enumeration is the wrong tool for most of the phenomenon. The one signal that beat
chance was matching against a known function body (out-of-line twin, AUC 0.562), so a
future detector should be content matching, not region enumeration.

The function entry is part of the grammar because an option value is global to the
run: it is applied once and every function is then decompiled under it.

### The region is re-derived, never trusted

The value names only a head and an exit. The member set is recomputed from the live
`bblocks` with `kuna_check_region`, a transcription of the private `check_region` in
`p7_regions/kuna_regionid.rs` (that file is a line-faithful port pinned by
`docs/spec/07-regions.md` and is not modified to expose it). A region is a block
**set**, never the interval `head..exit`: an inlined callee is routinely laid out in
several disjoint pieces, 66% of instances span more than one range, and in the stage
witness the exit address is below the head.

### The seam

Pre-SSA `bblocks` surgery, self-gated on `get_heritage_pass() == 0`. No `MULTIEQUAL`s
exist yet, so moving edges needs no phi patching, and `ActionFuncLink` still
materializes the call's argument and return varnodes afterwards. The pass is
registered before `varnodeprops` rather than directly before `heritage`, because the
`varnodeprops -> lowerswitchinstall -> heritage` run is a ported C++ adjacency
(`coreaction.cc:5751-5756`) pinned by `verify_w8x_allowlist.rs`.

The call is a real `CPUI_CALL` built from nothing — the first in the tree; every other
site mutates an op the decoder produced. It installs the fspec annotation by hand
exactly as `FlowInfo::build_call_specs` does, and `PrintC::call_callee_name`
short-circuits on the supplied name, so no printer change is needed.
`FuncCallSpecs::deindirect` is deliberately not used: it sets `restart_pending`, and
the restart re-lifts from raw bytes, destroying the synthesis.

### Liveness

There is no pre-SSA liveness API, so the pass computes its own over `bblocks`. Three
corrections matter more than the skeleton, each found by the transform declining:

* it is computed over the region's **continuation** — the blocks reachable from the
  exit without re-entering the region — because a read that happens *before* the
  region (the entry block's own flag test) is not a live-out. Without this every
  region is rejected;
* overlapping storage in one space is merged: an x86 32-bit write zero-extends, so
  SLEIGH writes both `EAX` and `RAX` at one offset and one register otherwise counts
  as two live-outs;
* slot 0 of a control-transfer op is its destination, a varnode in the code space. It
  is an address, not a value, and counting it makes every conditional look like a
  memory read.

Liveness is an over-approximation in the safe direction: over-counting can only cause
extra declines, never a wrong excision.

### Two primitives that do not do what their names suggest

* `Funcdata::node_split_block_edge` reads as "insert a block on this edge" and is not.
  It `copy_basic_range`s the target's p-code into the new block and gives it the
  target's successors, because it exists for tail duplication. Used for outlining it
  produces a clone of the region head whose out-edges point back into the region, and
  heritage then fails its dominator invariant. The correct sequence is that function
  minus the copy: `new_block_basic`, `switch_edge` to move the single entry edge onto
  it, `add_edge` to the join — after which the region is unreachable and
  `remove_unreachable_blocks` collects it, with no `remove_branch`/`push_branch`.
* A hand-built `FuncCallSpecs` has **no prototype store**, and every later
  `FuncProto::store` query panics on the null. Decoder-produced specs get theirs from
  `ActionDefaultParams`; a synthesized one must call
  `proto_mut().attach_internal_store(void_ty)`.

### v1 restrictions

The pass declines, leaving the function untouched, unless the head and exit really
bound a single-entry region over the live CFG, exactly one edge enters the head from
outside, at most one value is live out, and the region contains no call and no
`STORE`. Excising a region with a live-out that was not found silently produces C
that computes the wrong value, so v1 refuses rather than guesses. The no-call rule is
the most commonly hit: it declines regions containing compiler intrinsics such as
`__addvsi3` under `-ftrapv`, which are semantically pure.

It does not emit a pseudofunction **body**. There is no seam:
`PrintC::doc_function_full` is one `Funcdata` per document. The call is emitted and
named; the bytes remain in the binary and can be decompiled separately at the head
address.

### Measured coverage on real code

An LLM-driven evaluation over 15 coreutils `-O2` functions — agents read the
decompiled C plus the disassembly, chose a region, and applied the option — outlined
**0 of 15**, on cases selected so that a DWARF-confirmed inlined callee spanning at
least two whole basic blocks existed in every one. Two of those failures were the
pass emitting wrong C (both fixed, and pinned by
`tests/stages/kuna-outline-proto.xml`); the rest were correct declines against two
classes v1 cannot take at all:

* **Multi-exit inlines.** The compiler routinely fuses a callee's return-value
  dispatch into the caller's control flow, so the inlined body has two exits reaching
  two different places. No single join block exists, so no `exit` address bounds the
  region and `kuna_check_region` can never accept it.
  `__xargmatch_internal` — present in every coreutils binary — is this shape: the
  inlined `argmatch_exact` scan exits both to the "not found" tail and to the
  "return the index" tail.
* **Regions containing a genuine external call** (`strcmp`, `error`, `dcgettext`).
  Correctly refused; a function that is mostly I/O has no outlinable region at all.

These sit on top of the 63.8% of inlined instances that span fewer than two whole
basic blocks and are unaddressable by any block-set method. Taken together, the
single-entry/single-exit/one-live-out model fits the stage witness and comparatively
little real optimized code. Widening it — multi-exit regions returning a value through
a caller-side branch, and a purity allowlist so compiler intrinsics such as
`__addvsi3` under `-ftrapv` stop blocking otherwise-clean regions — is the work that
would make the transform generally applicable.

A switch can itself be a loop header. In `RegionStructurer::try_switch_cases`, a
case backedge to that same switch is an internal loop edge, not a terminal
continue to an enclosing loop. It nominates the switch as the exit so
`BlockGraph::new_block_switch` restores a self-edge after collapsing its cases;
the loop schema can then wrap the switch. Backedges to a distinct enclosing loop
head retain their existing continue treatment. Treating both forms alike drops
the internal edge and emits a one-shot switch that loses later iterations.
GCC and Clang emit this shape for `for (;;) switch (f(x)) { ... }`: one jump
table at the loop head, with every case but the default jumping straight back.
The rule is shared by every architecture. When such a switch also has a case
leaving to a different exit block, one switch exit cannot carry both edges, so
the schema declines and the ordinary collapse structures the loop. Folding it
anyway dropped the backedge as well. The same holds beside a case that
continues an enclosing loop: that edge survives the collapse, so
`new_block_switch` would restore no self-edge, and a switch that is its own
exit folds only when no such continue case remains. A direct edge from the
switch to itself makes it its exit in the same way; Clang emits one for an
empty case when no range check precedes the table. An x86-64 compact switch with a retry
case therefore prints its biased selector inline (`switch(mode - 5)`), exactly
as it does with `regionstructure off`; its earlier `switch(mode)` came from
that lossy fold, rescued only when the structurer later stranded.

The self-edge that `new_block_switch` restores leaves the folded switch as a
block whose only edge loops back to itself. When the dispatch loop is a case
of an outer switch, such a case is not a continue either: folding it as a
terminal case would make its self-edge internal and drop the inner loop, so
the outer switch declines until the loop schema has wrapped it.

A folded switch keeps at most one successor, as it does in `CollapseStructure`,
where every case reaches the one exit. A continue case adds a second kind of
exit: its backedge survives the fold as an out-edge of the `BlockSwitch`. When
that edge's loop head is not the switch's exit, the folded switch would be a
block with two successors whose last statement is a switch, and the if and loop
schemas, which read any two-successor block as a condition, printed it as one:
`if` followed by the whole switch statement a second time, which is not C.
`RegionStructurer::try_switch_cases` declines such a fold. Usually the exit is
only the unfolded rest of one case, a case that runs an inner loop before it
continues (bash `man2html`'s `scan_format` at `-O0`); once the loop and
sequence schemas have folded that rest into the case, every case continues and
the switch folds as the source wrote it, the inner loop inside its case. When
nothing else can move, because cases that `break` rejoin after the switch, the
schema runs once more with `split` before any edge is virtualized: each
continue case whose loop head is not the kept successor becomes a `BlockGoto`
to that head, which `loopcontinue` prints as `continue;`, and the switch then
folds with the one exit. Continue cases to two different loop heads keep the
first case's head and split the rest.

Whatever produced it, no schema puts a block that prints a statement into a
condition slot. `kuna_switchcond.rs (renders_branch)` is false for a switch, an
`if`, a loop or a goto at the end of any nesting of lists, and the if/else,
if-then, short-circuit, while-do, do-while and if-goto folds all require it.
`structure()` checks the finished tree with `kuna_switchcond.rs
(misplaced_condition)` and reports non-convergence on a hit, so
`ActionBlockStructure` falls back to `CollapseStructure`, which cannot build
such a tree. The printer holds the last line (§9).
