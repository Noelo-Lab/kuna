//! Adversarial verifier tests for item `w7-s6-merge` (round 1).
//!
//! These target the spots the hunt list flagged as most fragile for the
//! `merge.{cc,hh}` port — the wave's named concern is HighVariable merge /
//! Cover-intersection ordering:
//!
//!   * `BlockVarnode::find_front` binary search boundaries (empty / single /
//!     duplicate-block / first-of-run / absent) — `merge.cc:43-61`. The C++
//!     starts `max = list.size()-1` (an unsigned `0u-1` truncated into a signed
//!     `int4`); the Rust must reproduce the `-1` and the off-by-one-free first
//!     hit of a run of equal indices.
//!   * `compare_high_by_block` totality + tie-break chain (cover -> first-
//!     instance addr -> def-op addr -> null-def ordering) — `merge.hh:152-174`.
//!     A non-total or mis-ordered comparator silently corrupts the `mergeLinear`
//!     grouping.
//!   * `compare_copy_by_in_varnode` group/secondary ordering — `merge.cc:1045`.
//!   * `merge_by_datatype` order preservation across the erase-while-iterating
//!     C++ `list::erase(hiter++)` dance — `merge.cc:383-400`. The matched type
//!     group AND the leftover list must both keep discovery order.
//!   * `MergeIntersect::intersection` symmetry, self-non-intersection, the cache
//!     hit/dirty path, and the real two-cursor Cover block walk — `variable.cc`
//!     intersection driver hosted in merge.rs.
//!
//! Anchors: cpp `decompiler/cpp/merge.cc`, `decompiler/cpp/merge.hh`;
//! rust `decompiler/crates/kuna-decomp/src/merge.rs`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
use kuna_base::types::{int4, uintm};
use kuna_num::opcodes::OpCode;
use slotmap::{Key, KeyData};

use kuna_decomp::cover::{Cover, CoverPoint, PcodeOpSet};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::merge::{
    compare_copy_by_in_varnode, compare_high_by_block, AddrTiedRange, BlockVarnode,
    HighGroupInfo, Merge, MergeContext, MergeIntersect, MergePieceId,
};
use kuna_decomp::context::{BlockId, HighVariableId, OpId, VarnodeId};
use kuna_decomp::variable::{
    CompareNameView, HighContext, HighIntersectTest, VarnodeView, VarnodeViewLoc,
};

// ---------- id helpers -------------------------------------------------------

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

fn ram() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}
fn addr(space: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(space), off)
}

fn op_point(uindex: uintm) -> CoverPoint {
    CoverPoint::Op {
        id: oid(u64::from(uindex) + 1),
        uindex,
        code: OpCode::CPUI_COPY,
    }
}

/// A one-block cover spanning `[start,stop]` on block `blk` (an instance's
/// coverage). Built via two def-points + a merge so contain/intersect see the
/// explicit `[start,stop]` geometry.
fn cover_block(blk: int4, start: uintm, stop: uintm) -> Cover {
    let mut c = Cover::new();
    c.add_def_point(Some((blk, op_point(start))), false);
    let mut whole = Cover::new();
    whole.add_def_point(Some((blk, op_point(stop))), false);
    c.merge(&whole);
    c
}

// ---------- per-high state ---------------------------------------------------

#[derive(Clone, Default)]
struct HighState {
    type_: Option<Rc<Datatype>>,
    addr_tied: bool,
    has_cover: bool,
    cover_dirty: bool,
    mark: bool,
    instances: Vec<VarnodeId>,
    cover: Cover,
}

struct Mock {
    highs: BTreeMap<u32, HighState>,
    vn_high: BTreeMap<u64, HighVariableId>,
    vn_def: BTreeMap<u64, OpId>,
    vn_cover: BTreeMap<u64, Cover>,
    vn_addr: BTreeMap<u64, Address>,
    vn_create: BTreeMap<u64, u64>,
    op_addr: BTreeMap<u64, Address>,
    op_parent_idx: BTreeMap<u64, int4>,
    op_order: BTreeMap<u64, uintm>,
    op_in0: BTreeMap<u64, VarnodeId>,
    merged: Vec<(HighVariableId, HighVariableId, bool)>,
}

impl Mock {
    fn new() -> Mock {
        Mock {
            highs: BTreeMap::new(),
            vn_high: BTreeMap::new(),
            vn_def: BTreeMap::new(),
            vn_cover: BTreeMap::new(),
            vn_addr: BTreeMap::new(),
            vn_create: BTreeMap::new(),
            op_addr: BTreeMap::new(),
            op_parent_idx: BTreeMap::new(),
            op_order: BTreeMap::new(),
            op_in0: BTreeMap::new(),
            merged: Vec::new(),
        }
    }
    fn hs(&self, h: HighVariableId) -> &HighState {
        self.highs.get(&h.0).expect("mock: missing high")
    }
    fn hs_mut(&mut self, h: HighVariableId) -> &mut HighState {
        self.highs.get_mut(&h.0).expect("mock: missing high")
    }
    /// Create a high with one instance varnode that carries `cover`.
    fn make_high(&mut self, h: u32, inst: u64, ty: Rc<Datatype>, cover: Cover) {
        let v = vid(inst);
        self.vn_high.insert(vkey(v), hid(h));
        self.vn_cover.insert(vkey(v), cover.clone());
        self.highs.insert(
            h,
            HighState {
                type_: Some(ty),
                addr_tied: false,
                has_cover: true,
                cover_dirty: false,
                mark: false,
                instances: vec![v],
                cover,
            },
        );
    }
}

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
        VarnodeViewLoc {
            addr: self
                .vn_addr
                .get(&vkey(vn))
                .cloned()
                .unwrap_or_else(Address::new_invalid),
        }
    }
}

/// Stub the `MergeContext` methods none of these tests reach. Anything reached
/// by accident panics loudly rather than returning a silently-wrong value.
macro_rules! unused {
    ($($name:ident ( $($a:ty),* ) -> $r:ty;)*) => {
        $( fn $name(&self $(, _: $a)* ) -> $r { unimplemented!(concat!(stringify!($name), " not under test")) } )*
    };
}
macro_rules! unused_mut {
    ($($name:ident ( $($a:ty),* ) -> $r:ty;)*) => {
        $( fn $name(&mut self $(, _: $a)* ) -> $r { unimplemented!(concat!(stringify!($name), " not under test")) } )*
    };
}

impl MergeContext for Mock {
    // --- reached by the tests -------------------------------------------------
    fn high_get_type(&mut self, h: HighVariableId) -> Rc<Datatype> {
        self.hs(h).type_.clone().expect("mock: high has no type")
    }
    fn high_is_addr_tied(&mut self, h: HighVariableId) -> bool {
        self.hs(h).addr_tied
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
    fn bank_merge_highs(
        &mut self,
        high1: HighVariableId,
        high2: HighVariableId,
        isspeculative: bool,
        _cache: &mut HighIntersectTest,
    ) -> kuna_base::error::KunaResult<()> {
        self.merged.push((high1, high2, isspeculative));
        let h2 = self.highs.remove(&high2.0).expect("merge: missing high2");
        let h1 = self.hs_mut(high1);
        h1.instances.extend(h2.instances);
        h1.cover.merge(&h2.cover);
        Ok(())
    }
    fn bank_update_cover(&mut self, h: HighVariableId) {
        self.hs_mut(h).cover_dirty = false;
    }
    fn bank_set_mark(&mut self, h: HighVariableId) {
        self.hs_mut(h).mark = true;
    }
    fn bank_clear_mark(&mut self, h: HighVariableId) {
        self.hs_mut(h).mark = false;
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
    fn vn_is_implied(&self, _vn: VarnodeId) -> bool {
        false
    }
    fn vn_is_spacebase(&self, _vn: VarnodeId) -> bool {
        false
    }
    fn vn_is_proto_partial(&self, _vn: VarnodeId) -> bool {
        false
    }
    fn vn_has_cover_flag(&self, _vn: VarnodeId) -> bool {
        true
    }
    fn vn_size(&self, _vn: VarnodeId) -> int4 {
        4
    }
    fn vn_cover_ref(&self, vn: VarnodeId) -> Option<Cover> {
        self.vn_cover.get(&vkey(vn)).cloned()
    }
    fn vn_create_index(&self, vn: VarnodeId) -> u64 {
        self.vn_create.get(&vkey(vn)).copied().unwrap_or(0)
    }
    fn vn_copy_shadow(&self, _a: VarnodeId, _b: VarnodeId) -> bool {
        false
    }
    fn vn_partial_copy_shadow(&self, _a: VarnodeId, _b: VarnodeId, _off: int4) -> bool {
        false
    }
    fn op_in(&self, op: OpId, slot: int4) -> Option<VarnodeId> {
        if slot == 0 {
            self.op_in0.get(&okey(op)).copied()
        } else {
            None
        }
    }
    fn op_parent_index(&self, op: OpId) -> int4 {
        self.op_parent_idx.get(&okey(op)).copied().unwrap_or(0)
    }
    fn op_addr(&self, op: OpId) -> Address {
        self.op_addr
            .get(&okey(op))
            .cloned()
            .unwrap_or_else(Address::new_invalid)
    }
    fn op_order(&self, op: OpId) -> uintm {
        self.op_order.get(&okey(op)).copied().unwrap_or(0)
    }

    // --- benign-default high flag accessors (mergeTestSpeculative chain) ------
    // Every high in these tests is an ordinary, unlocked, non-input, non-persist,
    // non-partial, non-isolated, symbol-less local — so the speculative merge
    // gate falls through to the cover test (the part we are exercising).
    fn high_is_type_lock(&mut self, _h: HighVariableId) -> bool {
        false
    }
    fn high_is_input(&mut self, _h: HighVariableId) -> bool {
        false
    }
    fn high_is_persist(&mut self, _h: HighVariableId) -> bool {
        false
    }
    fn high_is_extra_out(&mut self, _h: HighVariableId) -> bool {
        false
    }
    fn high_is_proto_partial(&mut self, _h: HighVariableId) -> bool {
        false
    }
    fn high_is_name_lock(&mut self, _h: HighVariableId) -> bool {
        false
    }
    fn bank_symbol(&self, _h: HighVariableId) -> Option<u64> {
        None
    }
    fn bank_symbol_offset(&self, _h: HighVariableId) -> int4 {
        0
    }
    fn bank_symbol_isolated(&self, _h: HighVariableId) -> bool {
        false
    }
    fn bank_input_varnode(&self, _h: HighVariableId) -> Option<VarnodeId> {
        None
    }
    fn bank_update_piece_intersections(&mut self, _p: MergePieceId) {}

    // --- stubs (unreached) ----------------------------------------------------
    unused! {
        high_has_copy_in1(HighVariableId) -> bool;
        high_has_copy_in2(HighVariableId) -> bool;
        piece_offset(MergePieceId) -> int4;
        piece_size(MergePieceId) -> int4;
        piece_group(MergePieceId) -> u64;
        piece_high(MergePieceId) -> HighVariableId;
        piece_num_intersection(MergePieceId) -> int4;
        bank_tied_addr(HighVariableId) -> Address;
        bank_tied_varnode(HighVariableId) -> Option<VarnodeId>;
        vn_is_input(VarnodeId) -> bool;
        vn_is_written(VarnodeId) -> bool;
        vn_is_addr_force(VarnodeId) -> bool;
        vn_is_annotation(VarnodeId) -> bool;
        vn_is_illegal_input(VarnodeId) -> bool;
        vn_is_indirect_only(VarnodeId) -> bool;
        vn_has_no_local_alias(VarnodeId) -> bool;
        vn_has_no_descend(VarnodeId) -> bool;
        vn_space_big_endian(VarnodeId) -> bool;
        vn_space_internal(VarnodeId) -> bool;
        vn_offset(VarnodeId) -> u64;
        vn_symbol_entry_offset(VarnodeId) -> Option<int4>;
        vn_characterize_overlap(VarnodeId, VarnodeId) -> int4;
        op_code(OpId) -> OpCode;
        op_out(OpId) -> Option<VarnodeId>;
        op_num_input(OpId) -> int4;
        op_parent(OpId) -> BlockId;
        op_slot(OpId, VarnodeId) -> int4;
        op_is_dead(OpId) -> bool;
        op_is_call(OpId) -> bool;
        op_is_marker(OpId) -> bool;
        op_is_indirect_creation(OpId) -> bool;
        op_is_partial_root(OpId) -> bool;
        op_previous(OpId) -> Option<OpId>;
        op_output_type_local(OpId) -> Rc<Datatype>;
        op_input_type_local(OpId, int4) -> Rc<Datatype>;
        op_cover_point(OpId) -> (int4, CoverPoint);
        indirect_effect_op(OpId) -> OpId;
        bblocks_get_size() -> int4;
        bblocks_get_block(int4) -> BlockId;
        block_ops(BlockId) -> Vec<OpId>;
        block_dominates(BlockId, BlockId) -> bool;
        ops_alive() -> Vec<OpId>;
        single_read_cover(VarnodeId, OpId) -> Cover;
        copy_pair_range(OpId, OpId) -> Cover;
        trim_op_input_prep(OpId, int4) -> (VarnodeId, Address, bool);
        addr_tied_ranges() -> Vec<AddrTiedRange>;
        multi_entry_symbols() -> Vec<u64>;
        symbol_linked_varnodes(u64) -> (Vec<VarnodeId>, int4);
        gather_pieces(VarnodeId, int4) -> Vec<(VarnodeId, int4)>;
    }
    fn piece_intersection(&self, _p: MergePieceId, _i: int4) -> MergePieceId {
        unimplemented!("piece_intersection not under test")
    }
    fn varnode_def_point(&self, _vn: VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
        unimplemented!("varnode_def_point not under test")
    }
    fn vn_descend(&self, _vn: VarnodeId) -> Vec<OpId> {
        unimplemented!("vn_descend not under test")
    }
    fn populate_affecting_ops(&self, _op_set: &mut PcodeOpSet) {
        unimplemented!("populate_affecting_ops not under test")
    }
    fn sort_op_nodes_by_high(&self, _nodes: &mut Vec<kuna_decomp::expression::PcodeOpNode>) {
        unimplemented!("sort_op_nodes_by_high not under test")
    }

    unused_mut! {
        bank_group_with(HighVariableId, int4, HighVariableId) -> kuna_base::error::KunaResult<()>;
        bank_set_unmerged(HighVariableId) -> ();
        bank_set_copy_in1(HighVariableId) -> ();
        bank_set_copy_in2(HighVariableId) -> ();
        bank_clear_copy_ins(HighVariableId) -> ();
        vn_set_implied(VarnodeId) -> ();
        vn_set_cover_dirty(VarnodeId) -> ();
        vn_clear_implied_set_explicit(VarnodeId) -> ();
        vn_clear_proto_partial(VarnodeId) -> ();
        op_set_input(OpId, VarnodeId, int4) -> kuna_base::error::KunaResult<()>;
        op_insert_before(OpId, OpId) -> ();
        op_insert_after(OpId, OpId) -> ();
        op_insert_begin(OpId, BlockId) -> ();
        op_insert_end_pred(OpId, OpId, int4) -> ();
        op_mark_non_printing(OpId) -> ();
        trim_op_output(OpId) -> kuna_base::error::KunaResult<()>;
        indirect_inherit_union(VarnodeId, OpId, OpId) -> ();
        symbol_set_merge_problems(u64) -> ();
        symbol_merge_warning(u64, int4, int4, int4) -> ();
    }
    fn copy_trim_op(
        &mut self,
        _in_vn: VarnodeId,
        _addr: Address,
        _trim_op: OpId,
    ) -> kuna_base::error::KunaResult<OpId> {
        unimplemented!("copy_trim_op not under test")
    }
    fn snip_reads_insert_point(&self, _vn: VarnodeId) -> (BlockId, Address, Option<OpId>) {
        unimplemented!("snip_reads_insert_point not under test")
    }
    fn build_dominant_copy(
        &mut self,
        _high: HighVariableId,
        _copy: &[OpId],
        _pos: int4,
        _size: int4,
    ) -> kuna_base::error::KunaResult<()> {
        unimplemented!("build_dominant_copy not under test")
    }
}

fn fresh_cache() -> HighIntersectTest {
    HighIntersectTest::new(PcodeOpSet::new(
        Box::new(Vec::new),
        Box::new(|_op, _vn| true),
    ))
}

// ============================================================================
// 1. BlockVarnode::find_front boundary search (merge.cc:43-61)
// ============================================================================

fn bv_list(indices: &[int4]) -> Vec<BlockVarnode> {
    // BlockVarnode::new needs a ctx for vn_def/op_parent_index. Build a tiny
    // mock that maps each vn to a defining op whose parent index is the target
    // block index, so the resulting BlockVarnode carries that index.
    let mut m = Mock::new();
    let mut out = Vec::new();
    for (k, &ix) in indices.iter().enumerate() {
        let v = vid(1000 + k as u64);
        let o = oid(2000 + k as u64);
        m.vn_def.insert(vkey(v), o);
        m.op_parent_idx.insert(okey(o), ix);
        out.push(BlockVarnode::new(v, &m));
    }
    out
}

#[test]
fn w7_s6_merge_find_front_boundaries() {
    // Empty list: max = -1, returns -1 (the unsigned 0u-1 -> int4 -1 path).
    assert_eq!(BlockVarnode::find_front(5, &[]), -1);

    // Single element present / absent.
    let single = bv_list(&[7]);
    assert_eq!(BlockVarnode::find_front(7, &single), 0);
    assert_eq!(BlockVarnode::find_front(6, &single), -1);
    assert_eq!(BlockVarnode::find_front(8, &single), -1);

    // Sorted list with a run of equal block indices: must return the FIRST of
    // the run (the binary search keeps `max=cur` on `>=`).
    let list = bv_list(&[1, 3, 3, 3, 5, 9]);
    assert_eq!(BlockVarnode::find_front(1, &list), 0);
    assert_eq!(BlockVarnode::find_front(3, &list), 1, "first of the 3-run");
    assert_eq!(BlockVarnode::find_front(5, &list), 4);
    assert_eq!(BlockVarnode::find_front(9, &list), 5, "last element");

    // Absent values around / between / outside the runs.
    assert_eq!(BlockVarnode::find_front(0, &list), -1, "below all");
    assert_eq!(BlockVarnode::find_front(2, &list), -1, "gap");
    assert_eq!(BlockVarnode::find_front(4, &list), -1, "gap");
    assert_eq!(BlockVarnode::find_front(10, &list), -1, "above all");
}

// ============================================================================
// 2. compare_high_by_block tie-break chain + totality (merge.hh:152-174)
// ============================================================================

/// Register a high `h` with first instance `inst` at first-instance address
/// `inst_off`, defined by op `op` at op-address `def_off` (or undefined when
/// `def_off` is None). Cover earliest block = `blk`.
#[allow(clippy::too_many_arguments)]
fn reg_high(
    m: &mut Mock,
    space: &Rc<AddrSpace>,
    h: u32,
    inst: u64,
    inst_off: u64,
    blk: int4,
    op: Option<(u64, u64)>,
) {
    let v = vid(inst);
    m.vn_high.insert(vkey(v), hid(h));
    m.vn_addr.insert(vkey(v), addr(space, inst_off));
    let cov = cover_block(blk, 0, 5);
    m.vn_cover.insert(vkey(v), cov.clone());
    if let Some((o, def_off)) = op {
        let opid = oid(o);
        m.vn_def.insert(vkey(v), opid);
        m.op_addr.insert(okey(opid), addr(space, def_off));
    }
    m.highs.insert(
        h,
        HighState {
            type_: Some(dt(4, type_metatype::TYPE_UNKNOWN)),
            addr_tied: false,
            has_cover: true,
            cover_dirty: false,
            mark: false,
            instances: vec![v],
            cover: cov,
        },
    );
}

#[test]
fn w7_s6_merge_compare_high_by_block_tiebreaks() {
    let sp = ram();
    let mut m = Mock::new();
    // a: earliest block 1; b: earliest block 2 -> cover decides, a before b.
    reg_high(&mut m, &sp, 1, 10, 0x100, 1, Some((1, 0x100)));
    reg_high(&mut m, &sp, 2, 11, 0x100, 2, Some((2, 0x100)));
    assert!(compare_high_by_block(&m, hid(1), hid(2)), "lower block first");
    assert!(!compare_high_by_block(&m, hid(2), hid(1)), "antisymmetric");

    // Equal cover block, different first-instance addr -> addr decides.
    let mut m = Mock::new();
    reg_high(&mut m, &sp, 1, 10, 0x100, 3, Some((1, 0x500)));
    reg_high(&mut m, &sp, 2, 11, 0x200, 3, Some((2, 0x010)));
    assert!(
        compare_high_by_block(&m, hid(1), hid(2)),
        "smaller instance addr first, even though def addr is larger"
    );
    assert!(!compare_high_by_block(&m, hid(2), hid(1)));

    // Equal cover + equal instance addr, different def-op addr -> def decides.
    let mut m = Mock::new();
    reg_high(&mut m, &sp, 1, 10, 0x100, 3, Some((1, 0x10)));
    reg_high(&mut m, &sp, 2, 11, 0x100, 3, Some((2, 0x20)));
    assert!(compare_high_by_block(&m, hid(1), hid(2)), "smaller def addr first");
    assert!(!compare_high_by_block(&m, hid(2), hid(1)));

    // Equal cover + equal instance addr, a has NO def, b has a def.
    // C++: def1==0 -> return def2 != 0  (a before b).
    let mut m = Mock::new();
    reg_high(&mut m, &sp, 1, 10, 0x100, 3, None);
    reg_high(&mut m, &sp, 2, 11, 0x100, 3, Some((2, 0x20)));
    assert!(
        compare_high_by_block(&m, hid(1), hid(2)),
        "null-def orders before a defined high"
    );
    // C++: for (a=defined, b=null) def1!=0 && def2==0 -> return false.
    assert!(
        !compare_high_by_block(&m, hid(2), hid(1)),
        "defined high does NOT order before a null-def high"
    );

    // Fully equal (same cover, same instance addr, both null def) -> neither
    // orders before the other: a strict-weak-order tie (totality).
    let mut m = Mock::new();
    reg_high(&mut m, &sp, 1, 10, 0x100, 3, None);
    reg_high(&mut m, &sp, 2, 11, 0x100, 3, None);
    assert!(!compare_high_by_block(&m, hid(1), hid(2)), "tie is not <");
    assert!(!compare_high_by_block(&m, hid(2), hid(1)), "tie is not > either");
}

// ============================================================================
// 3. compare_copy_by_in_varnode group/secondary ordering (merge.cc:1045)
// ============================================================================

#[test]
fn w7_s6_merge_compare_copy_by_in_varnode_chain() {
    let mut m = Mock::new();
    // Two ops with different input varnodes: order by input createIndex.
    let (o1, o2) = (oid(1), oid(2));
    let (va, vb) = (vid(10), vid(11));
    m.op_in0.insert(okey(o1), va);
    m.op_in0.insert(okey(o2), vb);
    m.vn_create.insert(vkey(va), 100);
    m.vn_create.insert(vkey(vb), 50);
    // vb has the smaller createIndex, so the op with input vb comes first.
    assert!(compare_copy_by_in_varnode(&m, o2, o1));
    assert!(!compare_copy_by_in_varnode(&m, o1, o2));

    // Same input varnode -> compare by defining-block index.
    let mut m = Mock::new();
    let (o1, o2) = (oid(1), oid(2));
    let v = vid(10);
    m.op_in0.insert(okey(o1), v);
    m.op_in0.insert(okey(o2), v);
    m.op_parent_idx.insert(okey(o1), 3);
    m.op_parent_idx.insert(okey(o2), 1);
    assert!(compare_copy_by_in_varnode(&m, o2, o1), "lower block index first");
    assert!(!compare_copy_by_in_varnode(&m, o1, o2));

    // Same input + same block -> compare by SeqNum order.
    let mut m = Mock::new();
    let (o1, o2) = (oid(1), oid(2));
    let v = vid(10);
    m.op_in0.insert(okey(o1), v);
    m.op_in0.insert(okey(o2), v);
    m.op_parent_idx.insert(okey(o1), 2);
    m.op_parent_idx.insert(okey(o2), 2);
    m.op_order.insert(okey(o1), 9);
    m.op_order.insert(okey(o2), 4);
    assert!(compare_copy_by_in_varnode(&m, o2, o1), "lower seqnum order first");
    assert!(!compare_copy_by_in_varnode(&m, o1, o2));
    // Full tie: identical key triple is not <.
    m.op_order.insert(okey(o1), 4);
    assert!(!compare_copy_by_in_varnode(&m, o1, o2));
    assert!(!compare_copy_by_in_varnode(&m, o2, o1));
}

// ============================================================================
// 4. merge_by_datatype order preservation across the erase-while-iterating
//    list dance (merge.cc:383-400)
// ============================================================================

#[test]
fn w7_s6_merge_by_datatype_preserves_discovery_order() {
    // Two distinct types interleaved across five highs. The C++ pulls the first
    // type's group out preserving order (h0,h2,h4) and leaves (h1,h3) in order;
    // the leftover group is then processed. Each group goes to mergeLinear.
    // We give every high a DISJOINT cover (distinct blocks, non-overlapping),
    // making them all mergeable, so mergeLinear collapses each type group into
    // its first member by repeated `merge(stack[0], h)`. The recorded merge
    // order witnesses the preserved discovery order.
    let t_a = dt(4, type_metatype::TYPE_INT);
    let t_b = dt(4, type_metatype::TYPE_UINT);

    let mut m = Mock::new();
    // highs h1..h5 with instances 1..5; types A,B,A,B,A; covers on blocks 1..5.
    let types = [&t_a, &t_b, &t_a, &t_b, &t_a];
    for (k, ty) in types.iter().enumerate() {
        let h = (k + 1) as u32;
        let cov = cover_block((k as int4) + 1, 0, 2); // disjoint blocks -> no intersect
        m.make_high(h, (k + 1) as u64, (*ty).clone(), cov);
    }

    let mut merge = Merge::new(fresh_cache());
    let range: Vec<VarnodeId> = (1..=5).map(vid).collect();
    merge.merge_by_datatype(&mut m, &range).unwrap();

    // Type A group is {h1,h3,h5} in discovery order; mergeLinear sorts them by
    // compareHighByBlock (block 1<3<5, already sorted) then merges h3,h5 into
    // h1. Type B group {h2,h4} merges h4 into h2.
    // The merge log therefore records (h1<-h3),(h1<-h5) for A and (h2<-h4) for B.
    // Crucially the A group is processed entirely before the B group (the
    // erase-while-iterating dance pulls one whole type group at a time).
    let log = &m.merged;
    assert_eq!(log.len(), 3, "exactly three merges: two in A, one in B");

    // All type-A merges come before the type-B merge (group-at-a-time order).
    let a_targets: Vec<u32> = log
        .iter()
        .filter(|(dst, _, _)| *dst == hid(1))
        .map(|(_, src, _)| src.0)
        .collect();
    assert_eq!(a_targets, vec![3, 5], "A group merged in discovery order h3 then h5");

    let b = log.iter().find(|(dst, _, _)| *dst == hid(2));
    assert!(b.is_some(), "B group produced its merge");
    assert_eq!(b.unwrap().1, hid(4), "B group merged h4 into h2");

    // The first two log entries are the A group, the third is B (group ordering).
    assert_eq!(log[0].0, hid(1));
    assert_eq!(log[1].0, hid(1));
    assert_eq!(log[2].0, hid(2));
    // All merges here are speculative (mergeLinear passes isspeculative=true).
    assert!(log.iter().all(|(_, _, spec)| *spec));
}

// ============================================================================
// 5. MergeIntersect::intersection — symmetry / self / cache / cover walk
// ============================================================================

#[test]
fn w7_s6_merge_intersection_self_and_disjoint_and_overlap() {
    let mut m = Mock::new();
    // h1, h2 disjoint covers on DIFFERENT blocks -> intersect_list empty -> no
    // intersection.
    m.make_high(1, 1, dt(4, type_metatype::TYPE_INT), cover_block(1, 0, 5));
    m.make_high(2, 2, dt(4, type_metatype::TYPE_INT), cover_block(2, 0, 5));
    // h3 shares block 1 with h1 AND its instance cover overlaps at level 2.
    m.make_high(3, 3, dt(4, type_metatype::TYPE_INT), cover_block(1, 1, 4));

    let mut cache = fresh_cache();

    // self never intersects.
    assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(1)));

    // disjoint blocks: no intersection, symmetric.
    assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(2)));
    assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(2), hid(1)));

    // h1 vs h3: same block, overlapping instance covers -> intersection true,
    // symmetric.
    let ab = MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(3));
    let ba = MergeIntersect::intersection(&mut m, &mut cache, hid(3), hid(1));
    assert!(ab, "same-block overlapping instance covers intersect");
    assert_eq!(ab, ba, "intersection is symmetric");
}

#[test]
fn w7_s6_merge_intersection_cache_reused_when_clean() {
    // After a clean (non-dirty) test, the (a,b) and (b,a) edges are cached and
    // reused. We witness reuse by mutating the underlying cover AFTER the first
    // test WITHOUT marking dirty: a re-query must return the stale cached value
    // (proving the cache short-circuit fired) — matching the C++ which only
    // re-walks covers when a high is cover-dirty.
    let mut m = Mock::new();
    m.make_high(1, 1, dt(4, type_metatype::TYPE_INT), cover_block(1, 0, 5));
    m.make_high(2, 2, dt(4, type_metatype::TYPE_INT), cover_block(2, 0, 5));

    let mut cache = fresh_cache();
    assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(2)));

    // Now make their covers actually overlap, but leave cover_dirty=false.
    m.hs_mut(hid(2)).cover = cover_block(1, 1, 4);
    m.vn_cover.insert(vkey(vid(2)), cover_block(1, 1, 4));

    // Clean highs -> cached "false" is reused despite the new geometry.
    assert!(
        !MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(2)),
        "clean highs reuse the cached result"
    );
    // Both (a,b) and (b,a) cached -> reverse query also reuses.
    assert!(!MergeIntersect::intersection(&mut m, &mut cache, hid(2), hid(1)));

    // Marking dirty forces a re-walk: now the real overlap is seen.
    m.hs_mut(hid(2)).cover_dirty = true;
    assert!(
        MergeIntersect::intersection(&mut m, &mut cache, hid(1), hid(2)),
        "dirtying the high forces a cover re-walk that sees the overlap"
    );
}
