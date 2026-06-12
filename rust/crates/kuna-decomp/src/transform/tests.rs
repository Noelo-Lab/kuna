// Tests for the `transform` placeholder-graph (included into `mod tests` in
// transform.rs, hence plain `//` comments here).
//
// Covered:
//   - LaneDescription geometry (uniform / two-lane / getBoundary / subset /
//     restriction / extension), matching the C++ exactly.
//   - LanedRegister parse/iter/mask semantics.
//   - placeholder-graph construction: every `new*`/`get*` factory, the
//     pieceMap/newVarnodes/newOps allocation order, the def/output/input
//     cross-links, preexistingGuard, getPiece dedup.
//   - apply() materialization on a synthetic pure-varnode transform: the
//     pieceMap-then-newVarnodes visit order produces ASCENDING create-indices on
//     the real IR (the load-bearing allocation order).
//   - the W6/W3-varnode seams return structured errors (op materialization,
//     piece materialization, input transform).

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace,
};

use crate::dtype::{type_metatype, Datatype};
use crate::seams::{Architecture, VarnodeId};

// --- test harness -------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        3,
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

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// A fresh free varnode in ram at `off`, size `s`.
fn mk_vn(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(s, Address::new(ram, off), dt(s))
}

/// A fresh free constant varnode of value `val`, size `s`.
fn mk_const(fd: &mut Funcdata, val: u64, s: int4) -> VarnodeId {
    fd.new_constant(s, val)
}

// =============================================================================
// LaneDescription
// =============================================================================

#[test]
fn lane_description_uniform() {
    // 8-byte whole, 2-byte lanes → 4 lanes at positions 0,2,4,6.
    let d = LaneDescription::uniform(8, 2);
    assert_eq!(d.get_num_lanes(), 4);
    assert_eq!(d.get_whole_size(), 8);
    for i in 0..4 {
        assert_eq!(d.get_size(i), 2);
        assert_eq!(d.get_position(i), i * 2);
    }
}

#[test]
fn lane_description_two_lane() {
    // origSize 8, lo 3, hi 5 → positions 0 and 3.
    let d = LaneDescription::two_lane(8, 3, 5);
    assert_eq!(d.get_num_lanes(), 2);
    assert_eq!(d.get_size(0), 3);
    assert_eq!(d.get_size(1), 5);
    assert_eq!(d.get_position(0), 0);
    assert_eq!(d.get_position(1), 3);
}

#[test]
fn lane_description_get_boundary() {
    let d = LaneDescription::uniform(8, 2); // positions 0,2,4,6 ; whole 8
    assert_eq!(d.get_boundary(0), 0);
    assert_eq!(d.get_boundary(2), 1);
    assert_eq!(d.get_boundary(4), 2);
    assert_eq!(d.get_boundary(6), 3);
    assert_eq!(d.get_boundary(8), 4); // position == wholeSize → numLanes
    assert_eq!(d.get_boundary(1), -1); // not a boundary
    assert_eq!(d.get_boundary(-1), -1); // out of bounds
    assert_eq!(d.get_boundary(9), -1); // out of bounds
}

#[test]
fn lane_description_subset_whole_is_noop() {
    let mut d = LaneDescription::uniform(8, 2);
    assert!(d.subset(0, 8)); // subrange is the whole range
    assert_eq!(d.get_num_lanes(), 4);
    assert_eq!(d.get_whole_size(), 8);
}

#[test]
fn lane_description_subset_trim() {
    // Trim to the middle two lanes (offset 2, size 4) of an 8/2 description.
    let mut d = LaneDescription::uniform(8, 2);
    assert!(d.subset(2, 4));
    assert_eq!(d.get_whole_size(), 4);
    assert_eq!(d.get_num_lanes(), 2);
    // Repositioned from 0.
    assert_eq!(d.get_position(0), 0);
    assert_eq!(d.get_position(1), 2);
    assert_eq!(d.get_size(0), 2);
    assert_eq!(d.get_size(1), 2);
}

#[test]
fn lane_description_subset_partial_fails() {
    let mut d = LaneDescription::uniform(8, 2);
    // offset 1 is not a lane boundary → false.
    assert!(!d.subset(1, 4));
}

#[test]
fn lane_description_restriction() {
    let d = LaneDescription::uniform(8, 2); // positions 0,2,4,6
                                            // Subset: numLanes=4, skip=0; truncate bytePos=2 size=4 → lanes 1..3.
    let res = d.restriction(4, 0, 2, 4);
    assert_eq!(res, Some((2, 1))); // (resNumLanes, resSkipLanes)

    // A truncation that splits a lane (size 3 from pos 2) fails.
    assert_eq!(d.restriction(4, 0, 2, 3), None);
}

#[test]
fn lane_description_extension() {
    let d = LaneDescription::uniform(8, 2);
    // skip=1 (pos 2); extension bytePos=2 → resSkip from pos 0; size=4 → lanes 0..2.
    let res = d.extension(4, 1, 2, 4);
    assert_eq!(res, Some((2, 0)));

    // Extension going out of bounds fails.
    assert_eq!(d.extension(4, 0, 2, 4), None); // pos 0 - 2 = -2 → boundary -1
}

// =============================================================================
// LanedRegister
// =============================================================================

#[test]
fn laned_register_parse_sizes() {
    let mut lr = LanedRegister::new();
    lr.parse_sizes(8, "1,2,4,8").unwrap();
    assert_eq!(lr.get_whole_size(), 8);
    // Bit mask: bits 1,2,4,8 set.
    let expect = (1u32 << 1) | (1u32 << 2) | (1u32 << 4) | (1u32 << 8);
    assert_eq!(lr.get_size_bit_mask(), expect);
    assert!(lr.allowed_lane(1));
    assert!(lr.allowed_lane(2));
    assert!(lr.allowed_lane(4));
    assert!(lr.allowed_lane(8));
    assert!(!lr.allowed_lane(3));
}

#[test]
fn laned_register_parse_bad_size() {
    let mut lr = LanedRegister::new();
    // 17 is out of the [0,16] range → error.
    assert!(lr.parse_sizes(8, "1,17").is_err());
    // Empty token (trailing comma) → failed parse → -1 → error.
    let mut lr2 = LanedRegister::new();
    assert!(lr2.parse_sizes(8, "1,").is_err());
}

#[test]
fn laned_register_with_mask_and_add() {
    let mut lr = LanedRegister::with_mask(16, (1u32 << 4) | (1u32 << 8));
    assert_eq!(lr.get_whole_size(), 16);
    assert!(lr.allowed_lane(4));
    assert!(lr.allowed_lane(8));
    assert!(!lr.allowed_lane(2));
    lr.add_lane_size(2);
    assert!(lr.allowed_lane(2));
}

#[test]
fn laned_register_iter_sizes_ascending() {
    // bits 1,2,4,8 set → iterate 1,2,4,8 in ascending order.
    let lr = LanedRegister::with_mask(8, (1u32 << 1) | (1u32 << 2) | (1u32 << 4) | (1u32 << 8));
    let sizes: Vec<int4> = lr.iter_sizes().collect();
    assert_eq!(sizes, vec![1, 2, 4, 8]);
}

#[test]
fn laned_register_iter_sizes_empty() {
    let lr = LanedRegister::new(); // mask 0
    let sizes: Vec<int4> = lr.iter_sizes().collect();
    assert!(sizes.is_empty());
}

// =============================================================================
// Placeholder factory + allocation-order graph build
// =============================================================================

#[test]
fn new_unique_and_constant_append_to_newvarnodes_in_order() {
    let mut tm = TransformManager::new();
    let a = tm.new_unique(4);
    let b = tm.new_constant(2, 0, 0x1234);
    let c = tm.new_unique(8);
    // newVarnodes appended in factory-call order.
    assert_eq!(a, TVarRef::New(0));
    assert_eq!(b, TVarRef::New(1));
    assert_eq!(c, TVarRef::New(2));
    // type + size checks.
    assert_eq!(tm.var(a).get_type(), tvar_type::normal_temp);
    assert_eq!(tm.var(a).get_byte_size(), 4);
    assert_eq!(tm.var(a).get_bit_size(), 32);
    assert_eq!(tm.var(b).get_type(), tvar_type::constant);
    assert_eq!(tm.var(b).get_val(), 0x1234);
}

#[test]
fn new_constant_strips_lsb_offset_and_masks() {
    let mut tm = TransformManager::new();
    // size 2 (mask 0xffff), lsb_offset 8 → (0xAABBCCDD >> 8) & 0xffff = 0xBBCC.
    let r = tm.new_constant(2, 8, 0xAABBCCDD);
    assert_eq!(tm.var(r).get_val(), 0xBBCC);
}

#[test]
fn new_split_uniform_lanes_keys_on_create_index() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let ci = fd.vbank().get(vn).unwrap().get_create_index() as int4;
    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(8, 2);
    let lane0 = tm.new_split(&fd, vn, &desc);
    assert_eq!(lane0, TVarRef::Piece { key: ci, idx: 0 });
    // 4 lanes, last is split_terminator.
    for i in 0..4usize {
        let r = TVarRef::Piece { key: ci, idx: i };
        assert_eq!(tm.var(r).get_byte_size(), 2);
        // ram is little-endian here → all lanes are real pieces (preserveAddress true).
        assert_eq!(tm.var(r).get_type(), tvar_type::piece);
        // val carries the bit position.
        assert_eq!(tm.var(r).get_val(), (i as u64) * 16);
    }
    let last = TVarRef::Piece { key: ci, idx: 3 };
    assert_eq!(tm.var(last).get_flags() & tvar_flags::split_terminator, tvar_flags::split_terminator);
    let mid = TVarRef::Piece { key: ci, idx: 1 };
    assert_eq!(tm.var(mid).get_flags() & tvar_flags::split_terminator, 0);
}

#[test]
fn new_split_of_constant_makes_constant_lanes() {
    let mut fd = build_fd();
    let vn = mk_const(&mut fd, 0xAABBCCDD, 4);
    let ci = fd.vbank().get(vn).unwrap().get_create_index() as int4;
    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(4, 2); // two 2-byte lanes
    tm.new_split(&fd, vn, &desc);
    // lane 0 (bitpos 0): 0xAABBCCDD & 0xffff = 0xCCDD
    let l0 = TVarRef::Piece { key: ci, idx: 0 };
    assert_eq!(tm.var(l0).get_type(), tvar_type::constant);
    assert_eq!(tm.var(l0).get_val(), 0xCCDD);
    // lane 1 (bitpos 16): (0xAABBCCDD >> 16) & 0xffff = 0xAABB
    let l1 = TVarRef::Piece { key: ci, idx: 1 };
    assert_eq!(tm.var(l1).get_type(), tvar_type::constant);
    assert_eq!(tm.var(l1).get_val(), 0xAABB);
}

#[test]
fn new_split_subset_rebases_positions() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let ci = fd.vbank().get(vn).unwrap().get_create_index() as int4;
    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(8, 2); // positions 0,2,4,6
                                               // subset: 2 lanes starting at lane 1 (position 2) → base bit pos 16.
    tm.new_split_subset(&fd, vn, &desc, 2, 1);
    // lane 0: position(1)*8 - 16 = 0
    let l0 = TVarRef::Piece { key: ci, idx: 0 };
    assert_eq!(tm.var(l0).get_val(), 0);
    // lane 1: position(2)*8 - 16 = 16
    let l1 = TVarRef::Piece { key: ci, idx: 1 };
    assert_eq!(tm.var(l1).get_val(), 16);
}

#[test]
fn new_piece_byte_aligned_uses_overlapping_storage() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let mut tm = TransformManager::new();
    // bitSize 16, lsbOffset 0 → byte-aligned, ram space → piece (overlap).
    let r = tm.new_piece(&fd, vn, 16, 0);
    assert_eq!(tm.var(r).get_type(), tvar_type::piece);
    assert_eq!(tm.var(r).get_byte_size(), 2);
    assert_eq!(tm.var(r).get_bit_size(), 16);
    // unaligned lsbOffset → piece_temp.
    let mut tm2 = TransformManager::new();
    let r2 = tm2.new_piece(&fd, vn, 8, 4); // 4 bits not byte-aligned
    assert_eq!(tm2.var(r2).get_type(), tvar_type::piece_temp);
}

#[test]
fn new_piece_in_unique_space_is_piece_temp() {
    let mut fd = build_fd();
    // A varnode in the unique (internal) space → preserveAddress false → piece_temp.
    let vn = fd.vbank_mut().create_unique(8, dt(8));
    let mut tm = TransformManager::new();
    let r = tm.new_piece(&fd, vn, 16, 0);
    assert_eq!(tm.var(r).get_type(), tvar_type::piece_temp);
}

#[test]
fn get_preexisting_varnode_dedups_and_constant_path() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let mut tm = TransformManager::new();
    let a = tm.get_preexisting_varnode(&fd, vn);
    // Second call returns the same pieceMap entry (no new placeholder).
    let b = tm.get_preexisting_varnode(&fd, vn);
    assert_eq!(a, b);
    assert_eq!(tm.var(a).get_type(), tvar_type::preexisting);

    // A constant varnode goes through newConstant (a New(_) ref), not pieceMap.
    let cvn = mk_const(&mut fd, 7, 4);
    let c = tm.get_preexisting_varnode(&fd, cvn);
    assert!(matches!(c, TVarRef::New(_)));
    assert_eq!(tm.var(c).get_type(), tvar_type::constant);
    assert_eq!(tm.var(c).get_val(), 7);
}

#[test]
fn get_piece_dedups_and_rejects_mismatch() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let mut tm = TransformManager::new();
    let a = tm.get_piece(&fd, vn, 16, 0).unwrap();
    let b = tm.get_piece(&fd, vn, 16, 0).unwrap();
    assert_eq!(a, b);
    // Different bitSize/offset for the same Varnode → error.
    assert!(tm.get_piece(&fd, vn, 32, 0).is_err());
}

#[test]
fn get_split_dedups() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let desc = LaneDescription::uniform(8, 2);
    let mut tm = TransformManager::new();
    let a = tm.get_split(&fd, vn, &desc);
    let b = tm.get_split(&fd, vn, &desc);
    assert_eq!(a, b);
}

// --- ops ----------------------------------------------------------------------

#[test]
fn new_ops_append_in_order_and_link() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let real_op = fd.new_op(2, Address::new(ram, 0x1000));
    let mut tm = TransformManager::new();
    // newOpReplace then a following newOp.
    let rop0 = tm.new_op_replace(2, OpCode::CPUI_INT_ADD, real_op);
    let rop1 = tm.new_op(2, OpCode::CPUI_INT_ADD, rop0);
    assert_eq!(rop0, TOpRef(0));
    assert_eq!(rop1, TOpRef(1));
    assert_eq!(tm.op(rop0).get_special(), top_special::op_replacement);
    assert_eq!(tm.op(rop0).get_opc(), OpCode::CPUI_INT_ADD);
    assert_eq!(tm.op(rop1).get_special(), 0);
    // newOp inherits follow->op as its `op`.
    // (verified indirectly through follow linkage below)

    // opSetInput / opSetOutput wiring.
    let a = tm.new_unique(4);
    let b = tm.new_unique(4);
    let out = tm.new_unique(4);
    tm.op_set_input(rop0, a, 0);
    tm.op_set_input(rop0, b, 1);
    tm.op_set_output(rop0, out);
    assert_eq!(tm.op(rop0).get_in(0), Some(a));
    assert_eq!(tm.op(rop0).get_in(1), Some(b));
    assert_eq!(tm.op(rop0).get_out(), Some(out));
    // opSetOutput marks the var's def.
    assert_eq!(tm.var(out).get_def(), Some(rop0));
}

#[test]
fn new_preexisting_op_carries_flag() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let real_op = fd.new_op(1, Address::new(ram, 0x1000));
    let mut tm = TransformManager::new();
    let rop = tm.new_preexisting_op(1, OpCode::CPUI_COPY, real_op);
    assert_eq!(tm.op(rop).get_special(), top_special::op_preexisting);
}

#[test]
fn preexisting_guard_logic() {
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let mut tm = TransformManager::new();
    let piece = tm.new_piece(&fd, vn, 16, 0); // a piece-type var
    let uniq = tm.new_unique(4); // a normal_temp var
                                 // slot 0 → always true.
    assert!(tm.preexisting_guard(0, piece));
    assert!(tm.preexisting_guard(0, uniq));
    // slot != 0 with a piece → false (op was/will be visited on slot 0).
    assert!(!tm.preexisting_guard(1, piece));
    // slot != 0 with a non-piece → true.
    assert!(tm.preexisting_guard(1, uniq));
}

// =============================================================================
// apply(): allocation order on a synthetic pure-varnode transform
// =============================================================================

// A transform with NO placeholder ops and only constant/unique placeholders runs
// apply() to completion (no W6/xref seam reached) and lets us assert that the
// pieceMap-then-newVarnodes visit order produces ASCENDING create-indices on the
// real IR — the load-bearing allocation order.

#[test]
fn apply_materializes_newvarnodes_in_creation_order() {
    let mut fd = build_fd();
    let mut tm = TransformManager::new();
    // Three free placeholders, no ops, no pieces.
    let a = tm.new_unique(4);
    let b = tm.new_constant(2, 0, 0x1234);
    let c = tm.new_unique(8);

    let before = fd.vbank().get_create_index();
    tm.apply(&mut fd).expect("pure-varnode apply should succeed (no seams reached)");

    // Each placeholder now has a materialized replacement with a create-index in
    // factory-call order (a < b < c).
    let va = tm.var(a).get_replacement().expect("a materialized");
    let vb = tm.var(b).get_replacement().expect("b materialized");
    let vc = tm.var(c).get_replacement().expect("c materialized");
    let cia = fd.vbank().get(va).unwrap().get_create_index();
    let cib = fd.vbank().get(vb).unwrap().get_create_index();
    let cic = fd.vbank().get(vc).unwrap().get_create_index();
    assert!(cia < cib, "newVarnodes visited a before b");
    assert!(cib < cic, "newVarnodes visited b before c");
    assert!(cia >= before, "new IR allocated after the pre-apply counter");
}

#[test]
fn apply_visits_piece_map_before_newvarnodes() {
    // pieceMap entries (here: a constant-split, materialized as constants — no
    // seam) are visited BEFORE the newVarnodes list.  Build a constant split and a
    // trailing newUnique and assert the split lanes get the lower create-indices.
    let mut fd = build_fd();
    let cvn = mk_const(&mut fd, 0xAABBCCDD, 4);
    let ci = fd.vbank().get(cvn).unwrap().get_create_index() as int4;
    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(4, 2);
    tm.new_split(&fd, cvn, &desc); // two constant lanes in pieceMap
    let trailing = tm.new_unique(4); // a newVarnodes entry

    tm.apply(&mut fd).expect("constant-split + unique apply should succeed");

    let lane0 = TVarRef::Piece { key: ci, idx: 0 };
    let lane1 = TVarRef::Piece { key: ci, idx: 1 };
    let v0 = tm.var(lane0).get_replacement().unwrap();
    let v1 = tm.var(lane1).get_replacement().unwrap();
    let vt = tm.var(trailing).get_replacement().unwrap();
    let ci0 = fd.vbank().get(v0).unwrap().get_create_index();
    let ci1 = fd.vbank().get(v1).unwrap().get_create_index();
    let cit = fd.vbank().get(vt).unwrap().get_create_index();
    // pieceMap lane order is least→most significant.
    assert!(ci0 < ci1, "lane 0 materialized before lane 1");
    // pieceMap visited before newVarnodes.
    assert!(ci1 < cit, "pieceMap entries materialized before the newVarnodes list");
}

#[test]
fn apply_constant_replacement_value_round_trips() {
    let mut fd = build_fd();
    let mut tm = TransformManager::new();
    let r = tm.new_constant(2, 0, 0xBEEF);
    tm.apply(&mut fd).expect("constant apply succeeds");
    let vn = tm.var(r).get_replacement().unwrap();
    let v = fd.vbank().get(vn).unwrap();
    assert!(v.is_constant());
    assert_eq!(v.get_offset(), 0xBEEF);
    assert_eq!(v.get_size(), 2);
}

// =============================================================================
// apply(): the W6 / W3-varnode seams surface as structured errors
// =============================================================================

#[test]
fn apply_with_op_hits_opsetopcode_seam() {
    // A transform containing a placeholder op reaches createOps →
    // createReplacement → opSetOpcode(OpCode), the W6 TypeOp seam.
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let real_op = fd.new_op(0, Address::new(ram, 0x1000));
    let mut tm = TransformManager::new();
    let _rop = tm.new_op_replace(0, OpCode::CPUI_COPY, real_op);
    let err = tm.apply(&mut fd).expect_err("op materialization must hit the W6 seam");
    let msg = format!("{err}");
    assert!(msg.contains("opSetOpcode"), "seam error mentions opSetOpcode: {msg}");
}

#[test]
fn apply_with_piece_hits_transfer_properties_seam() {
    // A non-constant piece reaches createVarnodes → createReplacement (piece) →
    // transferVarnodeProperties, the funcdata_varnode seam.
    let mut fd = build_fd();
    let vn = mk_vn(&mut fd, 0x100, 8);
    let mut tm = TransformManager::new();
    let desc = LaneDescription::uniform(8, 2); // real (non-constant) pieces
    tm.new_split(&fd, vn, &desc);
    let err = tm.apply(&mut fd).expect_err("piece materialization must hit a seam");
    let msg = format!("{err}");
    assert!(
        msg.contains("transferVarnodeProperties") || msg.contains("newVarnodeOut"),
        "seam error mentions a piece-materialization seam: {msg}"
    );
}

#[test]
fn apply_constant_iop_hits_getopfromconst_seam() {
    let mut fd = build_fd();
    // An iop-space varnode → newIop placeholder → constant_iop materialization.
    let iop = Rc::clone(fd.get_arch().manage().get_iop_space().unwrap());
    let iop_vn = fd.vbank_mut().create(8, Address::new(iop, 0x1234), dt(8));
    let mut tm = TransformManager::new();
    let _r = tm.new_iop(&fd, iop_vn);
    let err = tm.apply(&mut fd).expect_err("constant_iop must hit the getOpFromConst seam");
    let msg = format!("{err}");
    assert!(msg.contains("getOpFromConst"), "seam error mentions getOpFromConst: {msg}");
}

// --- inherit_indirect ---------------------------------------------------------

#[test]
fn preserve_address_rules() {
    let mut fd = build_fd();
    let ram_vn = mk_vn(&mut fd, 0x100, 8);
    let uniq_vn = fd.vbank_mut().create_unique(8, dt(8));
    let tm = TransformManager::new();
    // aligned + non-internal → true.
    assert!(tm.preserve_address(&fd, ram_vn, 16, 0));
    // aligned + internal (unique) space → false.
    assert!(!tm.preserve_address(&fd, uniq_vn, 16, 0));
    // misaligned lsbOffset → false regardless.
    assert!(!tm.preserve_address(&fd, ram_vn, 16, 4));
}
