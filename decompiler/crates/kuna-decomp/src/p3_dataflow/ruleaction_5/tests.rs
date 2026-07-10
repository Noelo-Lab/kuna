// Tests for ruleaction_5 (RuleBoolNegate .. RulePushPtr).
//
// Each ported rule is exercised by hand-building the exact input op pattern on a
// real `Funcdata` and asserting the precise transformation per the C++ body
// (ops created/destroyed, opcode changes, constants), with negative tests for
// the early-out guards.  The three STUB-stubbed pointer rules
// (RulePtrArith/RuleStructOffset0/RulePushPtr) are verified only for their
// structural surface (name/getOpList/group filtering); their `applyOp` is a
// no-op stub by design (see the module-level STUB notes).

use super::*;

use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, VarnodeId};
use crate::varnode::DefOpInfo;

// --- harness -----------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
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

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// Minimal flag bits for input ops in the test fixtures (so def ops carry the
/// right `booloutput` bit when a rule reads `isBoolOutput`).  Transcribed from
/// the same `typeop.cc` values [`type_op_lookup`] uses.
fn opcode_flags(opc: OpCode) -> uint4 {
    match opc {
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESSEQUAL => pcodeop_flags::binary | pcodeop_flags::booloutput,
        OpCode::CPUI_BOOL_NEGATE => pcodeop_flags::unary | pcodeop_flags::booloutput,
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT => {
            pcodeop_flags::binary | pcodeop_flags::commutative
        }
        OpCode::CPUI_COPY => pcodeop_flags::unary | pcodeop_flags::nocollapse,
        OpCode::CPUI_INT_NEGATE | OpCode::CPUI_INT_2COMP => pcodeop_flags::unary,
        OpCode::CPUI_SUBPIECE => pcodeop_flags::binary,
        OpCode::CPUI_PIECE => pcodeop_flags::binary,
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
            pcodeop_flags::binary
        }
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

/// A free register varnode at `off` of size `s`.
fn mk_vn(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(s, Address::new(ram, off), dt(s))
}

/// A formal-input register varnode at `off` of size `s`: non-free and
/// heritage-known (insert flag), the way an SSA-named operand is in the real
/// flow.  Used wherever a rule's guard rejects a *free* operand
/// (`avn->isFree()` / `isHeritageKnown`).
fn mk_input(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(s, Address::new(ram, off), dt(s));
    fd.vbank_mut().set_input(vn, &mut |_b, _a, _c| Ok(())).expect("set_input")
}

/// A constant varnode of value `val` size `s`.
fn mk_const(fd: &mut Funcdata, val: u64, s: int4) -> VarnodeId {
    fd.new_constant(s, val)
}

/// Make `vn` written by `op` (sets def: insert + written flags, like the real
/// `opSetOutput` would).  Uses the bank `set_def` directly because
/// `Funcdata::op_set_output` is an `Err` stub in the merged W3 IR.
fn set_def(fd: &mut Funcdata, vn: VarnodeId, op: OpId) {
    let seqnum = fd.obank().get(op).unwrap().get_seq_num().clone();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    let info = DefOpInfo { id: op, seqnum };
    fd.vbank_mut().set_def(vn, info, &mut |_b, _a, _c| Ok(())).expect("set_def");
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

fn off_of(fd: &Funcdata, vn: VarnodeId) -> u64 {
    fd.vbank().get(vn).unwrap().get_offset()
}

fn is_const(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).unwrap().is_constant()
}

// =============================================================================
// RuleBoolNegate
// =============================================================================

#[test]
fn boolnegate_flips_def_and_copies_negates() {
    let mut fd = build_fd();
    // flip_op: INT_EQUAL -> vn ; op: BOOL_NEGATE(vn) ; a second BOOL_NEGATE(vn).
    let flip = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let a = mk_vn(&mut fd, 0x10, 4);
    let b = mk_vn(&mut fd, 0x20, 4);
    fd.op_set_input(flip, a, 0).unwrap();
    fd.op_set_input(flip, b, 1).unwrap();
    let vn = mk_vn(&mut fd, 0x30, 1);
    set_def(&mut fd, vn, flip);

    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();
    let op2 = mk_op(&mut fd, 1, 0x120, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op2, vn, 0).unwrap();

    let mut rule = RuleBoolNegate::new("analysis");
    let res = rule.apply_op(op, &mut fd);
    assert_eq!(res, 1);
    // flip_op INT_EQUAL -> INT_NOTEQUAL (get_booleanflip, no reorder)
    assert_eq!(code_of(&fd, flip), OpCode::CPUI_INT_NOTEQUAL);
    // both negates become COPY
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(code_of(&fd, op2), OpCode::CPUI_COPY);
}

#[test]
fn boolnegate_negative_unwritten_input() {
    let mut fd = build_fd();
    // vn is a free (unwritten) varnode.
    let vn = mk_vn(&mut fd, 0x30, 1);
    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();
    let mut rule = RuleBoolNegate::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn boolnegate_negative_non_negate_descendant() {
    let mut fd = build_fd();
    let flip = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let a = mk_vn(&mut fd, 0x10, 4);
    let b = mk_vn(&mut fd, 0x20, 4);
    fd.op_set_input(flip, a, 0).unwrap();
    fd.op_set_input(flip, b, 1).unwrap();
    let vn = mk_vn(&mut fd, 0x30, 1);
    set_def(&mut fd, vn, flip);

    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();
    // A non-negate descendant of vn blocks the rule.
    let other = mk_op(&mut fd, 1, 0x120, OpCode::CPUI_COPY);
    fd.op_set_input(other, vn, 0).unwrap();

    let mut rule = RuleBoolNegate::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    // unchanged
    assert_eq!(code_of(&fd, flip), OpCode::CPUI_INT_EQUAL);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_BOOL_NEGATE);
}

#[test]
fn boolnegate_negative_unflippable_def() {
    let mut fd = build_fd();
    // INT_ADD has no boolean flip (get_booleanflip returns CPUI_MAX).
    let flip = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let a = mk_vn(&mut fd, 0x10, 4);
    let b = mk_vn(&mut fd, 0x20, 4);
    fd.op_set_input(flip, a, 0).unwrap();
    fd.op_set_input(flip, b, 1).unwrap();
    let vn = mk_vn(&mut fd, 0x30, 1);
    set_def(&mut fd, vn, flip);
    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();
    let mut rule = RuleBoolNegate::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_BOOL_NEGATE);
}

// =============================================================================
// RuleLess2Zero
// =============================================================================

#[test]
fn less2zero_zero_lhs_becomes_notequal() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    let zero = mk_const(&mut fd, 0, 4);
    let v = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, zero, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    let mut rule = RuleLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // 0 < V => 0 != V (opcode change only; inputs unchanged)
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_NOTEQUAL);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 2);
}

#[test]
fn less2zero_mask_lhs_becomes_false_copy() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    // lvn = ffffffff (calc_mask(4))
    let mask = mk_const(&mut fd, 0xffff_ffff, 4);
    let v = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, mask, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    let mut rule = RuleLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // ffff < V => false: COPY of const(1,0), 1 input
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
    let in0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    assert!(is_const(&fd, in0));
    assert_eq!(off_of(&fd, in0), 0);
}

#[test]
fn less2zero_zero_rhs_becomes_false_copy() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    let v = mk_vn(&mut fd, 0x10, 4);
    let zero = mk_const(&mut fd, 0, 4);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, zero, 1).unwrap();
    let mut rule = RuleLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // V < 0 => false
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
    assert_eq!(off_of(&fd, fd.obank().get(op).unwrap().get_in(0).unwrap()), 0);
}

#[test]
fn less2zero_mask_rhs_becomes_notequal() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    let v = mk_vn(&mut fd, 0x10, 4);
    let mask = mk_const(&mut fd, 0xffff_ffff, 4);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, mask, 1).unwrap();
    let mut rule = RuleLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // V < ffff => V != ffff
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_NOTEQUAL);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 2);
}

#[test]
fn less2zero_negative_nonextremal_constants() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    let c = mk_const(&mut fd, 5, 4);
    let v = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, c, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    let mut rule = RuleLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
}

// =============================================================================
// RuleLessEqual2Zero
// =============================================================================

#[test]
fn lessequal2zero_zero_lhs_becomes_true_copy() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let zero = mk_const(&mut fd, 0, 4);
    let v = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, zero, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    let mut rule = RuleLessEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // 0 <= V => true: COPY of const(1,1)
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
    assert_eq!(off_of(&fd, fd.obank().get(op).unwrap().get_in(0).unwrap()), 1);
}

#[test]
fn lessequal2zero_mask_lhs_becomes_equal() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let mask = mk_const(&mut fd, 0xffff_ffff, 4);
    let v = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, mask, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    let mut rule = RuleLessEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // ffff <= V => ffff == V
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 2);
}

#[test]
fn lessequal2zero_zero_rhs_becomes_equal() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let v = mk_vn(&mut fd, 0x10, 4);
    let zero = mk_const(&mut fd, 0, 4);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, zero, 1).unwrap();
    let mut rule = RuleLessEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // V <= 0 => V == 0
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 2);
}

#[test]
fn lessequal2zero_mask_rhs_becomes_true_copy() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let v = mk_vn(&mut fd, 0x10, 4);
    let mask = mk_const(&mut fd, 0xffff_ffff, 4);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, mask, 1).unwrap();
    let mut rule = RuleLessEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // V <= ffff => true
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
    assert_eq!(off_of(&fd, fd.obank().get(op).unwrap().get_in(0).unwrap()), 1);
}

#[test]
fn lessequal2zero_negative() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let v0 = mk_vn(&mut fd, 0x8, 4);
    let v1 = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    let mut rule = RuleLessEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESSEQUAL);
}

// =============================================================================
// RuleSLess2Zero
// =============================================================================

#[test]
fn sless2zero_negate_rhs_minus1() {
    // -1 s< ~avn  =>  avn s< 0   (feed = INT_NEGATE, lvn is -1 const)
    let mut fd = build_fd();
    let neg = mk_op(&mut fd, 1, 0x90, OpCode::CPUI_INT_NEGATE);
    let avn = mk_input(&mut fd, 0x10, 4);
    fd.op_set_input(neg, avn, 0).unwrap();
    let rvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, rvn, neg);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let m1 = mk_const(&mut fd, 0xffff_ffff, 4); // -1 for size 4
    fd.op_set_input(op, m1, 0).unwrap();
    fd.op_set_input(op, rvn, 1).unwrap();

    let mut rule = RuleSLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // op: avn s< 0   (input0=avn, input1=const 0); opcode stays INT_SLESS
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESS);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(avn));
    let i1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert!(is_const(&fd, i1));
    assert_eq!(off_of(&fd, i1), 0);
}

#[test]
fn sless2zero_negate_lhs_zero() {
    // ~avn s< 0  =>  -1 s< avn   (feed = INT_NEGATE, rvn is 0 const)
    let mut fd = build_fd();
    let neg = mk_op(&mut fd, 1, 0x90, OpCode::CPUI_INT_NEGATE);
    let avn = mk_input(&mut fd, 0x10, 4);
    fd.op_set_input(neg, avn, 0).unwrap();
    let lvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lvn, neg);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let zero = mk_const(&mut fd, 0, 4);
    fd.op_set_input(op, lvn, 0).unwrap();
    fd.op_set_input(op, zero, 1).unwrap();

    let mut rule = RuleSLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // op: input1 = avn, input0 = const calc_mask(4) = -1
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESS);
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(avn));
    let i0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    assert!(is_const(&fd, i0));
    assert_eq!(off_of(&fd, i0), 0xffff_ffff);
}

#[test]
fn sless2zero_subpiece_rhs_minus1() {
    // -1 s< SUB(avn,#hi)  =>  -1 s< avn   when rvn.size + hi == avn.size
    let mut fd = build_fd();
    // avn size 8; SUB output (rvn) size 4; #hi = 4.
    let sub = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_SUBPIECE);
    let avn = mk_input(&mut fd, 0x10, 8);
    let hi = mk_const(&mut fd, 4, 8); // truncation amount in bytes
    fd.op_set_input(sub, avn, 0).unwrap();
    fd.op_set_input(sub, hi, 1).unwrap();
    let rvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, rvn, sub);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let m1 = mk_const(&mut fd, 0xffff_ffff, 4);
    fd.op_set_input(op, m1, 0).unwrap();
    fd.op_set_input(op, rvn, 1).unwrap();

    let mut rule = RuleSLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // input1 = avn, input0 = const calc_mask(8)
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(avn));
    let i0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    assert_eq!(off_of(&fd, i0), 0xffff_ffff_ffff_ffff);
}

#[test]
fn sless2zero_subpiece_too_big_avn_rejected() {
    // avn size 9 > 8 => rule rejects (no comparison bigger than 8 bytes).
    let mut fd = build_fd();
    let sub = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_SUBPIECE);
    let avn = mk_vn(&mut fd, 0x10, 9);
    let hi = mk_const(&mut fd, 5, 8);
    fd.op_set_input(sub, avn, 0).unwrap();
    fd.op_set_input(sub, hi, 1).unwrap();
    let rvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, rvn, sub);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let m1 = mk_const(&mut fd, 0xffff_ffff, 4);
    fd.op_set_input(op, m1, 0).unwrap();
    fd.op_set_input(op, rvn, 1).unwrap();
    let mut rule = RuleSLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn sless2zero_negative_unwritten_other_side() {
    // -1 s< rvn with rvn not written => 0
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let m1 = mk_const(&mut fd, 0xffff_ffff, 4);
    let rvn = mk_vn(&mut fd, 0x20, 4); // free, unwritten
    fd.op_set_input(op, m1, 0).unwrap();
    fd.op_set_input(op, rvn, 1).unwrap();
    let mut rule = RuleSLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn sless2zero_negative_rhs_const_nonzero() {
    // lvn written, rvn const != 0 => 0 (only rvn==0 case handled)
    let mut fd = build_fd();
    let neg = mk_op(&mut fd, 1, 0x90, OpCode::CPUI_INT_NEGATE);
    let avn = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(neg, avn, 0).unwrap();
    let lvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lvn, neg);
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let c = mk_const(&mut fd, 7, 4);
    fd.op_set_input(op, lvn, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    let mut rule = RuleSLess2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

// =============================================================================
// RuleEqual2Zero
// =============================================================================

#[test]
fn equal2zero_const_form() {
    // 0 == (V + c)  =>  V == -c    (c = 3, size 4)
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let v = mk_input(&mut fd, 0x10, 4); // posvn must be heritage-known
    let c = mk_const(&mut fd, 3, 4);
    fd.op_set_input(add, v, 0).unwrap();
    fd.op_set_input(add, c, 1).unwrap();
    let addvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, addvn, add);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let zero = mk_const(&mut fd, 0, 4);
    fd.op_set_input(op, zero, 0).unwrap();
    fd.op_set_input(op, addvn, 1).unwrap();

    let mut rule = RuleEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // op: input0 = posvn (V), input1 = unnegvn (new const = -(c-1)-1 = -c = uintb_negate(c-1,4))
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v));
    let i1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert!(is_const(&fd, i1));
    // uintb_negate(3-1, 4) = ~(2) & mask = 0xfffffffd ; size-4 mask
    assert_eq!(off_of(&fd, i1), 0xffff_fffd);
}

#[test]
fn equal2zero_mult_form() {
    // 0 == (V + W*-1)  =>  V == W   (negvn = W * mask)
    let mut fd = build_fd();
    let mult = mk_op(&mut fd, 2, 0x80, OpCode::CPUI_INT_MULT);
    let w = mk_input(&mut fd, 0x8, 4); // unnegvn must be heritage-known
    let m1 = mk_const(&mut fd, 0xffff_ffff, 4); // -1 == calc_mask(4)
    fd.op_set_input(mult, w, 0).unwrap();
    fd.op_set_input(mult, m1, 1).unwrap();
    let negvn = mk_vn(&mut fd, 0x18, 4);
    set_def(&mut fd, negvn, mult);

    let add = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let v = mk_input(&mut fd, 0x10, 4); // posvn must be heritage-known
    fd.op_set_input(add, v, 0).unwrap();
    fd.op_set_input(add, negvn, 1).unwrap();
    let addvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, addvn, add);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let zero = mk_const(&mut fd, 0, 4);
    fd.op_set_input(op, zero, 0).unwrap();
    fd.op_set_input(op, addvn, 1).unwrap();

    let mut rule = RuleEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // posvn = v (vn), unnegvn = w
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(w));
}

#[test]
fn equal2zero_negative_non_bool_descendant() {
    // The sum's descendant set must be all boolean ops; a COPY descendant blocks.
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let v = mk_vn(&mut fd, 0x10, 4);
    let c = mk_const(&mut fd, 3, 4);
    fd.op_set_input(add, v, 0).unwrap();
    fd.op_set_input(add, c, 1).unwrap();
    let addvn = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, addvn, add);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let zero = mk_const(&mut fd, 0, 4);
    fd.op_set_input(op, zero, 0).unwrap();
    fd.op_set_input(op, addvn, 1).unwrap();
    // non-boolean descendant of the sum
    let cp = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_COPY);
    fd.op_set_input(cp, addvn, 0).unwrap();

    let mut rule = RuleEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    // unchanged inputs
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(addvn));
}

#[test]
fn equal2zero_negative_no_zero_operand() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let v0 = mk_vn(&mut fd, 0x8, 4);
    let v1 = mk_vn(&mut fd, 0x10, 4);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    let mut rule = RuleEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

// =============================================================================
// RuleEqual2Constant
// =============================================================================

#[test]
fn equal2constant_add_form() {
    // (V + c) == d  =>  V == (d - c)   (c = 4, d = 10, size 4)
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let v = mk_input(&mut fd, 0x10, 4); // a = leftop.in(0) must not be free
    let c = mk_const(&mut fd, 4, 4);
    fd.op_set_input(add, v, 0).unwrap();
    fd.op_set_input(add, c, 1).unwrap();
    let lhs = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lhs, add);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let d = mk_const(&mut fd, 10, 4);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, d, 1).unwrap();

    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    // op: input0 = V, input1 = const (10 - 4) = 6
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v));
    let i1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert!(is_const(&fd, i1));
    assert_eq!(off_of(&fd, i1), 6);
}

#[test]
fn equal2constant_negate_form() {
    // (~V) == c  =>  V == ~c   (c = 5, size 4 => ~5 & mask = 0xfffffffa)
    let mut fd = build_fd();
    let neg = mk_op(&mut fd, 1, 0x90, OpCode::CPUI_INT_NEGATE);
    let v = mk_input(&mut fd, 0x10, 4); // a = leftop.in(0) must not be free
    fd.op_set_input(neg, v, 0).unwrap();
    let lhs = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lhs, neg);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let c = mk_const(&mut fd, 5, 4);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();

    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v));
    let i1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert_eq!(off_of(&fd, i1), 0xffff_fffa);
}

#[test]
fn equal2constant_mult_minus1_form() {
    // (V * -1) == c  =>  V == -c   (c = 3, size 4 => -3 & mask = 0xfffffffd)
    let mut fd = build_fd();
    let mult = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_MULT);
    let v = mk_input(&mut fd, 0x10, 4); // a = leftop.in(0) must not be free
    let m1 = mk_const(&mut fd, 0xffff_ffff, 4);
    fd.op_set_input(mult, v, 0).unwrap();
    fd.op_set_input(mult, m1, 1).unwrap();
    let lhs = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lhs, mult);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let c = mk_const(&mut fd, 3, 4);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();

    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v));
    let i1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert_eq!(off_of(&fd, i1), 0xffff_fffd);
}

#[test]
fn equal2constant_negative_mult_not_minus1() {
    // (V * 2) == c  is rejected (only multiply by -1 transforms).
    let mut fd = build_fd();
    let mult = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_MULT);
    let v = mk_vn(&mut fd, 0x10, 4);
    let two = mk_const(&mut fd, 2, 4);
    fd.op_set_input(mult, v, 0).unwrap();
    fd.op_set_input(mult, two, 1).unwrap();
    let lhs = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lhs, mult);
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let c = mk_const(&mut fd, 3, 4);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn equal2constant_negative_rhs_not_constant() {
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let v = mk_vn(&mut fd, 0x10, 4);
    let c = mk_const(&mut fd, 4, 4);
    fd.op_set_input(add, v, 0).unwrap();
    fd.op_set_input(add, c, 1).unwrap();
    let lhs = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lhs, add);
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let d = mk_vn(&mut fd, 0x30, 4); // non-constant rhs
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, d, 1).unwrap();
    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn equal2constant_negative_other_descendant_wrong_form() {
    // a second descendant of lhs that is not an EQUAL/NOTEQUAL-with-const blocks.
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let v = mk_input(&mut fd, 0x10, 4); // non-free so the rule reaches the descendant loop
    let c = mk_const(&mut fd, 4, 4);
    fd.op_set_input(add, v, 0).unwrap();
    fd.op_set_input(add, c, 1).unwrap();
    let lhs = mk_vn(&mut fd, 0x20, 4);
    set_def(&mut fd, lhs, add);
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let d = mk_const(&mut fd, 10, 4);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, d, 1).unwrap();
    // an extra COPY descendant of lhs
    let cp = mk_op(&mut fd, 1, 0x120, OpCode::CPUI_COPY);
    fd.op_set_input(cp, lhs, 0).unwrap();
    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

// =============================================================================
// getOpList / group filtering surface (all nine rules)
// =============================================================================

#[test]
fn op_lists_match_cpp() {
    assert_eq!(RuleBoolNegate::new("g").get_op_list(), vec![OpCode::CPUI_BOOL_NEGATE]);
    assert_eq!(RuleLess2Zero::new("g").get_op_list(), vec![OpCode::CPUI_INT_LESS]);
    assert_eq!(RuleLessEqual2Zero::new("g").get_op_list(), vec![OpCode::CPUI_INT_LESSEQUAL]);
    assert_eq!(RuleSLess2Zero::new("g").get_op_list(), vec![OpCode::CPUI_INT_SLESS]);
    assert_eq!(
        RuleEqual2Zero::new("g").get_op_list(),
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    );
    assert_eq!(
        RuleEqual2Constant::new("g").get_op_list(),
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    );
    assert_eq!(RulePtrArith::new("g").get_op_list(), vec![OpCode::CPUI_INT_ADD]);
    assert_eq!(
        RuleStructOffset0::new("g").get_op_list(),
        vec![OpCode::CPUI_LOAD, OpCode::CPUI_STORE]
    );
    assert_eq!(RulePushPtr::new("g").get_op_list(), vec![OpCode::CPUI_INT_ADD]);
}

#[test]
fn clone_filters_by_group() {
    let inlist = ActionGroupList::from_names(["analysis"]);
    let empty = ActionGroupList::new();
    assert!(RuleBoolNegate::new("analysis").clone_rule(&inlist).is_some());
    assert!(RuleBoolNegate::new("analysis").clone_rule(&empty).is_none());
    // pointer rules belong to typerecovery
    let tr = ActionGroupList::from_names(["typerecovery"]);
    assert!(RulePtrArith::new("typerecovery").clone_rule(&tr).is_some());
    assert!(RulePtrArith::new("typerecovery").clone_rule(&inlist).is_none());
}

#[test]
fn specs_lists_nine_rules_in_order() {
    let specs = specs();
    assert_eq!(specs.len(), 9);
    // groups: first six analysis, last three typerecovery
    assert_eq!(specs[0].group, "analysis");
    assert_eq!(specs[5].group, "analysis");
    assert_eq!(specs[6].group, "typerecovery");
    assert_eq!(specs[8].group, "typerecovery");
}

#[test]
fn pointer_rules_are_noop_stubs() {
    // STUB-stubbed: applyOp returns 0 (no transform) regardless of input.
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let v0 = mk_vn(&mut fd, 0x10, 8);
    let v1 = mk_vn(&mut fd, 0x20, 8);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    assert_eq!(RulePtrArith::new("typerecovery").apply_op(op, &mut fd), 0);
    assert_eq!(RulePushPtr::new("typerecovery").apply_op(op, &mut fd), 0);
    let ld = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_LOAD);
    assert_eq!(RuleStructOffset0::new("typerecovery").apply_op(ld, &mut fd), 0);
}
