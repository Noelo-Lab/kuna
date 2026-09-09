//! Unit tests for `zeroidiomuse`'s predicate: which descendant ops throw their
//! operand away, and how far the structural sameness test reaches.  The
//! end-to-end behaviour is `tests/stages/gh dec-zeroidiomuse.xml` and
//! `tests/cli/argument-recovery-drops-both.json`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;

use crate::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::varnode::{DefOpInfo, VarnodeBank};

use super::*;

// --- fixtures ---------------------------------------------------------------

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

fn build_fd(zero_idiom_use: bool) -> Funcdata {
    let mut ctx = ArchContext::new(build_manager());
    ctx.zero_idiom_use = zero_idiom_use;
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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, bl: BlockId, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert(op, bl, None);
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

fn set_output(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

/// Wire `vn` as input `slot` of `op`.  Only the op side: the predicate reads
/// `get_in`/`get_def` and never a descend list, and a free Varnode refuses a
/// second descendant.
fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// `out = opc(a, b)` at `off`.
fn binop(fd: &mut Funcdata, bl: BlockId, off: u64, opc: OpCode, a: VarnodeId, b: VarnodeId) -> OpId {
    let op = mk_op(fd, bl, 2, off, opc);
    wire(fd, a, op, 0);
    wire(fd, b, op, 1);
    op
}

/// `out = SUBPIECE(src, 0)` at `off`, 4 bytes wide — the register-slice shape
/// heritage leaves behind for a 32-bit read of a 64-bit register.
fn slice(fd: &mut Funcdata, bl: BlockId, off: u64, src: VarnodeId, outoff: u64) -> VarnodeId {
    let op = mk_op(fd, bl, 2, off, OpCode::CPUI_SUBPIECE);
    let zero = fd.new_constant(4, 0);
    wire(fd, src, op, 0);
    wire(fd, zero, op, 1);
    set_output(fd, op, outoff, 4)
}

// --- the predicate ----------------------------------------------------------

#[test]
fn self_xor_of_one_varnode_discards_its_operand() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 0x40, 8);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, a, a);
    assert!(op_discards_operand(&fd, op));
}

#[test]
fn self_sub_of_one_varnode_discards_its_operand() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 0x40, 8);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_SUB, a, a);
    assert!(op_discards_operand(&fd, op));
}

#[test]
fn xor_of_two_different_varnodes_is_a_real_use() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 0x40, 8);
    let b = mk_vn(&mut fd, 0x48, 8);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, a, b);
    assert!(!op_discards_operand(&fd, op));
}

#[test]
fn self_and_and_self_or_are_real_uses() {
    // `x & x` and `x | x` are `x`, not 0 — the value survives them, so the walk
    // must keep following it.
    for opc in [OpCode::CPUI_INT_AND, OpCode::CPUI_INT_OR] {
        let mut fd = build_fd(true);
        let bl = mk_block(&mut fd);
        let a = mk_vn(&mut fd, 0x40, 8);
        let op = binop(&mut fd, bl, 0x2000, opc, a, a);
        assert!(!op_discards_operand(&fd, op), "{opc:?} must stay a real use");
    }
}

#[test]
fn option_off_declines_every_op() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let a = mk_vn(&mut fd, 0x40, 8);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, a, a);
    assert!(!op_discards_operand(&fd, op));
}

// --- structural sameness ----------------------------------------------------

#[test]
fn two_subpieces_of_one_source_are_the_same_value() {
    // The witness shape: `xor esi,esi` after heritage is
    // `INT_XOR(SUBPIECE(rsi,0), SUBPIECE(rsi,0))` over two DISTINCT SUBPIECE
    // Varnodes, because the operands are not common-subexpression eliminated
    // until well after ActionActiveParam.
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let src = mk_vn(&mut fd, 0x40, 8);
    let lo1 = slice(&mut fd, bl, 0x2000, src, 0x100);
    let lo2 = slice(&mut fd, bl, 0x2000, src, 0x108);
    assert_ne!(lo1, lo2);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, lo1, lo2);
    assert!(op_discards_operand(&fd, op));
}

#[test]
fn subpieces_of_different_sources_are_not_the_same_value() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let src1 = mk_vn(&mut fd, 0x40, 8);
    let src2 = mk_vn(&mut fd, 0x48, 8);
    let lo1 = slice(&mut fd, bl, 0x2000, src1, 0x100);
    let lo2 = slice(&mut fd, bl, 0x2000, src2, 0x108);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, lo1, lo2);
    assert!(!op_discards_operand(&fd, op));
}

#[test]
fn equal_constants_are_the_same_value_and_unequal_ones_are_not() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let a = fd.new_constant(4, 7);
    let b = fd.new_constant(4, 7);
    let c = fd.new_constant(4, 9);
    let same = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, a, b);
    let diff = binop(&mut fd, bl, 0x2004, OpCode::CPUI_INT_XOR, a, c);
    assert!(op_discards_operand(&fd, same));
    assert!(!op_discards_operand(&fd, diff));
}

#[test]
fn a_constant_and_a_register_are_never_the_same_value() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let a = fd.new_constant(8, 0);
    let b = mk_vn(&mut fd, 0x40, 8);
    let op = binop(&mut fd, bl, 0x2000, OpCode::CPUI_INT_XOR, a, b);
    assert!(!op_discards_operand(&fd, op));
}

#[test]
fn sameness_does_not_look_through_an_impure_op() {
    // Two LOADs of one address are two reads of memory, not one value.
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let ptr = mk_vn(&mut fd, 0x40, 8);
    let mut load = |off: u64, outoff: u64| {
        let op = mk_op(&mut fd, bl, 2, off, OpCode::CPUI_LOAD);
        let sp = fd.new_constant(8, 4);
        wire(&mut fd, sp, op, 0);
        wire(&mut fd, ptr, op, 1);
        set_output(&mut fd, op, outoff, 4)
    };
    let v1 = load(0x2000, 0x100);
    let v2 = load(0x2004, 0x108);
    let op = binop(&mut fd, bl, 0x2008, OpCode::CPUI_INT_XOR, v1, v2);
    assert!(!op_discards_operand(&fd, op));
}

#[test]
fn sameness_stops_at_the_depth_bound() {
    // Three levels of reshaping is one more than `MAX_DEPTH` looks through, so
    // the predicate declines rather than recursing without a bound.
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    let src = mk_vn(&mut fd, 0x40, 8);
    let mut chain = |off: u64, base: u64| {
        let a = slice(&mut fd, bl, off, src, base);
        let op1 = mk_op(&mut fd, bl, 1, off, OpCode::CPUI_COPY);
        wire(&mut fd, a, op1, 0);
        let b = set_output(&mut fd, op1, base + 0x10, 4);
        let op2 = mk_op(&mut fd, bl, 1, off, OpCode::CPUI_COPY);
        wire(&mut fd, b, op2, 0);
        set_output(&mut fd, op2, base + 0x20, 4)
    };
    let c1 = chain(0x2000, 0x100);
    let c2 = chain(0x2004, 0x200);
    let op = binop(&mut fd, bl, 0x2008, OpCode::CPUI_INT_XOR, c1, c2);
    assert!(!op_discards_operand(&fd, op));
}
