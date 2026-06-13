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
//!      in parallel with no seam rights.  The methods that are *primarily*
//!      data-flow patch-up (`pushMultiequals`, `opZeroMulti`,
//!      `branchRemoveInternal`, `blockRemoveInternal`, `removeBranch`,
//!      `removeDoNothingBlock`, `removeUnreachableBlocks`, `pushBranch`,
//!      `nodeSplit`, `descend2Undef`) are seam-noted (`// SEAM(W3-op)`) with an
//!      explicit `Err` and a precise note of the missing API; the funcdata_op
//!      wave fills the bodies once it owns `opDestroy` & friends.
//!
//!   3. **Need W4 subsystems** — the jump-table methods (`linkJumpTable`,
//!      `findJumpTable`, `installJumpTable`, `recoverJumpTable`,
//!      `stageJumpTable`, `earlyJumpTableFail`, `switchOverJumpTables`,
//!      `removeJumpTable`) need the W4 `JumpTable`/`FlowInfo`/`ActionDatabase`.
//!      They are seam-noted (`// SEAM(W4)`); `clearJumpTables` and the dead-table
//!      sweep that `structureReset` performs operate on the opaque
//!      [`JumpTableId`](crate::funcdata::JumpTableId) handles and are carried.
//!
//! `stageJumpTable`'s Action-machinery (running the "jumptable" action set on a
//! truncated partial function) is explicitly `// SEAM(W4)`: it drives
//! `glb->allacts` (the W4 `ActionDatabase`) and `truncatedFlow` (W4 flow), with
//! no W3 surface.

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::block::{block_flags, BlockKind};
use crate::funcdata::Funcdata;
use crate::seams::{BlockId, OpId};

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

    /// Clear any jump-table information, preserving overrides
    /// (C++ `Funcdata::clearJumpTables`, `funcdata_block.cc:42`).
    ///
    /// SEAM(W4): the C++ keeps override tables (`jt->isOverride()`) and frees the
    /// rest.  At W3 the table contents are opaque ([`JumpTableId`]); we drop the
    /// whole vector (no W3 way to know which are overrides).  W4 reinstates the
    /// override-preserving filter.
    pub fn clear_jump_tables(&mut self) {
        // for jt in jumpvec: if jt->isOverride() keep(clear) else delete  -- SEAM(W4)
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
    /// at; at W3 the table contents are seamed out, so the sweep is `// SEAM(W4)`
    /// and the vector is left intact (no table is dropped here).  `heritage.
    /// forceRestructure()` is `// SEAM(W7)`.
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

        // Check for dead jumptables.  -- SEAM(W4): indOp->isDead() unavailable.

        // sblocks.clear() -> force structuring to start over.
        self.clear_sblocks();
        // heritage.forceRestructure(): the CFG may have changed (e.g. a block
        // removed by ActionConditionalExe), invalidating the heritage engine's
        // cached augmented dominator tree.  Forcing a rebuild on the next
        // `heritage()` pass prevents `rename_recurse` from walking a stale
        // (removed) block handle.  (Previously a `// SEAM(W7)`; reached now that
        // ActionDeadCode + condexe actually mutate the CFG.)
        self.heritage_force_restructure();
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
            // opInsertInput(op,newVarnode(op->getOut()->getSize(),op->getOut()->getAddr()),0);
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
            // setInputVarnode(op->getIn(0));
            let in0 = self.obank().get(op).expect("opZeroMulti: stale op").get_in(0).expect("opZeroMulti: just-inserted in0");
            self.set_input_varnode(in0)?;
            // opSetOpcode(op,CPUI_COPY);
            self.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        } else if nin == 1 {
            self.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        }
        Ok(())
    }

    /// Remove an outgoing branch of the given basic block, patching MULTIEQUAL
    /// ops in the target block (C++ `Funcdata::branchRemoveInternal`,
    /// `funcdata_block.cc:213`).
    ///
    /// \param bb is the given basic block
    /// \param num is the index of the outgoing edge to remove
    pub fn branch_remove_internal(&mut self, bb: BlockId, num: int4) -> KunaResult<()> {
        // if (bb->sizeOut() == 2) opDestroy(bb->lastOp());  // no decision left
        if self.bblocks_ref().block(bb).size_out() == 2 {
            if let Some(lastop) = self.bb_op_tail(bb) {
                self.op_destroy(lastop);
            }
        }
        // bbout = (BlockBasic *) bb->getOut(num);
        let bbout = self.bblocks_ref().block(bb).get_out(num);
        // blocknum = bbout->getInIndex(bb);
        let blocknum = self.bblocks_ref().block(bbout).get_in_index(bb);
        // bblocks.removeEdge(bb,bbout);  // Sever (one) connection
        self.bblocks_mut().remove_edge(bb, bbout);
        // for(iter=bbout->beginOp();...) { if (op->code()!=MULTIEQUAL) break;
        //   opRemoveInput(op,blocknum); opZeroMulti(op); }
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

    /// The data-flow half of `BlockBasic::negateCondition` (C++ `block.cc:2355`):
    /// flip the trailing CBRANCH's `boolean_flip` and `fallthru_true` flags, then
    /// swap the (bblocks) block's out-edge order.
    ///
    /// Called by [`ActionBlockStructure`](crate::blockaction::ActionBlockStructure)
    /// to realize the negation requests the structured collapse recorded against
    /// the underlying `BlockBasic` (the dual-arena `BlockCopy::copy` target).
    pub fn block_basic_negate_lastop(&mut self, bb: BlockId) {
        // lastop->flipFlag(boolean_flip); lastop->flipFlag(fallthru_true);
        if let Some(lastop) = self.bb_op_tail(bb) {
            let o = self.obank_mut().get_mut(lastop).expect("negate_lastop: stale op");
            o.flip_flag(crate::op::pcodeop_flags::boolean_flip);
            o.flip_flag(crate::op::pcodeop_flags::fallthru_true);
        }
        // FlowBlock::negateCondition(true): swapEdges() — flip the out-edge order.
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
    /// SEAM(W4): the per-table indirect op + default block come from the W4
    /// `JumpTable` (`jt->getIndirectOp()`, `jt->getDefaultBlock()`).  With the
    /// table contents seamed out there is nothing to label at W3; the iteration
    /// shape is carried so W4 fills only the body.
    pub fn install_switch_defaults(&mut self) {
        for _i in 0..self.num_jump_tables() {
            // indop = jt->getIndirectOp(); ind = indop->getParent();
            // if (jt->getDefaultBlock() != -1) ind->setDefaultSwitch(...);  -- SEAM(W4)
        }
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

    // --- Block-cover helpers (BlockBasic::setInitialRange/copyRange/mergeRange)

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
    /// `// SEAM(W3-op)`-noted below.  The MULTIEQUAL rejection is enforced (it is
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

        // Remove any jump op at the end of -bl-.
        if let Some(jumpop) = self.bb_op_tail(bl) {
            if self.obank().get(jumpop).expect("spliceBlockBasic").is_branch() {
                // opDestroy(jumpop);  -- SEAM(W3-op): destroy needs funcdata_op.
                return Err(KunaError::lowlevel(
                    "spliceBlockBasic: trailing branch destruction needs funcdata_op (SEAM W3-op)",
                ));
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
            // firstop->clearFlag(startbasic);
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

    /// Reset `sblocks` to a fresh empty graph (C++ `sblocks.clear()`).
    fn clear_sblocks(&mut self) {
        use crate::block::{BlockGraph, FlowBlock};
        let mut sb = BlockGraph::new();
        let sroot = sb.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
        sb.root = Some(sroot);
        *self.sblocks_mut() = sb;
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
    use crate::seams::{Architecture, BlockId, OpId, TypeOp};

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
        let glb = Rc::new(Architecture::new(manage));
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
    fn splice_block_basic_seams_branch_destruction() {
        // bl (with trailing BRANCH) -> outbl ; splice must Err (needs opDestroy).
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let rs = ramspace(&fd);
        let bl = fd.bblocks_mut().new_block_basic(root);
        let outbl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(bl, outbl);
        term_op(&mut fd, bl, OpCode::CPUI_BRANCH, pcodeop_flags::branch, addr(&rs, 0x1000));
        let err = fd.splice_block_basic(bl).unwrap_err();
        assert!(err.to_string().contains("funcdata_op"));
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
}
