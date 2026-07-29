//! S8 **short-circuit condition folding across a non-trivial sibling block**
//! (`option condfold off|on|wide`, default-OFF) — a port of the angr Phoenix
//! `MultiStatementExpression` relaxation in
//! `angr/analyses/decompiler/structuring/phoenix.py`'s
//! `_match_acyclic_short_circuit_conditions` (types a-d).
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
//! back into the first arm's clause:
//!
//! ```text
//! if (es_match(es,i,':')) {
//! label_40242e:
//!     ... shared clause ...
//! }
//! else {
//!     result_idx = &v10;                              // the prefix that declines the fold
//!     if (es_match(es,v12,'=')) goto label_40242e;    // the crossing edge
//!     ...
//! }
//! ```
//!
//! angr does not decline on this shape.  `_match_acyclic_short_circuit_conditions_type_a`
//! (phoenix.py:2844, repeated at :2887/:2934/:2977) checks
//! `_is_single_statement_block(left)` and, when the sibling is *not* a single
//! statement, wraps the operand in an AIL `MultiStatementExpression` — a C **comma
//! expression** — instead of refusing:
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
//! # What this option does
//!
//! `condfold` reproduces exactly that relaxation, and **only** that relaxation.  It
//! adds a second, narrower disjunct at the single `is_complex(orblock)` decline
//! inside the short-circuit schema of *both* structurers
//! ([`CollapseStructure::rule_block_or`](crate::blockaction) and
//! [`RegionStructurer::try_block_or`](crate::p8_structure::region_structurer)):
//! a *complex* sibling is accepted anyway when it is a
//! [`BlockCopy`](crate::block::BlockKind::Copy) of a single `BlockBasic` that passes
//! [`condfold_eligible`] — a bounded, branch-free, comment-free printed-statement
//! prefix.  Every other precondition `ruleBlockOr` enforces is untouched.
//!
//! # Why no p-code moves
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
//!   test.
//!
//! So the *semantic* precondition set beyond `ruleBlockOr`'s own is empty.  Every
//! guard [`condfold_eligible`] adds is a **rendering-validity or quality** guard:
//!
//! 1. bounded printed-statement count ([`MAX_PREFIX_STMTS_ANGR`] at `on`,
//!    [`MAX_PREFIX_STMTS_WIDE`] at `wide`) and call count ([`MAX_PREFIX_CALLS`]) —
//!    `on` uses angr's own tuned statement threshold; an unbounded comma chain in an
//!    `if (...)` is less readable than the nested `if` it replaces.  Note the call
//!    cap counts **statement-root** calls only, so it is *not* an exact match for
//!    angr's `MAX_ONE_CALL`: [`condfold_eligible`] skips an op whose output varnode
//!    is *implied* before it reaches the `is_call` test, so a call inlined into the
//!    sibling's own condition expression is never charged.  A folded operand can
//!    therefore render more than one call — measured, `mv -O2 copy_internal` at
//!    `wide` renders both `cached_umask(...)` (a statement root, counted) and
//!    `fchmod(...)` (implied into the test, not counted).  This is a readability
//!    bound, not a soundness bound (see *Why no p-code moves* above), so the looser
//!    count cannot produce wrong C;
//! 2. no non-terminal branch op — mirrors angr's
//!    `_build_multistatementexpr_statements` refusing a mid-block `ConditionalJump`
//!    or any `Jump` (such an op renders a `goto`, which is not an expression);
//! 3. no analysis comment attached inside the block — `emit_basic_block_ops` only
//!    calls `emit_comment_group` when `COMMA_SEPARATE` is *clear*, so folding a
//!    block that carries a `WARNING:` comment would silently delete it.  Decline
//!    instead of losing information.
//!
//! Two further guards live at the call site (see
//! [`CollapseStructure::condfold_ok`](crate::blockaction)):
//!
//! 4. the sibling must be a `BlockCopy` of one `BlockBasic` — a `BlockList`/`BlockIf`/
//!    `BlockCondition` operand can render braces, multiple lines, or a label inside
//!    the parentheses, i.e. invalid C;
//! 5. the pre-existing `!orblock.is_interior_goto_target()` decline (never relaxed):
//!    `emit_block_copy` emits `emit_any_label_statement` *first*, so a labelled
//!    sibling would print `label_x:` inside an expression.
//!
//! Finally, a folded `BlockCondition` is force-marked complex (`condfolded`) so
//! `ruleBlockWhileDo` can never lift the comma chain into a `while(...)` header:
//! `BlockCondition::isComplex` delegates to sub-block 0, which would report the
//! *left* operand's trivial verdict for a node whose right operand is a comma chain.
//!
//! When the option is OFF the whole precompute and both disjuncts are dead and the
//! emitted C is byte-identical to `main`.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::BlockId;
use crate::funcdata::Funcdata;

/// Marshaling element `<condfold>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce, 4101 tailcalljump, 4102 regionstructure, 4103 noreturn_extern,
/// 4104 noreturn_externmatch, 4105 crossjumprevert, 4106 switchsharedcase,
/// 4107 ifelseflatten, 4108 taildup, 4109 dedupitetail, 4110 returndup,
/// 4111 earlyreturn, 4112 switchreturn are taken; this is 4113.  `condjoin`
/// (PR #194), developed in parallel with this one, takes 4114).
pub const ELEM_CONDFOLD: ElementId = ElementId::new("condfold", 4113);

/// The statement cap of `option condfold on` — **angr parity**: angr's
/// `_multistmtexpr_stmt_threshold` (`PhoenixStructurer.__init__`, phoenix.py:103)
/// defaults to 5 non-label AIL statements.
pub const MAX_PREFIX_STMTS_ANGR: int4 = 5;

/// The statement cap of `option condfold wide`.
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
/// soundness and rendering-validity guard is identical at both levels.
pub const MAX_PREFIX_STMTS_WIDE: int4 = 9;

/// Maximum number of **statement-root** calls in the folded sibling, at every policy
/// level.  angr's default `MultiStmtExprMode.MAX_ONE_CALL` policy
/// (`_should_use_multistmtexprs`, phoenix.py:3239) allows at most one, and a comma
/// chain with two printed call statements in an `if (...)` is unreadable regardless
/// of the statement count.
///
/// **What this actually counts — read before quoting it as angr parity.**
/// [`condfold_eligible`] walks the block with the printer's own skip rules, and the
/// *implied*-output skip runs **before** the `is_call` test.  A call whose result is
/// inlined into the sibling's condition expression is therefore never charged
/// against this cap.  The bound the code enforces is "at most one call printed as
/// its own comma-chain element", not "at most one call in the rendered operand": a
/// folded operand may render two or more calls when the extra ones are implied.
/// Measured on the aggregate sweep, 1 of the 46 new call-bearing folded operands
/// does exactly that — `mv -O2 copy_internal` at `wide` renders `cached_umask(...)`
/// (a statement root, counted) *and* `fchmod(...)` (implied into the test, not
/// counted).
///
/// This is a **readability** bound, not a soundness bound: the fold moves no p-code
/// and the short-circuit + comma sequencing argument in the module docs is
/// independent of how many calls appear, so the looser count cannot produce wrong C.
/// Charging implied calls too is a two-line reorder (hoist the `is_call` test above
/// the implied-output skip), but it *tightens* the gate and would change the
/// measured metric table, the aggregate sweep and the stage-test witness — see
/// PR #193 for that deliberately-open choice.
pub const MAX_PREFIX_CALLS: int4 = 1;

/// Is the **bblocks** `BlockBasic` `bb` an acceptable right operand of a folded
/// short-circuit condition (angr `_should_use_multistmtexprs` +
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
/// * more than `max_stmts` printed statements or more than [`MAX_PREFIX_CALLS`]
///   **statement-root** calls — because the implied-output skip above runs first, a
///   call inlined into the condition expression is not charged, so the rendered
///   operand can hold more than one call (see [`MAX_PREFIX_CALLS`]);
/// * any non-terminal branch op survives the skips (a mid-block conditional jump —
///   angr `_build_multistatementexpr_statements` refuses these, and a `goto` is not
///   an expression);
/// * the block does not end in a `CPUI_CBRANCH` (nothing to fold);
/// * an analysis comment is attached at an address the block covers (the printer
///   suppresses `emit_comment_group` under `COMMA_SEPARATE`, so the comment would
///   be silently dropped).
pub fn condfold_eligible(data: &Funcdata, bb: BlockId, max_stmts: int4) -> bool {
    let ops = data.bb_ops(bb);
    if ops.is_empty() {
        return false;
    }
    // The block must end in a conditional branch (this is the `orblock` of a
    // 2-out short-circuit schema; the caller already checked `size_out() == 2`).
    let tail = *ops.last().expect("condfold_eligible: non-empty");
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
        // inlined into their consumer and never printed as a statement.
        if let Some(out) = o.get_out() {
            if data.vbank().get(out).map(|v| v.is_implied()).unwrap_or(false) {
                continue;
            }
        }
        if o.is_call() {
            calls += 1;
            if calls > MAX_PREFIX_CALLS {
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

/// Does any buffered analysis comment land on an address this block's ops cover?
///
/// `emit_basic_block_ops` calls `emit_comment_group` only when `COMMA_SEPARATE` is
/// clear, so a block folded into a condition operand loses its comments entirely.
/// Rather than silently drop a `WARNING: ...` line, decline the fold.
///
/// Only the *instruction* comments matter (`comment_type::warning`); a
/// `warningheader` is emitted with the prototype, not inside the block.
fn block_carries_comment(data: &Funcdata, ops: &[crate::context::OpId]) -> bool {
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

/// Precompute the set of **bblocks** `BlockBasic` ids that are condfold-eligible
/// (the [`condfold_eligible`] verdict), keyed by the bblocks id each structuring
/// `BlockCopy`'s `copy` pointer references — a mirror of the `complex_blocks`
/// precomputation both structurers already run.
///
/// `max_stmts` is the `Architecture::cond_fold` policy cap: **0 means the option is
/// off**, in which case this returns an empty set without touching the graph, so
/// the whole feature costs nothing (and both gate disjuncts are dead).
pub fn compute_condfold_blocks(
    data: &Funcdata,
    max_stmts: int4,
) -> std::collections::BTreeSet<BlockId> {
    let mut out: std::collections::BTreeSet<BlockId> = std::collections::BTreeSet::new();
    if max_stmts <= 0 {
        return out;
    }
    let nbb = data.bblocks_get_size();
    for i in 0..nbb {
        let bb = data.bblocks_get_block(i);
        if condfold_eligible(data, bb, max_stmts) {
            out.insert(bb);
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
/// The parsed value is the **statement cap** stored in `Architecture::cond_fold`:
///
/// | value  | cap | meaning |
/// |--------|-----|---------|
/// | `off`  | 0   | the fold is never relaxed — byte-identical to upstream |
/// | `on`   | [`MAX_PREFIX_STMTS_ANGR`] (5) | angr's own `MAX_ONE_CALL` policy |
/// | `wide` | [`MAX_PREFIX_STMTS_WIDE`] (9) | same guards, cap raised to absorb kuna's finer printed-statement granularity |
pub struct OptionCondFold;

impl OptionCondFold {
    /// The option name.
    pub const NAME: &'static str = "condfold";

    /// Parse `off`/`on`/`wide` into the statement cap (0 = off) + a confirmation
    /// message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((
                0,
                "Short-circuit fold across a non-trivial sibling turned off".to_string(),
            )),
            // Empty parameter reads as `on`, matching `options::on_or_off`.
            "on" | "" => Ok((
                MAX_PREFIX_STMTS_ANGR,
                format!(
                    "Short-circuit fold across a non-trivial sibling turned on \
                     (angr parity: <={MAX_PREFIX_STMTS_ANGR} statements, \
                     <={MAX_PREFIX_CALLS} call)"
                ),
            )),
            "wide" => Ok((
                MAX_PREFIX_STMTS_WIDE,
                format!(
                    "Short-circuit fold across a non-trivial sibling turned on, wide \
                     (<={MAX_PREFIX_STMTS_WIDE} statements, <={MAX_PREFIX_CALLS} call)"
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
        // The *value* matches; the counting does not — `condfold_eligible` charges
        // only statement-root calls (see MAX_PREFIX_CALLS' docs), so a folded
        // operand can render more than one call when the extras are implied.
        assert_eq!(MAX_PREFIX_CALLS, 1);
        // `wide` only ever loosens the statement cap, never the call cap.
        assert!(MAX_PREFIX_STMTS_WIDE > MAX_PREFIX_STMTS_ANGR);
    }

    #[test]
    fn element_id_is_unique_in_the_41xx_range() {
        assert_eq!(ELEM_CONDFOLD.get_id(), 4113);
        assert_eq!(ELEM_CONDFOLD.get_name(), "condfold");
    }

    #[test]
    fn option_parses_off_on_and_wide_into_the_statement_cap() {
        let (v, msg) = OptionCondFold.apply("on").expect("on parses");
        assert_eq!(v, MAX_PREFIX_STMTS_ANGR);
        assert!(msg.contains("on"));
        // An empty parameter reads as `on`, matching `options::on_or_off`.
        assert_eq!(OptionCondFold.apply("").expect("empty parses").0, MAX_PREFIX_STMTS_ANGR);
        let (v, msg) = OptionCondFold.apply("wide").expect("wide parses");
        assert_eq!(v, MAX_PREFIX_STMTS_WIDE);
        assert!(msg.contains("wide"));
        // `off` is the sentinel 0 cap: `compute_condfold_blocks` short-circuits on it
        // and both gate disjuncts stay dead.
        let (v, msg) = OptionCondFold.apply("off").expect("off parses");
        assert_eq!(v, 0);
        assert!(msg.contains("off"));
        assert!(OptionCondFold.apply("maybe").is_err());
        assert!(OptionCondFold.apply("7").is_err());
    }
}
