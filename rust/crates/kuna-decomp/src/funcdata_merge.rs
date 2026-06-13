//! The `Funcdata` <-> `Merge`/`HighVariable` bridge (C++ `Funcdata::covermerge`
//! and the `Merge` engine's access to the live `Funcdata`).
//!
//! The standalone [`Merge`](crate::merge::Merge) engine (S6/S7 HighVariable
//! formation) drives all of its `Funcdata` reads/mutations through the
//! [`MergeContext`](crate::merge::MergeContext) trait (which extends
//! [`HighContext`](crate::variable::HighContext)).  This file implements both
//! traits for [`Funcdata`], so `ActionMergeRequired` and friends can run the real
//! `mergeAddrTied`/`groupPartials`/`mergeMarker` passes over the analyzed IR.
//!
//! Faithfulness map (the C++ method each trait method transcribes):
//!  * the `high_*` / `bank_*` reads -> `HighVariable::is*`/`getType`/`numInstances`
//!    etc., re-derived lazily by the bank (`with_high_split` for the field-split
//!    borrow of `high_bank` vs. the `vbank`/`obank` read view);
//!  * the `vn_*` / `op_*` reads -> the corresponding `Varnode::*` / `PcodeOp::*`
//!    accessors on the live banks;
//!  * the IR-surgery mutators (`copy_trim_op`, `trim_op_*`, `op_insert_*`) ->
//!    `Funcdata::newOp`/`newUnique`/`opSet*`/`opInsert*` (`merge.cc:411-712`).
//!
//! ## Seams (documented losses, narrow on the merged-tree default)
//!  * The W4 Symbol/ScopeLocal layer is not present (`localmap` is a unit stub):
//!    `bank_symbol`/`multi_entry_symbols`/`symbol_*` return the un-recovered
//!    default (no symbol, no multi-entry groups), so `mergeMultiEntry` is a true
//!    no-op and merged locals are named directly on the HighVariable
//!    (`kuna_naming`), not via a `Symbol` in the local scope.
//!  * `addr_tied_ranges` (`Funcdata::overlapLoc`) and the union-resolution arms of
//!    the trim COPYs are the conservative default (no addr-tied overlap groups, no
//!    `needsResolution` types in the merged tree) — exact on boolless.
//!  * `build_dominant_copy` / `populate_affecting_ops` / the partial-copy-shadow
//!    characterization stay the conservative path (W7 `findCommonBlock`/
//!    `StackAffectingOps`); none are reached on the MULTIEQUAL/COPY merge path.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::cover::{Cover, CoverPoint, PcodeOpSet};
use crate::dtype::Datatype;
use crate::expression::PcodeOpNode;
use crate::funcdata::Funcdata;
use crate::merge::{AddrTiedRange, HighGroupInfo, MergeContext, MergePieceId};
use crate::seams::{BlockId, HighVariableId, OpId, VarnodeId};
use crate::variable::{
    CompareNameView, HighContext, HighIntersectTest, VarnodeView, VarnodeViewLoc,
};
use kuna_base::space::spacetype;
use kuna_base::types::{int2, int4, uintm};

// =============================================================================
// HighContext for Funcdata (the read view the bank's lazy re-derivation needs)
// =============================================================================

impl HighContext for Funcdata {
    fn vn_view(&self, vn: VarnodeId) -> VarnodeView {
        let v = self.vbank().get(vn).expect("vn_view: stale vn");
        let space_internal = v
            .get_addr()
            .get_space()
            .map(|s| s.get_type() == spacetype::IPTR_INTERNAL)
            .unwrap_or(false);
        let def_time = v
            .get_def()
            .and_then(|op| self.obank().get(op))
            .map(|o| o.get_time())
            .unwrap_or(0);
        VarnodeView {
            flags: v.get_flags(),
            size: v.get_size(),
            type_: Rc::clone(v.get_type()),
            type_lock: v.is_type_lock(),
            merge_group: v.get_merge_group(),
            written: v.is_written(),
            def_time,
            space_internal,
            create_index: v.get_create_index(),
        }
    }
    fn vn_cover(&self, vn: VarnodeId) -> Option<Cover> {
        self.vbank().get(vn).and_then(|v| v.cover().cloned())
    }
    fn vn_has_cover(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.has_cover()).unwrap_or(false)
    }
    fn vn_name_view(&self, vn: VarnodeId) -> CompareNameView {
        let v = self.vbank().get(vn).expect("vn_name_view: stale vn");
        let space_internal = v
            .get_addr()
            .get_space()
            .map(|s| s.get_type() == spacetype::IPTR_INTERNAL)
            .unwrap_or(false);
        let def_time = v
            .get_def()
            .and_then(|op| self.obank().get(op))
            .map(|o| o.get_time())
            .unwrap_or(0);
        CompareNameView {
            name_lock: v.is_name_lock(),
            unaffected: v.is_unaffected(),
            persist: v.is_persist(),
            input: v.is_input(),
            addr_tied: v.is_addr_tied(),
            proto_partial: v.is_proto_partial(),
            space_internal,
            written: v.is_written(),
            def_time,
        }
    }
    fn vn_loc_view(&self, vn: VarnodeId) -> VarnodeViewLoc {
        let v = self.vbank().get(vn).expect("vn_loc_view: stale vn");
        VarnodeViewLoc { addr: v.get_addr().clone() }
    }
}

// =============================================================================
// MergeContext for Funcdata
// =============================================================================

impl Funcdata {
    /// Refresh the (member-varnode) covers a HighVariable's cover read depends
    /// on, then re-derive the high's cover across the bank field-split.  The
    /// member-varnode rebuild (`Varnode::updateCover`) is the C++ side effect of
    /// `HighVariable::updateCover` reading `vn->getCover()`.
    fn refresh_high_cover(&mut self, high: HighVariableId) {
        let members: Vec<VarnodeId> = {
            match self.high_bank().get(high) {
                Some(h) => (0..h.num_instances()).map(|i| h.get_instance(i)).collect(),
                None => return,
            }
        };
        for vn in members {
            self.update_varnode_cover(vn);
        }
        self.high_update_cover(high);
    }
}

impl MergeContext for Funcdata {
    // --- HighVariable flag/type reads -------------------------------------
    fn high_is_type_lock(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_type_lock(ctx, None)).unwrap_or(false))
    }
    fn high_get_type(&mut self, high: HighVariableId) -> Rc<Datatype> {
        self.with_high_split(|hb, ctx| {
            hb.get_mut(high)
                .map(|h| h.get_type(ctx, None))
                .unwrap_or_else(|| Rc::new(Datatype::new(1, crate::dtype::type_metatype::TYPE_UNKNOWN)))
        })
    }
    fn high_is_addr_tied(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_addr_tied(ctx)).unwrap_or(false))
    }
    fn high_is_input(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_input(ctx)).unwrap_or(false))
    }
    fn high_is_persist(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_persist(ctx)).unwrap_or(false))
    }
    fn high_is_extra_out(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_extra_out(ctx)).unwrap_or(false))
    }
    fn high_is_proto_partial(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_proto_partial(ctx)).unwrap_or(false))
    }
    fn high_is_name_lock(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.is_name_lock(ctx)).unwrap_or(false))
    }
    fn high_has_cover(&mut self, high: HighVariableId) -> bool {
        self.with_high_split(|hb, ctx| hb.get_mut(high).map(|h| h.has_cover(ctx)).unwrap_or(false))
    }
    fn high_is_cover_dirty(&self, high: HighVariableId) -> bool {
        self.high_bank().get(high).map(|h| h.is_cover_dirty()).unwrap_or(false)
    }
    fn high_is_mark(&self, high: HighVariableId) -> bool {
        self.high_bank().get(high).map(|h| h.is_mark()).unwrap_or(false)
    }
    fn high_has_copy_in1(&self, high: HighVariableId) -> bool {
        self.high_bank().get(high).map(|h| h.has_copy_in1()).unwrap_or(false)
    }
    fn high_has_copy_in2(&self, high: HighVariableId) -> bool {
        self.high_bank().get(high).map(|h| h.has_copy_in2()).unwrap_or(false)
    }
    fn high_num_instances(&self, high: HighVariableId) -> int4 {
        self.high_bank().get(high).map(|h| h.num_instances()).unwrap_or(0)
    }
    fn high_get_instance(&self, high: HighVariableId, i: int4) -> VarnodeId {
        self.high_bank().get(high).expect("high_get_instance: stale high").get_instance(i)
    }
    fn high_get_cover(&self, high: HighVariableId) -> Cover {
        self.high_bank().get_cover(high).cloned().unwrap_or_else(Cover::new)
    }
    fn high_internal_cover(&self, high: HighVariableId) -> Cover {
        // With no overlap pieces (the merged-tree default) the internal cover IS
        // the high's cover (C++ `getCover()` == `internalCover` for an unpieced
        // high).  The pieced path is a W7 overlap-group seam (see module docs).
        self.high_bank().get_cover(high).cloned().unwrap_or_else(Cover::new)
    }
    fn bank_update_cover_for(&mut self, high: HighVariableId) {
        self.refresh_high_cover(high);
    }

    // --- Bank-mediated piece introspection (no overlap pieces in merged tree;
    //     `VariablePiece` is not exposed across the seam, so on the merged-tree
    //     default — registers/locals without CONCAT overlap groups — every high
    //     is unpieced and these report the empty/zero piece state) -----------
    fn high_group_info(&self, _high: HighVariableId) -> Option<HighGroupInfo> {
        None
    }
    fn high_piece(&self, _high: HighVariableId) -> Option<MergePieceId> {
        None
    }
    fn piece_offset(&self, _piece: MergePieceId) -> int4 {
        0
    }
    fn piece_size(&self, _piece: MergePieceId) -> int4 {
        0
    }
    fn piece_group(&self, _piece: MergePieceId) -> u64 {
        0
    }
    fn piece_high(&self, _piece: MergePieceId) -> HighVariableId {
        HighVariableId(0)
    }
    fn piece_num_intersection(&self, _piece: MergePieceId) -> int4 {
        0
    }
    fn piece_intersection(&self, _piece: MergePieceId, _i: int4) -> MergePieceId {
        MergePieceId(0)
    }
    fn bank_update_piece_intersections(&mut self, _piece: MergePieceId) {}
    fn bank_merge_highs(
        &mut self,
        high1: HighVariableId,
        high2: HighVariableId,
        isspeculative: bool,
        cache: &mut HighIntersectTest,
    ) -> KunaResult<()> {
        // high1->merge(high2,&testCache,isspeculative): drive the bank-level merge
        // across the field-split.  The bank's `merge` reads the moved members'
        // `merge_group`/loc through the read view and reports each `vn->setHigh`
        // via `set_high`; since the merge never reads `vn->high`, we *defer* the
        // back-pointer writes into a side buffer (`set_high_log`) so the read
        // view's `&vbank` borrow does not alias the `&mut vbank` the writes need,
        // then replay them after the borrow ends.  `set_mark`/`clear_mark`/
        // `is_mark` only touch `high_bank`, distinct from `vbank`/`obank`.
        let mut set_high_log: Vec<(VarnodeId, HighVariableId, int2)> = Vec::new();
        // The test_cache's transient mark state cannot be the HighVariable's own
        // `mark` bit here (the closures would alias `high_bank.merge`'s `&mut
        // self`); use a side mark-set, behaviorally equivalent for the
        // `moveIntersectTests` purge (the marks are cleared before this returns).
        let mark_set: std::cell::RefCell<std::collections::BTreeSet<HighVariableId>> =
            std::cell::RefCell::new(std::collections::BTreeSet::new());
        let res = self.bank_merge_with_log(high1, high2, isspeculative, cache, &mut set_high_log, &mark_set);
        // Replay the deferred `vn->setHigh` writes now the read view is dropped.
        for (vn, id, mg) in set_high_log {
            if let Some(v) = self.vbank_mut().get_mut(vn) {
                v.set_high(id, mg);
            }
        }
        res
    }
    fn bank_update_cover(&mut self, high: HighVariableId) {
        self.refresh_high_cover(high);
    }
    fn bank_group_with(&mut self, high2: HighVariableId, off: int4, high1: HighVariableId) -> KunaResult<()> {
        // vn2->getHigh()->groupWith(off, vn1->getHigh()): the C++ reads each high's
        // getInstance(0)->getSize() for the piece sizes.
        let first_size = self
            .high_bank()
            .get(high2)
            .map(|h| h.get_instance(0))
            .and_then(|vn| self.vbank().get(vn))
            .map(|v| v.get_size())
            .unwrap_or(0);
        let hi2_first_size = self
            .high_bank()
            .get(high1)
            .map(|h| h.get_instance(0))
            .and_then(|vn| self.vbank().get(vn))
            .map(|v| v.get_size())
            .unwrap_or(0);
        self.high_bank_mut().group_with(high2, off, high1, first_size, hi2_first_size)
    }
    fn bank_set_mark(&mut self, high: HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.set_mark();
        }
    }
    fn bank_clear_mark(&mut self, high: HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.clear_mark();
        }
    }
    fn bank_set_unmerged(&mut self, high: HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.set_unmerged();
        }
    }
    fn bank_set_copy_in1(&mut self, high: HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.set_copy_in1();
        }
    }
    fn bank_set_copy_in2(&mut self, high: HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.set_copy_in2();
        }
    }
    fn bank_clear_copy_ins(&mut self, high: HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.clear_copy_ins();
        }
    }

    // --- Symbol reads on a HighVariable (W4 surface; un-recovered default) ---
    fn bank_symbol(&self, _high: HighVariableId) -> Option<u64> {
        None
    }
    fn bank_symbol_offset(&self, high: HighVariableId) -> int4 {
        self.high_bank().get(high).map(|h| h.get_symbol_offset()).unwrap_or(-1)
    }
    fn bank_symbol_isolated(&self, _high: HighVariableId) -> bool {
        false
    }
    fn bank_tied_addr(&self, high: HighVariableId) -> Address {
        // high->getTiedVarnode()->getAddr(): the addr-tied member's address.
        match self.high_tied_or_input_varnode(high, false) {
            Some(vn) => self.vbank().get(vn).map(|v| v.get_addr().clone()).unwrap_or_else(Address::new_invalid),
            None => Address::new_invalid(),
        }
    }
    fn bank_tied_varnode(&self, high: HighVariableId) -> Option<VarnodeId> {
        self.high_tied_or_input_varnode(high, false)
    }
    fn bank_input_varnode(&self, high: HighVariableId) -> Option<VarnodeId> {
        self.high_tied_or_input_varnode(high, true)
    }

    // --- Varnode reads ----------------------------------------------------
    fn vn_high(&self, vn: VarnodeId) -> Option<HighVariableId> {
        self.vbank().get(vn).and_then(|v| v.get_high())
    }
    fn vn_def(&self, vn: VarnodeId) -> Option<OpId> {
        self.vbank().get(vn).and_then(|v| v.get_def())
    }
    fn vn_is_free(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_free()).unwrap_or(true)
    }
    fn vn_is_input(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_input()).unwrap_or(false)
    }
    fn vn_is_written(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
    }
    fn vn_is_addr_force(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_addr_force()).unwrap_or(false)
    }
    fn vn_is_implied(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_implied()).unwrap_or(false)
    }
    fn vn_is_spacebase(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_spacebase()).unwrap_or(false)
    }
    fn vn_is_proto_partial(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_proto_partial()).unwrap_or(false)
    }
    fn vn_is_annotation(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_annotation()).unwrap_or(false)
    }
    fn vn_is_illegal_input(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_illegal_input()).unwrap_or(false)
    }
    fn vn_is_indirect_only(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.is_indirect_only()).unwrap_or(false)
    }
    fn vn_has_no_local_alias(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.has_no_local_alias()).unwrap_or(false)
    }
    fn vn_has_no_descend(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.has_no_descend()).unwrap_or(true)
    }
    fn vn_space_big_endian(&self, vn: VarnodeId) -> bool {
        self.vbank()
            .get(vn)
            .and_then(|v| v.get_addr().get_space())
            .map(|s| s.is_big_endian())
            .unwrap_or(false)
    }
    fn vn_space_internal(&self, vn: VarnodeId) -> bool {
        self.vbank()
            .get(vn)
            .and_then(|v| v.get_addr().get_space())
            .map(|s| s.get_type() == spacetype::IPTR_INTERNAL)
            .unwrap_or(false)
    }
    fn vn_has_cover_flag(&self, vn: VarnodeId) -> bool {
        self.vbank().get(vn).map(|v| v.has_cover()).unwrap_or(false)
    }
    fn vn_size(&self, vn: VarnodeId) -> int4 {
        self.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
    }
    fn vn_offset(&self, vn: VarnodeId) -> u64 {
        self.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
    }
    fn vn_cover_ref(&self, vn: VarnodeId) -> Option<Cover> {
        self.vbank().get(vn).and_then(|v| v.cover().cloned())
    }
    fn vn_create_index(&self, vn: VarnodeId) -> u64 {
        self.vbank().get(vn).map(|v| v.get_create_index() as u64).unwrap_or(0)
    }
    fn vn_symbol_entry_offset(&self, _vn: VarnodeId) -> Option<int4> {
        None
    }
    fn vn_copy_shadow(&self, a: VarnodeId, b: VarnodeId) -> bool {
        self.varnode_copy_shadow(a, b)
    }
    fn vn_partial_copy_shadow(&self, _a: VarnodeId, _b: VarnodeId, _off: int4) -> bool {
        false
    }
    fn vn_characterize_overlap(&self, a: VarnodeId, b: VarnodeId) -> int4 {
        self.varnode_characterize_overlap(a, b)
    }
    fn vn_set_implied(&mut self, vn: VarnodeId) {
        if let Some(v) = self.vbank_mut().get_mut(vn) {
            v.set_implied();
        }
    }
    fn vn_set_cover_dirty(&mut self, vn: VarnodeId) {
        if let Some(v) = self.vbank_mut().get_mut(vn) {
            v.mark_cover_dirty();
        }
    }
    fn vn_clear_implied_set_explicit(&mut self, vn: VarnodeId) {
        if let Some(v) = self.vbank_mut().get_mut(vn) {
            v.clear_implied();
            v.set_explicit();
        }
    }
    fn vn_clear_proto_partial(&mut self, vn: VarnodeId) {
        if let Some(v) = self.vbank_mut().get_mut(vn) {
            v.clear_proto_partial();
        }
    }
    fn varnode_def_point(&self, vn: VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
        let v = self.vbank().get(vn).expect("varnode_def_point: stale vn");
        match v.get_def() {
            Some(op) => {
                let parent = self.obank().get(op).and_then(|o| o.get_parent());
                let blk = parent.map(|p| self.block_index_pub(p)).unwrap_or(0);
                (Some((blk, self.op_cover_point_pub(op))), false)
            }
            None => (None, v.is_input()),
        }
    }
    fn vn_descend(&self, vn: VarnodeId) -> Vec<OpId> {
        self.vbank().get(vn).map(|v| v.descend_iter().collect()).unwrap_or_default()
    }

    // --- Op reads ---------------------------------------------------------
    fn op_code(&self, op: OpId) -> OpCode {
        self.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY)
    }
    fn op_out(&self, op: OpId) -> Option<VarnodeId> {
        self.obank().get(op).and_then(|o| o.get_out())
    }
    fn op_in(&self, op: OpId, slot: int4) -> Option<VarnodeId> {
        self.obank().get(op).and_then(|o| o.get_in(slot))
    }
    fn op_num_input(&self, op: OpId) -> int4 {
        self.obank().get(op).map(|o| o.num_input()).unwrap_or(0)
    }
    fn op_parent(&self, op: OpId) -> BlockId {
        self.obank().get(op).and_then(|o| o.get_parent()).expect("op_parent: op has no parent")
    }
    fn op_parent_index(&self, op: OpId) -> int4 {
        match self.obank().get(op).and_then(|o| o.get_parent()) {
            Some(p) => self.block_index_pub(p),
            None => 0,
        }
    }
    fn op_addr(&self, op: OpId) -> Address {
        self.obank().get(op).map(|o| o.get_addr().clone()).unwrap_or_else(Address::new_invalid)
    }
    fn op_order(&self, op: OpId) -> uintm {
        self.obank().get(op).map(|o| o.get_seq_num().get_order()).unwrap_or(0)
    }
    fn op_slot(&self, op: OpId, vn: VarnodeId) -> int4 {
        self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1)
    }
    fn op_is_dead(&self, op: OpId) -> bool {
        self.obank().get(op).map(|o| o.is_dead()).unwrap_or(true)
    }
    fn op_is_call(&self, op: OpId) -> bool {
        self.obank().get(op).map(|o| o.is_call()).unwrap_or(false)
    }
    fn op_is_marker(&self, op: OpId) -> bool {
        self.obank().get(op).map(|o| o.is_marker()).unwrap_or(false)
    }
    fn op_is_indirect_creation(&self, op: OpId) -> bool {
        self.obank().get(op).map(|o| o.is_indirect_creation()).unwrap_or(false)
    }
    fn op_is_partial_root(&self, op: OpId) -> bool {
        self.obank().get(op).map(|o| o.is_partial_root()).unwrap_or(false)
    }
    fn op_previous(&self, op: OpId) -> Option<OpId> {
        self.op_previous_op(op)
    }
    fn op_output_type_local(&self, op: OpId) -> Rc<Datatype> {
        self.op_output_type_local_pub(op)
    }
    fn op_input_type_local(&self, op: OpId, slot: int4) -> Rc<Datatype> {
        self.op_input_type_local_pub(op, slot)
    }
    fn op_cover_point(&self, op: OpId) -> (int4, CoverPoint) {
        let parent = self.obank().get(op).and_then(|o| o.get_parent());
        let blk = parent.map(|p| self.block_index_pub(p)).unwrap_or(0);
        (blk, self.op_cover_point_pub(op))
    }
    fn indirect_effect_op(&self, indop: OpId) -> OpId {
        // getOpFromConst(indop->getIn(1)->getAddr())
        let addr = self
            .obank()
            .get(indop)
            .and_then(|o| o.get_in(1))
            .and_then(|in1| self.vbank().get(in1))
            .map(|v| v.get_addr().get_offset())
            .unwrap_or(0);
        OpId::from(slotmap::KeyData::from_ffi(addr))
    }

    // --- Op mutation ------------------------------------------------------
    fn op_set_input(&mut self, op: OpId, vn: VarnodeId, slot: int4) -> KunaResult<()> {
        Funcdata::op_set_input(self, op, vn, slot)
    }
    fn op_insert_before(&mut self, op: OpId, follow: OpId) {
        Funcdata::op_insert_before(self, op, follow);
    }
    fn op_insert_after(&mut self, op: OpId, prev: OpId) {
        Funcdata::op_insert_after(self, op, prev);
    }
    fn op_insert_begin(&mut self, op: OpId, bl: BlockId) {
        Funcdata::op_insert_begin(self, op, bl);
    }
    fn op_insert_end_pred(&mut self, copyop: OpId, op: OpId, slot: int4) {
        // opInsertEnd(copyop, (BlockBasic*)op->getParent()->getIn(slot))
        let parent = self.obank().get(op).and_then(|o| o.get_parent()).expect("op_insert_end_pred: no parent");
        let pred = self.bblocks_ref().block(parent).get_in(slot);
        Funcdata::op_insert_end(self, copyop, pred);
    }
    fn op_mark_non_printing(&mut self, op: OpId) {
        self.op_mark_non_printing_pub(op);
    }

    // --- Block reads ------------------------------------------------------
    fn bblocks_get_size(&self) -> int4 {
        Funcdata::bblocks_get_size(self)
    }
    fn bblocks_get_block(&self, i: int4) -> BlockId {
        Funcdata::bblocks_get_block(self, i)
    }
    fn block_ops(&self, bl: BlockId) -> Vec<OpId> {
        self.bb_ops(bl)
    }
    fn block_dominates(&self, dom: BlockId, sub: BlockId) -> bool {
        self.bblocks_ref().dominates(dom, Some(sub))
    }

    // --- Function-level op iteration --------------------------------------
    fn ops_alive(&self) -> Vec<OpId> {
        self.obank().iter_alive().collect()
    }

    // --- Cover construction for a single read -----------------------------
    fn single_read_cover(&self, vn: VarnodeId, op: OpId) -> Cover {
        self.build_single_read_cover(vn, op)
    }
    fn copy_pair_range(&self, dom_op: OpId, sub_op: OpId) -> Cover {
        self.build_copy_pair_range(dom_op, sub_op)
    }

    // --- IR-surgery seams -------------------------------------------------
    fn copy_trim_op(&mut self, in_vn: VarnodeId, addr: Address, trim_op: OpId) -> KunaResult<OpId> {
        self.build_copy_trim_op(in_vn, addr, trim_op)
    }
    fn trim_op_output(&mut self, op: OpId) -> KunaResult<()> {
        self.do_trim_op_output(op)
    }
    fn trim_op_input_prep(&self, op: OpId, slot: int4) -> (VarnodeId, Address, bool) {
        // C++ Merge::trimOpInput prelude (merge.cc:699-705).
        let o = self.obank().get(op).expect("trim_op_input_prep: stale op");
        let is_multiequal = o.code() == OpCode::CPUI_MULTIEQUAL;
        let in_vn = o.get_in(slot).expect("trim_op_input_prep: null input");
        let pc = if is_multiequal {
            // pc = ((BlockBasic*)op->getParent()->getIn(slot))->getStop()
            let parent = o.get_parent().expect("trim_op_input_prep: no parent");
            let pred = self.bblocks_ref().block(parent).get_in(slot);
            self.block_stop_addr(pred)
        } else {
            o.get_addr().clone()
        };
        (in_vn, pc, is_multiequal)
    }
    fn snip_reads_insert_point(&self, vn: VarnodeId) -> (BlockId, Address, Option<OpId>) {
        self.do_snip_reads_insert_point(vn)
    }
    fn indirect_inherit_union(&mut self, _outvn: VarnodeId, _newop: OpId, _indop: OpId) {
        // needsResolution union inheritance: no union types in the merged tree.
    }
    fn sort_op_nodes_by_high(&self, nodes: &mut Vec<PcodeOpNode>) {
        // PcodeOpNode::compareByHigh: sort by the input varnode's high (createIndex
        // tie-break).  No high crosses speculative groups on the merge path here;
        // a stable create-index sort matches the C++ deterministic order.
        nodes.sort_by_key(|n| {
            self.obank()
                .get(n.op)
                .and_then(|o| o.get_in(n.slot))
                .and_then(|v| self.vbank().get(v))
                .map(|v| v.get_create_index())
                .unwrap_or(0)
        });
    }
    fn build_dominant_copy(&mut self, _high: HighVariableId, _copy: &[OpId], _pos: int4, _size: int4) -> KunaResult<()> {
        // findCommonBlock/totalReplace surgery (W7); not reached on the
        // MULTIEQUAL/COPY merge path of the vertical slice.
        Err(kuna_base::error::KunaError::lowlevel("build_dominant_copy: W7 seam (not on merge path)"))
    }

    // --- mergeAddrTied / mergeMultiEntry seams ----------------------------
    fn addr_tied_ranges(&self) -> Vec<AddrTiedRange> {
        // Funcdata::overlapLoc (the maximally-overlapping addr-tied range
        // collection) is a W7 surface; on the merged-tree default there are no
        // addr-tied overlap groups (registers are not addrtied), so this is empty.
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
