//! Tests for the two flag-compare rules (kuna GH-1276 / GH-8777):
//! `RuleBoolSignLess` (`(b<<k) s< 0` -> `b != 0`) and `RuleSborrowGe`
//! (`XNOR(sign(V-K), SBORROW(V,K))` -> `INT_SLESSEQUAL(K,V)`), plus the gate-off
//! and decline paths.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::dtype::{type_metatype, Datatype};
use crate::seams::{Architecture, TypeOp, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

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

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(Architecture::new(manage));
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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()>
{
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

/// Wire a shared constant without a descend edge (see kuna_booleanmask tests).
fn wire_const(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn mk_written(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    set_def(fd, vn, defop)
}

fn build_bool(fd: &mut Funcdata, off: u64, opc: OpCode, in0: VarnodeId, in1: VarnodeId) -> VarnodeId {
    let b = mk_op(fd, off, 2, opc);
    wire(fd, in0, b, 0);
    wire(fd, in1, b, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    set_def(fd, out, b)
}

fn bool_neg(fd: &mut Funcdata, off: u64, src: VarnodeId) -> VarnodeId {
    let n = mk_op(fd, off, 1, OpCode::CPUI_BOOL_NEGATE);
    wire(fd, src, n, 0);
    let out = fd.new_unique(1, Some(unk(1)));
    set_def(fd, out, n)
}

// --- GH-1276 RuleBoolSignLess -----------------------------------------------

#[test]
fn boolsignless_folds_to_noteq() {
    let mut fd = build_fd();
    // boolvn = const 1, size 1 (NZMask 1).  leftshift = INT_LEFT(boolvn, 7).
    let boolvn = fd.new_constant(1, 1);
    let leftshift = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_LEFT);
    let sa = fd.new_constant(1, 7);
    wire_const(&mut fd, boolvn, leftshift, 0); // shared const, no descend
    wire(&mut fd, sa, leftshift, 1);
    let shiftout = fd.new_unique(1, Some(unk(1)));
    let shiftout = set_def(&mut fd, shiftout, leftshift);

    // op = INT_SLESS(shiftout, 0)
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_SLESS);
    let zero = fd.new_constant(1, 0);
    wire(&mut fd, shiftout, op, 0);
    wire(&mut fd, zero, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    let mut rule = RuleBoolSignLess::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    let o = fd.obank().get(op).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_NOTEQUAL);
    assert_eq!(o.get_in(0), Some(boolvn));
    let c = o.get_in(1).unwrap();
    assert!(fd.vbank().get(c).unwrap().is_constant());
    assert_eq!(fd.vbank().get(c).unwrap().get_offset(), 0);
}

#[test]
fn boolsignless_gate_off_is_noop() {
    let mut fd = build_fd();
    let boolvn = fd.new_constant(1, 1);
    let leftshift = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_LEFT);
    let sa = fd.new_constant(1, 7);
    wire_const(&mut fd, boolvn, leftshift, 0);
    wire(&mut fd, sa, leftshift, 1);
    let shiftout = fd.new_unique(1, Some(unk(1)));
    let shiftout = set_def(&mut fd, shiftout, leftshift);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_SLESS);
    let zero = fd.new_constant(1, 0);
    wire(&mut fd, shiftout, op, 0);
    wire(&mut fd, zero, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    let mut rule = RuleBoolSignLess::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_SLESS);
}

#[test]
fn boolsignless_extra_nzmask_bit_declines() {
    // boolvn = const 3 (NZMask bits 0 and 1) -> not exactly the single sign bit.
    let mut fd = build_fd();
    let boolvn = fd.new_constant(1, 3);
    let leftshift = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_LEFT);
    let sa = fd.new_constant(1, 7);
    wire_const(&mut fd, boolvn, leftshift, 0);
    wire(&mut fd, sa, leftshift, 1);
    let shiftout = fd.new_unique(1, Some(unk(1)));
    let shiftout = set_def(&mut fd, shiftout, leftshift);
    let op = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_SLESS);
    let zero = fd.new_constant(1, 0);
    wire(&mut fd, shiftout, op, 0);
    wire(&mut fd, zero, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    let mut rule = RuleBoolSignLess::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

// --- GH-8777 RuleSborrowGe (AND-of-ORs form) --------------------------------

/// Build `BOOL_AND(BOOL_OR(N,!V), BOOL_OR(!N,V))` over the flag pair (n, v),
/// returning the AND op.  `n` is the sign-of-(V-K) flag, `v` the SBORROW.
fn build_and_of_ors(fd: &mut Funcdata, n: VarnodeId, v: VarnodeId) -> OpId {
    let not_v = bool_neg(fd, 0x200, v);
    let not_n = bool_neg(fd, 0x204, n);
    // OR(N, !V)
    let or_a = mk_op(fd, 0x208, 2, OpCode::CPUI_BOOL_OR);
    wire(fd, n, or_a, 0);
    wire(fd, not_v, or_a, 1);
    let oa = fd.new_unique(1, Some(unk(1)));
    let oa = set_def(fd, oa, or_a);
    // OR(!N, V)
    let or_b = mk_op(fd, 0x20c, 2, OpCode::CPUI_BOOL_OR);
    wire(fd, not_n, or_b, 0);
    wire(fd, v, or_b, 1);
    let ob = fd.new_unique(1, Some(unk(1)));
    let ob = set_def(fd, ob, or_b);
    // AND(oa, ob)
    let andop = mk_op(fd, 0x210, 2, OpCode::CPUI_BOOL_AND);
    wire(fd, oa, andop, 0);
    wire(fd, ob, andop, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(fd, out, andop);
    andop
}

/// Build the {N (sign of V-K), V (SBORROW(V,K))} flag pair, sharing `vbase`.
/// Returns (n, v).
fn build_flag_pair(fd: &mut Funcdata, vbase: VarnodeId, kval: u64, sz: int4) -> (VarnodeId, VarnodeId) {
    // sub = V + (-K)   (INT_ADD with the two's-complement of K)
    let subop = mk_op(fd, 0x180, 2, OpCode::CPUI_INT_ADD);
    let negk = (kval.wrapping_neg()) & calc_mask(sz);
    let negk_c = fd.new_constant(sz, negk);
    wire(fd, vbase, subop, 0);
    wire(fd, negk_c, subop, 1);
    let sub = fd.new_unique(sz, Some(unk(sz)));
    let sub = set_def(fd, sub, subop);

    // N = INT_SLESSEQUAL(0, V-K)
    let zero = fd.new_constant(sz, 0);
    let n = build_bool(fd, 0x188, OpCode::CPUI_INT_SLESSEQUAL, zero, sub);

    // V = INT_SBORROW(V, K)
    let sbop = mk_op(fd, 0x190, 2, OpCode::CPUI_INT_SBORROW);
    let kc = fd.new_constant(sz, kval);
    wire(fd, vbase, sbop, 0);
    wire(fd, kc, sbop, 1);
    let v = fd.new_unique(1, Some(unk(1)));
    let v = set_def(fd, v, sbop);
    (n, v)
}

#[test]
fn sborrowge_and_of_ors_folds_to_slessequal() {
    let mut fd = build_fd();
    let vbase = mk_written(&mut fd, 0x40, 4);
    let (n, v) = build_flag_pair(&mut fd, vbase, 7, 4);
    let andop = build_and_of_ors(&mut fd, n, v);

    let mut rule = RuleSborrowGe::new(true);
    assert_eq!(rule.apply_op(andop, &mut fd), 1);

    let o = fd.obank().get(andop).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_SLESSEQUAL);
    // in0 = const K (= 7), in1 = V
    let k = o.get_in(0).unwrap();
    assert!(fd.vbank().get(k).unwrap().is_constant());
    assert_eq!(fd.vbank().get(k).unwrap().get_offset(), 7);
    assert_eq!(o.get_in(1), Some(vbase));
}

#[test]
fn sborrowge_gate_off_is_noop() {
    let mut fd = build_fd();
    let vbase = mk_written(&mut fd, 0x40, 4);
    let (n, v) = build_flag_pair(&mut fd, vbase, 7, 4);
    let andop = build_and_of_ors(&mut fd, n, v);

    let mut rule = RuleSborrowGe::new(false);
    assert_eq!(rule.apply_op(andop, &mut fd), 0);
    assert_eq!(fd.obank().get(andop).unwrap().code(), OpCode::CPUI_BOOL_AND);
}

#[test]
fn sborrowge_non_xnor_declines() {
    // A plain BOOL_AND of two booleans, not the XNOR shape.
    let mut fd = build_fd();
    let a = mk_written(&mut fd, 0x40, 1);
    let b = mk_written(&mut fd, 0x44, 1);
    let andop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_BOOL_AND);
    wire(&mut fd, a, andop, 0);
    wire(&mut fd, b, andop, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, andop);

    let mut rule = RuleSborrowGe::new(true);
    assert_eq!(rule.apply_op(andop, &mut fd), 0);
}

#[test]
fn specs_two_rules_in_definition_order() {
    let s = specs();
    assert_eq!(s.len(), 2);
    // C++ definition order: RuleBoolSignLess (INT_SLESS) then RuleSborrowGe
    // (BOOL_AND/BOOL_OR).
    let r0 = (s[0].ctor)();
    assert_eq!(r0.get_op_list(), vec![OpCode::CPUI_INT_SLESS]);
    let r1 = (s[1].ctor)();
    assert_eq!(r1.get_op_list(), vec![OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_OR]);
    assert_eq!(s[0].group, "analysis");
    assert_eq!(s[1].group, "analysis");
}
