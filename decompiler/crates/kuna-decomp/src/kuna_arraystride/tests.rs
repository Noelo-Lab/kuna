//! Tests for `RuleArrayStride` (kuna GH-8724): rewrite a stride accumulator phi
//! `MULTIEQUAL(#0, acc+STRIDE)` as `INT_MULT(cnt, STRIDE)` from its sibling
//! unit-step counter phi, plus the gate-off / decline paths.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::seams::{Architecture, BlockId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

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

fn no_replace(
) -> impl FnMut(&mut crate::varnode::VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()>
{
    |_, _, _| Ok(())
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

/// Op on the dead list (not yet inserted), so `op_insert` can move it in.
fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
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

/// Build the two-sibling-phi loop header.  Returns (acc_phi, acc_out, cnt_out,
/// external_reader_op, bl).
fn build_two_phis(fd: &mut Funcdata, sz: int4, stride: u64) -> (OpId, VarnodeId, VarnodeId, OpId, BlockId) {
    let bl = mk_block(fd);

    // --- accumulator phi: acc = MULTIEQUAL(#0, acc + stride) ---
    let acc_phi = mk_op(fd, 0x100, 2, OpCode::CPUI_MULTIEQUAL);
    let acc_out = fd.new_unique(sz, None);
    let acc_out = set_def(fd, acc_out, acc_phi);
    // acc_add = INT_ADD(acc_out, #stride)
    let acc_add = mk_op(fd, 0x108, 2, OpCode::CPUI_INT_ADD);
    let stride_c = fd.new_constant(sz, stride);
    wire(fd, acc_out, acc_add, 0); // acc_out's lone descend = acc_add
    wire(fd, stride_c, acc_add, 1);
    let acc_add_out = fd.new_unique(sz, None);
    let acc_add_out = set_def(fd, acc_add_out, acc_add);
    // phi inputs: #0 at slot 0, acc_add_out at slot 1
    let zero = fd.new_constant(sz, 0);
    wire(fd, zero, acc_phi, 0);
    wire(fd, acc_add_out, acc_phi, 1);
    fd.op_insert(acc_phi, bl, None);

    // --- counter phi: cnt = MULTIEQUAL(#0, cnt + 1) ---
    let cnt_phi = mk_op(fd, 0x110, 2, OpCode::CPUI_MULTIEQUAL);
    let cnt_out = fd.new_unique(sz, None);
    let cnt_out = set_def(fd, cnt_out, cnt_phi);
    let cnt_add = mk_op(fd, 0x118, 2, OpCode::CPUI_INT_ADD);
    let one_c = fd.new_constant(sz, 1);
    wire(fd, cnt_out, cnt_add, 0);
    wire(fd, one_c, cnt_add, 1);
    let cnt_add_out = fd.new_unique(sz, None);
    let cnt_add_out = set_def(fd, cnt_add_out, cnt_add);
    let zero2 = fd.new_constant(sz, 0);
    wire(fd, zero2, cnt_phi, 0);
    wire(fd, cnt_add_out, cnt_phi, 1);
    // Counter phi inserted after the accumulator (still first in the block).
    fd.op_insert(cnt_phi, bl, None);

    // --- an external use of acc (so totalReplace has a reader) ---
    let reader = mk_op(fd, 0x130, 1, OpCode::CPUI_COPY);
    wire(fd, acc_out, reader, 0);
    let rout = fd.new_unique(sz, None);
    let _ = set_def(fd, rout, reader);
    // (reader stays on the dead list; the rule reads/repoints it by id.)

    (acc_phi, acc_out, cnt_out, reader, bl)
}

#[test]
fn rewrites_accumulator_as_counter_times_stride() {
    let mut fd = build_fd();
    let (acc_phi, acc_out, cnt_out, reader, _bl) = build_two_phis(&mut fd, 4, 0x10);

    let mut rule = RuleArrayStride::new(true);
    assert_eq!(rule.apply_op(acc_phi, &mut fd), 1);

    // The external reader now reads `INT_MULT(cnt, 0x10)`.
    let multout = fd.obank().get(reader).unwrap().get_in(0).unwrap();
    let multop = fd.vbank().get(multout).unwrap().get_def().unwrap();
    let mo = fd.obank().get(multop).unwrap();
    assert_eq!(mo.code(), OpCode::CPUI_INT_MULT);
    assert_eq!(mo.get_in(0), Some(cnt_out));
    let strv = mo.get_in(1).unwrap();
    assert!(fd.vbank().get(strv).unwrap().is_constant());
    assert_eq!(fd.vbank().get(strv).unwrap().get_offset(), 0x10);

    // The accumulator phi was destroyed (opDestroy unhooks + marks dead; the op
    // object survives on the dead list until clearDeadOps, exactly as in C++).
    assert!(fd.obank().get(acc_phi).unwrap().is_dead());
    assert!(fd.obank().get(acc_phi).unwrap().get_out().is_none());
    // acc_out has no readers left (all redirected to the multiply output).
    assert_eq!(fd.vbank().get(acc_out).map(|v| v.num_descend()).unwrap_or(0), 0);
}

#[test]
fn gate_off_is_noop() {
    let mut fd = build_fd();
    let (acc_phi, _acc_out, _cnt_out, reader, _bl) = build_two_phis(&mut fd, 4, 0x10);
    let mut rule = RuleArrayStride::new(false);
    assert_eq!(rule.apply_op(acc_phi, &mut fd), 0);
    // phi still alive, reader still reads the accumulator output.
    assert!(fd.obank().get(acc_phi).is_some());
    assert_eq!(fd.obank().get(acc_phi).unwrap().code(), OpCode::CPUI_MULTIEQUAL);
    assert!(fd.obank().get(reader).unwrap().get_in(0).is_some());
}

#[test]
fn unit_stride_declines() {
    // An accumulator that itself steps by 1 is a counter, not a stride -> decline.
    let mut fd = build_fd();
    let (acc_phi, _a, _c, _r, _bl) = build_two_phis(&mut fd, 4, 1);
    let mut rule = RuleArrayStride::new(true);
    assert_eq!(rule.apply_op(acc_phi, &mut fd), 0);
    assert!(fd.obank().get(acc_phi).is_some());
}

#[test]
fn no_sibling_counter_declines() {
    // A lone stride accumulator with no unit-step sibling -> decline.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let acc_phi = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_MULTIEQUAL);
    let acc_out = fd.new_unique(4, None);
    let acc_out = set_def(&mut fd, acc_out, acc_phi);
    let acc_add = mk_op(&mut fd, 0x108, 2, OpCode::CPUI_INT_ADD);
    let stride_c = fd.new_constant(4, 0x10);
    wire(&mut fd, acc_out, acc_add, 0);
    wire(&mut fd, stride_c, acc_add, 1);
    let acc_add_out = fd.new_unique(4, None);
    let acc_add_out = set_def(&mut fd, acc_add_out, acc_add);
    let zero = fd.new_constant(4, 0);
    wire(&mut fd, zero, acc_phi, 0);
    wire(&mut fd, acc_add_out, acc_phi, 1);
    fd.op_insert(acc_phi, bl, None);

    let mut rule = RuleArrayStride::new(true);
    assert_eq!(rule.apply_op(acc_phi, &mut fd), 0);
    assert!(fd.obank().get(acc_phi).is_some());
}

#[test]
fn specs_one_rule_in_analysis() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    let rule = (s[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_MULTIEQUAL]);
}
