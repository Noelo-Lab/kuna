//! S8 **short-circuit condition folding across a complex sibling block**
//! (`option condfold off|on|wide`, default-OFF) — a port of the angr Phoenix
//! `MultiStatementExpression` relaxation in
//! `angr/analyses/decompiler/structuring/phoenix.py`'s
//! `_match_acyclic_short_circuit_conditions` (types a-d).
//!
//! This module is the consolidation of two independently-developed features,
//! PR #193 (`condfold`, closing proposal #39) and PR #194 (`condjoin`, closing
//! proposal #56).  They turned out to relax **the identical gate** from opposite
//! ends, so they ship as one option with a **union** admission predicate; see
//! *Two admission rules* below.
//!
//! # The structural gap
//!
//! Both of kuna's structurers implement the Ghidra `ruleBlockOr` short-circuit
//! schema: a 2-out condition `bl` whose sibling successor `orblock` is itself a
//! single-in 2-out condition reconverging through a shared `clauseblock` folds into
//! one [`BlockCondition`](crate::block::BlockKind::Condition), rendering
//! `if (A || B) { clause }` instead of a nested `if`/crossing `goto`.
//!
//! Ghidra additionally requires `orblock` to be **non-complex** (`BlockBasic::isComplex`,
//! `block.cc:2403`: at most two "statements", where the trailing branch already
//! counts as one).  In practice that means the sibling must be a *bare* condition
//! block.  When the compiler parked one extra statement in front of the second
//! test — a spill, an address computation, a call whose result is the test — the
//! fold is declined and the second arm is emitted as a separate block that jumps
//! back into the first arm's clause.
//!
//! Two witnesses, from the two proposals this option closes:
//!
//! * **#39 — coreutils `tr::build_spec_list`** (a *prefix* in front of the test):
//!
//!   ```text
//!   if (es_match(es,i,':')) {
//!   label_40242e:
//!       ... shared clause ...
//!   }
//!   else {
//!       result_idx = &v10;                              // the prefix that declines the fold
//!       if (es_match(es,v12,'=')) goto label_40242e;    // the crossing edge
//!       ...
//!   }
//!   ```
//!
//! * **#56 — lighttpd `server.c::main`** (angr corpus `newbury`, the classic SUID
//!   guard).  Three guards form a `ruleBlockOr` cascade over four basic blocks:
//!
//!   ```text
//!   B0: getuid();            if (== 0)          -> BODY   else B1
//!   B1: v = geteuid();       if (v == getuid()) -> B2     else B3
//!   B2: v = getegid();       if (v != getgid()) -> B3     else BODY
//!   B3: v = -1; fputs(...);  -> RET       (in-degree 2: from B1 and B2)
//!   BODY: ... server loop    -> RET       (in-degree 2: from B0 and B2)
//!   ```
//!
//!   Every gate holds for `(B1,B2)` and then for `(B0, B1||B2)` **except**
//!   `is_complex(orblock)` — B1/B2 each hold two calls plus the branch, three
//!   statements.  So one of the two edges into `BODY` is virtualized into
//!   `goto label_f6ec;` + a `label_f6ec:` inside the sibling arm: kuna emits
//!   1 goto / 1 label where angr emits 0 / 0.
//!
//! angr does not decline on either shape.
//! `_match_acyclic_short_circuit_conditions_type_a` (phoenix.py:2844, repeated at
//! :2887/:2934/:2977) checks `_is_single_statement_block(left)` and, when the
//! sibling is *not* a single statement, wraps the operand in an AIL
//! `MultiStatementExpression` — a C **comma expression** — instead of refusing:
//!
//! ```python
//! if not self._is_single_statement_block(left):
//!     if not self._should_use_multistmtexprs(left):
//!         return False
//!     stmts = self._build_multistatementexpr_statements(left)
//!     left_right_cond_expr = MultiStatementExpression(
//!         self.ail_manager.next_atom(), stmts, left_right_cond_expr, ins_addr=left.addr)
//! ```
//!
//! The eligibility policy is `_should_use_multistmtexprs` (phoenix.py:3239) under the
//! default `MultiStmtExprMode.MAX_ONE_CALL`: at most one call and at most
//! `_multistmtexpr_stmt_threshold` (default **5**) non-label statements; and
//! `_build_multistatementexpr_statements` (phoenix.py:3495) refuses outright on a
//! non-terminal `ConditionalJump` or any `Jump`.
//!
//! kuna's `COMMA_SEPARATE` printer modifier *is* `MultiStatementExpression`; kuna's
//! `is_complex` *is* `_is_single_statement_block`.
//!
//! # What this option does
//!
//! `condfold` reproduces exactly that relaxation, and **only** that relaxation.  It
//! widens the single `is_complex(orblock)` decline inside the short-circuit schema of
//! *both* structurers ([`CollapseStructure::rule_block_or`](crate::blockaction) and
//! [`RegionStructurer::try_block_or`](crate::p8_structure::region_structurer)): a
//! *complex* sibling is accepted anyway when it satisfies one of the two admission
//! rules below.  Every other precondition `ruleBlockOr` enforces is untouched.
//!
//! # Why no p-code moves (the soundness argument — do not weaken this)
//!
//! The fold is **rendering-only**.  [`BlockGraph::new_block_condition`](crate::block::BlockGraph)
//! re-parents two existing structuring nodes; it does not move, reorder, duplicate,
//! or delete a single p-code op.  The prefix statements are then printed by
//! [`PrintC::emit_block_condition`](crate::printc) inside the right operand of
//! `&&`/`||` under the existing `COMMA_SEPARATE` modifier, which the printer already
//! implements (`emit_basic_block_ops` glues statements with `, ` and suppresses the
//! `;`).  C's short-circuit-plus-comma sequencing means those statements execute on
//! exactly the paths, and in exactly the order, they did before:
//!
//! * `orblock.size_in() == 1` (an inherited `ruleBlockOr` precondition) — nothing
//!   else reaches the sibling, so its statements cannot run on a path where the
//!   compound condition is not evaluated;
//! * the right operand of `||`/`&&` is evaluated iff control used to fall into
//!   `orblock`;
//! * the comma operator is a sequence point, so the prefix still runs before the
//!   test;
//! * the only synthesized operation is `negate_condition_rec`, a boolean sense flip
//!   on an already-computed condition.  It is order-neutral.
//!
//! **Proposal #56 framed the fix as "invert the guard chain by de Morgan and hoist the
//! join", and warned that a wrong inversion emits wrong C because the predicates call
//! `getuid`/`geteuid`/`getegid`/`getgid`, which cannot be reordered past one another.
//! No such reordering is possible here and no purity analysis is needed** — the
//! operands are never swapped, only relabelled.
//!
//! So the *semantic* precondition set beyond `ruleBlockOr`'s own is empty.  Every
//! guard the two rules add is a **rendering-validity or quality** guard: the sibling
//! must render *as an expression at all*, and the result must stay readable.
//!
//! # One width budget, two admission rules (the predicate is their UNION)
//!
//! Every admitted sibling passes the **same printed-width budget** — [`printed_shape`]
//! counted against the policy level's cap ([`MAX_PREFIX_STMTS_ANGR`] = 5 at `on`,
//! [`MAX_PREFIX_STMTS_WIDE`] = 9 at `wide`).  That budget is what bounds the emitted
//! comma chain, and the level is the only thing that moves it.
//!
//! On top of that shared budget a sibling is admissible when **either** rule accepts
//! it.  The rules were derived independently against different targets and neither
//! subsumes the other; do not intersect them.
//!
//! ## Rule A — bounded prefix (from #193, closes #39)
//!
//! [`prefix_eligible`]: the sibling is a [`BlockCopy`](crate::block::BlockKind::Copy)
//! of ONE `BlockBasic`, within the shared width budget, with at most
//! [`MAX_PREFIX_ROOT_CALLS`] statement-root call, branch-free, and comment-free.  A
//! `BlockList`/`BlockIf`/`BlockCondition` sibling is never accepted by this rule —
//! such an operand can render braces, multiple lines, or a label inside the
//! parentheses, i.e. invalid C.
//!
//! ## Rule B — statement shape (from #194, closes #56)
//!
//! [`shape_score`]: an explicit **statement-shape allowlist** — every op is a marker
//! (never printed), an op with an output varnode, a void `CALL`/`CALLIND`, a `STORE`,
//! or the single terminal `CBRANCH`.  Anything else declines: `RETURN`, a second
//! branch, `BRANCHIND`, `CALLOTHER` (userop rendering can be multi-line or
//! special-cased), a no-return call.  At most [`MAX_SHAPE_STMTS`] *scored* statements
//! and [`MAX_SHAPE_CALLS`] calls per block, no comment, and — like Rule A — within the
//! shared printed-width budget.
//!
//! Rule B additionally admits a **nested `BlockCondition`** as the sibling — which
//! Rule A never does — because that is what a guard *cascade* needs: `newbury::main`
//! folds `(B1,B2)` first and then takes the resulting Condition as the sibling of
//! `B0`.  `PrintC::emit_block_condition` emits *both* sub-blocks of a nested
//! Condition under `COMMA_SEPARATE`, so every leaf must itself be Rule-B admissible
//! (the recursion in `CollapseStructure::is_shape_foldable`), and the fold site
//! enforces the cross-leaf caps [`MAX_JOIN_LEAVES`] / [`MAX_JOIN_TOTAL_STMTS`]
//! (`collapse_conditions` is a fixpoint loop with no natural bound, so without them a
//! long cascade would fold into an unreadable monster).
//!
//! ## Which rule fires, and what separates the two in the corpus
//!
//! `CollapseStructure::condfold_ok` asks Rule A first, so a sibling both rules accept
//! folds through Rule A.  Neither shared property can tell them apart — the width
//! budget is enforced by both on the same measure, and the comment guard declines
//! both.  Exactly two properties are Rule B's alone: the nested `BlockCondition`
//! operand above, and the **call cap**, which unlike the width budget does *not* move
//! with the policy level ([`MAX_PREFIX_ROOT_CALLS`] = 1 at `on` *and* at `wide`,
//! against [`MAX_SHAPE_CALLS`] = 2).  A block with two calls that each print as their
//! own comma element is therefore outside Rule A at every level and inside Rule B.
//!
//! The three stage witnesses split along exactly that line:
//!
//! | witness | folds through |
//! |---|---|
//! | `tests/stages/ghangr-condfold.xml` | Rule A (a store prefix, no calls) |
//! | `tests/stages/ghangr-condfold-newbury.xml` | Rule A (the guard-cascade *shape*; at `wide` Rule A's budget also admits its guard blocks) |
//! | `tests/stages/ghangr-condfold-ruleb.xml` | Rule B, twice, in both engines — a middle guard with two statement-root calls, then a nested `BlockCondition` sibling |
//!
//! Only the third one notices if Rule B is removed.  Which rule fired is traceable:
//! see [`trace_admit_fold`] and [`trace_ops`].
//!
//! ## What the width budget bounds, and what it does not
//!
//! Rule B *also* keeps a second, weaker measure: it scores a statement with the same
//! `Varnode::calc_explicit` approximation
//! [`Funcdata::bb_is_complex`](crate::funcdata::Funcdata::bb_is_complex) uses
//! (addr-tied / no-descend / used-outside-block / over `max_implied_ref` refs ⇒
//! printed).  **That score is not a printed width and must never be used as one.**  A
//! reviewer of the consolidation measured `tr::build_spec_list`'s third-term sibling
//! *scoring 2* under it while *rendering seven* comma elements; when the score was the
//! only Rule-B cap, `on` therefore emitted the option's widest operands and `wide`
//! could not differentiate itself on a single named target.  The score survives only
//! because [`MAX_JOIN_LEAVES`]/[`MAX_JOIN_TOTAL_STMTS`] are expressed in it.
//!
//! What the shared budget now guarantees, precisely:
//!
//! * **Per admitted leaf**, the comma chain [`printed_shape`] measures is at most 5
//!   elements at `on` and 9 at `wide`, counted with the printer's own skip rules.
//!   Measured across 2827 functions in 7 binaries, the widest comma operand condfold
//!   *creates* is exactly **5 at `on`** and **7 at `wide`**.  (The widest operand in
//!   the emitted C at `on` is 6, in `dd::human_readable` — and that one is already
//!   there with the option **off**: Ghidra's own `is_complex` budget has the same
//!   score-vs-width slack, which is the phenomenon this budget exists to stop
//!   inheriting.)
//! * The measure is taken on the op list **as it stands at structuring time**.  Passes
//!   that run afterwards can still add or drop an op, so the emitted chain can differ
//!   by an element or two: on `tr::build_spec_list` the measure reads 6–9 across the
//!   structuring passes and the operand finally emits 8 elements.
//! * Where a call's stack-effect ops are still live at structuring time (an
//!   unprototyped callee, no frame recovery — the stage witness
//!   `tests/stages/ghangr-condfold-newbury.xml` is the reproducible case, at 6 measured
//!   elements for a guard block whose operand finally emits none), the measure
//!   **over**-counts.  Over-counting only ever declines a fold, so the error is in the
//!   safe direction; it is why that witness needs `wide` while the real
//!   `newbury::main` closes at `on`.
//! * It is a **per-leaf** bound, not a bound on the whole folded condition.  A cascade
//!   folded under Rule B may have up to [`MAX_JOIN_LEAVES`] leaves, and the only
//!   cross-leaf cap ([`MAX_JOIN_TOTAL_STMTS`]) is expressed in the weak
//!   `calc_explicit` score, so it does not bound their summed printed width.  Nor does
//!   any of this bound the final condition's *global* width — see the residual below.
//!
//! # Guards that live at the call site
//!
//! * the pre-existing `!orblock.is_interior_goto_target()` decline (never relaxed):
//!   `emit_block_copy` emits `emit_any_label_statement` *first*, so a labelled
//!   sibling would print `label_x:` inside an expression;
//! * a folded `BlockCondition` is force-marked complex (`condfolded`) so
//!   `ruleBlockWhileDo` can never lift the comma chain into a `while(...)` header:
//!   `BlockCondition::isComplex` delegates to sub-block 0, which would report the
//!   *left* operand's trivial verdict for a node whose right operand is a comma chain.
//!
//! That `condfolded` marking is also what keeps Rule B's caps honest.  A reviewer of
//! #194 observed that [`MAX_JOIN_LEAVES`]/[`MAX_JOIN_TOTAL_STMTS`] were consulted only
//! on the `is_complex(orblock) == true` path, so once a relaxed fold produced a
//! Condition whose *leftmost* leaf happened to be non-complex, a later fold taking
//! that Condition as its sibling saw `is_complex == false` and skipped the caps
//! entirely.  Marking every relaxed result complex closes that path: such a node
//! always re-enters the admission check.  **A residual remains and is deliberate**:
//! the caps bound the relaxation at each site where it is *exercised*, they are not a
//! global bound on the final condition's width.  A purely upstream-legal fold (both
//! operands non-complex) can still extend a cascade on the *left* without consulting
//! them — exactly as it can on `main` today, and each such leaf is bounded by Ghidra's
//! own two-statement budget.
//!
//! # Known effects (accepted, not bugs)
//!
//! * **Not a monotone goto reducer.**  Folding changes which edges the structurer can
//!   subsequently match, so individual functions can gain a goto even though the
//!   aggregate is a clear net win.  Measured on the 7-binary / 2827-function sweep
//!   against `main` @ `d09f21ce`: `clientloop.o::process_escapes` 26 → 28 (at `on` and
//!   `wide`) and `true_ubuntu1804::usage` 1 → 2 (at `wide` only).  Two gotos added and
//!   77 removed at `on`; three added and 106 removed at `wide`.
//! * **A later-produced advisory comment can be lost.**  Both rules decline a block
//!   that already carries **any** buffered comment, but the check only sees comments
//!   *buffered in `Funcdata` at structuring time*.  `COMMA_SEPARATE` suppresses
//!   `emit_comment_group`, so an advisory produced by a pass that runs after
//!   structuring can still vanish.  Reproduced: `newbury::main` emits 4 `WARNING:`
//!   lines at `off` and 3 at `on`/`wide`; the lost one is
//!   `/* WARNING: branchflip: flipped negated guard for linearity ... */`.  These are
//!   kuna advisory annotations, never a correctness signal, and the emitted C is
//!   unaffected.
//!
//! When the option is OFF the whole precompute and every disjunct are dead and the
//! emitted C is byte-identical to `main`.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId};
use crate::funcdata::Funcdata;

/// Marshaling element `<condfold>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce, 4101 tailcalljump, 4102 regionstructure, 4103 noreturn_extern,
/// 4104 noreturn_externmatch, 4105 crossjumprevert, 4106 switchsharedcase,
/// 4107 ifelseflatten, 4108 taildup, 4109 dedupitetail, 4110 returndup,
/// 4111 earlyreturn, 4112 switchreturn are taken; this is 4113.  4114 was briefly
/// claimed by `condjoin` in PR #194, which this option absorbs — it is released.)
pub const ELEM_CONDFOLD: ElementId = ElementId::new("condfold", 4113);

/// The shared printed-width budget at `option condfold on` — **angr parity**: angr's
/// `_multistmtexpr_stmt_threshold` (`PhoenixStructurer.__init__`, phoenix.py:103)
/// defaults to 5 non-label AIL statements.
///
/// The name is historical (it came from Rule A, PR #193); **both** admission rules
/// enforce it, measured by [`printed_shape`].
pub const MAX_PREFIX_STMTS_ANGR: int4 = 5;

/// The shared printed-width budget at `option condfold wide`.
///
/// kuna's *printed* statement granularity is finer than angr's AIL for the same
/// source-level block, for two reasons that have nothing to do with the fold's
/// soundness:
///
/// * argument recovery — where angr renders `find_closing_delim(p, v11, *(char *)(v10
///   + cur), &v7)` as ONE statement, kuna may recover only the first two arguments
///   and print the remaining operand computations as separate assignments;
/// * address-tied copy shadows — a stack variable that also lives in a register
///   prints a `v14 = v1;` / `v14 = v14;` pair the AIL never materializes.
///
/// On `coreutils tr::build_spec_list` the block angr folds as 1 AIL statement prints
/// as 7–8 kuna statements, so the angr-parity cap declines a fold angr makes.  `wide`
/// raises the budget to 9 to cover that skew.  It is a *readability* knob only —
/// every soundness and rendering-validity guard is identical at both levels.
///
/// The level moves the budget for **both** rules.  It does not move Rule A's call cap
/// [`MAX_PREFIX_ROOT_CALLS`], Rule B's shape allowlist, Rule B's
/// [`MAX_SHAPE_STMTS`]/[`MAX_SHAPE_CALLS`], or the cross-leaf caps
/// [`MAX_JOIN_LEAVES`]/[`MAX_JOIN_TOTAL_STMTS`].
pub const MAX_PREFIX_STMTS_WIDE: int4 = 9;

/// Rule A's call cap, at every policy level: the maximum number of calls printed as
/// **their own comma-chain element** in the folded sibling.  angr's default
/// `MultiStmtExprMode.MAX_ONE_CALL` policy (`_should_use_multistmtexprs`,
/// phoenix.py:3239) allows at most one, and a comma chain with two printed call
/// statements in an `if (...)` is unreadable regardless of the statement count.
///
/// **The name is the contract — read it before quoting this as angr parity.**
/// [`prefix_eligible`] walks the block with the printer's own skip rules, and the
/// *implied*-output skip necessarily runs **before** the `is_call` test, because an
/// implied call is not a statement at all — it is inlined into its consumer's
/// expression.  A call whose result is folded into the sibling's condition is
/// therefore never charged here.  The bound this enforces is "at most one call
/// printed as its own comma-chain element", **not** "at most one call in the rendered
/// operand": a folded operand may render two or more calls when the extras are
/// implied.  Measured on the aggregate sweep, `mv -O2 copy_internal` at `wide`
/// renders `cached_umask(...)` (a statement root, counted) *and* `fchmod(...)`
/// (implied into the test, not counted).
///
/// **Deliberate decision (PR consolidating #193 + #194):** the statement-root
/// semantics are *kept*, and the constant is named for them.  Rationale: the cap
/// exists to bound the *printed comma chain*, which is what harms readability, and
/// the printer's statement granularity is the only granularity the structurer can
/// observe cheaply.  It is a readability bound, not a soundness bound — the fold
/// moves no p-code and the short-circuit + comma sequencing argument above is
/// independent of how many calls appear, so the looser count cannot produce wrong C.
/// Rule B, which counts *every* call op including implied ones
/// ([`MAX_SHAPE_CALLS`]), is the tighter half of the union and covers the shapes
/// where call density is the actual concern.
pub const MAX_PREFIX_ROOT_CALLS: int4 = 1;

/// Rule B's per-block cap on conservatively-scored statements.  Ghidra's own budget
/// admits 1 (its `statement > 2` counts the branch); angr's
/// `multistmtexpr_stmt_threshold` is 5.  2 is the tightened middle that admits the
/// `v = geteuid(); v == getuid()` shape and little else.
///
/// **This is a `calc_explicit`-approximation score, NOT a printed width.**  A reviewer
/// of the consolidation measured `tr::build_spec_list`'s third-term sibling scoring 2
/// here while rendering **seven** comma elements, so this cap on its own bounds
/// nothing an operator can see.  Rule B therefore *also* enforces the same printed
/// budget Rule A does ([`MAX_PREFIX_STMTS_ANGR`] / [`MAX_PREFIX_STMTS_WIDE`], measured
/// by [`printed_shape`]); this score survives only because the cross-leaf caps
/// [`MAX_JOIN_LEAVES`]/[`MAX_JOIN_TOTAL_STMTS`] are expressed in it, and because it is
/// the tighter of the two on the single-test shapes Rule B is for.
pub const MAX_SHAPE_STMTS: int4 = 2;

/// Rule B's per-block cap on calls — **every** call op, including ones whose result is
/// implied into the test (unlike Rule A's [`MAX_PREFIX_ROOT_CALLS`]).  The #56 target's
/// clause blocks hold two (`geteuid`/`getuid`, `getegid`/`getgid`), one of which inlines
/// into the compare.
pub const MAX_SHAPE_CALLS: int4 = 2;

/// Rule B's cap on condition leaves in the `BlockCondition` a relaxed fold may build.
/// The #56 target needs 3 (`getuid` test, `geteuid` test, `getegid` test).
pub const MAX_JOIN_LEAVES: int4 = 4;

/// Rule B's cap on scored statements summed over every leaf of the folded
/// `BlockCondition`.  The `newbury::main` target scores 0 (both of its relaxed clause
/// blocks reduce to two calls whose outputs inline), so the cap binds only on wider
/// folds.
///
/// **Expressed in [`MAX_SHAPE_STMTS`]'s `calc_explicit` score, therefore NOT a bound
/// on the cascade's summed printed width.**  Each leaf is individually bounded by the
/// shared printed-width budget; their sum is not.  Documented rather than tightened:
/// converting this cap to printed width would decline `newbury::main` (three leaves
/// printing 2–3 elements each), and the per-leaf budget is what an operator sees.
pub const MAX_JOIN_TOTAL_STMTS: int4 = 4;

//===========================================================================
// The shared printed-width measure (both rules' budget)
//===========================================================================

/// How wide does `ops` render as a `COMMA_SEPARATE` operand?
///
/// This is the option's **one width measure**, shared by both admission rules.  It
/// walks the block's live op list with the *same* skip rules
/// [`PrintC::emit_basic_block_ops`](crate::printc) applies, so its count is the number
/// of elements the emitted comma chain will actually have:
///
/// * `not_printed()` ops (which includes every SSA marker) are skipped;
/// * an op whose output varnode is *implied* is skipped — it is inlined into its
///   consumer's expression, not printed as a statement;
/// * an unconditional `CPUI_BRANCH` is skipped (the printer never emits it);
/// * the terminal `CPUI_CBRANCH` **is** counted: it renders as the chain's last
///   element, the operand's condition.
///
/// Returns `None` when the block cannot render as an expression at all — it does not
/// end in a `CPUI_CBRANCH` (nothing to fold), or a non-terminal branch op survives the
/// skips (a mid-block conditional jump or a `BRANCHIND`; angr's
/// `_build_multistatementexpr_statements` refuses the same shapes, and a `goto` is not
/// an expression).
fn printed_shape(data: &Funcdata, ops: &[OpId]) -> Option<PrintedShape> {
    let tail = *ops.last()?;
    // The block must end in a conditional branch (this is the `orblock` of a 2-out
    // short-circuit schema; the caller already checked `size_out() == 2`).
    if data.obank().get(tail).map(|o| o.code())? != OpCode::CPUI_CBRANCH {
        return None;
    }

    let mut out = PrintedShape {
        stmts: 0,
        root_calls: 0,
    };
    // Charge ONE comma-chain element.  Every `out.stmts` increment goes through
    // here, so the `KUNA_CONDFOLD_DEBUG=ops` listing and the width this function
    // reports can never drift apart: a block measured at N lists exactly N ops.
    // (It used to be possible: the terminal CBRANCH was charged at its own
    // `continue`, which skipped the trace, so every block listed one op short of
    // its own measurement.)
    macro_rules! charge {
        ($o:expr) => {{
            out.stmts += 1;
            if trace_ops() {
                eprintln!(
                    "[condfold] op #{} {:?} @ {:#x}",
                    out.stmts,
                    $o.code(),
                    $o.get_addr().get_offset()
                );
            }
        }};
    }
    for (idx, &op) in ops.iter().enumerate() {
        let o = data.obank().get(op)?;
        // printc.rs emit_basic_block_ops: `if o.not_printed() { continue; }`
        // (`not_printed` already covers every SSA marker: MULTIEQUAL / INDIRECT).
        if o.not_printed() || o.is_marker() {
            continue;
        }
        if o.is_branch() {
            // The printer never emits a bare CPUI_BRANCH.
            if o.code() == OpCode::CPUI_BRANCH {
                continue;
            }
            // The terminal CBRANCH is the operand's condition — the chain's last
            // element, so it is charged (and listed) like any other.
            if idx + 1 == ops.len() {
                charge!(o);
                continue;
            }
            // Any other branch (a mid-block conditional jump, a BRANCHIND, a
            // CALLOTHER-flavoured flow break) cannot render as an expression.
            return None;
        }
        // printc.rs emit_basic_block_ops: ops whose out varnode is implied are
        // inlined into their consumer and never printed as a statement.  This skip
        // is *why* MAX_PREFIX_ROOT_CALLS counts statement roots only.
        if let Some(outvn) = o.get_out() {
            if data
                .vbank()
                .get(outvn)
                .map(|v| v.is_implied())
                .unwrap_or(false)
            {
                continue;
            }
        }
        if o.is_call() {
            out.root_calls += 1;
        }
        charge!(o);
    }
    Some(out)
}

/// What [`printed_shape`] measures: the width of a block's rendered comma chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintedShape {
    /// Elements the emitted comma chain will have, **including** the terminal
    /// CBRANCH (which renders as the chain's last element, the condition itself).
    pub stmts: int4,
    /// Of those elements, how many are calls printed as their own element.  A call
    /// whose output is implied is inlined into its consumer and is not counted here
    /// — see [`MAX_PREFIX_ROOT_CALLS`].
    pub root_calls: int4,
}

//===========================================================================
// Rule A — bounded prefix (PR #193, proposal #39)
//===========================================================================

/// **Rule A.**  Is the **bblocks** `BlockBasic` `bb` an acceptable right operand of a
/// folded short-circuit condition (angr `_should_use_multistmtexprs` +
/// `_build_multistatementexpr_statements`)?
///
/// Measures the block with [`printed_shape`] — the printer's own skip rules — so the
/// caps govern exactly what will appear in the emitted comma expression.
///
/// `max_stmts` is the policy's statement cap ([`MAX_PREFIX_STMTS_ANGR`] for
/// `option condfold on`, [`MAX_PREFIX_STMTS_WIDE`] for `wide`).  Rule B enforces the
/// *same* cap on the *same* measure; the two rules differ in what else they demand,
/// never in how wide an operand they will let through.
///
/// Declines when:
///
/// * more than `max_stmts` printed comma-chain elements, or more than
///   [`MAX_PREFIX_ROOT_CALLS`] **statement-root** calls (see that constant for what
///   the count does and does not cover);
/// * [`printed_shape`] refuses the block outright — a non-terminal branch op survives
///   the skips (a mid-block conditional jump; angr
///   `_build_multistatementexpr_statements` refuses these, and a `goto` is not an
///   expression), or the block does not end in a `CPUI_CBRANCH` (nothing to fold);
/// * an analysis comment is attached at an address the block covers (the printer
///   suppresses `emit_comment_group` under `COMMA_SEPARATE`, so the comment would
///   be silently dropped).
pub fn prefix_eligible(data: &Funcdata, bb: BlockId, max_stmts: int4) -> bool {
    let ops = data.bb_ops(bb);
    let shape = match printed_shape(data, &ops) {
        Some(s) => s,
        None => return false,
    };
    if shape.stmts == 0 || shape.stmts > max_stmts {
        return false;
    }
    if shape.root_calls > MAX_PREFIX_ROOT_CALLS {
        return false;
    }
    if block_carries_comment(data, &ops) {
        return false;
    }
    true
}

//===========================================================================
// Rule B — statement shape (PR #194, proposal #56)
//===========================================================================

/// **Rule B.**  Score a `bblocks` `BlockBasic` for statement-shape admissibility.
///
/// Returns `Some(`[`ShapeVerdict`]`)` when the block satisfies the allowlist, the
/// per-block caps, the shared printed-width budget and the comment guard (see the
/// module docs) and may therefore be absorbed into a short-circuit condition as a
/// comma-separated multi-statement operand; `None` when any precondition fails.
///
/// Two independent measures are taken and **both** must pass:
///
/// * [`ShapeVerdict::printed`] — the shared [`printed_shape`] width, capped at
///   `max_stmts`, the same budget Rule A enforces.  This is what bounds the emitted
///   comma chain, and it is the only bound that does.
/// * [`ShapeVerdict::scored`] — the `Varnode::calc_explicit` approximation
///   [`Funcdata::bb_is_complex`](crate::funcdata::Funcdata::bb_is_complex) uses,
///   capped at [`MAX_SHAPE_STMTS`].  It deliberately does **not** read
///   `Varnode::is_explicit()`, which is not yet computed when structuring runs; like
///   Ghidra's own use of it the approximation is inexact in both directions, which is
///   precisely why it cannot serve as the width budget.  It survives because the
///   cross-leaf caps [`MAX_JOIN_LEAVES`]/[`MAX_JOIN_TOTAL_STMTS`] are expressed in it.
fn shape_score(data: &Funcdata, bb: BlockId, max_stmts: int4) -> Option<ShapeVerdict> {
    // Decline with a reason, traced to stderr under `KUNA_CONDFOLD_DEBUG=1` (run
    // `decomp_dbg` directly to see it; the `kuna` CLI drops the child's stderr).
    // Declining is always correct: the fold simply does not happen and today's
    // goto+label output stands.
    macro_rules! decline {
        ($reason:literal) => {{
            trace(bb, $reason);
            return None;
        }};
        ($fmt:literal, $($arg:tt)*) => {{
            trace(bb, &format!($fmt, $($arg)*));
            return None;
        }};
    }
    // A short-circuit clause block is a binary condition block by construction.
    if data.bblocks_ref().block(bb).size_out() != 2 {
        decline!("not a 2-out condition block");
    }
    let ops = data.bb_ops(bb);
    let tail = *ops.last()?;

    let maxref = data.get_arch().max_implied_ref;
    let mut statements: int4 = 0;
    let mut calls: int4 = 0;

    for inst in ops.iter().copied() {
        let instobj = data.obank().get(inst)?;
        // Markers (MULTIEQUAL/INDIRECT) are never printed.
        if instobj.is_marker() {
            continue;
        }
        let code = instobj.code();
        if inst == tail {
            // The block must end in exactly one CBRANCH (the condition the fold turns
            // into an operand).  `size_out() == 2` makes anything else impossible, but
            // check it so this module is safe by construction rather than by luck.
            if code != OpCode::CPUI_CBRANCH {
                decline!("tail op is not a CBRANCH");
            }
            continue;
        }
        // No second control-flow op anywhere before the tail.
        if instobj.is_flow_break() {
            decline!("interior flow-break op (RETURN / second branch)");
        }
        // Userop rendering can be multi-line / special-cased; never absorb one.
        if code == OpCode::CPUI_CALLOTHER {
            decline!("CALLOTHER (userop rendering is not expression-safe)");
        }
        // A call that does not return terminates the block; it can never be a
        // sub-expression of a condition.
        if instobj.is_call() {
            if (instobj.get_flags() & crate::op::pcodeop_flags::noreturn) != 0 {
                decline!("no-return call");
            }
            calls += 1;
            if calls > MAX_SHAPE_CALLS {
                decline!("over MAX_SHAPE_CALLS");
            }
        }
        match instobj.get_out() {
            None => {
                // Allowlist for void statements: a call or a STORE.  Anything else
                // without an output is not something we can vouch for inside an
                // expression.
                if !instobj.is_call() && code != OpCode::CPUI_STORE {
                    decline!("void op that is neither a call nor a STORE");
                }
                statements += 1;
            }
            Some(outvn) => {
                // Conservative Varnode::calc_explicit, mirroring
                // `Funcdata::bb_is_complex` (funcdata_block.rs).
                let vnobj = data.vbank().get(outvn)?;
                let mut printed = false;
                if vnobj.has_no_descend() || vnobj.is_addr_tied() {
                    printed = true;
                } else {
                    let mut totalref: int4 = 0;
                    for d_op in vnobj.descend_iter() {
                        let dobj = data.obank().get(d_op)?;
                        if dobj.is_marker() || dobj.get_parent() != Some(bb) {
                            printed = true;
                            break;
                        }
                        totalref += 1;
                        if totalref > maxref {
                            printed = true;
                            break;
                        }
                    }
                }
                if printed {
                    statements += 1;
                }
            }
        }
        if statements > MAX_SHAPE_STMTS {
            decline!(
                "over MAX_SHAPE_STMTS: scored {} > {}",
                statements,
                MAX_SHAPE_STMTS
            );
        }
    }

    // The shared printed-width budget — the SAME cap Rule A enforces, on the SAME
    // measure.  The `calc_explicit` score above is an approximation of what the
    // printer will emit and is inexact in both directions (a reviewer measured
    // `tr::build_spec_list`'s third-term sibling scoring 2 and rendering 7), so it
    // cannot bound the comma chain.  This can.
    let printed = match printed_shape(data, &ops) {
        Some(s) => s,
        None => decline!("cannot render as an expression (printed_shape refused)"),
    };
    if printed.stmts > max_stmts {
        decline!(
            "over the printed-width budget: prints {} > {}",
            printed.stmts,
            max_stmts
        );
    }

    // A comment anchored inside this block would be dropped by the COMMA_SEPARATE
    // printer path.  Decline rather than lose it.
    if block_carries_comment(data, &ops) {
        decline!("carries a comment the COMMA_SEPARATE printer would drop");
    }

    Some(ShapeVerdict {
        scored: statements,
        printed: printed.stmts,
    })
}

/// Rule B's two per-block measures — see [`shape_score`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeVerdict {
    /// The `calc_explicit`-approximated statement count (`<=`[`MAX_SHAPE_STMTS`]).
    /// The cross-leaf caps are expressed in this unit; it is **not** a printed width.
    pub scored: int4,
    /// The [`printed_shape`] comma-chain width (`<=` the policy level's cap — 5 at
    /// `on`, 9 at `wide`, the same budget Rule A enforces).
    pub printed: int4,
}

/// `KUNA_CONDFOLD_DEBUG=ops`: also list every op [`printed_shape`] charges, so a
/// measured width can be reconciled against the emitted comma chain op by op.  This
/// is the facility the consolidation's reviewer had to reconstruct by hand.
///
/// The listing is exactly as long as the width the same walk reports — every charge
/// goes through `printed_shape`'s `charge!`, terminal CBRANCH included.  It did not
/// used to: the CBRANCH was charged at its own early `continue`, so a block measured
/// at 6 listed 5 ops and the trace could not be reconciled at all.
fn trace_ops() -> bool {
    std::env::var_os("KUNA_CONDFOLD_DEBUG").is_some_and(|v| v == "ops")
}

/// Trace a Rule-B decline under `KUNA_CONDFOLD_DEBUG=1` (stderr; use `decomp_dbg`
/// directly, the `kuna` CLI drops the child's stderr).
fn trace(bb: BlockId, reason: &str) {
    if std::env::var_os("KUNA_CONDFOLD_DEBUG").is_some() {
        eprintln!("[condfold] decline {bb:?}: {reason}");
    }
}

/// Trace an admitted relaxed fold under `KUNA_CONDFOLD_DEBUG=1` (stderr), naming the
/// engine that took it — `CollapseStructure` (§8.1) or `region_structurer` (§8.2) —
/// and which admission rule fired.  Both engines carry the identical gate, and which
/// one runs on a given function depends on whether the region structurer converges,
/// so this is how a witness for either half is confirmed.
pub fn trace_admit_fold(engine: &str, rule: &str, bl: BlockId, orblock: BlockId) {
    if std::env::var_os("KUNA_CONDFOLD_DEBUG").is_some() {
        eprintln!("[condfold] {engine}/{rule}: fold {bl:?} with complex {orblock:?}");
    }
}

//===========================================================================
// Shared guards + precompute
//===========================================================================

/// Does any buffered analysis comment land on an address this block's ops cover?
///
/// `emit_basic_block_ops` calls `emit_comment_group` only when `COMMA_SEPARATE` is
/// clear, so a block folded into a condition operand loses its comments entirely.
/// Rather than silently drop a `WARNING: ...` line, decline the fold.
///
/// **Any** buffered comment at a covered address declines, whatever its
/// `comment_type`.  Today only `warning` and `warningheader` ever reach
/// `pending_comments`, and only `warning` is emitted inside the block, so filtering
/// to `warning` would be behaviourally identical *right now* — but that equivalence
/// rests on an invariant nothing in the comment subsystem promises, and the cost of
/// being wrong is a silently deleted comment.  The guard is deliberately the broader
/// one (this is what PR #194 shipped, and the consolidation had narrowed it).
///
/// **Known limitation** (see the module docs): this reads the buffer as it stands
/// *at structuring time*.  An advisory produced by a later pass is not covered and
/// can still be lost.
fn block_carries_comment(data: &Funcdata, ops: &[OpId]) -> bool {
    let pending = data.pending_comments_ref();
    if pending.is_empty() {
        return false;
    }
    for &op in ops {
        let ad = match data.obank().get(op) {
            Some(o) => o.get_addr().clone(),
            None => continue,
        };
        for (_tp, cad, _) in pending {
            if *cad == ad {
                return true;
            }
        }
    }
    false
}

/// The precomputed condfold admission sets, keyed by the **bblocks** `BlockBasic` id
/// each structuring `BlockCopy`'s `copy` pointer references — a mirror of the
/// `complex_blocks` precomputation both structurers already run (the structuring
/// graph is a `BlockCopy` mirror without op ownership, so the verdicts have to be
/// computed against the live `bblocks` op lists).
#[derive(Debug, Default, Clone)]
pub struct CondFoldSets {
    /// Rule A verdicts: blocks with a bounded, branch-free, comment-free printed
    /// prefix ahead of the trailing CBRANCH.
    pub prefix: BTreeSet<BlockId>,
    /// Rule B verdicts: blocks passing the statement-shape allowlist *and* the
    /// shared printed-width budget, mapped to both of their measures.
    pub shape: BTreeMap<BlockId, ShapeVerdict>,
}

impl CondFoldSets {
    /// Is the option off (nothing admitted)?  Both gate disjuncts short-circuit on
    /// this, so an empty set is exactly upstream behavior.
    pub fn is_empty(&self) -> bool {
        self.prefix.is_empty() && self.shape.is_empty()
    }
}

/// Precompute both admission rules' verdicts for every `BlockBasic` in the function.
///
/// `max_stmts` is the `Architecture::cond_fold` policy cap: **0 means the option is
/// off**, in which case this returns empty sets without touching the graph, so the
/// whole feature costs nothing (and every gate disjunct is dead).  It is the
/// printed-width budget for **both** rules — the level moves what either rule will
/// let through, never only Rule A's.
pub fn compute_condfold_sets(data: &Funcdata, max_stmts: int4) -> CondFoldSets {
    let mut out = CondFoldSets::default();
    if max_stmts <= 0 {
        return out;
    }
    let debug = std::env::var_os("KUNA_CONDFOLD_DEBUG").is_some();
    let nbb = data.bblocks_get_size();
    for i in 0..nbb {
        let bb = data.bblocks_get_block(i);
        if prefix_eligible(data, bb, max_stmts) {
            if debug {
                let w = printed_shape(data, &data.bb_ops(bb))
                    .map(|s| s.stmts)
                    .unwrap_or(-1);
                eprintln!(
                    "[condfold] admit(prefix) {:?} @ {:#x}: prints {} (budget {})",
                    bb,
                    data.bblocks_block_start(bb).get_offset(),
                    w,
                    max_stmts
                );
            }
            out.prefix.insert(bb);
        }
        if let Some(verdict) = shape_score(data, bb, max_stmts) {
            if debug {
                // Report BOTH measures: the reviewer's finding was a block that
                // *scored* 2 and *rendered* 7, which this line used to hide.
                eprintln!(
                    "[condfold] admit(shape) {:?} @ {:#x}: scored {}, prints {} (budget {})",
                    bb,
                    data.bblocks_block_start(bb).get_offset(),
                    verdict.scored,
                    verdict.printed,
                    max_stmts
                );
            }
            out.shape.insert(bb, verdict);
        }
    }
    out
}

//===========================================================================
// Option parse (`condfold off|on|wide`).
//===========================================================================

/// The `condfold off|on|wide` ArchOption (angr Phoenix `MultiStatementExpression`
/// short-circuit relaxation).
///
/// The parsed value is the **shared printed-width budget** (both rules), stored in
/// `Architecture::cond_fold`; it doubles as the on/off sentinel for the whole option:
///
/// | value  | budget | meaning |
/// |--------|--------|---------|
/// | `off`  | 0   | the fold is never relaxed — byte-identical to upstream |
/// | `on`   | [`MAX_PREFIX_STMTS_ANGR`] (5) | angr's own statement threshold |
/// | `wide` | [`MAX_PREFIX_STMTS_WIDE`] (9) | same guards, budget raised to absorb kuna's finer printed-statement granularity |
pub struct OptionCondFold;

impl OptionCondFold {
    /// The option name.
    pub const NAME: &'static str = "condfold";

    /// Parse `off`/`on`/`wide` into the shared printed-width budget (0 = the whole
    /// option off) + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((
                0,
                "Short-circuit fold across a complex sibling turned off".to_string(),
            )),
            // Empty parameter reads as `on`, matching `options::on_or_off`.
            "on" | "" => Ok((
                MAX_PREFIX_STMTS_ANGR,
                format!(
                    "Short-circuit fold across a complex sibling turned on \
                     (angr parity: operand prints <={MAX_PREFIX_STMTS_ANGR} comma \
                     elements, plus either the bounded-prefix rule \
                     (<={MAX_PREFIX_ROOT_CALLS} statement-root call) or the \
                     statement-shape allowlist)"
                ),
            )),
            "wide" => Ok((
                MAX_PREFIX_STMTS_WIDE,
                format!(
                    "Short-circuit fold across a complex sibling turned on, wide \
                     (operand prints <={MAX_PREFIX_STMTS_WIDE} comma elements, plus \
                     either the bounded-prefix rule (<={MAX_PREFIX_ROOT_CALLS} \
                     statement-root call) or the statement-shape allowlist)"
                ),
            )),
            _ => Err(kuna_base::error::KunaError::parse(
                "Must specify one of off, on, wide",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_matches_the_angr_policy_constants() {
        // angr PhoenixStructurer._multistmtexpr_stmt_threshold default (phoenix.py:103)
        assert_eq!(MAX_PREFIX_STMTS_ANGR, 5);
        // angr MultiStmtExprMode.MAX_ONE_CALL (phoenix.py:73-80, ctor default :102).
        // The *value* matches; the counting does not — `prefix_eligible` charges only
        // statement-root calls (see MAX_PREFIX_ROOT_CALLS' docs), so a folded operand
        // can render more than one call when the extras are implied.
        assert_eq!(MAX_PREFIX_ROOT_CALLS, 1);
        // `wide` only ever loosens the shared printed-width budget, never a call cap,
        // never the shape allowlist, never a cross-leaf cap.
        assert!(MAX_PREFIX_STMTS_WIDE > MAX_PREFIX_STMTS_ANGR);
    }

    #[test]
    fn the_width_budget_is_shared_by_both_rules() {
        // The consolidation's reviewer found Rule B admitting a block that scored 2
        // and rendered 7 comma elements, so `on` emitted the option's WIDEST operands.
        // `shape_score` now takes the same `max_stmts` budget `prefix_eligible` does,
        // measured by the same `printed_shape` walk.  These assertions pin the two
        // facts that made that bug possible, so it cannot silently return:
        //   1. the scored cap is NOT a width cap and is far below the width budget;
        //   2. the budget an operator sets is the width one.
        assert!(MAX_SHAPE_STMTS < MAX_PREFIX_STMTS_ANGR);
        assert_eq!(OptionCondFold.apply("on").expect("on").0, MAX_PREFIX_STMTS_ANGR);
        assert_eq!(
            OptionCondFold.apply("wide").expect("wide").0,
            MAX_PREFIX_STMTS_WIDE
        );
    }

    #[test]
    fn rule_b_caps_are_tighter_per_block_than_rule_a() {
        // Rule B admits a *shape*, not a length: its per-block scored budget is
        // deliberately far below the shared width budget, and it pays for that with a
        // nested-operand recursion Rule A does not have.
        assert!(MAX_SHAPE_STMTS < MAX_PREFIX_STMTS_ANGR);
        // Rule B counts every call, so its cap is allowed to be higher than Rule A's
        // statement-root-only count.
        assert!(MAX_SHAPE_CALLS >= MAX_PREFIX_ROOT_CALLS);
        // The cross-leaf caps must at least admit the #56 target (3 leaves).
        assert!(MAX_JOIN_LEAVES >= 3);
        assert!(MAX_JOIN_TOTAL_STMTS >= MAX_SHAPE_STMTS);
    }

    #[test]
    fn element_id_is_unique_in_the_41xx_range() {
        assert_eq!(ELEM_CONDFOLD.get_id(), 4113);
        assert_eq!(ELEM_CONDFOLD.get_name(), "condfold");
    }

    #[test]
    fn empty_sets_mean_the_option_is_off() {
        let s = CondFoldSets::default();
        assert!(s.is_empty());
    }

    #[test]
    fn option_parses_off_on_and_wide_into_the_width_budget() {
        let (v, msg) = OptionCondFold.apply("on").expect("on parses");
        assert_eq!(v, MAX_PREFIX_STMTS_ANGR);
        assert!(msg.contains("on"));
        // An empty parameter reads as `on`, matching `options::on_or_off`.
        assert_eq!(
            OptionCondFold.apply("").expect("empty parses").0,
            MAX_PREFIX_STMTS_ANGR
        );
        let (v, msg) = OptionCondFold.apply("wide").expect("wide parses");
        assert_eq!(v, MAX_PREFIX_STMTS_WIDE);
        assert!(msg.contains("wide"));
        // `off` is the sentinel 0 cap: `compute_condfold_sets` short-circuits on it
        // and every gate disjunct stays dead.
        let (v, msg) = OptionCondFold.apply("off").expect("off parses");
        assert_eq!(v, 0);
        assert!(msg.contains("off"));
        assert!(OptionCondFold.apply("maybe").is_err());
        assert!(OptionCondFold.apply("7").is_err());
    }

    //=======================================================================
    // The two scorers, on REAL op lists
    //
    // Everything above pins constants and the option parse; the tests below run
    // `printed_shape` / `prefix_eligible` / `shape_score` over hand-built
    // `BlockBasic`s carrying live p-code, with the real `type_op_for` flag table
    // (so `is_call`/`is_branch`/`is_marker`/`is_flow_break` mean what they mean
    // in the engine).  The load-bearing one is
    // `two_statement_root_calls_are_rule_bs_alone`, the unit-scale form of the
    // `tests/stages/ghangr-condfold-ruleb.xml` witness.
    //=======================================================================

    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use crate::context::{ArchContext, VarnodeId};

    /// A minimal `Funcdata` over a `const`/`unique`/`ram` space manager.
    fn scorer_fd() -> Funcdata {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        let glb = Rc::new(ArchContext::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let entry = Address::new(ram, 0x1000);
        Funcdata::new("g", "g", glb, entry, 0x10000000, 0x40).unwrap()
    }

    fn ram_at(fd: &Funcdata, off: u64) -> Address {
        Address::new(
            Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()),
            off,
        )
    }

    /// A fresh `BlockBasic` covering `off`, with `outs` successors wired.  A
    /// short-circuit sibling is a 2-out condition block by construction, so
    /// `shape_score` refuses anything else out of hand.
    fn scorer_block(fd: &mut Funcdata, off: u64, outs: int4) -> BlockId {
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        let beg = ram_at(fd, off);
        let end = ram_at(fd, off);
        fd.set_basic_block_range(bl, &beg, &end);
        for _ in 0..outs {
            let sink = fd.bblocks_mut().new_block_basic(root);
            fd.bblocks_mut().add_edge(bl, sink);
        }
        bl
    }

    /// Append `opc` with `inputs` distinct constant operands to `bl`; no output.
    fn push_void(fd: &mut Funcdata, bl: BlockId, off: u64, opc: OpCode, inputs: int4) -> OpId {
        let addr = ram_at(fd, off);
        let op = fd.new_op(inputs, addr);
        fd.op_set_opcode(op, crate::typeop::type_op_for(opc));
        for slot in 0..inputs {
            let c = fd.new_constant(4, off + slot as u64);
            fd.op_set_input(op, c, slot).expect("input slot");
        }
        fd.op_insert_end(op, bl);
        op
    }

    /// Same, with a fresh `unique` output nothing consumes — which is what the
    /// `calc_explicit` approximation calls *printed* (`has_no_descend`), and what
    /// the printer emits as its own comma element (not implied).
    fn push_out(fd: &mut Funcdata, bl: BlockId, off: u64, opc: OpCode, inputs: int4) -> VarnodeId {
        let op = push_void(fd, bl, off, opc, inputs);
        fd.new_unique_out(4, op).expect("unique out")
    }

    /// Append the block's terminal `CBRANCH`, reading `cond` in slot 1.
    fn push_cbranch(fd: &mut Funcdata, bl: BlockId, off: u64, cond: VarnodeId) -> OpId {
        let addr = ram_at(fd, off);
        let op = fd.new_op(2, addr);
        fd.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CBRANCH));
        let target = fd.new_constant(8, 0x9000);
        fd.op_set_input(op, target, 0).expect("branch target");
        fd.op_set_input(op, cond, 1).expect("condition");
        fd.op_insert_end(op, bl);
        op
    }

    /// Splice an interior op in ahead of `before`.  `op_insert_end` cannot place
    /// one: like C++ `Funcdata::opInsertEnd` it steps back over a trailing flow
    /// break, so appending after a branch is impossible by construction.
    fn push_before(
        fd: &mut Funcdata,
        bl: BlockId,
        off: u64,
        opc: OpCode,
        inputs: int4,
        before: OpId,
    ) {
        let addr = ram_at(fd, off);
        let op = fd.new_op(inputs, addr);
        fd.op_set_opcode(op, crate::typeop::type_op_for(opc));
        for slot in 0..inputs {
            let c = fd.new_constant(4, off + slot as u64);
            fd.op_set_input(op, c, slot).expect("input slot");
        }
        fd.op_insert(op, bl, Some(before));
    }

    #[test]
    fn printed_shape_charges_the_terminal_cbranch() {
        // `STORE; v = INT_ADD(..); if (v) ...` renders as a three-element comma
        // chain: the store, the assignment, and the condition itself.  The
        // CBRANCH is the element `KUNA_CONDFOLD_DEBUG=ops` used to omit.
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x2000, 2);
        push_void(&mut fd, bl, 0x2000, OpCode::CPUI_STORE, 3);
        let v = push_out(&mut fd, bl, 0x2004, OpCode::CPUI_INT_ADD, 2);
        push_cbranch(&mut fd, bl, 0x2008, v);
        let shape = printed_shape(&fd, &fd.bb_ops(bl)).expect("renders as an expression");
        assert_eq!(shape.stmts, 3, "STORE + assignment + the CBRANCH condition");
        assert_eq!(shape.root_calls, 0);
    }

    #[test]
    fn printed_shape_skips_markers_and_the_bare_branch() {
        // The printer emits neither an SSA marker nor an unconditional BRANCH, so
        // neither is an element of the comma chain.
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x2000, 2);
        push_out(&mut fd, bl, 0x2000, OpCode::CPUI_INDIRECT, 2);
        let v = push_out(&mut fd, bl, 0x2008, OpCode::CPUI_INT_ADD, 2);
        let cb = push_cbranch(&mut fd, bl, 0x200c, v);
        push_before(&mut fd, bl, 0x2004, OpCode::CPUI_BRANCH, 1, cb);
        let shape = printed_shape(&fd, &fd.bb_ops(bl)).expect("renders as an expression");
        assert_eq!(shape.stmts, 2, "only the assignment and the condition print");
    }

    #[test]
    fn printed_shape_refuses_what_cannot_render_as_an_expression() {
        // No terminal CBRANCH: there is no condition to fold.
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x2000, 2);
        push_void(&mut fd, bl, 0x2000, OpCode::CPUI_CALL, 1);
        assert!(printed_shape(&fd, &fd.bb_ops(bl)).is_none());

        // A non-terminal branch survives the skips: angr's
        // `_build_multistatementexpr_statements` refuses the same shape, and a
        // `goto` is not an expression.
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x3000, 2);
        let c0 = push_out(&mut fd, bl, 0x3000, OpCode::CPUI_INT_EQUAL, 2);
        let cb = push_cbranch(&mut fd, bl, 0x3004, c0);
        push_before(&mut fd, bl, 0x3002, OpCode::CPUI_BRANCHIND, 1, cb);
        assert!(printed_shape(&fd, &fd.bb_ops(bl)).is_none());
    }

    /// Build the unit-scale form of the `tests/stages/ghangr-condfold-ruleb.xml`
    /// middle guard: `su_log(); x = su_euid(); if (x != wu)`, i.e. TWO calls that
    /// each print as their own comma element.
    fn two_call_guard(fd: &mut Funcdata, off: u64) -> BlockId {
        let bl = scorer_block(fd, off, 2);
        // su_log() -- a void call.
        push_void(fd, bl, off, OpCode::CPUI_CALL, 1);
        // x = su_euid() -- an output the block does not consume (the failure arm
        // does), so it is neither implied nor in-block-only: it prints.
        push_out(fd, bl, off + 4, OpCode::CPUI_CALL, 1);
        // if (x != wu) -- the compare feeds only the CBRANCH, so it is inlined.
        let cond = push_out(fd, bl, off + 8, OpCode::CPUI_INT_EQUAL, 2);
        push_cbranch(fd, bl, off + 12, cond);
        bl
    }

    #[test]
    fn two_statement_root_calls_are_rule_bs_alone() {
        // THE separation between the two admission rules, on a real op list.
        // Rule A's call cap is 1 and does NOT move with the policy level; Rule B
        // allows 2.  So this block is outside Rule A at `on` AND at `wide`, and
        // inside Rule B at both -- which is exactly why the stage witness built on
        // this shape fails if Rule B is deleted.
        let mut fd = scorer_fd();
        let bl = two_call_guard(&mut fd, 0x2000);

        let shape = printed_shape(&fd, &fd.bb_ops(bl)).expect("renders as an expression");
        assert_eq!(shape.stmts, 4, "two calls, the compare, and the condition");
        assert_eq!(shape.root_calls, 2, "both calls print as their own element");

        assert!(
            !prefix_eligible(&fd, bl, MAX_PREFIX_STMTS_ANGR),
            "Rule A refuses a second statement-root call at `on`"
        );
        assert!(
            !prefix_eligible(&fd, bl, MAX_PREFIX_STMTS_WIDE),
            "and at `wide` -- the call cap is not a level knob"
        );

        let verdict = shape_score(&fd, bl, MAX_PREFIX_STMTS_ANGR).expect("Rule B admits");
        assert_eq!(verdict.printed, 4);
        assert_eq!(
            verdict.scored, 2,
            "both calls score; the compare is inlined into the condition"
        );

        // The precompute agrees, and records the block under Rule B only.
        let sets = compute_condfold_sets(&fd, MAX_PREFIX_STMTS_ANGR);
        assert!(!sets.prefix.contains(&bl));
        assert_eq!(sets.shape.get(&bl), Some(&verdict));
    }

    #[test]
    fn the_shared_width_budget_binds_rule_b_too() {
        // The consolidation's fix: Rule B is measured against the SAME budget Rule
        // A is.  The four-element guard above is admitted at `on` (5) and refused
        // below it, even though its `calc_explicit` score is unchanged at 2.
        let mut fd = scorer_fd();
        let bl = two_call_guard(&mut fd, 0x2000);
        assert!(shape_score(&fd, bl, 4).is_some(), "exactly at the budget");
        assert!(shape_score(&fd, bl, 3).is_none(), "one over: refused");
    }

    #[test]
    fn shape_score_refuses_the_shapes_outside_its_allowlist() {
        // A CALLOTHER: userop rendering can be multi-line or special-cased.
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x2000, 2);
        push_void(&mut fd, bl, 0x2000, OpCode::CPUI_CALLOTHER, 1);
        let cond = push_out(&mut fd, bl, 0x2004, OpCode::CPUI_INT_EQUAL, 2);
        push_cbranch(&mut fd, bl, 0x2008, cond);
        assert!(shape_score(&fd, bl, MAX_PREFIX_STMTS_WIDE).is_none());

        // An interior flow break (a RETURN before the tail).
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x3000, 2);
        let cond = push_out(&mut fd, bl, 0x3000, OpCode::CPUI_INT_EQUAL, 2);
        let cb = push_cbranch(&mut fd, bl, 0x3008, cond);
        push_before(&mut fd, bl, 0x3004, OpCode::CPUI_RETURN, 1, cb);
        assert!(shape_score(&fd, bl, MAX_PREFIX_STMTS_WIDE).is_none());

        // A one-out block is not a short-circuit sibling at all.
        let mut fd = scorer_fd();
        let bl = scorer_block(&mut fd, 0x4000, 1);
        let cond = push_out(&mut fd, bl, 0x4000, OpCode::CPUI_INT_EQUAL, 2);
        push_cbranch(&mut fd, bl, 0x4004, cond);
        assert!(shape_score(&fd, bl, MAX_PREFIX_STMTS_WIDE).is_none());
    }

    #[test]
    fn the_precompute_is_dead_at_budget_zero() {
        // `condfold off` is the 0 sentinel: no block is admitted under EITHER rule,
        // so every gate disjunct is dead and the emitted C is upstream's.
        let mut fd = scorer_fd();
        let bl = two_call_guard(&mut fd, 0x2000);
        assert!(shape_score(&fd, bl, MAX_PREFIX_STMTS_ANGR).is_some());
        assert!(compute_condfold_sets(&fd, 0).is_empty());
    }
}
