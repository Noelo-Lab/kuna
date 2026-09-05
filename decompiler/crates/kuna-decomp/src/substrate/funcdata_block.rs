//! Port of `decompiler/cpp/funcdata_block.cc` (W3, item `w3-ir-funcdata`) — the
//! `Funcdata` methods pertaining directly to blocks.
//!
//! These are additional `impl Funcdata` methods (the struct, IR ownership, and
//! the cross-arena op-list primitives live in [`crate::funcdata`]).  They split
//! into three classes by W3-portability:
//!
//!   1. **Fully self-contained at the W3 IR level** — only the basic-block edge
//!      primitives ([`BlockGraph`] edge ops), the block `cover` ([`BasicData`]),
//!      and the op-list primitives are needed.  Faithfully ported here:
//!      `clearBlocks`, `structureReset`, `forceGoto`, `switchEdge`,
//!      `removeFromFlowSplit`, `nodeSplitBlockEdge`, `nodeJoinCreateBlock`,
//!      `setBasicBlockRange`, `installSwitchDefaults`, the block-cover helpers,
//!      and the block-splice core of `spliceBlockBasic`.
//!
//!   2. **Need the op-graph mutation API** (`opDestroy`/`opRemoveInput`/
//!      `opSetInput`/`opInsertInput`/`newOp`/`opSetOpcode`/…) — that API is the
//!      **funcdata_op** (`w3-ir-funcdata-op`) wave's, which runs AFTER this item
//!      in parallel with no boundary rights.  The methods that are *primarily*
//!      data-flow patch-up (`pushMultiequals`, `opZeroMulti`,
//!      `branchRemoveInternal`, `blockRemoveInternal`, `removeBranch`,
//!      `removeDoNothingBlock`, `removeUnreachableBlocks`, `pushBranch`,
//!      `nodeSplit`, `descend2Undef`) are boundary-noted (`// STUB(W3-op)`) with an
//!      explicit `Err` and a precise note of the missing API; the funcdata_op
//!      wave fills the bodies once it owns `opDestroy` & friends.
//!
//!   3. **Need W4 subsystems** — the jump-table methods (`linkJumpTable`,
//!      `findJumpTable`, `installJumpTable`, `recoverJumpTable`,
//!      `stageJumpTable`, `earlyJumpTableFail`, `switchOverJumpTables`)
//!      need the W4 `JumpTable`/`FlowInfo`/`ActionDatabase`.
//!      They are boundary-noted (`// STUB(W4)`); `clearJumpTables` and the dead-table
//!      sweep that `structureReset` performs operate on the opaque
//!      [`JumpTableId`](crate::funcdata::JumpTableId) handles and are carried.
//!
//! `stageJumpTable`'s Action-machinery (running the "jumptable" action set on a
//! truncated partial function) is explicitly `// STUB(W4)`: it drives
//! `glb->allacts` (the W4 `ActionDatabase`) and `truncatedFlow` (W4 flow), with
//! no W3 surface.

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::block::{block_flags, BlockKind};
use crate::funcdata::Funcdata;
use crate::context::{BlockId, OpId, VarnodeId};

/// The deepest component that performs a conditional split (C++
/// `FlowBlock::getSplitPoint`), tagged by which arena it lives in.  A leaf
/// condition resolves to a **bblocks** `BlockBasic` (carrying the CBRANCH ops);
/// a short-circuit AND/OR resolves to an **sblocks** `BlockCondition`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SplitPoint {
    /// A bblocks `BlockBasic` (`BlockBasic::getSplitPoint` / `BlockCopy::copy`).
    Basic(BlockId),
    /// An sblocks `BlockCondition` (`BlockCondition::getSplitPoint`).
    Condition(BlockId),
}

/// Encode an [`OpId`] into the opaque `usize` cursor the [`SorterFuncdata`]
/// trait carries (the C++ `PcodeOpTree::const_iterator`), and back.  OpId is a
/// slotmap key whose 64-bit ffi form fits a 64-bit `usize`.
fn op_to_cursor(op: OpId) -> usize {
    use slotmap::Key;
    op.data().as_ffi() as usize
}
fn cursor_to_op(c: usize) -> OpId {
    OpId::from(slotmap::KeyData::from_ffi(c as u64))
}

impl crate::comment::SorterFuncdata for Funcdata {
    fn get_address(&self) -> &Address {
        Funcdata::get_address(self)
    }
    fn op_tree_empty(&self) -> bool {
        self.obank().empty()
    }
    fn first_op_at_or_after(&self, addr: &Address) -> crate::comment::OpProbe {
        use crate::comment::{OpCursor, OpProbe};
        match self.obank().first_op_at_or_after(addr) {
            Some(op) => OpProbe::At(OpCursor(op_to_cursor(op))),
            None => OpProbe::AtEnd,
        }
    }
    fn prev_op(&self, probe: &crate::comment::OpProbe) -> Option<crate::comment::OpCursor> {
        use crate::comment::{OpCursor, OpProbe};
        // C++ `--opiter`, guarded by `opiter != beginOpAll()`.
        match probe {
            // At-end on a non-empty tree -> the last op.
            OpProbe::AtEnd => self.obank().last_op_all().map(|op| OpCursor(op_to_cursor(op))),
            OpProbe::At(c) => {
                let op = cursor_to_op(c.0);
                self.obank().op_before(op).map(|p| OpCursor(op_to_cursor(p)))
            }
        }
    }
    fn op_block_info(
        &self,
        cursor: &crate::comment::OpCursor,
        addr: &Address,
    ) -> KunaResult<crate::comment::OpBlockInfo> {
        let op = cursor_to_op(cursor.0);
        let o = self.obank().get(op).ok_or_else(|| {
            KunaError::lowlevel("CommentSorter: stale op cursor")
        })?;
        // A dead op (no parent) is a C++ LowlevelError.
        let parent = o
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("Dead op reaching CommentSorter"))?;
        let block_index = self.bblocks_ref().block(parent).get_index();
        let order = o.get_seq_num().get_order();
        // block->contains(addr) == cover.inRange(addr,1).
        let block_contains_comment = match self.bblocks_ref().block(parent).kind() {
            BlockKind::Basic(b) => b.cover.in_range(addr, 1),
            _ => false,
        };
        Ok(crate::comment::OpBlockInfo { block_index, order, block_contains_comment })
    }
    fn op_addr(&self, cursor: &crate::comment::OpCursor) -> Address {
        let op = cursor_to_op(cursor.0);
        self.obank().get(op).expect("op_addr: stale cursor").get_addr().clone()
    }
}

impl Funcdata {
    // -----------------------------------------------------------------------
    // Class 1: self-contained at the W3 IR level
    // -----------------------------------------------------------------------

    /// Clear all basic blocks (C++ `Funcdata::clearBlocks`, `funcdata_block.cc:34`).
    ///
    /// The C++ `bblocks.clear()`/`sblocks.clear()` empties each BlockGraph's
    /// component list and frees the blocks while keeping the BlockGraph object.
    /// Here that is a fresh empty graph (arena dropped) with a new root graph
    /// node, equivalent at the W3 IR level.
    pub fn clear_blocks(&mut self) {
        use crate::block::{BlockGraph, FlowBlock};
        let mut bb = BlockGraph::new();
        let broot = bb.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        bb.root = Some(broot);
        *self.bblocks_mut() = bb;
        let mut sb = BlockGraph::new();
        let sroot = sb.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        sb.root = Some(sroot);
        *self.sblocks_mut() = sb;
    }

    /// Sort the top-level components of the structured graph into their final
    /// printing order (C++ `BlockGraph::orderBlocks`, `block.hh:437`), the first
    /// step of `ActionFinalStructure::apply` (`blockaction.cc:2192`).
    ///
    /// Resolves the RETURN half of `FlowBlock::compareFinalOrder` here — a
    /// component's `lastOp` reaches through the `BlockCopy` leaves into `bblocks`
    /// and its opcode needs the op bank — then hands the answer to
    /// [`BlockGraph::order_blocks`](crate::block::BlockGraph::order_blocks).
    pub fn sblocks_order_blocks(&mut self) {
        let sroot = self.sblocks_root();
        let comps: Vec<BlockId> = self.sblocks_ref().block(sroot).get_list().to_vec();
        let mut ret_terminated: std::collections::BTreeSet<BlockId> =
            std::collections::BTreeSet::new();
        for bl in comps {
            let last_op = self.sblocks_ref().struct_last_op_with(Some(self.bblocks_ref()), bl);
            if let Some(op) = last_op {
                if self.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_RETURN) {
                    ret_terminated.insert(bl);
                }
            }
        }
        self.sblocks_mut().order_blocks(sroot, &ret_terminated);
    }

    /// Recurse the structured-block graph and finalize each `BlockSwitch` for
    /// printing (C++ `BlockGraph::finalizePrinting` → `BlockSwitch::finalizePrinting`,
    /// `block.cc:3607`): assign each case its label from the recovered
    /// [`JumpTable`](crate::jumptable::JumpTable), propagate labels down fall-thru
    /// chains, and stable-sort the cases by `(label, depth)`.
    ///
    /// Called from `ActionFinalStructure::apply`.  Walks every sblocks node; only
    /// `t_switch` nodes carry case state.  The JumpTable is reached via the
    /// switch's stored `jumpvec` slot.
    pub fn finalize_switch_printing(&mut self) {
        // Collect the sblocks switch node ids (immutable walk first).
        let switch_ids: Vec<BlockId> = self
            .sblocks_ref()
            .arena
            .iter()
            .filter(|(_, b)| b.switch_jt_index().is_some())
            .map(|(id, _)| id)
            .collect();
        for sw in switch_ids {
            self.finalize_one_switch(sw);
        }
    }

    /// `BlockSwitch::finalizePrinting` for a single switch node (block.cc:3607).
    fn finalize_one_switch(&mut self, sw: BlockId) {
        let jt_index = match self.sblocks_ref().block(sw).switch_jt_index() {
            Some(j) => j,
            None => return,
        };
        // Snapshot the case list (block ids + basicblocks + chains) for the label
        // computation, which needs &Funcdata for the JumpTable queries.
        let mut cases = match self.sblocks_ref().block(sw).switch_caseblocks_mut_snapshot() {
            Some(c) => c,
            None => return,
        };
        let ncase = cases.len();
        // Construct the depth parameter, to sort fall-thru cases.
        for i in 0..ncase {
            let mut j = cases[i].chain;
            while j != -1 {
                if cases[j as usize].depth != 0 {
                    break; // Break any possible loops (already visited)
                }
                cases[j as usize].depth = -1; // Mark non-roots of chains
                j = cases[j as usize].chain;
            }
        }
        for i in 0..ncase {
            let basicbl = cases[i].basicblock;
            let nind = match basicbl {
                Some(bb) => self
                    .get_jump_table(jt_index as int4)
                    .num_indices_by_block(self, bb)
                    .unwrap_or(0),
                None => 0,
            };
            if nind > 0 {
                if cases[i].depth == 0 {
                    // Only set label on chain roots.
                    let bb = basicbl.unwrap();
                    let ind = self
                        .get_jump_table(jt_index as int4)
                        .get_index_by_block(self, bb, 0)
                        .unwrap_or(0);
                    let label = self.get_jump_table(jt_index as int4).get_label_by_index(ind);
                    cases[i].label = label;
                    let mut j = cases[i].chain;
                    let mut depthcount = 1;
                    while j != -1 {
                        if cases[j as usize].depth > 0 {
                            break; // depth already set; break possible loops
                        }
                        cases[j as usize].depth = depthcount;
                        depthcount += 1;
                        cases[j as usize].label = label;
                        j = cases[j as usize].chain;
                    }
                }
            } else {
                cases[i].label = 0; // Should never happen
            }
        }
        // stable_sort by CaseOrder::compare (label, then depth).
        cases.sort_by(crate::block::CaseOrder::compare);
        if let Some(slot) = self.sblocks_mut().block_mut(sw).switch_caseblocks_mut() {
            *slot = cases;
        }
    }

    // =======================================================================
    // for-loop reroll: BlockWhileDo::finalTransform / finalizePrinting
    // (block.cc:3405 / block.cc:3454).  Drives the whiledo->for conversion by
    // discovering the loop variable, iterator, and initializer statements and
    // recording them on the structured BlockWhileDo so PrintC::emitForLoop can
    // hoist them into the `for (init; cond; iter)` header.
    // =======================================================================

    /// `BlockGraph::finalTransform` over the structured graph: recurse into
    /// every BlockWhileDo and run its `finalTransform` (C++ `block.cc:1356` +
    /// `block.cc:3405`).  Called from `ActionStructureTransform::apply`.
    pub fn finalize_forloop_transform(&mut self, allow_op_moves: bool) {
        if !self.get_arch().analyze_for_loops {
            return;
        }
        // Collect the sblocks whiledo node ids (immutable walk first), in arena
        // order.  The C++ recursion sets `f_final_transform` on each BlockGraph,
        // but the only side effects flow through the BlockWhileDo bodies, so the
        // flat scan over whiledo nodes is equivalent for the transform's result.
        let whiledo_ids: Vec<BlockId> = self
            .sblocks_ref()
            .arena
            .iter()
            .filter(|(_, b)| b.get_type() == crate::block::BlockType::WhileDo)
            .map(|(id, _)| id)
            .collect();
        for wd in whiledo_ids {
            self.whiledo_final_transform(wd, allow_op_moves);
        }
    }

    /// `BlockWhileDo::finalTransform` for one whiledo node (block.cc:3405).
    fn whiledo_final_transform(&mut self, wd: BlockId, allow_op_moves: bool) {
        let sb = self.sblocks_ref();
        if sb.block(wd).has_overflow_syntax() {
            return;
        }
        // copyBl = getFrontLeaf(); head = (BlockBasic*)copyBl->subBlock(0).
        let copy_bl = match sb.get_front_leaf(wd) {
            Some(c) => c,
            None => return,
        };
        let head = match sb.sub_block(copy_bl, 0) {
            Some(h) => h,
            None => return,
        };
        // head must be a basic block (in bblocks).
        if self.bblocks_ref().block(head).get_type() != crate::block::BlockType::Basic {
            return;
        }
        // lastOp = getBlock(1)->lastOp(); must exist for an iterator statement.
        let body = self.sblocks_ref().sub_block(wd, 1);
        let mut last_op = match body.and_then(|b| self.fd_sblock_last_op(b)) {
            Some(o) => o,
            None => return,
        };
        let tail = match self.obank().get(last_op).and_then(|o| o.get_parent()) {
            Some(t) => t,
            None => return,
        };
        if self.bblocks_ref().block(tail).size_out() != 1 {
            return;
        }
        if self.bblocks_ref().block(tail).get_out(0) != head {
            return;
        }
        // cbranch = getBlock(0)->lastOp(); must be a CBRANCH.
        let cond = match self.sblocks_ref().sub_block(wd, 0) {
            Some(c) => c,
            None => return,
        };
        let cbranch = match self.fd_sblock_last_op(cond) {
            Some(c) => c,
            None => return,
        };
        if self.obank().get(cbranch).map(|o| o.code()) != Some(OpCode::CPUI_CBRANCH) {
            return;
        }
        // If lastOp is a branch, step back: iterateOp must appear after it.
        if self.obank().get(last_op).map(|o| o.is_branch()).unwrap_or(false) {
            last_op = match self.op_previous_op(last_op) {
                Some(p) => p,
                None => return,
            };
        }

        // findLoopVariable(cbranch, head, tail, lastOp): sets iterateOp + loopDef.
        let (iterate_op, loop_def) = self.find_loop_variable(cbranch, head, tail, last_op);
        let iterate_op = match iterate_op {
            Some(o) => o,
            None => return,
        };
        let loop_def = loop_def.expect("find_loop_variable sets loopDef with iterateOp");
        // Record loopDef + iterateOp on the whiledo node.
        self.sblocks_mut().block_mut(wd).set_loop_def(Some(loop_def));
        self.sblocks_mut().block_mut(wd).set_iterate_op(Some(iterate_op));

        // Move iterateOp to be the last op if it isn't (allow_op_moves gates it).
        if iterate_op != last_op {
            if !allow_op_moves {
                self.sblocks_mut().block_mut(wd).set_iterate_op(None);
                return;
            }
            self.op_uninsert(iterate_op);
            self.op_insert_after(iterate_op, last_op);
        }

        // Set up initializer statement.
        let slot = self.bblocks_ref().block(tail).get_out_rev_index(0);
        let init_last_op = self.find_initializer(wd, head, slot);
        let init_last_op = match init_last_op {
            Some(o) => o,
            None => return,
        };
        let initialize_op =
            self.sblocks_ref().block(wd).get_initialize_op().expect("findInitializer sets it");
        if !self.op_is_moveable(initialize_op, init_last_op) {
            self.sblocks_mut().block_mut(wd).set_initialize_op(None); // Turn it off
            return;
        }
        if initialize_op != init_last_op {
            if !allow_op_moves {
                self.sblocks_mut().block_mut(wd).set_initialize_op(None);
                return;
            }
            self.op_uninsert(initialize_op);
            self.op_insert_after(initialize_op, init_last_op);
        }
    }

    /// `BlockWhileDo::findLoopVariable` (block.cc:3212).  Walks back from the
    /// loop-exit `cbranch`'s tested Varnode looking for a MULTIEQUAL in the
    /// `head` whose tail input is a moveable, non-marker op in `tail`.  Returns
    /// `(iterateOp, loopDef)` or `(None, None)`.
    fn find_loop_variable(
        &self,
        cbranch: OpId,
        head: BlockId,
        tail: BlockId,
        last_op: OpId,
    ) -> (Option<OpId>, Option<OpId>) {
        let vn = match self.obank().get(cbranch).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return (None, None),
        };
        if !self.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
            return (None, None); // No loop variable found
        }
        let op = self.vbank().get(vn).unwrap().get_def().unwrap();
        let slot = self.bblocks_ref().block(tail).get_out_rev_index(0);

        // path[4] of (op, slot); count starts at 0.
        let mut path: Vec<(OpId, int4)> = Vec::with_capacity(4);
        if self.obank().get(op).map(|o| o.is_call() || o.is_marker()).unwrap_or(true) {
            return (None, None);
        }
        path.push((op, 0));
        while !path.is_empty() {
            let count = path.len() - 1;
            let cur_op = path[count].0;
            let ind = path[count].1;
            path[count].1 += 1;
            let num_in = self.obank().get(cur_op).map(|o| o.num_input()).unwrap_or(0);
            if ind >= num_in {
                path.pop();
                continue;
            }
            let next_vn = match self.obank().get(cur_op).and_then(|o| o.get_in(ind)) {
                Some(v) => v,
                None => continue,
            };
            if !self.vbank().get(next_vn).map(|v| v.is_written()).unwrap_or(false) {
                continue;
            }
            let def_op = self.vbank().get(next_vn).unwrap().get_def().unwrap();
            if self.obank().get(def_op).map(|o| o.code()) == Some(OpCode::CPUI_MULTIEQUAL) {
                if self.obank().get(def_op).and_then(|o| o.get_parent()) != Some(head) {
                    continue;
                }
                let itvn = match self.obank().get(def_op).and_then(|o| o.get_in(slot)) {
                    Some(v) => v,
                    None => continue,
                };
                if !self.vbank().get(itvn).map(|v| v.is_written()).unwrap_or(false) {
                    continue;
                }
                let possible_iterate = self.vbank().get(itvn).unwrap().get_def().unwrap();
                if self.obank().get(possible_iterate).and_then(|o| o.get_parent()) == Some(tail) {
                    // Found proper head/tail configuration.
                    if self.obank().get(possible_iterate).map(|o| o.is_marker()).unwrap_or(true) {
                        continue; // No iteration in tail
                    }
                    if !self.op_is_moveable(possible_iterate, last_op) {
                        continue; // Not the final statement
                    }
                    return (Some(possible_iterate), Some(def_op)); // loopDef + iterateOp
                }
            } else {
                if count == 3 {
                    continue;
                }
                if self.obank().get(def_op).map(|o| o.is_call() || o.is_marker()).unwrap_or(true) {
                    continue;
                }
                path.push((def_op, 0));
            }
        }
        (None, None) // No loop variable found
    }

    /// `BlockWhileDo::findInitializer` (block.cc:3271).  The initializer must be
    /// read by `loopDef` from the non-tail slot, defined in the unique block
    /// flowing into `head`.  On success sets `initializeOp` on the whiledo node
    /// `wd` and returns the last (non-branch) op of the initializer block.
    fn find_initializer(&mut self, wd: BlockId, head: BlockId, slot: int4) -> Option<OpId> {
        if self.bblocks_ref().block(head).size_in() != 2 {
            return None;
        }
        let slot = 1 - slot;
        let loop_def = self.sblocks_ref().block(wd).get_loop_def()?;
        let init_vn = self.obank().get(loop_def).and_then(|o| o.get_in(slot))?;
        if !self.vbank().get(init_vn).map(|v| v.is_written()).unwrap_or(false) {
            return None;
        }
        let res = self.vbank().get(init_vn).unwrap().get_def().unwrap();
        if self.obank().get(res).map(|o| o.is_marker()).unwrap_or(true) {
            return None;
        }
        let initial_block = self.obank().get(res).and_then(|o| o.get_parent())?;
        // Statement must terminate in block flowing to head.
        if initial_block != self.bblocks_ref().block(head).get_in(slot) {
            return None;
        }
        let mut last_op = self.bblocks_ref().struct_last_op(initial_block)?;
        if self.bblocks_ref().block(initial_block).size_out() != 1 {
            return None; // Initializer block must flow only to the for loop
        }
        if self.obank().get(last_op).map(|o| o.is_branch()).unwrap_or(false) {
            last_op = self.op_previous_op(last_op)?;
        }
        self.sblocks_mut().block_mut(wd).set_initialize_op(Some(res));
        Some(last_op)
    }

    /// `BlockGraph::finalizePrinting` over the structured graph: recurse into
    /// every BlockWhileDo and run its `finalizePrinting` (C++ `block.cc:1368` +
    /// `block.cc:3454`).  Called from `ActionFinalStructure::apply`, after the
    /// switch case labels are finalized.
    pub fn finalize_forloop_printing(&mut self) {
        let whiledo_ids: Vec<BlockId> = self
            .sblocks_ref()
            .arena
            .iter()
            .filter(|(_, b)| b.get_type() == crate::block::BlockType::WhileDo)
            .map(|(id, _)| id)
            .collect();
        for wd in whiledo_ids {
            self.whiledo_finalize_printing(wd);
        }
    }

    /// `BlockWhileDo::finalizePrinting` for one whiledo node (block.cc:3454).
    fn whiledo_finalize_printing(&mut self, wd: BlockId) {
        let iterate_op = match self.sblocks_ref().block(wd).get_iterate_op() {
            Some(o) => o,
            None => return, // For-loop printing not enabled
        };
        let iter_parent = match self.obank().get(iterate_op).and_then(|o| o.get_parent()) {
            Some(p) => p,
            None => {
                self.sblocks_mut().block_mut(wd).set_iterate_op(None);
                return;
            }
        };
        let slot = self.bblocks_ref().block(iter_parent).get_out_rev_index(0);
        // iterateOp = testTerminal(data, slot).
        let new_iterate = self.while_test_terminal(wd, slot);
        match new_iterate {
            Some(o) => self.sblocks_mut().block_mut(wd).set_iterate_op(Some(o)),
            None => {
                self.sblocks_mut().block_mut(wd).set_iterate_op(None);
                return;
            }
        }
        if !self.while_test_iterate_form(wd) {
            self.sblocks_mut().block_mut(wd).set_iterate_op(None);
            return;
        }
        if self.sblocks_ref().block(wd).get_initialize_op().is_none() {
            // Last chance initializer (loopDef->getParent(), slot).
            let loop_def = self.sblocks_ref().block(wd).get_loop_def();
            if let Some(ld) = loop_def {
                if let Some(ld_parent) = self.obank().get(ld).and_then(|o| o.get_parent()) {
                    self.find_initializer(wd, ld_parent, slot);
                }
            }
        }
        if self.sblocks_ref().block(wd).get_initialize_op().is_some() {
            let new_init = self.while_test_terminal(wd, 1 - slot);
            self.sblocks_mut().block_mut(wd).set_initialize_op(new_init);
        }

        let iterate_op = self.sblocks_ref().block(wd).get_iterate_op();
        if let Some(o) = iterate_op {
            self.op_mark_non_printing_pub(o);
        }
        let initialize_op = self.sblocks_ref().block(wd).get_initialize_op();
        if let Some(o) = initialize_op {
            self.op_mark_non_printing_pub(o);
        }
    }

    /// `BlockWhileDo::testTerminal` (block.cc:3304).  Verify the statement read
    /// by `loopDef` from `slot` is an explicit, printed, last-in-block op (or
    /// can be moved to be so).  Returns the root explicit op or `None`.
    fn while_test_terminal(&mut self, wd: BlockId, slot: int4) -> Option<OpId> {
        let loop_def = self.sblocks_ref().block(wd).get_loop_def()?;
        let vn = self.obank().get(loop_def).and_then(|o| o.get_in(slot))?;
        if !self.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
            return None;
        }
        let final_op = self.vbank().get(vn).unwrap().get_def().unwrap();
        let loop_parent = self.obank().get(loop_def).and_then(|o| o.get_parent())?;
        let parent_block = self.bblocks_ref().block(loop_parent).get_in(slot);
        let mut res_op = final_op;
        let mut vn = vn;
        if self.obank().get(final_op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY)
            == OpCode::CPUI_COPY
            && self.obank().get(final_op).map(|o| o.not_printed()).unwrap_or(false)
        {
            vn = self.obank().get(final_op).and_then(|o| o.get_in(0))?;
            if !self.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false) {
                return None;
            }
            res_op = self.vbank().get(vn).unwrap().get_def().unwrap();
            if self.obank().get(res_op).and_then(|o| o.get_parent()) != Some(parent_block) {
                return None;
            }
        }
        if !self.vbank().get(vn).map(|v| v.is_explicit()).unwrap_or(false) {
            return None;
        }
        if self.obank().get(res_op).map(|o| o.not_printed()).unwrap_or(true) {
            return None; // Statement MUST be printed
        }
        // finalOp MUST be the last op in the block (except for the branch).
        let fparent = self.obank().get(final_op).and_then(|o| o.get_parent())?;
        let mut last_op = self.bblocks_ref().struct_last_op(fparent)?;
        if self.obank().get(last_op).map(|o| o.is_branch()).unwrap_or(false) {
            last_op = self.op_previous_op(last_op)?;
        }
        if !self.op_move_respecting_cover(final_op, last_op) {
            return None;
        }
        Some(res_op)
    }

    /// `BlockWhileDo::testIterateForm` (block.cc:3335).  Confirm the loop
    /// variable (loopDef's output HighVariable) is an input to the iterator
    /// statement.
    fn while_test_iterate_form(&self, wd: BlockId) -> bool {
        let loop_def = match self.sblocks_ref().block(wd).get_loop_def() {
            Some(o) => o,
            None => return false,
        };
        let iterate_op = match self.sblocks_ref().block(wd).get_iterate_op() {
            Some(o) => o,
            None => return false,
        };
        let target_vn = match self.obank().get(loop_def).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return false,
        };
        let high = self.vbank().get(target_vn).and_then(|v| v.get_high());

        let mut path: Vec<(OpId, int4)> = Vec::new();
        path.push((iterate_op, 0));
        while let Some(&(node_op, node_slot)) = path.last() {
            let num_in = self.obank().get(node_op).map(|o| o.num_input()).unwrap_or(0);
            if num_in <= node_slot {
                path.pop();
                continue;
            }
            let vn = self.obank().get(node_op).and_then(|o| o.get_in(node_slot));
            path.last_mut().unwrap().1 += 1;
            let vn = match vn {
                Some(v) => v,
                None => continue,
            };
            let v = self.vbank().get(vn).expect("while_test_iterate_form: stale vn");
            if v.is_annotation() {
                continue;
            }
            if v.get_high() == high {
                return true;
            }
            if v.is_explicit() {
                continue; // Truncate at explicit
            }
            if !v.is_written() {
                continue;
            }
            let def = self.vbank().get(vn).unwrap().get_def().unwrap();
            path.push((def, 0));
        }
        false
    }

    /// Clear the structured-block graph (C++ `data.getStructure().clear()`,
    /// `BlockGraph::clear`, `block.cc:1239`): empties the sblocks component list
    /// and resets its flags, so the next `ActionBlockStructure::apply` sees
    /// `sblocks.getSize()==0` and re-runs the collapse from `bblocks` (used by
    /// `ActionSwitchNorm` after `foldInGuards` rewrites the switch's default
    /// edge).  The `bblocks` graph is left intact.
    pub fn sblocks_clear(&mut self) {
        use crate::block::{BlockGraph, FlowBlock};
        let mut sb = BlockGraph::new();
        let sroot = sb.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        sb.root = Some(sroot);
        *self.sblocks_mut() = sb;
    }

    /// Clear any jump-table information, preserving overrides
    /// (C++ `Funcdata::clearJumpTables`, `funcdata_block.cc:42`).
    ///
    /// STUB(W4): the C++ keeps override tables (`jt->isOverride()`) and frees the
    /// rest.  At W3 the table contents are opaque ([`JumpTableId`]); we drop the
    /// whole vector (no W3 way to know which are overrides).  W4 reinstates the
    /// override-preserving filter.
    pub fn clear_jump_tables(&mut self) {
        // for jt in jumpvec: if jt->isOverride() keep(clear) else delete  -- STUB(W4)
        self.jumpvec_mut().clear();
    }

    /// (Re)calculate loop structure and dominance for the current control-flow
    /// graph and reset the structured hierarchy (C++ `Funcdata::structureReset`,
    /// `funcdata_block.cc:723`).
    ///
    /// The block-graph half (`structureLoops`/`calcForwardDominator`, the
    /// `blocks_unreachable` flag, `sblocks.clear()`) is faithful.  The dead
    /// jump-table sweep operates on opaque handles: the C++ drops tables whose
    /// indirect op `isDead()`.  Determining that needs the op the table points
    /// at; at W3 the table contents are behind the boundary, so the sweep is `// STUB(W4)`
    /// and the vector is left intact (no table is dropped here).  `heritage.
    /// forceRestructure()` is `// STUB(W7)`.
    pub fn structure_reset(&mut self) {
        // flags &= ~blocks_unreachable;
        self.clear_flag_raw(crate::funcdata::funcdata_flags::blocks_unreachable);

        let broot = self.bblocks_root_pub();
        let mut rootlist: Vec<BlockId> = Vec::new();
        self.bblocks_mut().structure_loops(broot, &mut rootlist);
        self.bblocks_mut().calc_forward_dominator(broot, &rootlist);
        if rootlist.len() > 1 {
            self.set_flag_raw(crate::funcdata::funcdata_flags::blocks_unreachable);
        }

        // Check for dead jumptables.  -- STUB(W4): indOp->isDead() unavailable.

        // Force structuring to start over.
        self.clear_sblocks();
        // heritage.forceRestructure(): the CFG may have changed (e.g. a block
        // removed by ActionConditionalExe), invalidating the heritage engine's
        // cached augmented dominator tree.  Forcing a rebuild on the next
        // `heritage()` pass prevents `rename_recurse` from walking a stale
        // (removed) block handle.  (Previously a `// STUB(W7)`; reached now that
        // ActionDeadCode + condexe actually mutate the CFG.)
        self.heritage_force_restructure();
    }

    /// Push a CBRANCH edge into an indirect (switch) block (C++
    /// `Funcdata::pushBranch`, `funcdata_block.cc`).
    ///
    /// Turns block `bb`'s conditional branch into an unconditional BRANCH and
    /// re-routes its `slot` out-edge onto the BRANCHIND block `bbnew`, so the
    /// guarded target becomes a destination of the switch.  Used by
    /// `JumpBasic::foldInOneGuard` when the guard targets a block not already in
    /// the switch table.
    pub fn push_branch(&mut self, bb: BlockId, slot: int4, bbnew: BlockId) -> KunaResult<()> {
        let cbranch = self
            .bb_op_tail(bb)
            .ok_or_else(|| KunaError::lowlevel("pushBranch: source block has no terminator"))?;
        let cb_ok = self.obank().get(cbranch).map(|o| o.code()) == Some(OpCode::CPUI_CBRANCH)
            && self.bblocks_ref().block(bb).size_out() == 2;
        if !cb_ok {
            return Err(KunaError::lowlevel("Cannot push non-conditional edge"));
        }
        let indop = self
            .bb_op_tail(bbnew)
            .ok_or_else(|| KunaError::lowlevel("pushBranch: target block has no terminator"))?;
        if self.obank().get(indop).map(|o| o.code()) != Some(OpCode::CPUI_BRANCHIND) {
            return Err(KunaError::lowlevel("Can only push branch into indirect jump"));
        }
        // Turn the conditional branch into an unconditional branch.
        self.op_remove_input(cbranch, 1); // Remove the conditional variable
        self.op_set_opcode_code(cbranch, OpCode::CPUI_BRANCH);
        self.bblocks_mut().move_out_edge(bb, slot, bbnew);
        // The indirect branch handles its new branch implicitly.
        self.structure_reset();
        Ok(())
    }

    /// Convert a degenerate MULTIEQUAL (after an incoming edge was severed) into
    /// the appropriate simpler op (C++ `Funcdata::opZeroMulti`,
    /// `funcdata_block.cc:195`).
    ///
    /// With no branches left the MULTIEQUAL becomes a COPY of a fresh input
    /// varnode (the value is now an input to the function); with a single branch
    /// it becomes a plain COPY of its sole input.
    pub fn op_zero_multi(&mut self, op: OpId) -> KunaResult<()> {
        let nin = self.obank().get(op).expect("opZeroMulti: stale op").num_input();
        if nin == 0 {
            let out = self
                .obank()
                .get(op)
                .expect("opZeroMulti: stale op")
                .get_out()
                .expect("opZeroMulti: MULTIEQUAL with no output");
            let sz = self.vbank().get(out).expect("opZeroMulti: stale out").get_size();
            let m = self.vbank().get(out).expect("opZeroMulti: stale out").get_addr().clone();
            let nv = self.new_varnode(sz, &m, None);
            self.op_insert_input(op, nv, 0)?;
            let in0 = self.obank().get(op).expect("opZeroMulti: stale op").get_in(0).expect("opZeroMulti: just-inserted in0");
            self.set_input_varnode(in0)?;
            self.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        } else if nin == 1 {
            self.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // BlockBasic do-nothing predicates (C++ block.cc:2626..2667)
    //
    // These mirror `BlockBasic::hasOnlyMarkers` / `isDoNothing` /
    // `unblockedMulti` / `hasNoImmediateCopy`.  In C++ they are methods on
    // `BlockBasic`, which owns the `list<PcodeOp*> op`.  In the Rust port the op
    // list is an intrusive third list driven through `Funcdata` (ADR 0001), so
    // these live on `Funcdata` as `bb_*` queries -- exactly as
    // `branch_remove_internal`/`splice_block_basic` do.  They are pure reads.
    // -----------------------------------------------------------------------

    /// Does block `bb` contain only MULTIEQUAL/INDIRECT marker ops and branches
    /// (C++ `BlockBasic::hasOnlyMarkers`, `block.cc:2626`)?
    pub fn bb_has_only_markers(&self, bb: BlockId) -> bool {
        for op in self.bb_ops(bb) {
            let o = self.obank().get(op).expect("bb_has_only_markers: stale op");
            if o.is_marker() {
                continue;
            }
            if o.is_branch() {
                continue;
            }
            return false;
        }
        true
    }

    /// Should block `bb` be removed because it does nothing useful
    /// (C++ `BlockBasic::isDoNothing`, `block.cc:2644`)?
    pub fn bb_is_do_nothing(&self, bb: BlockId) -> bool {
        let g = self.bblocks_ref();
        // No return / cbranch.
        if g.block(bb).size_out() != 1 {
            return false;
        }
        // Starting block placeholder.
        if g.block(bb).size_in() == 0 {
            return false;
        }
        // Switch-target guard: a single-out switch target that merges other edges
        // may still be propagating a unique value -- don't remove it.
        let size_in = g.block(bb).size_in();
        for i in 0..size_in {
            let switchbl = g.block(bb).get_in(i);
            if !g.block(switchbl).is_switch_out() {
                continue;
            }
            if g.block(switchbl).size_out() > 1 {
                let out0 = g.block(bb).get_out(0);
                if g.block(out0).size_in() > 1 {
                    // Multiple edges coming together -- switch edge may still be
                    // propagating a unique value.  Don't remove it.
                    return false;
                }
            }
        }
        // Don't remove single-out indirect jumps (BRANCHIND last op).
        if let Some(lastop) = self.bb_op_tail(bb) {
            if self.obank().get(lastop).expect("bb_is_do_nothing: stale op").code()
                == OpCode::CPUI_BRANCHIND
            {
                return false;
            }
        }
        self.bb_has_only_markers(bb)
    }

    /// Can block `bb` be removed (collapsing into out edge `outslot`) without
    /// introducing inconsistent redundant MULTIEQUAL entries (C++
    /// `BlockBasic::unblockedMulti`, `block.cc:2561`)?
    pub fn bb_unblocked_multi(&self, bb: BlockId, outslot: int4) -> bool {
        let g = self.bblocks_ref();
        let blout = g.block(bb).get_out(outslot);
        // Build list of blocks which would have redundant branches into blout.
        let mut redundlist: Vec<BlockId> = Vec::new();
        let size_in = g.block(bb).size_in();
        for i in 0..size_in {
            let bl = g.block(bb).get_in(i);
            let bl_size_out = g.block(bl).size_out();
            for j in 0..bl_size_out {
                if g.block(bl).get_out(j) == blout {
                    redundlist.push(bl);
                }
            }
        }
        if redundlist.is_empty() {
            return true;
        }
        let in_index_to_this = g.block(blout).get_in_index(bb);
        for multiop in self.bb_ops(blout) {
            let mop = self.obank().get(multiop).expect("bb_unblocked_multi: stale op");
            if mop.code() != OpCode::CPUI_MULTIEQUAL {
                continue;
            }
            for &bl in &redundlist {
                let redund_slot = g.block(blout).get_in_index(bl);
                let vnredund = mop.get_in(redund_slot);
                let mut vnremove = mop.get_in(in_index_to_this);
                // If vnremove is written by a MULTIEQUAL in -bb-, dereference it.
                if let Some(vr) = vnremove {
                    let vnobj = self.vbank().get(vr).expect("bb_unblocked_multi: stale vn");
                    if vnobj.is_written() {
                        let othermulti =
                            vnobj.get_def().expect("bb_unblocked_multi: written vn has def");
                        let omop =
                            self.obank().get(othermulti).expect("bb_unblocked_multi: stale def");
                        if omop.code() == OpCode::CPUI_MULTIEQUAL
                            && omop.get_parent() == Some(bb)
                        {
                            vnremove = omop.get_in(g.block(bb).get_in_index(bl));
                        }
                    }
                }
                if vnremove != vnredund {
                    return false; // Redundant branches must be identical
                }
            }
        }
        true
    }

    /// Was there an immediate COPY propagation out of `bb` into a MULTIEQUAL in
    /// the immediate out block (C++ `BlockBasic::hasNoImmediateCopy`,
    /// `block.cc:2605`)?  Returns \b true if there was \e no immediate copy.
    pub fn bb_has_no_immediate_copy(&self, bb: BlockId, outslot: int4) -> bool {
        let g = self.bblocks_ref();
        if !g.block(bb).has_immed_copy_edge(outslot) {
            return true;
        }
        let blout = g.block(bb).get_out(outslot);
        let in_index_to_this = g.block(blout).get_in_index(bb);
        for op in self.bb_ops(blout) {
            let mop = self.obank().get(op).expect("bb_has_no_immediate_copy: stale op");
            if mop.code() != OpCode::CPUI_MULTIEQUAL {
                continue;
            }
            if self.op_has_copy_immed(op, in_index_to_this) {
                return false;
            }
        }
        true
    }

    /// Is basic block `bb` too complex to be folded into a condition (C++
    /// `BlockBasic::isComplex`, `block.cc:2403`)?  Counts the "statements" in the
    /// block (a calculation whose output is used outside the block, addr-tied, or
    /// dead, plus calls and branches); the block is complex once the count
    /// exceeds 2.
    ///
    /// Used by [`CollapseStructure::rule_block_or`](crate::blockaction) to decide
    /// whether the OR-clause block can be absorbed.  Precomputed per BlockBasic in
    /// `ActionBlockStructure::apply` and threaded into `CollapseStructure` (the
    /// structuring graph is the BlockCopy mirror, which does not own the op list).
    pub fn bb_is_complex(&self, bb: BlockId) -> bool {
        let mut statement: int4 = 0;
        // The branch counts as a statement.
        if self.bblocks_ref().block(bb).size_out() >= 2 {
            statement = 1;
        }
        let maxref = self.get_arch().max_implied_ref;
        for inst in self.bb_ops(bb) {
            let instobj = self.obank().get(inst).expect("bb_is_complex: stale op");
            if instobj.is_marker() {
                continue;
            }
            let vn = instobj.get_out();
            if instobj.is_call() {
                statement += 1;
            } else if vn.is_none() {
                if instobj.is_flow_break() {
                    continue;
                }
                statement += 1;
            } else {
                // A calculation with output: conservative Varnode::calc_explicit.
                let outvn = vn.expect("bb_is_complex: out present");
                let vnobj = self.vbank().get(outvn).expect("bb_is_complex: stale out vn");
                let mut yesstatement = false;
                if vnobj.has_no_descend() {
                    yesstatement = true;
                } else if vnobj.is_addr_tied() {
                    // Being conservative.
                    yesstatement = true;
                } else {
                    let mut totalref: int4 = 0;
                    for d_op in vnobj.descend_iter() {
                        let dobj = self.obank().get(d_op).expect("bb_is_complex: stale descend");
                        if dobj.is_marker() || dobj.get_parent() != Some(bb) {
                            // Variable used outside of block.
                            yesstatement = true;
                            break;
                        }
                        totalref += 1;
                        if totalref > maxref {
                            yesstatement = true;
                            break;
                        }
                    }
                }
                if yesstatement {
                    statement += 1;
                }
            }
            if statement > 2 {
                return true;
            }
        }
        false
    }

    /// Did a COPY propagation from the immediate input block at `slot` of `op`
    /// happen (C++ `PcodeOp::hasCopyImmed`, `op.cc:139`)?  Needs `op`'s parent
    /// block (to resolve the in-edge), so it lives on `Funcdata`.
    fn op_has_copy_immed(&self, op: OpId, slot: int4) -> bool {
        let o = self.obank().get(op).expect("op_has_copy_immed: stale op");
        if (o.get_addlflags() & crate::op::pcodeop_addlflags::immed_copy) == 0 {
            return false;
        }
        let parent = match o.get_parent() {
            Some(p) => p,
            None => return false,
        };
        let g = self.bblocks_ref();
        let inbl = g.block(parent).get_in(slot);
        let outedge = g.block(parent).get_in_rev_index(slot);
        g.block(inbl).has_immed_copy_edge(outedge)
    }

    /// The last PcodeOp of a structured **sblocks** FlowBlock, threading the
    /// sblocks→bblocks cross-arena `BlockCopy` hop (C++ `FlowBlock::lastOp`,
    /// virtual per subtype, `block.cc`/`block.hh:546`).
    ///
    /// [`BlockGraph::struct_last_op`](crate::block::BlockGraph::struct_last_op)
    /// resolves the structured sblocks subtypes (`t_ls`/`t_condition`/`t_if`),
    /// but lacks the `t_copy` arm (`BlockCopy::lastOp` = `copy->lastOp()`,
    /// `block.hh:546`) because a `BlockCopy`'s `copy` points into the **bblocks**
    /// arena, which the sblocks `BlockGraph` cannot reach.  This dispatches the
    /// structured arms by recursion and resolves a `t_copy` leaf to the wrapped
    /// bblocks `BlockBasic`'s tail op.
    pub fn fd_sblock_last_op(&self, this_id: BlockId) -> Option<OpId> {
        let sb = self.sblocks_ref();
        match sb.block(this_id).get_type() {
            // BlockCopy: hop to the wrapped bblocks block.
            crate::block::BlockType::Copy => {
                let bb = sb.block(this_id).get_copy()?;
                // The wrapped block is a bblocks BlockBasic.
                self.bb_op_tail(bb)
            }
            // BlockBasic: same arena, direct tail.
            crate::block::BlockType::Basic => self.bb_op_tail(this_id),
            crate::block::BlockType::Ls => {
                let n = sb.block(this_id).get_size();
                if n == 0 {
                    return None;
                }
                let last = sb.sub_block(this_id, n - 1)?;
                self.fd_sblock_last_op(last)
            }
            crate::block::BlockType::Condition => {
                let b1 = sb.sub_block(this_id, 1)?;
                self.fd_sblock_last_op(b1)
            }
            // BlockIf: only an ifgoto (size 1) has a last op.
            crate::block::BlockType::If => {
                if sb.block(this_id).get_size() == 1 {
                    let b0 = sb.sub_block(this_id, 0)?;
                    self.fd_sblock_last_op(b0)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Mark that a COPY propagation from the immediate input block at `slot` of
    /// `op` has happened (C++ `PcodeOp::setCopyImmed`, `op.cc:128`).  Needs
    /// `op`'s parent block (to resolve the in-edge), so it lives on `Funcdata`.
    pub fn op_set_copy_immed(&mut self, op: OpId, slot: int4) {
        let parent = match self.obank().get(op).and_then(|o| o.get_parent()) {
            Some(p) => p,
            None => return,
        };
        let (inbl, outedge) = {
            let g = self.bblocks_ref();
            (g.block(parent).get_in(slot), g.block(parent).get_in_rev_index(slot))
        };
        self.bblocks_mut().set_immed_copy_edge(inbl, outedge);
        if let Some(o) = self.obank_mut().get_mut(op) {
            o.set_additional_flag(crate::op::pcodeop_addlflags::immed_copy);
        }
    }

    /// Remove an outgoing branch of the given basic block, patching MULTIEQUAL
    /// ops in the target block (C++ `Funcdata::branchRemoveInternal`,
    /// `funcdata_block.cc:213`).
    ///
    /// \param bb is the given basic block
    /// \param num is the index of the outgoing edge to remove
    pub fn branch_remove_internal(&mut self, bb: BlockId, num: int4) -> KunaResult<()> {
        // No decision left.
        if self.bblocks_ref().block(bb).size_out() == 2 {
            if let Some(lastop) = self.bb_op_tail(bb) {
                self.op_destroy(lastop);
            }
        }
        let bbout = self.bblocks_ref().block(bb).get_out(num);
        let blocknum = self.bblocks_ref().block(bbout).get_in_index(bb);
        // Sever (one) connection.
        self.bblocks_mut().remove_edge(bb, bbout);
        for op in self.bb_ops(bbout) {
            if self.obank().get(op).expect("branchRemoveInternal: stale op").code() != OpCode::CPUI_MULTIEQUAL {
                break;
            }
            self.op_remove_input(op, blocknum);
            self.op_zero_multi(op)?;
        }
        Ok(())
    }

    /// Remove the indicated branch and rebuild the structured-block state (C++
    /// `Funcdata::removeBranch`, `funcdata_block.cc:`).
    pub fn remove_branch(&mut self, bb: BlockId, num: int4) -> KunaResult<()> {
        self.branch_remove_internal(bb, num)?;
        self.structure_reset();
        Ok(())
    }

    /// Build a replacement Varnode for `origvn` (C++ `Funcdata::createReplaceVarnode`,
    /// `funcdata_block.cc:86`).  Used by [`Funcdata::push_multiequals`] when a
    /// MULTIEQUAL output is read beyond the block being removed.
    fn create_replace_varnode(&mut self, origvn: VarnodeId, make_unique: bool) -> VarnodeId {
        let (sz, ty) = {
            let v = self.vbank().get(origvn).expect("createReplaceVarnode: stale origvn");
            (v.get_size(), v.get_type().clone())
        };
        let replacevn = if make_unique {
            self.new_unique(sz, Some(ty))
        } else {
            let m = self.vbank().get(origvn).expect("createReplaceVarnode: stale origvn").get_addr().clone();
            self.new_varnode(sz, &m, Some(ty))
        };
        if self.is_high_on() {
            self.vn_replace_in_high(origvn, replacevn);
            self.vbank_mut().get_mut(replacevn).expect("createReplaceVarnode").set_explicit();
        }
        replacevn
    }

    /// Swap `origvn` with the freshly created `replacevn` between their high
    /// variables (C++ `Varnode::replaceInHigh`, `varnode.cc:353`), across the
    /// `high_bank`/`vbank` field split.  The cross-arena `vn->setHigh` writes are
    /// deferred to a log and applied after the borrow (same idiom as the merge).
    fn vn_replace_in_high(&mut self, origvn: VarnodeId, replacevn: VarnodeId) {
        let orig_high = match self.vbank().get(origvn).and_then(|v| v.get_high()) {
            Some(h) => h,
            None => return, // origvn has no high (annotation etc.); nothing to swap
        };
        let replace_high = match self.vbank().get(replacevn).and_then(|v| v.get_high()) {
            Some(h) => h,
            None => return,
        };
        let orig_mergegroup =
            self.vbank().get(origvn).expect("vn_replace_in_high: stale origvn").get_merge_group();
        // vn->getSymbolEntry() is a W4 surface (no Varnode-Symbol link in the
        // merged tree), so the symbol-dirty trigger is conservatively false.
        let vn_has_symbol_entry = false;
        let mut set_high_log: Vec<(VarnodeId, crate::context::HighVariableId, kuna_base::types::int2)> =
            Vec::new();
        self.with_high_split(|hb, ctx| {
            hb.replace_in_high(
                origvn,
                orig_high,
                orig_mergegroup,
                replacevn,
                replace_high,
                vn_has_symbol_entry,
                ctx,
                &mut |vn, id, mg| set_high_log.push((vn, id, mg)),
            );
        });
        for (vn, id, mg) in set_high_log {
            if let Some(v) = self.vbank_mut().get_mut(vn) {
                v.set_high(id, mg);
            }
        }
    }

    /// Force any Varnode defined by a MULTIEQUAL in the (about-to-be-removed)
    /// block `bb` to be defined in the output block instead (C++
    /// `Funcdata::pushMultiequals`, `funcdata_block.cc:105`).
    pub fn push_multiequals(&mut self, bb: BlockId) -> KunaResult<()> {
        if self.bblocks_ref().block(bb).size_out() == 0 {
            return Ok(());
        }
        if self.bblocks_ref().block(bb).size_out() > 1 {
            // A do-nothing block has exactly one out (isDoNothing guard), so this
            // never fires on the removeDoNothingBlock path.  The warning facility
            // is a W4 surface; the C++ continues past it (non-fatal).
        }
        let outblock = self.bblocks_ref().block(bb).get_out(0);
        let outblock_ind = self.bblocks_ref().block(bb).get_out_rev_index(0);

        for origop in self.bb_ops(bb) {
            if self.obank().get(origop).expect("pushMultiequals: stale op").code()
                != OpCode::CPUI_MULTIEQUAL
            {
                continue;
            }
            let origvn = self
                .obank()
                .get(origop)
                .expect("pushMultiequals")
                .get_out()
                .expect("pushMultiequals: MULTIEQUAL with no output");
            if self.vbank().get(origvn).expect("pushMultiequals: stale origvn").has_no_descend() {
                continue;
            }
            // Scan descendants: does anything read origvn NOT through the dead edge?
            let mut needreplace = false;
            let mut neednewunique = false;
            for op in self.vbank().get(origvn).expect("pushMultiequals").descend_iter() {
                let opobj = self.obank().get(op).expect("pushMultiequals: stale descend op");
                if opobj.code() == OpCode::CPUI_MULTIEQUAL && opobj.get_parent() == Some(outblock) {
                    let mut dead_edge = true; // ref to origvn NOT thru the dead edge?
                    let ni = opobj.num_input();
                    for i in 0..ni {
                        if i == outblock_ind {
                            continue; // The dead edge
                        }
                        if opobj.get_in(i) == Some(origvn) {
                            dead_edge = false;
                            break;
                        }
                    }
                    if dead_edge {
                        // If origvn is addrtied and feeds a same-address MULTIEQUAL
                        // in outblock, the new MULTIEQUAL must write a fresh unique.
                        let out_of_op = opobj.get_out();
                        let origvn_addr_tied =
                            self.vbank().get(origvn).expect("pushMultiequals").is_addr_tied();
                        if origvn_addr_tied {
                            if let Some(oo) = out_of_op {
                                let same_addr = self.vbank().get(oo).expect("pushMultiequals").get_addr()
                                    == self.vbank().get(origvn).expect("pushMultiequals").get_addr();
                                if same_addr {
                                    neednewunique = true;
                                }
                            }
                        }
                        continue;
                    }
                }
                needreplace = true;
                break;
            }
            if !needreplace {
                continue;
            }
            // Construct artificial MULTIEQUAL in outblock.
            let replacevn = self.create_replace_varnode(origvn, neednewunique);
            let mut branches: Vec<VarnodeId> = Vec::new();
            let outblock_size_in = self.bblocks_ref().block(outblock).size_in();
            for i in 0..outblock_size_in {
                if self.bblocks_ref().block(outblock).get_in(i) == bb {
                    branches.push(origvn);
                } else {
                    branches.push(replacevn);
                }
            }
            let start = self.bb_get_start(outblock);
            let replaceop = self.new_op(branches.len() as int4, start);
            self.op_set_opcode(replaceop, crate::typeop::type_op_for(OpCode::CPUI_MULTIEQUAL));
            self.op_set_output(replaceop, replacevn)?;
            self.op_set_all_input(replaceop, &branches)?;
            self.op_insert_begin(replaceop, outblock);

            // Replace obsolete origvn with replacevn in all descendant reads,
            // except the dead-edge slot of the outblock MULTIEQUAL itself.
            let descenders: Vec<OpId> =
                self.vbank().get(origvn).expect("pushMultiequals").descend_iter().collect();
            for op in descenders {
                let (ni, parent, opc) = {
                    let o = self.obank().get(op).expect("pushMultiequals: stale descend");
                    (o.num_input(), o.get_parent(), o.code())
                };
                for i in 0..ni {
                    if self.obank().get(op).expect("pushMultiequals").get_in(i) != Some(origvn) {
                        continue;
                    }
                    if i == outblock_ind
                        && parent == Some(outblock)
                        && opc == OpCode::CPUI_MULTIEQUAL
                    {
                        continue;
                    }
                    self.op_set_input(op, replacevn, i)?;
                    break;
                }
            }
        }
        Ok(())
    }

    /// Rewrite every read of `vn` so it instead reads a special "undefined"
    /// constant (C++ `Funcdata::descend2Undef`, `funcdata_varnode.cc:573`).
    ///
    /// Used by [`remove_unreachable_blocks`](Funcdata::remove_unreachable_blocks):
    /// when a block is deleted, any Varnode it defined that is still read from a
    /// (possibly reachable) block has its descendants rewired to a `0xBADDEF`
    /// constant.  MULTIEQUAL/INDIRECT cannot take a constant directly, so a COPY
    /// of the constant is interposed.  Returns `true` if any rewritten reader is
    /// in a block that is not itself fully orphaned (i.e. has in-edges), which the
    /// caller turns into a one-time "creating undefined varnodes" warning.
    pub fn descend2_undef(&mut self, vn: VarnodeId) -> KunaResult<bool> {
        let sz = self.vbank().get(vn).expect("descend2Undef: stale vn").get_size();
        // Snapshot the descendant list: we rewrite reads (and create new ops)
        // while walking, so we cannot hold the live iterator (C++ advances the
        // iterator past the current op before mutating).
        let descend: Vec<OpId> =
            self.vbank().get(vn).expect("descend2Undef: stale vn").descend_iter().collect();
        let mut res = false;
        for op in descend {
            let parent = match self.obank().get(op).expect("descend2Undef: stale op").get_parent() {
                Some(p) => p,
                None => continue,
            };
            if self.bblocks_ref().block(parent).is_dead() {
                continue;
            }
            if self.bblocks_ref().block(parent).size_in() != 0 {
                res = true;
            }
            let i = self.obank().get(op).expect("descend2Undef: stale op").get_slot(vn);
            let opc = self.obank().get(op).expect("descend2Undef: stale op").code();
            let badconst = self.new_constant(sz, 0xBADDEF);
            match opc {
                OpCode::CPUI_MULTIEQUAL => {
                    // Cannot put a constant directly into a MULTIEQUAL: interpose
                    // a COPY at the end of the corresponding predecessor block.
                    let inbl = self.bblocks_ref().block(parent).get_in(i);
                    let start = self.bb_get_start(inbl);
                    let copyop = self.new_op(1, start);
                    let inputvn = self.new_unique_out(sz, copyop)?;
                    self.op_set_opcode_code(copyop, OpCode::CPUI_COPY);
                    self.op_set_input(copyop, badconst, 0)?;
                    self.op_insert_end(copyop, inbl);
                    self.op_set_input(op, inputvn, i)?;
                }
                OpCode::CPUI_INDIRECT => {
                    // Cannot put a constant directly into an INDIRECT: interpose a
                    // COPY just before the INDIRECT op.
                    let addr =
                        self.obank().get(op).expect("descend2Undef: stale op").get_addr().clone();
                    let copyop = self.new_op(1, addr);
                    let inputvn = self.new_unique_out(sz, copyop)?;
                    self.op_set_opcode_code(copyop, OpCode::CPUI_COPY);
                    self.op_set_input(copyop, badconst, 0)?;
                    self.op_insert_before(copyop, op);
                    self.op_set_input(op, inputvn, i)?;
                }
                _ => {
                    self.op_set_input(op, badconst, i)?;
                }
            }
        }
        Ok(res)
    }

    /// Does the given Varnode have any descendant op in a block \e not marked
    /// dead (C++ `Funcdata::descendantsOutside`, `funcdata_block.cc:251`)?
    fn descendants_outside(&self, vn: VarnodeId) -> bool {
        for op in self.vbank().get(vn).expect("descendantsOutside: stale vn").descend_iter() {
            let parent = self.obank().get(op).expect("descendantsOutside: stale op").get_parent();
            match parent {
                Some(p) => {
                    if !self.bblocks_ref().block(p).is_dead() {
                        return true;
                    }
                }
                None => return true, // No parent => not in a dead block
            }
        }
        false
    }

    /// Remove an active basic block from the function, patching up data-flow
    /// (C++ `Funcdata::blockRemoveInternal`, `funcdata_block.cc:272`).
    ///
    /// `unreachable` selects the warning/`descend2Undef` arm used by
    /// `removeUnreachableBlocks`; `removeDoNothingBlock` passes `false`.
    pub fn block_remove_internal(&mut self, bb: BlockId, unreachable: bool) -> KunaResult<()> {
        // BRANCHIND last op -> drop its jump table: the ops below are destroyed,
        // so a surviving table would point at a dead BRANCHIND.
        if let Some(op) = self.bb_op_tail(bb) {
            if self.obank().get(op).expect("blockRemoveInternal: stale op").code()
                == OpCode::CPUI_BRANCHIND
            {
                if let Some(idx) = self.find_jump_table_index(op) {
                    self.remove_jump_table(idx);
                }
            }
        }
        if !unreachable {
            self.push_multiequals(bb)?; // Make sure data flow is preserved

            let size_out = self.bblocks_ref().block(bb).size_out();
            for i in 0..size_out {
                let bbout = self.bblocks_ref().block(bb).get_out(i);
                if self.bblocks_ref().block(bbout).is_dead() {
                    continue;
                }
                let blocknum = self.bblocks_ref().block(bbout).get_in_index(bb);
                for op in self.bb_ops(bbout) {
                    if self.obank().get(op).expect("blockRemoveInternal").code()
                        != OpCode::CPUI_MULTIEQUAL
                    {
                        continue;
                    }
                    let deadvn = self
                        .obank()
                        .get(op)
                        .expect("blockRemoveInternal")
                        .get_in(blocknum)
                        .expect("blockRemoveInternal: MULTIEQUAL slot");
                    self.op_remove_input(op, blocknum); // Remove the deleted block's branch
                    let deadop = self.vbank().get(deadvn).expect("blockRemoveInternal").get_def();
                    let written =
                        self.vbank().get(deadvn).expect("blockRemoveInternal").is_written();
                    let dead_is_multi_in_bb = match deadop {
                        Some(dop) => {
                            let d = self.obank().get(dop).expect("blockRemoveInternal");
                            d.code() == OpCode::CPUI_MULTIEQUAL && d.get_parent() == Some(bb)
                        }
                        None => false,
                    };
                    let bb_size_in = self.bblocks_ref().block(bb).size_in();
                    if written && dead_is_multi_in_bb {
                        // Append new branches from deadop's inputs.
                        let dop = deadop.expect("blockRemoveInternal: deadop present");
                        for j in 0..bb_size_in {
                            let v = self
                                .obank()
                                .get(dop)
                                .expect("blockRemoveInternal")
                                .get_in(j)
                                .expect("blockRemoveInternal: deadop slot");
                            let ni = self.obank().get(op).expect("blockRemoveInternal").num_input();
                            self.op_insert_input(op, v, ni)?;
                        }
                    } else {
                        // Otherwise make copies of deadvn.
                        for _j in 0..bb_size_in {
                            let ni = self.obank().get(op).expect("blockRemoveInternal").num_input();
                            self.op_insert_input(op, deadvn, ni)?;
                        }
                    }
                    self.op_zero_multi(op)?;
                }
            }
        }
        self.bblocks_mut().remove_from_flow(bb);

        // Finally remove all the ops in -bb-.
        let mut desc_warning = false;
        for op in self.bb_ops(bb) {
            if self.obank().get(op).expect("blockRemoveInternal").is_assignment() {
                let deadvn = self
                    .obank()
                    .get(op)
                    .expect("blockRemoveInternal")
                    .get_out()
                    .expect("blockRemoveInternal: assignment with no out");
                if unreachable {
                    // Mark descendants in (possibly) reachable blocks as undefined.
                    let undef = self.descend2_undef(deadvn)?;
                    if undef && !desc_warning {
                        // W4 warning facility; the descend2Undef rewrite is the
                        // realized side effect.  Print once.
                        desc_warning = true;
                    }
                }
                if self.descendants_outside(deadvn) {
                    return Err(KunaError::lowlevel("Deleting op with descendants"));
                }
            }
            if self.obank().get(op).expect("blockRemoveInternal").is_call() {
                // Prune the call from the call-spec registry.
                self.delete_call_specs(op);
            }
            self.op_destroy(op); // No longer has descendants
        }
        let graph = self.bblocks_root_pub();
        self.bblocks_mut().remove_block(graph, bb); // Remove the block altogether
        Ok(())
    }

    /// Remove a basic block that performs no operations from control-flow (C++
    /// `Funcdata::removeDoNothingBlock`, `funcdata_block.cc:345`).
    ///
    /// The block must contain only marker ops (MULTIEQUAL/INDIRECT) and possibly
    /// a single unconditional branch.  Forces a structuring reset.
    pub fn remove_do_nothing_block(&mut self, bb: BlockId) -> KunaResult<()> {
        if self.bblocks_ref().block(bb).size_out() > 1 {
            return Err(KunaError::lowlevel(
                "Cannot delete a reachable block unless it has 1 out or less",
            ));
        }
        self.bblocks_mut().block_mut(bb).set_dead();
        self.block_remove_internal(bb, false)?;
        self.structure_reset(); // Delete any structure we had before
        Ok(())
    }

    /// Remove any unreachable basic blocks (C++ `Funcdata::removeUnreachableBlocks`,
    /// `funcdata_block.cc:364`).
    ///
    /// `checkexistence` forces an active scan (a non-entry block with no immediate
    /// dominator is unreachable); otherwise the cached `blocks_unreachable` flag
    /// (set by [`structure_reset`](Self::structure_reset)) gates the work.  When
    /// unreachable blocks exist they are collected from the entry's reachable set,
    /// marked dead, severed from control-flow ([`branch_remove_internal`]), and
    /// then deleted ([`block_remove_internal`] with `unreachable=true`, which
    /// rewires stranded reads via [`descend2_undef`]).  A structuring reset follows.
    /// Returns `true` if any block was removed.
    pub fn remove_unreachable_blocks(
        &mut self,
        issuewarning: bool,
        checkexistence: bool,
    ) -> KunaResult<bool> {
        let _ = issuewarning; // W4 warning facility; the block deletion is realized.
        let size = self.bblocks_get_size();
        if checkexistence {
            // Quick scan: a non-entry block with no immediate dominator is
            // unreachable.
            let mut found = false;
            for i in 0..size {
                let blk = self.bblocks_get_block(i);
                if self.bblocks_ref().block(blk).is_entry_point() {
                    continue;
                }
                if self.bblocks_ref().block(blk).get_immed_dom().is_none() {
                    found = true;
                    break;
                }
            }
            if !found {
                return Ok(false);
            }
        } else if !self.has_unreachable_blocks() {
            return Ok(false);
        }

        // There is at least one unreachable block.  Find the entry point.
        let mut entry: Option<BlockId> = None;
        for i in 0..size {
            let blk = self.bblocks_get_block(i);
            if self.bblocks_ref().block(blk).is_entry_point() {
                entry = Some(blk);
                break;
            }
        }
        let entry = match entry {
            // No entry point => nothing reachable to anchor the sweep on.
            None => return Ok(false),
            Some(e) => e,
        };

        // Collect the unreachable blocks (everything not reachable from entry).
        let graph = self.bblocks_root_pub();
        let mut list: Vec<BlockId> = Vec::new();
        self.bblocks_mut().collect_reachable(graph, &mut list, entry, true);
        if list.is_empty() {
            return Ok(false);
        }

        // Mark them all dead first (so the later edge/op surgery sees a consistent
        // dead/alive partition; descend2Undef skips reads inside dead blocks).
        for &bb in &list {
            self.bblocks_mut().block_mut(bb).set_dead();
            // W4 warning facility.
        }
        // Sever every out-edge of every dead block.
        for &bb in &list {
            while self.bblocks_ref().block(bb).size_out() > 0 {
                self.branch_remove_internal(bb, 0)?;
            }
        }
        // Finally remove each dead block's ops and the block itself.
        for &bb in &list {
            self.block_remove_internal(bb, true)?;
        }
        self.structure_reset();
        Ok(true)
    }

    /// The data-flow half of `BlockBasic::negateCondition` (C++ `block.cc:2355`):
    /// flip the trailing CBRANCH's `boolean_flip` and `fallthru_true` flags, then
    /// swap the (bblocks) block's out-edge order.
    ///
    /// Called by [`ActionBlockStructure`](crate::blockaction::ActionBlockStructure)
    /// to realize the negation requests the structured collapse recorded against
    /// the underlying `BlockBasic` (the dual-arena `BlockCopy::copy` target).
    pub fn block_basic_negate_lastop(&mut self, bb: BlockId) {
        if let Some(lastop) = self.bb_op_tail(bb) {
            let o = self.obank_mut().get_mut(lastop).expect("negate_lastop: stale op");
            o.flip_flag(crate::op::pcodeop_flags::boolean_flip);
            o.flip_flag(crate::op::pcodeop_flags::fallthru_true);
        }
        // Flip the out-edge order.
        self.bblocks_mut().swap_edges(bb);
    }

    /// Set the initial ownership range for a basic block (C++
    /// `Funcdata::setBasicBlockRange` -> `BlockBasic::setInitialRange`,
    /// `block.cc:2673`).  Replaces the block's `cover` with the single range
    /// `[beg, end]`.
    pub fn set_basic_block_range(&mut self, bb: BlockId, beg: &Address, end: &Address) {
        let cover = &mut self.basic_cover_mut(bb);
        cover.clear();
        let spc = beg.get_space().expect("setBasicBlockRange: invalid address (C++ UB)");
        cover.insert_range(spc.clone(), beg.get_offset(), end.get_offset());
    }

    /// Force a control-flow edge to be marked \e unstructured
    /// (C++ `Funcdata::forceGoto`, `funcdata_block.cc:760`).
    ///
    /// Scans basic blocks for the one whose last op is at `pcop`, then the out
    /// edge whose target block's last op is at `pcdest`, and marks that edge a
    /// goto branch.  Returns `true` if an edge was labeled.
    pub fn force_goto(&mut self, pcop: &Address, pcdest: &Address) -> KunaResult<bool> {
        let n = self.bblocks_get_size();
        for i in 0..n {
            let bl = self.bblocks_get_block(i);
            let op = match self.bb_op_tail(bl) {
                Some(o) => o,
                None => continue,
            };
            if self.obank().get(op).expect("forceGoto").get_addr() != pcop {
                continue;
            }
            let sizeout = self.bblocks_ref().block(bl).size_out();
            for j in 0..sizeout {
                let bl2 = self.bblocks_ref().block(bl).get_out(j);
                let op2 = match self.bb_op_tail(bl2) {
                    Some(o) => o,
                    None => continue,
                };
                if self.obank().get(op2).expect("forceGoto").get_addr() != pcdest {
                    continue;
                }
                self.bblocks_mut().set_goto_branch(bl, j)?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Switch an outgoing edge from `inblock` to flow into `outafter` instead of
    /// `outbefore` (C++ `Funcdata::switchEdge`, `funcdata_block.cc:916`).
    /// Does \e not adjust MULTIEQUAL data-flow.
    pub fn switch_edge(&mut self, inblock: BlockId, outbefore: BlockId, outafter: BlockId) {
        self.bblocks_mut().switch_edge(inblock, outbefore, outafter);
        self.structure_reset();
    }

    /// Remove a basic block splitting its control-flow into two distinct paths
    /// (C++ `Funcdata::removeFromFlowSplit`, `funcdata_block.cc:900`).
    ///
    /// The given block must have 2 inputs / 2 outputs and \b no operations.
    pub fn remove_from_flow_split(&mut self, bl: BlockId, swap: bool) -> KunaResult<()> {
        if !self.bb_empty_op(bl) {
            return Err(KunaError::lowlevel("Can only split the flow for an empty block"));
        }
        self.bblocks_mut().remove_from_flow_split(bl, swap);
        let graph = self.bblocks_root_pub();
        self.bblocks_mut().remove_block(graph, bl);
        self.structure_reset();
        Ok(())
    }

    /// Make sure default switch cases are properly labeled
    /// (C++ `Funcdata::installSwitchDefaults`, `funcdata_block.cc:706`).
    ///
    /// Mark each switch block's default out-edge (C++
    /// `Funcdata::installSwitchDefaults`, `funcdata_block.cc:706`).
    pub fn install_switch_defaults(&mut self) {
        for i in 0..self.num_jump_tables() {
            let jt = self.get_jump_table(i);
            let default_block = jt.get_default_block();
            let indop = match jt.get_indirect_op() {
                Some(op) => op,
                None => continue,
            };
            if default_block == -1 {
                continue;
            }
            if let Some(ind) = self.obank().get(indop).and_then(|o| o.get_parent()) {
                self.bblocks_mut().set_default_switch(ind, default_block);
            }
        }
    }

    // -----------------------------------------------------------------------
    // jump-table recovery chain (funcdata_block.cc:444-704)
    // -----------------------------------------------------------------------

    /// Search for a pre-existing jump-table whose op-address matches `op`, and
    /// link it to `op` (C++ `Funcdata::linkJumpTable`, `funcdata_block.cc:444`).
    /// Returns its index in `jumpvec`.
    pub fn link_jump_table(&mut self, op: OpId) -> Option<usize> {
        let addr = self.obank().get(op)?.get_addr().clone();
        let idx = self.jumpvec_ref().iter().position(|jt| *jt.get_op_address() == addr)?;
        // The op's address is `addr` (same op-address).
        self.jumpvec_mut()[idx].set_indirect_op_addr(op, addr);
        Some(idx)
    }

    /// Drop the jump-table at `idx` from the registry (C++
    /// `Funcdata::removeJumpTable`, `funcdata_block.cc:470`).  The C++ `clear()`
    /// before `delete` is the Rust drop.
    pub fn remove_jump_table(&mut self, idx: usize) {
        if idx < self.jumpvec_ref().len() {
            self.jumpvec_mut().remove(idx);
        }
    }

    /// Install a fresh (empty) jump-table for a BRANCHIND at `addr` (C++
    /// `Funcdata::installJumpTable`, `funcdata_block.cc:480`).  Must be called
    /// before flow is traced.
    pub fn install_jump_table(&mut self, addr: Address) -> KunaResult<usize> {
        if self.is_proc_started() {
            return Err(KunaError::lowlevel(
                "Cannot install jumptable if flow is already traced",
            ));
        }
        if self.jumpvec_ref().iter().any(|jt| *jt.get_op_address() == addr) {
            return Err(KunaError::lowlevel("Trying to install over existing jumptable"));
        }
        let newjt = crate::jumptable::JumpTable::new(addr);
        self.jumpvec_mut().push(newjt);
        Ok(self.num_jump_tables() as usize - 1)
    }

    /// For each jump-table, map every recovered address to its target basic-block
    /// out-edge (C++ `Funcdata::switchOverJumpTables`, `funcdata_block.cc:697`).
    ///
    /// `target` resolves an [`Address`] to the op starting its block (the
    /// `FlowInfo::target` surface).
    pub fn switch_over_jump_tables<F>(&mut self, target: F) -> KunaResult<()>
    where
        F: Fn(&Funcdata, &Address) -> KunaResult<OpId>,
    {
        let n = self.num_jump_tables();
        for i in 0..n {
            let mut jt = std::mem::replace(
                &mut self.jumpvec_mut()[i as usize],
                crate::jumptable::JumpTable::new(Address::default()),
            );
            let res = jt.switch_over(self, |addr| target(self, addr));
            self.jumpvec_mut()[i as usize] = jt;
            res?;
        }
        Ok(())
    }

    /// Generate a clone with truncated control-flow given a partial function
    /// (C++ `Funcdata::truncatedFlow`, `funcdata_op.cc:792`): the op + jump-table
    /// clone half.
    ///
    /// Clones the source function's raw dead p-code into \b this (empty) function
    /// and copies its jump-tables (relinked to the partial's matching ops).  The
    /// block-building half (`partialflow.generateBlocks()`) is driven separately
    /// by the recovery pipeline (which owns the [`crate::flow::FlowInfo`] +
    /// architecture env).
    ///
    /// STUB(W4): the FuncCallSpecs cloning (`oldspec->clone(newop)` + the fspec
    /// annotation swap) is the W4 call-spec surface; the op + jump-table clone is
    /// the load-bearing part for switch recovery and is ported.
    pub fn truncated_flow_clone(&mut self, src: &Funcdata) -> KunaResult<()> {
        if !self.obank().empty() {
            return Err(KunaError::lowlevel(
                "Trying to do truncated flow on pre-existing pcode",
            ));
        }
        // Clone the raw pcode (in dead-list order, preserving SeqNums).
        let src_ops: Vec<OpId> = src.obank().iter_dead().collect();
        for op in src_ops {
            let seq = src.obank().get(op).unwrap().get_seq_num().clone();
            self.clone_op_from(src, op, seq)?;
        }
        self.obank_mut().set_uniq_id(src.obank().get_uniq_id());

        // Clone callspecs (C++ `Funcdata::truncatedFlow`, funcdata_op.cc:803-815):
        // re-attach each discovered FuncCallSpecs to the partial's matching op
        // (found by SeqNum).  This carries the call's FuncProto effect list, which
        // is what produces the post-call INDIRECT on a stack slot — without it a
        // constant written before the call (e.g. a switch variable) folds straight
        // through in the partial and the jump table collapses to one entry.  The
        // fspec handle is the call op's own identity here (lookup is by op, not by
        // the offset), so the cloned op's in0 fspec annotation already resolves;
        // only the spec entries themselves need re-pointing at the cloned ops.
        let src_q = src.num_calls();
        for i in 0..src_q {
            let oldspec = src.get_call_specs(i);
            let oldop = oldspec.get_op();
            let seq = src.obank().get(oldop).unwrap().get_seq_num().clone();
            let newop = self.obank().find_op(&seq).ok_or_else(|| {
                KunaError::lowlevel("Could not trace call spec across partial clone")
            })?;
            let newspec = oldspec.clone_for_op(newop);
            self.push_call_specs(newspec);
        }

        // Clone the jumptables: relink each to the partial's matching op.
        let src_n = src.num_jump_tables();
        for i in 0..src_n {
            let srcjt = src.get_jump_table(i);
            let indop = match srcjt.get_indirect_op() {
                Some(op) => op,
                None => continue, // unlinked override not reached by flow yet
            };
            let seq = src.obank().get(indop).unwrap().get_seq_num().clone();
            let newop = self
                .obank()
                .find_op(&seq)
                .ok_or_else(|| KunaError::lowlevel("Could not trace jumptable across partial clone"))?;
            let mut jtclone = crate::jumptable::JumpTable::new_clone(srcjt);
            let opaddr = self.obank().get(newop).unwrap().get_addr().clone();
            jtclone.set_indirect_op_addr(newop, opaddr);
            self.jumpvec_mut().push(jtclone);
        }
        Ok(())
    }

    /// Immutable view of the jump-table vector (companion to `jumpvec_mut`).
    fn jumpvec_ref(&self) -> &[crate::jumptable::JumpTable] {
        self.jumpvec_slice()
    }

    /// Recover the switch-variable Varnode read by the head's comparison (the
    /// non-constant operand of the comparison that writes the CBRANCH boolean).
    /// Peels transparent COPY/CAST/ZEXT/SEXT/zero-SUBPIECE ops so the BRANCHIND
    /// reads the canonical switch value (the `canonSwitchVar` analog).  Returns the
    /// canonical Varnode, or `None` if the head is not a comparison-on-constant.
    fn kuna_head_switch_var(&self, cbranch: OpId, var_addr: &Address) -> Option<VarnodeId> {
        let boolvn = self.obank().get(cbranch)?.get_in(1)?;
        let cmp = self.vbank().get(boolvn)?.get_def()?;
        let v0 = self.obank().get(cmp)?.get_in(0)?;
        let v1 = self.obank().get(cmp)?.get_in(1)?;
        // The switch variable is the non-constant operand.
        let mut vn = if self.vbank().get(v0).map(|v| v.is_constant()).unwrap_or(false) {
            v1
        } else {
            v0
        };
        // Peel transparent ops (bounded, the canonSwitchVar guard).
        for _ in 0..8 {
            let v = match self.vbank().get(vn) {
                Some(v) => v,
                None => break,
            };
            if v.is_constant() || !v.is_written() {
                break;
            }
            let def = match v.get_def() {
                Some(d) => d,
                None => break,
            };
            let oc = self.obank().get(def).map(|o| o.code());
            match oc {
                Some(OpCode::CPUI_COPY)
                | Some(OpCode::CPUI_CAST)
                | Some(OpCode::CPUI_INT_ZEXT)
                | Some(OpCode::CPUI_INT_SEXT) => {
                    match self.obank().get(def).and_then(|o| o.get_in(0)) {
                        Some(nv) => vn = nv,
                        None => break,
                    }
                }
                Some(OpCode::CPUI_SUBPIECE) => {
                    let in1 = self.obank().get(def).and_then(|o| o.get_in(1));
                    let zero = in1
                        .and_then(|i| self.vbank().get(i))
                        .map(|v| v.is_constant() && v.get_offset() == 0)
                        .unwrap_or(false);
                    if zero {
                        match self.obank().get(def).and_then(|o| o.get_in(0)) {
                            Some(nv) => vn = nv,
                            None => break,
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        let _ = var_addr;
        Some(vn)
    }

    /// Find the basic block whose first address is `addr` (the install's
    /// target-block locator; the cascade record stores block START addresses).
    fn kuna_block_at_start(&self, addr: &Address) -> Option<BlockId> {
        let n = self.bblocks_get_size();
        for i in 0..n {
            let bl = self.bblocks_get_block(i);
            if self.bblocks_ref().block(bl).get_type() != crate::block::BlockType::Basic {
                continue;
            }
            if &crate::block::block_get_start(&self.bblocks_ref().arena, bl) == addr {
                return Some(bl);
            }
        }
        None
    }

    /// Find the basic block whose terminating op is at address `addr` (the
    /// cascade head, recorded as `branch_addr` = the head CBRANCH's op address).
    fn kuna_block_with_tail_at(&self, addr: &Address) -> Option<BlockId> {
        let n = self.bblocks_get_size();
        for i in 0..n {
            let bl = self.bblocks_get_block(i);
            if self.bblocks_ref().block(bl).get_type() != crate::block::BlockType::Basic {
                continue;
            }
            if let Some(tail) = self.bb_op_tail(bl) {
                if self.obank().get(tail).map(|o| o.get_addr()) == Some(addr) {
                    return Some(bl);
                }
            }
        }
        None
    }

    /// (kuna) Manufacture the S2 switch artifact from a recovered lowered
    /// comparison cascade (C++ `Funcdata::kunaInstallLoweredSwitch`,
    /// `kuna_loweredswitch.cc:463`).
    ///
    /// Runs pre-SSA on the restart pass (`ActionLowerSwitchInstall::apply`, before
    /// `ActionHeritage`).  It:
    ///   1. finds the cascade head (the block whose terminating op is at
    ///      `branch_addr`);
    ///   2. resolves the distinct target blocks (the case bodies + the default)
    ///      by their recorded start addresses;
    ///   3. rewires the head: drops its out-edges, replaces the trailing CBRANCH
    ///      with a `BRANCHIND(V)` over a fresh Varnode at the switch-variable
    ///      storage (heritage renames it to the value reaching the head), and adds
    ///      one out-edge per distinct target (cases first, default last);
    ///   4. sweeps the now-orphaned comparison-cascade spine
    ///      (`removeUnreachableBlocks`);
    ///   5. builds a fully-labelled, trivial-model `JumpTable` in `jumpvec` linked
    ///      to the new BRANCHIND so the existing structurer + printer emit a
    ///      `switch`.
    ///
    /// Returns `Ok(Some(jt_index))` on success, `Ok(None)` if the CFG no longer
    /// matches the record (e.g. the head/targets were merged away — install
    /// declines gracefully rather than corrupting the CFG).
    #[allow(clippy::too_many_arguments)]
    pub fn kuna_install_lowered_switch(
        &mut self,
        branch_addr: &Address,
        var_addr: &Address,
        var_size: int4,
        case_vals: &[kuna_base::types::uintb],
        case_targets: &[Address],
        default_target: &Address,
    ) -> KunaResult<Option<usize>> {
        use kuna_base::types::uintb;

        // Don't re-install if a table already exists for this branch address.
        if self.jumpvec_ref().iter().any(|jt| jt.get_op_address() == branch_addr) {
            return Ok(None);
        }

        // (1) Locate the cascade head block.
        let head = match self.kuna_block_with_tail_at(branch_addr) {
            Some(h) => h,
            None => return Ok(None),
        };

        // (2) Build the ordered distinct out-edge target list: each distinct case
        // target (in first-seen order), then the default last.  Record, per
        // out-edge, a representative case label (the printer emits one `case` per
        // address-table entry; duplicate-valued cases collapse to the same edge,
        // and the C++ trivial recovery emits one label per edge).
        let mut out_targets: Vec<Address> = Vec::new();
        let mut out_label: Vec<uintb> = Vec::new();
        let mut out_is_default: Vec<bool> = Vec::new();

        for (i, tgt) in case_targets.iter().enumerate() {
            if tgt == default_target {
                continue; // a case that coincides with the default folds away
            }
            if out_targets.iter().any(|a| a == tgt) {
                continue; // duplicate target: keep the first label
            }
            out_targets.push(tgt.clone());
            out_label.push(case_vals[i]);
            out_is_default.push(false);
        }
        // Default out-edge last.
        out_targets.push(default_target.clone());
        out_label.push(0);
        out_is_default.push(true);

        // Resolve every target to a live basic block; if any is missing the CFG no
        // longer matches — decline.
        let mut out_blocks: Vec<BlockId> = Vec::with_capacity(out_targets.len());
        for tgt in &out_targets {
            match self.kuna_block_at_start(tgt) {
                Some(b) => out_blocks.push(b),
                None => return Ok(None),
            }
        }
        // Need at least 2 case edges + a default for a switch.
        if out_blocks.len() < 3 {
            return Ok(None);
        }

        // The rewiring severs the head's out-edges, so everything reachable only
        // through the cascade spine is swept.  If an already-recovered jump table
        // sits down there, the cascade was a guard chain in front of a genuine
        // switch and installing would delete it — decline.
        if self.kuna_lowered_switch_strands_table(head, &out_blocks) {
            return Ok(None);
        }

        // The head's trailing op must be the recorded CBRANCH.
        let cbranch = match self.bb_op_tail(head) {
            Some(o) => o,
            None => return Ok(None),
        };
        if self.obank().get(cbranch).map(|o| o.code()) != Some(OpCode::CPUI_CBRANCH) {
            return Ok(None);
        }

        // Recover the live switch-variable Varnode from the head comparison BEFORE
        // any surgery: the CBRANCH's boolean input is written by a comparison whose
        // non-constant operand is the switch variable.  Reusing that exact Varnode
        // (rather than a fresh free read of `var_addr`) keeps its full def-use chain
        // intact so heritage threads SSA through it and the printer names it.
        let swvn = self.kuna_head_switch_var(cbranch, var_addr);
        // The comparison op writing the CBRANCH boolean (destroyed below so the
        // switch variable is read only by the new BRANCHIND at heritage time —
        // heritage's `guard` requires a free read to have exactly one descendant).
        let head_cmp = self
            .obank()
            .get(cbranch)
            .and_then(|o| o.get_in(1))
            .and_then(|b| self.vbank().get(b))
            .and_then(|v| v.get_def());

        // (3) Rewire the head block.
        //
        // Sever ALL of the head's existing out-edges.  branch_remove_internal
        // destroys the CBRANCH on the first call (sizeOut()==2); subsequent calls
        // just drop edges.  We run pre-SSA (no MULTIEQUALs yet), so this is a clean
        // edge drop with no phi patch-up.
        while self.bblocks_ref().block(head).size_out() > 0 {
            self.branch_remove_internal(head, 0)?;
        }
        // Destroy the now-dead head comparison chain (its boolean fed only the
        // destroyed CBRANCH) so the switch variable's sole remaining reader is the
        // BRANCHIND we add next.  op_destroy_recursive drops the comparison and any
        // transparent peel ops that become dead, but stops at live Varnodes (so the
        // recovered switch variable's def survives).
        if let Some(cmp) = head_cmp {
            if self.obank().get(cmp).is_some() {
                let mut scratch: Vec<OpId> = Vec::new();
                self.op_destroy_recursive(cmp, &mut scratch);
            }
        }

        // Build the BRANCHIND at the head's branch address over the recovered
        // switch variable (falling back to a fresh free read of `var_addr` if the
        // head comparison did not yield a still-live Varnode — heritage then
        // renames it).
        let swvn = match swvn {
            Some(v) if self.vbank().get(v).is_some() => v,
            _ => self.new_varnode(var_size, var_addr, None),
        };
        let newop = self.new_op(1, branch_addr.clone());
        self.op_set_opcode_code(newop, OpCode::CPUI_BRANCHIND);
        self.op_set_input(newop, swvn, 0)?;
        // Append the BRANCHIND as the head's terminator (the branch flag marks the
        // block is_switch_out, the same path bb_insert_op uses).
        self.bb_insert_op(newop, head, None);

        // Add one out-edge per distinct target, in (case…, default) order.  The
        // out-edge index equals the order of insertion.
        for &tb in &out_blocks {
            self.bblocks_mut().add_edge(head, tb);
        }

        // (4) Sweep the now-unreachable comparison-cascade spine.  structure_reset
        // recomputes dominators so removeUnreachableBlocks can find the orphans.
        self.structure_reset();
        self.remove_unreachable_blocks(false, true)?;

        // Re-find the head after the sweep (block ids are stable, but be defensive)
        // and build the table against its post-sweep out-edge indices.
        let head = match self.kuna_block_with_tail_at(branch_addr) {
            Some(h) => h,
            None => return Ok(None),
        };
        let nout = self.bblocks_ref().block(head).size_out();
        if nout as usize != out_blocks.len() {
            return Ok(None);
        }

        // (5) Build the JumpTable: one address-table entry per out-edge, mapped to
        // its out-edge index; the case label is the representative constant; the
        // default out-edge is recorded as the table's default block.
        let indop = self
            .bb_op_tail(head)
            .ok_or_else(|| KunaError::lowlevel("install: head lost its BRANCHIND"))?;
        let opaddr = self.obank().get(indop).unwrap().get_addr().clone();
        let mut jt = crate::jumptable::JumpTable::new(opaddr.clone());
        jt.set_indirect_op_addr(indop, opaddr);
        jt.kuna_set_trivial_model();

        let mut default_out_index: int4 = -1;
        for oi in 0..nout {
            let outbl = self.bblocks_ref().block(head).get_out(oi);
            let start = crate::block::block_get_start(&self.bblocks_ref().arena, outbl);
            let mut lab: uintb = 0;
            let mut isdef = false;
            for k in 0..out_blocks.len() {
                if out_blocks[k] == outbl {
                    lab = out_label[k];
                    isdef = out_is_default[k];
                    break;
                }
            }
            if isdef {
                default_out_index = oi;
            }
            jt.kuna_push_entry(start, oi, lab);
        }
        jt.kuna_finalize(default_out_index);
        jt.mark_complete();

        // Render case labels signed when any recovered case constant is negative
        // in the switch-variable width (the cascade used signed comparisons over a
        // signed `int` switch value — the getopt-return idiom; the C++ derives this
        // from `getSwitchType()`).
        let signbit = if var_size >= 8 {
            1u64 << 63
        } else {
            1u64 << (var_size * 8 - 1)
        };
        let signed = case_vals.iter().any(|&v| (v & signbit) != 0);
        jt.kuna_set_signed_labels(signed);
        // Record the recovered switch-variable storage so the post-heritage
        // repair (`kuna_repair_lowered_switch_inputs`) can re-point the
        // synthesized BRANCHIND at the live SSA value if heritage normalized the
        // too-small switch-var read up to the register range's width and then
        // swept the resulting undefined wide read (nulling the BRANCHIND input).
        jt.kuna_set_lowered_var(var_addr.clone(), var_size);

        self.jumpvec_mut().push(jt);
        let idx = self.num_jump_tables() as usize - 1;
        Ok(Some(idx))
    }

    /// (kuna) Would installing the lowered switch at `head` strand a genuine jump
    /// table?
    ///
    /// Walks reachability from the entry over the *post-surgery* successor
    /// relation (`head`'s out-edges replaced by `out_blocks`) and reports whether
    /// any block that drops out of the reachable set ends in a BRANCHIND with a
    /// recovered [`JumpTable`](crate::jumptable::JumpTable).  The
    /// `remove_unreachable_blocks` sweep at the end of
    /// [`kuna_install_lowered_switch`](Self::kuna_install_lowered_switch) would
    /// destroy that BRANCHIND and its table, replacing a real switch with the
    /// synthetic cascade dispatch and losing every case the table dispatched.
    fn kuna_lowered_switch_strands_table(&self, head: BlockId, out_blocks: &[BlockId]) -> bool {
        let size = self.bblocks_get_size();
        let mut entry: Option<BlockId> = None;
        for i in 0..size {
            let blk = self.bblocks_get_block(i);
            if self.bblocks_ref().block(blk).is_entry_point() {
                entry = Some(blk);
                break;
            }
        }
        let entry = match entry {
            Some(e) => e,
            None => return false,
        };

        let mut seen: std::collections::HashSet<BlockId> = std::collections::HashSet::new();
        let mut stack: Vec<BlockId> = vec![entry];
        seen.insert(entry);
        while let Some(b) = stack.pop() {
            if b == head {
                for &t in out_blocks {
                    if seen.insert(t) {
                        stack.push(t);
                    }
                }
                continue;
            }
            let nout = self.bblocks_ref().block(b).size_out();
            for i in 0..nout {
                let t = self.bblocks_ref().block(b).get_out(i);
                if seen.insert(t) {
                    stack.push(t);
                }
            }
        }

        for i in 0..size {
            let blk = self.bblocks_get_block(i);
            if seen.contains(&blk) {
                continue;
            }
            let op = match self.bb_op_tail(blk) {
                Some(o) => o,
                None => continue,
            };
            if self.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_BRANCHIND)
                && self.find_jump_table_index(op).is_some()
            {
                return true;
            }
        }
        false
    }

    /// (kuna) Post-heritage repair of synthetic lowered-switch BRANCHIND inputs.
    ///
    /// `kuna_install_lowered_switch` runs pre-SSA and points the synthesized
    /// `BRANCHIND` at a *free read* of the recovered switch-variable storage
    /// (e.g. a 4-byte `EAX` read), trusting heritage to thread SSA through it.
    /// But when that register's overlapping range is read elsewhere at a wider
    /// width (e.g. the full 8-byte `RAX`), heritage's `guard`/`normalizeReadSize`
    /// widens the too-small read to the range width — wrapping it in a
    /// `SUBPIECE` of a fresh wide free read.  At the BRANCHIND's program point no
    /// wide reaching def exists (only the narrow switch value reaches), so the
    /// wide read is undefined and dead-code removal sweeps the SUBPIECE,
    /// **nulling the BRANCHIND input** — the header then renders the invalid
    /// `switch()`.
    ///
    /// This runs right after [`op_heritage`](crate::funcdata::Funcdata::op_heritage)
    /// (its only caller is the action wired immediately after `ActionHeritage`).
    /// For each synthetic lowered-switch table (the only ones carrying
    /// [`JumpTable::kuna_lowered_var`]) whose BRANCHIND input is null/free, it
    /// finds the live SSA Varnode of exactly the recorded `(addr,size)` reaching
    /// the BRANCHIND — the narrow switch value (e.g. the call output) — by
    /// scanning the BRANCHIND's own block backward and then up the dominator
    /// tree, and re-points input 0 at it.  Scoped strictly to the recorded
    /// synthetic tables, so it cannot perturb the datatest corpus (which has no
    /// `kuna_lowered_var` tables).  Declines silently if no exact-size reaching
    /// def is found, leaving the current behavior intact.
    pub fn kuna_repair_lowered_switch_inputs(&mut self) {
        // Snapshot the (indirect op, var addr, var size) of every synthetic table.
        let targets: Vec<(OpId, Address, int4)> = self
            .jumpvec_ref()
            .iter()
            .filter_map(|jt| {
                let (a, s) = jt.kuna_lowered_var()?;
                let op = jt.get_indirect_op()?;
                Some((op, a.clone(), *s))
            })
            .collect();
        for (indop, var_addr, var_size) in targets {
            // Confirm this op is still a live BRANCHIND.
            if self.obank().get(indop).map(|o| o.code()) != Some(OpCode::CPUI_BRANCHIND) {
                continue;
            }
            // If input 0 already names a live, non-free Varnode, leave it.  A
            // constant is "heritage known" (C++ `Varnode::isHeritageKnown()` =
            // insert|constant|annotation) and must count as healthy: on the
            // hang-repro stripped-ELF corpus `ActionConditionalConst` legitimately
            // proves the switch variable constant down the guarding edge and
            // replaces this very input with the constant.  Classifying that
            // constant as broken re-pointed the BRANCHIND back at the register
            // SSA def on every heritage pass, and condconst re-replaced it on
            // every mainloop iteration — the two actions toggled the same input
            // forever and the repeatapply mainloop never converged (the
            // `decompile-all` infinite hang on stripped openssh/bash binaries).
            let in0 = self.obank().get(indop).and_then(|o| o.get_in(0));
            let healthy = in0
                .and_then(|v| self.vbank().get(v))
                .map(|v| v.is_written() || v.is_input() || v.is_heritage_known())
                .unwrap_or(false);
            if healthy {
                continue;
            }
            // Find the live reaching def of exactly (var_addr, var_size).
            if let Some(def_vn) = self.kuna_reaching_def_at(indop, &var_addr, var_size) {
                let _ = self.op_set_input(indop, def_vn, 0);
            }
        }
    }

    /// (kuna) Find the live SSA Varnode of exactly `(addr,size)` reaching `op`
    /// (its def dominates `op`).  Scans `op`'s own block backward from `op`, then
    /// walks the immediate-dominator chain scanning each block's ops in reverse;
    /// returns the first output Varnode matching the storage exactly.  Returns
    /// `None` if no exact match dominates `op` (the caller then declines).
    ///
    /// Used only by [`kuna_repair_lowered_switch_inputs`]; an exact-size match is
    /// required so we never re-point a BRANCHIND at a mis-sized value.
    fn kuna_reaching_def_at(
        &self,
        op: OpId,
        addr: &Address,
        size: int4,
    ) -> Option<VarnodeId> {
        let want_space = addr.get_space()?.get_index();
        let want_off = addr.get_offset();
        let matches = |this: &Self, vn: VarnodeId| -> bool {
            let v = match this.vbank().get(vn) {
                Some(v) => v,
                None => return false,
            };
            v.get_size() == size
                && v.get_offset() == want_off
                && v.get_addr().get_space().map(|s| s.get_index()) == Some(want_space)
        };
        // Start in the BRANCHIND's own block, scanning ops strictly before it.
        let mut block = self.obank().get(op)?.get_parent()?;
        let mut stop_before: Option<OpId> = Some(op);
        // Bound the dominator walk defensively (the tree depth is finite).
        for _ in 0..4096 {
            let ops = self.bb_ops(block);
            // On the start block, restrict to ops strictly before `op`; on every
            // dominator block, scan the whole block.
            let upper = match stop_before {
                Some(stop) => ops.iter().position(|&o| o == stop).unwrap_or(ops.len()),
                None => ops.len(),
            };
            // Reverse scan: the closest-preceding def reaches `op`.
            for &bop in ops[..upper].iter().rev() {
                if let Some(out) = self.obank().get(bop).and_then(|o| o.get_out()) {
                    if matches(self, out) {
                        return Some(out);
                    }
                }
            }
            // Move to the immediate dominator and scan its whole block.
            match self.bblocks_ref().block(block).get_immed_dom() {
                Some(idom) if idom != block => {
                    block = idom;
                    stop_before = None;
                }
                _ => break,
            }
        }
        None
    }

    /// Test whether the given Varnode holds the function's return address (C++
    /// `Funcdata::testForReturnAddress`, `funcdata_varnode.cc:1463`).
    ///
    /// Follow the Varnode back through COPY, INDIRECT, and alignment-mask INT_AND
    /// ops; if it terminates at the function input Varnode occupying
    /// `glb->defaultReturnAddr`'s storage, the BRANCHIND is really a tail return
    /// through the return-address register (the jump-table `fail_return` mode).
    /// `glb.default_return_addr == None` (C++ `space == 0`, no standard return
    /// storage in the cspec) yields `false`, exactly as the C++ does.
    pub fn test_for_return_address(&self, vn: VarnodeId) -> bool {
        let retaddr = match self.get_arch().default_return_addr.as_ref() {
            Some(rd) => rd,
            None => return false, // No standard storage location to compare to
        };
        let ra_space = match retaddr.space.as_ref() {
            Some(sp) => sp,
            None => return false,
        };
        let mut cur = vn;
        // Follow the def chain while the Varnode is written.
        while self.vbank().get(cur).map(|v| v.is_written()).unwrap_or(false) {
            let op = self.vbank().get(cur).unwrap().get_def().unwrap();
            let opc = self.obank().get(op).map(|o| o.code());
            match opc {
                Some(OpCode::CPUI_INDIRECT) | Some(OpCode::CPUI_COPY) => {
                    cur = match self.obank().get(op).and_then(|o| o.get_in(0)) {
                        Some(v) => v,
                        None => return false,
                    };
                }
                Some(OpCode::CPUI_INT_AND) => {
                    // Only allow "alignment" style masking (constant mask).
                    let in1 = match self.obank().get(op).and_then(|o| o.get_in(1)) {
                        Some(v) => v,
                        None => return false,
                    };
                    if !self.vbank().get(in1).map(|v| v.is_constant()).unwrap_or(false) {
                        return false;
                    }
                    cur = match self.obank().get(op).and_then(|o| o.get_in(0)) {
                        Some(v) => v,
                        None => return false,
                    };
                }
                _ => return false,
            }
        }
        // Compare the terminal Varnode to defaultReturnAddr's storage and require
        // it be a function input (C++ `vn->isInput()`).
        let v = match self.vbank().get(cur) {
            Some(v) => v,
            None => return false,
        };
        if v.get_space().get_index() != ra_space.get_index()
            || v.get_offset() != retaddr.offset
            || v.get_size() != retaddr.size as int4
        {
            return false;
        }
        v.is_input()
    }

    /// Recover a jump-table for a BRANCHIND using the existing flow, running the
    /// "jumptable" action set on a partial clone via `run_pipeline` (C++
    /// `Funcdata::recoverJumpTable` + `stageJumpTable`, `funcdata_block.cc:680`/
    /// `512`).
    ///
    /// Returns:
    ///   - `Ok(Some(idx))`: the table was recovered; `idx` is its `jumpvec` slot.
    ///   - `Ok(None)`: could not recover (caller truncates to a call).
    ///   - `Err(mode)`: a specific failure mode (thunk/return/callother) — caller
    ///     truncates accordingly.
    #[allow(clippy::mutable_key_type)]
    pub fn recover_jump_table_flow(
        &mut self,
        op: OpId,
        record_loads: bool,
        visited: &crate::flow::VisitedMap,
        run_pipeline: &mut crate::flow::JtPipelineFn<'_>,
        shared: &mut Option<Funcdata>,
    ) -> Result<Option<usize>, crate::jumptable::RecoveryMode> {
        use crate::jumptable::RecoveryMode;
        // linkJumpTable: search for a pre-existing table.
        if let Some(idx) = self.link_jump_table(op) {
            let jt = self.get_jump_table(idx as int4);
            if !jt.is_override() && !jt.is_partial() && jt.num_entries() != 0 {
                return Ok(Some(idx)); // Previously calculated, complete, non-override
            }
            // Recover empty/override table.
            let mode =
                self.stage_jump_table(idx, op, record_loads, visited, run_pipeline, shared);
            if mode != RecoveryMode::Success {
                return Err(mode);
            }
            let addr = self.obank().get(op).unwrap().get_addr().clone();
            self.jumpvec_mut()[idx].set_indirect_op_addr(op, addr);
            return Ok(Some(idx));
        }

        if (self.flags() & crate::funcdata::funcdata_flags::jumptablerecovery_dont) != 0 {
            return Ok(None); // Explicitly told not to recover
        }
        // earlyJumpTableFail.
        let early = self.early_jump_table_fail(op);
        if early != RecoveryMode::Success {
            return Err(early);
        }
        // Trial recovery into a fresh table appended to jumpvec.
        let addr = self.obank().get(op).unwrap().get_addr().clone();
        let trial = crate::jumptable::JumpTable::new(addr);
        self.jumpvec_mut().push(trial);
        let idx = self.num_jump_tables() as usize - 1;
        let mode = self.stage_jump_table(idx, op, record_loads, visited, run_pipeline, shared);
        if mode != RecoveryMode::Success {
            // Discard the trial table on failure.
            self.jumpvec_mut().remove(idx);
            return Err(mode);
        }
        let addr = self.obank().get(op).unwrap().get_addr().clone();
        self.jumpvec_mut()[idx].set_indirect_op_addr(op, addr);
        Ok(Some(idx))
    }

    /// Run the reduced "jumptable" pipeline on a partial clone and recover the
    /// table's addresses (C++ `Funcdata::stageJumpTable`, `funcdata_block.cc:512`).
    #[allow(clippy::mutable_key_type)]
    fn stage_jump_table(
        &mut self,
        jt_idx: usize,
        op: OpId,
        record_loads: bool,
        visited: &crate::flow::VisitedMap,
        run_pipeline: &mut crate::flow::JtPipelineFn<'_>,
        shared: &mut Option<Funcdata>,
    ) -> crate::jumptable::RecoveryMode {
        use crate::jumptable::RecoveryMode;
        self.jumpvec_mut()[jt_idx].increment_recovery_count();

        // Build the partial function (clone ops + jump-tables) and run the reduced
        // pipeline over it, or reuse the one an earlier table in this batch already
        // built (C++ `if (!partial.isJumptableRecoveryOn())`, `option jtsharepartial`).
        let share = self.get_arch().jumptable_share_partial;
        let mut owned: Option<Funcdata> = None;
        if !share || shared.is_none() {
            let mut fresh = match self.build_jumptable_partial() {
                Ok(p) => p,
                Err(_) => return RecoveryMode::FailNormal,
            };
            // Mark the partial as a jump-table-recovery clone, then build its blocks
            // and run the reduced "jumptable" universalAction over it (C++
            // partial.truncatedFlow + allacts.setCurrent("jumptable") + perform).
            fresh.set_flag_raw(crate::funcdata::funcdata_flags::jumptablerecovery_on);
            if run_pipeline(&mut fresh, visited).is_err() {
                // C++ catches LowlevelError, warns, and returns fail_normal.
                return RecoveryMode::FailNormal;
            }
            if share {
                *shared = Some(fresh);
            } else {
                owned = Some(fresh);
            }
        }
        let partial: &mut Funcdata = if share {
            shared.as_mut().expect("stage_jump_table: shared partial just built")
        } else {
            owned.as_mut().expect("stage_jump_table: owned partial just built")
        };

        // findOp(op->getSeqNum()) on the partial.
        let seq = self.obank().get(op).unwrap().get_seq_num().clone();
        let addr = self.obank().get(op).unwrap().get_addr().clone();
        let partop = match partial.obank().find_op(&seq) {
            Some(p) => p,
            None => return RecoveryMode::FailNormal, // Bad partial clone
        };
        let partop_ok = partial.obank().get(partop).map(|o| {
            o.code() == OpCode::CPUI_BRANCHIND && *o.get_addr() == addr
        }).unwrap_or(false);
        if !partop_ok {
            return RecoveryMode::FailNormal;
        }
        if partial.obank().get(partop).unwrap().is_dead() {
            // Indirect op eliminated as dead code (unreachable).
            return RecoveryMode::Success;
        }
        // testForReturnAddress on the partial's BRANCHIND input.
        let in0 = partial.obank().get(partop).unwrap().get_in(0).unwrap();
        if partial.test_for_return_address(in0) {
            return RecoveryMode::FailReturn;
        }

        // Recover addresses into the table (clone the table out, recover, store).
        let mut jt = std::mem::replace(
            &mut self.jumpvec_mut()[jt_idx],
            crate::jumptable::JumpTable::new(Address::default()),
        );
        jt.set_load_collect(record_loads);
        let addr2 = partial.obank().get(partop).unwrap().get_addr().clone();
        jt.set_indirect_op_addr(partop, addr2);
        let res = if jt.is_partial() {
            jt.recover_multistage(partial)
        } else {
            jt.recover_addresses(partial)
        };
        // Relink the table to the original op before storing it back.
        jt.set_indirect_op_addr(op, addr);
        self.jumpvec_mut()[jt_idx] = jt;
        match res {
            Ok(()) => RecoveryMode::Success,
            Err(e) => {
                // recoverAddresses throws JumptableThunkError as "Likely thunk".
                if e.explain().contains("Likely thunk") {
                    RecoveryMode::FailThunk
                } else {
                    RecoveryMode::FailNormal
                }
            }
        }
    }

    /// Build the partial Funcdata for jump-table recovery: a fresh function
    /// sharing \b this function's arch + entry, with the raw p-code + jump-tables
    /// cloned (C++ `Funcdata partial(...)` + `partial.truncatedFlow`'s op-clone
    /// half).  The block-building + action pipeline run inside `run_pipeline`.
    fn build_jumptable_partial(&self) -> KunaResult<Funcdata> {
        let glb = self.get_arch().clone();
        let uniq_start = self.vbank().get_uniqbase();
        let mut partial = Funcdata::new(
            "@@jumprecovery",
            "@@jumprecovery",
            glb,
            self.get_address().clone(),
            uniq_start,
            0,
        )?;
        // C++ `Funcdata` ctor calls `funcp.setScope(localmap, baseaddr-1)`, which
        // attaches the architecture's default prototype model when none is set
        // (fspec.cc:2785).  The merged tree defers model assignment to
        // `ActionPrototypeTypes`, but the reduced "jumptable" pipeline reads
        // `funcp.getModel()` (e.g. `getMaxOutputDelay`) before that action runs, so
        // the default model must be attached at construction time exactly as the C++
        // ctor does.
        if !partial.get_func_proto().has_model() {
            if let Some(defaultfp) = partial.get_arch().default_fp().cloned() {
                partial.get_func_proto_mut().set_model(Some(defaultfp));
            }
        }
        partial.truncated_flow_clone(self)?;
        Ok(partial)
    }

    /// Backtrack from a BRANCHIND looking for an un-injected CALLOTHER in the
    /// destination calculation (C++ `Funcdata::earlyJumpTableFail`,
    /// `funcdata_block.cc:568`).
    ///
    /// STUB(W4): the CALLOTHER user-op-type classification
    /// (`glb->userops.getOp(id)->getType()`) is the W4 user-op table; without it
    /// a CALLOTHER that writes the address is conservatively treated as a genuine
    /// switch input (returns `Success`, so recovery proceeds), matching the C++
    /// "assume special will not interfere" continuation.  The non-CALLOTHER
    /// backtracking (unary/binary realigning ops, CALL/STORE/branch cutoffs) is
    /// ported faithfully.
    pub fn early_jump_table_fail(&self, op: OpId) -> crate::jumptable::RecoveryMode {
        use crate::jumptable::RecoveryMode;
        let mut vn = self.obank().get(op).unwrap().get_in(0).unwrap();
        // Walk backward over the dead-op list from `op`.
        let order: Vec<OpId> = self.obank().iter_dead().collect();
        let start_pos = match order.iter().position(|&o| o == op) {
            Some(p) => p,
            None => return RecoveryMode::Success,
        };
        let mut count_max = 8;
        let mut idx = start_pos;
        while idx > 0 {
            if self.vbank().get(vn).unwrap().get_size() == 1 {
                return RecoveryMode::Success;
            }
            count_max -= 1;
            if count_max < 0 {
                return RecoveryMode::Success;
            }
            idx -= 1;
            let cur = order[idx];
            let outvn = self.obank().get(cur).unwrap().get_out();
            let outhit = match outvn {
                Some(ov) => self.varnodes_intersect(vn, ov),
                None => false,
            };
            let opc = self.obank().get(cur).unwrap().code();
            let eval = self.obank().get(cur).unwrap().get_eval_type();
            use crate::op::pcodeop_flags as pf;
            if eval & pf::special != 0 {
                if self.obank().get(cur).unwrap().is_call() {
                    if opc == OpCode::CPUI_CALLOTHER {
                        // STUB(W4): userop-type classification (injected/jumpassist/
                        // segment short-circuit Success; an uninjected CALLOTHER
                        // writing the address would be fail_callother).  Without the
                        // W4 user-op table, assume it does not interfere; continue.
                    } else {
                        return RecoveryMode::Success; // CALL/CALLIND
                    }
                } else if self.obank().get(cur).unwrap().is_branch() {
                    return RecoveryMode::Success;
                } else {
                    if opc == OpCode::CPUI_STORE {
                        return RecoveryMode::Success;
                    }
                    if outhit {
                        return RecoveryMode::Success;
                    }
                }
            } else if eval & pf::unary != 0 {
                if outhit {
                    let invn = self.obank().get(cur).unwrap().get_in(0).unwrap();
                    if self.vbank().get(invn).unwrap().get_size()
                        != self.vbank().get(vn).unwrap().get_size()
                    {
                        return RecoveryMode::Success;
                    }
                    vn = invn;
                }
            } else if eval & pf::binary != 0 {
                if outhit {
                    if opc != OpCode::CPUI_INT_ADD
                        && opc != OpCode::CPUI_INT_SUB
                        && opc != OpCode::CPUI_INT_XOR
                    {
                        return RecoveryMode::Success;
                    }
                    let in1 = self.obank().get(cur).unwrap().get_in(1).unwrap();
                    if !self.vbank().get(in1).unwrap().is_constant() {
                        return RecoveryMode::Success;
                    }
                    let invn = self.obank().get(cur).unwrap().get_in(0).unwrap();
                    if self.vbank().get(invn).unwrap().get_size()
                        != self.vbank().get(vn).unwrap().get_size()
                    {
                        return RecoveryMode::Success;
                    }
                    vn = invn;
                }
            } else if outhit {
                return RecoveryMode::Success;
            }
        }
        RecoveryMode::Success
    }

    /// Do the two Varnodes' storage ranges intersect (C++ `Varnode::intersects`)?
    fn varnodes_intersect(&self, a: VarnodeId, b: VarnodeId) -> bool {
        let va = self.vbank().get(a).unwrap();
        let vb = self.vbank().get(b).unwrap();
        va.intersects(vb)
    }

    /// Create a new basic block holding a merged CBRANCH
    /// (C++ `Funcdata::nodeJoinCreateBlock`, `funcdata_block.cc:798`).
    ///
    /// Used by ConditionalJoin: the new block gets one out edge from each of the
    /// two input blocks, and the remaining out edges are re-pointed into it.
    /// Pure control-flow + cover manipulation; no data-flow patch-up here.
    #[allow(clippy::too_many_arguments)]
    pub fn node_join_create_block(
        &mut self,
        block1: BlockId,
        block2: BlockId,
        exita: BlockId,
        exitb: BlockId,
        fora_block1ishigh: bool,
        forb_block1ishigh: bool,
        addr: &Address,
    ) -> BlockId {
        let graph = self.bblocks_root_pub();
        let newblock = self.bblocks_mut().new_block_basic(graph);
        self.bblocks_mut().block_mut(newblock).set_flag(block_flags::f_joined_block);
        self.set_basic_block_range(newblock, addr, addr);

        // Delete 2 of the original edges into exita and exitb, picking the swap.
        let swapa = if fora_block1ishigh {
            self.bblocks_mut().remove_edge(block1, exita);
            block2
        } else {
            self.bblocks_mut().remove_edge(block2, exita);
            block1
        };
        let swapb = if forb_block1ishigh {
            self.bblocks_mut().remove_edge(block1, exitb);
            block2
        } else {
            self.bblocks_mut().remove_edge(block2, exitb);
            block1
        };

        // Move the remaining two from block1,block2 to newblock.
        let sa = self.bblocks_ref().block(swapa).get_out_index(exita);
        self.bblocks_mut().move_out_edge(swapa, sa, newblock);
        let sb = self.bblocks_ref().block(swapb).get_out_index(exitb);
        self.bblocks_mut().move_out_edge(swapb, sb, newblock);

        self.bblocks_mut().add_edge(block1, newblock);
        self.bblocks_mut().add_edge(block2, newblock);
        self.structure_reset();
        newblock
    }

    /// Split basic block `b` along an \e in edge, duplicating the block with the
    /// same out edges but only the indicated in edge
    /// (C++ `Funcdata::nodeSplitBlockEdge`, `funcdata_block.cc:843`).
    ///
    /// This is the control-flow half of `nodeSplit`; the p-code cloning
    /// (`CloneBlockOps`) is the funcdata_op wave's.
    pub fn node_split_block_edge(&mut self, b: BlockId, inedge: int4) -> BlockId {
        let a = self.bblocks_ref().block(b).get_in(inedge);
        let graph = self.bblocks_root_pub();
        let bprime = self.bblocks_mut().new_block_basic(graph);
        self.bblocks_mut().block_mut(bprime).set_flag(block_flags::f_duplicate_block);
        self.copy_basic_range(b, bprime);
        self.bblocks_mut().switch_edge(a, b, bprime);
        let sizeout = self.bblocks_ref().block(b).size_out();
        for i in 0..sizeout {
            let out = self.bblocks_ref().block(b).get_out(i);
            self.bblocks_mut().add_edge(bprime, out);
        }
        bprime
    }

    /// Split control-flow into a basic block, duplicating its p-code into a new
    /// block (C++ `Funcdata::nodeSplit`, `funcdata_block.cc:864`).
    ///
    /// P-code is duplicated into another block, and control-flow is modified so
    /// the new block takes over flow from one input edge to the original block.
    /// The original block must have no out edges and more than one (non-redundant)
    /// in edge.
    pub fn node_split(&mut self, b: BlockId, inedge: int4) -> KunaResult<()> {
        // Split node b along inedge.
        if self.bblocks_ref().block(b).size_out() != 0 {
            return Err(KunaError::lowlevel(
                "Cannot (currently) nodesplit block with out flow",
            ));
        }
        let size_in = self.bblocks_ref().block(b).size_in();
        if size_in <= 1 {
            return Err(KunaError::lowlevel("Cannot nodesplit block with only 1 in edge"));
        }
        // Detect redundant (duplicate) in edges via the mark bit, mirroring the
        // C++ setMark/clearMark pass.
        let mut seen: std::collections::BTreeSet<BlockId> = std::collections::BTreeSet::new();
        for i in 0..size_in {
            let inbl = self.bblocks_ref().block(b).get_in(i);
            if !seen.insert(inbl) {
                return Err(KunaError::lowlevel(
                    "Cannot nodesplit block with redundant in edges",
                ));
            }
        }

        // Create duplicate block (control-flow half).
        let bprime = self.node_split_block_edge(b, inedge);
        // Copy b's ops into bprime (p-code half).
        let mut cloner = CloneBlockOps::new();
        cloner.clone_block(self, b, bprime, inedge)?;

        // We would need to patch outputs here for the more general case when b
        // has out edges (any references not in b to varnodes defined in b need
        // MULTIEQUALs defined in b's out blocks).  b has no out edges here.
        self.structure_reset();
        Ok(())
    }

    /// Clone the p-code ops of an expression right before `follow_op`, returning
    /// the output Varnode of the last cloned op (drives `CloneBlockOps::
    /// cloneExpression`).  Used by `RuleConditionalMove::constructBool` to pull a
    /// boolean expression out of a conditional block via duplication.
    pub fn clone_expression(
        &mut self,
        ops: &[OpId],
        follow_op: OpId,
    ) -> KunaResult<VarnodeId> {
        let mut cloner = CloneBlockOps::new();
        cloner.clone_expression(self, ops, follow_op)
    }

    /// Gather all structured blocks that have a \e goto edge to a RETURN block
    /// (C++ `ActionReturnSplit::gatherReturnGotos`, `blockaction.cc:2206`).
    ///
    /// Collects BlockGoto/BlockIf nodes (in the structured `sblocks` tree) whose
    /// unstructured branch flows to `parent` (a basic block ending in RETURN).
    /// Each found block is marked (`set_mark`) and pushed into `vec`.
    /// `parent` is a `bblocks` basic block id.
    fn gather_return_gotos(&mut self, parent: BlockId, vec: &mut Vec<BlockId>) {
        use crate::block::BlockType;
        let size_in = self.bblocks_ref().block(parent).size_in();
        for i in 0..size_in {
            let inbl = self.bblocks_ref().block(parent).get_in(i);
            // bblock -> its structured copy.
            let mut bl = self.bblocks_ref().block(inbl).get_copy_map();
            while let Some(cur) = bl {
                if !self.sblocks_ref().block(cur).is_mark() {
                    let mut ret: Option<BlockId> = None;
                    let ty = self.sblocks_ref().block(cur).get_type();
                    if ty == BlockType::Goto {
                        if self.sblocks_ref().goto_prints(cur) {
                            ret = self.sblocks_ref().block(cur).get_goto_target();
                        }
                    } else if ty == BlockType::If {
                        // ifgoto block: get target, otherwise null.
                        ret = self.sblocks_ref().block(cur).get_if_goto_target();
                    }
                    if let Some(r) = ret {
                        // C++ descends `while(ret->getType()!=t_basic) ret=ret->subBlock(0)`
                        // then tests `ret == parent` (a BlockBasic pointer).  In this
                        // cross-arena port the sblocks leaves are `BlockCopy` nodes
                        // (t_copy) whose `copy` field is the *bblocks* id they mirror;
                        // `descend_to_bblock` walks subBlock(0) through the structured
                        // subtree and resolves the leaf Copy to its bblock, so the
                        // comparison is `leaf_bblock == parent` (both bblocks ids).
                        if self.descend_to_bblock(r) == Some(parent) {
                            self.sblocks_mut().block_mut(cur).set_mark();
                            vec.push(cur);
                        }
                    }
                }
                bl = self.sblocks_ref().block(cur).get_parent();
            }
        }
    }

    /// Descend a structured (`sblocks`) block to the `bblocks` basic block it
    /// fronts (the cross-arena form of the C++
    /// `while(ret->getType()!=t_basic) ret=ret->subBlock(0)` walk that ends at a
    /// BlockBasic).  Walks `subBlock(0)` until a `BlockCopy` leaf, then returns the
    /// `copy` field (the mirrored `bblocks` id).  `None` if no leaf is reachable.
    fn descend_to_bblock(&self, start: BlockId) -> Option<BlockId> {
        use crate::block::BlockType;
        let mut r = start;
        loop {
            match self.sblocks_ref().block(r).get_type() {
                BlockType::Copy => return self.sblocks_ref().block(r).get_copy(),
                BlockType::Basic => return Some(r), // single-arena fallback
                _ => match self.sblocks_ref().sub_block(r, 0) {
                    Some(s) => r = s,
                    None => return None,
                },
            }
        }
    }

    /// Given a basic block ending in a RETURN, determine if there is any other
    /// substantive operation in the block (C++ `ActionReturnSplit::isSplittable`,
    /// `blockaction.cc:2242`).  Only MULTIEQUAL / COPY / RETURN ops with
    /// constant/annotation/non-free inputs are allowed.
    fn return_split_is_splittable(&self, b: BlockId) -> bool {
        let mut global_stores: usize = 0;
        let mut cur = self.bb_op_head(b);
        while let Some(op) = cur {
            let o = self.obank().get(op).expect("isSplittable: stale op");
            let opc = o.code();
            let next = o.basic_neighbours().1;
            match opc {
                OpCode::CPUI_MULTIEQUAL => {}
                OpCode::CPUI_COPY | OpCode::CPUI_RETURN => {
                    // (kuna `retsplitglobal`) upstream never looks at the COPY's
                    // OUTPUT, so a shared epilogue that stores to globals reads
                    // as a bare epilogue and gets cloned into every predecessor.
                    // See `crate::p8_structure::kuna_retsplitglobal`.
                    // (kuna `retsplitglobal`) upstream never looks at the COPY's
                    // OUTPUT, so a shared epilogue that stores to globals reads
                    // as a bare epilogue and gets cloned into every predecessor.
                    // See `crate::p8_structure::kuna_retsplitglobal`.
                    let outvn = o.get_out().and_then(|ov| self.vbank().get(ov));
                    let same_storage = match (outvn, o.get_in(0).and_then(|iv| self.vbank().get(iv)))
                    {
                        (Some(ov), Some(iv)) => ov.get_addr() == iv.get_addr(),
                        _ => false,
                    };
                    if crate::p8_structure::kuna_retsplitglobal::is_global_store(
                        opc == OpCode::CPUI_COPY,
                        outvn.map(|v| v.is_persist()).unwrap_or(false),
                        o.is_return_copy(),
                        same_storage,
                    ) {
                        global_stores += 1;
                    }
                    let n = o.num_input();
                    for i in 0..n {
                        let inv = o.get_in(i).expect("isSplittable: input slot");
                        let v = self.vbank().get(inv).expect("isSplittable: stale vn");
                        if v.is_constant() || v.is_annotation() {
                            continue;
                        }
                        if v.is_free() {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
            cur = next;
        }
        // (kuna `retsplitglobal`) a bare epilogue is cheap to clone; one that
        // stores to more globals than the bound is not one.
        !crate::p8_structure::kuna_retsplitglobal::split_is_declined(
            self.get_arch().ret_split_global,
            global_stores,
        )
    }

    /// Split the epilog of the function (C++ `ActionReturnSplit::apply`,
    /// `blockaction.cc:2265`).  Returns the number of edges split (the action's
    /// change count).
    pub(crate) fn return_split_apply(&mut self) -> int4 {
        let mut count = 0;
        if self.sblocks_get_size() == 0 {
            return 0; // Some other restructuring happened first
        }
        let mut splitedge: Vec<int4> = Vec::new();
        let mut retnode: Vec<BlockId> = Vec::new();

        let return_ops: Vec<OpId> = self.obank().iter_code(OpCode::CPUI_RETURN).collect();
        for op in return_ops {
            if self.obank().get(op).expect("returnsplit: stale op").is_dead() {
                continue;
            }
            let parent = match self.obank().get(op).expect("returnsplit").get_parent() {
                Some(p) => p,
                None => continue,
            };
            if self.bblocks_ref().block(parent).size_in() <= 1 {
                continue;
            }
            if !self.return_split_is_splittable(parent) {
                continue;
            }
            let mut gotoblocks: Vec<BlockId> = Vec::new();
            self.gather_return_gotos(parent, &mut gotoblocks);
            if gotoblocks.is_empty() {
                continue;
            }

            let mut splitcount = 0;
            // splitedge holds edges to split, biggest index first, so edge removal
            // does not shift the index of remaining edges.
            let size_in = self.bblocks_ref().block(parent).size_in();
            for i in (0..size_in).rev() {
                let inbl = self.bblocks_ref().block(parent).get_in(i);
                let mut bl = self.bblocks_ref().block(inbl).get_copy_map();
                while let Some(cur) = bl {
                    if self.sblocks_ref().block(cur).is_mark() {
                        splitedge.push(i);
                        retnode.push(parent);
                        bl = None;
                        splitcount += 1;
                    } else {
                        bl = self.sblocks_ref().block(cur).get_parent();
                    }
                }
            }

            for &gb in &gotoblocks {
                self.sblocks_mut().block_mut(gb).clear_mark(); // Clear our marks
            }

            // Can't split ALL in edges.
            if size_in == splitcount {
                splitedge.pop();
                retnode.pop();
            }
        }

        for i in 0..splitedge.len() {
            self.node_split(retnode[i], splitedge[i]).expect("ActionReturnSplit: nodeSplit");
            count += 1;
        }
        count
    }

    /// (kuna) Duplicate a shared **bare-epilogue** RETURN block into each of its
    /// predecessors but one — kuna's analog of angr's SAILR gotoless
    /// `ReturnDuplicatorHigh` (driver for
    /// [`ActionReturnDup`](crate::p8_structure::kuna_returndup::ActionReturnDup),
    /// option `returndup`).  Returns the number of edges split.
    ///
    /// Unlike [`Self::return_split_apply`] (the goto-driven `ReturnDuplicatorLow`
    /// analog, which needs the structured `sblocks` tree to find goto-in edges via
    /// [`Self::gather_return_gotos`]), this runs **pre-structuring on the raw bblocks
    /// CFG**: for every RETURN block with `size_in > 1` that passes the *existing*
    /// [`Self::return_split_is_splittable`] filter (only MULTIEQUAL/COPY/RETURN over
    /// constant/annotation/non-free inputs — a bare epilogue, exactly angr's "simple
    /// return graph"), it splits every in-edge but the first via the *existing*
    /// [`Self::node_split`] (the exact p-code-clone + control-flow-duplication
    /// machinery `ActionReturnSplit` drives).  Each predecessor then owns its own
    /// return, so the guards no longer share an out-target and
    /// [`rule_block_or`](crate::blockaction) cannot comma-merge them — the structurer
    /// yields the source's early-return `if`s.
    ///
    /// Bounded by construction (the splittable filter admits only a side-effect-free
    /// return-only block, so `node_split` never clones a call/store) and by an explicit
    /// per-function cap (`max_splits`) plus a per-block in-degree cap (`max_inedges`):
    /// a block with more predecessors than `max_inedges` is left merged.  A
    /// `node_split` that errors (e.g. redundant in edges) stops splitting that block.
    /// True if the shared RETURN block `b` returns a CONSTANT value (or a phi/COPY
    /// resolving to constants) — angr `ReturnDuplicatorHigh._should_duplicate_dst =
    /// dst_is_const_ret` (it only duplicates *simple const-return* regions). A
    /// `return <variable>` block is NOT const-ret. kuna's unfiltered duplication of
    /// variable-return shares diverges from the source's merged short-circuit form on
    /// the aggregate (decbench measured a ~976 GED-perfect regression); gating on this
    /// makes returndup selective like angr's.
    fn returndup_is_const_ret(&self, b: BlockId) -> bool {
        let mut cur = self.bb_op_head(b);
        let mut ret_op = None;
        while let Some(op) = cur {
            let o = self.obank().get(op).expect("rdup: stale op");
            if o.code() == OpCode::CPUI_RETURN {
                ret_op = Some(op);
            }
            cur = o.basic_neighbours().1;
        }
        let ret_op = match ret_op {
            Some(o) => o,
            None => return false,
        };
        let o = self.obank().get(ret_op).expect("rdup ret");
        // CPUI_RETURN input 0 = return-address offset; input 1 = the returned value.
        if o.num_input() < 2 {
            return true; // void return — trivially a simple const-return region
        }
        match o.get_in(1) {
            Some(valv) => self.returndup_value_is_const(valv),
            None => false,
        }
    }

    /// Constant-resolution helper for [`returndup_is_const_ret`]: a varnode resolves to
    /// a constant if it is a literal/annotation, a COPY chain to one, or a MULTIEQUAL all
    /// of whose inputs recursively resolve to constants (a phi-of-constants: `return 0`
    /// merged from several predecessors, each a `mov eax,K; jmp` = COPY of a literal).
    /// Recursion is depth-bounded (phis can cycle in loops) and never admits a `free`
    /// varnode — a `return <variable>` (a load/call result) is NOT a const return.
    fn returndup_value_is_const(&self, vn: VarnodeId) -> bool {
        self.returndup_value_is_const_rec(vn, 8)
    }

    fn returndup_value_is_const_rec(&self, vn: VarnodeId, depth: u32) -> bool {
        if depth == 0 {
            return false;
        }
        let v = self.vbank().get(vn).expect("rdup vn");
        if v.is_constant() || v.is_annotation() {
            return true;
        }
        if v.is_free() {
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        let d = self.obank().get(def).expect("rdup def");
        match d.code() {
            OpCode::CPUI_MULTIEQUAL => {
                let n = d.num_input();
                for i in 0..n {
                    match d.get_in(i) {
                        Some(iv) if self.returndup_value_is_const_rec(iv, depth - 1) => {}
                        _ => return false,
                    }
                }
                true
            }
            OpCode::CPUI_COPY => match d.get_in(0) {
                Some(iv) => self.returndup_value_is_const_rec(iv, depth - 1),
                None => false,
            },
            _ => false,
        }
    }

    pub(crate) fn returndup_apply(&mut self, max_splits: int4, max_inedges: int4) -> int4 {
        let mut count = 0;

        // Collect the candidate RETURN blocks up front — `node_split` mutates the CFG
        // (adds duplicate blocks, calls `structure_reset`), so we must not iterate the
        // op bank while splitting.
        let return_ops: Vec<OpId> = self.obank().iter_code(OpCode::CPUI_RETURN).collect();
        let mut parents: Vec<BlockId> = Vec::new();
        for op in return_ops {
            let o = match self.obank().get(op) {
                Some(o) => o,
                None => continue,
            };
            if o.is_dead() {
                continue;
            }
            let parent = match o.get_parent() {
                Some(p) => p,
                None => continue,
            };
            let size_in = self.bblocks_ref().block(parent).size_in();
            if size_in <= 1 || size_in > max_inedges {
                continue;
            }
            if !self.return_split_is_splittable(parent) {
                continue;
            }
            // (kuna, angr-selective) Only duplicate a SIMPLE CONST-RETURN region — angr
            // `ReturnDuplicatorHigh._should_duplicate_dst = dst_is_const_ret`. kuna used to
            // duplicate every splittable shared return, INCLUDING `return <variable>`, which
            // diverges from the source's merged short-circuit form on the aggregate (decbench:
            // ~976 GED-perfect regression across 21768 firings). Const-return guard clauses
            // (`if (c) return 0;`) are the early-return shape the source actually wrote, so
            // restricting to them keeps the wins (factor, compspec_dispose) and drops the
            // over-eager variable-return splits (~57% of candidates on coreutils b2sum).
            if !self.returndup_is_const_ret(parent) {
                continue;
            }
            if !parents.contains(&parent) {
                parents.push(parent);
            }
        }

        // (kuna, `option orchain`) A shared return that is the out-target two guards
        // must keep in common is the operand chain of a short-circuit expression:
        // splitting it blocks `rule_block_or` permanently, and the source's
        // `return a || (b && c);` becomes a cascade of constant-return guards.
        if self.get_arch().returndup_orchain {
            let shared = crate::p8_structure::kuna_orchain::shortcircuit_shared_targets(self);
            if parents.iter().any(|p| shared.contains(p)) {
                return 0;
            }
        }

        // For each candidate, split every in-edge but the first (leave edge 0 on the
        // original block — never split ALL edges).  Split the highest index each pass
        // so removing an edge does not shift the lower indices.
        for parent in parents {
            loop {
                if count >= max_splits {
                    return count;
                }
                let size_in = self.bblocks_ref().block(parent).size_in();
                if size_in <= 1 {
                    break; // one predecessor left on the original — done with this block
                }
                match self.node_split(parent, size_in - 1) {
                    Ok(()) => count += 1,
                    // Redundant in edges (or any other split rejection) — stop this
                    // block; leave it merged rather than risk a partial/incorrect split.
                    Err(_) => break,
                }
            }
        }
        count
    }

    /// (kuna, angr `earlyreturn`) The in-edge indices of shared RETURN block `b` whose
    /// contribution to the return-value phi is a **constant** — the early-return guard
    /// arms (`if (c) return 0;`).  This is the PER-EDGE analog of
    /// [`Self::returndup_is_const_ret`] (which requires the *whole* return value const and
    /// so rejects the common mixed diamond `MULTIEQUAL(#0, <var>)` = a const guard arm + a
    /// variable-return body).  angr's `ReturnDuplicatorHigh` peels per-predecessor via
    /// `_is_simple_return_graph`, which admits a variable body return; mirroring that,
    /// return just the input slots that resolve to constants.  Empty unless the returned
    /// value is a `CPUI_MULTIEQUAL` defined in `b` (phi input slot `i` aligns with in-edge
    /// `i`, so a slot index is directly a [`Self::node_split`] edge index).
    fn earlyreturn_const_edges(&self, b: BlockId) -> Vec<int4> {
        // Find the RETURN op in b and its returned value (input 1).
        let mut cur = self.bb_op_head(b);
        let mut ret_val: Option<VarnodeId> = None;
        while let Some(op) = cur {
            let o = self.obank().get(op).expect("earlyret: stale op");
            if o.code() == OpCode::CPUI_RETURN && o.num_input() >= 2 {
                ret_val = o.get_in(1);
            }
            cur = o.basic_neighbours().1;
        }
        let ret_val = match ret_val {
            Some(v) => v,
            None => return Vec::new(), // void return — nothing to peel per-edge
        };
        // The returned value must be defined by a MULTIEQUAL *in this block* (the phi that
        // merges the arm values); its input slots align 1:1 with b's in-edges.
        let v = self.vbank().get(ret_val).expect("earlyret vn");
        let def = match v.get_def() {
            Some(d) => d,
            None => return Vec::new(),
        };
        let d = self.obank().get(def).expect("earlyret def");
        if d.code() != OpCode::CPUI_MULTIEQUAL || d.get_parent() != Some(b) {
            return Vec::new();
        }
        let n = d.num_input();
        let mut edges = Vec::new();
        for i in 0..n {
            if let Some(iv) = d.get_in(i) {
                if self.returndup_value_is_const(iv) {
                    edges.push(i);
                }
            }
        }
        edges
    }

    /// (kuna) Peel the **constant arms** out of a shared RETURN block into their own
    /// per-predecessor `return K`, so `if (c) { return K; } body; return <var>;` (the
    /// early-return guard) and `switch (x) { case A: return K0; ... }` (the multi-way
    /// const dispatch) recover per-predecessor instead of kuna's merged
    /// `v = MULTIEQUAL(#K0, #K1, …); return v;` — angr SAILR `ReturnDuplicatorHigh`, but
    /// PER-EDGE (see [`Self::earlyreturn_const_edges`]).  Distinct from
    /// [`Self::returndup_apply`], which requires the *whole* return const and so cannot
    /// reach the mixed const/variable diamond; this only ever splits a **constant** arm,
    /// so it cannot re-introduce the variable-return over-firing that made broad
    /// `returndup` regress the aggregate.  Same reuse of
    /// [`Self::return_split_is_splittable`] + [`Self::node_split`], and it never splits the
    /// last remaining edge (leaves the variable body merged).
    ///
    /// The shared core behind both [`Self::earlyreturn_apply`] (the narrow diamond, capped
    /// at 16 in-edges by [`ActionEarlyReturn`](crate::p8_structure::kuna_earlyreturn)) and
    /// [`Self::switchreturn_apply`] (the wide multi-case switch-phi that exceeds that cap,
    /// gated by [`ActionSwitchReturn`](crate::p8_structure::kuna_switchreturn)) — the ONLY
    /// difference between the two is the `max_inedges` / `max_splits` caps.
    fn const_return_peel(&mut self, max_splits: int4, max_inedges: int4) -> int4 {
        let mut count = 0;
        // Collect candidates up front — node_split mutates the CFG.
        let return_ops: Vec<OpId> = self.obank().iter_code(OpCode::CPUI_RETURN).collect();
        let mut work: Vec<(BlockId, Vec<int4>)> = Vec::new();
        for op in return_ops {
            let o = match self.obank().get(op) {
                Some(o) => o,
                None => continue,
            };
            if o.is_dead() {
                continue;
            }
            let parent = match o.get_parent() {
                Some(p) => p,
                None => continue,
            };
            let size_in = self.bblocks_ref().block(parent).size_in();
            if size_in <= 1 || size_in > max_inedges {
                continue;
            }
            if !self.return_split_is_splittable(parent) {
                continue;
            }
            let mut edges = self.earlyreturn_const_edges(parent);
            if edges.is_empty() {
                continue;
            }
            edges.sort_unstable(); // ascending; slots align with in-edge indices
            // Never split ALL in-edges — if every arm is const (the whole-block returndup
            // shape), leave the lowest merged so a predecessor remains on the original.
            if edges.len() as int4 >= size_in {
                edges.remove(0);
            }
            if edges.is_empty() {
                continue;
            }
            if !work.iter().any(|(b, _)| *b == parent) {
                work.push((parent, edges));
            }
        }
        // Split highest edge index first, so removing an edge does not shift lower indices.
        for (parent, edges) in work {
            for &i in edges.iter().rev() {
                if count >= max_splits {
                    return count;
                }
                if self.bblocks_ref().block(parent).size_in() <= 1 {
                    break; // one predecessor left — leave it merged
                }
                match self.node_split(parent, i) {
                    Ok(()) => count += 1,
                    Err(_) => break,
                }
            }
        }
        count
    }

    /// (kuna, `earlyreturn`) Peel the const guard arm(s) of a **narrow** shared-return
    /// diamond (`if (c) return K; body; return <var>;`) — the ActionEarlyReturn driver.
    /// Thin wrapper over [`Self::const_return_peel`] with earlyreturn's caps (≤16 in-edges).
    pub(crate) fn earlyreturn_apply(&mut self, max_splits: int4, max_inedges: int4) -> int4 {
        self.const_return_peel(max_splits, max_inedges)
    }

    /// (kuna, `switchreturn`) Peel the const case arms of a **wide multi-way switch-phi**
    /// shared-return block (`switch (x) { case A: v = K0; break; … } return v;` →
    /// `case A: return K0; …`) — the ActionSwitchReturn driver.  The direct continuation of
    /// [`Self::earlyreturn_apply`]: identical per-edge const-peel machinery
    /// ([`Self::earlyreturn_const_edges`] + [`Self::node_split`]), but with a wider in-edge
    /// cap so the many-case dispatch table that overflows earlyreturn's ≤16 limit
    /// (e.g. libedit `tty__getcharindex`, 17 const cases) is reached too.  Because it still
    /// only ever splits a **constant** arm (never a `return <variable>` share), it inherits
    /// earlyreturn's safety: it cannot cause the variable-return over-firing that made broad
    /// `returndup` regress the aggregate.
    pub(crate) fn switchreturn_apply(&mut self, max_splits: int4, max_inedges: int4) -> int4 {
        self.const_return_peel(max_splits, max_inedges)
    }

    // --- Block-cover helpers (BlockBasic::setInitialRange/copyRange/mergeRange)

    /// First address covered by basic block `bb` (C++ `BlockBasic::getStart`,
    /// `block.cc:2323`): the first address of the first cover range, or an invalid
    /// address when the cover is empty.
    fn bb_get_start(&self, bb: BlockId) -> Address {
        match self.bblocks_ref().block(bb).kind() {
            BlockKind::Basic(b) => match b.cover.iter().next() {
                Some(range) => range.get_first_addr(),
                None => Address::new_invalid(),
            },
            _ => Address::new_invalid(),
        }
    }

    /// (kuna GH-8467) The entry address of a structured block, for angr-style
    /// `label_<addr>` goto-label naming (C++ `PrintC::emitLabelStatement`'s
    /// `getFrontLeaf()` basic-block start).  Invalid when the block has no leaf.
    pub fn sblock_entry_addr(&self, bl: BlockId) -> Address {
        let front = match self.sblocks_ref().get_front_leaf(bl) {
            Some(f) => f,
            None => return Address::new_invalid(),
        };
        match self.sblocks_ref().sub_block(front, 0) {
            Some(bb) => self.bb_get_start(bb),
            None => Address::new_invalid(),
        }
    }

    /// Mutable access to a basic block's address cover (`BasicData::cover`).
    fn basic_cover_mut(&mut self, bb: BlockId) -> &mut kuna_base::address::RangeList {
        match self.bblocks_mut().block_mut(bb).kind_mut() {
            BlockKind::Basic(b) => &mut b.cover,
            _ => panic!("basic_cover_mut: expected BlockBasic (internal invariant)"),
        }
    }

    /// Copy the address cover from `src` into `dst` (C++ `BlockBasic::copyRange`).
    fn copy_basic_range(&mut self, src: BlockId, dst: BlockId) {
        let cover = match self.bblocks_ref().block(src).kind() {
            BlockKind::Basic(b) => b.cover.clone(),
            _ => panic!("copy_basic_range: src not BlockBasic"),
        };
        *self.basic_cover_mut(dst) = cover;
    }

    /// Merge the address cover of `src` into `dst` (C++ `BlockBasic::mergeRange`).
    fn merge_basic_range(&mut self, dst: BlockId, src: BlockId) {
        let src_ranges: Vec<(std::rc::Rc<kuna_base::space::AddrSpace>, u64, u64)> =
            match self.bblocks_ref().block(src).kind() {
                BlockKind::Basic(b) => b
                    .cover
                    .iter()
                    .map(|r| (r.get_space().clone(), r.get_first(), r.get_last()))
                    .collect(),
                _ => panic!("merge_basic_range: src not BlockBasic"),
            };
        let cover = self.basic_cover_mut(dst);
        for (spc, first, last) in src_ranges {
            cover.insert_range(spc, first, last);
        }
    }

    /// Merge the given basic block with the block it flows into
    /// (C++ `Funcdata::spliceBlockBasic`, `funcdata_block.cc:927`).
    ///
    /// The block-level mechanics (validate the single in/out chain, transfer the
    /// op list, reparent ops, recompute order, merge the cover, splice the
    /// blocks) are faithful.  The two p-code edits — destroying a trailing branch
    /// op and rejecting a leading MULTIEQUAL — need `opDestroy` (funcdata_op) and
    /// op-list inspection; the MULTIEQUAL check and the branch-op removal are
    /// `// STUB(W3-op)`-noted below.  The MULTIEQUAL rejection is enforced (it is
    /// a read-only `code()` check), but the branch-op destruction is deferred to
    /// the funcdata_op wave; until then a trailing branch is moved with the rest
    /// of the list (an over-approximation flagged here, not silently wrong: the
    /// caller in ConditionalExecution only splices blocks whose tail is not a
    /// 2-way/n-way branch, and a plain unconditional BRANCH op left in the middle
    /// is dead and removed by later dead-code; W3 has no such later pass yet, so
    /// this method returns `Err` if a branch op would be stranded).
    pub fn splice_block_basic(&mut self, bl: BlockId) -> KunaResult<()> {
        // Validate the single-out / single-in chain.
        let outbl = {
            let mut o: Option<BlockId> = None;
            if self.bblocks_ref().block(bl).size_out() == 1 {
                let cand = self.bblocks_ref().block(bl).get_out(0);
                if self.bblocks_ref().block(cand).size_in() == 1 {
                    o = Some(cand);
                }
            }
            o.ok_or_else(|| KunaError::lowlevel("Cannot splice basic blocks"))?
        };

        // Remove any jump op at the end of -bl- (C++ funcdata_block.cc:941).
        // `op_destroy` is now available (used by condexe.execute), so the
        // W3-op boundary is closed.
        if let Some(jumpop) = self.bb_op_tail(bl) {
            if self.obank().get(jumpop).expect("spliceBlockBasic").is_branch() {
                self.op_destroy(jumpop);
            }
        }

        if !self.bb_empty_op(outbl) {
            // Check for MULTIEQUALs at the head of -outbl-.
            let firstop = self.bb_op_head(outbl).expect("spliceBlockBasic: head");
            if self.obank().get(firstop).expect("spliceBlockBasic").code()
                == kuna_num::opcodes::OpCode::CPUI_MULTIEQUAL
            {
                return Err(KunaError::lowlevel("Splicing block with MULTIEQUAL"));
            }
            self.obank_mut()
                .get_mut(firstop)
                .expect("spliceBlockBasic")
                .clear_flag(crate::op::pcodeop_flags::startbasic);

            // Move all ops from -outbl- onto the end of -bl-, reparenting.
            let ops: Vec<OpId> = self.bb_ops(outbl);
            for op in ops {
                self.bb_remove_op(outbl, op);
                self.bb_insert_op(op, bl, None); // append at end; reparents + reorders
            }
            // bl->setOrder() is implied by the per-insert ordering; force a clean
            // recompute to mirror the C++ trailing `bl->setOrder()`.
            self.bb_set_order(bl);
        }

        self.merge_basic_range(bl, outbl); // Update the address cover
        let graph = self.bblocks_root_pub();
        self.bblocks_mut().splice_block(graph, bl)?;
        self.structure_reset();
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Small internal helpers exposed for the block ports (kept private to crate)
    // -----------------------------------------------------------------------

    /// The root graph node of `bblocks` (public-within-crate wrapper so the
    /// funcdata_block methods can reach it without re-deriving).
    pub(crate) fn bblocks_root_pub(&self) -> BlockId {
        self.bblocks_ref().root.expect("bblocks root not constructed (internal invariant)")
    }

    // -----------------------------------------------------------------------
    // ActionPreferComplement support (C++ `FlowBlock::preferComplement` and the
    // `getSplitPoint`/`flipInPlaceTest`/`flipInPlaceExecute` virtual cascade,
    // block.cc:2365-2401 / 3024-3062 / block.hh:850-892).
    //
    // The structured graph (`sblocks`) is a `BlockCopy` mirror of the live basic-
    // block graph (`bblocks`): each leaf `BlockCopy::copy` points at the bblocks
    // `BlockBasic` carrying the ops.  A split point therefore resolves to one of
    // two arenas: a bblocks `BlockBasic` (the leaf condition) or an sblocks
    // `BlockCondition` (a short-circuit AND/OR).  `SplitPoint` tags which.
    // -----------------------------------------------------------------------

    /// `getBlock(0)->getSplitPoint()` resolved across the dual arena.  Walks the
    /// sblocks structure from `sbl` (an sblocks node) down to the deepest
    /// component that performs a conditional split, returning `None` if the
    /// branch doesn't end in a 2-way condition (C++ `getSplitPoint` virtuals).
    fn get_split_point(&self, sbl: BlockId) -> Option<SplitPoint> {
        use crate::block::BlockType;
        match self.sblocks_ref().block(sbl).get_type() {
            // The copy is a bblocks BlockBasic whose getSplitPoint returns itself
            // iff sizeOut==2.
            BlockType::Copy => {
                let bb = self.sblocks_ref().block(sbl).get_copy()?;
                if self.bblocks_ref().block(bb).size_out() != 2 {
                    return None;
                }
                Some(SplitPoint::Basic(bb))
            }
            BlockType::Basic => {
                if self.sblocks_ref().block(sbl).size_out() != 2 {
                    return None;
                }
                // A bare BlockBasic should not appear in sblocks (leaves are
                // BlockCopy), but mirror C++ in case a hand-built graph does.
                Some(SplitPoint::Basic(sbl))
            }
            BlockType::Ls => {
                let n = self.sblocks_ref().block(sbl).get_size();
                if n == 0 {
                    return None;
                }
                let last = self.sblocks_ref().block(sbl).get_block(n - 1);
                self.get_split_point(last)
            }
            BlockType::Condition => Some(SplitPoint::Condition(sbl)),
            _ => None,
        }
    }

    /// `split->flipInPlaceTest(fliplist,allowOpRemoval)` across the dual arena
    /// (C++ `BlockBasic::flipInPlaceTest` block.cc:2372, `BlockCondition::`
    /// block.cc:3038).  Returns 0 (flip normalizes), 1 (denormalizes), 2 (not
    /// possible); appends the ops to adjust to `fliplist`.
    fn split_flip_in_place_test(
        &self,
        split: SplitPoint,
        fliplist: &mut Vec<OpId>,
        allow_op_removal: bool,
    ) -> int4 {
        match split {
            SplitPoint::Basic(bb) => {
                let lastop = match self.bb_op_tail(bb) {
                    Some(op) => op,
                    None => return 2,
                };
                if self.obank().get(lastop).expect("flip_test: stale op").code() != OpCode::CPUI_CBRANCH {
                    return 2;
                }
                if self.obank().get(lastop).expect("flip_test: stale op").is_boolean_flip() {
                    // Flip bit set: don't change any ops (use a throwaway list).
                    let mut unused: Vec<OpId> = Vec::new();
                    self.op_flip_in_place_test(lastop, &mut unused, allow_op_removal)
                } else {
                    self.op_flip_in_place_test(lastop, fliplist, allow_op_removal)
                }
            }
            SplitPoint::Condition(scond) => {
                let b0 = self.sblocks_ref().block(scond).get_block(0);
                let split1 = match self.get_split_point(b0) {
                    Some(s) => s,
                    None => return 2,
                };
                let b1 = self.sblocks_ref().block(scond).get_block(1);
                let split2 = match self.get_split_point(b1) {
                    Some(s) => s,
                    None => return 2,
                };
                let subtest1 = self.split_flip_in_place_test(split1, fliplist, allow_op_removal);
                if subtest1 == 2 {
                    return 2;
                }
                let subtest2 = self.split_flip_in_place_test(split2, fliplist, allow_op_removal);
                if subtest2 == 2 {
                    return 2;
                }
                subtest1
            }
        }
    }

    /// `split->flipInPlaceExecute()` across the dual arena (C++
    /// `BlockBasic::flipInPlaceExecute` block.cc:2391, `BlockCondition::`
    /// block.cc:3056).  Reverses the split's outgoing edge order without
    /// modifying the comparison ops (those are handled by `opFlipInPlaceExecute`).
    fn split_flip_in_place_execute(&mut self, split: SplitPoint) {
        match split {
            SplitPoint::Basic(bb) => {
                if let Some(lastop) = self.bb_op_tail(bb) {
                    let o = self.obank_mut().get_mut(lastop).expect("flip_exec: stale op");
                    o.flip_flag(crate::op::pcodeop_flags::fallthru_true);
                    if o.is_boolean_flip() {
                        o.flip_flag(crate::op::pcodeop_flags::boolean_flip);
                    }
                }
                // Swap the out-edge order on the bblocks block.
                self.bblocks_mut().swap_edges(bb);
            }
            SplitPoint::Condition(scond) => {
                self.sblocks_mut().block_mut(scond).flip_condition_opcode();
                let b0 = self.sblocks_ref().block(scond).get_block(0);
                if let Some(s0) = self.get_split_point(b0) {
                    self.split_flip_in_place_execute(s0);
                }
                let b1 = self.sblocks_ref().block(scond).get_block(1);
                if let Some(s1) = self.get_split_point(b1) {
                    self.split_flip_in_place_execute(s1);
                }
            }
        }
    }

    /// `BlockIf::preferComplement` (C++ block.cc:3141): for a 3-component
    /// if/else, rearrange to the complementary (preferred) boolean form when the
    /// condition's split point can be flipped to a normalized comparison.  `sif`
    /// is the sblocks `BlockIf`.  Returns `true` if a change was made.
    fn block_if_prefer_complement(&mut self, sif: BlockId, allow_op_removal: bool) -> KunaResult<bool> {
        // Only if/else collapses.
        if self.sblocks_ref().block(sif).get_size() != 3 {
            return Ok(false);
        }
        let cond = self.sblocks_ref().block(sif).get_block(0);
        let split = match self.get_split_point(cond) {
            Some(s) => s,
            None => return Ok(false),
        };
        let mut fliplist: Vec<OpId> = Vec::new();
        if self.split_flip_in_place_test(split, &mut fliplist, allow_op_removal) != 0 {
            return Ok(false);
        }
        self.split_flip_in_place_execute(split);
        self.op_flip_in_place_execute(&fliplist)?;
        // Swap the BlockIf's true/false children.
        self.sblocks_mut().swap_blocks(sif, 1, 2);
        Ok(true)
    }

    /// `ActionPreferComplement::apply` (C++ blockaction.cc:2140): walk the
    /// structured-block tree and call `preferComplement` on every `BlockGraph`
    /// node (only `BlockIf` overrides it non-trivially).  Returns the number of
    /// nodes rearranged.
    pub fn prefer_complement(&mut self, allow_op_mods: bool) -> KunaResult<int4> {
        use crate::block::BlockType;
        if self.sblocks_get_size() == 0 {
            return Ok(0);
        }
        let root = self.sblocks_root();
        if self.sblocks_ref().block(root).has_final_transform() {
            return Ok(0);
        }
        let mut count = 0;
        // BFS the BlockGraph nodes (skip t_copy/t_basic leaves), preferComplement each.
        let mut vec: Vec<BlockId> = vec![root];
        let mut pos = 0;
        while pos < vec.len() {
            let curbl = vec[pos];
            pos += 1;
            let sz = self.sblocks_ref().block(curbl).get_size();
            for i in 0..sz {
                let childbl = self.sblocks_ref().block(curbl).get_block(i);
                let bt = self.sblocks_ref().block(childbl).get_type();
                if bt == BlockType::Copy || bt == BlockType::Basic {
                    continue;
                }
                vec.push(childbl);
            }
            // Only BlockIf has a non-trivial preferComplement; all others are no-ops.
            if self.sblocks_ref().block(curbl).get_type() == BlockType::If
                && self.block_if_prefer_complement(curbl, allow_op_mods)?
            {
                count += 1;
            }
        }
        // data.clearDeadOps();  -- clear ops removed by the BOOL_NEGATE folding.
        self.clear_dead_ops();
        Ok(count)
    }

    /// (kuna, option `branchflip`) Flip negated-guard `if/else` branches for
    /// linearity — the angr-SAILR-style condition-polarity normalization.
    ///
    /// This is the *complement* of [`Self::prefer_complement`]: where
    /// `preferComplement` rearranges only when the flip **removes** negation
    /// (`split_flip_in_place_test == 0`, e.g. `!=` / `BOOL_NEGATE` / non-constant
    /// `<`), this rearranges the residual `if/else` blocks whose guard is the
    /// *negated / equality-to-zero* form (`split_flip_in_place_test == 1`, i.e.
    /// `==` / `== 0` / a constant-operand `<` / `<=`).  Flipping those yields the
    /// positive-complement `if (x != 0) {else-body} else {if-body}` reading angr
    /// prefers (`if (a0 != 0)` vs `if (a0 == 0)` on tee_O2's `x2nrealloc`).
    ///
    /// Each flip reuses the proven in-place machinery (`split_flip_in_place_*` +
    /// `op_flip_in_place_execute` + `swap_blocks(sif,1,2)`) and is logged as a
    /// `branchflip:` warning comment at the guard's CBRANCH so the rewrite is
    /// observable.  Returns the number of `if/else` blocks flipped.
    pub fn branch_flip_complement(&mut self, allow_op_mods: bool) -> KunaResult<int4> {
        use crate::block::BlockType;
        if self.sblocks_get_size() == 0 {
            return Ok(0);
        }
        let root = self.sblocks_root();
        if self.sblocks_ref().block(root).has_final_transform() {
            return Ok(0);
        }
        let mut count = 0;
        // BFS the BlockGraph nodes (skip t_copy/t_basic leaves), same walk as
        // `prefer_complement`.
        let mut vec: Vec<BlockId> = vec![root];
        let mut pos = 0;
        while pos < vec.len() {
            let curbl = vec[pos];
            pos += 1;
            let sz = self.sblocks_ref().block(curbl).get_size();
            for i in 0..sz {
                let childbl = self.sblocks_ref().block(curbl).get_block(i);
                let bt = self.sblocks_ref().block(childbl).get_type();
                if bt == BlockType::Copy || bt == BlockType::Basic {
                    continue;
                }
                vec.push(childbl);
            }
            if self.sblocks_ref().block(curbl).get_type() == BlockType::If
                && self.block_if_flip_negated_guard(curbl, allow_op_mods)?
            {
                count += 1;
            }
        }
        self.clear_dead_ops();
        Ok(count)
    }

    /// (kuna, option `branchflip`) The per-`BlockIf` half of
    /// [`Self::branch_flip_complement`]: flip a 3-component `if/else` whose guard
    /// is the negated / equality-to-zero form to its positive complement.
    ///
    /// Mirrors [`Self::block_if_prefer_complement`] but accepts the *denormalizing*
    /// flip class (`split_flip_in_place_test == 1`) instead of the normalizing one
    /// (`== 0`).  Result `1` is exactly the equality-to-zero / `==` / constant-
    /// compare guard that `preferComplement` leaves alone — the one whose positive
    /// reading angr produces.  `sif` is the sblocks `BlockIf`.  Returns `true` if a
    /// flip was made (and logs a `branchflip:` warning at the guard).
    fn block_if_flip_negated_guard(&mut self, sif: BlockId, allow_op_removal: bool) -> KunaResult<bool> {
        // Only if/else (3-component) collapses can swap arms in place.
        if self.sblocks_ref().block(sif).get_size() != 3 {
            return Ok(false);
        }
        let cond = self.sblocks_ref().block(sif).get_block(0);
        let split = match self.get_split_point(cond) {
            Some(s) => s,
            None => return Ok(false),
        };
        // Accept ONLY the denormalizing flip (== 1): the negated / equality-to-zero
        // guard.  `0` (already-positive, `preferComplement`'s job) and `2` (can't
        // flip) are both left untouched.
        let mut fliplist: Vec<OpId> = Vec::new();
        if self.split_flip_in_place_test(split, &mut fliplist, allow_op_removal) != 1 {
            return Ok(false);
        }
        // Log the flip at the guard's CBRANCH address (observable in `print C`).
        let warn_addr = self.split_point_addr(split);
        if let Some(ad) = warn_addr {
            self.warning(
                "branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped)",
                &ad,
            );
        }
        // The flip is a pure arm swap: the set of statements printed under this
        // `if` must be exactly the same afterwards, only their arms exchanged.
        // Snapshot the reachable leaf multiset so the debug assertion below can
        // police that — any structural pass in this family that drops, duplicates
        // or re-parents a component trips it (debug builds only).
        #[cfg(debug_assertions)]
        let leaves_before = self.kuna_flip_leaf_multiset(sif);
        self.split_flip_in_place_execute(split);
        self.op_flip_in_place_execute(&fliplist)?;
        // Swap the BlockIf's true/false children to match.
        self.sblocks_mut().swap_blocks(sif, 1, 2);
        #[cfg(debug_assertions)]
        debug_assert_eq!(
            self.kuna_flip_leaf_multiset(sif),
            leaves_before,
            "branchflip: arm swap changed the leaf set under the BlockIf"
        );
        Ok(true)
    }

    /// The sorted multiset of leaf components reachable under the sblocks node
    /// `sbl` — the guard-rail instrument for the arm-swapping structuring family
    /// (debug builds only; see [`Self::block_if_flip_negated_guard`]).
    #[cfg(debug_assertions)]
    fn kuna_flip_leaf_multiset(&self, sbl: BlockId) -> Vec<BlockId> {
        use crate::block::BlockType;
        let mut leaves: Vec<BlockId> = Vec::new();
        let mut stack: Vec<BlockId> = vec![sbl];
        while let Some(cur) = stack.pop() {
            let n = self.sblocks_ref().block(cur).get_size();
            let ty = self.sblocks_ref().block(cur).get_type();
            if n == 0 || ty == BlockType::Copy || ty == BlockType::Basic {
                leaves.push(cur);
                continue;
            }
            for i in 0..n {
                stack.push(self.sblocks_ref().block(cur).get_block(i));
            }
        }
        leaves.sort_unstable();
        leaves
    }

    /// The address of a split point's guard op (the bblocks `BlockBasic`'s tail
    /// CBRANCH), used to key the `branchflip` warning comment.  Returns `None` for
    /// a short-circuit `BlockCondition` (no single guard address).
    fn split_point_addr(&self, split: SplitPoint) -> Option<Address> {
        match split {
            SplitPoint::Basic(bb) => {
                let lastop = self.bb_op_tail(bb)?;
                Some(self.obank().get(lastop)?.get_addr().clone())
            }
            SplitPoint::Condition(_) => None,
        }
    }

    /// Reset `sblocks` to a fresh empty graph (C++ `sblocks.clear()`).
    fn clear_sblocks(&mut self) {
        use crate::block::{BlockGraph, FlowBlock};
        let mut sb = BlockGraph::new();
        let sroot = sb.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        sb.root = Some(sroot);
        *self.sblocks_mut() = sb;
    }

    /// C++ `ActionRestructureVarnode::isCopyConstant` (`coreaction.cc:2232`): is
    /// `vn` a constant or a `COPY` of a constant?
    fn switch_is_copy_constant(&self, vn: VarnodeId) -> bool {
        let v = match self.vbank().get(vn) {
            Some(v) => v,
            None => return false,
        };
        if v.is_constant() {
            return true;
        }
        if !v.is_written() {
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        let o = match self.obank().get(def) {
            Some(o) => o,
            None => return false,
        };
        if o.code() != OpCode::CPUI_COPY {
            return false;
        }
        match o.get_in(0) {
            Some(i0) => self.vbank().get(i0).map(|v| v.is_constant()).unwrap_or(false),
            None => false,
        }
    }

    /// C++ `ActionRestructureVarnode::isDelayedConstant` (`coreaction.cc:2246`):
    /// `vn` is a constant, or the not-yet-simplified `COPY`/`INT_ADD` of constants.
    fn switch_is_delayed_constant(&self, vn: VarnodeId) -> bool {
        let v = match self.vbank().get(vn) {
            Some(v) => v,
            None => return false,
        };
        if v.is_constant() {
            return true;
        }
        if !v.is_written() {
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        // C++ `isDelayedConstant` (coreaction.cc:2245): for CPUI_COPY it reads
        // ONLY getIn(0); getIn(1) is reached solely on the CPUI_INT_ADD path.
        // The slot reads must stay deferred behind the opcode check — a COPY def
        // has a single input, so eagerly reading getIn(1) panics (op.rs:568
        // bounds check).  This is the over-/under-protect bug from the prior wave.
        let opc = match self.obank().get(def) {
            Some(o) => o.code(),
            None => return false,
        };
        if opc == OpCode::CPUI_COPY {
            return self
                .obank()
                .get(def)
                .and_then(|o| o.get_in(0))
                .and_then(|i| self.vbank().get(i))
                .map(|v| v.is_constant())
                .unwrap_or(false);
        }
        if opc != OpCode::CPUI_INT_ADD {
            return false;
        }
        let (i0, i1) = match self.obank().get(def) {
            Some(o) => match (o.get_in(0), o.get_in(1)) {
                (Some(a), Some(b)) => (a, b),
                _ => return false,
            },
            None => return false,
        };
        self.switch_is_copy_constant(i1) && self.switch_is_copy_constant(i0)
    }

    /// C++ `ActionRestructureVarnode::protectSwitchPathIndirects`
    /// (`coreaction.cc:2264`): backtrack the data-flow path from a BRANCHIND switch
    /// destination; if it originates from a constant but passes through INDIRECT
    /// ops, mark the earliest INDIRECT `noIndirectCollapse` so the indirect switch
    /// value is not lost when later analysis collapses INDIRECTs.
    fn protect_switch_path_indirects(&mut self, branchind: OpId) {
        use crate::op::pcodeop_flags as pf;
        let mut last_indirect: Option<OpId> = None;
        let mut cur_vn = match self.obank().get(branchind).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return,
        };
        loop {
            let written = self.vbank().get(cur_vn).map(|v| v.is_written()).unwrap_or(false);
            if !written {
                break;
            }
            let cur_op = match self.vbank().get(cur_vn).and_then(|v| v.get_def()) {
                Some(d) => d,
                None => break,
            };
            // C++ accesses getIn(slot) only within the per-opcode arms below, so a
            // slot is never read past `numInput`.  Read in0/in1 bounds-safely here
            // (a unary/COPY/single-input INDIRECT def has only slot 0 — eagerly
            // reading slot 1 would index out of bounds: op.rs:568 panic).
            let (eval_type, code, ninput, in0, in1) = match self.obank().get(cur_op) {
                Some(o) => {
                    let n = o.num_input();
                    let i0 = if n > 0 { o.get_in(0) } else { None };
                    let i1 = if n > 1 { o.get_in(1) } else { None };
                    (o.get_eval_type(), o.code(), n, i0, i1)
                }
                None => return,
            };
            if (eval_type & (pf::binary | pf::ternary)) != 0 {
                if ninput > 1 {
                    let i0 = match in0 {
                        Some(v) => v,
                        None => return,
                    };
                    let i1 = match in1 {
                        Some(v) => v,
                        None => return,
                    };
                    if self.switch_is_delayed_constant(i1) {
                        cur_vn = i0;
                    } else if self.switch_is_delayed_constant(i0) {
                        cur_vn = i1;
                    } else {
                        return; // Multiple paths
                    }
                } else {
                    cur_vn = match in0 {
                        Some(v) => v,
                        None => return,
                    };
                }
            } else if (eval_type & pf::unary) != 0 {
                cur_vn = match in0 {
                    Some(v) => v,
                    None => return,
                };
            } else if code == OpCode::CPUI_INDIRECT {
                last_indirect = Some(cur_op);
                cur_vn = match in0 {
                    Some(v) => v,
                    None => return,
                };
            } else if code == OpCode::CPUI_LOAD {
                cur_vn = match in1 {
                    Some(v) => v,
                    None => return,
                };
            } else if code == OpCode::CPUI_MULTIEQUAL {
                // A path from a constant that splits and rejoins: protect an INDIRECT
                // input if present, else assume the MULTIEQUAL is unlikely to collapse.
                for k in 0..ninput {
                    let inv = match self.obank().get(cur_op).and_then(|o| o.get_in(k)) {
                        Some(v) => v,
                        None => continue,
                    };
                    if !self.vbank().get(inv).map(|v| v.is_written()).unwrap_or(false) {
                        continue;
                    }
                    let indef = match self.vbank().get(inv).and_then(|v| v.get_def()) {
                        Some(d) => d,
                        None => continue,
                    };
                    if self.obank().get(indef).map(|o| o.code()) == Some(OpCode::CPUI_INDIRECT) {
                        if let Some(o) = self.obank_mut().get_mut(indef) {
                            o.set_no_indirect_collapse();
                        }
                        break;
                    }
                }
                return; // Don't backtrack further
            } else {
                return;
            }
        }
        // Exactly one path, from a constant to a switch.
        if !self.vbank().get(cur_vn).map(|v| v.is_constant()).unwrap_or(false) {
            return;
        }
        if let Some(li) = last_indirect {
            if let Some(o) = self.obank_mut().get_mut(li) {
                o.set_no_indirect_collapse();
            }
        }
    }

    /// C++ `ActionRestructureVarnode::protectSwitchPaths` (`coreaction.cc:2320`):
    /// run through BRANCHIND ops (switches) and protect the data-flow path to the
    /// destination variable from INDIRECT collapse.  Wired into
    /// `ActionRestructureVarnode::apply` (gated on `is_jumptable_recovery_on`) and
    /// byte-faithful (the prior wave's "30 Switch regressions" were really an
    /// out-of-bounds `getIn(1)` panic in `switch_is_delayed_constant` / the walk's
    /// eager input read — now fixed — that aborted jump-table recovery).  It shields
    /// the switch-index INDIRECTs in the jump-table-recovery partial clone from the
    /// RuleIndirectCollapse passes that the `is_unmapped_unaliased` `nolocalalias`
    /// flip enables; with it correct, all switch datatests hold when that flip is
    /// turned on (the flip is staged-off pending the Gp Test #1 over-merge — see
    /// `Funcdata::sync_varnodes_with_symbols`).
    pub(crate) fn protect_switch_paths(&mut self) {
        let nblocks = self.bblocks_get_size();
        let mut branchinds: Vec<OpId> = Vec::new();
        for i in 0..nblocks {
            let bl = self.bblocks_get_block(i);
            let op = match self.bb_op_tail(bl) {
                Some(o) => o,
                None => continue,
            };
            if self.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_BRANCHIND) {
                branchinds.push(op);
            }
        }
        for op in branchinds {
            self.protect_switch_path_indirects(op);
        }
    }
}

/// Helper for cloning the p-code ops of a basic block during a node-split
/// (C++ `CloneBlockOps`, `funcdata_block.cc` / `funcdata.hh`).
///
/// Holds the clone bookkeeping: `clone_list` is the ordered (origOp, cloneOp)
/// pairs, and `orig_to_clone` maps an original op to its clone so input
/// Varnodes can be re-wired in [`patch_inputs`](CloneBlockOps::patch_inputs).
struct CloneBlockOps {
    /// Ordered (origOp, cloneOp) pairs (C++ `cloneList`).
    clone_list: Vec<(OpId, OpId)>,
    /// Map from original op to its clone (C++ `origToClone`).
    orig_to_clone: std::collections::BTreeMap<OpId, OpId>,
}

impl CloneBlockOps {
    fn new() -> Self {
        CloneBlockOps { clone_list: Vec::new(), orig_to_clone: std::collections::BTreeMap::new() }
    }

    /// Make a basic clone of the p-code op copying its basic control-flow
    /// properties (C++ `CloneBlockOps::buildOpClone`, `funcdata_block.cc:970`).
    /// In the case of a \e branch, the op is not cloned and `None` is returned.
    fn build_op_clone(&mut self, data: &mut Funcdata, op: OpId) -> KunaResult<Option<OpId>> {
        let (is_branch, opc, num_input, addr, op_flags, op_addl) = {
            let o = data.obank().get(op).expect("buildOpClone: stale op");
            (o.is_branch(), o.code(), o.num_input(), o.get_addr().clone(), o.get_flags(), o.get_addlflags())
        };
        if is_branch {
            if opc != OpCode::CPUI_BRANCH {
                return Err(KunaError::lowlevel(
                    "Cannot duplicate 2-way or n-way branch in nodeplit",
                ));
            }
            return Ok(None);
        }
        let dup = data.new_op(num_input, addr);
        data.op_set_opcode_code(dup, opc);
        // Copy the flag subset that survives a clone (verbatim mask from
        // `buildOpClone`, funcdata_block.cc:982-985).
        use crate::op::pcodeop_flags as opf;
        let fl = op_flags
            & (opf::startbasic
                | opf::nocollapse
                | opf::startmark
                | opf::nonprinting
                | opf::halt
                | opf::badinstruction
                | opf::unimplemented
                | opf::noreturn
                | opf::missing
                | opf::indirect_creation
                | opf::indirect_store
                // C++ (funcdata_block.cc:985) lists `PcodeOp::no_indirect_collapse`
                // in this `op->flags & (...)` mask, but that constant (0x200) is
                // the *addlflags* enum value; masked against the `flags` word it
                // selects bit 0x200 == `fallthru_true`.  Transcribe the numeric
                // behaviour faithfully (matches the upstream byte output).
                | opf::fallthru_true
                | opf::calculated_bool
                | opf::ptrflow);
        // The addl-flag subset (funcdata_block.cc:987-988).
        use crate::op::pcodeop_addlflags as opaf;
        let afl = op_addl
            & (opaf::special_prop
                | opaf::special_print
                | opaf::incidental_copy
                | opaf::is_cpool_transformed
                | opaf::stop_type_propagation
                | opaf::store_unmapped);
        {
            let o = data.obank_mut().get_mut(dup).expect("buildOpClone: stale dup");
            o.set_flag(fl);
            o.set_additional_flag(afl);
        }
        self.clone_list.push((op, dup)); // Map from clone to orig
        self.orig_to_clone.insert(op, dup); // Map from orig to clone
        Ok(Some(dup))
    }

    /// Make a basic clone of a Varnode and its flags, as the output of a cloned
    /// op (C++ `CloneBlockOps::buildVarnodeOutput`, `funcdata_block.cc:1000`).
    fn build_varnode_output(
        &self,
        data: &mut Funcdata,
        orig_op: OpId,
        clone_op: OpId,
    ) -> KunaResult<()> {
        let opvn = match data.obank().get(orig_op).expect("buildVarnodeOutput: stale op").get_out()
        {
            Some(v) => v,
            None => return Ok(()),
        };
        let (size, addr, src_flags, src_addl) = {
            let v = data.vbank().get(opvn).expect("buildVarnodeOutput: stale vn");
            (v.get_size(), v.get_addr().clone(), v.get_flags(), v.get_addlflags())
        };
        let newvn = data.new_varnode_out(size, &addr, clone_op)?;
        data.vbank_mut()
            .get_mut(newvn)
            .expect("buildVarnodeOutput: stale newvn")
            .copy_clone_flags(src_flags, src_addl);
        Ok(())
    }

    /// P-code in a basic block is cloned into the split version of the block
    /// (C++ `CloneBlockOps::cloneBlock`, `funcdata_block.cc:1023`).
    fn clone_block(
        &mut self,
        data: &mut Funcdata,
        b: BlockId,
        bprime: BlockId,
        inedge: int4,
    ) -> KunaResult<()> {
        // Collect b's ops in order (the C++ `beginOp()..endOp()` block walk).
        let mut ops: Vec<OpId> = Vec::new();
        let mut cur = data.bb_op_head(b);
        while let Some(o) = cur {
            ops.push(o);
            cur = data.obank().get(o).expect("cloneBlock: stale op").basic_neighbours().1;
        }
        for orig_op in ops {
            let clone_op = match self.build_op_clone(data, orig_op)? {
                Some(c) => c,
                None => continue,
            };
            self.build_varnode_output(data, orig_op, clone_op)?;
            data.op_insert_end(clone_op, bprime);
        }
        self.patch_inputs(data, inedge)
    }

    /// Clone the p-code ops in an expression right before the given `follow_op`,
    /// returning the output Varnode of the last cloned op (C++
    /// `CloneBlockOps::cloneExpression`, `funcdata_block.cc:1043`).
    fn clone_expression(
        &mut self,
        data: &mut Funcdata,
        ops: &[OpId],
        follow_op: OpId,
    ) -> KunaResult<VarnodeId> {
        for &orig_op in ops {
            let clone_op = match self.build_op_clone(data, orig_op)? {
                Some(c) => c,
                None => continue,
            };
            self.build_varnode_output(data, orig_op, clone_op)?;
            data.op_insert_before(clone_op, follow_op);
        }
        if self.clone_list.is_empty() {
            return Err(KunaError::lowlevel("No expression to clone"));
        }
        self.patch_inputs(data, 0)?;
        let last_clone = self.clone_list.last().expect("cloneExpression: non-empty cloneList").1;
        Ok(data
            .obank()
            .get(last_clone)
            .expect("cloneExpression: stale last clone")
            .get_out()
            .expect("cloneExpression: last clone has output"))
    }

    /// Map original-op input Varnodes to the input slots of the cloned ops
    /// (C++ `CloneBlockOps::patchInputs`, `funcdata_block.cc:1066`).
    fn patch_inputs(&self, data: &mut Funcdata, inedge: int4) -> KunaResult<()> {
        for pos in 0..self.clone_list.len() {
            let (orig_op, clone_op) = self.clone_list[pos];
            let orig_code = data.obank().get(orig_op).expect("patchInputs: stale orig").code();
            if orig_code == OpCode::CPUI_MULTIEQUAL {
                // One edge now goes into the new block.
                data.obank_mut().get_mut(clone_op).expect("patchInputs: stale clone").set_num_inputs(1);
                data.op_set_opcode_code(clone_op, OpCode::CPUI_COPY);
                let in_vn =
                    data.obank().get(orig_op).expect("patchInputs").get_in(inedge).expect("MULTIEQUAL in");
                data.op_set_input(clone_op, in_vn, 0)?;
                // One edge is removed from the original block.
                data.op_remove_input(orig_op, inedge);
                if data.obank().get(orig_op).expect("patchInputs").num_input() == 1 {
                    data.op_set_opcode_code(orig_op, OpCode::CPUI_COPY);
                }
            } else if orig_code == OpCode::CPUI_INDIRECT {
                return Err(KunaError::lowlevel("Can't clone INDIRECTs"));
            } else if data.obank().get(orig_op).expect("patchInputs").is_call() {
                return Err(KunaError::lowlevel("Can't clone CALLs"));
            } else {
                let n_in = data.obank().get(clone_op).expect("patchInputs").num_input();
                for i in 0..n_in {
                    let orig_vn = data
                        .obank()
                        .get(orig_op)
                        .expect("patchInputs")
                        .get_in(i)
                        .expect("patchInputs: input slot");
                    let (is_const, is_annot, is_free, is_written, vdef, vaddr) = {
                        let v = data.vbank().get(orig_vn).expect("patchInputs: stale in vn");
                        (
                            v.is_constant(),
                            v.is_annotation(),
                            v.is_free(),
                            v.is_written(),
                            v.get_def(),
                            v.get_addr().clone(),
                        )
                    };
                    let clone_vn = if is_const {
                        orig_vn
                    } else if is_annot {
                        data.new_code_ref(&vaddr)
                    } else if is_free {
                        return Err(KunaError::lowlevel("Can't clone free varnode"));
                    } else if is_written {
                        match vdef.and_then(|d| self.orig_to_clone.get(&d).copied()) {
                            Some(clone_def) => data
                                .obank()
                                .get(clone_def)
                                .expect("patchInputs: stale clone def")
                                .get_out()
                                .expect("cloned def has output"),
                            None => orig_vn,
                        }
                    } else {
                        orig_vn
                    };
                    data.op_set_input(clone_op, clone_vn, i)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_num::opcodes::OpCode;

    use crate::funcdata::Funcdata;
    use crate::op::pcodeop_flags;
    use crate::context::{ArchContext, BlockId, OpId, TypeOp};

    fn build_manager() -> AddrSpaceManager {
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
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    /// The `ram` space as a clonable handle (captured once per test so it does
    /// not re-borrow `fd` while building ops).
    fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn addr(rs: &Rc<AddrSpace>, off: u64) -> Address {
        Address::new(Rc::clone(rs), off)
    }

    /// Append a terminating op of the given opcode (with the matching cached
    /// flags) at address `pc` into basic block `bl`; returns the op.
    fn term_op(fd: &mut Funcdata, bl: BlockId, opc: OpCode, flags: u32, pc: Address) -> OpId {
        let op = fd.obank_mut().create_at(1, pc);
        fd.obank_mut().change_opcode(op, TypeOp::new(opc, flags, format!("{opc:?}")));
        // A block op is alive in the real pipeline (create_at starts it dead, the
        // flow lift `mark_alive`s on integration); mark it so `op_destroy`'s
        // `mark_dead` can move it off the alive list.
        fd.obank_mut().mark_alive(op);
        fd.bb_insert_op(op, bl, None);
        op
    }

    #[test]
    fn structure_reset_diamond_dominators_and_reachability() {
        // entry -> {a, b} -> join   (a reducible diamond)
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let entry = fd.bblocks_mut().new_block_basic(root);
        let a = fd.bblocks_mut().new_block_basic(root);
        let b = fd.bblocks_mut().new_block_basic(root);
        let join = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(entry, a);
        fd.bblocks_mut().add_edge(entry, b);
        fd.bblocks_mut().add_edge(a, join);
        fd.bblocks_mut().add_edge(b, join);
        fd.bblocks_mut().set_start_block(root, entry);

        fd.structure_reset();

        // All four blocks are reachable -> single root -> no unreachable flag.
        assert!(!fd.has_unreachable_blocks());
        // join's immediate dominator is the entry (the diamond join point).
        let jdom = fd.bblocks_ref().block(join).get_immed_dom();
        assert_eq!(jdom, Some(entry));
        // The structured hierarchy was reset to empty.
        assert_eq!(fd.sblocks_get_size(), 0);
    }

    #[test]
    fn structure_reset_flags_unreachable_block() {
        // entry -> a ;  orphan with no in-edge.
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let entry = fd.bblocks_mut().new_block_basic(root);
        let a = fd.bblocks_mut().new_block_basic(root);
        let _orphan = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(entry, a);
        fd.bblocks_mut().set_start_block(root, entry);

        fd.structure_reset();
        // The orphan is its own root -> rootlist > 1 -> blocks_unreachable set.
        assert!(fd.has_unreachable_blocks());
    }

    #[test]
    fn set_basic_block_range_sets_cover() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        let beg = addr(&rs, 0x1000);
        let end = addr(&rs, 0x100f);
        fd.set_basic_block_range(bl, &beg, &end);
        // The block now covers [0x1000, 0x100f].
        let bb = fd.bblocks_ref().block(bl);
        if let crate::block::BlockKind::Basic(b) = bb.kind() {
            assert_eq!(b.cover.num_ranges(), 1);
            let r = b.cover.get_first_range().unwrap();
            assert_eq!(r.get_first(), 0x1000);
            assert_eq!(r.get_last(), 0x100f);
        } else {
            panic!("not basic");
        }
    }

    #[test]
    fn force_goto_labels_edge() {
        // bl0 (CBRANCH at 0x1000) -> bl1 (BRANCH at 0x2000)
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl0 = fd.bblocks_mut().new_block_basic(root);
        let bl1 = fd.bblocks_mut().new_block_basic(root);
        let bl2 = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(bl0, bl1);
        fd.bblocks_mut().add_edge(bl0, bl2);
        // bl0's tail op at 0x1000, bl1's tail op at 0x2000.
        term_op(&mut fd, bl0, OpCode::CPUI_CBRANCH, pcodeop_flags::branch, addr(&rs, 0x1000));
        term_op(&mut fd, bl1, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x2000));
        term_op(&mut fd, bl2, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x3000));

        let pcop = addr(&rs, 0x1000);
        let pcdest = addr(&rs, 0x2000);
        let labeled = fd.force_goto(&pcop, &pcdest).unwrap();
        assert!(labeled);
        // The edge bl0 -> bl1 (out index 0) is now a goto branch.
        assert!(fd.bblocks_ref().block(bl0).is_goto_out(0));
        // A non-matching destination labels nothing.
        let nope = addr(&rs, 0x9999);
        assert!(!fd.force_goto(&pcop, &nope).unwrap());
    }

    #[test]
    fn switch_edge_reroutes_and_resets() {
        // in -> before ; reroute to after.
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let inb = fd.bblocks_mut().new_block_basic(root);
        let before = fd.bblocks_mut().new_block_basic(root);
        let after = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(inb, before);
        fd.bblocks_mut().set_start_block(root, inb);
        fd.switch_edge(inb, before, after);
        // inb now flows to `after`, not `before`.
        assert_eq!(fd.bblocks_ref().block(inb).get_out(0), after);
        assert_eq!(fd.bblocks_ref().block(inb).size_out(), 1);
    }

    #[test]
    fn node_split_block_edge_duplicates_outs() {
        // a -> b -> c ;  split b on its single in-edge from a.
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let a = fd.bblocks_mut().new_block_basic(root);
        let b = fd.bblocks_mut().new_block_basic(root);
        let c = fd.bblocks_mut().new_block_basic(root);
        // Two in-edges into b so the in/out structure is non-trivial.
        let a2 = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(a, b);
        fd.bblocks_mut().add_edge(a2, b);
        fd.bblocks_mut().add_edge(b, c);
        // Give b an address cover so copyRange has something to copy.
        let rs = ramspace(&fd);
        fd.set_basic_block_range(b, &addr(&rs, 0x2000), &addr(&rs, 0x2010));

        let inedge = 0; // the edge from `a`
        let bprime = fd.node_split_block_edge(b, inedge);
        // bprime inherits b's out edges (to c) and the moved in-edge from a.
        assert_eq!(fd.bblocks_ref().block(bprime).get_out(0), c);
        // a now flows to bprime, not b.
        assert_eq!(fd.bblocks_ref().block(a).get_out(0), bprime);
        // b keeps the a2 in-edge.
        assert_eq!(fd.bblocks_ref().block(b).size_in(), 1);
        // bprime copied b's address cover.
        if let crate::block::BlockKind::Basic(bd) = fd.bblocks_ref().block(bprime).kind() {
            assert_eq!(bd.cover.num_ranges(), 1);
        } else {
            panic!("bprime not basic");
        }
    }

    #[test]
    fn splice_block_basic_destroys_trailing_branch() {
        // bl (with trailing BRANCH) -> outbl : splice now DESTROYS the trailing
        // branch op (C++ funcdata_block.cc:941) — the W3-op boundary is closed.
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let rs = ramspace(&fd);
        let bl = fd.bblocks_mut().new_block_basic(root);
        let outbl = fd.bblocks_mut().new_block_basic(root);
        let succ = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(bl, outbl);
        fd.bblocks_mut().add_edge(outbl, succ);
        fd.set_basic_block_range(bl, &addr(&rs, 0x1000), &addr(&rs, 0x1004));
        fd.set_basic_block_range(outbl, &addr(&rs, 0x1008), &addr(&rs, 0x100c));
        let br = term_op(&mut fd, bl, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x1000));
        let op_out = term_op(&mut fd, outbl, OpCode::CPUI_COPY, 0, addr(&rs, 0x1008));

        fd.splice_block_basic(bl).unwrap();
        // The trailing BRANCH was destroyed; bl now holds only outbl's COPY.
        assert!(fd.obank().get(br).expect("br slot kept").is_dead());
        assert_eq!(fd.bb_ops(bl), vec![op_out]);
        // bl inherited outbl's out edge to succ.
        assert_eq!(fd.bblocks_ref().block(bl).get_out(0), succ);
    }

    #[test]
    fn splice_block_basic_rejects_invalid_chain() {
        // bl with no single-in/single-out successor -> "Cannot splice basic blocks".
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        let o1 = fd.bblocks_mut().new_block_basic(root);
        let o2 = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(bl, o1);
        fd.bblocks_mut().add_edge(bl, o2); // two outs -> invalid
        let err = fd.splice_block_basic(bl).unwrap_err();
        assert!(err.to_string().contains("Cannot splice"));
    }

    #[test]
    fn splice_block_basic_merges_op_lists() {
        // bl (one plain op) -> outbl (one plain op).  No branch, no MULTIEQUAL:
        // the op lists merge, cover merges, blocks splice.
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        let outbl = fd.bblocks_mut().new_block_basic(root);
        let succ = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(bl, outbl);
        fd.bblocks_mut().add_edge(outbl, succ);
        fd.set_basic_block_range(bl, &addr(&rs, 0x1000), &addr(&rs, 0x1004));
        fd.set_basic_block_range(outbl, &addr(&rs, 0x1008), &addr(&rs, 0x100c));
        let op_bl = term_op(&mut fd, bl, OpCode::CPUI_COPY, 0, addr(&rs, 0x1000));
        let op_out = term_op(&mut fd, outbl, OpCode::CPUI_COPY, 0, addr(&rs, 0x1008));

        fd.splice_block_basic(bl).unwrap();
        // bl now holds both ops in order; outbl is gone.
        assert_eq!(fd.bb_ops(bl), vec![op_bl, op_out]);
        // bl inherited outbl's out edge to succ.
        assert_eq!(fd.bblocks_ref().block(bl).get_out(0), succ);
        // bl's cover now spans both ranges.
        if let crate::block::BlockKind::Basic(bd) = fd.bblocks_ref().block(bl).kind() {
            assert_eq!(bd.cover.num_ranges(), 2);
        } else {
            panic!("bl not basic");
        }
    }

    // -----------------------------------------------------------------------
    // do-nothing predicates + removeDoNothingBlock + isComplex
    // -----------------------------------------------------------------------

    #[test]
    fn has_only_markers_and_is_do_nothing() {
        // A block with only a MULTIEQUAL marker + a BRANCH, one in, one out, is a
        // do-nothing block; adding a real COPY makes it not.
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let pred = fd.bblocks_mut().new_block_basic(root);
        let bb = fd.bblocks_mut().new_block_basic(root);
        let succ = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, bb);
        fd.bblocks_mut().add_edge(bb, succ);

        // MULTIEQUAL marker (flags=marker) + BRANCH (flags=branch).
        let _mq = term_op(&mut fd, bb, OpCode::CPUI_MULTIEQUAL, pcodeop_flags::marker, addr(&rs, 0x2000));
        let _br = term_op(&mut fd, bb, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x2004));
        assert!(fd.bb_has_only_markers(bb));
        assert!(fd.bb_is_do_nothing(bb));
        assert!(!fd.bb_is_complex(bb)); // marker + single branch -> statement <= 1

        // Insert a real calculation op -> no longer marker-only / do-nothing.
        let _copy = term_op(&mut fd, bb, OpCode::CPUI_COPY, 0, addr(&rs, 0x2008));
        assert!(!fd.bb_has_only_markers(bb));
        assert!(!fd.bb_is_do_nothing(bb));
    }

    #[test]
    fn is_do_nothing_rejects_zero_in_and_multi_out() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        // Starting block (no in-edges) is never do-nothing (global placeholder).
        let start = fd.bblocks_mut().new_block_basic(root);
        let s_out = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(start, s_out);
        let _br = term_op(&mut fd, start, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x3000));
        assert!(!fd.bb_is_do_nothing(start)); // sizeIn() == 0

        // A two-out block (CBRANCH) is never do-nothing (sizeOut != 1).
        let pred = fd.bblocks_mut().new_block_basic(root);
        let twoout = fd.bblocks_mut().new_block_basic(root);
        let o1 = fd.bblocks_mut().new_block_basic(root);
        let o2 = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, twoout);
        fd.bblocks_mut().add_edge(twoout, o1);
        fd.bblocks_mut().add_edge(twoout, o2);
        assert!(!fd.bb_is_do_nothing(twoout)); // sizeOut() == 2
    }

    #[test]
    fn remove_do_nothing_block_splices_empty_marker_block() {
        // pred -> bb(empty: MULTIEQUAL marker + BRANCH) -> succ.  removeDoNothing
        // deletes bb, reconnecting pred directly to succ, and forces a structure
        // reset (sblocks cleared).
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let pred = fd.bblocks_mut().new_block_basic(root);
        let bb = fd.bblocks_mut().new_block_basic(root);
        let succ = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, bb);
        fd.bblocks_mut().add_edge(bb, succ);
        // A bare BRANCH (no MULTIEQUAL: no data flow to patch) keeps this test
        // focused on the control-flow splice.
        let _br = term_op(&mut fd, bb, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x4000));
        assert!(fd.bb_is_do_nothing(bb));
        assert!(fd.bb_unblocked_multi(bb, 0));

        let n_before = fd.bblocks_get_size();
        fd.remove_do_nothing_block(bb).unwrap();
        // bb is gone; pred now flows straight to succ.
        assert_eq!(fd.bblocks_get_size(), n_before - 1);
        assert_eq!(fd.bblocks_ref().block(pred).get_out(0), succ);
        assert_eq!(fd.bblocks_ref().block(succ).get_in(0), pred);
        // sblocks was cleared by structure_reset.
        assert_eq!(fd.sblocks_get_size(), 0);
    }

    #[test]
    fn remove_do_nothing_block_rejects_multi_out() {
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let pred = fd.bblocks_mut().new_block_basic(root);
        let bb = fd.bblocks_mut().new_block_basic(root);
        let o1 = fd.bblocks_mut().new_block_basic(root);
        let o2 = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, bb);
        fd.bblocks_mut().add_edge(bb, o1);
        fd.bblocks_mut().add_edge(bb, o2);
        let err = fd.remove_do_nothing_block(bb).unwrap_err();
        assert!(err.to_string().contains("Cannot delete a reachable block"));
    }

    // =======================================================================
    // VERIFIER adversarial tests (rport/w10-emptyblock-orform).
    //
    // Target the hunt-list spots most fragile for this item: the
    // `BlockBasic::isComplex` statement-count BOUNDARY (`statement > 2`, the
    // off-by-one that gates ruleBlockOr/whileDo), the `unblockedMulti`
    // redundant-branch dereference, and the anti-mis-fold guard (a *complex*
    // orblock must NOT be absorbed, i.e. `is_complex` genuinely rejects).
    // =======================================================================

    /// Append a COPY op at `pc` into `bb` whose output is a fresh, no-descend
    /// `ram` Varnode at `out_off`.  In `BlockBasic::isComplex` such a calculation
    /// (output with `hasNoDescend()`) counts as one statement.
    fn copy_with_dead_out(
        fd: &mut Funcdata,
        bb: BlockId,
        pc: Address,
        out_off: u64,
    ) -> OpId {
        let rs = ramspace(fd);
        let op = fd.new_op(1, pc);
        fd.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        let outvn = fd.new_varnode(4, &addr(&rs, out_off), None);
        fd.op_set_output(op, outvn).unwrap();
        fd.obank_mut().mark_alive(op);
        fd.bb_insert_op(op, bb, None);
        op
    }

    /// VF1 — `bb_is_complex` boundary: with `max_implied_ref == 2` the block is
    /// complex strictly once the statement count EXCEEDS 2.  A single-out block
    /// (statement starts at 0) with two dead-output COPYs counts 2 -> NOT
    /// complex; a third pushes it to 3 (> 2) -> complex.  This pins the C++
    /// `if (statement > 2) return true;` off-by-one (block.cc:2456).
    #[test]
    fn vf1_bb_is_complex_statement_count_boundary() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let pred = fd.bblocks_mut().new_block_basic(root);
        let bb = fd.bblocks_mut().new_block_basic(root);
        let succ = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, bb);
        fd.bblocks_mut().add_edge(bb, succ); // single out -> statement base 0

        // Two calculations (statement == 2): NOT complex (2 is not > 2).
        let _c1 = copy_with_dead_out(&mut fd, bb, addr(&rs, 0x5000), 0x90);
        let _c2 = copy_with_dead_out(&mut fd, bb, addr(&rs, 0x5004), 0x98);
        assert!(!fd.bb_is_complex(bb), "two statements must NOT be complex (2 !> 2)");

        // A third calculation (statement == 3): complex (3 > 2).
        let _c3 = copy_with_dead_out(&mut fd, bb, addr(&rs, 0x5008), 0xa0);
        assert!(fd.bb_is_complex(bb), "three statements MUST be complex (3 > 2)");
    }

    /// VF2 — `bb_is_complex` two-out branch counts as the first statement.  A
    /// CBRANCH block (`sizeOut() >= 2`) starts at statement == 1, so only TWO
    /// dead-output calculations are needed to cross the > 2 threshold (1 + 2 ==
    /// 3).  This pins the C++ `if (sizeOut()>=2) statement = 1;` seed
    /// (block.cc:2414) AND that a marker op (MULTIEQUAL) is skipped, not counted.
    #[test]
    fn vf2_bb_is_complex_branch_seed_and_marker_skip() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let pred = fd.bblocks_mut().new_block_basic(root);
        let bb = fd.bblocks_mut().new_block_basic(root);
        let o1 = fd.bblocks_mut().new_block_basic(root);
        let o2 = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, bb);
        fd.bblocks_mut().add_edge(bb, o1);
        fd.bblocks_mut().add_edge(bb, o2); // two out -> statement seeded to 1

        // A MULTIEQUAL marker must be SKIPPED (not counted) — add one to prove it.
        let _mq = term_op(&mut fd, bb, OpCode::CPUI_MULTIEQUAL, pcodeop_flags::marker, addr(&rs, 0x6000));
        // One calculation: statement == 1 (seed) + 1 == 2 -> NOT complex.
        let _c1 = copy_with_dead_out(&mut fd, bb, addr(&rs, 0x6004), 0xb0);
        assert!(!fd.bb_is_complex(bb), "branch-seed 1 + 1 calc == 2, marker skipped, NOT complex");

        // Second calculation: 1 + 2 == 3 -> complex.  (If the marker had been
        // miscounted, the block would already be complex above.)
        let _c2 = copy_with_dead_out(&mut fd, bb, addr(&rs, 0x6008), 0xb8);
        assert!(fd.bb_is_complex(bb), "branch-seed 1 + 2 calcs == 3 -> complex");
    }

    /// VF3 — `bb_unblocked_multi` rejects a redundant branch carrying a DIFFERENT
    /// value through a MULTIEQUAL in the out block.  Two predecessors of `bb`
    /// also branch directly to `bb`'s out block `blout`, which has a MULTIEQUAL.
    /// When the redundant slot's value differs from `bb`'s slot value, removing
    /// `bb` would hide an inconsistent implied copy -> `unblockedMulti` is false.
    /// Pins the C++ `if (vnremove != vnredund) return false;` (block.cc:2595).
    #[test]
    fn vf3_bb_unblocked_multi_rejects_inconsistent_redundant_branch() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        // CFG: pred -> bb -> blout ; pred ALSO -> blout (the redundant branch).
        let pred = fd.bblocks_mut().new_block_basic(root);
        let bb = fd.bblocks_mut().new_block_basic(root);
        let blout = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(pred, bb);
        fd.bblocks_mut().add_edge(bb, blout);
        fd.bblocks_mut().add_edge(pred, blout); // redundant edge pred->blout

        // blout's in-edges: [bb, pred] (insertion order).  Build a 2-input
        // MULTIEQUAL there whose slot for `bb` and slot for `pred` differ.
        let in_bb = fd.bblocks_ref().block(blout).get_in_index(bb);
        let in_pred = fd.bblocks_ref().block(blout).get_in_index(pred);
        // The MULTIEQUAL slot values must be WRITTEN (a free varnode may hold only
        // one descendant — the addDescend invariant), so define each by a COPY in
        // `pred`; that lets one value feed two slots in the "identical" half.
        let c_bb = copy_with_dead_out(&mut fd, pred, addr(&rs, 0x6f00), 0xc0);
        let v_bb = fd.obank().get(c_bb).unwrap().get_out().unwrap();
        let c_pred = copy_with_dead_out(&mut fd, pred, addr(&rs, 0x6f04), 0xc8);
        let v_pred = fd.obank().get(c_pred).unwrap().get_out().unwrap(); // DIFFERENT value
        let mq = fd.new_op(2, addr(&rs, 0x7000));
        fd.op_set_opcode(mq, crate::typeop::type_op_for(OpCode::CPUI_MULTIEQUAL));
        let mq_out = fd.new_varnode(4, &addr(&rs, 0xd0), None);
        fd.op_set_output(mq, mq_out).unwrap();
        fd.op_set_input(mq, v_bb, in_bb).unwrap();
        fd.op_set_input(mq, v_pred, in_pred).unwrap();
        fd.obank_mut().mark_alive(mq);
        fd.bb_insert_op(mq, blout, None);

        // The redundant branch (pred->blout) carries v_pred; bb's slot carries
        // v_bb.  They differ -> removing bb is NOT unblocked.
        assert!(
            !fd.bb_unblocked_multi(bb, 0),
            "inconsistent redundant MULTIEQUAL value must block removal"
        );

        // Make the two slots IDENTICAL: now the redundant branch is harmless.
        fd.op_set_input(mq, v_bb, in_pred).unwrap();
        assert!(
            fd.bb_unblocked_multi(bb, 0),
            "identical redundant MULTIEQUAL value -> unblocked"
        );
    }

    // =======================================================================
    // VERIFIER adversarial tests (rport/w10-returnsplit-nodesplit): Funcdata::
    // nodeSplit (return-block duplication per in-edge) + CloneBlockOps.
    // =======================================================================

    /// NS1 — `node_split` duplicates a multi-in-edge RETURN block per edge,
    /// cloning its ops and pulling the split edge's MULTIEQUAL input out into a
    /// COPY on the duplicate (C++ `Funcdata::nodeSplit` + `CloneBlockOps`).
    /// Two predecessors flow into a return block whose RETURN reads a MULTIEQUAL;
    /// splitting edge 0 moves predecessor `p0` onto a fresh duplicate carrying its
    /// own (COPY-fed) RETURN, while the original keeps edge 1.
    #[test]
    fn ns1_node_split_return_block_clones_per_in_edge() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let p0 = fd.bblocks_mut().new_block_basic(root);
        let p1 = fd.bblocks_mut().new_block_basic(root);
        let ret = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(p0, ret);
        fd.bblocks_mut().add_edge(p1, ret);
        fd.set_basic_block_range(ret, &addr(&rs, 0x8000), &addr(&rs, 0x8010));

        // A value feeding each MULTIEQUAL slot (written in the predecessors).
        let c0 = copy_with_dead_out(&mut fd, p0, addr(&rs, 0x7f00), 0x40);
        let v0 = fd.obank().get(c0).unwrap().get_out().unwrap();
        let c1 = copy_with_dead_out(&mut fd, p1, addr(&rs, 0x7f04), 0x48);
        let v1 = fd.obank().get(c1).unwrap().get_out().unwrap();

        // MULTIEQUAL in `ret`: slot 0 <- v0 (from p0), slot 1 <- v1 (from p1).
        let in0 = fd.bblocks_ref().block(ret).get_in_index(p0);
        let in1 = fd.bblocks_ref().block(ret).get_in_index(p1);
        let mq = fd.new_op(2, addr(&rs, 0x8000));
        fd.op_set_opcode(mq, crate::typeop::type_op_for(OpCode::CPUI_MULTIEQUAL));
        let mq_out = fd.new_varnode(4, &addr(&rs, 0x50), None);
        fd.op_set_output(mq, mq_out).unwrap();
        fd.op_set_input(mq, v0, in0).unwrap();
        fd.op_set_input(mq, v1, in1).unwrap();
        fd.obank_mut().mark_alive(mq);
        fd.bb_insert_op(mq, ret, None);

        // RETURN reads the MULTIEQUAL output (and a code-ref like a real return).
        let retop = fd.new_op(1, addr(&rs, 0x8008));
        fd.op_set_opcode(retop, crate::typeop::type_op_for(OpCode::CPUI_RETURN));
        fd.op_set_input(retop, mq_out, 0).unwrap();
        fd.obank_mut().mark_alive(retop);
        fd.bb_insert_op(retop, ret, None);

        let n_before = fd.bblocks_get_size();
        // Split the edge from p0 (inedge index in0).
        fd.node_split(ret, in0).expect("node_split");
        // A duplicate block was created.
        assert_eq!(fd.bblocks_get_size(), n_before + 1, "nodeSplit adds the duplicate block");
        // p0 now flows to the duplicate, not the original ret.
        let p0_out = fd.bblocks_ref().block(p0).get_out(0);
        assert_ne!(p0_out, ret, "p0 re-routed onto the duplicate");
        // The original ret keeps exactly the p1 in-edge.
        assert_eq!(fd.bblocks_ref().block(ret).size_in(), 1);
        assert_eq!(fd.bblocks_ref().block(ret).get_in(0), p1);
        // The duplicate has p0 as its sole in-edge and carries a cloned RETURN.
        assert_eq!(fd.bblocks_ref().block(p0_out).size_in(), 1);
        assert_eq!(fd.bblocks_ref().block(p0_out).get_in(0), p0);
        let dup_ret = fd.bb_op_tail(p0_out).expect("duplicate has ops");
        assert_eq!(
            fd.obank().get(dup_ret).unwrap().code(),
            OpCode::CPUI_RETURN,
            "the cloned tail is a RETURN"
        );
        // The original 2-input MULTIEQUAL had edge in0 removed -> 1 input -> COPY.
        assert_eq!(
            fd.obank().get(mq).unwrap().code(),
            OpCode::CPUI_COPY,
            "original MULTIEQUAL collapses to COPY after one edge is pulled out"
        );
    }

    /// NS2 — `node_split` rejects a block with out-flow and a block with a single
    /// in edge (C++ `nodeSplit` guards, funcdata_block.cc:867-870).  These are
    /// pure structural guards — no function-name/value special-casing.
    #[test]
    fn ns2_node_split_rejects_outflow_and_single_inedge() {
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        // Block with an out edge -> rejected.
        let p0 = fd.bblocks_mut().new_block_basic(root);
        let p1 = fd.bblocks_mut().new_block_basic(root);
        let b = fd.bblocks_mut().new_block_basic(root);
        let out = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(p0, b);
        fd.bblocks_mut().add_edge(p1, b);
        fd.bblocks_mut().add_edge(b, out); // out flow
        let err = fd.node_split(b, 0).unwrap_err();
        assert!(err.to_string().contains("out flow"), "out-flow block rejected");

        // Block with a single in edge -> rejected.
        let q0 = fd.bblocks_mut().new_block_basic(root);
        let single = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(q0, single);
        let err2 = fd.node_split(single, 0).unwrap_err();
        assert!(err2.to_string().contains("only 1 in edge"), "single-in-edge block rejected");
    }

    /// NS3 — `CloneBlockOps` clones a generic calculation op verbatim (opcode,
    /// inputs, constant sharing) — exercises the non-MULTIEQUAL `patch_inputs`
    /// branch with no opcode/value special-casing.  An INT_ADD reading a written
    /// value and a constant is duplicated into the split block with the same
    /// opcode and the same constant input.
    #[test]
    fn ns3_clone_block_duplicates_generic_op_and_shares_constants() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let p0 = fd.bblocks_mut().new_block_basic(root);
        let p1 = fd.bblocks_mut().new_block_basic(root);
        let ret = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(p0, ret);
        fd.bblocks_mut().add_edge(p1, ret);
        fd.set_basic_block_range(ret, &addr(&rs, 0x9000), &addr(&rs, 0x9010));

        // MULTIEQUAL output `m`, then an INT_ADD: t = m + 7 (constant), then a
        // RETURN reading t.  The INT_ADD is the generic op to clone.
        let c0 = copy_with_dead_out(&mut fd, p0, addr(&rs, 0x8f00), 0x60);
        let v0 = fd.obank().get(c0).unwrap().get_out().unwrap();
        let c1 = copy_with_dead_out(&mut fd, p1, addr(&rs, 0x8f04), 0x68);
        let v1 = fd.obank().get(c1).unwrap().get_out().unwrap();
        let in0 = fd.bblocks_ref().block(ret).get_in_index(p0);
        let in1 = fd.bblocks_ref().block(ret).get_in_index(p1);
        let mq = fd.new_op(2, addr(&rs, 0x9000));
        fd.op_set_opcode(mq, crate::typeop::type_op_for(OpCode::CPUI_MULTIEQUAL));
        let m = fd.new_varnode(4, &addr(&rs, 0x70), None);
        fd.op_set_output(mq, m).unwrap();
        fd.op_set_input(mq, v0, in0).unwrap();
        fd.op_set_input(mq, v1, in1).unwrap();
        fd.obank_mut().mark_alive(mq);
        fd.bb_insert_op(mq, ret, None);

        let add = fd.new_op(2, addr(&rs, 0x9004));
        fd.op_set_opcode(add, crate::typeop::type_op_for(OpCode::CPUI_INT_ADD));
        let t = fd.new_varnode(4, &addr(&rs, 0x78), None);
        fd.op_set_output(add, t).unwrap();
        let k7 = fd.new_constant(4, 7);
        fd.op_set_input(add, m, 0).unwrap();
        fd.op_set_input(add, k7, 1).unwrap();
        fd.obank_mut().mark_alive(add);
        fd.bb_insert_op(add, ret, None);

        let retop = fd.new_op(1, addr(&rs, 0x9008));
        fd.op_set_opcode(retop, crate::typeop::type_op_for(OpCode::CPUI_RETURN));
        fd.op_set_input(retop, t, 0).unwrap();
        fd.obank_mut().mark_alive(retop);
        fd.bb_insert_op(retop, ret, None);

        fd.node_split(ret, in0).expect("node_split");
        let dup = fd.bblocks_ref().block(p0).get_out(0);
        // The duplicate carries a cloned INT_ADD whose constant input is shared.
        let mut found_add = false;
        let mut op = fd.bb_op_head(dup);
        while let Some(o) = op {
            let oref = fd.obank().get(o).unwrap();
            if oref.code() == OpCode::CPUI_INT_ADD {
                found_add = true;
                let kin = oref.get_in(1).expect("clone INT_ADD second input");
                let kv = fd.vbank().get(kin).expect("const in");
                assert!(kv.is_constant(), "constant input shared verbatim on the clone");
                assert_eq!(kv.get_offset(), 7, "constant value preserved (no special-casing)");
            }
            op = fd.obank().get(o).unwrap().basic_neighbours().1;
        }
        assert!(found_add, "the generic INT_ADD was cloned into the duplicate block");
    }
}
