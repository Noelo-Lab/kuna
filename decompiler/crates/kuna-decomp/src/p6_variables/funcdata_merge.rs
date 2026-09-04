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
//! ## Boundaries (documented losses, narrow on the merged-tree default)
//!  * The W4 Symbol/ScopeLocal layer is not present (`localmap` is a unit stub):
//!    `bank_symbol`/`multi_entry_symbols`/`symbol_*` return the un-recovered
//!    default (no symbol, no multi-entry groups), so `mergeMultiEntry` is a true
//!    no-op and merged locals are named directly on the HighVariable
//!    (`kuna_naming`), not via a `Symbol` in the local scope.
//!  * `addr_tied_ranges` (`Funcdata::overlapLoc`) and the union-resolution arms of
//!    the trim COPYs are the conservative default (no addr-tied overlap groups, no
//!    `needsResolution` types in the merged tree) — exact on boolless.
//!  * `build_dominant_copy` is now the full `Merge::buildDominantCopy` body
//!    (findCommonBlock / cover-intersection / totalReplace / opDestroy /
//!    high->merge) on `Funcdata::build_dominant_copy_impl`; it drives the
//!    `ActionDominantCopy` hoist over the persistent `covermerge`'s `copyTrims`.
//!  * `populate_affecting_ops` (the `StackAffectingOps` cross-call test source)
//!    and the partial-copy-shadow characterization stay the conservative path
//!    (W7 `StackAffectingOps`); empty on the merged-tree slices here.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::cover::{Cover, CoverPoint, PcodeOpSet};
use crate::dtype::Datatype;
use crate::expression::PcodeOpNode;
use crate::funcdata::Funcdata;
use crate::merge::{AddrTiedRange, HighGroupInfo, MergeContext, MergePieceId};
use crate::context::{BlockId, HighVariableId, OpId, VarnodeId};
use crate::variable::{
    CompareNameView, HighContext, HighIntersectTest, VariablePieceId, VarnodeView, VarnodeViewLoc,
};
use kuna_base::space::spacetype;
use kuna_base::types::{int2, int4, uintm};

/// (kuna LOSS-229) Disjoint id-spaces for `MergeContext::bank_symbol`.  A local
/// (dynamic/equate) Symbol is a slotmap ffi key; a global mapped Symbol is keyed
/// off its storage address.  We clear the top bit on the local id and set it on
/// the global id so the two id-spaces never collide in the `mergeTestRequired`
/// symbol guard (`symbolIn != symbolOut`).  A slotmap ffi key never legitimately
/// reaches the top bit (the index/version halves are far smaller), so clearing it
/// is a no-op on every real local Symbol.
const SYM_ID_LOCAL_MASK: u64 = !(1u64 << 63);
const SYM_ID_GLOBAL_TAG: u64 = 1u64 << 63;

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
    /// Construct the function's persistent [`Merge`](crate::merge::Merge) engine
    /// (C++ `Merge covermerge` built by `Funcdata(...)` with `stackAffectingOps(fd)`
    /// + `testCache(stackAffectingOps)`).
    ///
    /// The `StackAffectingOps` set is the W7 stack-alias cross-call test source;
    /// the merged-tree default populates it empty (no stack-affecting ops in the
    /// boolless/condconst slices), exactly as `MergeContext::populate_affecting_ops`.
    fn make_covermerge() -> crate::merge::Merge {
        let opset = crate::cover::PcodeOpSet::new(Box::new(Vec::new), Box::new(|_, _| false));
        let cache = crate::variable::HighIntersectTest::new(opset);
        crate::merge::Merge::new(cache)
    }

    /// Ensure the persistent merge engine exists (C++ `covermerge` is a value
    /// member, always present; the Rust field is lazily built so the bare
    /// `Funcdata::new` need not construct it).
    pub fn ensure_covermerge(&mut self) {
        if self.covermerge.is_none() {
            self.covermerge = Some(Self::make_covermerge());
        }
    }

    /// (kuna LOSS-229) Model `HighVariable::getSymbol()` (variable.cc:418-432) for a
    /// `map addr`-mapped access, returning the covering Symbol's storage address and
    /// in-symbol byte offset `(entry_addr, sym_off)`, or `None`.
    ///
    /// C++ `updateSymbol()` scans the high's member Varnodes for one carrying a
    /// `SymbolEntry`; for a `map addr` symbol (global *or* a stack-mapped local) that
    /// entry is the Symbol's.  The merged kuna tree does not paint mapped SymbolEntries
    /// onto varnodes before the merge group (`ActionMapGlobals` runs after it,
    /// coreaction.cc:6025; the local sync only updates flags), so we resolve the
    /// covering Symbol directly from the scope snapshot — the same
    /// `Scope::queryContainer` covering-entry lookup `linkSymbol` would use.
    ///
    /// Restricted to an address-tied member: a `map addr` global is persist+addrtied,
    /// a `map addr` stack symbol is addrtied — exactly the Varnodes C++
    /// `handleSymbolConflict` (funcdata_varnode.cc:1023) attaches the SymbolEntry to,
    /// and never a free/internal temp.  The local stack scope is queried first (a
    /// stack member is never global), the frozen global scope second.  Returns
    /// `(entry_addr, sym_off)` for the first member that maps, location-ordered (the
    /// C++ `updateSymbol` first-hit; members of one high share the same Symbol).
    fn kuna_mapped_symbol_entry(&self, high: HighVariableId) -> Option<(Address, int4)> {
        let h = self.high_bank().get(high)?;
        // (kuna perf) The scan below only ever accepts an address-tied member,
        // and a high's cached flag word is the OR of its members', so a clean
        // `addrtied == false` settles it without walking them.  `mergeTestRequired`
        // — this is its inner loop — reads `high_is_addr_tied` on both highs before
        // it gets here, so the flags are clean by construction; a dirty word just
        // falls through to the scan.
        if h.kuna_addr_tied_if_clean() == Some(false) {
            return None;
        }
        let invalid = Address::new_invalid();
        let n = h.num_instances();
        for i in 0..n {
            let vn = h.get_instance(i);
            let v = match self.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            // Only an address-tied member carries a `map addr` SymbolEntry pre-merge
            // (C++ handleSymbolConflict's input/addrtied/persist/constant arm).  A
            // free/internal temp never does.
            if v.is_free() || !v.is_addr_tied() {
                continue;
            }
            let addr = v.get_addr().clone();
            let size = v.get_size();
            // Local (stack) scope first: a `map addr s...` symbol.  `entry_addr` is the
            // covering Symbol's storage address (its identity), `sym_off` the in-symbol
            // byte offset of the access.
            if let Some(lm) = self.get_scope_local() {
                // `container_symbol_link` is the same `findContainer` query as
                // `query_container_for_link` without the display-name String and
                // data-type clones that result carries; this is a hot merge-test
                // path that reads only the entry geometry.
                if let Some((_sym, entry_addr, _sz, entry_off)) =
                    lm.container_symbol_link(&addr, &invalid)
                {
                    let delta =
                        addr.get_offset().wrapping_sub(entry_addr.get_offset()) as int4;
                    return Some((entry_addr, delta.wrapping_add(entry_off)));
                }
            }
            // Then the frozen global scope (`map addr r...`): the covering global Symbol.
            if let Some(c) = self.get_arch().query_container_global(&addr, size, &invalid) {
                let delta = addr.get_offset().wrapping_sub(c.entry_addr.get_offset()) as int4;
                return Some((c.entry_addr, delta.wrapping_add(c.symbol_offset)));
            }
        }
        None
    }

    /// (kuna LOSS-229) The `map addr` Symbol id for the `bank_symbol` guard, keyed off
    /// the covering entry's storage address (a `map addr` Symbol has one storage),
    /// tagged into the mapped id-space so it never collides with a local slotmap key.
    fn kuna_mapped_symbol_id(&self, high: HighVariableId) -> Option<u64> {
        let (entry_addr, _off) = self.kuna_mapped_symbol_entry(high)?;
        let off = entry_addr.get_offset();
        let space = entry_addr.get_space().map(|s| s.get_index() as u64).unwrap_or(0);
        // Combine space + offset into one id; clear the top bit of the payload then
        // tag the mapped id-space.  Two members of the same Symbol resolve to the same
        // entry_addr -> same id (the C++ `symbolIn == symbolOut`); space differentiates
        // a stack symbol from a global at the same numeric offset.
        Some(SYM_ID_GLOBAL_TAG | (((space << 48) ^ off) & SYM_ID_LOCAL_MASK))
    }

    /// (kuna LOSS-229) The in-symbol byte offset for a `map addr` access (C++
    /// `HighVariable::setSymbol`'s `symboloffset`, variable.cc:269: `(addr -
    /// entry_addr) + entry_offset`), used by the `mergeTestRequired` "different parts
    /// of same symbol" arm.
    fn kuna_mapped_symbol_offset(&self, high: HighVariableId) -> Option<int4> {
        self.kuna_mapped_symbol_entry(high).map(|(_addr, off)| off)
    }

    /// Run a closure with the persistent merge engine and `self` as the
    /// [`MergeContext`] (C++ `data.getMerge().<method>()`).
    ///
    /// The engine is moved out of `self` for the duration so it can take `&mut
    /// self` as the context (the C++ `Merge` holds a `data` back-pointer; Rust
    /// expresses the same self-mutation with a move-out / move-back), then moved
    /// back so its `copyTrims` accumulator persists across actions.  This is the
    /// faithful equivalent of the single `Funcdata::covermerge` member the C++
    /// merge actions all share.
    pub fn with_covermerge<R>(
        &mut self,
        f: impl FnOnce(&mut crate::merge::Merge, &mut Funcdata) -> R,
    ) -> R {
        self.ensure_covermerge();
        let mut merge = self.covermerge.take().expect("with_covermerge: just ensured");
        let r = f(&mut merge, self);
        self.covermerge = Some(merge);
        r
    }

    /// Clear the persistent merge engine's cached state (C++ `Merge::clear`,
    /// called by `Funcdata::clear`).  Drops the test cache, the `copyTrims`
    /// accumulator, and the proto-partial roots.
    pub fn clear_covermerge(&mut self) {
        if let Some(m) = self.covermerge.as_mut() {
            m.clear();
        }
    }

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

/// Decode a [`MergePieceId`] (the boundary's opaque `u64` handle) back to the
/// bank's [`VariablePieceId`].  The encoding is `u64::from(piece.0)`, so the
/// narrowing is exact (`piece.0` was a `u32`).
fn decode_piece(piece: MergePieceId) -> VariablePieceId {
    VariablePieceId(piece.0 as u32)
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
        // C++ `high->internalCover` (merge.cc:1622): the RAW internal cover, NOT
        // `getCover()` (which is the piece's extended cover when the high is
        // grouped).  `inflateTest` deliberately checks intersections only against
        // the cover contributing to the inflate.
        self.high_bank().internal_cover(high).cloned().unwrap_or_else(Cover::new)
    }
    fn bank_update_cover_for(&mut self, high: HighVariableId) {
        self.refresh_high_cover(high);
    }

    // --- Bank-mediated piece introspection --------------------------------
    //     The `VariableGroup`/`VariablePiece` overlap model lives in the
    //     `high_bank` arena (variable.rs); these read it back across the boundary so
    //     `mergeTestRequired`'s piece-group arm (merge.cc:147-154),
    //     `markInternalCopies`'s PIECE/SUBPIECE self-assign suppression
    //     (merge.cc:1478-1528), and `inflateTest`'s extended-cover walk
    //     (merge.cc:1631-1645) see the real pieces formed by `groupWith`/
    //     `groupPartialRoot`.  A `MergePieceId(u64)` wraps the bank's
    //     `VariablePieceId(u32)`; `piece_group` returns the `VariableGroupId(u32)`
    //     widened to `u64` (a stable per-group identity, exactly what the C++
    //     `groupIn == groupOut` pointer compare needs).
    fn high_group_info(&self, high: HighVariableId) -> Option<HighGroupInfo> {
        let pid = self.high_bank().high_piece_id(high)?;
        let group = self.high_bank().piece_group(pid)?;
        Some(HighGroupInfo {
            group: u64::from(group.0),
            piece_size: self.high_bank().piece_size(pid),
            group_size: self.high_bank().group_size(group),
        })
    }
    fn high_piece(&self, high: HighVariableId) -> Option<MergePieceId> {
        self.high_bank().high_piece_id(high).map(|p| MergePieceId(u64::from(p.0)))
    }
    fn piece_offset(&self, piece: MergePieceId) -> int4 {
        self.high_bank().piece_offset(decode_piece(piece))
    }
    fn piece_size(&self, piece: MergePieceId) -> int4 {
        self.high_bank().piece_size(decode_piece(piece))
    }
    fn piece_group(&self, piece: MergePieceId) -> u64 {
        self.high_bank()
            .piece_group(decode_piece(piece))
            .map(|g| u64::from(g.0))
            .unwrap_or(0)
    }
    fn piece_high(&self, piece: MergePieceId) -> HighVariableId {
        self.high_bank().piece_high(decode_piece(piece)).unwrap_or(HighVariableId(0))
    }
    fn piece_num_intersection(&self, piece: MergePieceId) -> int4 {
        self.high_bank().piece_num_intersection(decode_piece(piece))
    }
    fn piece_intersection(&self, piece: MergePieceId, i: int4) -> MergePieceId {
        self.high_bank()
            .piece_get_intersection(decode_piece(piece), i)
            .map(|p| MergePieceId(u64::from(p.0)))
            .unwrap_or(MergePieceId(0))
    }
    fn bank_update_piece_intersections(&mut self, piece: MergePieceId) {
        self.high_bank_mut().update_piece_intersections(decode_piece(piece));
    }
    fn bank_merge_highs(
        &mut self,
        high1: HighVariableId,
        high2: HighVariableId,
        isspeculative: bool,
        cache: &mut HighIntersectTest,
    ) -> KunaResult<()> {
        // Drive the bank-level merge across the field-split.  The bank's `merge`
        // reads the moved members'
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
        // The C++ `groupWith` reads each high's `getInstance(0)->getSize()` for the
        // piece sizes.
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
    fn bank_symbol(&self, high: HighVariableId) -> Option<u64> {
        // (kuna LOSS-229) Expose the Symbol id bound to this high so
        // `Merge::mergeTestRequired` (merge.cc:157-164) keeps a dynamic temp distinct
        // from the storage it copies.  Equate symbols count too (same C++ guard).
        use slotmap::Key;
        let h = self.high_bank().get(high)?;
        if let Some(s) = h.kuna_dynamic_symbol().or_else(|| h.kuna_equate_symbol()) {
            // Tag with the top bit cleared so a slotmap ffi key never collides with a
            // mapped-Symbol id (which is keyed off the entry storage address + tag bit).
            return Some(s.data().as_ffi() & SYM_ID_LOCAL_MASK);
        }
        // C++ `HighVariable::getSymbol()` also resolves a `map addr` access to its
        // Symbol (the member Varnode carries the SymbolEntry, variable.cc:418-432).
        // The merged kuna tree does not paint mapped SymbolEntries onto varnodes before
        // the merge group (ActionMapGlobals runs after, coreaction.cc:6025), so resolve
        // the covering Symbol directly from the scope snapshot (the local stack scope,
        // then the frozen global scope) — the same covering-entry lookup `linkSymbol`
        // would use.
        self.kuna_mapped_symbol_id(high)
    }
    fn bank_symbol_offset(&self, high: HighVariableId) -> int4 {
        let h = match self.high_bank().get(high) {
            Some(h) => h,
            None => return -1,
        };
        // A dynamic/equate symbol parks its in-symbol offset on the high; a `map addr`
        // access reads the covering entry's `(addr - entry_addr) + entry_off`.
        if h.kuna_dynamic_symbol().is_some() || h.kuna_equate_symbol().is_some() {
            return h.get_symbol_offset();
        }
        self.kuna_mapped_symbol_offset(high).unwrap_or_else(|| h.get_symbol_offset())
    }
    fn bank_symbol_isolated(&self, high: HighVariableId) -> bool {
        // (kuna L2) C++ `Merge::mergeTestAdjacent` (merge.cc:196-205) bails out
        // when `high->getSymbol()` is a non-null isolated Symbol.  Resolve the
        // SAME Symbol `bank_symbol` resolves, then read its
        // `Symbol::isIsolated()` (database.hh:241).  A dynamic-hash / equate
        // Symbol is bound directly on the high (`type varnode`/equate); a
        // `map addr` access resolves through the covering SymbolEntry.  Mirrors
        // `HighVariable::updateSymbol`'s first-hit member scan.
        let h = match self.high_bank().get(high) {
            Some(h) => h,
            None => return false,
        };
        // Dynamic / equate Symbol bound on the high (the `type varnode tmp`
        // isolated return temp, or an equate): read its isolated flag from the
        // local scope's symbol table.
        if let Some(sym) = h.kuna_dynamic_symbol().or_else(|| h.kuna_equate_symbol()) {
            if let Some(lm) = self.get_scope_local() {
                return lm.symbol_isolated(sym);
            }
            return false;
        }
        // A covering Symbol: query the same containing entry the `bank_symbol`
        // member scan resolves and read its isolated flag.  The local scope
        // carries the `isolate`-able Symbol; a frozen global entry is never
        // isolated in the merged-tree snapshots.
        //
        // (kuna L4) Unlike `bank_symbol`'s addr-tied member scan, a `type varnode
        // %REG(pc)` isolated Symbol is REGISTER storage (not addr-tied) and is
        // usepoint-SCOPED: `find_container` only matches it when the query
        // usepoint falls in the entry's `uselimit`.  Iterate ALL members (not
        // just addr-tied), query at each member's real use point, and return
        // isolated on the first covering entry — mirroring `HighVariable::
        // getSymbol`'s first-hit member scan against a properly painted
        // SymbolEntry.
        //
        // (kuna perf) The scan is `merge_test_adjacent`'s inner loop and is
        // quadratic in a big function: it runs once per candidate pair and
        // walks every member of both highs, so on a 43 KB function it issued
        // 26 M containment queries.  A scope that has never had a Symbol
        // isolated cannot answer `true` from any of them, and the global arm
        // below is unconditionally `false`, so the whole scan is skippable —
        // see `ScopeLocal::has_isolated_symbols`.
        if !self.get_scope_local().map(|lm| lm.has_isolated_symbols()).unwrap_or(false) {
            return false;
        }
        let members: Vec<VarnodeId> = (0..h.num_instances()).map(|i| h.get_instance(i)).collect();
        for vn in members {
            let (addr, is_free) = match self.vbank().get(vn) {
                Some(v) => (v.get_addr().clone(), v.is_free()),
                None => continue,
            };
            if is_free {
                continue;
            }
            let usepoint = self.vn_use_point(vn);
            if let Some(lm) = self.get_scope_local() {
                if let Some(info) = lm.query_container_for_link(&addr, &usepoint) {
                    if info.is_isolated {
                        return true;
                    }
                }
            }
            // Global covering entry: not isolated in the merged-tree snapshots.
        }
        false
    }
    fn bank_tied_addr(&self, high: HighVariableId) -> Address {
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
    fn vn_partial_copy_shadow(&self, a: VarnodeId, b: VarnodeId, off: int4) -> bool {
        // C++ `b->partialCopyShadow(a, off)`: allow the partial-shadow overlap in
        // the pieced inflate/intersection tests (merge.cc:1640, variable.cc).
        self.varnode_partial_copy_shadow(a, b, off)
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
        // C++ `Varnode::setFlags` (varnode.cc:371-380) forwards a `coverdirty` set
        // to the owning HighVariable (`high->coverDirty()`).  `Varnode::set_flags`
        // is a W7 stub that cannot reach the high bank, so the forward happens
        // here: without it `HighIntersectTest::updateHigh` short-circuits on a
        // clean high and the member Cover is never rebuilt.
        if let Some(h) = self.vbank().get(vn).and_then(|v| v.get_high()) {
            self.high_bank_mut().cover_dirty(h);
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

    // --- IR-surgery hooks -------------------------------------------------
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
    fn build_dominant_copy(&mut self, high: HighVariableId, copy: &[OpId], pos: int4, size: int4) -> KunaResult<()> {
        // C++ `Merge::buildDominantCopy` body (findCommonBlock / totalReplace /
        // opDestroy / high->merge surgery).  Lives on `Funcdata` since the whole
        // body is live-graph IR mutation + cover math off the varnode/op/block
        // arenas.
        self.build_dominant_copy_impl(high, copy, pos, size)
    }

    // --- mergeAddrTied / mergeMultiEntry hooks ----------------------------
    fn addr_tied_ranges(&self) -> Vec<AddrTiedRange> {
        // Faithful `Merge::mergeAddrTied` range collection (merge.cc:609-648) over
        // the loc-ordered Varnode set, driving `VarnodeBank::overlapLoc`
        // (varnode.cc:1810-1838): walk PROCESSOR/SPACEBASE Varnodes, gather each
        // *maximally-overlapping* window of non-free Varnodes, and within it emit:
        //   * `must_merge_ranges` — one per (addr,size) group (the C++
        //     `[bounds[i],bounds[i+1])` pairs that `mergeRangeMust` force-merges
        //     into a single HighVariable), and
        //   * `group_with` — for a window spanning more than one storage address,
        //     the `(vn2->getHigh(), off, vn1->getHigh())` group-with triples
        //     (`merge.cc:636-643`, `off = vn2.offset - vn1.offset`).
        // `addrtied` is the OR of every member's flags (C++ `overlapLoc` returns the
        // unioned flag set).  Now that `set_varnode_properties` paints global RAM
        // stores `addrtied`, the addr-tied global reads + the MULTIEQUAL/RETURN-COPY
        // outputs at one address pre-merge here, BEFORE `mergeMarker` processes the
        // phi — so the phi's input is already in the output's high and is NOT
        // trimmed to a `unique` (the C++ order; without this the persist `isInput()`
        // gate of `mergeTestRequired` snips a spurious `glob = glob` COPY).
        use crate::varnode::varnode_flags;
        use kuna_base::types::uintb;

        let order: Vec<VarnodeId> = self.vbank().iter_loc().collect();
        let mut ranges: Vec<AddrTiedRange> = Vec::new();
        let mut i = 0usize;
        while i < order.len() {
            let vn = order[i];
            let v = match self.vbank().get(vn) {
                Some(v) => v,
                None => {
                    i += 1;
                    continue;
                }
            };
            let spc = match v.get_addr().get_space() {
                Some(s) => s,
                None => {
                    i += 1;
                    continue;
                }
            };
            let stype = spc.get_type();
            if stype != spacetype::IPTR_PROCESSOR && stype != spacetype::IPTR_SPACEBASE {
                // C++ `startiter = data.endLoc(spc)` — skip the whole space.  The loc
                // order groups a space contiguously, so advance past every Varnode of
                // this space.
                let sp_index = spc.get_index();
                i += 1;
                while i < order.len() {
                    let same = self
                        .vbank()
                        .get(order[i])
                        .and_then(|v2| v2.get_addr().get_space())
                        .map(|s| s.get_index() == sp_index)
                        .unwrap_or(false);
                    if !same {
                        break;
                    }
                    i += 1;
                }
                continue;
            }

            // Collect the maximally-overlapping window (C++ `overlapLoc`): start at
            // `vn`, extend `maxOff` as later Varnodes (same space) overlap, OR the
            // flags.  Group members by (offset,size) into `must_merge_ranges`.
            let start_off = v.get_offset();
            let mut max_off = start_off.wrapping_add((v.get_size() as uintb).wrapping_sub(1));
            let sp_index = spc.get_index();
            let mut flags = v.get_flags();
            let mut all_varnodes: Vec<VarnodeId> = Vec::new();
            // (offset, size) -> member list, in first-seen order (the loc order).
            let mut groups: Vec<((uintb, int4), Vec<VarnodeId>)> = Vec::new();

            let mut j = i;
            while j < order.len() {
                let v2 = match self.vbank().get(order[j]) {
                    Some(v2) => v2,
                    None => {
                        j += 1;
                        continue;
                    }
                };
                let a2 = v2.get_addr();
                let s2 = match a2.get_space() {
                    Some(s) => s,
                    None => break,
                };
                if s2.get_index() != sp_index || v2.get_offset() > max_off {
                    break; // left the overlap window / the space
                }
                if v2.is_free() {
                    j += 1;
                    continue; // C++ skips free Varnodes (they are not merged)
                }
                let end_off = v2.get_offset().wrapping_add((v2.get_size() as uintb).wrapping_sub(1));
                if end_off > max_off {
                    max_off = end_off;
                }
                flags |= v2.get_flags();
                all_varnodes.push(order[j]);
                let key = (v2.get_offset(), v2.get_size());
                match groups.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, lst)) => lst.push(order[j]),
                    None => groups.push((key, vec![order[j]])),
                }
                j += 1;
            }

            // Advance the outer cursor past this whole window (C++ `startiter =
            // bounds[max]`).
            i = j.max(i + 1);

            let addrtied = (flags & varnode_flags::addrtied) != 0;
            if !addrtied || groups.is_empty() {
                continue;
            }

            let must_merge_ranges: Vec<Vec<VarnodeId>> =
                groups.iter().map(|(_, lst)| lst.clone()).collect();

            // group_with: when the window spans more than one storage address, group
            // each later address's high with the first's, at `off = addr2 - addr1`
            // (C++ `merge.cc:636-643`, `if (max > 2)`).  Each group is already a
            // single high after `mergeRangeMust`, so its representative Varnode's
            // high is the group's high.
            let mut group_with: Vec<(HighVariableId, int4, HighVariableId)> = Vec::new();
            if groups.len() > 1 {
                let (off1, _) = groups[0].0;
                let high1 = self.vbank().get(groups[0].1[0]).and_then(|v| v.get_high());
                if let Some(high1) = high1 {
                    for (k, lst) in groups.iter().skip(1) {
                        if let Some(high2) = self.vbank().get(lst[0]).and_then(|v| v.get_high()) {
                            // cast: uintb difference -> int4 storage offset, the C++
                            // `(int4)` narrowing; an in-window offset delta is small.
                            let off = (k.0.wrapping_sub(off1)) as int4;
                            group_with.push((high2, off, high1));
                        }
                    }
                }
            }

            ranges.push(AddrTiedRange {
                addrtied,
                all_varnodes,
                must_merge_ranges,
                group_with,
            });
        }
        ranges
    }
    fn multi_entry_symbols(&self) -> Vec<u64> {
        Vec::new()
    }
    fn symbol_linked_varnodes(&self, _symbol: u64) -> (Vec<VarnodeId>, int4) {
        (Vec::new(), 0)
    }
    fn symbol_set_merge_problems(&mut self, _symbol: u64) {}
    fn symbol_merge_warning(&mut self, _symbol: u64, _m: int4, _s: int4, _c: int4) {}
    fn populate_affecting_ops(&self, op_set: &mut PcodeOpSet) {
        // C++ `StackAffectingOps::populate` (merge.cc:63): add every CALL op,
        // then every still-valid store-guard STORE; finalize; then `affectsTest`
        // (merge.cc:78) does the secondary test (a STORE consults its store-guard
        // `isGuarded(vn->getAddr())`, a CALL always affects).
        use crate::cover::PcodeOpSetEntry;
        use kuna_base::types::uintb;

        // CALL-arm: every call site (data.numCalls()/getCallSpecs(i)->getOp()).
        for i in 0..self.num_calls() {
            let op = self.get_call_specs(i).get_op();
            let (block_index, point) = self.op_cover_point(op);
            let order = self.op_order(op);
            op_set.add_op_entry(PcodeOpSetEntry { id: op, block_index, point, order });
        }

        // store-guard arm: every store-guard whose op is still a live STORE
        // (`(*iter).isValid(CPUI_STORE)`), plus snapshot its guard range so the
        // `affectsTest` closure can run `isGuarded(vn->getAddr())` without the
        // arena.
        let mut guard_ranges: Vec<(OpId, Rc<kuna_base::space::AddrSpace>, uintb, uintb)> =
            Vec::new();
        for guard in self.get_store_guards() {
            let op = guard.op;
            let valid = self
                .obank()
                .get(op)
                .map(|o| !o.is_dead() && o.code() == OpCode::CPUI_STORE)
                .unwrap_or(false);
            if !valid {
                continue;
            }
            let (block_index, point) = self.op_cover_point(op);
            let order = self.op_order(op);
            op_set.add_op_entry(PcodeOpSetEntry { id: op, block_index, point, order });
            guard_ranges.push((
                op,
                Rc::clone(&guard.spc),
                guard.get_minimum(),
                guard.get_maximum(),
            ));
        }

        op_set.finalize_pub();

        // Bind the secondary `affectsTest`: a STORE with a guard tests
        // `isGuarded(addr)`; a STORE without a guard (or a CALL) always affects.
        op_set.set_affects(Box::new(move |op, addr: &kuna_base::address::Address| {
            match guard_ranges.iter().find(|(g, ..)| *g == op) {
                None => true, // not a guarded STORE (a CALL, or a STORE with no record)
                Some((_, spc, min, max)) => {
                    // LoadGuard::isGuarded(addr): same space, offset within range.
                    match addr.get_space() {
                        Some(s) if Rc::ptr_eq(s, spc) => {}
                        _ => return false,
                    }
                    let off = addr.get_offset();
                    off >= *min && off <= *max
                }
            }
        }));
    }
    fn gather_pieces(&self, vn: VarnodeId, base_offset: int4) -> Vec<(VarnodeId, int4)> {
        let def = match self.vbank().get(vn).and_then(|v| v.get_def()) {
            Some(d) => d,
            None => return Vec::new(),
        };
        let mut stack: Vec<crate::op::PieceNode> = Vec::new();
        crate::op::gather_pieces(&mut stack, self.obank(), self.vbank(), vn, def, base_offset, base_offset);
        // The C++ groupPartialRoot iterates *all* nodes' Varnodes (internal and
        // leaf): each must be proto-partial (throw-out test) and gets grouped with
        // the root at `typeOffset - baseOffset`.  Expose each node's
        // `(varnode, typeOffset)` in that exact order.
        stack
            .iter()
            .map(|n| (n.get_varnode(self.obank()), n.get_type_offset()))
            .collect()
    }
}
