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
//! # Two admission rules (the predicate is their UNION)
//!
//! A sibling is admissible when **either** rule accepts it.  They were derived
//! independently against different targets and neither subsumes the other; do not
//! intersect them.
//!
//! ## Rule A — bounded prefix (from #193, closes #39)
//!
//! [`prefix_eligible`]: the sibling is a [`BlockCopy`](crate::block::BlockKind::Copy)
//! of ONE `BlockBasic` whose **printed-statement prefix** ahead of the trailing
//! CBRANCH is bounded ([`MAX_PREFIX_STMTS_ANGR`] at `on`, [`MAX_PREFIX_STMTS_WIDE`]
//! at `wide`; at most [`MAX_PREFIX_ROOT_CALLS`] statement-root call), branch-free,
//! and comment-free.  The walk mirrors the printer's own skip rules, so the caps
//! govern exactly what appears in the emitted comma expression.  A `BlockList`/
//! `BlockIf`/`BlockCondition` sibling is never accepted by this rule — such an
//! operand can render braces, multiple lines, or a label inside the parentheses,
//! i.e. invalid C.
//!
//! ## Rule B — statement shape (from #194, closes #56)
//!
//! [`shape_score`]: an explicit **statement-shape allowlist** — every op is a marker
//! (never printed), an op with an output varnode, a void `CALL`/`CALLIND`, a `STORE`,
//! or the single terminal `CBRANCH`.  Anything else declines: `RETURN`, a second
//! branch, `BRANCHIND`, `CALLOTHER` (userop rendering can be multi-line or
//! special-cased), a no-return call.  At most [`MAX_SHAPE_STMTS`] *scored* statements
//! and [`MAX_SHAPE_CALLS`] calls per block, and no comment.
//!
//! Rule B additionally admits a **nested `BlockCondition`** as the sibling — which
//! Rule A never does — because that is what a guard *cascade* needs: `newbury::main`
//! folds `(B1,B2)` first and then takes the resulting Condition as the sibling of
//! `B0`.  `PrintC::emit_block_condition` emits *both* sub-blocks of a nested
//! Condition under `COMMA_SEPARATE`, so every leaf must itself be Rule-B admissible
//! (the recursion in `CollapseStructure::is_shape_foldable`), and the fold site
//! enforces the expression-size caps [`MAX_JOIN_LEAVES`] / [`MAX_JOIN_TOTAL_STMTS`]
//! (`collapse_conditions` is a fixpoint loop with no natural bound, so without them a
//! long cascade would fold into an unreadable monster).
//!
//! Rule B scores a statement with the same `Varnode::calc_explicit` approximation
//! [`Funcdata::bb_is_complex`](crate::funcdata::Funcdata::bb_is_complex) uses (addr-tied
//! / no-descend / used-outside-block / over `max_implied_ref` refs ⇒ printed), never by
//! reading `Varnode::is_explicit()`: structuring runs before `ActionMarkExplicit`, so
//! those flags are not yet meaningful.  This is Ghidra's own approximation and it is
//! *not* exact in either direction — a varnode the final `ActionMarkExplicit` makes
//! explicit can score as implied here, so a block admitted at the cap can render one
//! statement wider than the nominal budget (bash `get_random` is the observed case:
//! scored 2, renders 3).  That is readability slack, never a correctness one —
//! `COMMA_SEPARATE` emits every op either way.
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
//!   aggregate is a clear net win.  Measured regressions: `clientloop.o::process_escapes`
//!   26 → 28, `newbury::network_init` 3 → 4, `newbury::lshpack_enc_encode` 16 → 17.
//! * **A later-produced advisory comment can be lost.**  Both rules decline a block
//!   that already carries a comment, but the check only sees comments *buffered in
//!   `Funcdata` at structuring time*.  `COMMA_SEPARATE` suppresses `emit_comment_group`,
//!   so an advisory produced by a pass that runs after structuring can still vanish.
//!   Reproduced: `newbury::main`'s `/* WARNING: branchflip: ... */` is present at `off`
//!   and absent at `on`.  These are kuna advisory annotations, never a correctness
//!   signal, and the emitted C is unaffected.
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

/// Rule A's statement cap at `option condfold on` — **angr parity**: angr's
/// `_multistmtexpr_stmt_threshold` (`PhoenixStructurer.__init__`, phoenix.py:103)
/// defaults to 5 non-label AIL statements.
pub const MAX_PREFIX_STMTS_ANGR: int4 = 5;

/// Rule A's statement cap at `option condfold wide`.
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
/// as 7 kuna statements, so the angr-parity cap declines a fold angr makes.  `wide`
/// raises the cap to 9 to cover that skew.  It is a *readability* knob only — every
/// soundness and rendering-validity guard is identical at both levels, and it moves
/// **only** Rule A's cap: Rule B's [`MAX_SHAPE_STMTS`] and the expression-size caps
/// are the same at `on` and `wide`.
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

/// Rule B's per-block cap on conservatively-scored printed statements.  Ghidra's own
/// budget admits 1 (its `statement > 2` counts the branch); angr's
/// `multistmtexpr_stmt_threshold` is 5.  2 is the tightened middle that admits the
/// `v = geteuid(); v == getuid()` shape and little else.
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
pub const MAX_JOIN_TOTAL_STMTS: int4 = 4;

//===========================================================================
// Rule A — bounded prefix (PR #193, proposal #39)
//===========================================================================

/// **Rule A.**  Is the **bblocks** `BlockBasic` `bb` an acceptable right operand of a
/// folded short-circuit condition (angr `_should_use_multistmtexprs` +
/// `_build_multistatementexpr_statements`)?
///
/// Walks the block's live op list with the *same* skip rules
/// [`PrintC::emit_basic_block_ops`](crate::printc) applies, so the caps govern
/// exactly what will appear in the emitted comma expression:
///
/// * `not_printed()` ops (which includes every SSA marker) are skipped;
/// * an op whose output varnode is *implied* is skipped (it is inlined into its
///   consumer's expression, not printed as a statement);
/// * an unconditional `CPUI_BRANCH` is skipped (the printer never emits it);
/// * the block's terminal `CPUI_CBRANCH` is the condition, not a prefix statement,
///   so it is skipped for the purpose of the prefix caps but the *statement* count
///   still charges for it (it is what the operand ultimately renders as).
///
/// `max_stmts` is the policy's statement cap ([`MAX_PREFIX_STMTS_ANGR`] for
/// `option condfold on`, [`MAX_PREFIX_STMTS_WIDE`] for `wide`).
///
/// Declines when:
///
/// * more than `max_stmts` printed statements or more than [`MAX_PREFIX_ROOT_CALLS`]
///   **statement-root** calls (see that constant for what the count does and does not
///   cover);
/// * any non-terminal branch op survives the skips (a mid-block conditional jump —
///   angr `_build_multistatementexpr_statements` refuses these, and a `goto` is not
///   an expression);
/// * the block does not end in a `CPUI_CBRANCH` (nothing to fold);
/// * an analysis comment is attached at an address the block covers (the printer
///   suppresses `emit_comment_group` under `COMMA_SEPARATE`, so the comment would
///   be silently dropped).
pub fn prefix_eligible(data: &Funcdata, bb: BlockId, max_stmts: int4) -> bool {
    let ops = data.bb_ops(bb);
    if ops.is_empty() {
        return false;
    }
    // The block must end in a conditional branch (this is the `orblock` of a
    // 2-out short-circuit schema; the caller already checked `size_out() == 2`).
    let tail = *ops.last().expect("prefix_eligible: non-empty");
    match data.obank().get(tail).map(|o| o.code()) {
        Some(OpCode::CPUI_CBRANCH) => {}
        _ => return false,
    }

    let mut stmts: int4 = 0;
    let mut calls: int4 = 0;
    for (idx, &op) in ops.iter().enumerate() {
        let o = match data.obank().get(op) {
            Some(o) => o,
            None => return false,
        };
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
            // The terminal CBRANCH is the operand's condition.
            if idx + 1 == ops.len() {
                stmts += 1;
                if stmts > max_stmts {
                    return false;
                }
                continue;
            }
            // Any other branch (a mid-block conditional jump, a BRANCHIND, a
            // CALLOTHER-flavoured flow break) cannot render as an expression.
            return false;
        }
        // printc.rs emit_basic_block_ops: ops whose out varnode is implied are
        // inlined into their consumer and never printed as a statement.  This skip
        // is *why* MAX_PREFIX_ROOT_CALLS counts statement roots only.
        if let Some(out) = o.get_out() {
            if data.vbank().get(out).map(|v| v.is_implied()).unwrap_or(false) {
                continue;
            }
        }
        if o.is_call() {
            calls += 1;
            if calls > MAX_PREFIX_ROOT_CALLS {
                return false;
            }
        }
        stmts += 1;
        if stmts > max_stmts {
            return false;
        }
    }
    if stmts == 0 {
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
/// Returns `Some(statement_count)` when the block satisfies the allowlist, the
/// per-block caps and the comment guard (see the module docs) and may therefore be
/// absorbed into a short-circuit condition as a comma-separated multi-statement
/// operand; `None` when any precondition fails.
///
/// The statement count uses the same `Varnode::calc_explicit` approximation as
/// [`Funcdata::bb_is_complex`](crate::funcdata::Funcdata::bb_is_complex) — it
/// deliberately does **not** read `Varnode::is_explicit()`/`is_implied()`, which are
/// not yet computed when structuring runs.  Like Ghidra's own use of it, the
/// approximation is inexact in both directions.
fn shape_score(data: &Funcdata, bb: BlockId) -> Option<int4> {
    // Decline with a reason, traced to stderr under `KUNA_CONDFOLD_DEBUG=1` (run
    // `decomp_dbg` directly to see it; the `kuna` CLI drops the child's stderr).
    // Declining is always correct: the fold simply does not happen and today's
    // goto+label output stands.
    macro_rules! decline {
        ($reason:literal) => {{
            trace(bb, $reason);
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
            decline!("over MAX_SHAPE_STMTS");
        }
    }

    // A comment anchored inside this block would be dropped by the COMMA_SEPARATE
    // printer path.  Decline rather than lose it.
    if block_carries_comment(data, &ops) {
        decline!("carries a comment the COMMA_SEPARATE printer would drop");
    }

    Some(statements)
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
/// Only the *instruction* comments matter (`comment_type::warning`); a
/// `warningheader` is emitted with the prototype, not inside the block.
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
        for (tp, cad, _) in pending {
            if *tp == crate::architecture::comment_type::warning && *cad == ad {
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
    /// Rule B verdicts: blocks passing the statement-shape allowlist, mapped to
    /// their conservatively-scored printed-statement count.
    pub shape: BTreeMap<BlockId, int4>,
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
/// whole feature costs nothing (and every gate disjunct is dead).
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
            out.prefix.insert(bb);
        }
        if let Some(stmts) = shape_score(data, bb) {
            if debug {
                eprintln!(
                    "[condfold] admit(shape) {:?} @ {:#x}: {} statement(s)",
                    bb,
                    data.bblocks_block_start(bb).get_offset(),
                    stmts
                );
            }
            out.shape.insert(bb, stmts);
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
/// The parsed value is Rule A's **statement cap**, stored in
/// `Architecture::cond_fold`; it doubles as the on/off sentinel for the whole option
/// (Rule B is enabled at any non-zero level and its own caps do not move):
///
/// | value  | cap | meaning |
/// |--------|-----|---------|
/// | `off`  | 0   | the fold is never relaxed — byte-identical to upstream |
/// | `on`   | [`MAX_PREFIX_STMTS_ANGR`] (5) | angr's own `MAX_ONE_CALL` policy |
/// | `wide` | [`MAX_PREFIX_STMTS_WIDE`] (9) | same guards, Rule A's cap raised to absorb kuna's finer printed-statement granularity |
pub struct OptionCondFold;

impl OptionCondFold {
    /// The option name.
    pub const NAME: &'static str = "condfold";

    /// Parse `off`/`on`/`wide` into Rule A's statement cap (0 = the whole option off)
    /// + a confirmation message.
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
                     (angr parity: bounded prefix <={MAX_PREFIX_STMTS_ANGR} statements / \
                     <={MAX_PREFIX_ROOT_CALLS} statement-root call, or the statement-shape \
                     allowlist)"
                ),
            )),
            "wide" => Ok((
                MAX_PREFIX_STMTS_WIDE,
                format!(
                    "Short-circuit fold across a complex sibling turned on, wide \
                     (bounded prefix <={MAX_PREFIX_STMTS_WIDE} statements / \
                     <={MAX_PREFIX_ROOT_CALLS} statement-root call, or the statement-shape \
                     allowlist)"
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
    fn rule_a_on_matches_the_angr_policy_constants() {
        // angr PhoenixStructurer._multistmtexpr_stmt_threshold default (phoenix.py:103)
        assert_eq!(MAX_PREFIX_STMTS_ANGR, 5);
        // angr MultiStmtExprMode.MAX_ONE_CALL (phoenix.py:73-80, ctor default :102).
        // The *value* matches; the counting does not — `prefix_eligible` charges only
        // statement-root calls (see MAX_PREFIX_ROOT_CALLS' docs), so a folded operand
        // can render more than one call when the extras are implied.
        assert_eq!(MAX_PREFIX_ROOT_CALLS, 1);
        // `wide` only ever loosens Rule A's statement cap, never a call cap and never
        // any Rule B cap.
        assert!(MAX_PREFIX_STMTS_WIDE > MAX_PREFIX_STMTS_ANGR);
    }

    #[test]
    fn rule_b_caps_are_tighter_per_block_than_rule_a() {
        // Rule B admits a *shape*, not a length: its per-block statement budget is
        // deliberately far below Rule A's, and it pays for that with a nested-operand
        // recursion Rule A does not have.
        assert!(MAX_SHAPE_STMTS < MAX_PREFIX_STMTS_ANGR);
        // Rule B counts every call, so its cap is allowed to be higher than Rule A's
        // statement-root-only count.
        assert!(MAX_SHAPE_CALLS >= MAX_PREFIX_ROOT_CALLS);
        // The expression-size caps must at least admit the #56 target (3 leaves).
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
    fn option_parses_off_on_and_wide_into_the_statement_cap() {
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
}
