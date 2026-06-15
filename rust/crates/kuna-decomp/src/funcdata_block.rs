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
use crate::seams::{BlockId, OpId, VarnodeId};

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
        // for(iter=op.begin();...) { if (isMarker()) continue;
        //   if (isBranch()) continue; return false; } return true;
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
        // if (sizeOut() != 1) return false;   // no return / cbranch
        if g.block(bb).size_out() != 1 {
            return false;
        }
        // if (sizeIn() == 0) return false;    // starting block placeholder
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
                // vnredund = multiop->getIn(blout->getInIndex(bl));
                let redund_slot = g.block(blout).get_in_index(bl);
                let vnredund = mop.get_in(redund_slot);
                // vnremove = multiop->getIn(inIndexToThis);
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
        // if (!hasImmedCopyEdge(outslot)) return true;
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
        // if (sizeOut() >= 2) statement = 1;  // the branch counts as a statement
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
        // inbl = parent->getIn(slot); outedge = parent->getInRevIndex(slot);
        let inbl = g.block(parent).get_in(slot);
        let outedge = g.block(parent).get_in_rev_index(slot);
        // return inbl->hasImmedCopyEdge(outedge);
        g.block(inbl).has_immed_copy_edge(outedge)
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
        // if (isHighOn()) { origvn->replaceInHigh(replacevn); replacevn->setExplicit(); }
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
        let mut set_high_log: Vec<(VarnodeId, crate::seams::HighVariableId, kuna_base::types::int2)> =
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
            // warningHeader("push_multiequal on block with multiple outputs"); --
            // a do-nothing block has exactly one out (isDoNothing guard), so this
            // never fires on the removeDoNothingBlock path.  The warning facility
            // is a W4 surface; the C++ continues past it (non-fatal).
        }
        // outblock = bb->getOut(0); outblock_ind = bb->getOutRevIndex(0);
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
        // BRANCHIND last op -> remove its jump table (W4: JumpTable registry not
        // ported; a do-nothing block never ends in BRANCHIND so this is unreached
        // from removeDoNothingBlock).
        if let Some(op) = self.bb_op_tail(bb) {
            if self.obank().get(op).expect("blockRemoveInternal: stale op").code()
                == OpCode::CPUI_BRANCHIND
            {
                // JumpTable *jt = findJumpTable(op); if (jt) removeJumpTable(jt); -- SEAM(W4)
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
                    // deadop = deadvn->getDef();
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
        for op in self.bb_ops(bb) {
            if self.obank().get(op).expect("blockRemoveInternal").is_assignment() {
                let deadvn = self
                    .obank()
                    .get(op)
                    .expect("blockRemoveInternal")
                    .get_out()
                    .expect("blockRemoveInternal: assignment with no out");
                if unreachable {
                    // descend2Undef + the one-time warning -- SEAM(W3-op): the
                    // descend-to-undefined rewrite is a funcdata_op surface not on
                    // the removeDoNothingBlock path (unreachable==false).  Reached
                    // only by removeUnreachableBlocks, which is itself unported.
                    return Err(KunaError::lowlevel(
                        "blockRemoveInternal(unreachable=true): descend2Undef SEAM",
                    ));
                }
                if self.descendants_outside(deadvn) {
                    return Err(KunaError::lowlevel("Deleting op with descendants"));
                }
            }
            if self.obank().get(op).expect("blockRemoveInternal").is_call() {
                // deleteCallSpecs(op) -- a do-nothing block (markers + branch only)
                // never contains a CALL, so this is unreachable on the
                // removeDoNothingBlock path.  The call-spec registry prune is a W4
                // surface; guard rather than silently drop.
                return Err(KunaError::lowlevel(
                    "blockRemoveInternal: CALL in removable block (deleteCallSpecs SEAM)",
                ));
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
            // ind = indop->getParent(); ind->setDefaultSwitch(default_block)
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
        // jt->setIndirectOp(op): the op's address is `addr` (same op-address).
        self.jumpvec_mut()[idx].set_indirect_op_addr(op, addr);
        Some(idx)
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
    /// SEAM(W4): the FuncCallSpecs cloning (`oldspec->clone(newop)` + the fspec
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

        // SEAM(W4): clone FuncCallSpecs (qlst) — the W4 call-spec surface.

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

    /// Test whether the given Varnode holds the function's return address (C++
    /// `Funcdata::testForReturnAddress`, `funcdata_varnode.cc:1463`).
    ///
    /// SEAM(W4): the comparison target is `glb->defaultReturnAddr`, which the W3
    /// seam [`Architecture`](crate::seams::Architecture) does not carry; the C++
    /// returns `false` whenever `defaultReturnAddr.space == 0` (no standard
    /// storage), so the W3 behavior is the same `false` — the BRANCHIND is treated
    /// as a genuine switch (the common case), not a tail-`ret`.  Recorded as a
    /// loss for the rare "indirect jump to the return address" pattern.
    pub fn test_for_return_address(&self, _vn: VarnodeId) -> bool {
        false
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
    ) -> Result<Option<usize>, crate::jumptable::RecoveryMode> {
        use crate::jumptable::RecoveryMode;
        // linkJumpTable: search for a pre-existing table.
        if let Some(idx) = self.link_jump_table(op) {
            let jt = self.get_jump_table(idx as int4);
            if !jt.is_override() && !jt.is_partial() && jt.num_entries() != 0 {
                return Ok(Some(idx)); // Previously calculated, complete, non-override
            }
            // Recover empty/override table.
            let mode = self.stage_jump_table(idx, op, record_loads, visited, run_pipeline);
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
        let mode = self.stage_jump_table(idx, op, record_loads, visited, run_pipeline);
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
    ) -> crate::jumptable::RecoveryMode {
        use crate::jumptable::RecoveryMode;
        self.jumpvec_mut()[jt_idx].increment_recovery_count();

        // Build the partial function (clone ops + jump-tables).
        let mut partial = match self.build_jumptable_partial() {
            Ok(p) => p,
            Err(_) => return RecoveryMode::FailNormal,
        };
        // Mark the partial as a jump-table-recovery clone, then build its blocks
        // and run the reduced "jumptable" universalAction over it (C++
        // partial.truncatedFlow + allacts.setCurrent("jumptable") + perform).
        partial.set_flag_raw(crate::funcdata::funcdata_flags::jumptablerecovery_on);
        if run_pipeline(&mut partial, visited).is_err() {
            // C++ catches LowlevelError, warns, and returns fail_normal.
            return RecoveryMode::FailNormal;
        }

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
            jt.recover_multistage(&mut partial)
        } else {
            jt.recover_addresses(&mut partial)
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
        partial.truncated_flow_clone(self)?;
        Ok(partial)
    }

    /// Backtrack from a BRANCHIND looking for an un-injected CALLOTHER in the
    /// destination calculation (C++ `Funcdata::earlyJumpTableFail`,
    /// `funcdata_block.cc:568`).
    ///
    /// SEAM(W4): the CALLOTHER user-op-type classification
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
                        // SEAM(W4): userop-type classification (injected/jumpassist/
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

        // Remove any jump op at the end of -bl- (C++ funcdata_block.cc:941:
        // `if (jumpop->isBranch()) opDestroy(jumpop);`).  `op_destroy` is now
        // available (used by condexe.execute), so the W3-op seam is closed.
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
        // branch op (C++ funcdata_block.cc:941 `if (jumpop->isBranch())
        // opDestroy(jumpop)`) — the W3-op seam is closed.
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
}
