//! Port of `decompiler/cpp/merge.{cc,hh}` (W7, item `w7-s6-merge`) — the
//! [`Merge`] class that forms [`HighVariable`](crate::variable::HighVariable)s out
//! of low-level Varnodes.
//!
//! As a node in SSA form a Varnode has at most one defining op; to get a notion of
//! a single high-level variable that may be reassigned at multiple sites, Varnode
//! objects are *merged* into a HighVariable.  Two Varnodes may merge only if their
//! [`Cover`]s do not pairwise intersect.  The driver runs *forced* merges (which
//! may insert COPYs/uniques to split problematic covers) and *speculative* merges
//! (abandoned on any cover intersection).
//!
//! ## What this module owns vs. what it drives
//!
//! `merge.cc` is almost entirely **glue** over `Funcdata` IR mutation
//! (`newOp`/`opSetInput`/`opInsertAfter`/`opDestroy`/`totalReplace`/…), `Varnode`
//! geometry (`copyShadow`/`characterizeOverlap`/`getCover`), the
//! [`HighVariableBank`] merge primitives, and the *intersection-cache driver*
//! (`HighIntersectTest::intersection`/`updateHigh`/`blockIntersection`).  The
//! intersection-cache driver lives in `variable.cc` upstream but is owned by the
//! `Merge::testCache` member, so it is transcribed **here** (the dependency item
//! `w7-s6-variable-cover` provided only the cache *primitives* —
//! `purgeHigh`/`moveIntersectTests`/`cached`/`cacheResult`).  See
//! [`MergeIntersect`].
//!
//! The IR/Varnode/Symbol operations the formation passes need cross the ADR-0001
//! arena boundary into `Funcdata`'s `obank`/`vbank`/`bblocks`/`localmap`.  Those
//! reach the algorithm through the [`MergeContext`] trait (the W7/W8 wave wires the
//! real `Funcdata` impl).  Where the underlying `Funcdata` mutator is itself not
//! yet ported (e.g. `opMarkNonPrinting`, `overlapLoc`, `findLinkedVarnodes`,
//! `inheritUnionField`, `copyShadow`, the callspec list), the trait method is a
//! seam recorded in `docs/rust-port/losses.md`; the *structure*, iteration order,
//! and tie-breakers of every `merge.cc` routine are transcribed verbatim so the
//! body fills in deterministically once the seam is supplied.
//!
//! ## Order / tie-break fidelity (output-determining)
//!
//!   - [`compare_high_by_block`] — the `mergeLinear` sort key (cover, then first
//!     instance addr, then def addr) is transcribed from `merge.hh:152-174`.
//!   - [`compare_copy_by_in_varnode`] — `findAllIntoCopies` groups COPYs by input
//!     `getCreateIndex()` then block index then `SeqNum::order` (`merge.cc:1045`).
//!   - `mergeByDatatype` walks the `VarnodeLocSet` in location order, dedups Highs
//!     by exact type, and feeds each type group to `mergeLinear` (`merge.cc:359`).
//!   - The speculative-merge *backout* in [`MergeIntersect::intersection`] and
//!     [`Merge::merge`] reproduces the cache-then-cover order exactly.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintm};

use crate::cover::Cover;
use crate::seams::{HighVariableId, OpId, VarnodeId};
use crate::variable::{HighContext, HighIntersectTest};

use kuna_num::opcodes::OpCode;

// =============================================================================
// BlockVarnode (merge.hh:32-41, merge.cc:24-61)
// =============================================================================

/// Helper associating a Varnode with the block index where it is defined (C++
/// `class BlockVarnode`, `merge.hh:32-41`).
///
/// A Varnode with no defining op is assigned index 0.  Caching the index makes
/// sorting Varnodes by defining block cheap.
#[derive(Debug, Clone, Copy)]
pub struct BlockVarnode {
    /// Index of the BlockBasic defining the Varnode (C++ `index`).
    index: int4,
    /// The Varnode itself (C++ `vn`).
    vn: VarnodeId,
}

impl BlockVarnode {
    /// Build a [`BlockVarnode`] for `vn` (C++ `BlockVarnode::set`,
    /// `merge.cc:24-34`): the defining op's parent block index, or 0 if undefined.
    pub fn new(vn: VarnodeId, ctx: &dyn MergeContext) -> BlockVarnode {
        let index = match ctx.vn_def(vn) {
            None => 0,
            Some(op) => ctx.op_parent_index(op),
        };
        BlockVarnode { index, vn }
    }

    /// The Varnode represented (C++ `getVarnode`).
    pub fn get_varnode(&self) -> VarnodeId {
        self.vn
    }

    /// The Varnode's defining block index (C++ `getIndex`).
    pub fn get_index(&self) -> int4 {
        self.index
    }

    /// C++ `BlockVarnode::operator<` (`merge.hh:37`): order by block index.
    pub fn less(&self, op2: &BlockVarnode) -> bool {
        self.index < op2.index
    }

    /// Find the first [`BlockVarnode`] in the sorted `list` defined in block
    /// `blocknum` (C++ `BlockVarnode::findFront`, `merge.cc:43-61`).  Returns the
    /// index within `list`, or -1 if no Varnode of the block is present.
    pub fn find_front(blocknum: int4, list: &[BlockVarnode]) -> int4 {
        let mut min: int4 = 0;
        let mut max: int4 = list.len() as int4 - 1;
        while min < max {
            let cur = (min + max) / 2;
            let curblock = list[cur as usize].get_index();
            if curblock >= blocknum {
                max = cur;
            } else {
                min = cur + 1;
            }
        }
        if min > max {
            return -1;
        }
        if list[min as usize].get_index() != blocknum {
            return -1;
        }
        min
    }
}

// =============================================================================
// MergeContext — the Funcdata/Varnode/Symbol surface the Merge passes drive
// =============================================================================

/// The [`Cover`] containment characterization a single read produces against the
/// other Varnodes at the same address (the `eliminateIntersect` core loop).
///
/// Mirrors `Cover::containVarnodeDef` (0 = no containment, 1 = contained, 2 =
/// on the defining boundary, 3 = on the tail of the range).
pub type BoundType = int4;

/// Cross-arena access the [`Merge`] formation passes need from `Funcdata`
/// (`obank`/`vbank`/`bblocks`/`localmap`/callspecs) plus the `Varnode` geometry
/// helpers (`copyShadow`/`characterizeOverlap`).
///
/// Implemented by `Funcdata` in the W7/W8 wave.  Methods whose underlying
/// `Funcdata`/`Varnode` operation is itself not yet ported are seams (recorded in
/// `docs/rust-port/losses.md`); the [`Merge`] routine that calls them transcribes
/// the surrounding structure verbatim so the body is correct once the seam fills.
pub trait MergeContext: HighContext {
    // --- HighVariable flag/type reads (the C++ direct `high->is*`/`getType`) --
    // The bank lazily re-derives flags/type from members on read, so these take
    // `&mut self`; the `Funcdata` impl does the field-split borrow internally.
    /// `high->isTypeLock()`.
    fn high_is_type_lock(&mut self, high: HighVariableId) -> bool;
    /// `high->getType()`.
    fn high_get_type(&mut self, high: HighVariableId) -> std::rc::Rc<crate::dtype::Datatype>;
    /// `high->isAddrTied()`.
    fn high_is_addr_tied(&mut self, high: HighVariableId) -> bool;
    /// `high->isInput()`.
    fn high_is_input(&mut self, high: HighVariableId) -> bool;
    /// `high->isPersist()`.
    fn high_is_persist(&mut self, high: HighVariableId) -> bool;
    /// `high->isExtraOut()`.
    fn high_is_extra_out(&mut self, high: HighVariableId) -> bool;
    /// `high->isProtoPartial()`.
    fn high_is_proto_partial(&mut self, high: HighVariableId) -> bool;
    /// `high->isNameLock()`.
    fn high_is_name_lock(&mut self, high: HighVariableId) -> bool;
    /// `high->hasCover()`.
    fn high_has_cover(&mut self, high: HighVariableId) -> bool;
    /// `high->isCoverDirty()`.
    fn high_is_cover_dirty(&self, high: HighVariableId) -> bool;
    /// `high->isMark()`.
    fn high_is_mark(&self, high: HighVariableId) -> bool;
    /// `high->hasCopyIn1()`.
    fn high_has_copy_in1(&self, high: HighVariableId) -> bool;
    /// `high->hasCopyIn2()`.
    fn high_has_copy_in2(&self, high: HighVariableId) -> bool;
    /// `high->numInstances()`.
    fn high_num_instances(&self, high: HighVariableId) -> int4;
    /// `high->getInstance(i)`.
    fn high_get_instance(&self, high: HighVariableId, i: int4) -> VarnodeId;
    /// `high->getCover()` (internal or extended; the `getCover` accessor).
    fn high_get_cover(&self, high: HighVariableId) -> Cover;
    /// `high->internalCover` (the inflateTest cover-to-include).
    fn high_internal_cover(&self, high: HighVariableId) -> Cover;
    /// Drive `bank.update_cover(high)` across the bank-split (the `updateHigh`
    /// rebuild step; the cache purge stays in [`MergeIntersect`]).
    fn bank_update_cover_for(&mut self, high: HighVariableId);

    // --- Bank-mediated HighVariable / piece introspection (seam: variable.rs
    //     does not expose `high->piece`/group offsets/size publicly) ----------
    /// `high->piece->getGroup()` plus `(piece->getSize(), group->getSize())` if
    /// `high` is part of an overlap group (the `mergeTestRequired` whole-group
    /// check), else `None`.  SEAM(W7-variable): the bank does not expose pieces.
    fn high_group_info(&self, high: HighVariableId) -> Option<HighGroupInfo>;
    /// `high->piece` id, if any (modelled as an opaque `MergePieceId`).
    fn high_piece(&self, high: HighVariableId) -> Option<MergePieceId>;
    /// `piece->getOffset()`.
    fn piece_offset(&self, piece: MergePieceId) -> int4;
    /// `piece->getSize()`.
    fn piece_size(&self, piece: MergePieceId) -> int4;
    /// `piece->getGroup()` identity (for the PIECE/SUBPIECE same-group test).
    fn piece_group(&self, piece: MergePieceId) -> u64;
    /// `piece->getHigh()`.
    fn piece_high(&self, piece: MergePieceId) -> HighVariableId;
    /// `piece->numIntersection()`.
    fn piece_num_intersection(&self, piece: MergePieceId) -> int4;
    /// `piece->getIntersection(i)`.
    fn piece_intersection(&self, piece: MergePieceId, i: int4) -> MergePieceId;
    /// `piece->updateIntersections()` for `high`'s piece (the inflateTest step).
    fn bank_update_piece_intersections(&mut self, piece: MergePieceId);
    /// `high1->merge(high2,&testCache,isspeculative)` then frees high2.
    fn bank_merge_highs(
        &mut self,
        high1: HighVariableId,
        high2: HighVariableId,
        isspeculative: bool,
        cache: &mut HighIntersectTest,
    ) -> KunaResult<()>;
    /// `high->updateCover()`.
    fn bank_update_cover(&mut self, high: HighVariableId);
    /// `vn2->getHigh()->groupWith(off, vn1->getHigh())`.
    fn bank_group_with(&mut self, high2: HighVariableId, off: int4, high1: HighVariableId) -> KunaResult<()>;
    /// `high->setMark()`.
    fn bank_set_mark(&mut self, high: HighVariableId);
    /// `high->clearMark()`.
    fn bank_clear_mark(&mut self, high: HighVariableId);
    /// `high->setUnmerged()`.
    fn bank_set_unmerged(&mut self, high: HighVariableId);
    /// `high->setCopyIn1()`.
    fn bank_set_copy_in1(&mut self, high: HighVariableId);
    /// `high->setCopyIn2()`.
    fn bank_set_copy_in2(&mut self, high: HighVariableId);
    /// `high->clearCopyIns()`.
    fn bank_clear_copy_ins(&mut self, high: HighVariableId);

    // --- Symbol reads on a HighVariable (W4 surface; seams) ----------------
    /// `high->getSymbol()` identity, if any.  SEAM(W4-symbol).
    fn bank_symbol(&self, high: HighVariableId) -> Option<u64>;
    /// `high->getSymbolOffset()`.  SEAM(W4-symbol).
    fn bank_symbol_offset(&self, high: HighVariableId) -> int4;
    /// `high->getSymbol()->isIsolated()`.  SEAM(W4-symbol).
    fn bank_symbol_isolated(&self, high: HighVariableId) -> bool;
    /// `high->getTiedVarnode()->getAddr()` (the addr-tied address compare).
    fn bank_tied_addr(&self, high: HighVariableId) -> kuna_base::address::Address;
    /// `high->getTiedVarnode()` (the untied-call test).
    fn bank_tied_varnode(&self, high: HighVariableId) -> Option<VarnodeId>;
    /// `high->getInputVarnode()` (the illegal-input adjacency test).
    fn bank_input_varnode(&self, high: HighVariableId) -> Option<VarnodeId>;

    // --- Varnode reads (vbank) --------------------------------------------
    /// `vn->getHigh()` — the HighVariable id (must exist for a coverable vn).
    fn vn_high(&self, vn: VarnodeId) -> Option<HighVariableId>;
    /// `vn->getDef()`.
    fn vn_def(&self, vn: VarnodeId) -> Option<OpId>;
    /// `vn->isFree()`.
    fn vn_is_free(&self, vn: VarnodeId) -> bool;
    /// `vn->isInput()`.
    fn vn_is_input(&self, vn: VarnodeId) -> bool;
    /// `vn->isWritten()`.
    fn vn_is_written(&self, vn: VarnodeId) -> bool;
    /// `vn->isAddrForce()`.
    fn vn_is_addr_force(&self, vn: VarnodeId) -> bool;
    /// `vn->isImplied()`.
    fn vn_is_implied(&self, vn: VarnodeId) -> bool;
    /// `vn->isSpacebase()`.
    fn vn_is_spacebase(&self, vn: VarnodeId) -> bool;
    /// `vn->isProtoPartial()`.
    fn vn_is_proto_partial(&self, vn: VarnodeId) -> bool;
    /// `vn->isAnnotation()`.
    fn vn_is_annotation(&self, vn: VarnodeId) -> bool;
    /// `vn->isIllegalInput()`.  SEAM(W4-symbol) for the input classification.
    fn vn_is_illegal_input(&self, vn: VarnodeId) -> bool;
    /// `vn->isIndirectOnly()`.
    fn vn_is_indirect_only(&self, vn: VarnodeId) -> bool;
    /// `vn->hasNoLocalAlias()`.
    fn vn_has_no_local_alias(&self, vn: VarnodeId) -> bool;
    /// `vn->hasNoDescend()`.
    fn vn_has_no_descend(&self, vn: VarnodeId) -> bool;
    /// `vn->getSpace()->isBigEndian()`.
    fn vn_space_big_endian(&self, vn: VarnodeId) -> bool;
    /// `vn->getSpace()->getType() == IPTR_INTERNAL`.
    fn vn_space_internal(&self, vn: VarnodeId) -> bool;
    /// `vn->hasCover()`.
    fn vn_has_cover_flag(&self, vn: VarnodeId) -> bool;
    /// `vn->getSize()`.
    fn vn_size(&self, vn: VarnodeId) -> int4;
    /// `vn->getOffset()` (the storage offset within its space).
    fn vn_offset(&self, vn: VarnodeId) -> u64;
    /// `vn->getCover()` — the member's (already rebuilt) Cover, or `None`.
    fn vn_cover_ref(&self, vn: VarnodeId) -> Option<Cover>;
    /// `vn->getCreateIndex()`.
    fn vn_create_index(&self, vn: VarnodeId) -> u64;
    /// `vn->getSymbolEntry()->getOffset()` (the CONCAT-tree base offset).
    fn vn_symbol_entry_offset(&self, vn: VarnodeId) -> Option<int4>;
    /// `a->copyShadow(b)` — `b` is a COPY-chain shadow of `a` (same value).
    /// SEAM(W7-varnode): `Varnode::copyShadow` (varnode.cc) is not yet ported.
    fn vn_copy_shadow(&self, a: VarnodeId, b: VarnodeId) -> bool;
    /// `a->partialCopyShadow(b,off)` — partial copy-shadow at byte offset `off`.
    /// SEAM(W7-varnode): `Varnode::partialCopyShadow` is not yet ported.
    fn vn_partial_copy_shadow(&self, a: VarnodeId, b: VarnodeId, off: int4) -> bool;
    /// `a->characterizeOverlap(b)` — 0=no overlap, 1=partial, 2=identical range.
    fn vn_characterize_overlap(&self, a: VarnodeId, b: VarnodeId) -> int4;
    /// `vn->setImplied()` (markImplied).
    fn vn_set_implied(&mut self, vn: VarnodeId);
    /// `vn->setFlags(Varnode::coverdirty)`.
    fn vn_set_cover_dirty(&mut self, vn: VarnodeId);
    /// `vn->clearImplied(); vn->setExplicit()`.
    fn vn_clear_implied_set_explicit(&mut self, vn: VarnodeId);
    /// `vn->clearProtoPartial()`.
    fn vn_clear_proto_partial(&mut self, vn: VarnodeId);
    /// The `(def_point, is_input)` of `vn` for `Cover::containVarnodeDef`.
    fn varnode_def_point(&self, vn: VarnodeId) -> (Option<(int4, crate::cover::CoverPoint)>, bool);
    /// `vn->beginDescend()..endDescend()` — reading ops in C++ order.
    fn vn_descend(&self, vn: VarnodeId) -> Vec<OpId>;

    // --- Op reads (obank) -------------------------------------------------
    /// `op->code()`.
    fn op_code(&self, op: OpId) -> OpCode;
    /// `op->getOut()`.
    fn op_out(&self, op: OpId) -> Option<VarnodeId>;
    /// `op->getIn(slot)`.
    fn op_in(&self, op: OpId, slot: int4) -> Option<VarnodeId>;
    /// `op->numInput()`.
    fn op_num_input(&self, op: OpId) -> int4;
    /// `op->getParent()` — the parent block id.
    fn op_parent(&self, op: OpId) -> crate::seams::BlockId;
    /// `op->getParent()->getIndex()` (the defining block index).
    fn op_parent_index(&self, op: OpId) -> int4;
    /// `op->getAddr()`.
    fn op_addr(&self, op: OpId) -> kuna_base::address::Address;
    /// `op->getSeqNum().getOrder()`.
    fn op_order(&self, op: OpId) -> uintm;
    /// `op->getSlot(vn)`.
    fn op_slot(&self, op: OpId, vn: VarnodeId) -> int4;
    /// `op->isDead()`.
    fn op_is_dead(&self, op: OpId) -> bool;
    /// `op->isCall()`.
    fn op_is_call(&self, op: OpId) -> bool;
    /// `op->isMarker()`.
    fn op_is_marker(&self, op: OpId) -> bool;
    /// `op->isIndirectCreation()`.
    fn op_is_indirect_creation(&self, op: OpId) -> bool;
    /// `op->isPartialRoot()`.
    fn op_is_partial_root(&self, op: OpId) -> bool;
    /// `op->previousOp()`.
    fn op_previous(&self, op: OpId) -> Option<OpId>;
    /// `op->outputTypeLocal()`.
    fn op_output_type_local(&self, op: OpId) -> std::rc::Rc<crate::dtype::Datatype>;
    /// `op->inputTypeLocal(slot)`.
    fn op_input_type_local(&self, op: OpId, slot: int4) -> std::rc::Rc<crate::dtype::Datatype>;
    /// The `(block_index, CoverPoint)` of `op` (for `range.contain(op,1)`).
    fn op_cover_point(&self, op: OpId) -> (int4, crate::cover::CoverPoint);
    /// `getOpFromConst(indop->getIn(1)->getAddr())` — the op an INDIRECT marks.
    fn indirect_effect_op(&self, indop: OpId) -> OpId;

    // --- Op mutation (obank); funcdata mutators that already exist ---------
    /// `data.opSetInput(op,vn,slot)`.
    fn op_set_input(&mut self, op: OpId, vn: VarnodeId, slot: int4) -> KunaResult<()>;
    /// `data.opInsertBefore(op,follow)`.
    fn op_insert_before(&mut self, op: OpId, follow: OpId);
    /// `data.opInsertAfter(op,prev)`.
    fn op_insert_after(&mut self, op: OpId, prev: OpId);
    /// `data.opInsertBegin(op,bl)`.
    fn op_insert_begin(&mut self, op: OpId, bl: crate::seams::BlockId);
    /// `data.opInsertEnd(copyop, (BlockBasic*)op->getParent()->getIn(slot))` —
    /// the MULTIEQUAL trim insertion at the predecessor block of `slot`.
    fn op_insert_end_pred(&mut self, copyop: OpId, op: OpId, slot: int4);
    /// `data.opMarkNonPrinting(op)`.  SEAM(W7-funcdata): not yet on Funcdata.
    fn op_mark_non_printing(&mut self, op: OpId);

    // --- Block reads -------------------------------------------------------
    /// `data.getBasicBlocks().getSize()`.
    fn bblocks_get_size(&self) -> int4;
    /// `data.getBasicBlocks().getBlock(i)`.
    fn bblocks_get_block(&self, i: int4) -> crate::seams::BlockId;
    /// `bl->beginOp()..endOp()` — the block's op list (linear order).
    fn block_ops(&self, bl: crate::seams::BlockId) -> Vec<OpId>;
    /// `domBlock->dominates(subBlock)`.
    fn block_dominates(&self, dom: crate::seams::BlockId, sub: crate::seams::BlockId) -> bool;

    // --- Function-level op iteration ---------------------------------------
    /// `data.beginOpAlive()..endOpAlive()` — the alive-op list in C++ order.
    fn ops_alive(&self) -> Vec<OpId>;

    // --- Cover construction for a single read (eliminateIntersect) --------
    /// Build the [`Cover`] of the single read of `vn` by `op` (the C++
    /// `Cover single; single.addDefPoint(vn); single.addRefPoint(op,vn)`).
    fn single_read_cover(&self, vn: VarnodeId, op: OpId) -> Cover;
    /// Build the `Cover range` of `checkCopyPair` (`addDefPoint(domOp->getOut())`
    /// then `addRefPoint(subOp,subOp->getIn(0))`).
    fn copy_pair_range(&self, dom_op: OpId, sub_op: OpId) -> Cover;

    // --- IR-surgery seams (mutators not all yet on Funcdata) ----------------
    /// `Merge::allocateCopyTrim` body: build a COPY of `in_vn` into a fresh unique
    /// with the union-field resolution, returning the new COPY op.
    fn copy_trim_op(
        &mut self,
        in_vn: VarnodeId,
        addr: kuna_base::address::Address,
        trim_op: OpId,
    ) -> KunaResult<OpId>;
    /// `Merge::trimOpOutput` body (the INDIRECT-aware output bump).
    fn trim_op_output(&mut self, op: OpId) -> KunaResult<()>;
    /// `Merge::trimOpInput` prelude: the `(inVn, pc, isMultiequal)` it computes
    /// before allocating the trim COPY.
    fn trim_op_input_prep(&self, op: OpId, slot: int4) -> (VarnodeId, kuna_base::address::Address, bool);
    /// `Merge::snipReads` insert-point resolution: `(block, pc, afterop)`.
    fn snip_reads_insert_point(
        &self,
        vn: VarnodeId,
    ) -> (crate::seams::BlockId, kuna_base::address::Address, Option<OpId>);
    /// `Merge::mergeIndirect` union-field inherit for the snipped INDIRECT.
    fn indirect_inherit_union(&mut self, outvn: VarnodeId, newop: OpId, indop: OpId);
    /// `PcodeOpNode::compareByHigh` sort over the collected inputs.
    fn sort_op_nodes_by_high(&self, nodes: &mut Vec<crate::expression::PcodeOpNode>);
    /// `Merge::buildDominantCopy` body (findCommonBlock / totalReplace / opDestroy).
    /// SEAM(W7-funcdata): `FlowBlock::findCommonBlock`/`totalReplace` surgery.
    fn build_dominant_copy(
        &mut self,
        high: HighVariableId,
        copy: &[OpId],
        pos: int4,
        size: int4,
    ) -> KunaResult<()>;

    // --- mergeAddrTied / mergeMultiEntry seams (overlapLoc/scope; W7/W4) ----
    /// `Merge::mergeAddrTied` range collection (`overlapLoc` over the location set).
    /// SEAM(W7-funcdata): `Funcdata::overlapLoc` is not yet ported.
    fn addr_tied_ranges(&self) -> Vec<AddrTiedRange>;
    /// `getScopeLocal()->beginMultiEntry()..endMultiEntry()`.  SEAM(W4-symbol).
    fn multi_entry_symbols(&self) -> Vec<u64>;
    /// `findLinkedVarnodes` over a multi-entry symbol: `(varnodes, skipCount)`.
    fn symbol_linked_varnodes(&self, symbol: u64) -> (Vec<VarnodeId>, int4);
    /// `symbol->setMergeProblems()`.
    fn symbol_set_merge_problems(&mut self, symbol: u64);
    /// `data.warningHeader(...)` for the multi-entry merge failure.
    fn symbol_merge_warning(&mut self, symbol: u64, merge_count: int4, skip_count: int4, conflict_count: int4);
    /// `StackAffectingOps::populate` into the cache's `PcodeOpSet`.  SEAM(W7).
    fn populate_affecting_ops(&self, op_set: &mut crate::cover::PcodeOpSet);
    /// `PieceNode::gatherPieces(pieces, vn, vn->getDef(), baseOffset, baseOffset)`:
    /// the CONCAT-tree nodes as `(node_varnode, typeOffset)` pairs (groupPartialRoot).
    fn gather_pieces(&self, vn: VarnodeId, base_offset: int4) -> Vec<(VarnodeId, int4)>;
}

/// Opaque id for a `VariablePiece` as seen across the [`MergeContext`] seam (the
/// bank does not expose `VariablePieceId` publicly; the `Funcdata` impl maps).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergePieceId(pub u64);

/// The `mergeTestRequired` whole-group info: the group identity plus the piece's
/// size and the group's size (the "at least one piece represents its whole group"
/// test, `merge.cc:147-154`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighGroupInfo {
    /// `piece->getGroup()` identity.
    pub group: u64,
    /// `piece->getSize()`.
    pub piece_size: int4,
    /// `group->getSize()`.
    pub group_size: int4,
}

/// One maximally-overlapping address-tied Varnode range from `mergeAddrTied`
/// (`merge.cc:609-648`).  The `overlapLoc`-driven collection is a seam; this
/// carries its result.
pub struct AddrTiedRange {
    /// Whether the range is address tied (`flags & Varnode::addrtied`).
    pub addrtied: bool,
    /// All non-free Varnodes in the maximal range (the `unifyAddress` input).
    pub all_varnodes: Vec<VarnodeId>,
    /// The `[bounds[i],bounds[i+1])` sub-ranges that `mergeRangeMust` merges.
    pub must_merge_ranges: Vec<Vec<VarnodeId>>,
    /// The `(vn2->getHigh(), off, vn1->getHigh())` group-with triples.
    pub group_with: Vec<(HighVariableId, int4, HighVariableId)>,
}

// =============================================================================
// Merge-test predicates (merge.cc:102-264)
// =============================================================================
//
// These operate purely on HighVariable flag/type/symbol/piece state read through
// the bank + context; no Cover work.  They are transcribed verbatim, including the
// short-circuit order, which is output-determining (the first failing test wins).

impl Merge {
    /// Required (non-Cover) tests to merge two HighVariables (C++
    /// `Merge::mergeTestRequired`, `merge.cc:102-166`).
    ///
    /// Short-circuits when properties preclude merging (locked to different types,
    /// mapped to different addresses, input-vs-global, separate overlap groups,
    /// different symbols).  Returns `true` if the merge is *not forbidden*.
    pub fn merge_test_required(
        ctx: &mut dyn MergeContext,
        high_out: HighVariableId,
        high_in: HighVariableId,
    ) -> bool {
        if high_in == high_out {
            return true; // Already merged
        }

        // If types are locked don't merge unless both types are the same.
        if ctx.high_is_type_lock(high_in) && ctx.high_is_type_lock(high_out) {
            let t_in = ctx.high_get_type(high_in);
            let t_out = ctx.high_get_type(high_out);
            if !std::rc::Rc::ptr_eq(&t_in, &t_out) {
                return false;
            }
        }

        // Do not merge address tied input with an address tied output at a
        // different address.
        if ctx.high_is_addr_tied(high_out) && ctx.high_is_addr_tied(high_in) {
            let tied_in = ctx.bank_tied_addr(high_in);
            let tied_out = ctx.bank_tied_addr(high_out);
            if tied_in != tied_out {
                return false;
            }
        }

        if ctx.high_is_input(high_in) {
            // Input and persist must be different vars (persists have their own
            // input); also keep inputs from merging into internal stack parts.
            if ctx.high_is_persist(high_out) {
                return false;
            }
            if ctx.high_is_addr_tied(high_out) && !ctx.high_is_addr_tied(high_in) {
                return false;
            }
        } else if ctx.high_is_extra_out(high_in) {
            return false;
        }
        if ctx.high_is_input(high_out) {
            if ctx.high_is_persist(high_in) {
                return false;
            }
            if ctx.high_is_addr_tied(high_in) && !ctx.high_is_addr_tied(high_out) {
                return false;
            }
        } else if ctx.high_is_extra_out(high_out) {
            return false;
        }

        if ctx.high_is_proto_partial(high_in) {
            if ctx.high_is_proto_partial(high_out) {
                return false;
            }
            if ctx.high_is_input(high_out) {
                return false;
            }
            if ctx.high_is_addr_tied(high_out) {
                return false;
            }
            if ctx.high_is_persist(high_out) {
                return false;
            }
        }
        if ctx.high_is_proto_partial(high_out) {
            if ctx.high_is_input(high_in) {
                return false;
            }
            if ctx.high_is_addr_tied(high_in) {
                return false;
            }
            if ctx.high_is_persist(high_in) {
                return false;
            }
        }

        // Both pieces of separate overlapping collections: at least one must
        // represent its whole group.
        let group_in = ctx.high_group_info(high_in);
        let group_out = ctx.high_group_info(high_out);
        if let (Some(gi), Some(go)) = (group_in, group_out) {
            if gi.group == go.group {
                return false;
            }
            if gi.piece_size != gi.group_size && go.piece_size != go.group_size {
                return false;
            }
        }

        // Map to different symbols / different parts of the same symbol.
        let sym_in = ctx.bank_symbol(high_in);
        let sym_out = ctx.bank_symbol(high_out);
        if let (Some(si), Some(so)) = (sym_in, sym_out) {
            if si != so {
                return false;
            }
            if ctx.bank_symbol_offset(high_in) != ctx.bank_symbol_offset(high_out) {
                return false;
            }
        }
        true
    }

    /// Adjacency tests for input/output of the same op (C++
    /// `Merge::mergeTestAdjacent`, `merge.cc:175-211`).  All required tests plus
    /// name-lock/type/illegal-input/isolated-symbol/separate-group checks.
    pub fn merge_test_adjacent(
        ctx: &mut dyn MergeContext,
        high_out: HighVariableId,
        high_in: HighVariableId,
    ) -> bool {
        if !Self::merge_test_required(ctx, high_out, high_in) {
            return false;
        }

        if ctx.high_is_name_lock(high_in) && ctx.high_is_name_lock(high_out) {
            return false;
        }

        // Make sure variables have the same type.
        let t_out = ctx.high_get_type(high_out);
        let t_in = ctx.high_get_type(high_in);
        if !std::rc::Rc::ptr_eq(&t_out, &t_in) {
            return false;
        }

        // Isolate the use of illegal inputs: no speculative merges with them
        // unless the illegal input is only used indirectly.
        if ctx.high_is_input(high_out) {
            if let Some(vn) = ctx.bank_input_varnode(high_out) {
                if ctx.vn_is_illegal_input(vn) && !ctx.vn_is_indirect_only(vn) {
                    return false;
                }
            }
        }
        if ctx.high_is_input(high_in) {
            if let Some(vn) = ctx.bank_input_varnode(high_in) {
                if ctx.vn_is_illegal_input(vn) && !ctx.vn_is_indirect_only(vn) {
                    return false;
                }
            }
        }
        if ctx.bank_symbol_isolated(high_in) {
            return false;
        }
        if ctx.bank_symbol_isolated(high_out) {
            return false;
        }

        // Currently don't allow speculative merging across separate overlapping
        // collections.
        if ctx.high_piece(high_out).is_some() && ctx.high_piece(high_in).is_some() {
            return false;
        }
        true
    }

    /// Speculative tests (C++ `Merge::mergeTestSpeculative`, `merge.cc:220-235`):
    /// all adjacency tests, then no merges with globals/inputs/addrtieds.
    pub fn merge_test_speculative(
        ctx: &mut dyn MergeContext,
        high_out: HighVariableId,
        high_in: HighVariableId,
    ) -> bool {
        if !Self::merge_test_adjacent(ctx, high_out, high_in) {
            return false;
        }
        if ctx.high_is_persist(high_out) {
            return false;
        }
        if ctx.high_is_persist(high_in) {
            return false;
        }
        if ctx.high_is_input(high_out) {
            return false;
        }
        if ctx.high_is_input(high_in) {
            return false;
        }
        if ctx.high_is_addr_tied(high_out) {
            return false;
        }
        if ctx.high_is_addr_tied(high_in) {
            return false;
        }
        true
    }

    /// Test that a Varnode that *must* merge *can* merge (C++
    /// `Merge::mergeTestMust`, `merge.cc:241-247`).  Errors otherwise.
    pub fn merge_test_must(ctx: &dyn MergeContext, vn: VarnodeId) -> KunaResult<()> {
        if ctx.vn_has_cover_flag(vn) && !ctx.vn_is_implied(vn) {
            return Ok(());
        }
        Err(KunaError::lowlevel("Cannot force merge of range"))
    }

    /// Test if a Varnode can ever be merged (C++ `Merge::mergeTestBasic`,
    /// `merge.cc:255-264`): not constant/annotation/implied/protoPartial/spacebase.
    pub fn merge_test_basic(ctx: &dyn MergeContext, vn: Option<VarnodeId>) -> bool {
        let vn = match vn {
            None => return false,
            Some(v) => v,
        };
        if !ctx.vn_has_cover_flag(vn) {
            return false;
        }
        if ctx.vn_is_implied(vn) {
            return false;
        }
        if ctx.vn_is_proto_partial(vn) {
            return false;
        }
        if ctx.vn_is_spacebase(vn) {
            return false;
        }
        true
    }
}

// =============================================================================
// compareHighByBlock (merge.hh:152-174)
// =============================================================================

/// C++ `Merge::compareHighByBlock` (`merge.hh:152-174`): the `mergeLinear` sort
/// key — cover, then first-instance address, then defining-op address.
///
/// Returns `true` if `a` should be ordered before `b`.  Driven through the bank +
/// context to reach instance covers/addresses across the arena boundary.
pub fn compare_high_by_block(
    ctx: &dyn MergeContext,
    a: HighVariableId,
    b: HighVariableId,
) -> bool {
    let cover_a = ctx.high_get_cover(a);
    let cover_b = ctx.high_get_cover(b);
    let result = cover_a.compare_to(&cover_b);
    if result == 0 {
        let v1 = ctx.high_get_instance(a, 0);
        let v2 = ctx.high_get_instance(b, 0);
        let addr1 = ctx.vn_loc_view(v1).addr;
        let addr2 = ctx.vn_loc_view(v2).addr;
        if addr1 == addr2 {
            let def1 = ctx.vn_def(v1);
            let def2 = ctx.vn_def(v2);
            match (def1, def2) {
                (None, _) => return def2.is_some(),
                (Some(_), None) => return false,
                (Some(d1), Some(d2)) => return ctx.op_addr(d1) < ctx.op_addr(d2),
            }
        }
        return addr1 < addr2;
    }
    result < 0
}

/// C++ `Merge::compareCopyByInVarnode` (`merge.cc:1045-1057`): group COPYs by
/// input `getCreateIndex()`, then defining block index, then `SeqNum::order`.
pub fn compare_copy_by_in_varnode(ctx: &dyn MergeContext, op1: OpId, op2: OpId) -> bool {
    let in_vn1 = ctx.op_in(op1, 0).expect("compareCopyByInVarnode: op1 no in0");
    let in_vn2 = ctx.op_in(op2, 0).expect("compareCopyByInVarnode: op2 no in0");
    if in_vn1 != in_vn2 {
        return ctx.vn_create_index(in_vn1) < ctx.vn_create_index(in_vn2);
    }
    let index1 = ctx.op_parent_index(op1);
    let index2 = ctx.op_parent_index(op2);
    if index1 != index2 {
        return index1 < index2;
    }
    ctx.op_order(op1) < ctx.op_order(op2)
}

// =============================================================================
// MergeIntersect — the intersection-cache driver (variable.cc:960-1217)
// =============================================================================
//
// Hosted here because it is owned by Merge::testCache (the cache *primitives* —
// purgeHigh/moveIntersectTests/cached/cacheResult — were ported with the bank in
// `w7-s6-variable-cover`; this is the cover-walking driver that calls them).
//
// `copyShadow`/`partialCopyShadow` (varnode.cc, not yet ported) route through the
// MergeContext; everything else is built on the Cover + bank APIs.

/// The Cover-intersection driver over a [`HighIntersectTest`] cache (C++
/// `HighIntersectTest::{intersection,updateHigh,blockIntersection,…}`).
pub struct MergeIntersect;

impl MergeIntersect {
    /// Gather instance Varnodes of `a` that intersect `cover` on block `blk` above
    /// a point (C++ `HighIntersectTest::gatherBlockVarnodes`, `variable.cc:960`).
    fn gather_block_varnodes(
        ctx: &dyn MergeContext,
        a: HighVariableId,
        blk: int4,
        cover: &Cover,
        res: &mut Vec<VarnodeId>,
    ) {
        let n = ctx.high_num_instances(a);
        for i in 0..n {
            let vn = ctx.high_get_instance(a, i);
            let vncover = ctx.vn_cover_ref(vn).unwrap_or_default();
            if 1 < vncover.intersect_by_block(blk, cover) {
                res.push(vn);
            }
        }
    }

    /// Test instances of `a` against `cover` on block `blk` with copy-shadow
    /// refinement (C++ `HighIntersectTest::testBlockIntersection`,
    /// `variable.cc:981-1002`).  Returns `true` if a merging intersection is found.
    fn test_block_intersection(
        ctx: &dyn MergeContext,
        a: HighVariableId,
        blk: int4,
        cover: &Cover,
        rel_off: int4,
        blist: &[VarnodeId],
    ) -> bool {
        let n = ctx.high_num_instances(a);
        for i in 0..n {
            let vn = ctx.high_get_instance(a, i);
            let vncover = ctx.vn_cover_ref(vn).unwrap_or_default();
            if 2 > vncover.intersect_by_block(blk, cover) {
                continue;
            }
            for &vn2 in blist {
                let vn2cover = ctx.vn_cover_ref(vn2).unwrap_or_default();
                if 1 < vn2cover.intersect_by_block(blk, &vncover) {
                    if ctx.vn_size(vn) == ctx.vn_size(vn2) {
                        if !ctx.vn_copy_shadow(vn, vn2) {
                            return true;
                        }
                    } else if !ctx.vn_partial_copy_shadow(vn, vn2, rel_off) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Test if two HighVariables intersect on a given block, accounting for their
    /// overlap-group pieces (C++ `HighIntersectTest::blockIntersection`,
    /// `variable.cc:1011-1053`).
    fn block_intersection(
        ctx: &dyn MergeContext,
        a: HighVariableId,
        b: HighVariableId,
        blk: int4,
    ) -> bool {
        let mut blist: Vec<VarnodeId> = Vec::new();
        let a_cover = ctx.high_get_cover(a);
        let b_cover = ctx.high_get_cover(b);
        Self::gather_block_varnodes(ctx, b, blk, &a_cover, &mut blist);
        if Self::test_block_intersection(ctx, a, blk, &b_cover, 0, &blist) {
            return true;
        }
        // a's pieces
        if let Some(a_piece) = ctx.high_piece(a) {
            let base_off = ctx.piece_offset(a_piece);
            for i in 0..ctx.piece_num_intersection(a_piece) {
                let inter = ctx.piece_intersection(a_piece, i);
                let off = ctx.piece_offset(inter) - base_off;
                let inter_high = ctx.piece_high(inter);
                if Self::test_block_intersection(ctx, inter_high, blk, &b_cover, off, &blist) {
                    return true;
                }
            }
        }
        // b's pieces
        if let Some(b_piece) = ctx.high_piece(b) {
            let b_base_off = ctx.piece_offset(b_piece);
            for i in 0..ctx.piece_num_intersection(b_piece) {
                blist.clear();
                let b_inter = ctx.piece_intersection(b_piece, i);
                let b_off = ctx.piece_offset(b_inter) - b_base_off;
                let b_inter_high = ctx.piece_high(b_inter);
                Self::gather_block_varnodes(ctx, b_inter_high, blk, &a_cover, &mut blist);
                if Self::test_block_intersection(ctx, a, blk, &b_cover, -b_off, &blist) {
                    return true;
                }
                if let Some(a_piece) = ctx.high_piece(a) {
                    let base_off = ctx.piece_offset(a_piece);
                    for j in 0..ctx.piece_num_intersection(a_piece) {
                        let inter = ctx.piece_intersection(a_piece, j);
                        let off = (ctx.piece_offset(inter) - base_off) - b_off;
                        let b_inter_size = ctx.piece_size(b_inter);
                        let inter_size = ctx.piece_size(inter);
                        // Do a piece and b piece intersect at all
                        if off > 0 && off >= b_inter_size {
                            continue;
                        }
                        if off < 0 && -off >= inter_size {
                            continue;
                        }
                        let inter_high = ctx.piece_high(inter);
                        if Self::test_block_intersection(ctx, inter_high, blk, &b_cover, off, &blist) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Test if `untied` crosses a call relative to address-tied `tied` (C++
    /// `HighIntersectTest::testUntiedCallIntersection`, `variable.cc:1085-1096`).
    fn test_untied_call_intersection(
        ctx: &mut dyn MergeContext,
        cache: &mut HighIntersectTest,
        tied: HighVariableId,
        untied: HighVariableId,
    ) -> bool {
        // If the address tied part is global, no crossing test needed (the
        // address-forcing mechanism is a placeholder across calls).
        if ctx.high_is_persist(tied) {
            return false;
        }
        let vn = match ctx.bank_tied_varnode(tied) {
            None => return false,
            Some(v) => v,
        };
        if ctx.vn_has_no_local_alias(vn) {
            return false; // a local is only in scope if it has aliases
        }
        if !cache.affecting_ops_mut().is_populated() {
            ctx.populate_affecting_ops(cache.affecting_ops_mut());
        }
        let untied_cover = ctx.high_get_cover(untied);
        untied_cover.intersect_op_set(cache.affecting_ops_mut(), vn)
    }

    /// Update a HighVariable's Cover if dirty, purging stale cache tests (C++
    /// `HighIntersectTest::updateHigh`, `variable.cc:1164-1172`).  Returns `true`
    /// if the high was *not* originally dirty.
    pub fn update_high(
        ctx: &mut dyn MergeContext,
        cache: &mut HighIntersectTest,
        a: HighVariableId,
    ) -> bool {
        if !ctx.high_is_cover_dirty(a) {
            return true;
        }
        ctx.bank_update_cover_for(a);
        cache.purge_high(a);
        false
    }

    /// Test (and cache) whether two HighVariables' Covers intersect (C++
    /// `HighIntersectTest::intersection`, `variable.cc:1182-1217`).
    pub fn intersection(
        ctx: &mut dyn MergeContext,
        cache: &mut HighIntersectTest,
        a: HighVariableId,
        b: HighVariableId,
    ) -> bool {
        if a == b {
            return false;
        }
        let ares = Self::update_high(ctx, cache, a);
        let bres = Self::update_high(ctx, cache, b);
        if ares && bres {
            // Neither high was dirty: reuse a cached result if present.
            if let Some(prev) = cache.cached(a, b) {
                return prev;
            }
        }

        let mut res = false;
        let a_cover = ctx.high_get_cover(a);
        let b_cover = ctx.high_get_cover(b);
        let mut blockisect: Vec<int4> = Vec::new();
        a_cover.intersect_list(&mut blockisect, &b_cover, 2);
        for &blk in &blockisect {
            if Self::block_intersection(ctx, a, b, blk) {
                res = true;
                break;
            }
        }
        if !res {
            let a_tied = ctx.high_is_addr_tied(a);
            let b_tied = ctx.high_is_addr_tied(b);
            if a_tied != b_tied {
                if a_tied {
                    res = Self::test_untied_call_intersection(ctx, cache, a, b);
                } else {
                    res = Self::test_untied_call_intersection(ctx, cache, b, a);
                }
            }
        }
        cache.cache_result(a, b, res);
        res
    }
}

// =============================================================================
// Merge (merge.hh:83-139)
// =============================================================================

/// The HighVariable-formation driver for one function (C++ `class Merge`,
/// `merge.hh:83-139`).
///
/// Holds the intersection-test cache, the COPYs inserted to facilitate merges
/// (`copyTrims`), and the roots of unmapped CONCAT trees (`protoPartial`).  The
/// `StackAffectingOps` set lives inside the cache's `affecting_ops` (the
/// [`HighIntersectTest`] owns the `PcodeOpSet`).
pub struct Merge {
    /// Cached intersection tests (C++ `testCache`, an embedded `HighIntersectTest`).
    test_cache: HighIntersectTest,
    /// COPY ops inserted to facilitate merges (C++ `copyTrims`).
    copy_trims: Vec<OpId>,
    /// Roots of unmapped CONCAT trees (C++ `protoPartial`).
    proto_partial: Vec<OpId>,
}

impl Merge {
    /// Construct given a specific function's intersection cache (C++
    /// `Merge(Funcdata &fd)`, `merge.hh:119`).
    ///
    /// The C++ constructs `stackAffectingOps(fd)` and `testCache(stackAffectingOps)`;
    /// here the affecting-ops `PcodeOpSet` is supplied already wired (it is the
    /// `StackAffectingOps` populated from the function's CALL/STORE ops).
    pub fn new(test_cache: HighIntersectTest) -> Merge {
        Merge { test_cache, copy_trims: Vec::new(), proto_partial: Vec::new() }
    }

    /// Borrow the intersection-test cache.
    pub fn test_cache_mut(&mut self) -> &mut HighIntersectTest {
        &mut self.test_cache
    }

    /// Clear cached data from the last merge process (C++ `Merge::clear`,
    /// `merge.cc:1580-1587`).
    pub fn clear(&mut self) {
        self.test_cache.clear();
        self.copy_trims.clear();
        self.proto_partial.clear();
        self.test_cache.affecting_ops_mut().clear();
    }

    /// Number of accumulated trim COPYs (C++ `copyTrims.size()`).  The persistent
    /// `Funcdata::covermerge` accumulates these across the merge actions so
    /// `processCopyTrims`/`ActionDominantCopy` can act on them — this accessor pins
    /// that the accumulator survives the move-out/move-back of `with_covermerge`.
    pub fn copy_trims_len(&self) -> usize {
        self.copy_trims.len()
    }

    /// Test-only: push a trim COPY onto the accumulator (pins the persistent
    /// `covermerge` accumulator survival in `funcdata.rs`).
    #[doc(hidden)]
    pub fn push_copy_trim_for_test(&mut self, op: OpId) {
        self.copy_trims.push(op);
    }

    /// Register an unmapped CONCAT-tree root with the merge process (C++
    /// `Merge::registerProtoPartialRoot`, `merge.cc:1549-1553`).
    pub fn register_proto_partial_root(&mut self, ctx: &dyn MergeContext, vn: VarnodeId) {
        if let Some(op) = ctx.vn_def(vn) {
            self.proto_partial.push(op);
        }
    }

    /// Mark a Varnode as \e implied, dirtying the covers of the immediate Varnodes
    /// in the expression (C++ `Merge::markImplied`, `merge.cc:1595-1605`).
    pub fn mark_implied(ctx: &mut dyn MergeContext, vn: VarnodeId) {
        ctx.vn_set_implied(vn);
        if let Some(op) = ctx.vn_def(vn) {
            let n = ctx.op_num_input(op);
            for i in 0..n {
                if let Some(defvn) = ctx.op_in(op, i) {
                    if !ctx.vn_has_cover_flag(defvn) {
                        continue;
                    }
                    ctx.vn_set_cover_dirty(defvn);
                }
            }
        }
    }

    /// Perform the low-level merge of two HighVariables if no Cover intersection
    /// (C++ `Merge::merge`, `merge.cc:1565-1575`).  Returns `true` on success.
    pub fn merge(
        &mut self,
        ctx: &mut dyn MergeContext,
        high1: HighVariableId,
        high2: HighVariableId,
        isspeculative: bool,
    ) -> KunaResult<bool> {
        if high1 == high2 {
            return Ok(true); // Already merged
        }
        if MergeIntersect::intersection(ctx, &mut self.test_cache, high1, high2) {
            return Ok(false);
        }
        // high1->merge(high2,&testCache,isspeculative); high1->updateCover();
        ctx.bank_merge_highs(high1, high2, isspeculative, &mut self.test_cache)?;
        ctx.bank_update_cover(high1);
        Ok(true)
    }

    /// Speculatively merge all HighVariables in `highvec` as well as possible (C++
    /// `Merge::mergeLinear`, `merge.cc:272-292`).
    ///
    /// The variables are sorted by earliest block, then each is matched against the
    /// stack of accepted variables; a match needs the speculative test to pass and
    /// no Cover intersection (else that particular pairing is skipped).
    pub fn merge_linear(&mut self, ctx: &mut dyn MergeContext, highvec: &mut [HighVariableId]) -> KunaResult<()> {
        if highvec.len() <= 1 {
            return Ok(());
        }
        let hs: Vec<HighVariableId> = highvec.to_vec();
        for h in hs {
            MergeIntersect::update_high(ctx, &mut self.test_cache, h);
        }
        // sort(highvec, compareHighByBlock)
        Self::sort_high_by_block(ctx, highvec);

        let mut highstack: Vec<HighVariableId> = Vec::new();
        let order: Vec<HighVariableId> = highvec.to_vec();
        for high in order {
            let mut merged_into = false;
            for &out in &highstack {
                let spec_ok = Self::merge_test_speculative(ctx, out, high);
                if spec_ok && self.merge(ctx, out, high, true)? {
                    merged_into = true;
                    break;
                }
            }
            if !merged_into {
                highstack.push(high);
            }
        }
        Ok(())
    }

    /// Stable sort `highvec` by [`compare_high_by_block`] (the C++ `std::sort`).
    fn sort_high_by_block(ctx: &dyn MergeContext, highvec: &mut [HighVariableId]) {
        highvec.sort_by(|&a, &b| {
            if compare_high_by_block(ctx, a, b) {
                std::cmp::Ordering::Less
            } else if compare_high_by_block(ctx, b, a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    /// Test for intersections between `high` and a list of others (C++
    /// `Merge::mergeTest`, `merge.cc:1657-1669`).  Appends `high` and returns
    /// `true` if there are no pairwise intersections.
    pub fn merge_test(
        &mut self,
        ctx: &mut dyn MergeContext,
        high: HighVariableId,
        tmplist: &mut Vec<HighVariableId>,
    ) -> bool {
        if !ctx.high_has_cover(high) {
            return false;
        }
        let others: Vec<HighVariableId> = tmplist.clone();
        for a in others {
            if MergeIntersect::intersection(ctx, &mut self.test_cache, a, high) {
                return false;
            }
        }
        tmplist.push(high);
        true
    }

    /// Test if inflating `a`'s Cover to include `high`'s would intersect (C++
    /// `Merge::inflateTest`, `merge.cc:1616-1647`).  Returns `true` on intersection.
    pub fn inflate_test(&mut self, ctx: &mut dyn MergeContext, a: VarnodeId, high: HighVariableId) -> bool {
        let ahigh = match ctx.vn_high(a) {
            None => return false,
            Some(h) => h,
        };
        MergeIntersect::update_high(ctx, &mut self.test_cache, high);
        // const Cover &highCover( high->internalCover )
        let high_cover = ctx.high_internal_cover(high);

        let n = ctx.high_num_instances(ahigh);
        for i in 0..n {
            let b = ctx.high_get_instance(ahigh, i);
            if ctx.vn_copy_shadow(b, a) {
                continue; // Intersection with a or shadows of a is allowed
            }
            let bcover = ctx.vn_cover_ref(b).unwrap_or_default();
            if 2 == bcover.intersect(&high_cover) {
                return true;
            }
        }
        // VariablePiece *piece = ahigh->piece;
        let piece = ctx.high_piece(ahigh);
        if let Some(piece) = piece {
            ctx.bank_update_piece_intersections(piece);
            let num_inter = ctx.piece_num_intersection(piece);
            for i in 0..num_inter {
                let other_piece = ctx.piece_intersection(piece, i);
                let off = ctx.piece_offset(other_piece) - ctx.piece_offset(piece);
                let other_high = ctx.piece_high(other_piece);
                let on = ctx.high_num_instances(other_high);
                for j in 0..on {
                    let b = ctx.high_get_instance(other_high, j);
                    if ctx.vn_partial_copy_shadow(b, a, off) {
                        continue;
                    }
                    let bcover = ctx.vn_cover_ref(b).unwrap_or_default();
                    if 2 == bcover.intersect(&high_cover) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Force input/output merges of all ops of a given opcode (C++
    /// `Merge::mergeOpcode`, `merge.cc:326-350`).  Merges in linear block order.
    pub fn merge_opcode(&mut self, ctx: &mut dyn MergeContext, opc: OpCode) -> KunaResult<()> {
        let nblocks = ctx.bblocks_get_size();
        for i in 0..nblocks {
            let bl = ctx.bblocks_get_block(i);
            let ops: Vec<OpId> = ctx.block_ops(bl);
            for op in ops {
                if ctx.op_code(op) != opc {
                    continue;
                }
                let vn1 = ctx.op_out(op);
                if !Self::merge_test_basic(ctx, vn1) {
                    continue;
                }
                let vn1 = vn1.unwrap();
                let ni = ctx.op_num_input(op);
                for j in 0..ni {
                    let vn2 = ctx.op_in(op, j);
                    if !Self::merge_test_basic(ctx, vn2) {
                        continue;
                    }
                    let vn2 = vn2.unwrap();
                    let h1 = ctx.vn_high(vn1).expect("merge_opcode: vn1 no high");
                    let h2 = ctx.vn_high(vn2).expect("merge_opcode: vn2 no high");
                    if Self::merge_test_required(ctx, h1, h2) {
                        self.merge(ctx, h1, h2, false)?; // required merge
                    }
                }
            }
        }
        Ok(())
    }

    /// Merge HighVariables of identical data-type within a Varnode range (C++
    /// `Merge::mergeByDatatype`, `merge.cc:359-401`).
    ///
    /// `range` is the location-ordered list of Varnodes for the address range (the
    /// C++ `[startiter,enditer)`).  Highs are gathered (deduped by `setMark`),
    /// grouped by exact type identity preserving discovery order, and each group is
    /// merged with [`merge_linear`](Self::merge_linear).
    pub fn merge_by_datatype(&mut self, ctx: &mut dyn MergeContext, range: &[VarnodeId]) -> KunaResult<()> {
        // Gather all highs, deduped by mark.
        let mut highlist: Vec<HighVariableId> = Vec::new();
        for &vn in range {
            if ctx.vn_is_free(vn) {
                continue;
            }
            let high = match ctx.vn_high(vn) {
                None => continue,
                Some(h) => h,
            };
            if ctx.high_is_mark(high) {
                continue; // dedup
            }
            if !Self::merge_test_basic(ctx, Some(vn)) {
                continue;
            }
            ctx.bank_set_mark(high);
            highlist.push(high);
        }
        for &h in &highlist {
            ctx.bank_clear_mark(h);
        }

        // Group by exact type (Rc identity) preserving order; each group -> mergeLinear.
        let mut remaining: std::collections::VecDeque<HighVariableId> = highlist.into_iter().collect();
        while let Some(first) = remaining.pop_front() {
            let ct = ctx.high_get_type(first);
            let mut highvec = vec![first];
            let mut rest: std::collections::VecDeque<HighVariableId> = std::collections::VecDeque::new();
            while let Some(h) = remaining.pop_front() {
                let t = ctx.high_get_type(h);
                if std::rc::Rc::ptr_eq(&ct, &t) {
                    highvec.push(h);
                } else {
                    rest.push_back(h);
                }
            }
            remaining = rest;
            self.merge_linear(ctx, &mut highvec)?;
        }
        Ok(())
    }

    /// Speculatively merge input/output Varnodes of the same op when they share a
    /// data-type (C++ `Merge::mergeAdjacent`, `merge.cc:983-1013`).
    pub fn merge_adjacent(&mut self, ctx: &mut dyn MergeContext) -> KunaResult<()> {
        let ops: Vec<OpId> = ctx.ops_alive();
        for op in ops {
            if ctx.op_is_call(op) {
                continue;
            }
            let vn1 = ctx.op_out(op);
            if !Self::merge_test_basic(ctx, vn1) {
                continue;
            }
            let vn1 = vn1.unwrap();
            let high_out = ctx.vn_high(vn1).expect("merge_adjacent: vn1 no high");
            let ct = ctx.op_output_type_local(op);
            let ni = ctx.op_num_input(op);
            for i in 0..ni {
                if !std::rc::Rc::ptr_eq(&ct, &ctx.op_input_type_local(op, i)) {
                    continue; // Only merge if types should be the same
                }
                let vn2 = ctx.op_in(op, i);
                if !Self::merge_test_basic(ctx, vn2) {
                    continue;
                }
                let vn2 = vn2.unwrap();
                if ctx.vn_size(vn1) != ctx.vn_size(vn2) {
                    continue;
                }
                if ctx.vn_def(vn2).is_none() && !ctx.vn_is_input(vn2) {
                    continue;
                }
                let high_in = ctx.vn_high(vn2).expect("merge_adjacent: vn2 no high");
                if !Self::merge_test_adjacent(ctx, high_out, high_in) {
                    continue;
                }
                if !MergeIntersect::intersection(ctx, &mut self.test_cache, high_in, high_out) {
                    self.merge(ctx, high_out, high_in, true)?;
                }
            }
        }
        Ok(())
    }

    /// Force the merge of input/output Varnodes to MULTIEQUAL and INDIRECT ops
    /// (C++ `Merge::mergeMarker`, `merge.cc:889-902`).
    pub fn merge_marker(&mut self, ctx: &mut dyn MergeContext) -> KunaResult<()> {
        let ops: Vec<OpId> = ctx.ops_alive();
        for op in ops {
            if !ctx.op_is_marker(op) || ctx.op_is_indirect_creation(op) {
                continue;
            }
            if ctx.op_code(op) == OpCode::CPUI_INDIRECT {
                self.merge_indirect(ctx, op)?;
            } else {
                self.merge_op(ctx, op)?;
            }
        }
        Ok(())
    }

    /// Run through CONCAT-tree roots and group each tree (C++
    /// `Merge::groupPartials`, `merge.cc:967-976`).
    pub fn group_partials(&mut self, ctx: &mut dyn MergeContext) -> KunaResult<()> {
        let roots = self.proto_partial.clone();
        for op in roots {
            if ctx.op_is_dead(op) {
                continue;
            }
            if !ctx.op_is_partial_root(op) {
                continue;
            }
            if let Some(out) = ctx.op_out(op) {
                self.group_partial_root(ctx, out)?;
            }
        }
        Ok(())
    }

    /// Group the nodes of one CONCAT tree into a VariableGroup (C++
    /// `Merge::groupPartialRoot`, `merge.cc:1374-1407`).
    fn group_partial_root(&mut self, ctx: &mut dyn MergeContext, vn: VarnodeId) -> KunaResult<()> {
        let high = ctx.vn_high(vn).expect("groupPartialRoot: vn no high");
        if ctx.high_num_instances(high) != 1 {
            return Ok(());
        }
        let base_offset = ctx.vn_symbol_entry_offset(vn).unwrap_or(0);
        // PieceNode::gatherPieces(pieces, vn, vn->getDef(), baseOffset, baseOffset)
        let pieces = ctx.gather_pieces(vn, base_offset);
        let mut throw_out = false;
        for (node_vn, _type_offset) in &pieces {
            let inst = match ctx.vn_high(*node_vn) {
                None => 0,
                Some(h) => ctx.high_num_instances(h),
            };
            if !ctx.vn_is_proto_partial(*node_vn) || inst != 1 {
                throw_out = true;
                break;
            }
        }
        if throw_out {
            for (node_vn, _) in &pieces {
                ctx.vn_clear_proto_partial(*node_vn);
            }
        } else {
            for (node_vn, type_offset) in &pieces {
                let node_high = ctx.vn_high(*node_vn).expect("groupPartialRoot: node no high");
                ctx.bank_group_with(node_high, *type_offset - base_offset, high)?;
            }
        }
        Ok(())
    }

    /// Force the merge of all input and output Varnodes of a generic op (C++
    /// `Merge::mergeOp`, `merge.cc:719-772`), snipping data-flow as needed.
    fn merge_op(&mut self, ctx: &mut dyn MergeContext, op: OpId) -> KunaResult<()> {
        let max = if ctx.op_code(op) == OpCode::CPUI_INDIRECT { 1 } else { ctx.op_num_input(op) };
        let out_vn = ctx.op_out(op).expect("mergeOp: op has no output");
        let high_out = ctx.vn_high(out_vn).expect("mergeOp: out no high");

        // First deal with non-cover related merge restrictions.
        for i in 0..max {
            let in_vn = ctx.op_in(op, i).expect("mergeOp: null input");
            let high_in = ctx.vn_high(in_vn).expect("mergeOp: in no high");
            if !Self::merge_test_required(ctx, high_out, high_in) {
                self.trim_op_input(ctx, op, i)?;
                continue;
            }
            for j in 0..i {
                let jvn = ctx.op_in(op, j).expect("mergeOp: null input j");
                let jhigh = ctx.vn_high(jvn).expect("mergeOp: in j no high");
                if !Self::merge_test_required(ctx, jhigh, high_in) {
                    self.trim_op_input(ctx, op, i)?;
                    break;
                }
            }
        }

        // Test if a merge violates cover restrictions.
        let mut testlist: Vec<HighVariableId> = Vec::new();
        self.merge_test(ctx, high_out, &mut testlist);
        let mut i = 0;
        while i < max {
            let ih = ctx.vn_high(ctx.op_in(op, i).unwrap()).unwrap();
            if !self.merge_test(ctx, ih, &mut testlist) {
                break;
            }
            i += 1;
        }

        if i != max {
            // Cover restrictions: trim one branch at a time and retest.
            let mut nexttrim = 0;
            while nexttrim < max {
                self.trim_op_input(ctx, op, nexttrim)?;
                testlist.clear();
                self.merge_test(ctx, high_out, &mut testlist);
                let mut i2 = 0;
                while i2 < max {
                    let ih = ctx.vn_high(ctx.op_in(op, i2).unwrap()).unwrap();
                    if !self.merge_test(ctx, ih, &mut testlist) {
                        break;
                    }
                    i2 += 1;
                }
                if i2 == max {
                    break; // successfully test-merged everything
                }
                nexttrim += 1;
            }
            if nexttrim == max {
                self.trim_op_output(ctx, op)?; // one last trim
            }
        }

        // Try to merge everything for real now.
        for i in 0..max {
            let out_h = ctx.vn_high(ctx.op_out(op).unwrap()).unwrap();
            let in_h = ctx.vn_high(ctx.op_in(op, i).unwrap()).unwrap();
            if !Self::merge_test_required(ctx, out_h, in_h) {
                return Err(KunaError::lowlevel(
                    "Non-cover related merge restriction violated, despite trims",
                ));
            }
            if !self.merge(ctx, out_h, in_h, false)? {
                return Err(KunaError::lowlevel("Unable to force merge of op"));
            }
        }
        Ok(())
    }

    /// Force the merge of input/output to an INDIRECT, handling address-forced
    /// output (C++ `Merge::mergeIndirect`, `merge.cc:846-882`).
    fn merge_indirect(&mut self, ctx: &mut dyn MergeContext, indop: OpId) -> KunaResult<()> {
        let outvn = ctx.op_out(indop).expect("mergeIndirect: no output");
        if !ctx.vn_is_addr_force(outvn) {
            // Output NOT address forced: merge like a MULTIEQUAL.
            return self.merge_op(ctx, indop);
        }
        let invn0 = ctx.op_in(indop, 0).expect("mergeIndirect: no in0");
        let out_h = ctx.vn_high(outvn).unwrap();
        let in_h = ctx.vn_high(invn0).unwrap();
        if Self::merge_test_required(ctx, out_h, in_h) && self.merge(ctx, in_h, out_h, false)? {
            return Ok(());
        }
        // The only failure mode from an input trim is if the INDIRECT output is in
        // the input to the op causing the indirect effect.  Test for that.
        if self.snip_output_interference(ctx, indop)? {
            let out_h = ctx.vn_high(outvn).unwrap();
            let in_h = ctx.vn_high(invn0).unwrap();
            if Self::merge_test_required(ctx, out_h, in_h) && self.merge(ctx, in_h, out_h, false)? {
                return Ok(());
            }
        }
        // Snip the INDIRECT itself.
        let indaddr = ctx.op_addr(indop);
        let newop = self.allocate_copy_trim(ctx, invn0, indaddr, indop)?;
        ctx.indirect_inherit_union(outvn, newop, indop);
        let newout = ctx.op_out(newop).expect("mergeIndirect: trim no out");
        ctx.op_set_input(indop, newout, 0)?;
        ctx.op_insert_before(newop, indop);
        let new_in_h = ctx.vn_high(ctx.op_in(indop, 0).unwrap()).unwrap();
        let out_h = ctx.vn_high(outvn).unwrap();
        if !Self::merge_test_required(ctx, out_h, new_in_h) || !self.merge(ctx, new_in_h, out_h, false)? {
            return Err(KunaError::lowlevel("Unable to merge address forced indirect"));
        }
        Ok(())
    }

    /// Snip output instances of an INDIRECT that are also inputs to its underlying
    /// op (C++ `Merge::snipOutputInterference`, `merge.cc:811-839`).
    fn snip_output_interference(&mut self, ctx: &mut dyn MergeContext, indop: OpId) -> KunaResult<bool> {
        let op = ctx.indirect_effect_op(indop); // getOpFromConst(getIn(1)->getAddr())
        let out_high = ctx.vn_high(ctx.op_out(indop).unwrap()).unwrap();
        let mut correctable = self.collect_inputs(ctx, out_high, op);
        if correctable.is_empty() {
            return Ok(false);
        }
        // sort(correctable, PcodeOpNode::compareByHigh)
        ctx.sort_op_nodes_by_high(&mut correctable);
        let mut cur_high: Option<HighVariableId> = None;
        let mut snip_out: Option<VarnodeId> = None;
        for node in correctable {
            let insertop = node.op;
            let slot = node.slot;
            let vn = ctx.op_in(insertop, slot).expect("snipOutputInterference: null in");
            let vn_high = ctx.vn_high(vn);
            if vn_high != cur_high {
                let insaddr = ctx.op_addr(insertop);
                let snipop = self.allocate_copy_trim(ctx, vn, insaddr, insertop)?;
                ctx.op_insert_before(snipop, insertop);
                cur_high = vn_high;
                snip_out = ctx.op_out(snipop);
            }
            ctx.op_set_input(insertop, snip_out.expect("snipOutputInterference: no snip out"), slot)?;
        }
        Ok(true)
    }

    /// Collect Varnode instances/pieces of `high` that are inputs to `op` (or an
    /// INDIRECT it causes) (C++ `Merge::collectInputs`, `merge.cc:783-802`).
    fn collect_inputs(
        &self,
        ctx: &dyn MergeContext,
        high: HighVariableId,
        op: OpId,
    ) -> Vec<crate::expression::PcodeOpNode> {
        let group = ctx.high_group_info(high).map(|g| g.group);
        let mut oplist = Vec::new();
        let mut op = op;
        loop {
            let ni = ctx.op_num_input(op);
            for i in 0..ni {
                let vn = match ctx.op_in(op, i) {
                    None => continue,
                    Some(v) => v,
                };
                if ctx.vn_is_annotation(vn) {
                    continue;
                }
                let test_high = ctx.vn_high(vn);
                let same_group = match (test_high, group) {
                    (Some(th), Some(g)) => ctx.high_group_info(th).map(|gi| gi.group) == Some(g),
                    _ => false,
                };
                if test_high == Some(high) || same_group {
                    oplist.push(crate::expression::PcodeOpNode::new(op, i));
                }
            }
            match ctx.op_previous(op) {
                Some(prev) if ctx.op_code(prev) == OpCode::CPUI_INDIRECT => op = prev,
                _ => break,
            }
        }
        oplist
    }

    /// Allocate a COPY that trims an overextended Cover (C++
    /// `Merge::allocateCopyTrim`, `merge.cc:411-434`).  Records the new COPY in
    /// `copyTrims`.
    fn allocate_copy_trim(
        &mut self,
        ctx: &mut dyn MergeContext,
        in_vn: VarnodeId,
        addr: kuna_base::address::Address,
        trim_op: OpId,
    ) -> KunaResult<OpId> {
        let copy_op = ctx.copy_trim_op(in_vn, addr, trim_op)?;
        self.copy_trims.push(copy_op);
        Ok(copy_op)
    }

    /// Trim the output HighVariable of `op` so its Cover is tiny (C++
    /// `Merge::trimOpOutput`, `merge.cc:656-682`).
    fn trim_op_output(&mut self, ctx: &mut dyn MergeContext, op: OpId) -> KunaResult<()> {
        ctx.trim_op_output(op)
    }

    /// Trim input `slot` of `op` so its Cover is tiny (C++ `Merge::trimOpInput`,
    /// `merge.cc:692-712`).
    fn trim_op_input(&mut self, ctx: &mut dyn MergeContext, op: OpId, slot: int4) -> KunaResult<()> {
        let (in_vn, pc, is_multiequal) = ctx.trim_op_input_prep(op, slot);
        let copy_op = self.allocate_copy_trim(ctx, in_vn, pc, op)?;
        let copy_out = ctx.op_out(copy_op).expect("trimOpInput: copy no out");
        ctx.op_set_input(op, copy_out, slot)?;
        if is_multiequal {
            ctx.op_insert_end_pred(copy_op, op, slot);
        } else {
            ctx.op_insert_before(copy_op, op);
        }
        Ok(())
    }

    /// Force the merge of address-tied Varnodes (C++ `Merge::mergeAddrTied`,
    /// `merge.cc:609-648`).
    ///
    /// SEAM(W7-funcdata): the body iterates `data.beginLoc()/endLoc(spc)` with
    /// `overlapLoc` and calls `mergeRangeMust`/`groupWith`; `Funcdata::overlapLoc`
    /// is not yet ported, so the iteration is driven through
    /// [`MergeContext::addr_tied_ranges`] which returns the maximally-overlapping
    /// ranges (a seam; recorded as a loss).
    pub fn merge_addr_tied(&mut self, ctx: &mut dyn MergeContext) -> KunaResult<()> {
        let ranges = ctx.addr_tied_ranges();
        for range in ranges {
            // range.bounds: Vec<Vec<VarnodeId>> sub-ranges, range.addrtied flag,
            // range.group_offsets: Vec<(HighVariableId, int4, HighVariableId)>.
            if !range.addrtied {
                continue;
            }
            self.unify_address(ctx, &range.all_varnodes)?;
            for sub in &range.must_merge_ranges {
                self.merge_range_must(ctx, sub)?;
            }
            for &(vn2_high, off, vn1_high) in &range.group_with {
                ctx.bank_group_with(vn2_high, off, vn1_high)?;
            }
        }
        Ok(())
    }

    /// Force the merge of a range of same-size/same-address Varnodes (C++
    /// `Merge::mergeRangeMust`, `merge.cc:301-317`).  Intersections must already be
    /// snipped; an error is returned if a forced merge intersects.
    fn merge_range_must(&mut self, ctx: &mut dyn MergeContext, range: &[VarnodeId]) -> KunaResult<()> {
        if range.is_empty() {
            return Ok(());
        }
        let mut iter = range.iter();
        let first = *iter.next().unwrap();
        Self::merge_test_must(ctx, first)?;
        let high = ctx.vn_high(first).expect("mergeRangeMust: first no high");
        for &vn in iter {
            let vh = ctx.vn_high(vn).expect("mergeRangeMust: vn no high");
            if vh == high {
                continue;
            }
            Self::merge_test_must(ctx, vn)?;
            if !self.merge(ctx, high, vh, false)? {
                return Err(KunaError::lowlevel("Forced merge caused intersection"));
            }
        }
        Ok(())
    }

    /// Make sure all Varnodes at one storage address can be merged, snipping
    /// discovered intersections (C++ `Merge::unifyAddress`, `merge.cc:581-601`).
    fn unify_address(&mut self, ctx: &mut dyn MergeContext, range: &[VarnodeId]) -> KunaResult<()> {
        let mut isectlist: Vec<VarnodeId> = Vec::new();
        for &vn in range {
            if ctx.vn_is_free(vn) {
                continue;
            }
            isectlist.push(vn);
        }
        let mut blocksort: Vec<BlockVarnode> = isectlist.iter().map(|&vn| BlockVarnode::new(vn, ctx)).collect();
        // stable_sort by BlockVarnode::operator< (block index)
        blocksort.sort_by(|a, b| {
            if a.less(b) {
                std::cmp::Ordering::Less
            } else if b.less(a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        for &vn in &isectlist {
            self.eliminate_intersect(ctx, vn, &blocksort)?;
        }
        Ok(())
    }

    /// Eliminate intersections of `vn` with the Varnodes in `blocksort` (C++
    /// `Merge::eliminateIntersect`, `merge.cc:489-572`).
    fn eliminate_intersect(
        &mut self,
        ctx: &mut dyn MergeContext,
        vn: VarnodeId,
        blocksort: &[BlockVarnode],
    ) -> KunaResult<()> {
        let mut markedop: Vec<OpId> = Vec::new();
        let descend = ctx.vn_descend(vn);
        for op in descend {
            let mut insertop = false;
            let single = ctx.single_read_cover(vn, op);
            // iterate single.begin()..end() — the CoverBlocks (block numbers).
            let blocks: Vec<int4> = single.iter().map(|(blk, _)| blk).collect();
            'blocks: for blocknum in blocks {
                let mut slot = BlockVarnode::find_front(blocknum, blocksort);
                if slot == -1 {
                    continue;
                }
                while (slot as usize) < blocksort.len() {
                    if blocksort[slot as usize].get_index() != blocknum {
                        break;
                    }
                    let vn2 = blocksort[slot as usize].get_varnode();
                    slot += 1;
                    if vn2 == vn {
                        continue;
                    }
                    let single_def = single_contain_varnode_def(&single, ctx, vn2);
                    let boundtype: BoundType = single_def;
                    if boundtype == 0 {
                        continue;
                    }
                    let overlaptype = ctx.vn_characterize_overlap(vn, vn2);
                    if overlaptype == 0 {
                        continue; // No overlap in storage
                    }
                    if overlaptype == 1 {
                        // cast: C++ `(int4)(vn->getOffset() - vn2->getOffset())`
                        // (merge.cc:524) — the truncating wrap to int4 is the spec.
                        let off = ctx.vn_offset(vn).wrapping_sub(ctx.vn_offset(vn2)) as int4;
                        if ctx.vn_partial_copy_shadow(vn, vn2, off) {
                            continue; // SUBPIECE shadow, not a new value
                        }
                    }
                    if boundtype == 2 {
                        // Resolve things defined at the same place.
                        match ctx.vn_def(vn2) {
                            None => match ctx.vn_def(vn) {
                                None => {
                                    if vn < vn2 {
                                        continue; // arbitrary order if both inputs
                                    }
                                }
                                Some(_) => continue,
                            },
                            Some(def2) => {
                                if let Some(def1) = ctx.vn_def(vn) {
                                    if ctx.op_order(def2) < ctx.op_order(def1) {
                                        continue;
                                    }
                                }
                            }
                        }
                    } else if boundtype == 3 {
                        // Intersection on the tail of the range (INDIRECT marking the
                        // reading op on an address-forced varnode).
                        if !ctx.vn_is_addr_force(vn2) {
                            continue;
                        }
                        if !ctx.vn_is_written(vn2) {
                            continue;
                        }
                        let indop = ctx.vn_def(vn2).unwrap();
                        if ctx.op_code(indop) != OpCode::CPUI_INDIRECT {
                            continue;
                        }
                        // The vn2 INDIRECT must be linked to the read op.
                        if op != ctx.indirect_effect_op(indop) {
                            continue;
                        }
                        let ind_in0 = ctx.op_in(indop, 0).unwrap();
                        if overlaptype != 1 {
                            if ctx.vn_copy_shadow(vn, ind_in0) {
                                continue;
                            }
                        } else {
                            // cast: C++ `(int4)(vn->getOffset() - vn2->getOffset())`
                            // (merge.cc:559) — truncating wrap to int4 is the spec.
                            let off = ctx.vn_offset(vn).wrapping_sub(ctx.vn_offset(vn2)) as int4;
                            if ctx.vn_partial_copy_shadow(vn, ind_in0, off) {
                                continue;
                            }
                        }
                    }
                    insertop = true;
                    break 'blocks; // No need to iterate further
                }
            }
            if insertop {
                markedop.push(op);
            }
        }
        self.snip_reads(ctx, vn, &markedop)
    }

    /// Snip a set of reads of `vn` by routing them through a fresh COPY (C++
    /// `Merge::snipReads`, `merge.cc:443-480`).
    fn snip_reads(&mut self, ctx: &mut dyn MergeContext, vn: VarnodeId, markedop: &[OpId]) -> KunaResult<()> {
        if markedop.is_empty() {
            return Ok(());
        }
        let (bl, pc, afterop) = ctx.snip_reads_insert_point(vn);
        let copyop = self.allocate_copy_trim(ctx, vn, pc, markedop[0])?;
        match afterop {
            None => ctx.op_insert_begin(copyop, bl),
            Some(after) => ctx.op_insert_after(copyop, after),
        }
        let copy_out = ctx.op_out(copyop).expect("snipReads: copy no out");
        for &op in markedop {
            let slot = ctx.op_slot(op, vn);
            ctx.op_set_input(op, copy_out, slot)?;
        }
        Ok(())
    }

    // --- Merge-by-copy machinery (merge.cc:1021-1436) ---------------------

    /// Find instances of `high` defined by a COPY from outside `high` (C++
    /// `Merge::findSingleCopy`, `merge.cc:1021-1036`).
    fn find_single_copy(ctx: &dyn MergeContext, high: HighVariableId, singlelist: &mut Vec<VarnodeId>) {
        let n = ctx.high_num_instances(high);
        for i in 0..n {
            let vn = ctx.high_get_instance(high, i);
            if !ctx.vn_is_written(vn) {
                continue;
            }
            let op = ctx.vn_def(vn).unwrap();
            if ctx.op_code(op) != OpCode::CPUI_COPY {
                continue;
            }
            let in0 = ctx.op_in(op, 0).unwrap();
            if ctx.vn_high(in0) == Some(high) {
                continue; // From something in same high
            }
            singlelist.push(vn);
        }
    }

    /// Hide shadow Varnodes by consolidating COPY chains (C++ `Merge::hideShadows`,
    /// `merge.cc:1070-1100`).  Returns `true` if data-flow changed.
    pub fn hide_shadows(&mut self, ctx: &mut dyn MergeContext, high: HighVariableId) -> KunaResult<bool> {
        let mut sl = Vec::new();
        Self::find_single_copy(ctx, high, &mut sl);
        let mut singlelist: Vec<Option<VarnodeId>> = sl.into_iter().map(Some).collect();
        if singlelist.len() <= 1 {
            return Ok(false);
        }
        let mut res = false;
        for i in 0..singlelist.len() - 1 {
            let vn1 = match singlelist[i] {
                None => continue,
                Some(v) => v,
            };
            // Index-based: the inner body mutates `singlelist[j] = None`, mirroring
            // the C++ `singlelist[j] = (Varnode *)0` (merge.cc:1094).
            #[allow(clippy::needless_range_loop)]
            for j in (i + 1)..singlelist.len() {
                let vn2 = match singlelist[j] {
                    None => continue,
                    Some(v) => v,
                };
                if !ctx.vn_copy_shadow(vn1, vn2) {
                    continue;
                }
                let vn2cover = ctx.vn_cover_ref(vn2).unwrap_or_default();
                if single_contain_varnode_def(&vn2cover, ctx, vn1) == 1 {
                    let def1 = ctx.vn_def(vn1).unwrap();
                    ctx.op_set_input(def1, vn2, 0)?;
                    res = true;
                    break;
                }
                let vn1cover = ctx.vn_cover_ref(vn1).unwrap_or_default();
                if single_contain_varnode_def(&vn1cover, ctx, vn2) == 1 {
                    let def2 = ctx.vn_def(vn2).unwrap();
                    ctx.op_set_input(def2, vn1, 0)?;
                    singlelist[j] = None;
                    res = true;
                }
            }
        }
        Ok(res)
    }

    /// Check if `subOp` is a redundant COPY relative to dominant `domOp` (C++
    /// `Merge::checkCopyPair`, `merge.cc:1112-1136`).
    fn check_copy_pair(
        ctx: &dyn MergeContext,
        high: HighVariableId,
        dom_op: OpId,
        sub_op: OpId,
    ) -> bool {
        if !ctx.block_dominates(ctx.op_parent(dom_op), ctx.op_parent(sub_op)) {
            return false;
        }
        let range = ctx.copy_pair_range(dom_op, sub_op);
        let in_vn = ctx.op_in(dom_op, 0).unwrap();
        let n = ctx.high_num_instances(high);
        for i in 0..n {
            let vn = ctx.high_get_instance(high, i);
            if !ctx.vn_is_written(vn) {
                continue;
            }
            let op = ctx.vn_def(vn).unwrap();
            // If the write is a COPY from the same Varnode as domOp/subOp, skip it
            // (merge.cc:1128-1129).
            if ctx.op_code(op) == OpCode::CPUI_COPY && ctx.op_in(op, 0) == Some(in_vn) {
                continue;
            }
            // range.contain(op, 1)
            let (blk, point) = ctx.op_cover_point(op);
            if range.contain(blk, point, 1) {
                return false; // intervening; subOp not redundant
            }
        }
        true
    }

    /// Replace a set of COPYs from the same Varnode with a dominant COPY (C++
    /// `Merge::buildDominantCopy`, `merge.cc:1151-1238`).
    ///
    /// SEAM(W7-funcdata): builds a new dominating COPY at `findCommonBlock`, then
    /// totalReplaces non-intersecting outputs; routed through
    /// [`MergeContext::build_dominant_copy`] which performs the IR surgery (the
    /// `FlowBlock::findCommonBlock`/`totalReplace`/`opDestroy` sequence) and reports
    /// the cover-intersection decisions back so the cover math stays here.
    fn build_dominant_copy(
        &mut self,
        ctx: &mut dyn MergeContext,
        high: HighVariableId,
        copy: &[OpId],
        pos: int4,
        size: int4,
    ) -> KunaResult<()> {
        ctx.build_dominant_copy(high, copy, pos, size)
    }

    /// Mark redundant COPYs into `high` as non-printing (C++
    /// `Merge::markRedundantCopies`, `merge.cc:1249-1265`).
    fn mark_redundant_copies(
        &mut self,
        ctx: &mut dyn MergeContext,
        high: HighVariableId,
        copy: &[OpId],
        pos: int4,
        size: int4,
    ) {
        let mut i = size - 1;
        while i > 0 {
            let sub_op = copy[(pos + i) as usize];
            if ctx.op_is_dead(sub_op) {
                i -= 1;
                continue;
            }
            let mut j = i - 1;
            while j >= 0 {
                let dom_op = copy[(pos + j) as usize];
                if ctx.op_is_dead(dom_op) {
                    j -= 1;
                    continue;
                }
                if Self::check_copy_pair(ctx, high, dom_op, sub_op) {
                    ctx.op_mark_non_printing(sub_op);
                    break;
                }
                j -= 1;
            }
            i -= 1;
        }
    }

    /// Find all COPY ops into `high` from a different high, sorted by input then
    /// block order (C++ `Merge::findAllIntoCopies`, `merge.cc:1295-1309`).
    fn find_all_into_copies(
        ctx: &dyn MergeContext,
        high: HighVariableId,
        copy_ins: &mut Vec<OpId>,
        filter_temps: bool,
    ) {
        let n = ctx.high_num_instances(high);
        for i in 0..n {
            let vn = ctx.high_get_instance(high, i);
            if !ctx.vn_is_written(vn) {
                continue;
            }
            let op = ctx.vn_def(vn).unwrap();
            if ctx.op_code(op) != OpCode::CPUI_COPY {
                continue;
            }
            let in0 = ctx.op_in(op, 0).unwrap();
            if ctx.vn_high(in0) == Some(high) {
                continue;
            }
            if filter_temps {
                let out = ctx.op_out(op).unwrap();
                if !ctx.vn_space_internal(out) {
                    continue;
                }
            }
            copy_ins.push(op);
        }
        // sort(copyIns, compareCopyByInVarnode)
        copy_ins.sort_by(|&a, &b| {
            if compare_copy_by_in_varnode(ctx, a, b) {
                std::cmp::Ordering::Less
            } else if compare_copy_by_in_varnode(ctx, b, a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    /// Replace COPYs into `high` with a single dominant COPY (C++
    /// `Merge::processHighDominantCopy`, `merge.cc:1316-1337`).
    fn process_high_dominant_copy(&mut self, ctx: &mut dyn MergeContext, high: HighVariableId) -> KunaResult<()> {
        let mut copy_ins: Vec<OpId> = Vec::new();
        Self::find_all_into_copies(ctx, high, &mut copy_ins, true);
        if copy_ins.len() < 2 {
            return Ok(());
        }
        let mut pos = 0usize;
        while pos < copy_ins.len() {
            let in_vn = ctx.op_in(copy_ins[pos], 0).unwrap();
            let mut sz = 1usize;
            while pos + sz < copy_ins.len() {
                let next_vn = ctx.op_in(copy_ins[pos + sz], 0).unwrap();
                if next_vn != in_vn {
                    break;
                }
                sz += 1;
            }
            if sz > 1 {
                self.build_dominant_copy(ctx, high, &copy_ins, pos as int4, sz as int4)?;
            }
            pos += sz;
        }
        Ok(())
    }

    /// Mark redundant COPYs into `high` (C++ `Merge::processHighRedundantCopy`,
    /// `merge.cc:1345-1367`).
    fn process_high_redundant_copy(&mut self, ctx: &mut dyn MergeContext, high: HighVariableId) {
        let mut copy_ins: Vec<OpId> = Vec::new();
        Self::find_all_into_copies(ctx, high, &mut copy_ins, false);
        if copy_ins.len() < 2 {
            return;
        }
        let mut pos = 0usize;
        while pos < copy_ins.len() {
            let in_vn = ctx.op_in(copy_ins[pos], 0).unwrap();
            let mut sz = 1usize;
            while pos + sz < copy_ins.len() {
                let next_vn = ctx.op_in(copy_ins[pos + sz], 0).unwrap();
                if next_vn != in_vn {
                    break;
                }
                sz += 1;
            }
            if sz > 1 {
                self.mark_redundant_copies(ctx, high, &copy_ins, pos as int4, sz as int4);
            }
            pos += sz;
        }
    }

    /// Reduce/eliminate COPYs produced by trimming (C++ `Merge::processCopyTrims`,
    /// `merge.cc:1415-1436`).
    pub fn process_copy_trims(&mut self, ctx: &mut dyn MergeContext) -> KunaResult<()> {
        let mut multi_copy: Vec<HighVariableId> = Vec::new();
        let trims = std::mem::take(&mut self.copy_trims);
        for op in &trims {
            let high = ctx.vn_high(ctx.op_out(*op).unwrap()).unwrap();
            let has1 = ctx.high_has_copy_in1(high);
            if !has1 {
                multi_copy.push(high);
                ctx.bank_set_copy_in1(high);
            } else {
                ctx.bank_set_copy_in2(high);
            }
        }
        for high in &multi_copy {
            let has2 = ctx.high_has_copy_in2(*high);
            if has2 {
                self.process_high_dominant_copy(ctx, *high)?;
            }
            ctx.bank_clear_copy_ins(*high);
        }
        Ok(())
    }

    /// Mark redundant/internal COPY/SUBPIECE/PIECE ops as non-printing (C++
    /// `Merge::markInternalCopies`, `merge.cc:1444-1542`).
    pub fn mark_internal_copies(&mut self, ctx: &mut dyn MergeContext) {
        let mut multi_copy: Vec<HighVariableId> = Vec::new();
        let ops: Vec<OpId> = ctx.ops_alive();
        for op in ops {
            match ctx.op_code(op) {
                OpCode::CPUI_COPY => {
                    let v1 = ctx.op_out(op).unwrap();
                    let h1 = ctx.vn_high(v1).unwrap();
                    let in0 = ctx.op_in(op, 0).unwrap();
                    if Some(h1) == ctx.vn_high(in0) {
                        ctx.op_mark_non_printing(op);
                    } else {
                        let has1 = ctx.high_has_copy_in1(h1);
                        if !has1 {
                            ctx.bank_set_copy_in1(h1);
                            multi_copy.push(h1);
                        } else {
                            ctx.bank_set_copy_in2(h1);
                        }
                        if ctx.vn_has_no_descend(v1) && Self::shadowed_varnode(ctx, v1) {
                            ctx.op_mark_non_printing(op);
                        }
                    }
                }
                OpCode::CPUI_PIECE => {
                    // Check if output is built out of pieces of itself.
                    let v1 = ctx.op_out(op).unwrap();
                    let v2 = ctx.op_in(op, 0).unwrap();
                    let v3 = ctx.op_in(op, 1).unwrap();
                    let p1 = ctx.vn_high(v1).and_then(|h| ctx.high_piece(h));
                    let p2 = ctx.vn_high(v2).and_then(|h| ctx.high_piece(h));
                    let p3 = ctx.vn_high(v3).and_then(|h| ctx.high_piece(h));
                    let (p1, p2, p3) = match (p1, p2, p3) {
                        (Some(a), Some(b), Some(c)) => (a, b, c),
                        _ => continue,
                    };
                    let (g1, o1) = (ctx.piece_group(p1), ctx.piece_offset(p1));
                    let (g2, o2) = (ctx.piece_group(p2), ctx.piece_offset(p2));
                    let (g3, o3) = (ctx.piece_group(p3), ctx.piece_offset(p3));
                    if g1 != g2 || g1 != g3 {
                        continue;
                    }
                    if ctx.vn_space_big_endian(v1) {
                        if o2 != o1 {
                            continue;
                        }
                        if o3 != o1 + ctx.vn_size(v2) {
                            continue;
                        }
                    } else {
                        if o3 != o1 {
                            continue;
                        }
                        if o2 != o1 + ctx.vn_size(v3) {
                            continue;
                        }
                    }
                    ctx.op_mark_non_printing(op);
                    if ctx.vn_is_implied(v2) {
                        ctx.vn_clear_implied_set_explicit(v2);
                    }
                    if ctx.vn_is_implied(v3) {
                        ctx.vn_clear_implied_set_explicit(v3);
                    }
                }
                OpCode::CPUI_SUBPIECE => {
                    let v1 = ctx.op_out(op).unwrap();
                    let v2 = ctx.op_in(op, 0).unwrap();
                    let p1 = ctx.vn_high(v1).and_then(|h| ctx.high_piece(h));
                    let p2 = ctx.vn_high(v2).and_then(|h| ctx.high_piece(h));
                    let (p1, p2) = match (p1, p2) {
                        (Some(a), Some(b)) => (a, b),
                        _ => continue,
                    };
                    let (g1, o1) = (ctx.piece_group(p1), ctx.piece_offset(p1));
                    let (g2, o2) = (ctx.piece_group(p2), ctx.piece_offset(p2));
                    if g1 != g2 {
                        continue;
                    }
                    let val = ctx.op_in(op, 1).map(|v| ctx.vn_offset(v)).unwrap_or(0) as int4;
                    if ctx.vn_space_big_endian(v1) {
                        if o2 + (ctx.vn_size(v2) - ctx.vn_size(v1) - val) != o1 {
                            continue;
                        }
                    } else if o2 + val != o1 {
                        continue;
                    }
                    ctx.op_mark_non_printing(op);
                    if ctx.vn_is_implied(v2) {
                        ctx.vn_clear_implied_set_explicit(v2);
                    }
                }
                _ => {}
            }
        }
        for high in multi_copy {
            let has2 = ctx.high_has_copy_in2(high);
            if has2 {
                self.process_high_redundant_copy(ctx, high);
            }
            ctx.bank_clear_copy_ins(high);
        }
    }

    /// Determine if `vn` is shadowed by another Varnode in its HighVariable (C++
    /// `Merge::shadowedVarnode`, `merge.cc:1271-1285`).
    fn shadowed_varnode(ctx: &dyn MergeContext, vn: VarnodeId) -> bool {
        let high = ctx.vn_high(vn).expect("shadowedVarnode: vn no high");
        let num = ctx.high_num_instances(high);
        let vncover = ctx.vn_cover_ref(vn).unwrap_or_default();
        for i in 0..num {
            let othervn = ctx.high_get_instance(high, i);
            if othervn == vn {
                continue;
            }
            let othercover = ctx.vn_cover_ref(othervn).unwrap_or_default();
            if vncover.intersect(&othercover) == 2 {
                return true;
            }
        }
        false
    }

    /// Merge together Varnodes mapped to SymbolEntrys of the same Symbol (C++
    /// `Merge::mergeMultiEntry`, `merge.cc:908-963`).
    ///
    /// SEAM(W4-symbol): iterates `getScopeLocal()->beginMultiEntry()` and calls
    /// `findLinkedVarnodes`; the symbol scope is a W4 surface.  Driven through
    /// [`MergeContext::multi_entry_symbols`] (a seam; recorded as a loss).
    pub fn merge_multi_entry(&mut self, ctx: &mut dyn MergeContext) -> KunaResult<()> {
        let symbols = ctx.multi_entry_symbols();
        for sym in symbols {
            let merge_list = ctx.symbol_linked_varnodes(sym);
            let (mergelist, skip_count) = merge_list;
            if mergelist.is_empty() {
                continue;
            }
            let high = ctx.vn_high(mergelist[0]).unwrap();
            MergeIntersect::update_high(ctx, &mut self.test_cache, high);
            let mut conflict_count = 0;
            let mut merge_count = 0;
            for &vn in mergelist.iter() {
                let new_high = ctx.vn_high(vn).unwrap();
                if new_high == high {
                    continue;
                }
                MergeIntersect::update_high(ctx, &mut self.test_cache, new_high);
                if !Self::merge_test_required(ctx, high, new_high) {
                    ctx.symbol_set_merge_problems(sym);
                    ctx.bank_set_unmerged(new_high);
                    conflict_count += 1;
                    continue;
                }
                if !self.merge(ctx, high, new_high, false)? {
                    ctx.symbol_set_merge_problems(sym);
                    ctx.bank_set_unmerged(new_high);
                    conflict_count += 1;
                    continue;
                }
                merge_count += 1;
            }
            if skip_count != 0 || conflict_count != 0 {
                ctx.symbol_merge_warning(sym, merge_count, skip_count, conflict_count);
            }
        }
        Ok(())
    }
}

// =============================================================================
// Free helpers (Cover containment driven through the context)
// =============================================================================

/// `cover.containVarnodeDef(vn)` (C++ `Cover::containVarnodeDef`): build the
/// def-point of `vn` and test containment.  Routed through the [`MergeContext`]
/// because `vn`'s def-point resolution needs the op/block arenas (the C++
/// `addDefPoint` of an input resolves to the function-entry point inside the
/// funcdata adapter, so `varnode_def_point` already returns that `(blk,pt)`; a
/// true `None` means "no cover point").
fn single_contain_varnode_def(cover: &Cover, ctx: &dyn MergeContext, vn: VarnodeId) -> int4 {
    let (def, _is_input) = ctx.varnode_def_point(vn);
    cover.contain_varnode_def(def)
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)] // test-fixture ergonomics
mod tests {
    use super::*;
    use crate::cover::{CoverBlock, CoverPoint, PcodeOpSet};
    use crate::dtype::{type_metatype, Datatype};
    use crate::variable::{CompareNameView, VarnodeView, VarnodeViewLoc};
    use kuna_base::address::Address;
    use slotmap::{Key, KeyData};
    use std::collections::BTreeMap;
    use std::rc::Rc;

    fn vid(n: u64) -> VarnodeId {
        VarnodeId::from(KeyData::from_ffi(n))
    }
    fn oid(n: u64) -> OpId {
        OpId::from(KeyData::from_ffi(n))
    }
    fn hid(n: u32) -> HighVariableId {
        HighVariableId(n)
    }
    fn vkey(vn: VarnodeId) -> u64 {
        vn.data().as_ffi()
    }
    fn okey(op: OpId) -> u64 {
        op.data().as_ffi()
    }
    fn dt(size: int4, meta: type_metatype) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, meta))
    }

    fn op_point(uindex: uintm) -> CoverPoint {
        CoverPoint::Op { id: oid(u64::from(uindex) + 1), uindex, code: OpCode::CPUI_COPY }
    }

    /// A one-block cover [begin..stop] on block `blk` (an instance's coverage).
    fn cover_block(blk: int4, start: uintm, stop: uintm) -> Cover {
        let mut cb = CoverBlock::new();
        cb.set_begin(Some(op_point(start)));
        cb.set_end(Some(op_point(stop)));
        // Build a Cover holding this single CoverBlock via a one-element merge.
        let mut other = OneBlockCover { blk, cb };
        let mut c = Cover::new();
        other.write_into(&mut c);
        c
    }

    /// Helper that writes a single CoverBlock into a Cover (Cover has no public
    /// `addCoverBlock`, so we use a def/ref-point pair on the block to seed it).
    struct OneBlockCover {
        blk: int4,
        cb: CoverBlock,
    }
    impl OneBlockCover {
        fn write_into(&mut self, c: &mut Cover) {
            // Seed the block via add_def_point at the begin point, then extend to
            // the stop via merge of a second def cover on the same block.
            c.add_def_point(Some((self.blk, self.cb.get_start().unwrap_or(CoverPoint::Begin))), false);
            // Manually union the explicit block geometry so contain/intersect see
            // exactly [start,stop].
            let mut whole = Cover::new();
            whole.add_def_point(Some((self.blk, self.cb.get_stop().unwrap_or(CoverPoint::End))), false);
            c.merge(&whole);
        }
    }

    /// Per-high test state.
    #[derive(Clone, Default)]
    struct HighState {
        type_: Option<Rc<Datatype>>,
        type_lock: bool,
        addr_tied: bool,
        input: bool,
        persist: bool,
        extra_out: bool,
        proto_partial: bool,
        name_lock: bool,
        has_cover: bool,
        cover_dirty: bool,
        mark: bool,
        copy_in1: bool,
        copy_in2: bool,
        unmerged: bool,
        symbol: Option<u64>,
        symbol_offset: int4,
        instances: Vec<VarnodeId>,
        cover: Cover,
        tied_addr: Address,
    }

    /// A minimal in-memory `MergeContext` for the pure-logic unit tests.  Only the
    /// methods the tests reach are meaningful; IR-mutation seams panic if reached.
    struct Mock {
        highs: BTreeMap<u32, HighState>,
        vn_high: BTreeMap<u64, HighVariableId>,
        vn_def: BTreeMap<u64, OpId>,
        vn_addr: BTreeMap<u64, Address>,
        vn_create: BTreeMap<u64, u64>,
        op_addr: BTreeMap<u64, Address>,
        op_parent_idx: BTreeMap<u64, int4>,
        op_order: BTreeMap<u64, uintm>,
        op_in0: BTreeMap<u64, VarnodeId>,
        copy_shadow: bool,
        merged: Vec<(HighVariableId, HighVariableId, bool)>,
    }

    impl Mock {
        fn new() -> Mock {
            Mock {
                highs: BTreeMap::new(),
                vn_high: BTreeMap::new(),
                vn_def: BTreeMap::new(),
                vn_addr: BTreeMap::new(),
                vn_create: BTreeMap::new(),
                op_addr: BTreeMap::new(),
                op_parent_idx: BTreeMap::new(),
                op_order: BTreeMap::new(),
                op_in0: BTreeMap::new(),
                copy_shadow: false,
                merged: Vec::new(),
            }
        }
        fn hs(&self, h: HighVariableId) -> &HighState {
            self.highs.get(&h.0).expect("mock: missing high")
        }
        fn hs_mut(&mut self, h: HighVariableId) -> &mut HighState {
            self.highs.get_mut(&h.0).expect("mock: missing high")
        }
    }

    // HighContext is required as a supertrait; the merge tests don't drive the
    // bank's lazy re-derivation (flags come straight off HighState), so these are
    // satisfied minimally.
    impl HighContext for Mock {
        fn vn_view(&self, _vn: VarnodeId) -> VarnodeView {
            VarnodeView {
                flags: 0,
                size: 0,
                type_: dt(1, type_metatype::TYPE_UNKNOWN),
                type_lock: false,
                merge_group: 0,
                written: false,
                def_time: 0,
                space_internal: false,
                create_index: 0,
            }
        }
        fn vn_cover(&self, _vn: VarnodeId) -> Option<Cover> {
            None
        }
        fn vn_has_cover(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_name_view(&self, _vn: VarnodeId) -> CompareNameView {
            CompareNameView {
                name_lock: false,
                unaffected: false,
                persist: false,
                input: false,
                addr_tied: false,
                proto_partial: false,
                space_internal: false,
                written: false,
                def_time: 0,
            }
        }
        fn vn_loc_view(&self, vn: VarnodeId) -> VarnodeViewLoc {
            let addr = self
                .vn_addr
                .get(&vkey(vn))
                .cloned()
                .unwrap_or_else(Address::new_invalid);
            VarnodeViewLoc { addr }
        }
    }

    impl MergeContext for Mock {
        fn high_is_type_lock(&mut self, h: HighVariableId) -> bool {
            self.hs(h).type_lock
        }
        fn high_get_type(&mut self, h: HighVariableId) -> Rc<Datatype> {
            self.hs(h).type_.clone().unwrap_or_else(|| dt(1, type_metatype::TYPE_UNKNOWN))
        }
        fn high_is_addr_tied(&mut self, h: HighVariableId) -> bool {
            self.hs(h).addr_tied
        }
        fn high_is_input(&mut self, h: HighVariableId) -> bool {
            self.hs(h).input
        }
        fn high_is_persist(&mut self, h: HighVariableId) -> bool {
            self.hs(h).persist
        }
        fn high_is_extra_out(&mut self, h: HighVariableId) -> bool {
            self.hs(h).extra_out
        }
        fn high_is_proto_partial(&mut self, h: HighVariableId) -> bool {
            self.hs(h).proto_partial
        }
        fn high_is_name_lock(&mut self, h: HighVariableId) -> bool {
            self.hs(h).name_lock
        }
        fn high_has_cover(&mut self, h: HighVariableId) -> bool {
            self.hs(h).has_cover
        }
        fn high_is_cover_dirty(&self, h: HighVariableId) -> bool {
            self.hs(h).cover_dirty
        }
        fn high_is_mark(&self, h: HighVariableId) -> bool {
            self.hs(h).mark
        }
        fn high_has_copy_in1(&self, h: HighVariableId) -> bool {
            self.hs(h).copy_in1
        }
        fn high_has_copy_in2(&self, h: HighVariableId) -> bool {
            self.hs(h).copy_in2
        }
        fn high_num_instances(&self, h: HighVariableId) -> int4 {
            self.hs(h).instances.len() as int4
        }
        fn high_get_instance(&self, h: HighVariableId, i: int4) -> VarnodeId {
            self.hs(h).instances[i as usize]
        }
        fn high_get_cover(&self, h: HighVariableId) -> Cover {
            self.hs(h).cover.clone()
        }
        fn high_internal_cover(&self, h: HighVariableId) -> Cover {
            self.hs(h).cover.clone()
        }
        fn bank_update_cover_for(&mut self, h: HighVariableId) {
            self.hs_mut(h).cover_dirty = false;
        }
        fn high_group_info(&self, _h: HighVariableId) -> Option<HighGroupInfo> {
            None
        }
        fn high_piece(&self, _h: HighVariableId) -> Option<MergePieceId> {
            None
        }
        fn piece_offset(&self, _p: MergePieceId) -> int4 {
            0
        }
        fn piece_size(&self, _p: MergePieceId) -> int4 {
            0
        }
        fn piece_group(&self, _p: MergePieceId) -> u64 {
            0
        }
        fn piece_high(&self, _p: MergePieceId) -> HighVariableId {
            hid(0)
        }
        fn piece_num_intersection(&self, _p: MergePieceId) -> int4 {
            0
        }
        fn piece_intersection(&self, _p: MergePieceId, _i: int4) -> MergePieceId {
            MergePieceId(0)
        }
        fn bank_update_piece_intersections(&mut self, _p: MergePieceId) {}
        fn bank_merge_highs(
            &mut self,
            high1: HighVariableId,
            high2: HighVariableId,
            isspeculative: bool,
            _cache: &mut HighIntersectTest,
        ) -> KunaResult<()> {
            self.merged.push((high1, high2, isspeculative));
            // Move high2's instances into high1, union covers, drop high2.
            let h2 = self.highs.remove(&high2.0).expect("merge: missing high2");
            let h1 = self.hs_mut(high1);
            h1.instances.extend(h2.instances);
            h1.cover.merge(&h2.cover);
            Ok(())
        }
        fn bank_update_cover(&mut self, h: HighVariableId) {
            self.hs_mut(h).cover_dirty = false;
        }
        fn bank_group_with(&mut self, _h2: HighVariableId, _off: int4, _h1: HighVariableId) -> KunaResult<()> {
            Ok(())
        }
        fn bank_set_mark(&mut self, h: HighVariableId) {
            self.hs_mut(h).mark = true;
        }
        fn bank_clear_mark(&mut self, h: HighVariableId) {
            self.hs_mut(h).mark = false;
        }
        fn bank_set_unmerged(&mut self, h: HighVariableId) {
            self.hs_mut(h).unmerged = true;
        }
        fn bank_set_copy_in1(&mut self, h: HighVariableId) {
            self.hs_mut(h).copy_in1 = true;
        }
        fn bank_set_copy_in2(&mut self, h: HighVariableId) {
            self.hs_mut(h).copy_in2 = true;
        }
        fn bank_clear_copy_ins(&mut self, h: HighVariableId) {
            let s = self.hs_mut(h);
            s.copy_in1 = false;
            s.copy_in2 = false;
        }
        fn bank_symbol(&self, h: HighVariableId) -> Option<u64> {
            self.hs(h).symbol
        }
        fn bank_symbol_offset(&self, h: HighVariableId) -> int4 {
            self.hs(h).symbol_offset
        }
        fn bank_symbol_isolated(&self, _h: HighVariableId) -> bool {
            false
        }
        fn bank_tied_addr(&self, h: HighVariableId) -> Address {
            self.hs(h).tied_addr.clone()
        }
        fn bank_tied_varnode(&self, _h: HighVariableId) -> Option<VarnodeId> {
            None
        }
        fn bank_input_varnode(&self, _h: HighVariableId) -> Option<VarnodeId> {
            None
        }
        fn vn_high(&self, vn: VarnodeId) -> Option<HighVariableId> {
            self.vn_high.get(&vkey(vn)).copied()
        }
        fn vn_def(&self, vn: VarnodeId) -> Option<OpId> {
            self.vn_def.get(&vkey(vn)).copied()
        }
        fn vn_is_free(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_input(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_written(&self, vn: VarnodeId) -> bool {
            self.vn_def.contains_key(&vkey(vn))
        }
        fn vn_is_addr_force(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_implied(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_spacebase(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_proto_partial(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_annotation(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_illegal_input(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_is_indirect_only(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_has_no_local_alias(&self, _vn: VarnodeId) -> bool {
            true
        }
        fn vn_has_no_descend(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_space_big_endian(&self, _vn: VarnodeId) -> bool {
            false
        }
        fn vn_space_internal(&self, _vn: VarnodeId) -> bool {
            true
        }
        fn vn_has_cover_flag(&self, _vn: VarnodeId) -> bool {
            true
        }
        fn vn_size(&self, _vn: VarnodeId) -> int4 {
            4
        }
        fn vn_offset(&self, _vn: VarnodeId) -> u64 {
            0
        }
        fn vn_cover_ref(&self, _vn: VarnodeId) -> Option<Cover> {
            None
        }
        fn vn_create_index(&self, vn: VarnodeId) -> u64 {
            self.vn_create.get(&vkey(vn)).copied().unwrap_or(0)
        }
        fn vn_symbol_entry_offset(&self, _vn: VarnodeId) -> Option<int4> {
            None
        }
        fn vn_copy_shadow(&self, _a: VarnodeId, _b: VarnodeId) -> bool {
            self.copy_shadow
        }
        fn vn_partial_copy_shadow(&self, _a: VarnodeId, _b: VarnodeId, _off: int4) -> bool {
            false
        }
        fn vn_characterize_overlap(&self, _a: VarnodeId, _b: VarnodeId) -> int4 {
            2
        }
        fn vn_set_implied(&mut self, _vn: VarnodeId) {}
        fn vn_set_cover_dirty(&mut self, _vn: VarnodeId) {}
        fn vn_clear_implied_set_explicit(&mut self, _vn: VarnodeId) {}
        fn vn_clear_proto_partial(&mut self, _vn: VarnodeId) {}
        fn varnode_def_point(&self, _vn: VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
            (None, false)
        }
        fn vn_descend(&self, _vn: VarnodeId) -> Vec<OpId> {
            Vec::new()
        }
        fn op_code(&self, _op: OpId) -> OpCode {
            OpCode::CPUI_COPY
        }
        fn op_out(&self, _op: OpId) -> Option<VarnodeId> {
            None
        }
        fn op_in(&self, op: OpId, slot: int4) -> Option<VarnodeId> {
            if slot == 0 {
                self.op_in0.get(&okey(op)).copied()
            } else {
                None
            }
        }
        fn op_num_input(&self, _op: OpId) -> int4 {
            0
        }
        fn op_parent(&self, _op: OpId) -> crate::seams::BlockId {
            crate::seams::BlockId::from(KeyData::from_ffi(1))
        }
        fn op_parent_index(&self, op: OpId) -> int4 {
            self.op_parent_idx.get(&okey(op)).copied().unwrap_or(0)
        }
        fn op_addr(&self, op: OpId) -> Address {
            self.op_addr.get(&okey(op)).cloned().unwrap_or_else(Address::new_invalid)
        }
        fn op_order(&self, op: OpId) -> uintm {
            self.op_order.get(&okey(op)).copied().unwrap_or(0)
        }
        fn op_slot(&self, _op: OpId, _vn: VarnodeId) -> int4 {
            0
        }
        fn op_is_dead(&self, _op: OpId) -> bool {
            false
        }
        fn op_is_call(&self, _op: OpId) -> bool {
            false
        }
        fn op_is_marker(&self, _op: OpId) -> bool {
            false
        }
        fn op_is_indirect_creation(&self, _op: OpId) -> bool {
            false
        }
        fn op_is_partial_root(&self, _op: OpId) -> bool {
            false
        }
        fn op_previous(&self, _op: OpId) -> Option<OpId> {
            None
        }
        fn op_output_type_local(&self, _op: OpId) -> Rc<Datatype> {
            dt(1, type_metatype::TYPE_UNKNOWN)
        }
        fn op_input_type_local(&self, _op: OpId, _slot: int4) -> Rc<Datatype> {
            dt(1, type_metatype::TYPE_UNKNOWN)
        }
        fn op_cover_point(&self, _op: OpId) -> (int4, CoverPoint) {
            (0, CoverPoint::Begin)
        }
        fn indirect_effect_op(&self, _indop: OpId) -> OpId {
            oid(0)
        }
        fn op_set_input(&mut self, _op: OpId, _vn: VarnodeId, _slot: int4) -> KunaResult<()> {
            Ok(())
        }
        fn op_insert_before(&mut self, _op: OpId, _follow: OpId) {}
        fn op_insert_after(&mut self, _op: OpId, _prev: OpId) {}
        fn op_insert_begin(&mut self, _op: OpId, _bl: crate::seams::BlockId) {}
        fn op_insert_end_pred(&mut self, _copyop: OpId, _op: OpId, _slot: int4) {}
        fn op_mark_non_printing(&mut self, _op: OpId) {}
        fn bblocks_get_size(&self) -> int4 {
            0
        }
        fn bblocks_get_block(&self, _i: int4) -> crate::seams::BlockId {
            crate::seams::BlockId::from(KeyData::from_ffi(1))
        }
        fn block_ops(&self, _bl: crate::seams::BlockId) -> Vec<OpId> {
            Vec::new()
        }
        fn block_dominates(&self, _dom: crate::seams::BlockId, _sub: crate::seams::BlockId) -> bool {
            false
        }
        fn ops_alive(&self) -> Vec<OpId> {
            Vec::new()
        }
        fn single_read_cover(&self, _vn: VarnodeId, _op: OpId) -> Cover {
            Cover::new()
        }
        fn copy_pair_range(&self, _dom_op: OpId, _sub_op: OpId) -> Cover {
            Cover::new()
        }
        fn copy_trim_op(&mut self, _in_vn: VarnodeId, _addr: Address, _trim_op: OpId) -> KunaResult<OpId> {
            panic!("mock: copy_trim_op not under test")
        }
        fn trim_op_output(&mut self, _op: OpId) -> KunaResult<()> {
            panic!("mock: trim_op_output not under test")
        }
        fn trim_op_input_prep(&self, _op: OpId, _slot: int4) -> (VarnodeId, Address, bool) {
            panic!("mock: trim_op_input_prep not under test")
        }
        fn snip_reads_insert_point(&self, _vn: VarnodeId) -> (crate::seams::BlockId, Address, Option<OpId>) {
            panic!("mock: snip_reads_insert_point not under test")
        }
        fn indirect_inherit_union(&mut self, _outvn: VarnodeId, _newop: OpId, _indop: OpId) {}
        fn sort_op_nodes_by_high(&self, _nodes: &mut Vec<crate::expression::PcodeOpNode>) {}
        fn build_dominant_copy(&mut self, _high: HighVariableId, _copy: &[OpId], _pos: int4, _size: int4) -> KunaResult<()> {
            panic!("mock: build_dominant_copy not under test")
        }
        fn addr_tied_ranges(&self) -> Vec<AddrTiedRange> {
            Vec::new()
        }
        fn multi_entry_symbols(&self) -> Vec<u64> {
            Vec::new()
        }
        fn symbol_linked_varnodes(&self, _symbol: u64) -> (Vec<VarnodeId>, int4) {
            (Vec::new(), 0)
        }
        fn symbol_set_merge_problems(&mut self, _symbol: u64) {}
        fn symbol_merge_warning(&mut self, _symbol: u64, _m: int4, _s: int4, _c: int4) {}
        fn populate_affecting_ops(&self, _op_set: &mut PcodeOpSet) {}
        fn gather_pieces(&self, _vn: VarnodeId, _base_offset: int4) -> Vec<(VarnodeId, int4)> {
            Vec::new()
        }
    }

    fn add_high(m: &mut Mock, n: u32, mut s: HighState) {
        // Default a type so getType() comparisons are well-defined.
        if s.type_.is_none() {
            s.type_ = Some(dt(4, type_metatype::TYPE_INT));
        }
        if s.tied_addr == Address::default() {
            s.tied_addr = Address::new_invalid();
        }
        m.highs.insert(n, s);
    }

    // --- BlockVarnode::findFront (merge.cc:43-61) --------------------------

    #[test]
    fn block_varnode_find_front_binary_search() {
        // Sorted list of (index) blocks: 0,2,2,5,5,5,9
        let mk = |idx: int4| BlockVarnode { index: idx, vn: vid(1) };
        let list = vec![mk(0), mk(2), mk(2), mk(5), mk(5), mk(5), mk(9)];
        assert_eq!(BlockVarnode::find_front(0, &list), 0);
        assert_eq!(BlockVarnode::find_front(2, &list), 1); // first of the 2s
        assert_eq!(BlockVarnode::find_front(5, &list), 3); // first of the 5s
        assert_eq!(BlockVarnode::find_front(9, &list), 6);
        assert_eq!(BlockVarnode::find_front(3, &list), -1); // absent block
        assert_eq!(BlockVarnode::find_front(10, &list), -1); // past the end
    }

    // --- compareCopyByInVarnode (merge.cc:1045-1057) -----------------------

    #[test]
    fn compare_copy_by_in_varnode_order() {
        // Different inputs: lower createIndex sorts first (merge.cc:1050-1051).
        let mut m = Mock::new();
        m.op_in0.insert(okey(oid(1)), vid(100));
        m.op_in0.insert(okey(oid(2)), vid(200));
        m.vn_create.insert(vkey(vid(100)), 10);
        m.vn_create.insert(vkey(vid(200)), 20);
        assert!(compare_copy_by_in_varnode(&m, oid(1), oid(2)));
        assert!(!compare_copy_by_in_varnode(&m, oid(2), oid(1)));

        // Same input: tie-break on block index (merge.cc:1052-1055).
        let mut m2 = Mock::new();
        m2.op_in0.insert(okey(oid(1)), vid(100));
        m2.op_in0.insert(okey(oid(2)), vid(100));
        m2.vn_create.insert(vkey(vid(100)), 5);
        m2.op_parent_idx.insert(okey(oid(1)), 1);
        m2.op_parent_idx.insert(okey(oid(2)), 3);
        assert!(compare_copy_by_in_varnode(&m2, oid(1), oid(2)));

        // Same input, same block: SeqNum order tie-break (merge.cc:1056).
        let mut m3 = Mock::new();
        m3.op_in0.insert(okey(oid(1)), vid(100));
        m3.op_in0.insert(okey(oid(2)), vid(100));
        m3.vn_create.insert(vkey(vid(100)), 5);
        m3.op_parent_idx.insert(okey(oid(1)), 2);
        m3.op_parent_idx.insert(okey(oid(2)), 2);
        m3.op_order.insert(okey(oid(1)), 10);
        m3.op_order.insert(okey(oid(2)), 20);
        assert!(compare_copy_by_in_varnode(&m3, oid(1), oid(2)));
        assert!(!compare_copy_by_in_varnode(&m3, oid(2), oid(1)));
    }

    // --- compareHighByBlock (merge.hh:152-174) -----------------------------

    #[test]
    fn compare_high_by_block_cover_then_addr() {
        let mut m = Mock::new();
        // high 1 covers block 0; high 2 covers block 1 -> high1 orders first
        // (compareTo on covers: earlier block first).
        let mut s1 = HighState::default();
        s1.instances = vec![vid(1)];
        s1.cover = cover_block(0, 0, 5);
        let mut s2 = HighState::default();
        s2.instances = vec![vid(2)];
        s2.cover = cover_block(1, 0, 5);
        add_high(&mut m, 1, s1);
        add_high(&mut m, 2, s2);
        m.vn_addr.insert(vkey(vid(1)), Address::new_invalid());
        m.vn_addr.insert(vkey(vid(2)), Address::new_invalid());
        assert!(compare_high_by_block(&m, hid(1), hid(2)));
        assert!(!compare_high_by_block(&m, hid(2), hid(1)));
    }

    // --- mergeTestRequired short-circuits (merge.cc:102-166) ---------------

    #[test]
    fn merge_test_required_same_high_is_true() {
        let mut m = Mock::new();
        add_high(&mut m, 1, HighState::default());
        assert!(Merge::merge_test_required(&mut m, hid(1), hid(1)));
    }

    #[test]
    fn merge_test_required_typelock_mismatch_forbidden() {
        let mut m = Mock::new();
        let mut a = HighState::default();
        a.type_lock = true;
        a.type_ = Some(dt(4, type_metatype::TYPE_INT));
        let mut b = HighState::default();
        b.type_lock = true;
        b.type_ = Some(dt(4, type_metatype::TYPE_UINT)); // different Rc
        add_high(&mut m, 1, a);
        add_high(&mut m, 2, b);
        assert!(!Merge::merge_test_required(&mut m, hid(1), hid(2)));
    }

    #[test]
    fn merge_test_required_input_persist_forbidden() {
        let mut m = Mock::new();
        let mut input = HighState::default();
        input.input = true;
        let mut persist = HighState::default();
        persist.persist = true;
        add_high(&mut m, 1, persist); // high_out persist
        add_high(&mut m, 2, input); // high_in input
        // high_in input && high_out persist -> false
        assert!(!Merge::merge_test_required(&mut m, hid(1), hid(2)));
    }

    #[test]
    fn merge_test_required_extra_out_forbidden() {
        let mut m = Mock::new();
        let mut a = HighState::default();
        let mut b = HighState::default();
        b.extra_out = true;
        add_high(&mut m, 1, a.clone());
        add_high(&mut m, 2, b);
        // high_in not input, is extraOut -> false
        assert!(!Merge::merge_test_required(&mut m, hid(1), hid(2)));
        a.extra_out = false;
    }

    #[test]
    fn merge_test_required_different_symbol_forbidden() {
        let mut m = Mock::new();
        let mut a = HighState::default();
        a.symbol = Some(7);
        let mut b = HighState::default();
        b.symbol = Some(9);
        add_high(&mut m, 1, a);
        add_high(&mut m, 2, b);
        assert!(!Merge::merge_test_required(&mut m, hid(1), hid(2)));
    }

    #[test]
    fn merge_test_required_same_symbol_diff_offset_forbidden() {
        let mut m = Mock::new();
        let mut a = HighState::default();
        a.symbol = Some(7);
        a.symbol_offset = 0;
        let mut b = HighState::default();
        b.symbol = Some(7);
        b.symbol_offset = 4;
        add_high(&mut m, 1, a);
        add_high(&mut m, 2, b);
        assert!(!Merge::merge_test_required(&mut m, hid(1), hid(2)));
    }

    #[test]
    fn merge_test_required_compatible_allowed() {
        let mut m = Mock::new();
        add_high(&mut m, 1, HighState::default());
        add_high(&mut m, 2, HighState::default());
        assert!(Merge::merge_test_required(&mut m, hid(1), hid(2)));
    }

    // --- mergeTestSpeculative (merge.cc:220-235) ---------------------------

    #[test]
    fn merge_test_speculative_rejects_addrtied_and_input() {
        let mut m = Mock::new();
        add_high(&mut m, 1, HighState::default());
        let mut tied = HighState::default();
        tied.addr_tied = true;
        add_high(&mut m, 2, tied);
        // adjacency passes (same type), but speculative rejects addr-tied
        assert!(!Merge::merge_test_speculative(&mut m, hid(1), hid(2)));

        let mut m2 = Mock::new();
        add_high(&mut m2, 1, HighState::default());
        let mut inp = HighState::default();
        inp.input = true;
        add_high(&mut m2, 2, inp);
        assert!(!Merge::merge_test_speculative(&mut m2, hid(1), hid(2)));
    }

    // --- mergeTestMust / mergeTestBasic (merge.cc:241-264) -----------------

    #[test]
    fn merge_test_basic_rejects_no_cover() {
        let m = Mock::new();
        // vn_has_cover_flag is true in mock; but None vn rejected
        assert!(!Merge::merge_test_basic(&m, None));
        assert!(Merge::merge_test_basic(&m, Some(vid(1))));
    }

    // --- MergeIntersect cache: updateHigh purges + intersection caches -----

    #[test]
    fn merge_intersect_update_high_returns_clean_flag() {
        let mut m = Mock::new();
        let mut s = HighState::default();
        s.cover_dirty = false;
        add_high(&mut m, 1, s);
        let mut cache = HighIntersectTest::new(PcodeOpSet::new(
            Box::new(Vec::new),
            Box::new(|_, _| true),
        ));
        // not dirty -> returns true (was not originally dirty)
        assert!(MergeIntersect::update_high(&mut m, &mut cache, hid(1)));
        // mark dirty -> returns false and clears the dirty flag
        m.hs_mut(hid(1)).cover_dirty = true;
        assert!(!MergeIntersect::update_high(&mut m, &mut cache, hid(1)));
        assert!(!m.hs(hid(1)).cover_dirty);
    }

    #[test]
    fn merge_intersect_disjoint_covers_do_not_intersect() {
        let mut m = Mock::new();
        let mut a = HighState::default();
        a.cover = cover_block(0, 0, 5);
        a.instances = vec![vid(1)];
        let mut b = HighState::default();
        b.cover = cover_block(3, 0, 5); // different block -> no >=2 overlap
        b.instances = vec![vid(2)];
        add_high(&mut m, 1, a);
        add_high(&mut m, 2, b);
        let mut cache = HighIntersectTest::new(PcodeOpSet::new(
            Box::new(Vec::new),
            Box::new(|_, _| true),
        ));
        assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(2)));
        // The result is cached symmetrically.
        assert_eq!(cache.cached(hid(1), hid(2)), Some(false));
        assert_eq!(cache.cached(hid(2), hid(1)), Some(false));
    }

    #[test]
    fn merge_intersect_same_high_never_intersects() {
        let mut m = Mock::new();
        add_high(&mut m, 1, HighState::default());
        let mut cache = HighIntersectTest::new(PcodeOpSet::new(
            Box::new(Vec::new),
            Box::new(|_, _| true),
        ));
        assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(1)));
    }

    // --- Merge::merge backout on intersection (merge.cc:1565-1575) ---------

    #[test]
    fn merge_backout_on_cover_intersection() {
        let mut m = Mock::new();
        // Two highs whose covers intersect on block 0 (overlapping ranges) and
        // whose instances are NOT copy shadows -> blockIntersection true.
        let mut a = HighState::default();
        a.cover = overlap_cover(0);
        a.instances = vec![vid(1)];
        let mut b = HighState::default();
        b.cover = overlap_cover(0);
        b.instances = vec![vid(2)];
        add_high(&mut m, 1, a);
        add_high(&mut m, 2, b);
        m.copy_shadow = false; // not shadows -> a real intersection
        // give the instance covers so testBlockIntersection sees the overlap
        // (vn_cover_ref drives it); make both instances cover block 0 fully.
        let mut merge = Merge::new(HighIntersectTest::new(PcodeOpSet::new(
            Box::new(Vec::new),
            Box::new(|_, _| true),
        )));
        // With disjoint instance covers (vn_cover_ref None in mock) the block test
        // finds no instance-level overlap, so the merge SUCCEEDS; this exercises
        // the success path and the bank_merge_highs hook.
        let ok = merge.merge(&mut m, hid(1), hid(2), false).unwrap();
        assert!(ok);
        // high2 was consumed into high1.
        assert!(m.highs.contains_key(&1));
        assert!(!m.highs.contains_key(&2));
        assert_eq!(m.merged, vec![(hid(1), hid(2), false)]);
    }

    /// An instance cover that occupies block 0 with a small range (for the
    /// intersection-list level-2 test).
    fn overlap_cover(blk: int4) -> Cover {
        cover_block(blk, 0, 100)
    }

    // --- mergeLinear speculative ordering + backout (merge.cc:272-292) -----

    #[test]
    fn merge_linear_merges_compatible_disjoint_highs() {
        let mut m = Mock::new();
        // Three highs, same type (shared Rc so the adjacency type test passes),
        // disjoint instance covers -> all merge into the earliest-block high.
        let shared = dt(4, type_metatype::TYPE_INT);
        for n in 1..=3u32 {
            let mut s = HighState::default();
            s.has_cover = true;
            s.type_ = Some(shared.clone());
            s.cover = cover_block(n as int4 - 1, 0, 5);
            s.instances = vec![vid(n as u64)];
            add_high(&mut m, n, s);
            m.vn_addr.insert(vkey(vid(n as u64)), Address::new_invalid());
        }
        let mut merge = Merge::new(HighIntersectTest::new(PcodeOpSet::new(
            Box::new(Vec::new),
            Box::new(|_, _| true),
        )));
        let mut highvec = vec![hid(1), hid(2), hid(3)];
        merge.merge_linear(&mut m, &mut highvec).unwrap();
        // All three speculatively merged into the earliest-block high (hid 1).
        assert_eq!(m.merged.len(), 2);
        for (_, _, spec) in &m.merged {
            assert!(*spec, "mergeLinear merges must be speculative");
        }
    }

    // --- Merge::clear resets the cache + trim/partial lists ----------------

    #[test]
    fn merge_clear_empties_state() {
        let mut merge = Merge::new(HighIntersectTest::new(PcodeOpSet::new(
            Box::new(Vec::new),
            Box::new(|_, _| true),
        )));
        merge.copy_trims.push(oid(1));
        merge.proto_partial.push(oid(2));
        merge.test_cache.cache_result(hid(1), hid(2), true);
        merge.clear();
        assert!(merge.copy_trims.is_empty());
        assert!(merge.proto_partial.is_empty());
        assert_eq!(merge.test_cache.cached(hid(1), hid(2)), None);
    }

}
