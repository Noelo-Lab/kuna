//! Tests for `RuleSimdShuffleLane` (kuna `simdlane`): the
//! `SUBPIECE(pshufb(src,<const mask>),k)` -> `SUBPIECE(src,mask[k])` rewrite, the
//! lane-selection table it rests on, and every clause that declines it.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::action::Rule;
use crate::context::{ArchContext, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::varnode::{DefOpInfo, VarnodeBank};

/// The user-op id the fixture registers as `pshufb`.
const SHUF_ID: u64 = 7;

// --- the lane-selection table ----------------------------------------------

#[test]
fn broadcast_mask_selects_lane_zero_for_every_lane() {
    // The `pxor xmm2,xmm2; pshufb xmm0,xmm2` idiom: an all-zero 16-byte mask.
    for k in 0..16 {
        assert_eq!(shuffle_lane_source(0, 16, 16, k), Some(Some(0)));
    }
}

#[test]
fn eight_byte_mask_is_decoded_per_lane() {
    // MMX width, an identity mask: lane k selects source lane k.  The offset is
    // little-endian, so lane 0's byte is the least significant one.
    let mask: u64 = 0x0706_0504_0302_0100;
    for k in 0..8 {
        assert_eq!(shuffle_lane_source(mask, 8, 8, k), Some(Some(k)));
    }
}

#[test]
fn high_bit_mask_byte_zeroes_the_lane() {
    assert_eq!(shuffle_lane_source(0x80, 8, 8, 0), Some(None));
    assert_eq!(shuffle_lane_source(0xff, 8, 8, 0), Some(None));
}

#[test]
fn lane_index_is_masked_to_the_vector_width() {
    // PSHUFB uses only the low log2(N) bits of a non-zeroing mask byte.
    assert_eq!(shuffle_lane_source(0x1f, 8, 16, 0), Some(Some(0xf)));
    assert_eq!(shuffle_lane_source(0x0f, 8, 8, 0), Some(Some(7)));
}

#[test]
fn a_wide_nonzero_mask_is_declined_not_guessed() {
    // >8 bytes does not fit a uintb offset, so only the provable all-zero value
    // is accepted; anything else must decline rather than read a truncated mask.
    assert_eq!(shuffle_lane_source(1, 16, 16, 0), None);
}

#[test]
fn an_out_of_range_lane_is_declined() {
    assert_eq!(shuffle_lane_source(0, 16, 16, 16), None);
    assert_eq!(shuffle_lane_source(0, 16, 16, -1), None);
}

// --- harness (mirrors kuna_booleanmask/tests.rs) ----------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd(shuffle_ids: Vec<u32>) -> Funcdata {
    build_fd_gated(shuffle_ids, true)
}

fn build_fd_gated(shuffle_ids: Vec<u32>, gate: bool) -> Funcdata {
    let manage = build_manager();
    let mut ctx = ArchContext::new(manage);
    ctx.simd_lane_fold = gate;
    ctx.simd_shuffle_userops = shuffle_ids;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn no_replace(
) -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| Ok(())
}

fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn set_def(fd: &mut Funcdata, vn: VarnodeId, defop: OpId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn wire_no_descend(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// Build `out = SUBPIECE(CALLOTHER(#userop, src, #mask), k)` and return
/// `(subpiece_op, src)`.
fn build_lane_read(
    fd: &mut Funcdata,
    userop: u64,
    vecsize: int4,
    mask_is_constant: bool,
    k: u64,
    outsize: int4,
) -> (OpId, VarnodeId) {
    let shuf = mk_op(fd, 0x100, 3, OpCode::CPUI_CALLOTHER);
    let idc = fd.new_constant(4, userop);
    wire_no_descend(fd, idc, shuf, 0);
    // `src` is given a definition: a free Varnode may not gain a second
    // descendant, and the rewrite adds one (the lane read re-anchors on it).
    let srcdef = mk_op(fd, 0x0f8, 1, OpCode::CPUI_COPY);
    let src = fd.new_unique(vecsize, Some(unk(vecsize)));
    let src = set_def(fd, src, srcdef);
    wire(fd, src, shuf, 1);
    let mask = if mask_is_constant {
        fd.new_constant(vecsize, 0)
    } else {
        fd.new_unique(vecsize, Some(unk(vecsize)))
    };
    wire(fd, mask, shuf, 2);
    let shufout = fd.new_unique(vecsize, Some(unk(vecsize)));
    let shufout = set_def(fd, shufout, shuf);

    let op = mk_op(fd, 0x104, 2, OpCode::CPUI_SUBPIECE);
    wire(fd, shufout, op, 0);
    let kc = fd.new_constant(4, k);
    wire(fd, kc, op, 1);
    let out = fd.new_unique(outsize, Some(unk(outsize)));
    let _out = set_def(fd, out, op);
    (op, src)
}

fn subpiece_shape(fd: &Funcdata, op: OpId) -> (OpCode, Option<VarnodeId>, u64) {
    let o = fd.obank().get(op).unwrap();
    let k = o
        .get_in(1)
        .and_then(|v| fd.vbank().get(v).map(|vn| vn.get_offset()))
        .unwrap_or(u64::MAX);
    (o.code(), o.get_in(0), k)
}

// --- the rewrite ------------------------------------------------------------

#[test]
fn broadcast_lane_read_is_reanchored_on_the_source_lane_zero() {
    let mut fd = build_fd(vec![SHUF_ID as u32]);
    let (op, src) = build_lane_read(&mut fd, SHUF_ID, 16, true, 5, 1);

    let mut rule = RuleSimdShuffleLane::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    let (code, in0, k) = subpiece_shape(&fd, op);
    assert_eq!(code, OpCode::CPUI_SUBPIECE);
    assert_eq!(in0, Some(src), "lane read now reads the shuffle's SOURCE");
    assert_eq!(k, 0, "an all-zero mask broadcasts source lane 0");
}

#[test]
fn the_gate_off_leaves_the_lane_read_alone() {
    // ArchContext gate off AND the rule's own override off.
    let mut fd = build_fd_gated(vec![SHUF_ID as u32], false);
    let (op, _src) = build_lane_read(&mut fd, SHUF_ID, 16, true, 5, 1);
    let mut rule = RuleSimdShuffleLane::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn a_non_constant_mask_is_declined() {
    let mut fd = build_fd(vec![SHUF_ID as u32]);
    let (op, _src) = build_lane_read(&mut fd, SHUF_ID, 16, false, 5, 1);
    let mut rule = RuleSimdShuffleLane::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn a_multi_byte_lane_read_is_declined() {
    // Two lanes of a broadcast are a concatenation, not another SUBPIECE.
    let mut fd = build_fd(vec![SHUF_ID as u32]);
    let (op, _src) = build_lane_read(&mut fd, SHUF_ID, 16, true, 4, 2);
    let mut rule = RuleSimdShuffleLane::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn an_unnamed_user_op_is_declined() {
    // Same three-input CALLOTHER shape, a user op the architecture did not
    // register under a shuffle name.
    let mut fd = build_fd(vec![SHUF_ID as u32]);
    let (op, _src) = build_lane_read(&mut fd, SHUF_ID + 1, 16, true, 5, 1);
    let mut rule = RuleSimdShuffleLane::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn an_architecture_with_no_shuffle_user_op_never_fires() {
    let mut fd = build_fd(vec![]);
    let (op, _src) = build_lane_read(&mut fd, SHUF_ID, 16, true, 5, 1);
    let mut rule = RuleSimdShuffleLane::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn an_unsupported_vector_width_is_declined() {
    // 32-byte AVX2 vpshufb is two independent 16-byte halves, not one N=32
    // permutation, so the width is not matched.
    let mut fd = build_fd(vec![SHUF_ID as u32]);
    let (op, _src) = build_lane_read(&mut fd, SHUF_ID, 32, true, 5, 1);
    let mut rule = RuleSimdShuffleLane::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn the_rule_registers_in_the_analysis_group() {
    let specs = specs();
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].group, "analysis");
}

#[test]
fn the_option_parses_on_and_off() {
    assert!(OptionSimdLane.apply("on").unwrap().0);
    assert!(!OptionSimdLane.apply("off").unwrap().0);
    assert_eq!(OptionSimdLane::NAME, "simdlane");
}
