//! S8 goto-reduction by **return-tail duplication with calls** (port of angr's
//! SAILR `ReturnDuplicatorLow` / `ReturnDuplicatorBase` *goto* version; Basque et
//! al., USENIX Security 2024 "SAILR").
//!
//! # The structural gap
//!
//! Ghidra's `CollapseStructure` (the verbatim [`crate::blockaction`] port of
//! `CollapseStructure`/`TraceDAG`/`BlockTrace`) must express one in-edge of a
//! shared, multi-predecessor *return tail* as an unstructured `goto` — it has no
//! schema that duplicates a tail to avoid the goto.  On `morton::my_message_callback`
//! (x86-64) a common epilogue `free(ptr); return;` has several predecessors (each
//! command arm), so kuna emits:
//!
//! ```c
//!   else {
//!     v5 = strlen("Invalid Command");
//!     mosquitto_publish(a0, 0, "app/error", v5 & 0xffffffff, "Invalid Command", 1);
//!     goto label_41d6;          // <-- the residual unstructured edge
//!   }
//!   ...
//! label_41d6:
//!   free(v6);
//!   return;                      // <-- the shared epilogue (contains a CALL)
//! ```
//!
//! angr (zero gotos) **duplicates the `free(ptr); return;` epilogue** into the
//! `goto` source, turning the cross-edge into a structured early return.
//!
//! # Distinct from the two sibling tail-duplication passes
//!
//! This is the **third** SAILR goto-reduction pass and fills the gap between the two
//! merged siblings, both of which decline the morton shape:
//!
//! * [`crate::p8_structure::kuna_gotoreduce`] (`ActionGotoReduce`, option
//!   `gotoreduce`) duplicates a **return tail** but **rejects any
//!   `CALL`/`CALLIND`/`CALLOTHER`** in the tail — so it cannot handle a
//!   `free(p); return;` epilogue (a return tail *with* a call).
//! * [`crate::p8_structure::kuna_crossjumpreverter`] (`ActionCrossJumpReverter`,
//!   option `crossjumprevert`) permits a call but only for a **non-return,
//!   fall-through** cross-jump tail — it explicitly **declines a tail that ends in
//!   `return`** (the closed-path case).
//!
//! The morton epilogue is a **return tail containing a call**, which lands in
//! neither: this pass duplicates exactly that.  It mirrors angr's
//! `ReturnDuplicatorBase`, whose return-region size budget is measured in **calls**
//! (`max_calls_in_regions = 2`) rather than rejecting them — see
//! `angr/analyses/decompiler/optimization_passes/return_duplicator_{base,low}.py`.
//!
//! # What this pass does
//!
//! Running once after [`ActionFinalStructure`](crate::blockaction::ActionFinalStructure)
//! (the structured tree is final and goto targets are labelled), and after the two
//! sibling passes, [`ActionTailDup`] finds every unstructured `goto T` whose target
//! `T` is a small single-successor basic-block chain ending in a `return`,
//! **permitting up to [`MAX_TAIL_CALLS`] calls** (and rejecting `STORE`), and inlines
//! the tail.  Two source shapes carry the goto, and each gets the matching surgery:
//!
//! * a bare `BlockIf` `if (cond) goto T` → `if (cond) { <tail> }`
//!   ([`kuna_inline_return_tail`](crate::block::BlockGraph::kuna_inline_return_tail),
//!   gotoreduce's surgery); and
//! * a `BlockGoto` leaf `{ stmts; goto T }` → `{ stmts; <tail> }`
//!   ([`kuna_inline_crossjump_tail`](crate::block::BlockGraph::kuna_inline_crossjump_tail),
//!   crossjumprevert's surgery).  This is the **morton** case: the goto sits at the
//!   end of a multi-statement `if` arm (`if (c) { v2 = strlen(..); publish(..); goto
//!   label; }`), so the structurer wraps it in a `BlockGoto`, not a bare `BlockIf`.
//!
//! The duplicated tail is a set of fresh `BlockCopy` leaves referencing the *same*
//! underlying basic blocks (the printer re-emits their ops — a print-tree
//! duplication, semantically identical to the `goto` because the tail
//! deterministically falls through to the `return`), so no p-code is cloned and
//! SSA/def-use is untouched.  Unlike crossjumprevert's *non-return* fall-through tail,
//! a **return** tail closes its own path — there is no fall-through successor to
//! reconcile, so no convergence precondition is needed.  When the converted `goto` was
//! the only unstructured edge into `T`, `T`'s label is cleared.  Each duplication is
//! logged via [`Funcdata::warning`].
//!
//! The transform is bounded and conservative (≤ [`MAX_TAIL_BLOCKS`] blocks /
//! [`MAX_TAIL_OPS`] printed ops, ≤ [`MAX_TAIL_CALLS`] calls, no `STORE`), so
//! duplication stays cheap and side-effect-safe.  Because the duplicated tail ends
//! in a `return`, a duplicated call is reached on *exactly one* of the two structured
//! paths (the converted arm), never on both — duplicating it is semantics-preserving
//! (no extra evaluation of the call).  It is gated by `option taildup on|off`
//! (`dup_return_call_tails`, default-OFF) — when off,
//! [`ActionTailDup::apply`] early-returns and output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_flags, BlockType};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::context::BlockId;

use kuna_base::marshal::ElementId;

/// Marshaling element `<taildup>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce, 4101 tailcalljump, 4102 regionstructure, 4103 noreturn_extern,
/// 4104 noreturn_externmatch, 4105 crossjumprevert, 4106 switchsharedcase,
/// 4107 ifelseflatten are taken; this is 4108).
pub const ELEM_TAILDUP: ElementId = ElementId::new("taildup", 4108);

/// Maximum number of basic blocks in a duplicated return tail.
pub const MAX_TAIL_BLOCKS: usize = 3;
/// Maximum number of printed p-code ops across a duplicated return tail.
pub const MAX_TAIL_OPS: usize = 12;
/// Maximum number of calls in a duplicated return tail (angr
/// `ReturnDuplicatorBase.max_calls_in_regions = 2`).  This is exactly the budget
/// that distinguishes this pass from `gotoreduce` (which rejects all calls).
pub const MAX_TAIL_CALLS: usize = 2;

/// \brief Reduce gotos by duplicating a small **return tail that may contain calls**
/// into the `goto` source (angr SAILR `ReturnDuplicatorLow` / `max_calls_in_regions`).
/// Option-gated (`dup_return_call_tails`).  Fills the gap between `gotoreduce`
/// (return tail, no calls) and `crossjumprevert` (non-return tail, calls allowed).
pub struct ActionTailDup {
    base: ActionBase,
}

impl ActionTailDup {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionTailDup { base: ActionBase::new(0, "taildup", g) })
    }
}

impl Action for ActionTailDup {
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
        Some(Box::new(ActionTailDup { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option taildup on|off`, carried on the seam Architecture.
        if !data.get_arch().dup_return_call_tails {
            return 0;
        }
        if data.sblocks_get_size() == 0 {
            return 0; // not structured
        }
        let root = data.sblocks_root();

        // Find every unstructured `goto T` whose target T is a duplicable return tail
        // (a return tail that may contain calls — the morton `free(p); return;` shape).
        // Two source shapes carry the goto:
        //   * a bare `BlockIf` `if (cond) goto T`  (the gotoreduce shape), and
        //   * a `BlockGoto` leaf `{ stmts; goto T }` (the crossjumprevert shape — the
        //     morton case: `if (c) { v2 = strlen(..); publish(..); goto label; }`).
        // (src_node, bbchain, target_sblock, is_goto_node, warn_addr)
        let mut conversions: Vec<(
            BlockId,
            Vec<BlockId>,
            BlockId,
            bool,
            kuna_base::address::Address,
        )> = Vec::new();
        let nodes = collect_nodes(data, root);
        for &n in &nodes {
            let blk = data.sblocks_ref().block(n);
            // Resolve `(goto-target, is_goto_node)` for either source shape.
            let (target, is_goto_node) = match blk.get_type() {
                BlockType::If => {
                    if blk.get_if_goto_type() != block_flags::f_goto_goto {
                        continue;
                    }
                    match blk.get_if_goto_target() {
                        Some(t) => (t, false),
                        None => continue,
                    }
                }
                BlockType::Goto => {
                    if blk.get_goto_type() != block_flags::f_goto_goto {
                        continue;
                    }
                    match blk.get_goto_target() {
                        Some(t) => (t, true),
                        None => continue,
                    }
                }
                _ => continue,
            };
            let tgt = data.sblocks_ref().block(target);
            if tgt.get_type() != BlockType::Copy {
                continue;
            }
            let bb0 = match tgt.get_copy() {
                Some(b) => b,
                None => continue,
            };
            if let Some(chain) = return_call_tail_chain(data, bb0) {
                // Warn address: the head op of the duplicated tail's first block.
                let warn_addr = match data
                    .bb_op_head(bb0)
                    .and_then(|o| data.obank().get(o))
                    .map(|o| o.get_addr().clone())
                {
                    Some(a) => a,
                    None => continue,
                };
                conversions.push((n, chain, target, is_goto_node, warn_addr));
            }
        }
        if conversions.is_empty() {
            return 0;
        }

        let mut count: int4 = 0;
        let mut converted_targets: Vec<BlockId> = Vec::new();
        for (src_id, chain, target, is_goto_node, warn_addr) in &conversions {
            // Append a fresh BlockCopy tail and drop the goto.  Sound because the tail
            // ends in a `return` (a closed path: no fall-through to reconcile), so the
            // converted arm reaches the duplicated call/return on exactly one structured
            // path.  The surgery differs by source shape:
            if *is_goto_node {
                // `{ stmts; goto T }` -> `{ stmts; <tail> }`: append the tail after the
                // goto's body and switch the node from Goto to a plain list (the
                // `kuna_inline_crossjump_tail` helper — no fall-through to reconcile here
                // because the tail returns).
                data.sblocks_mut().kuna_inline_crossjump_tail(*src_id, chain);
            } else {
                // `if (cond) goto T` -> `if (cond) { <tail> }`: attach the tail as the
                // if's true clause and drop the goto target (gotoreduce's surgery).
                data.sblocks_mut().kuna_inline_return_tail(*src_id, chain);
            }
            data.warning("taildup: duplicated return-call tail to remove goto", warn_addr);
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
/// `[bb0, .., R]` where `R`'s last op is a `return`, or `None` if the run is not a
/// bounded, side-effect-safe **return-call** tail.
///
/// Unlike `return_tail_chain` in
/// [`crate::p8_structure::kuna_gotoreduce`] (gotoreduce), which rejects *any* call,
/// this permits up to [`MAX_TAIL_CALLS`] calls (angr's `max_calls_in_regions`).
/// Duplicating a call is safe **only because the tail ends in a `return`**: the
/// converted `if` arm reaches the duplicated call on exactly one structured path (the
/// early-return arm), never on both, so the call is evaluated the same number of times
/// as before.  `STORE` is still always rejected.
fn return_call_tail_chain(data: &Funcdata, bb0: BlockId) -> Option<Vec<BlockId>> {
    let mut chain: Vec<BlockId> = Vec::new();
    let mut total_ops = 0usize;
    let mut total_calls = 0usize;
    let mut cur = bb0;
    loop {
        if chain.len() >= MAX_TAIL_BLOCKS {
            return None;
        }
        // Scan the block's ops: reject anything unsafe to duplicate, count the rest.
        let mut op = data.bb_op_head(cur);
        while let Some(o) = op {
            op = data.bb_op_next(o);
            if let Some(opref) = data.obank().get(o) {
                // SSA-bookkeeping ops (MULTIEQUAL phis, INDIRECT, other nonprinting
                // markers) emit no C statement and are re-resolved by the printer on
                // the duplicated copy path — don't count them toward the size bound.
                if opref.not_printed() {
                    continue;
                }
                total_ops += 1;
                match opref.code() {
                    // STORE is never duplicated (a duplicated write is unsafe).
                    OpCode::CPUI_STORE => return None,
                    // angr permits up to max_calls_in_regions calls in the region.
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
// Option parse (`taildup on|off`).
//===========================================================================

/// The `taildup on|off` ArchOption (angr SAILR `ReturnDuplicatorLow`
/// return-tail-with-calls duplication).
pub struct OptionTailDup;

impl OptionTailDup {
    /// The option name.
    pub const NAME: &'static str = "taildup";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Return-call-tail goto reduction turned {prop}")))
    }
}
