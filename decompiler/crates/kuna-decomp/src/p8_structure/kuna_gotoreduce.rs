//! S8 goto-reduction by return-tail duplication (port of angr's SAILR / Phoenix
//! `ReturnDuplicator` / goto-reduction family; Basque et al., USENIX Security
//! 2024 "SAILR").
//!
//! # The structural gap
//!
//! Ghidra's `CollapseStructure` (the verbatim
//! [`p8_structure::blockaction`](crate::blockaction) port of
//! `CollapseStructure`/`TraceDAG`/`BlockTrace`) must express one in-edge of a
//! shared, multi-predecessor *return tail* as an unstructured `goto` — it has no
//! schema that duplicates a tail to avoid the goto.  On `dd::argmatch_to_argument`
//! kuna therefore emits:
//!
//! ```c
//!   if (v2 == 0) {
//! label_40a6a3:
//!     v2 = 0;
//!   } else {
//!     ... while (...) { ... if (v2 == 0) goto label_40a6a3; ... }
//!   }
//!   return v2;
//! ```
//!
//! angr (zero gotos) **duplicates the constant return tail** (`v2 = 0; return v2;`)
//! into the in-loop edge, turning the cross-edge into a structured early return.
//!
//! # What this pass does
//!
//! Running once after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! (the structured tree is final and goto targets are labelled), [`ActionGotoReduce`]
//! finds every `BlockIf` that renders as `if (cond) goto T` where `T` is a small
//! single-successor basic-block chain ending in a `return`, and rewrites it to
//! `if (cond) { <duplicated tail> }`.  The duplicated tail is a set of fresh
//! `BlockCopy` leaves referencing the *same* underlying basic blocks (the printer
//! re-emits their ops — a print-tree duplication, semantically identical to the
//! `goto` because the tail deterministically falls through to the `return`), so no
//! p-code is cloned and SSA/def-use is untouched.  When the converted `goto` was
//! the only unstructured edge into `T`, `T`'s label is cleared.
//!
//! The transform is bounded and conservative (≤ [`MAX_TAIL_BLOCKS`] blocks /
//! [`MAX_TAIL_OPS`] ops, no `CALL`/`STORE` in the tail), so duplication stays
//! cheap and side-effect-safe.  It is gated by `option gotoreduce on|off`
//! (`reduce_return_gotos`, default-OFF) — when off, [`ActionGotoReduce::apply`]
//! early-returns and output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockType};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::seams::BlockId;

use kuna_base::marshal::ElementId;

/// Marshaling element `<gotoreduce>` (kuna).  ElementIds live in the 4000+ range.
pub const ELEM_GOTOREDUCE: ElementId = ElementId::new("gotoreduce", 4100);

/// Maximum number of basic blocks in a duplicated return tail.
pub const MAX_TAIL_BLOCKS: usize = 3;
/// Maximum number of p-code ops across a duplicated return tail.
pub const MAX_TAIL_OPS: usize = 8;

/// \brief Reduce gotos by duplicating a small return tail into the `goto` source
/// (C++/angr `ReturnDuplicator`).  Option-gated (`reduce_return_gotos`).
pub struct ActionGotoReduce {
    base: ActionBase,
}

impl ActionGotoReduce {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionGotoReduce { base: ActionBase::new(0, "gotoreduce", g) })
    }
}

impl Action for ActionGotoReduce {
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
        Some(Box::new(ActionGotoReduce { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option gotoreduce on|off`, carried on the seam Architecture.
        if !data.get_arch().reduce_return_gotos {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        // Find every `if (cond) goto T` whose target T is a duplicable return tail.
        // (if_block, bbchain, target_sblock)
        let mut conversions: Vec<(BlockId, Vec<BlockId>, BlockId)> = Vec::new();
        let nodes = collect_nodes(data, root);
        for &n in &nodes {
            let blk = data.sblocks_ref().block(n);
            if blk.get_type() != BlockType::If {
                continue;
            }
            if blk.get_if_goto_type() != block_flags::f_goto_goto {
                continue;
            }
            let target = match blk.get_if_goto_target() {
                Some(t) => t,
                None => continue,
            };
            let tgt = data.sblocks_ref().block(target);
            if tgt.get_type() != BlockType::Copy {
                continue;
            }
            let bb0 = match tgt.get_copy() {
                Some(b) => b,
                None => continue,
            };
            if let Some(chain) = return_tail_chain(data, bb0) {
                conversions.push((n, chain, target));
            }
        }
        if conversions.is_empty() {
            return 0;
        }

        let mut count: int4 = 0;
        let mut converted_targets: Vec<BlockId> = Vec::new();
        for (if_id, chain, target) in &conversions {
            data.sblocks_mut().kuna_inline_return_tail(*if_id, chain);
            converted_targets.push(*target);
            count += 1;
        }

        // Drop the label on any target no longer reached by an unstructured edge.
        let still: std::collections::BTreeSet<BlockId> = referenced_goto_targets(data);
        for t in converted_targets {
            if !still.contains(&t) {
                data.sblocks_mut().block_mut(t).clear_flag(block_flags::f_unstructured_targ);
            }
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

/// The set of sblocks still targeted by an unstructured `if`-goto or `goto`
/// (after the conversions have mutated the tree).
fn referenced_goto_targets(data: &Funcdata) -> std::collections::BTreeSet<BlockId> {
    let mut set = std::collections::BTreeSet::new();
    let root = data.sblocks_root();
    for n in collect_nodes(data, root) {
        let blk = data.sblocks_ref().block(n);
        if let Some(t) = blk.get_if_goto_target() {
            set.insert(t);
        }
        if let Some(t) = blk.get_goto_target() {
            set.insert(t);
        }
    }
    set
}

/// Walk the *bblocks* single-successor run starting at `bb0`; return the chain
/// `[bb0, .., R]` where `R`'s last op is a `return`, or `None` if the run is not
/// a bounded, side-effect-safe return tail.
fn return_tail_chain(data: &Funcdata, bb0: BlockId) -> Option<Vec<BlockId>> {
    let mut chain: Vec<BlockId> = Vec::new();
    let mut total_ops = 0usize;
    let mut cur = bb0;
    loop {
        if chain.len() >= MAX_TAIL_BLOCKS {
            return None;
        }
        // Scan the block's ops: reject anything unsafe to duplicate, count them.
        let mut op = data.bb_op_head(cur);
        while let Some(o) = op {
            op = data.bb_op_next(o);
            if let Some(opref) = data.obank().get(o) {
                total_ops += 1;
                match opref.code() {
                    OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_CALLOTHER
                    | OpCode::CPUI_STORE => return None,
                    _ => {}
                }
            }
        }
        if total_ops > MAX_TAIL_OPS {
            return None;
        }
        chain.push(cur);
        // End of the tail: this block returns.
        let tail_is_return = data
            .bb_op_tail(cur)
            .and_then(|t| data.obank().get(t))
            .map(|o| o.code() == OpCode::CPUI_RETURN)
            .unwrap_or(false);
        if tail_is_return {
            return Some(chain);
        }
        // Otherwise the run must continue with exactly one successor.
        let blk = data.bblocks_ref().block(cur);
        if blk.size_out() != 1 {
            return None;
        }
        let next = blk.get_out(0);
        if chain.contains(&next) {
            return None; // cycle guard
        }
        cur = next;
    }
}

//===========================================================================
// Option parse (`gotoreduce on|off`).
//===========================================================================

/// The `gotoreduce on|off` ArchOption (angr SAILR goto-reduction).
pub struct OptionGotoReduce;

impl OptionGotoReduce {
    /// The option name.
    pub const NAME: &'static str = "gotoreduce";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Return-tail goto reduction turned {prop}")))
    }
}
