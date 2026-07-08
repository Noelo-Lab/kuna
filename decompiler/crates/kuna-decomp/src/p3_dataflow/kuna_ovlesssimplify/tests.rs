//! Tests for `RuleOvLessSimplify` (kuna GH-7190): collapse
//! `NE(SLESS(V+K,0), AND(sign, SLESS(-1,V+K)))` -> `INT_SLESS(V, -K)`, plus the
//! gate-off / non-matching paths.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, TypeOp, VarnodeId};
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
    let glb = Rc::new(ArchContext::new(manage));
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

/// Wire a (shared) constant input without recording a descend edge (so the rule
/// can relink it; see kuna_booleanmask tests for the rationale — the C++
/// constant-dup branch of opSetInput is a deferred funcdata_op seam).
fn wire_const(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// Make a written varnode at a ram address (output of a fresh COPY), non-free so
/// it may carry several descendants (the relink target V).
fn mk_written(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    set_def(fd, vn, defop)
}

/// Build a `V + K` (INT_ADD) cone with V = `vbase`, K = `kval`, size `sz`,
/// returning the sum varnode (a written unique).
fn build_add(fd: &mut Funcdata, off: u64, vbase: VarnodeId, kval: u64, sz: int4) -> VarnodeId {
    let addop = mk_op(fd, off, 2, OpCode::CPUI_INT_ADD);
    let kconst = fd.new_constant(sz, kval);
    wire(fd, vbase, addop, 0);
    wire(fd, kconst, addop, 1);
    let sum = fd.new_unique(sz, Some(unk(sz)));
    set_def(fd, sum, addop)
}

/// Build a written boolean varnode = (the output of) `opc(in0,in1)`.
fn build_bool(fd: &mut Funcdata, off: u64, opc: OpCode, in0: VarnodeId, in1: VarnodeId) -> VarnodeId {
    let b = mk_op(fd, off, 2, opc);
    wire(fd, in0, b, 0);
    wire(fd, in1, b, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    set_def(fd, out, b)
}

/// Build the full idiom rooted at an INT_NOTEQUAL.  Returns (ne_op, vbase).
/// `sz` is the width of V and the V+K adds.
fn build_idiom(fd: &mut Funcdata, sz: int4, kval: u64) -> (OpId, VarnodeId) {
    let vbase = mk_written(fd, 0x40, sz);

    // R1 = V + K  ;  in0 = SLESS(R1, 0)
    let r1 = build_add(fd, 0x100, vbase, kval, sz);
    let zero = fd.new_constant(sz, 0);
    let in0 = build_bool(fd, 0x108, OpCode::CPUI_INT_SLESS, r1, zero);

    // R2 = V + K  ;  slessHi = SLESS(-1, R2)
    let r2 = build_add(fd, 0x110, vbase, kval, sz);
    let mone = fd.new_constant(sz, kuna_base::address::calc_mask(sz)); // -1
    let sless_hi = build_bool(fd, 0x118, OpCode::CPUI_INT_SLESS, mone, r2);

    // signTest: any written boolean (the V s< 0 sign), unused by the rewrite.
    let zero2 = fd.new_constant(sz, 0);
    let signtest = build_bool(fd, 0x120, OpCode::CPUI_INT_SLESS, vbase, zero2);

    // in1 = BOOL_AND(signTest, slessHi)
    let in1 = build_bool(fd, 0x128, OpCode::CPUI_BOOL_AND, signtest, sless_hi);

    // op = INT_NOTEQUAL(in0, in1)
    let op = mk_op(fd, 0x130, 2, OpCode::CPUI_INT_NOTEQUAL);
    wire(fd, in0, op, 0);
    wire(fd, in1, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(fd, out, op);

    (op, vbase)
}

#[test]
fn collapses_ovless_idiom_to_sless() {
    let mut fd = build_fd();
    let (op, vbase) = build_idiom(&mut fd, 4, 8); // K = 8

    let mut rule = RuleOvLessSimplify::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    // op is now INT_SLESS(V, -8).
    let o = fd.obank().get(op).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_SLESS);
    assert_eq!(o.get_in(0), Some(vbase));
    let neg = o.get_in(1).unwrap();
    let nv = fd.vbank().get(neg).unwrap();
    assert!(nv.is_constant());
    // -8 within 4 bytes == 0xFFFFFFF8
    assert_eq!(nv.get_offset(), 0xFFFFFFF8);
}

#[test]
fn gate_off_is_noop() {
    let mut fd = build_fd();
    let (op, _v) = build_idiom(&mut fd, 4, 8);
    let mut rule = RuleOvLessSimplify::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_NOTEQUAL);
}

#[test]
fn non_matching_in1_declines() {
    // in1 is a plain boolean, not BOOL_AND(sign, SLESS(-1,V+K)).
    let mut fd = build_fd();
    let vbase = mk_written(&mut fd, 0x40, 4);
    let r1 = build_add(&mut fd, 0x100, vbase, 8, 4);
    let zero = fd.new_constant(4, 0);
    let in0 = build_bool(&mut fd, 0x108, OpCode::CPUI_INT_SLESS, r1, zero);
    let zero2 = fd.new_constant(4, 0);
    let in1 = build_bool(&mut fd, 0x120, OpCode::CPUI_INT_SLESS, vbase, zero2); // not AND
    let op = mk_op(&mut fd, 0x130, 2, OpCode::CPUI_INT_NOTEQUAL);
    wire(&mut fd, in0, op, 0);
    wire(&mut fd, in1, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    let mut rule = RuleOvLessSimplify::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    let _ = wire_const; // silence unused in this test path
}

#[test]
fn mismatched_constants_decline() {
    // R1 uses K=8 but R2 uses K=9 -> k1 != k2 -> decline.
    let mut fd = build_fd();
    let vbase = mk_written(&mut fd, 0x40, 4);
    let r1 = build_add(&mut fd, 0x100, vbase, 8, 4);
    let zero = fd.new_constant(4, 0);
    let in0 = build_bool(&mut fd, 0x108, OpCode::CPUI_INT_SLESS, r1, zero);
    let r2 = build_add(&mut fd, 0x110, vbase, 9, 4);
    let mone = fd.new_constant(4, kuna_base::address::calc_mask(4));
    let sless_hi = build_bool(&mut fd, 0x118, OpCode::CPUI_INT_SLESS, mone, r2);
    let zero2 = fd.new_constant(4, 0);
    let signtest = build_bool(&mut fd, 0x120, OpCode::CPUI_INT_SLESS, vbase, zero2);
    let in1 = build_bool(&mut fd, 0x128, OpCode::CPUI_BOOL_AND, signtest, sless_hi);
    let op = mk_op(&mut fd, 0x130, 2, OpCode::CPUI_INT_NOTEQUAL);
    wire(&mut fd, in0, op, 0);
    wire(&mut fd, in1, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);

    let mut rule = RuleOvLessSimplify::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn specs_one_rule_in_analysis() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    let rule = (s[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_INT_NOTEQUAL]);
}
