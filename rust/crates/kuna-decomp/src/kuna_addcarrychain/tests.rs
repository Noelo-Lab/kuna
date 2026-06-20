//! Tests for `RuleAddCarryChain` (kuna GH-8913): the carry-chain wide-add fusion
//! `PIECE(hi, lo) -> INT_ADD(PIECE(hipart,b), zext(a))` and its gate-off / decline
//! paths.

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

fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    // Create on the dead list (NOT marked alive): the PIECE op is moved to the
    // block by `op_insert` (which marks it alive); the other ops stay dead, which
    // is fine — the rule reads them by id.  Pre-marking-alive here would make
    // `op_insert`'s `mark_alive` double-unlink the op from the dead list.
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

/// A written byte at a ram address (output of a fresh COPY).
fn mk_byte(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let r = ram(fd);
    let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
    let vn = fd.new_varnode(1, &Address::new(r, off), None);
    set_def(fd, vn, defop)
}

/// Build the carry-chain idiom rooted at a 2-byte PIECE, with the INT_CARRY
/// carry form, placing the PIECE op in a basic block.  Returns (piece_op, a, b,
/// hipart).
fn build_idiom(fd: &mut Funcdata) -> (OpId, VarnodeId, VarnodeId, VarnodeId) {
    let bl = mk_block(fd);

    let a = mk_byte(fd, 0x40); // index (variable)
    let b = mk_byte(fd, 0x44); // base low byte
    let hipart = mk_byte(fd, 0x48); // base high byte

    // lo = INT_ADD(a, b)  -> lovn (byte)
    let loadd = mk_op(fd, 0x100, 2, OpCode::CPUI_INT_ADD);
    wire(fd, a, loadd, 0);
    wire(fd, b, loadd, 1);
    let lovn = fd.new_unique(1, None);
    let lovn = set_def(fd, lovn, loadd);

    // carry = INT_CARRY(a, b)  -> carryvn (byte)
    let carryop = mk_op(fd, 0x108, 2, OpCode::CPUI_INT_CARRY);
    wire(fd, a, carryop, 0);
    wire(fd, b, carryop, 1);
    let carryvn = fd.new_unique(1, None);
    let carryvn = set_def(fd, carryvn, carryop);

    // hi = INT_ADD(hipart, carry)  -> hivn (byte)
    let hiadd = mk_op(fd, 0x110, 2, OpCode::CPUI_INT_ADD);
    wire(fd, hipart, hiadd, 0);
    wire(fd, carryvn, hiadd, 1);
    let hivn = fd.new_unique(1, None);
    let hivn = set_def(fd, hivn, hiadd);

    // op = PIECE(hivn, lovn)  -> out (2 bytes), inserted in the block.
    let op = mk_op(fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(fd, hivn, op, 0);
    wire(fd, lovn, op, 1);
    let out = fd.new_unique(2, None);
    let _out = set_def(fd, out, op);
    fd.op_insert(op, bl, None);

    (op, a, b, hipart)
}

#[test]
fn fuses_carry_chain_to_wide_add() {
    let mut fd = build_fd();
    let (op, a, _b, _hipart) = build_idiom(&mut fd);

    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    // op is now INT_ADD(base, zext(index)).
    let o = fd.obank().get(op).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_ADD);
    let base_out = o.get_in(0).unwrap();
    let zext_out = o.get_in(1).unwrap();

    // in0 = PIECE(hipart, b)
    let baseop = fd.vbank().get(base_out).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(baseop).unwrap().code(), OpCode::CPUI_PIECE);

    // in1 = INT_ZEXT(a)
    let zextop = fd.vbank().get(zext_out).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(zextop).unwrap().code(), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(zextop).unwrap().get_in(0), Some(a));
    // zext output width matches the 2-byte PIECE output.
    assert_eq!(fd.vbank().get(zext_out).unwrap().get_size(), 2);
}

#[test]
fn gate_off_is_noop() {
    let mut fd = build_fd();
    let (op, _a, _b, _hi) = build_idiom(&mut fd);
    let mut rule = RuleAddCarryChain::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_PIECE);
}

#[test]
fn non_carry_high_input_declines() {
    // hi = INT_ADD(hipart, <not a carry of a,b>) -> rule declines.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_byte(&mut fd, 0x40);
    let b = mk_byte(&mut fd, 0x44);
    let hipart = mk_byte(&mut fd, 0x48);
    let other = mk_byte(&mut fd, 0x4c); // not a carry
    let loadd = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_ADD);
    wire(&mut fd, a, loadd, 0);
    wire(&mut fd, b, loadd, 1);
    let lovn = fd.new_unique(1, None);
    let lovn = set_def(&mut fd, lovn, loadd);
    let hiadd = mk_op(&mut fd, 0x110, 2, OpCode::CPUI_INT_ADD);
    wire(&mut fd, hipart, hiadd, 0);
    wire(&mut fd, other, hiadd, 1);
    let hivn = fd.new_unique(1, None);
    let hivn = set_def(&mut fd, hivn, hiadd);
    let op = mk_op(&mut fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(&mut fd, hivn, op, 0);
    wire(&mut fd, lovn, op, 1);
    let out = fd.new_unique(2, None);
    let _ = set_def(&mut fd, out, op);
    fd.op_insert(op, bl, None);

    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_PIECE);
}

#[test]
fn wide_halves_decline() {
    // 2-byte halves (not 1-byte) -> first-cut guard declines.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let r = ram(&fd);
    // Build hi/lo as 2-byte written varnodes.
    let mk_word = |fd: &mut Funcdata, off: u64| -> VarnodeId {
        let defop = mk_op(fd, off, 1, OpCode::CPUI_COPY);
        let vn = fd.new_varnode(2, &Address::new(Rc::clone(&r), off), None);
        set_def(fd, vn, defop)
    };
    let hivn = mk_word(&mut fd, 0x40);
    let lovn = mk_word(&mut fd, 0x44);
    let op = mk_op(&mut fd, 0x118, 2, OpCode::CPUI_PIECE);
    wire(&mut fd, hivn, op, 0);
    wire(&mut fd, lovn, op, 1);
    let out = fd.new_unique(4, None);
    let _ = set_def(&mut fd, out, op);
    fd.op_insert(op, bl, None);

    let mut rule = RuleAddCarryChain::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn specs_one_rule_in_analysis() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    let rule = (s[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_PIECE]);
}
