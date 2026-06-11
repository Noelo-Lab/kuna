//! Adversarial verifier tests for item `w5-s3-rules-1`
//! (ruleaction.cc:1-1443, RuleEarlyRemoval .. RuleRangeMeld).
//!
//! These exercise the spots the hunt list flagged as most fragile for the
//! ported (non-seamed) rules:
//!   * raw `>>` by a `uintb`-lineage shift amount (`RuleRightShiftAnd`): C++
//!     leaves an out-of-range shift as UB (x86 masks to 6 bits); Rust `u64 >> i32`
//!     panics in debug for `sa >= 64`.  (F1)
//!   * `RuleShiftBitops` INT_MULT `leastsigbit_set` path + the post-loop
//!     `getIn(1-i)` index selection.
//!   * `RuleHighOrderAnd` `(val-1)|val` wrapping at `val==0`.
//!   * `RuleLessOne` both opcode/constant variants (LESS w/ val!=1 declines,
//!     LESSEQUAL w/ val==0 keeps the constant slot).
//!   * `RuleNegateIdentity` descendant iteration order (first matching wins).
//!
//! The harness mirrors the in-module `tests.rs` harness exactly.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_1::{
    RuleHighOrderAnd, RuleLessOne, RuleNegateIdentity, RuleRightShiftAnd, RuleShiftBitops,
};
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};
use kuna_decomp::action::Rule;

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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

fn make_op_n(fd: &mut Funcdata, off: u64, opc: OpCode, ninputs: int4, flags: u32) -> OpId {
    let pc = Address::new(ram(fd), off);
    let id = fd.obank_mut().create_at(ninputs, pc);
    fd.obank_mut().mark_alive(id);
    fd.obank_mut().change_opcode(id, TypeOp::new(opc, flags, format!("{opc:?}")));
    id
}

fn make_op(fd: &mut Funcdata, off: u64, opc: OpCode) -> OpId {
    make_op_n(fd, off, opc, 2, 0)
}

fn make_reg(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let vn = fd.new_varnode(size, &Address::new(ram(fd), off), Some(unk(size)));
    fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
}

fn make_const(fd: &mut Funcdata, val: u64, size: int4) -> VarnodeId {
    fd.new_constant(size, val)
}

fn make_written(fd: &mut Funcdata, off: u64, size: int4, defopc: OpCode) -> (VarnodeId, OpId) {
    let pc = Address::new(ram(fd), off);
    let defop = fd.obank_mut().create_at(2, pc);
    fd.obank_mut().mark_alive(defop);
    fd.obank_mut().change_opcode(defop, TypeOp::new(defopc, 0, format!("{defopc:?}")));
    let vn = fd.new_varnode(size, &Address::new(ram(fd), off), Some(unk(size)));
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    (vn, defop)
}

fn give_output(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let vn = fd.new_varnode(size, &Address::new(ram(fd), off), Some(unk(size)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn wire_input(fd: &mut Funcdata, op: OpId, vn: VarnodeId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}
fn in_of(fd: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    fd.obank().get(op).unwrap().get_in(slot)
}
fn off_of(fd: &Funcdata, vn: VarnodeId) -> u64 {
    fd.vbank().get(vn).unwrap().get_offset()
}
fn is_const(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).unwrap().is_constant()
}

// -----------------------------------------------------------------------------
// F1 (FIXED): RuleRightShiftAnd shift by an out-of-range shift amount.
//
// C++ ruleaction.cc:593-596 does `maskVn->getOffset() >> sa` and
// `calc_mask(rootVn->getSize()) >> sa` with raw C++ `>>`.  `sa` is a
// data-derived p-code shift count that can be >= 64 (or negative after the
// `(int4)` truncation).  On the x86-64 reference build the count is masked to
// 6 bits, so `>> 64` behaves like `>> 0` and the rule APPLIES.  The Rust port
// now routes both sites through `wshr` (wrapping_shr, count taken mod 64),
// replicating the x86 masking exactly; this test pins the applied outcome.
// -----------------------------------------------------------------------------

#[test]
fn w5s3_rightshiftand_large_shift_masks_count_and_applies() {
    let mut fd = build_fd();
    // root size 8 so calc_mask(8) is a real full mask. and = root & mask.
    let root = make_reg(&mut fd, 0x40, 8);
    let (inv, and_op) = make_written(&mut fd, 0x60, 8, OpCode::CPUI_INT_AND);
    let mask = make_const(&mut fd, u64::MAX, 8);
    wire_input(&mut fd, and_op, root, 0);
    wire_input(&mut fd, and_op, mask, 1);
    // op = inv >> 64  (shift amount 64, a legal constant of size 8)
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_RIGHT);
    let sa = make_const(&mut fd, 64, 8);
    wire_input(&mut fd, op, inv, 0);
    wire_input(&mut fd, op, sa, 1);
    let mut r = RuleRightShiftAnd::new("analysis");

    // x86-masked: sa==64 -> count mod 64 == 0, so mask>>0 == full>>0 == u64::MAX,
    // full == mask passes, root is non-free -> rule applies and bypasses the AND.
    assert_eq!(r.apply_op(op, &mut fd), 1, "rule applies (x86-masked shift, full==mask)");
    assert_eq!(in_of(&fd, op, 0), Some(root), "AND bypassed, slot0 -> root");
}

// Boundary: sa == 63 must NOT panic (in range) and must behave correctly.
// With root size 8, full = calc_mask(8) = u64::MAX; mask const = u64::MAX.
// full>>63 == mask>>63 == 1, so the rule applies and bypasses the AND.
#[test]
fn w5s3_rightshiftand_shift_63_in_range_applies() {
    let mut fd = build_fd();
    let root = make_reg(&mut fd, 0x40, 8);
    let (inv, and_op) = make_written(&mut fd, 0x60, 8, OpCode::CPUI_INT_AND);
    let mask = make_const(&mut fd, u64::MAX, 8);
    wire_input(&mut fd, and_op, root, 0);
    wire_input(&mut fd, and_op, mask, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_RIGHT);
    let sa = make_const(&mut fd, 63, 8);
    wire_input(&mut fd, op, inv, 0);
    wire_input(&mut fd, op, sa, 1);
    let mut r = RuleRightShiftAnd::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(in_of(&fd, op, 0), Some(root), "AND bypassed, slot0 -> root");
}

// -----------------------------------------------------------------------------
// RuleShiftBitops: INT_MULT shift via leastsigbit_set + post-loop getIn(1-i).
// bitop = OR; its input i (the one whose NZM clears after the implied shift)
// breaks the loop, and for OR the result rewires op slot0 to bitop->getIn(1-i).
// Here we make branch i=1 the clearing one (NZM 0), so 1-i == 0 selects the
// heritage-known operand in slot 0.  This stresses the index arithmetic that a
// faithful port must keep (a transposed `1-i` would pick the zero operand).
// -----------------------------------------------------------------------------

#[test]
fn w5s3_shiftbitops_mult_or_picks_one_minus_i_operand() {
    let mut fd = build_fd();
    // bitop = OR(keep, z): slot1 (z) has NZM 0 -> clears -> i becomes 1 -> getIn(0)=keep
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_OR);
    let keep = make_reg(&mut fd, 0x40, 4); // slot 0, heritage-known
    let z = make_const(&mut fd, 0, 4); // slot 1, NZM 0
    wire_input(&mut fd, bitop, keep, 0);
    wire_input(&mut fd, bitop, z, 1);
    // op = innervn * 8  (INT_MULT, leastsigbit_set(8)==3, leftshift=true)
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_MULT);
    let coef = make_const(&mut fd, 8, 4);
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, coef, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // OR case, i==1 -> slot0 becomes bitop->getIn(1-1)=getIn(0)=keep.
    assert_eq!(in_of(&fd, op, 0), Some(keep));
}

// INT_MULT with an odd coefficient: leastsigbit_set returns 0, leftshift=true,
// sa==0 means no NZM clears (nzm<<0 unchanged), so no branch breaks and the
// rule declines.  (Also confirms leastsigbit_set(0)==-1 path is unreachable
// here because the const is non-zero; an even-but-cleared case is the prior
// test.)
#[test]
fn w5s3_shiftbitops_mult_odd_coeff_declines() {
    let mut fd = build_fd();
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_OR);
    let a = make_reg(&mut fd, 0x40, 4); // full NZM
    let b = make_reg(&mut fd, 0x48, 4); // full NZM, nothing clears under <<0
    wire_input(&mut fd, bitop, a, 0);
    wire_input(&mut fd, bitop, b, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_MULT);
    let coef = make_const(&mut fd, 5, 4); // odd -> leastsigbit_set==0 -> sa 0
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, coef, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// INT_MULT with coefficient 0: leastsigbit_set(0) == -1 -> rule returns 0 early.
#[test]
fn w5s3_shiftbitops_mult_zero_coeff_declines() {
    let mut fd = build_fd();
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_AND);
    let z = make_const(&mut fd, 0, 4);
    let other = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, bitop, z, 0);
    wire_input(&mut fd, bitop, other, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_MULT);
    let coef = make_const(&mut fd, 0, 4); // leastsigbit_set(0) == -1
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, coef, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleHighOrderAnd `(val-1)|val` wrapping at val==0.
// C++ uses `uintb` arithmetic: (0-1)|0 == 0xffffffffffffffff, which != the
// 4-byte calc_mask, so the rule declines.  A non-wrapping (checked) sub would
// have panicked; a faithful `wsub` reproduces the C++ value and declines.
// -----------------------------------------------------------------------------

#[test]
fn w5s3_highorderand_val_zero_wraps_and_declines() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let (addvn, addop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_ADD);
    let x = make_reg(&mut fd, 0x40, 4);
    let c = make_const(&mut fd, 0x10, 4);
    wire_input(&mut fd, addop, x, 0);
    wire_input(&mut fd, addop, c, 1);
    let cvn1 = make_const(&mut fd, 0, 4); // val == 0 -> (val-1)|val wraps to all ones
    wire_input(&mut fd, op, addvn, 0);
    wire_input(&mut fd, op, cvn1, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleHighOrderAnd::new("analysis");
    // No panic, and declines (0xffffffffffffffff != calc_mask(4)).
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// Form-check boundary: val == 0xfff0 IS a clean 1..10..0 mask, so
// `((val-1)|val) == calc_mask(size)` passes (the `wsub` path computes
// 0xffef|0xfff0 == 0xffff for size 2).  The rule then proceeds to the
// `cvn2->isConstant()` branch and declines only at the NZ-mask guard
// (`(NZM(xalign) & val) != NZM(xalign)` — xalign is a full-mask input reg).
// This separates "form check accepted" from the later decline, confirming the
// wrapping subtraction reached the right verdict for a clean sub-word mask
// (a `checked_sub` would have panicked on this exact `val`-path family).
#[test]
fn w5s3_highorderand_clean_submask_passes_form_then_nzmask_declines() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let (addvn, addop) = make_written(&mut fd, 0x60, 2, OpCode::CPUI_INT_ADD);
    let xalign = make_reg(&mut fd, 0x40, 2); // non-free input reg, full NZM
    let c2 = make_const(&mut fd, 0x30, 2);
    wire_input(&mut fd, addop, xalign, 0);
    wire_input(&mut fd, addop, c2, 1);
    let cvn1 = make_const(&mut fd, 0xfff0, 2); // ((val-1)|val)==0xffff==calc_mask(2)
    wire_input(&mut fd, op, addvn, 0);
    wire_input(&mut fd, op, cvn1, 1);
    let _out = give_output(&mut fd, op, 0x80, 2);
    let mut r = RuleHighOrderAnd::new("analysis");
    // Form accepted; NZmask guard fires -> declines, op unchanged.
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_AND);
}

// Negative form check: val == 0x0ff0 is NOT a clean 1..10..0 mask, so
// `((val-1)|val) != calc_mask(size)` and the rule declines at the form check
// (before any NZ-mask read).  Contrasts with the clean-mask test above.
#[test]
fn w5s3_highorderand_dirty_mask_declines_at_form_check() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let (addvn, addop) = make_written(&mut fd, 0x60, 2, OpCode::CPUI_INT_ADD);
    let xalign = make_reg(&mut fd, 0x40, 2);
    let c2 = make_const(&mut fd, 0x30, 2);
    wire_input(&mut fd, addop, xalign, 0);
    wire_input(&mut fd, addop, c2, 1);
    let cvn1 = make_const(&mut fd, 0x0ff0, 2); // ((0x0fef)|0x0ff0)=0x0fff != 0xffff
    wire_input(&mut fd, op, addvn, 0);
    wire_input(&mut fd, op, cvn1, 1);
    let _out = give_output(&mut fd, op, 0x80, 2);
    let mut r = RuleHighOrderAnd::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_AND);
}

// -----------------------------------------------------------------------------
// RuleLessOne variants: LESS w/ val 1 -> EQUAL 0 (resets slot1 const);
// LESSEQUAL w/ val 0 -> EQUAL keeping slot1; LESS w/ val 2 declines.
// -----------------------------------------------------------------------------

#[test]
fn w5s3_lessone_less_val1_becomes_equal_zero() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESS);
    let v = make_reg(&mut fd, 0x40, 4);
    let one = make_const(&mut fd, 1, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, one, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
    // val != 0 -> slot1 reset to a fresh const 0.
    let s1 = in_of(&fd, op, 1).unwrap();
    assert!(is_const(&fd, s1));
    assert_eq!(off_of(&fd, s1), 0);
}

#[test]
fn w5s3_lessone_lessequal_val0_keeps_slot1() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESSEQUAL);
    let v = make_reg(&mut fd, 0x40, 4);
    let zero = make_const(&mut fd, 0, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, zero, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
    // val == 0 -> slot1 unchanged (the original zero const).
    assert_eq!(in_of(&fd, op, 1), Some(zero));
}

#[test]
fn w5s3_lessone_less_val2_declines() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESS);
    let v = make_reg(&mut fd, 0x40, 4);
    let two = make_const(&mut fd, 2, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, two, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
}

// -----------------------------------------------------------------------------
// RuleNegateIdentity descendant iteration: when the NEGATE output feeds several
// logical ops, the rule must act on the FIRST (insertion-order) descendant that
// matches `getIn(1-slot)==vn` (C++ beginDescend()/endDescend()).  We wire a
// non-matching AND first (its other input is a foreign reg) and a matching XOR
// second; only the XOR should be collapsed, and only after the AND is skipped.
// -----------------------------------------------------------------------------

#[test]
fn w5s3_negateidentity_skips_nonmatching_then_collapses_match() {
    let mut fd = build_fd();
    // vn (the negated input) and its NEGATE op producing outVn.
    let vn = make_reg(&mut fd, 0x40, 4);
    let neg = make_op_n(&mut fd, 0x1000, OpCode::CPUI_INT_NEGATE, 1, 0);
    wire_input(&mut fd, neg, vn, 0);
    let out_vn = give_output(&mut fd, neg, 0x80, 4);

    // descendant 1: AND(outVn, foreign) -- getIn(1-slot) != vn -> skipped.
    let foreign = make_reg(&mut fd, 0x48, 4);
    let and_op = make_op(&mut fd, 0x1010, OpCode::CPUI_INT_AND);
    wire_input(&mut fd, and_op, out_vn, 0);
    wire_input(&mut fd, and_op, foreign, 1);

    // descendant 2: XOR(outVn, vn) -- getIn(1-slot)==vn -> the match.
    let xor_op = make_op(&mut fd, 0x1020, OpCode::CPUI_INT_XOR);
    wire_input(&mut fd, xor_op, out_vn, 0);
    wire_input(&mut fd, xor_op, vn, 1);

    let mut r = RuleNegateIdentity::new("analysis");
    assert_eq!(r.apply_op(neg, &mut fd), 1);
    // The AND was skipped (still INT_AND, 2 inputs); the XOR collapsed to COPY.
    assert_eq!(code_of(&fd, and_op), OpCode::CPUI_INT_AND);
    assert_eq!(code_of(&fd, xor_op), OpCode::CPUI_COPY);
    // XOR (opc != AND) -> slot0 set to const all-ones (calc_mask(4)).
    let s0 = in_of(&fd, xor_op, 0).unwrap();
    assert!(is_const(&fd, s0));
    assert_eq!(off_of(&fd, s0), kuna_base::address::calc_mask(4));
}
