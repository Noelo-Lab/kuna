//! S8 goto-reduction by **cross-jump tail duplication** (port of angr's SAILR
//! `CrossJumpReverter`; Basque et al., USENIX Security 2024 "SAILR").
//!
//! # The structural gap
//!
//! Compilers perform *Irreducible Statement Condensing* (ISC, "cross jumping"):
//! identical tail code reached from several predecessors is condensed into **one**
//! shared basic block, and all but one predecessor reach it by a `goto`.  Ghidra's
//! `CollapseStructure` (the verbatim [`crate::blockaction`] port) must then express
//! that in-edge as an unstructured `goto`.  On `who.o::scan_entries` kuna emits:
//!
//! ```c
//!   else if (need_deadprocs && v1 == 8) {
//!       print_deadprocs(a1);
//!       goto label_4012b5;          // <-- the residual cross-jump goto
//!   }
//!   ...
//!   else {
//!   label_4012b5:
//!       v1 = *a1;                   // <-- the shared, condensed tail
//!   }
//!   if (v1 == 2) { ... }            // <-- both paths converge here
//! ```
//!
//! angr (zero gotos) **reverts the condensing** by duplicating the shared tail
//! (`v1 = *a1;`) back into the `goto` source, so both paths fall straight through to
//! the common successor and the `goto`/label vanish.
//!
//! # What this pass does
//!
//! This is the **sibling** of [`crate::s8_structure::kuna_gotoreduce`]
//! ([`ActionGotoReduce`](crate::s8_structure::kuna_gotoreduce::ActionGotoReduce)):
//! `gotoreduce` duplicates a tail that ends in `return` (a *closed* path — no
//! fall-through to reconcile), while this pass duplicates a **non-return,
//! fall-through** cross-jump tail, which additionally requires a
//! **convergence precondition**.
//!
//! Running once after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! (the structured tree is final and goto targets are labelled),
//! [`ActionCrossJumpReverter`] finds every `BlockIf` that renders as
//! `if (cond) goto T` where:
//!
//! 1. `T` is a small **single-successor** basic block (angr: `out_degree(T) == 1`)
//!    — a *non-return* fall-through tail — with no side-effecting op (no
//!    `CALL`/`STORE`) and at most [`MAX_TAIL_CALLS`] calls (angr `max_call_dup`),
//!    bounded to [`MAX_TAIL_OPS`] printed ops (angr duplicates exactly the single
//!    `goto_target` region node, so the kuna port duplicates the one tail block).
//! 2. **Convergence precondition (correctness-critical):** the place flow reaches
//!    after the `if` completes (its structured fall-through,
//!    [`next_flow_after`](crate::block::BlockGraph::next_flow_after)) is the *same*
//!    leaf the tail `T` falls through to.  Only then is duplicating `T` inline
//!    semantics-preserving: the removed path was `goto T; (T); → S`, and the new
//!    path is `(T inline); → S'` with `S == S'`.  Decline on any doubt.
//!
//! When both hold it rewrites the `if` to `if (cond) { <duplicated tail> }`, reusing
//! the same print-tree surgery as `gotoreduce`
//! ([`kuna_inline_return_tail`](crate::block::BlockGraph::kuna_inline_return_tail)):
//! fresh `BlockCopy` leaves referencing the *same* underlying basic blocks (the
//! printer re-emits their ops — no p-code cloned, SSA/def-use untouched).  When the
//! converted `goto` was the only unstructured edge into `T`, `T`'s label is cleared.
//! Each duplication is logged via [`Funcdata::warning`].
//!
//! The transform is bounded and conservative, so duplication stays cheap and
//! side-effect-safe.  It is gated by `option crossjumprevert on|off`
//! (`revert_cross_jumps`, default-OFF) — when off, [`ActionCrossJumpReverter::apply`]
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

/// Marshaling element `<crossjumprevert>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce, 4101 tailcalljump, 4102 regionstructure are taken; this is 4103).
pub const ELEM_CROSSJUMPREVERT: ElementId = ElementId::new("crossjumprevert", 4103);

/// Maximum number of printed p-code ops in the duplicated cross-jump tail block.
pub const MAX_TAIL_OPS: usize = 8;
/// Maximum number of calls in a duplicated tail (angr `max_call_duplications=1`).
pub const MAX_TAIL_CALLS: usize = 1;

/// \brief Revert compiler cross-jumping by duplicating a small non-return
/// cross-jump tail into the `goto` source (angr SAILR `CrossJumpReverter`).
/// Option-gated (`revert_cross_jumps`).
pub struct ActionCrossJumpReverter {
    base: ActionBase,
}

impl ActionCrossJumpReverter {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionCrossJumpReverter { base: ActionBase::new(0, "crossjumprevert", g) })
    }
}

impl Action for ActionCrossJumpReverter {
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
        Some(Box::new(ActionCrossJumpReverter { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option crossjumprevert on|off`, carried on the seam Architecture.
        if !data.get_arch().revert_cross_jumps {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        // Find every `BlockGoto` node `<body>; goto T` whose target T is a
        // duplicable, convergent non-return cross-jump tail.
        // (goto_node, bbchain, target_sblock, warn_addr)
        let mut conversions: Vec<(BlockId, Vec<BlockId>, BlockId, kuna_base::address::Address)> =
            Vec::new();
        let nodes = collect_nodes(data, root);
        for &n in &nodes {
            let blk = data.sblocks_ref().block(n);
            // The cross-jump source is an unstructured `BlockGoto` leaf (the goto is
            // a separate component, e.g. `{ print_deadprocs(a1); goto label; }`).
            if blk.get_type() != BlockType::Goto {
                continue;
            }
            if blk.get_goto_type() != block_flags::f_goto_goto {
                continue;
            }
            let target = match blk.get_goto_target() {
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
            // The non-return cross-jump tail: a bounded single-successor chain whose
            // last block does *not* return and falls through to a unique successor S.
            let (chain, succ) = match cross_jump_tail_chain(data, bb0) {
                Some(cs) => cs,
                None => continue,
            };
            // Convergence precondition (correctness-critical): the place flow reaches
            // after the goto's body completes (its structured fall-through, were the
            // goto removed) must be exactly S (the tail's own fall-through).  Only
            // then is duplicating T inline + dropping the goto semantics-preserving.
            if !converges_on(data, n, succ) {
                continue;
            }
            // Warn address: the head op of the duplicated tail block.
            let warn_addr = match data
                .bb_op_head(bb0)
                .and_then(|o| data.obank().get(o))
                .map(|o| o.get_addr().clone())
            {
                Some(a) => a,
                None => continue,
            };
            conversions.push((n, chain, target, warn_addr));
        }
        if conversions.is_empty() {
            return 0;
        }

        let mut count: int4 = 0;
        let mut converted_targets: Vec<BlockId> = Vec::new();
        for (goto_id, chain, target, warn_addr) in &conversions {
            // Append the duplicated BlockCopy tail after the goto's body and switch
            // the node to a plain BlockList (the goto/label vanish).  Sound for the
            // fall-through case because the convergence precondition above proved
            // both paths reach the same successor S.
            data.sblocks_mut().kuna_inline_crossjump_tail(*goto_id, chain);
            data.warning("crossjumprevert: duplicated cross-jump tail to remove goto", warn_addr);
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

/// Check the *bblock* `bb0` is a duplicable **non-return** cross-jump tail.
/// Returns `(chain, successor)` where `chain = [bb0]` (the single tail block — angr
/// duplicates exactly the one `goto_target` node) and `successor` is the unique
/// bblock `bb0` falls through to (angr: `out_degree(T) == 1`), or `None` if `bb0` is
/// not a duplicable cross-jump tail (ends in a return, is too big, side-effecting,
/// or multi-successor).
fn cross_jump_tail_chain(data: &Funcdata, bb0: BlockId) -> Option<(Vec<BlockId>, BlockId)> {
    // angr duplicates exactly the single `goto_target` region node, so the kuna
    // port duplicates exactly the one cross-jump tail bblock — a conservative,
    // semantics-preserving choice (the convergence precondition is checked against
    // its single fall-through successor).
    let mut total_ops = 0usize;
    let mut total_calls = 0usize;
    let cur = bb0;

    // Scan the block's ops: reject anything unsafe to duplicate, count the rest.
    let mut op = data.bb_op_head(cur);
    while let Some(o) = op {
        op = data.bb_op_next(o);
        if let Some(opref) = data.obank().get(o) {
            // SSA-bookkeeping ops (MULTIEQUAL phis, INDIRECT, other markers /
            // nonprinting) emit no C statement and are re-resolved by the printer on
            // the duplicated copy path — don't count them toward the size bound and
            // don't treat them as duplicable code.
            if opref.not_printed() {
                continue;
            }
            total_ops += 1;
            match opref.code() {
                // STORE is never duplicated (a duplicated write is unsafe).
                OpCode::CPUI_STORE => return None,
                // angr permits up to max_call_dup calls in the duplicated tail.
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => {
                    total_calls += 1;
                    if total_calls > MAX_TAIL_CALLS {
                        return None;
                    }
                }
                _ => {}
            }
        }
    }
    if total_ops > MAX_TAIL_OPS {
        return None;
    }
    // A block ending in `return` is the gotoreduce (closed-path) case, not a
    // cross-jump fall-through tail — decline here.
    let tail_is_return = data
        .bb_op_tail(cur)
        .and_then(|t| data.obank().get(t))
        .map(|o| o.code() == OpCode::CPUI_RETURN)
        .unwrap_or(false);
    if tail_is_return {
        return None;
    }
    // The tail must fall through to exactly one successor (angr out_degree==1).
    let blk = data.bblocks_ref().block(cur);
    if blk.size_out() != 1 {
        return None;
    }
    let next = blk.get_out(0);
    if next == cur {
        return None; // self-loop guard
    }
    // `next` is the convergence target S, checked by the convergence precondition.
    Some((vec![cur], next))
}

/// Convergence precondition: does flow after the `if` block `if_id` completes reach
/// exactly the bblock `succ` (the tail's fall-through)?  Computed on the structured
/// tree via [`BlockGraph::next_flow_after`](crate::block::BlockGraph::next_flow_after);
/// the leaf it returns is a `BlockCopy` whose underlying bblock must equal `succ`.
fn converges_on(data: &Funcdata, if_id: BlockId, succ: BlockId) -> bool {
    let g = data.sblocks_ref();
    let parent = match g.block(if_id).get_parent() {
        Some(p) => p,
        None => return false,
    };
    let leaf = match g.next_flow_after(parent, if_id) {
        Some(l) => l,
        None => return false,
    };
    // The fall-through leaf must be a Copy of exactly the tail's successor bblock.
    g.block(leaf).get_copy() == Some(succ)
}

//===========================================================================
// Option parse (`crossjumprevert on|off`).
//===========================================================================

/// The `crossjumprevert on|off` ArchOption (angr SAILR `CrossJumpReverter`).
pub struct OptionCrossJumpReverter;

impl OptionCrossJumpReverter {
    /// The option name.
    pub const NAME: &'static str = "crossjumprevert";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Cross-jump tail reversion turned {prop}")))
    }
}
