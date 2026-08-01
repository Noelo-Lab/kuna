//! S8 **short-circuit boolean re-rolling** — collapse a `0`/`1` materialization
//! diamond whose condition is a folded `&&`/`||` chain back into a single boolean
//! assignment (`x = a && b;`).
//!
//! # The structural gap
//!
//! At `-O0` gcc materializes a source `x = a && b;` as a jump-based select: the
//! `1` arm is reached only from the last test, while the `0` arm is the shared
//! target of *every* short-circuit exit.  P3 `RuleConditionalMove`
//! (`p3_dataflow/ruleaction_7.rs`) is the pass that folds `MULTIEQUAL(0,1)` into
//! `zext(cond)`, and it fires on the simple one-test shape — but upstream Ghidra
//! requires each MULTIEQUAL input block to be the CBRANCH root itself or a
//! **single-predecessor** pass-through (`inblock->sizeIn() != 1` bails,
//! `ruleaction.cc:9427`).  A short-circuit chain gives the constant arm two or more
//! predecessors, so it bails.  That bail is faithful to upstream and, at the IR
//! level, *correct*: the constant arm has no single dominating CBRANCH whose
//! boolean could stand for the whole chain, and hoisting a guarded operand
//! (`command->flags` under `command != 0`) out of its branch would evaluate it
//! unconditionally.  The short-circuit must stay in the CFG; only C's own
//! `&&`/`||` can express it.
//!
//! So kuna renders the idiom as an explicit two-arm constant diamond:
//!
//! ```text
//! if ((a0) && (v2 = a0[1] & 0x100, v2))
//!   v2 = 1;
//! else {
//!   v2 = 0;
//! }
//! v28 = v2;
//! ```
//!
//! where the source and Hex-Rays have one statement.  Each such diamond costs 3
//! CFG blocks / 4 edges; on bash `time_command` two of them are the *entire*
//! residual difference from IDA's pane (34n/51e vs the source's 28n/43e).
//!
//! # What this pass does
//!
//! By the time the structurer is finished, the chain that P3 could not see has
//! already been folded into a single [`BlockCondition`](crate::block::BlockType::Condition)
//! — kuna prints it today as the `if`'s condition, complete with C's short-circuit
//! semantics and any comma-expression side effects.  It is therefore renderable
//! **unchanged** as an r-value, which is exactly the relaxation the P3 predicate
//! cannot safely make.
//!
//! Running alongside [`ActionIteRegion`](crate::p8_structure::kuna_iteregion::ActionIteRegion),
//! [`ActionIteBoolean`] finds every 3-component [`BlockIf`](crate::block::BlockType::If)
//! with no own goto whose
//!
//! * condition component is a `BlockCondition` tree of clean, unlabelled
//!   `CBRANCH`-terminated leaves (the folded `&&`/`||` chain), and whose
//! * true- and else-clauses are each a single leaf whose only printed op is a
//!   `COPY` of the constant `1`/`0` (in either order) to the **same** integral
//!   variable,
//!
//! and marks it for boolean rendering: `dest = ( cond );` when the true arm writes
//! `1`, `dest = !( cond );` when it writes `0`.  Requiring the `BlockCondition` is
//! what makes the rewrite value-safe as well as shape-safe: the rendered form then
//! always has `&&`/`||` (or the negating `!`) at its top level, and those C
//! operators yield exactly `0`/`1` -- the constants the arms wrote.  A bare leaf
//! condition gives no such guarantee (under `truthycond` an `if (x & 0x100)` prints
//! as `(x & 0x100)`, worth 0x100), and is also the shape P3 already folds.
//!
//! The match is **print-only**: the pass mutates *no* p-code.  It sets the
//! [`kuna_iteboolean`](crate::op::pcodeop_addlflags::kuna_iteboolean) addl-flag on
//! the condition's terminal `CBRANCH`; the S9 C printer
//! ([`PrintC::emit_block_if`](crate::printc)) sees that flag, re-derives the same
//! match via [`match_ite_boolean`], and emits the single assignment in place of the
//! `if`/`else`.  Because the condition is emitted by the *same* renderer that
//! produced the `if (...)` header, evaluation order, short-circuiting and the
//! comma-expression side effects are all preserved verbatim.
//!
//! # Why this is a **runtime choice**
//!
//! An explicit `if (c) x = 1; else x = 0;` in the source compiles to the same
//! object code as `x = c;`, so — exactly like
//! [`iteregion`](crate::p8_structure::kuna_iteregion) (DIV-17) — the rewrite can
//! diverge from the source.  It is gated by `option iteboolean on|off`; with it off
//! [`ActionIteBoolean::apply`] early-returns, nothing is marked, and output is
//! byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockType;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_addlflags;
use crate::options::on_or_off;
use crate::p8_structure::kuna_iteregion::{leaf_bblock, printed_ops, same_storage};

use kuna_base::marshal::ElementId;

/// Marshaling element `<iteboolean>` (kuna).  ElementIds live in the 4000+ range
/// (4118 callsitestackargs was the previous high-water mark; this is 4119).
pub const ELEM_ITEBOOLEAN: ElementId = ElementId::new("iteboolean", 4119);

/// How deep a `BlockCondition` chain the matcher walks.  A short-circuit condition
/// of more clauses than this is left alone (it would not read as a single boolean
/// assignment anyway).
const MAX_CONDITION_DEPTH: u32 = 16;

/// A matched `0`/`1` materialization diamond that renders as `dest = cond;`.
#[derive(Clone, Copy)]
pub struct IteBoolMatch {
    /// The condition component (structured block 0 of the `if`) — a `CBRANCH` leaf
    /// or a folded `&&`/`||` [`BlockCondition`](crate::block::BlockType::Condition).
    /// Rendered with `ONLY_BRANCH` it gives the parenthesized `( cond )`.
    pub cond_block: BlockId,
    /// The condition's **terminal** `CBRANCH` (carries the
    /// [`kuna_iteboolean`](pcodeop_addlflags::kuna_iteboolean) mark).
    pub cbranch: OpId,
    /// The `COPY` of the **true** clause; the statement's markup anchor.
    pub assign_op: OpId,
    /// The assignment target (`assign_op`'s output; storage-equal to the else arm's).
    pub dest: VarnodeId,
    /// The true clause writes `0` (and the else clause `1`), so the rendered form
    /// is `dest = !( cond );`.
    pub negate: bool,
}

/// \brief Re-roll a short-circuit `0`/`1` materialization diamond into a single
/// boolean assignment (`x = a && b;`).  Print-only + option-gated (`iteboolean`),
/// a runtime choice: it matches the source only when the source wrote a boolean
/// assignment rather than an explicit `if/else`.
pub struct ActionIteBoolean {
    base: ActionBase,
}

impl ActionIteBoolean {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionIteBoolean { base: ActionBase::new(0, "iteboolean", g) })
    }
}

impl Action for ActionIteBoolean {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionIteBoolean { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option iteboolean on|off`, carried on the Architecture
        // boundary.  Off => nothing marked => byte-identical output.
        if !data.get_arch().iteboolean {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        let mut marks: Vec<OpId> = Vec::new();
        for n in collect_nodes(data, root) {
            if let Some(m) = match_ite_boolean(data, n) {
                marks.push(m.cbranch);
            }
        }
        if marks.is_empty() {
            return 0;
        }

        let count = marks.len() as int4;
        for cbranch in marks {
            if let Some(o) = data.obank_mut().get_mut(cbranch) {
                o.set_additional_flag(pcodeop_addlflags::kuna_iteboolean);
            }
        }

        let name = data.get_name().to_string();
        data.warning_header(&format!(
            "iteboolean: re-rolled {count} 0/1 select diamond(s) to a boolean assignment in {name}"
        ));
        self.base_mut().count += count;
        0
    }
}

/// Depth-first snapshot of every structured-block node reachable from `root`.
fn collect_nodes(data: &Funcdata, root: BlockId) -> Vec<BlockId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        out.push(id);
        let g = data.sblocks_ref();
        let n = g.block(id).get_size();
        for i in 0..n {
            stack.push(g.block(id).get_block(i));
        }
    }
    out
}

/// If the structured node `n` is a `0`/`1` materialization diamond, return the
/// [`IteBoolMatch`] the S9 printer needs.  Otherwise `None`.
///
/// Shared by [`ActionIteBoolean::apply`] (which marks the terminal `CBRANCH`) and
/// the printer (which re-derives the same match), so the schema is defined once.
pub fn match_ite_boolean(data: &Funcdata, n: BlockId) -> Option<IteBoolMatch> {
    let g = data.sblocks_ref();
    let blk = g.block(n);
    if blk.get_type() != BlockType::If || blk.get_size() != 3 {
        return None;
    }
    if blk.get_if_goto_target().is_some() {
        return None;
    }
    let cond_block = blk.get_block(0);
    let true_clause = blk.get_block(1);
    let else_clause = blk.get_block(2);

    // The condition component must be a folded short-circuit chain.  This is what
    // makes the rewrite VALUE-safe, not just shape-safe: `emit_block_condition`
    // renders it with `&&`/`||` at the top level, and C's `&&`/`||` (and the `!`
    // of the negated form) always yield exactly 0 or 1 -- the two constants the
    // arms wrote.  A bare leaf condition carries no such guarantee: under
    // `truthycond` (DIV-37) `if (x & 0x100)` renders as `(x & 0x100)`, whose value
    // is 0x100, so `v = (x & 0x100)` would store the wrong number.  (The bare-leaf
    // shape is also the one P3 `RuleConditionalMove` already folds.)
    if data.sblocks_ref().block(cond_block).get_type() != BlockType::Condition {
        return None;
    }
    let cbranch = cond_terminal_cbranch(data, cond_block, 0)?;

    let (true_op, dest_true, val_true) = const_bool_arm(data, true_clause)?;
    let (_else_op, dest_else, val_else) = const_bool_arm(data, else_clause)?;

    // The two arms must write the SAME variable and DIFFERENT constants, so the
    // assignment has one well-defined destination and the condition's own truth
    // value is exactly what is stored.
    if !same_storage(data, dest_true, dest_else) || val_true == val_else {
        return None;
    }
    // A pointer/float/aggregate destination would lose the constant's own rendering
    // (its cast) when the constants are replaced by the boolean expression.
    if !integral_dest(data, dest_true) {
        return None;
    }

    Some(IteBoolMatch {
        cond_block,
        cbranch,
        assign_op: true_op,
        dest: dest_true,
        negate: val_true == 0,
    })
}

/// The terminal `CBRANCH` of a condition component that is renderable as a pure
/// r-value: a clean `CBRANCH`-terminated leaf, or a `BlockCondition` tree (the
/// folded `&&`/`||` chain) *every* clause of which is itself one.  `None` for
/// anything else — a loop, a nested `if`, or a labelled goto target, none of which
/// the printer renders inside the `if (...)` header either.
fn cond_terminal_cbranch(data: &Funcdata, id: BlockId, depth: u32) -> Option<OpId> {
    if depth > MAX_CONDITION_DEPTH {
        return None;
    }
    if data.sblocks_ref().block(id).get_type() == BlockType::Condition {
        if data.sblocks_ref().block(id).get_size() != 2 {
            return None;
        }
        let first = data.sblocks_ref().block(id).get_block(0);
        let second = data.sblocks_ref().block(id).get_block(1);
        // Both clauses must themselves be renderable; the LAST one owns the
        // terminal CBRANCH that carries the mark.
        cond_terminal_cbranch(data, first, depth + 1)?;
        return cond_terminal_cbranch(data, second, depth + 1);
    }
    let bb = leaf_bblock(data, id)?;
    let op = *printed_ops(data, bb).last()?;
    if data.obank().get(op)?.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    Some(op)
}

/// If the structured leaf `id` is an arm whose single printed op is a `COPY` of the
/// constant `0` or `1`, return `(copy_op, out_vn, const_value)`.  Any other arm — a
/// computed value, a second statement, a side effect, a labelled leaf — declines.
fn const_bool_arm(data: &Funcdata, id: BlockId) -> Option<(OpId, VarnodeId, u64)> {
    let bb = leaf_bblock(data, id)?;
    let ops = printed_ops(data, bb);
    if ops.len() != 1 {
        return None;
    }
    let op = ops[0];
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_COPY {
        return None;
    }
    let out = o.get_out()?;
    let in0 = o.get_in(0)?;
    let vn = data.vbank().get(in0)?;
    if !vn.is_constant() {
        return None;
    }
    let val = vn.get_offset();
    if val > 1 {
        return None;
    }
    Some((op, out, val))
}

/// Does `vn` render as a plain integral scalar — the only destination for which
/// replacing the constants `0`/`1` with a boolean expression is faithful?
fn integral_dest(data: &Funcdata, vn: VarnodeId) -> bool {
    matches!(
        data.vn_type_def_facing(vn).get_metatype(),
        type_metatype::TYPE_UNKNOWN
            | type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_BOOL
    )
}

//===========================================================================
// Option parse (`iteboolean on|off`).
//===========================================================================

/// The `iteboolean on|off` ArchOption.
pub struct OptionIteBoolean;

impl OptionIteBoolean {
    /// The option name.
    pub const NAME: &'static str = "iteboolean";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((
            val,
            format!("Short-circuit 0/1 select -> boolean assignment re-rolling turned {prop}"),
        ))
    }
}
