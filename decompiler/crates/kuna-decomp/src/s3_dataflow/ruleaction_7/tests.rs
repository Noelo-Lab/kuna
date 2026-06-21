//! Adversarial unit tests for the 15 rules ported in `ruleaction_7`
//! (`RuleSignDiv2` .. `RuleIgnoreNan`).
//!
//! For the intricate rules we hand-build the exact input op pattern on a real
//! [`Funcdata`] and assert the precise transformation (opcode changes, inputs
//! rewired, constants created) demanded by the C++ `applyOp` body, plus negative
//! tests for the early-out guards.  The "shared helper" `check_sign_extraction`
//! is also unit-tested directly.
//!
//! IR-building helpers mirror `tests/verify_w3_ir_funcdata_varnode.rs`:
//! `make_input` (a formal input varnode), `make_op` (an op with a fixed opcode
//! and a written output varnode), `wire` (descend link + op input slot), and
//! `konst` (a fresh constant).  `op_of` reads back an op's current opcode.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{Architecture, OpId, TypeOp, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

use super::*;

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

/// A free (non-input, non-written) varnode at (off,size).  Used for the
/// `isFree()` negative-guard tests.
fn free_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

/// A formal input varnode at (off,size).
fn make_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), None);
    fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
}

/// A fresh constant varnode (size,val).
fn konst(fd: &mut Funcdata, size: int4, val: uintb) -> VarnodeId {
    fd.new_constant(size, val)
}

/// Allocate an op with `num_in` inputs, opcode `opc`, located at `off`, that
/// writes a fresh output varnode at address (off,outsize).  Returns (op, out).
fn make_op(
    fd: &mut Funcdata,
    off: u64,
    opc: OpCode,
    num_in: int4,
    outsize: int4,
) -> (OpId, VarnodeId) {
    let r = ram(fd);
    let pc = Address::new(Rc::clone(&r), off);
    let op = fd.obank_mut().create_at(num_in, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    // Output varnode at a distinct address (off) so set_def doesn't dedup.
    let out = fd.new_varnode(outsize, &Address::new(r, off), Some(unk(outsize)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let out = fd.vbank_mut().set_def(out, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
    (op, out)
}

/// An op with `num_in` inputs and NO output (e.g. the root op under test).
fn make_op_noout(fd: &mut Funcdata, off: u64, opc: OpCode, num_in: int4) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(num_in, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

/// Wire `vn` as input slot `slot` of `op` (descend link + op input).
fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// Minimal opcode flags for ops we build (branch/booloutput where it matters).
fn opcode_flags(opc: OpCode) -> u32 {
    use crate::op::pcodeop_flags as f;
    match opc {
        OpCode::CPUI_CBRANCH => f::branch,
        OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_EQUAL | OpCode::CPUI_FLOAT_EQUAL => f::booloutput,
        _ => 0,
    }
}

/// Read back an op's current opcode.
fn op_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

/// Read back the constant offset of an op's input slot (assumes a constant).
fn in_off(fd: &Funcdata, op: OpId, slot: int4) -> uintb {
    let vn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
    fd.vbank().get(vn).unwrap().get_offset()
}

/// Read back an op's input varnode id at slot.
fn in_vn(fd: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    fd.obank().get(op).unwrap().get_in(slot).unwrap()
}

// ===========================================================================
// RuleSignDiv2  ( a s>> 0x1f, then  ... INT_MULT by -1, then  ... INT_ADD,
//   with INT_SRIGHT(in1==1) at the root  =>  a s/ 2 )
// ===========================================================================

/// Build the full positive pattern for RuleSignDiv2 and return (root, a).
/// Pattern (size 4, so 8*4-1 = 31 = 0x1f):
///   shiftop  = a s>> 0x1f
///   multop   = shiftout * (-1)            (-1 == calc_mask(4) == 0xffffffff)
///   addop    = a + multout                (a in slot 1-i)
///   root     = addout s>> 1               (in1 == 1)
fn build_signdiv2(fd: &mut Funcdata) -> (OpId, VarnodeId) {
    let a = make_input(fd, 0x100, 4);

    // shiftop = a s>> 0x1f
    let (shiftop, shiftout) = make_op(fd, 0x200, OpCode::CPUI_INT_SRIGHT, 2, 4);
    wire(fd, a, shiftop, 0);
    let c1f = konst(fd, 4, 0x1f);
    wire(fd, c1f, shiftop, 1);

    // multop = shiftout * (-1)
    let (multop, multout) = make_op(fd, 0x210, OpCode::CPUI_INT_MULT, 2, 4);
    wire(fd, shiftout, multop, 0);
    let cneg = konst(fd, 4, calc_mask(4)); // -1
    wire(fd, cneg, multop, 1);

    // addop = multout + a   (multout in slot 0, a in slot 1 => i==0, a==getIn(1))
    let (addop, addout) = make_op(fd, 0x220, OpCode::CPUI_INT_ADD, 2, 4);
    wire(fd, multout, addop, 0);
    wire(fd, a, addop, 1);

    // root = addout s>> 1
    let root = make_op_noout(fd, 0x230, OpCode::CPUI_INT_SRIGHT, 2);
    wire(fd, addout, root, 0);
    let c1 = konst(fd, 4, 1);
    wire(fd, c1, root, 1);

    (root, a)
}

#[test]
fn signdiv2_positive_rewrites_to_sdiv_by_2() {
    let mut fd = build_fd();
    let (root, a) = build_signdiv2(&mut fd);

    let mut rule = RuleSignDiv2::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "must fire");

    // op now: a s/ 2
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_SDIV, "opcode -> INT_SDIV");
    assert_eq!(in_vn(&fd, root, 0), a, "input0 = a");
    assert_eq!(in_off(&fd, root, 1), 2, "input1 = constant 2");
}

#[test]
fn signdiv2_negative_in1_not_one() {
    let mut fd = build_fd();
    let (root, _a) = build_signdiv2(&mut fd);
    // Replace the root's in1 (==1) with constant 2 -> guard fails.
    let c2 = konst(&mut fd, 4, 2);
    // unwire old, set new
    fd.op_set_input(root, c2, 1).unwrap();

    let mut rule = RuleSignDiv2::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "in1 != 1 must not fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_SRIGHT, "unchanged");
}

#[test]
fn signdiv2_negative_wrong_shift_amount() {
    // n must equal 8*a->getSize()-1 (==31).  Use 30 -> no match.
    let mut fd = build_fd();
    let a = make_input(&mut fd, 0x100, 4);
    let (shiftop, shiftout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SRIGHT, 2, 4);
    wire(&mut fd, a, shiftop, 0);
    let cbad = konst(&mut fd, 4, 30); // wrong shift amount
    wire(&mut fd, cbad, shiftop, 1);
    let (multop, multout) = make_op(&mut fd, 0x210, OpCode::CPUI_INT_MULT, 2, 4);
    wire(&mut fd, shiftout, multop, 0);
    let cneg = konst(&mut fd, 4, calc_mask(4));
    wire(&mut fd, cneg, multop, 1);
    let (addop, addout) = make_op(&mut fd, 0x220, OpCode::CPUI_INT_ADD, 2, 4);
    wire(&mut fd, multout, addop, 0);
    wire(&mut fd, a, addop, 1);
    let root = make_op_noout(&mut fd, 0x230, OpCode::CPUI_INT_SRIGHT, 2);
    wire(&mut fd, addout, root, 0);
    let c1 = konst(&mut fd, 4, 1);
    wire(&mut fd, c1, root, 1);

    let mut rule = RuleSignDiv2::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "wrong shift -> no fire");
}

// ===========================================================================
// RuleDivChain  ( (x / c1) / c2  =>  x / (c1*c2) )
// ===========================================================================

#[test]
fn divchain_positive_sdiv_collapses() {
    // x sdiv 3 ; result sdiv 5  =>  x sdiv 15
    let mut fd = build_fd();
    let x = make_input(&mut fd, 0x100, 4);
    let (divop, divout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SDIV, 2, 4);
    wire(&mut fd, x, divop, 0);
    let c3 = konst(&mut fd, 4, 3);
    wire(&mut fd, c3, divop, 1);
    // root = divout sdiv 5  (divout has a lone descend -> root)
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_SDIV, 2);
    wire(&mut fd, divout, root, 0);
    let c5 = konst(&mut fd, 4, 5);
    wire(&mut fd, c5, root, 1);

    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_SDIV, "still SDIV");
    assert_eq!(in_vn(&fd, root, 0), x, "input0 = x");
    assert_eq!(in_off(&fd, root, 1), 15, "input1 = 3*5 = 15");
}

#[test]
fn divchain_negative_intermediate_has_other_use() {
    // If divout has more than one descendant, loneDescend()==0 -> no fire.
    let mut fd = build_fd();
    let x = make_input(&mut fd, 0x100, 4);
    let (divop, divout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SDIV, 2, 4);
    wire(&mut fd, x, divop, 0);
    let c3 = konst(&mut fd, 4, 3);
    wire(&mut fd, c3, divop, 1);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_SDIV, 2);
    wire(&mut fd, divout, root, 0);
    let c5 = konst(&mut fd, 4, 5);
    wire(&mut fd, c5, root, 1);
    // Second use of divout.
    let other = make_op_noout(&mut fd, 0x220, OpCode::CPUI_INT_ADD, 2);
    wire(&mut fd, divout, other, 0);
    let c1 = konst(&mut fd, 4, 1);
    wire(&mut fd, c1, other, 1);

    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "non-lone intermediate -> no fire");
}

#[test]
fn divchain_negative_signed_overflow() {
    // SDIV: bitcount > sz*8-2 must reject.  sz=4 -> limit 30.
    // c1 = 0x40000000 (bit 30), c2 = 2 (bit1): msb(val1)=30,msb(val2)=1 ->
    // bitcount = 30+1+2 = 33 > 30 -> reject.  (resval nonzero: 0x80000000.)
    let mut fd = build_fd();
    let x = make_input(&mut fd, 0x100, 4);
    let (divop, divout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SDIV, 2, 4);
    wire(&mut fd, x, divop, 0);
    let c1 = konst(&mut fd, 4, 0x4000_0000);
    wire(&mut fd, c1, divop, 1);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_SDIV, 2);
    wire(&mut fd, divout, root, 0);
    let c2 = konst(&mut fd, 4, 2);
    wire(&mut fd, c2, root, 1);

    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "signed overflow -> no fire");
}

// ===========================================================================
// RuleSignForm  ( sub(sext(V),c)  =>  V s>> (8*size-1) )  with c >= size
// ===========================================================================

#[test]
fn signform_positive() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    // sextop: 8-byte SEXT of v
    let (sextop, sextout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SEXT, 1, 8);
    wire(&mut fd, v, sextop, 0);
    // root = SUBPIECE(sextout, c)  with c (==4) >= a->getSize() (==4)
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_SUBPIECE, 2);
    wire(&mut fd, sextout, root, 0);
    let c = konst(&mut fd, 4, 4);
    wire(&mut fd, c, root, 1);

    let mut rule = RuleSignForm::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_SRIGHT, "-> INT_SRIGHT");
    assert_eq!(in_vn(&fd, root, 0), v, "input0 = V");
    assert_eq!(in_off(&fd, root, 1), 31, "input1 = 8*4-1 = 31");
}

#[test]
fn signform_negative_c_too_small() {
    // c < a->getSize() rejects.
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (sextop, sextout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SEXT, 1, 8);
    wire(&mut fd, v, sextop, 0);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_SUBPIECE, 2);
    wire(&mut fd, sextout, root, 0);
    let c = konst(&mut fd, 4, 2); // < 4
    wire(&mut fd, c, root, 1);

    let mut rule = RuleSignForm::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "c < size -> no fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_SUBPIECE, "unchanged");
}

#[test]
fn signform_negative_not_sext() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (copyop, copyout) = make_op(&mut fd, 0x200, OpCode::CPUI_COPY, 1, 8);
    wire(&mut fd, v, copyop, 0);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_SUBPIECE, 2);
    wire(&mut fd, copyout, root, 0);
    let c = konst(&mut fd, 4, 4);
    wire(&mut fd, c, root, 1);

    let mut rule = RuleSignForm::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "in0 not SEXT -> no fire");
}

// ===========================================================================
// RuleModOpt  ( x - (x/d)*d  =>  x % d   via INT_REM/INT_SREM )
// ===========================================================================

#[test]
fn modopt_positive_signed_constant_divisor() {
    // op = x sdiv d (d=7) ; multop = (x/d) * (-7) ; addop = x + multout
    //   => addop becomes  x SREM 7
    let mut fd = build_fd();
    let x = make_input(&mut fd, 0x100, 4);
    let (op, divout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SDIV, 2, 4);
    wire(&mut fd, x, op, 0);
    let d = konst(&mut fd, 4, 7);
    wire(&mut fd, d, op, 1);

    // multop: divout * (-7)  ; -7 == (~7+1)&mask
    let (multop, multout) = make_op(&mut fd, 0x210, OpCode::CPUI_INT_MULT, 2, 4);
    wire(&mut fd, divout, multop, 0);
    let neg7 = konst(&mut fd, 4, ((!7u64).wrapping_add(1)) & calc_mask(4));
    wire(&mut fd, neg7, multop, 1);

    // addop: x + multout  (x in slot 0, multout in slot 1 => lvn==x)
    let addop = make_op_noout(&mut fd, 0x220, OpCode::CPUI_INT_ADD, 2);
    wire(&mut fd, x, addop, 0);
    wire(&mut fd, multout, addop, 1);

    let mut rule = RuleModOpt::new();
    assert_eq!(rule.apply_op(op, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, addop), OpCode::CPUI_INT_SREM, "addop -> INT_SREM");
    assert_eq!(in_vn(&fd, addop, 0), x, "addop in0 = x");
    assert_eq!(in_off(&fd, addop, 1), 7, "addop in1 = divisor 7");
}

#[test]
fn modopt_negative_wrong_complement() {
    // multop's constant is NOT the 2's complement of d -> no fire.
    let mut fd = build_fd();
    let x = make_input(&mut fd, 0x100, 4);
    let (op, divout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SDIV, 2, 4);
    wire(&mut fd, x, op, 0);
    let d = konst(&mut fd, 4, 7);
    wire(&mut fd, d, op, 1);
    let (multop, multout) = make_op(&mut fd, 0x210, OpCode::CPUI_INT_MULT, 2, 4);
    wire(&mut fd, divout, multop, 0);
    let bad = konst(&mut fd, 4, 5); // not -7
    wire(&mut fd, bad, multop, 1);
    let addop = make_op_noout(&mut fd, 0x220, OpCode::CPUI_INT_ADD, 2);
    wire(&mut fd, x, addop, 0);
    wire(&mut fd, multout, addop, 1);

    let mut rule = RuleModOpt::new();
    assert_eq!(rule.apply_op(op, &mut fd), 0, "wrong complement -> no fire");
    assert_eq!(op_of(&fd, addop), OpCode::CPUI_INT_ADD, "addop unchanged");
}

#[test]
fn modopt_negative_div_unsigned_becomes_rem() {
    // INT_DIV path -> INT_REM (unsigned).  Verify opcode chosen correctly.
    let mut fd = build_fd();
    let x = make_input(&mut fd, 0x100, 4);
    let (op, divout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_DIV, 2, 4);
    wire(&mut fd, x, op, 0);
    let d = konst(&mut fd, 4, 7);
    wire(&mut fd, d, op, 1);
    let (multop, multout) = make_op(&mut fd, 0x210, OpCode::CPUI_INT_MULT, 2, 4);
    wire(&mut fd, divout, multop, 0);
    let neg7 = konst(&mut fd, 4, ((!7u64).wrapping_add(1)) & calc_mask(4));
    wire(&mut fd, neg7, multop, 1);
    let addop = make_op_noout(&mut fd, 0x220, OpCode::CPUI_INT_ADD, 2);
    wire(&mut fd, x, addop, 0);
    wire(&mut fd, multout, addop, 1);

    let mut rule = RuleModOpt::new();
    assert_eq!(rule.apply_op(op, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, addop), OpCode::CPUI_INT_REM, "INT_DIV -> INT_REM (unsigned)");
}

// ===========================================================================
// RuleNegateNegate  ( ~~V  =>  V )
// ===========================================================================

#[test]
fn negatenegate_positive() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (neg2, neg2out) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_NEGATE, 1, 4);
    wire(&mut fd, v, neg2, 0);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_NEGATE, 1);
    wire(&mut fd, neg2out, root, 0);

    let mut rule = RuleNegateNegate::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_COPY, "-> COPY");
    assert_eq!(in_vn(&fd, root, 0), v, "input0 = V (inner negate's input)");
}

#[test]
fn negatenegate_negative_inner_not_negate() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (cop, copout) = make_op(&mut fd, 0x200, OpCode::CPUI_COPY, 1, 4);
    wire(&mut fd, v, cop, 0);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_NEGATE, 1);
    wire(&mut fd, copout, root, 0);

    let mut rule = RuleNegateNegate::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "inner not NEGATE -> no fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_NEGATE, "unchanged");
}

#[test]
fn negatenegate_negative_input_not_written() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let root = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_NEGATE, 1);
    wire(&mut fd, v, root, 0); // input is an INPUT varnode, not written

    let mut rule = RuleNegateNegate::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "in0 not written -> no fire");
}

// ===========================================================================
// RuleFloatCast  (several arms)
// ===========================================================================

#[test]
fn floatcast_float2float_superfluous_identity() {
    // F2F(F2F(vn2:4):8):4  with insize1(8) > outsize(4) and outsize==insize2(4)
    //   => identity COPY of vn2.
    let mut fd = build_fd();
    let vn2 = make_input(&mut fd, 0x100, 4);
    let (inner, innerout) = make_op(&mut fd, 0x200, OpCode::CPUI_FLOAT_FLOAT2FLOAT, 1, 8);
    wire(&mut fd, vn2, inner, 0);
    let root = make_op(&mut fd, 0x210, OpCode::CPUI_FLOAT_FLOAT2FLOAT, 1, 4).0;
    wire(&mut fd, innerout, root, 0);

    let mut rule = RuleFloatCast::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_COPY, "outsize==insize2 -> COPY identity");
    assert_eq!(in_vn(&fd, root, 0), vn2, "input0 = vn2");
}

#[test]
fn floatcast_int2float_through_float2float() {
    // F2F(INT2F(vn2):8):4  =>  INT2F(vn2):4
    let mut fd = build_fd();
    let vn2 = make_input(&mut fd, 0x100, 4);
    let (inner, innerout) = make_op(&mut fd, 0x200, OpCode::CPUI_FLOAT_INT2FLOAT, 1, 8);
    wire(&mut fd, vn2, inner, 0);
    let root = make_op(&mut fd, 0x210, OpCode::CPUI_FLOAT_FLOAT2FLOAT, 1, 4).0;
    wire(&mut fd, innerout, root, 0);

    let mut rule = RuleFloatCast::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "must fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_FLOAT_INT2FLOAT, "-> INT2FLOAT");
    assert_eq!(in_vn(&fd, root, 0), vn2, "input0 = vn2");
}

#[test]
fn floatcast_negative_free_input() {
    // vn2 free -> don't propagate.
    let mut fd = build_fd();
    let vn2 = free_vn(&mut fd, 0x100, 4);
    let (inner, innerout) = make_op(&mut fd, 0x200, OpCode::CPUI_FLOAT_FLOAT2FLOAT, 1, 8);
    wire(&mut fd, vn2, inner, 0);
    let root = make_op(&mut fd, 0x210, OpCode::CPUI_FLOAT_FLOAT2FLOAT, 1, 4).0;
    wire(&mut fd, innerout, root, 0);

    let mut rule = RuleFloatCast::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "free vn2 -> no fire");
}

// ===========================================================================
// RuleSignMod2nOpt::check_sign_extraction  (shared helper)
// ===========================================================================

#[test]
fn check_sign_extraction_matches_full_width_sright() {
    // V s>> 31 (size 4) -> returns V.
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (sop, sout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SRIGHT, 2, 4);
    wire(&mut fd, v, sop, 0);
    let c = konst(&mut fd, 4, 31);
    wire(&mut fd, c, sop, 1);

    assert_eq!(
        RuleSignMod2nOpt::check_sign_extraction(&fd, sout),
        Some(v),
        "V s>> (8*size-1) recognized"
    );
}

#[test]
fn check_sign_extraction_rejects_wrong_shift() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (sop, sout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SRIGHT, 2, 4);
    wire(&mut fd, v, sop, 0);
    let c = konst(&mut fd, 4, 30); // not 31
    wire(&mut fd, c, sop, 1);

    assert_eq!(
        RuleSignMod2nOpt::check_sign_extraction(&fd, sout),
        None,
        "wrong shift amount rejected"
    );
}

#[test]
fn check_sign_extraction_rejects_non_sright() {
    let mut fd = build_fd();
    let v = make_input(&mut fd, 0x100, 4);
    let (op, out) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_RIGHT, 2, 4); // unsigned
    wire(&mut fd, v, op, 0);
    let c = konst(&mut fd, 4, 31);
    wire(&mut fd, c, op, 1);

    assert_eq!(RuleSignMod2nOpt::check_sign_extraction(&fd, out), None, "not INT_SRIGHT");
}

// ===========================================================================
// RulePtrFlow  (propagation helpers, COPY arm)
// ===========================================================================

#[test]
fn ptrflow_copy_propagates_when_flagged() {
    // A COPY op already marked ptrflow: applyOp should propagate to its out's
    // readers and its in0's def, returning a change.
    let mut fd = build_fd();
    let src = make_input(&mut fd, 0x100, 8);
    let (copyop, copyout) = make_op(&mut fd, 0x200, OpCode::CPUI_COPY, 1, 8);
    wire(&mut fd, src, copyop, 0);
    // A reader of copyout (an INT_ADD, in the propagateFlowToReads switch set).
    let reader = make_op_noout(&mut fd, 0x210, OpCode::CPUI_INT_ADD, 2);
    wire(&mut fd, copyout, reader, 0);
    let c = konst(&mut fd, 8, 1);
    wire(&mut fd, c, reader, 1);

    // Mark the COPY as ptrflow (precondition of the COPY arm).
    fd.obank_mut().get_mut(copyop).unwrap().set_ptr_flow();

    let mut rule = RulePtrFlow::new();
    assert_eq!(rule.apply_op(copyop, &mut fd), 1, "ptrflow COPY must report a change");
    // copyout varnode is now ptrflow.
    assert!(fd.vbank().get(copyout).unwrap().is_ptr_flow(), "out varnode marked ptrflow");
    // the reader (INT_ADD) is now ptrflow.
    assert!(fd.obank().get(reader).unwrap().is_ptr_flow(), "reader op marked ptrflow");
}

#[test]
fn ptrflow_copy_not_flagged_is_noop() {
    let mut fd = build_fd();
    let src = make_input(&mut fd, 0x100, 8);
    let (copyop, _copyout) = make_op(&mut fd, 0x200, OpCode::CPUI_COPY, 1, 8);
    wire(&mut fd, src, copyop, 0);
    // NOT marked ptrflow -> early return 0.
    let mut rule = RulePtrFlow::new();
    assert_eq!(rule.apply_op(copyop, &mut fd), 0, "un-flagged COPY -> no change");
}

#[test]
fn ptrflow_oplist_empty_without_truncations() {
    // hasTruncations defaults false (W4 seam) -> getOpList must be empty.
    let rule = RulePtrFlow::new();
    assert!(rule.get_op_list().is_empty(), "no truncations -> rule installs nothing");
}

// ===========================================================================
// getOpList / clone spot-checks (registration faithfulness)
// ===========================================================================

#[test]
fn oplists_match_cpp() {
    assert_eq!(RuleSignDiv2::new().get_op_list(), vec![OpCode::CPUI_INT_SRIGHT]);
    assert_eq!(
        RuleDivChain::new().get_op_list(),
        vec![OpCode::CPUI_INT_DIV, OpCode::CPUI_INT_SDIV]
    );
    assert_eq!(RuleSignForm::new().get_op_list(), vec![OpCode::CPUI_SUBPIECE]);
    assert_eq!(RuleSignForm2::new().get_op_list(), vec![OpCode::CPUI_INT_SRIGHT]);
    assert_eq!(RuleSignNearMult::new().get_op_list(), vec![OpCode::CPUI_INT_AND]);
    assert_eq!(
        RuleModOpt::new().get_op_list(),
        vec![OpCode::CPUI_INT_DIV, OpCode::CPUI_INT_SDIV]
    );
    assert_eq!(RuleSignMod2nOpt::new().get_op_list(), vec![OpCode::CPUI_INT_RIGHT]);
    assert_eq!(RuleSignMod2Opt::new().get_op_list(), vec![OpCode::CPUI_INT_AND]);
    assert_eq!(RuleSignMod2nOpt2::new().get_op_list(), vec![OpCode::CPUI_INT_MULT]);
    assert_eq!(RuleSegment::new().get_op_list(), vec![OpCode::CPUI_SEGMENTOP]);
    assert_eq!(RuleNegateNegate::new().get_op_list(), vec![OpCode::CPUI_INT_NEGATE]);
    assert_eq!(RuleConditionalMove::new().get_op_list(), vec![OpCode::CPUI_MULTIEQUAL]);
    assert_eq!(
        RuleFloatCast::new().get_op_list(),
        vec![OpCode::CPUI_FLOAT_FLOAT2FLOAT, OpCode::CPUI_FLOAT_TRUNC]
    );
    assert_eq!(RuleIgnoreNan::new().get_op_list(), vec![OpCode::CPUI_FLOAT_NAN]);
}

#[test]
fn clone_respects_group_membership() {
    let analysis = ActionGroupList::from_names(["analysis"]);
    let empty = ActionGroupList::new();
    assert!(RuleSignDiv2::new().clone_rule(&analysis).is_some(), "in analysis");
    assert!(RuleSignDiv2::new().clone_rule(&empty).is_none(), "not in empty");

    // RuleConditionalMove belongs to "conditionalexe".
    let condexe = ActionGroupList::from_names(["conditionalexe"]);
    assert!(RuleConditionalMove::new().clone_rule(&condexe).is_some());
    assert!(RuleConditionalMove::new().clone_rule(&analysis).is_none());

    // RuleFloatCast / RuleIgnoreNan belong to "floatprecision".
    let fp = ActionGroupList::from_names(["floatprecision"]);
    assert!(RuleFloatCast::new().clone_rule(&fp).is_some());
    assert!(RuleIgnoreNan::new().clone_rule(&fp).is_some());
    assert!(RuleFloatCast::new().clone_rule(&analysis).is_none());

    // RulePtrFlow belongs to "subvar"; RuleSegment to "segment".
    let subvar = ActionGroupList::from_names(["subvar"]);
    assert!(RulePtrFlow::new().clone_rule(&subvar).is_some());
    let segment = ActionGroupList::from_names(["segment"]);
    assert!(RuleSegment::new().clone_rule(&segment).is_some());
}

#[test]
fn specs_lists_all_fifteen_in_definition_order() {
    let specs = specs();
    assert_eq!(specs.len(), 15, "15 rules in this batch");
    // Spot-check group assignments at known positions.
    assert_eq!(specs[0].group, "analysis"); // RuleSignDiv2
    assert_eq!(specs[9].group, "segment"); // RuleSegment
    assert_eq!(specs[10].group, "subvar"); // RulePtrFlow
    assert_eq!(specs[12].group, "conditionalexe"); // RuleConditionalMove
    assert_eq!(specs[13].group, "floatprecision"); // RuleFloatCast
    assert_eq!(specs[14].group, "floatprecision"); // RuleIgnoreNan
}
