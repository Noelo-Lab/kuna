//! S8 if-else flattening (port of angr's `IfElseFlattener` region simplifier,
//! `angr/analyses/decompiler/region_simplifiers/ifelse.py`).
//!
//! # The structural gap
//!
//! angr's SAILR structurer drops the `else` arm of a conditional whose `if` arm
//! is *statement-terminating* (every endpoint returns / aborts) and re-parents
//! the former `else` body as a plain follower of the `if`.  The net effect is the
//! classic early-return / guard-clause flattening:
//!
//! ```c
//!   if (cond) { ...; return X; } else { body }   ==>   if (cond) { ...; return X; } body
//! ```
//!
//! Most of this work is already done in kuna by the region structurer
//! (`p8_structure::region_structurer`, default-on since DIV-12): for a clean
//! diamond it forms an if/then (not an if/else) and the printer's guard
//! inversion already produces the flat early-return form.  What is left over —
//! and what this pass catches — is the residual case where the structurer (or
//! the [`CollapseStructure`](crate::blockaction) fallback) produced a *full
//! 3-component* [`BlockIf`](crate::block::BlockKind::If) (`cond` / true-clause /
//! else-clause) whose true-clause is nonetheless statement-terminating.  That
//! happens when the true-clause ends in a `goto` to a no-return / shared tail,
//! or when the clauses were folded with an explicit `else` that the structurer
//! could not invert away.  Such a `BlockIf` renders with an `else { ... }` whose
//! body is reached only by falling out of the `if`, so the `else` is pure
//! syntactic noise (and frequently carries a residual `goto`/label).
//!
//! # What this pass does
//!
//! Running once after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! (the structured tree is final and goto targets are labelled), and after
//! [`ActionGotoReduce`], [`ActionIfElseFlatten`] finds every 3-component
//! `BlockIf` whose
//!
//!   * true-clause is statement-terminating ([`clause_terminates`]), and
//!   * else-clause is **not** statement-terminating (angr's exact dual test), and
//!   * parent is a [`BlockList`](crate::block::BlockKind::Ls) /
//!     [`BlockGraph`](crate::block::BlockKind::Graph) that can hold a sibling,
//!
//! and rewrites it: the else-clause is unlinked from the `BlockIf` (which becomes
//! a 2-component if/then) and spliced into the parent's component list as the
//! `BlockIf`'s immediate successor.  This is a pure *print-tree* edit (the
//! `list`/`parent` of structured nodes), exactly as
//! [`kuna_inline_return_tail`](crate::block::BlockGraph::kuna_inline_return_tail)
//! does — no p-code is cloned and SSA/def-use is untouched, because the `else`
//! body deterministically executes immediately after the (non-returning) `if`.
//!
//! The transform is conservative (only the precise terminating-if /
//! non-terminating-else shape, only when the parent can host the moved sibling)
//! and gated by `option ifelseflatten on|off` (`flatten_ifelse`, default-ON per DIV-14) —
//! when off, [`ActionIfElseFlatten::apply`] early-returns and output is
//! byte-identical.  It logs an `ifelseflatten:` warning per function flattened so
//! the effect is observable.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockType;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::context::BlockId;

use kuna_base::marshal::ElementId;

/// Marshaling element `<ifelseflatten>` (kuna).  ElementIds live in the 4000+ range.
/// Renumbered from 4103 at the merge into main: main now uses 4100 gotoreduce,
/// 4101 tailcalljump, 4102 regionstructure, 4103 noreturn_extern,
/// 4104 noreturn_externmatch, 4105 crossjumprevert, 4106 switchsharedcase, so the
/// next free id is 4107.
pub const ELEM_IFELSEFLATTEN: ElementId = ElementId::new("ifelseflatten", 4107);

/// \brief Flatten `if (c) { ...return } else { body }` to `if (c) { ...return } body`
/// (angr `IfElseFlattener`).  Option-gated (`flatten_ifelse`).
pub struct ActionIfElseFlatten {
    base: ActionBase,
}

impl ActionIfElseFlatten {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionIfElseFlatten { base: ActionBase::new(0, "ifelseflatten", g) })
    }
}

impl Action for ActionIfElseFlatten {
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
        Some(Box::new(ActionIfElseFlatten { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option ifelseflatten on|off`, carried on the seam Architecture.
        if !data.get_arch().flatten_ifelse {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        // Collect the flattenable `BlockIf`s.  A node is flattenable when it is a
        // 3-component `BlockIf` (cond / true / else), its true-clause terminates,
        // its else-clause does not, and its parent is a list/graph able to host
        // the moved else sibling.  We snapshot the candidates, then re-verify each
        // at apply time (a prior splice can shift the tree under us).
        let mut victims: Vec<BlockId> = Vec::new();
        for n in collect_nodes(data, root) {
            if is_flattenable(data, n) {
                victims.push(n);
            }
        }
        if victims.is_empty() {
            return 0;
        }

        let mut count: int4 = 0;
        for if_id in victims {
            if !is_flattenable(data, if_id) {
                continue;
            }
            if data.sblocks_mut().kuna_flatten_ifelse(if_id) {
                count += 1;
            }
        }

        if count > 0 {
            let name = data.get_name().to_string();
            data.warning_header(&format!(
                "ifelseflatten: dropped {count} else-arm(s) after a terminating if in {name}"
            ));
        }
        self.base_mut().count += count;
        0
    }
}

/// Depth-first snapshot of every structured-block node reachable from `root`
/// (each node's components are its `BlockGraph::list`).
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

/// Is `n` a 3-component `BlockIf` (cond / true-clause / else-clause) whose
/// true-clause is statement-terminating, whose else-clause is not, and whose
/// parent is a list/graph that can host the moved sibling?  (angr
/// `IfElseFlattener._handle_Condition`'s exact firing predicate.)
fn is_flattenable(data: &Funcdata, n: BlockId) -> bool {
    let g = data.sblocks_ref();
    let blk = g.block(n);
    if blk.get_type() != BlockType::If {
        return false;
    }
    // A full if/else is a 3-component BlockIf: [cond, true-clause, else-clause].
    // (if/then is 2-component; an if-goto is 1-component.)
    if blk.get_size() != 3 {
        return false;
    }
    // The if must not carry its own unstructured goto (an if-goto target) — that
    // is a different shape the printer renders specially.
    if blk.get_if_goto_target().is_some() {
        return false;
    }
    // Parent must be a plain list/graph able to hold the spliced sibling.
    let parent = match blk.get_parent() {
        Some(p) => p,
        None => return false,
    };
    match g.block(parent).get_type() {
        BlockType::Ls | BlockType::Graph => {}
        _ => return false,
    }
    let true_clause = blk.get_block(1);
    let else_clause = blk.get_block(2);
    // angr's exact condition: all endpoints of the true node return / abort, and
    // not all endpoints of the false node do.
    clause_terminates(data, true_clause) && !clause_terminates(data, else_clause)
}

/// Does every flow-endpoint of the structured clause `cl` terminate the routine
/// (return / no-return) so control never falls out the bottom of the clause?
/// (angr `is_statement_terminating` over `get_last_statements`.)
///
/// This mirrors angr's [`is_statement_terminating`] exactly: only a `return` or a
/// no-return call counts.  Critically a trailing **`goto`/`break`/`continue` is
/// NOT terminating** — it transfers control to other code that may still reach
/// the after-`if` region, so dropping the `else` for such a clause would be
/// unsound.  Anything we cannot resolve is treated as *non*-terminating (the safe
/// default — it keeps the `else`).
///
/// [`is_statement_terminating`]: https://github.com/angr/angr (region_simplifiers)
fn clause_terminates(data: &Funcdata, cl: BlockId) -> bool {
    let g = data.sblocks_ref();
    match g.block(cl).get_type() {
        // A `goto`/`break`/`continue` / `MultiGoto` transfers control to other
        // code rather than ending the routine, so it is NOT terminating — this
        // matches angr's `is_statement_terminating` (only Return / no-return Call
        // count).  Treating a goto as terminating is unsound in the general case
        // (the target may rejoin the after-`if` flow), so we keep the `else`.
        BlockType::Goto | BlockType::MultiGoto => false,
        // A nested conditional (if/else or if-goto) may fall through on one arm,
        // so it is conservatively *non*-terminating for the flatten test.
        BlockType::If => false,
        // A list terminates iff its last component terminates.
        BlockType::Ls | BlockType::Graph => {
            let nsz = g.block(cl).get_size();
            if nsz == 0 {
                return false;
            }
            clause_terminates(data, g.block(cl).get_block(nsz - 1))
        }
        // A basic/copy leaf terminates iff its tail op is a RETURN or a
        // no-return call.
        BlockType::Basic | BlockType::Copy => leaf_terminates(data, cl),
        // Loops, switches, conditions: conservatively non-terminating.
        _ => false,
    }
}

/// Does the basic/copy leaf `leaf` end in a `return` (or a no-return call), so
/// control never falls out of it?
fn leaf_terminates(data: &Funcdata, leaf: BlockId) -> bool {
    let g = data.sblocks_ref();
    // Resolve a BlockCopy to the underlying BlockBasic; a BlockBasic is itself.
    let bb = match g.block(leaf).get_type() {
        BlockType::Copy => match g.block(leaf).get_copy() {
            Some(b) => b,
            None => return false,
        },
        BlockType::Basic => leaf,
        _ => return false,
    };
    let tail = match data.bb_op_tail(bb) {
        Some(t) => t,
        None => return false,
    };
    let op = match data.obank().get(tail) {
        Some(o) => o,
        None => return false,
    };
    match op.code() {
        OpCode::CPUI_RETURN => true,
        // A tail call to a no-return callee (e.g. abort / __stack_chk_fail /
        // exit) never falls through.  The basic block then has no structural
        // out-edges in the CFG.
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {
            data.bblocks_ref().block(bb).size_out() == 0
        }
        _ => false,
    }
}

//===========================================================================
// Option parse (`ifelseflatten on|off`).
//===========================================================================

/// The `ifelseflatten on|off` ArchOption (angr `IfElseFlattener`).
pub struct OptionIfElseFlatten;

impl OptionIfElseFlatten {
    /// The option name.
    pub const NAME: &'static str = "ifelseflatten";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("If-else flattening turned {prop}")))
    }
}

