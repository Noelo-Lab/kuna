// Tests for ruleaction_6 (W5 item w5-s3-rules-6).  Included into `mod tests`
// in ruleaction_6.rs (hence plain `//` comments here).
//
// Strategy (the merge base lacks opSetOpcode-by-OpCode, opSetOutput,
// newUniqueOut, and the type-facing/type-factory surface — see the module-level
// STUB notes):
//
//   * The PURE / read-only helpers are the load-bearing transcriptions and are
//     fully testable: `RuleDivOpt::calc_divisor` (128-bit reciprocal-divisor
//     math), `RuleDivTermAdd::find_subshift`, `RulePtrsubUndo::get_const_offset_back`
//     / `get_extra_offset`, `RuleDivOpt::find_form` (the constant-extended stub
//     forces None — asserted), `RuleDivOpt::move_sign_bit_extraction`
//     (opSetInput-only, fully committed), `RulePieceStructure::spanning_range`.
//   * The early-out GUARDS of the rules are exercised on hand-built op patterns:
//     each guard that uses available API is hit, and the stub-blocked commit
//     points are asserted to return 0 (no change), matching the C++ "made no
//     change" contract while the next wave fills the primitives.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, OpId, TypeOp, VarnodeId};
use crate::varnode::{varnode_flags, DefOpInfo, VarnodeBank};

// -----------------------------------------------------------------------------
// fixtures
// -----------------------------------------------------------------------------

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

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

/// A live op with `inputs` slots and the given opcode at ram offset `off`.
fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

/// A constant Varnode in the constant space.
fn mk_const(fd: &mut Funcdata, size: int4, val: uintb) -> VarnodeId {
    fd.new_constant(size, val)
}

/// A free (input-style) Varnode at a distinct ram address.
fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

/// Make `op` define a fresh written output Varnode at ram `off` (size `size`),
/// returning that output.  Mirrors funcdata_varnode's `make_written`.
fn give_output(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

/// Wire `vn` into `op`'s slot.
fn set_in(fd: &mut Funcdata, op: OpId, vn: VarnodeId, slot: int4) {
    fd.op_set_input(op, vn, slot).unwrap();
}

// =============================================================================
// RuleDivOpt::calc_divisor — pure 128-bit reciprocal-divisor arithmetic
// =============================================================================

#[test]
fn calc_divisor_precision_guards() {
    // n > 127 or xsize > 64 -> 0 (not enough precision).
    assert_eq!(RuleDivOpt::calc_divisor(128, &[3, 0], 32), 0);
    assert_eq!(RuleDivOpt::calc_divisor(40, &[3, 0], 65), 0);
}

#[test]
fn calc_divisor_boundary_y_le_1_is_zero() {
    // y <= 1 is the wrong form (ulessequal128(y, 1)).
    assert_eq!(RuleDivOpt::calc_divisor(32, &[1, 0], 16), 0);
    assert_eq!(RuleDivOpt::calc_divisor(32, &[0, 0], 16), 0);
}

#[test]
fn calc_divisor_recovers_known_magic() {
    // The classic unsigned div-by-3 magic for 32-bit: multiplier 0xAAAAAAAB,
    // shift 33 (2^33/(0xAAAAAAAB-1) == 3).  With xsize=32 (max numerand bits)
    // the optimization is valid and the recovered divisor is 3.
    let y: [u64; 2] = [0xAAAAAAAB, 0];
    let d = RuleDivOpt::calc_divisor(33, &y, 32);
    assert_eq!(d, 3, "div-by-3 reciprocal should recover divisor 3");
}

#[test]
fn calc_divisor_div_by_5() {
    // div-by-5: multiplier 0xCCCCCCCD, shift 34 (2^34/(0xCCCCCCCD-1) == 5).
    assert_eq!(RuleDivOpt::calc_divisor(34, &[0xCCCCCCCD, 0], 32), 5);
}

#[test]
fn calc_divisor_div_by_9() {
    // div-by-9: multiplier 0x38E38E39, shift 33 (2^33/(0x38E38E39-1) == 9, no
    // off-by-one adjustment needed).
    assert_eq!(RuleDivOpt::calc_divisor(33, &[0x38E38E39, 0], 32), 9);
}

// =============================================================================
// RuleDivTermAdd::find_subshift — pure form matcher
// =============================================================================

#[test]
fn find_subshift_plain_subpiece_high() {
    // sub(V8, c=4) with out size 4 == in size 8 - c=4  ->  high SUB, n = 4*8.
    let mut fd = build_fd();
    let vsrc_def = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let vsrc = give_output(&mut fd, vsrc_def, 0x10, 8);
    let sub = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, vsrc, 0);
    let c = mk_const(&mut fd, 4, 4);
    set_in(&mut fd, sub, c, 1);
    let _subout = give_output(&mut fd, sub, 0x20, 4);

    let r = RuleDivTermAdd::find_subshift(&fd, sub).expect("plain high SUBPIECE matches");
    assert_eq!(r.0, sub);
    assert_eq!(r.1, 4 * 8, "n == c*8 for a no-shift SUBPIECE");
    assert_eq!(r.2, OpCode::CPUI_MAX, "no shift -> CPUI_MAX");
}

#[test]
fn find_subshift_rejects_low_subpiece() {
    // out size 4, c=0 -> out+c (4) != in size (8) -> not high -> None.
    let mut fd = build_fd();
    let vsrc_def = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let vsrc = give_output(&mut fd, vsrc_def, 0x10, 8);
    let sub = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, vsrc, 0);
    let c = mk_const(&mut fd, 4, 0);
    set_in(&mut fd, sub, c, 1);
    let _subout = give_output(&mut fd, sub, 0x20, 4);

    assert!(RuleDivTermAdd::find_subshift(&fd, sub).is_none());
}

#[test]
fn find_subshift_shift_over_subpiece() {
    // (sub(V8, c=4) [out 4]) >> n=3  -> subop is the SUBPIECE, total n = 3 + 4*8.
    let mut fd = build_fd();
    let vsrc_def = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let vsrc = give_output(&mut fd, vsrc_def, 0x10, 8);
    let sub = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, vsrc, 0);
    let c = mk_const(&mut fd, 4, 4);
    set_in(&mut fd, sub, c, 1);
    let subout = give_output(&mut fd, sub, 0x20, 4);
    let shift = mk_op(&mut fd, 2, 0x108, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, subout, 0);
    let n = mk_const(&mut fd, 4, 3);
    set_in(&mut fd, shift, n, 1);

    let r = RuleDivTermAdd::find_subshift(&fd, shift).expect("shift-over-high-SUB matches");
    assert_eq!(r.0, sub);
    assert_eq!(r.1, 3 + 4 * 8);
    assert_eq!(r.2, OpCode::CPUI_INT_RIGHT);
}

#[test]
fn find_subshift_shift_without_subpiece_is_none() {
    // right shift whose input is NOT defined by a SUBPIECE -> None.
    let mut fd = build_fd();
    let src_def = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let src = give_output(&mut fd, src_def, 0x10, 8);
    let shift = mk_op(&mut fd, 2, 0x108, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, src, 0);
    let n = mk_const(&mut fd, 4, 3);
    set_in(&mut fd, shift, n, 1);
    assert!(RuleDivTermAdd::find_subshift(&fd, shift).is_none());
}

// =============================================================================
// RulePtrsubUndo::get_const_offset_back / get_extra_offset — pure walkers
// =============================================================================

#[test]
fn get_const_offset_back_constant_leaf() {
    let mut fd = build_fd();
    let c = mk_const(&mut fd, 8, 0x40);
    let (r, m) = RulePtrsubUndo::get_const_offset_back(&fd, c, RulePtrsubUndo::DEPTH_LIMIT);
    assert_eq!(r, 0x40);
    assert_eq!(m, 0);
}

#[test]
fn get_const_offset_back_add_of_constants() {
    // INT_ADD(0x10, 0x20) -> sum 0x30, multiplier 0.
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let c0 = mk_const(&mut fd, 8, 0x10);
    let c1 = mk_const(&mut fd, 8, 0x20);
    set_in(&mut fd, add, c0, 0);
    set_in(&mut fd, add, c1, 1);
    let out = give_output(&mut fd, add, 0x30, 8);
    let (r, m) = RulePtrsubUndo::get_const_offset_back(&fd, out, RulePtrsubUndo::DEPTH_LIMIT);
    assert_eq!(r, 0x30);
    assert_eq!(m, 0);
}

#[test]
fn get_const_offset_back_mult_sets_multiplier() {
    // INT_MULT(x, 8) -> retval 0, multiplier 8.
    let mut fd = build_fd();
    let mult = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
    let xdef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 8);
    set_in(&mut fd, mult, x, 0);
    let c = mk_const(&mut fd, 8, 8);
    set_in(&mut fd, mult, c, 1);
    let out = give_output(&mut fd, mult, 0x30, 8);
    let (r, m) = RulePtrsubUndo::get_const_offset_back(&fd, out, RulePtrsubUndo::DEPTH_LIMIT);
    assert_eq!(r, 0, "INT_MULT contributes 0 to the additive sum");
    assert_eq!(m, 8, "the multiplicative coefficient is passed back");
}

#[test]
fn get_const_offset_back_depth_limit_stops() {
    // A written non-const, non-add/mult op at depth 0 -> 0 (maxLevel guard).
    let mut fd = build_fd();
    let def = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let v = give_output(&mut fd, def, 0x10, 8);
    let (r, m) = RulePtrsubUndo::get_const_offset_back(&fd, v, 0); // maxLevel-1 < 0
    assert_eq!(r, 0);
    assert_eq!(m, 0);
}

// =============================================================================
// RuleDivOpt::find_form — the isConstantExtended stub forces None
// =============================================================================

#[test]
fn find_form_blocked_on_constant_extended_stub() {
    // sub(zext(x)*c, hi) with everything wired; the C++ would match, but
    // isConstantExtended is a W3-varnode stub (returns None here), so the MULT's
    // "there MUST be a constant" check fails -> None.  This pins the stub.
    let mut fd = build_fd();
    let xdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    let zext = mk_op(&mut fd, 1, 0x84, OpCode::CPUI_INT_ZEXT);
    set_in(&mut fd, zext, x, 0);
    let zout = give_output(&mut fd, zext, 0x18, 8);
    let mult = mk_op(&mut fd, 2, 0x88, OpCode::CPUI_INT_MULT);
    set_in(&mut fd, mult, zout, 0);
    let cmag = mk_const(&mut fd, 8, 0xAAAAAAAB);
    set_in(&mut fd, mult, cmag, 1);
    let mout = give_output(&mut fd, mult, 0x20, 8);
    let sub = mk_op(&mut fd, 2, 0x8c, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, mout, 0);
    let c = mk_const(&mut fd, 4, 4);
    set_in(&mut fd, sub, c, 1);
    let _sout = give_output(&mut fd, sub, 0x28, 4);

    assert!(
        RuleDivOpt::find_form(&fd, sub).is_none(),
        "isConstantExtended stub (None) makes the MULT constant check fail"
    );
}

#[test]
fn find_form_rejects_non_shift_non_subpiece_root() {
    // A root that is neither a shift nor a SUBPIECE -> None immediately.
    let mut fd = build_fd();
    let add = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let a = mk_vn(&mut fd, 0x10, 4);
    let b = mk_vn(&mut fd, 0x14, 4);
    set_in(&mut fd, add, a, 0);
    set_in(&mut fd, add, b, 1);
    let _o = give_output(&mut fd, add, 0x20, 4);
    assert!(RuleDivOpt::find_form(&fd, add).is_none());
}

// =============================================================================
// RuleDivOpt::move_sign_bit_extraction — fully committed (opSetInput only)
// =============================================================================

#[test]
fn move_sign_bit_extraction_redirects_signbit_shift() {
    // first = COPY out (size 4); a descendant `first s>> 31` extracts the sign
    // bit.  After moveSignBitExtraction(first, replace) the shift reads replace.
    let mut fd = build_fd();
    let fdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let first = give_output(&mut fd, fdef, 0x10, 4);
    let rdef = mk_op(&mut fd, 0, 0x84, OpCode::CPUI_COPY);
    let replace = give_output(&mut fd, rdef, 0x18, 4);

    let shift = mk_op(&mut fd, 2, 0x88, OpCode::CPUI_INT_SRIGHT);
    set_in(&mut fd, shift, first, 0);
    let sa = mk_const(&mut fd, 4, 31); // size*8 - 1 == 31 for a 4-byte vn
    set_in(&mut fd, shift, sa, 1);
    let _so = give_output(&mut fd, shift, 0x20, 4);

    RuleDivOpt::move_sign_bit_extraction(&mut fd, first, replace);
    assert_eq!(
        fd.obank().get(shift).unwrap().get_in(0),
        Some(replace),
        "sign-bit extraction shift should now read the replacement varnode"
    );
}

#[test]
fn move_sign_bit_extraction_ignores_non_signbit_shift() {
    // A right shift by a non-(size*8-1) amount must be left alone.
    let mut fd = build_fd();
    let fdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let first = give_output(&mut fd, fdef, 0x10, 4);
    let rdef = mk_op(&mut fd, 0, 0x84, OpCode::CPUI_COPY);
    let replace = give_output(&mut fd, rdef, 0x18, 4);

    let shift = mk_op(&mut fd, 2, 0x88, OpCode::CPUI_INT_SRIGHT);
    set_in(&mut fd, shift, first, 0);
    let sa = mk_const(&mut fd, 4, 5); // not the sign bit
    set_in(&mut fd, shift, sa, 1);
    let _so = give_output(&mut fd, shift, 0x20, 4);

    RuleDivOpt::move_sign_bit_extraction(&mut fd, first, replace);
    assert_eq!(
        fd.obank().get(shift).unwrap().get_in(0),
        Some(first),
        "a non-sign-bit shift must be untouched"
    );
}

// =============================================================================
// RulePieceStructure::spanning_range — pure dtype-tree walk
// =============================================================================

#[test]
fn spanning_range_out_of_bounds_is_false() {
    // offset+size > ct->getSize() -> false.
    let ct = Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN));
    assert!(!RulePieceStructure::spanning_range(&ct, 4, 8).unwrap());
}

#[test]
fn spanning_range_no_subtype_is_true() {
    // A non-structured base type has no sub-type -> getSubType returns None
    // -> "assume multiple".
    let ct = Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN));
    assert!(RulePieceStructure::spanning_range(&ct, 0, 4).unwrap());
}

// =============================================================================
// Rule early-out guards (negative tests on available API)
// =============================================================================

#[test]
fn mult_neg_one_requires_const_minus_one() {
    // INT_MULT(x, non-const) -> 0; INT_MULT(x, 5) -> 0 (5 != -1 mask).
    let mut fd = build_fd();
    let mut rule = RuleMultNegOne::new("g");

    let m1 = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
    let xdef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    set_in(&mut fd, m1, x, 0);
    let yvn = mk_vn(&mut fd, 0x14, 4); // non-constant slot 1
    set_in(&mut fd, m1, yvn, 1);
    assert_eq!(rule.apply_op(m1, &mut fd), 0, "non-constant multiplier: no fire");

    let m2 = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_MULT);
    set_in(&mut fd, m2, x, 0);
    let c5 = mk_const(&mut fd, 4, 5);
    set_in(&mut fd, m2, c5, 1);
    assert_eq!(rule.apply_op(m2, &mut fd), 0, "multiplier != -1: no fire");
}

#[test]
fn mult_neg_one_minus_one_fires_to_2comp() {
    // INT_MULT(x, 0xffffffff) passes both guards; the opSetOpcode/opRemoveInput
    // commit is now wired, so the rule fires: `x * -1 => -x` (INT_2COMP(x)).
    let mut fd = build_fd();
    let mut rule = RuleMultNegOne::new("g");
    let m = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
    let xdef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    set_in(&mut fd, m, x, 0);
    let cm1 = mk_const(&mut fd, 4, 0xffffffff);
    set_in(&mut fd, m, cm1, 1);
    assert_eq!(rule.apply_op(m, &mut fd), 1, "x * -1 => -x fires");
    assert_eq!(fd.obank().get(m).unwrap().code(), OpCode::CPUI_INT_2COMP);
    assert_eq!(fd.obank().get(m).unwrap().num_input(), 1, "constant input removed");
    assert_eq!(fd.obank().get(m).unwrap().get_in(0), Some(x), "operand preserved");
}

#[test]
fn positive_div_bails_when_input_may_be_negative() {
    // INT_SDIV where input 0's NZ mask has the sign bit set -> 0.
    let mut fd = build_fd();
    let mut rule = RulePositiveDiv::new("g");
    let sdiv = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SDIV);
    // input 0: a free varnode (NZ mask = full mask incl. sign bit for size 4).
    let a = mk_vn(&mut fd, 0x10, 4);
    let b = mk_vn(&mut fd, 0x14, 4);
    set_in(&mut fd, sdiv, a, 0);
    set_in(&mut fd, sdiv, b, 1);
    let _o = give_output(&mut fd, sdiv, 0x20, 4);
    assert_eq!(rule.apply_op(sdiv, &mut fd), 0, "sign bit possibly set -> no fire");
}

#[test]
fn positive_div_positive_inputs_converts_to_unsigned() {
    // Both inputs are small constants (NZ mask sign bit clear), so both guards
    // pass; the opSetOpcode commit is now wired, so the provably-positive signed
    // division becomes unsigned: INT_SDIV => INT_DIV.
    let mut fd = build_fd();
    let mut rule = RulePositiveDiv::new("g");
    let sdiv = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SDIV);
    let a = mk_const(&mut fd, 4, 0x10); // sign bit clear
    let b = mk_const(&mut fd, 4, 0x03);
    set_in(&mut fd, sdiv, a, 0);
    set_in(&mut fd, sdiv, b, 1);
    let _o = give_output(&mut fd, sdiv, 0x20, 4);
    assert_eq!(rule.apply_op(sdiv, &mut fd), 1, "provably positive -> unsigned div");
    assert_eq!(fd.obank().get(sdiv).unwrap().code(), OpCode::CPUI_INT_DIV);
}

#[test]
fn two_comp_two_sub_no_lone_add_descendant() {
    // INT_2COMP whose output has no lone INT_ADD descendant -> 0.
    let mut fd = build_fd();
    let mut rule = Rule2Comp2Sub::new("g");
    let comp = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_2COMP);
    let xdef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    set_in(&mut fd, comp, x, 0);
    let _o = give_output(&mut fd, comp, 0x20, 4); // no descendants at all
    assert_eq!(rule.apply_op(comp, &mut fd), 0);
}

#[test]
fn two_comp_two_sub_lone_descendant_not_add() {
    // The lone descendant is a COPY, not INT_ADD -> 0.
    let mut fd = build_fd();
    let mut rule = Rule2Comp2Sub::new("g");
    let comp = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_2COMP);
    let xdef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    set_in(&mut fd, comp, x, 0);
    let compout = give_output(&mut fd, comp, 0x20, 4);
    let copy = mk_op(&mut fd, 1, 0x104, OpCode::CPUI_COPY);
    set_in(&mut fd, copy, compout, 0);
    assert_eq!(rule.apply_op(comp, &mut fd), 0, "lone descendant must be INT_ADD");
}

#[test]
fn two_comp_two_sub_fires_v_plus_neg_w_to_v_minus_w() {
    // INT_2COMP feeding a lone INT_ADD: `v + -w => v - w`.  The opSetOpcode /
    // opSetInput / opDestroy commit is now wired, so the rule fires: the INT_ADD
    // becomes INT_SUB reading (v, w) and the 2COMP is destroyed.
    let mut fd = build_fd();
    let mut rule = Rule2Comp2Sub::new("g");
    let comp = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_2COMP);
    let wdef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let w = give_output(&mut fd, wdef, 0x10, 4);
    set_in(&mut fd, comp, w, 0);
    let compout = give_output(&mut fd, comp, 0x20, 4);
    let add = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD);
    let vdef = mk_op(&mut fd, 0, 0x94, OpCode::CPUI_COPY);
    let v = give_output(&mut fd, vdef, 0x14, 4);
    set_in(&mut fd, add, v, 0);
    set_in(&mut fd, add, compout, 1);
    let _ao = give_output(&mut fd, add, 0x28, 4);

    assert_eq!(rule.apply_op(comp, &mut fd), 1, "v + -w => v - w fires");
    // INT_ADD became INT_SUB reading (v, w); 2COMP destroyed (inputs unset).
    assert_eq!(fd.obank().get(add).unwrap().code(), OpCode::CPUI_INT_SUB);
    assert_eq!(fd.obank().get(add).unwrap().get_in(0), Some(v));
    assert_eq!(fd.obank().get(add).unwrap().get_in(1), Some(w), "subtrahend is w");
    // opDestroy unsets the 2COMP's input/output (the parent-less synthetic op is
    // not block-removed, but its def/use links are severed).
    assert_eq!(fd.obank().get(comp).unwrap().get_in(0), None, "2COMP input unset");
    assert_eq!(fd.obank().get(comp).unwrap().get_out(), None, "2COMP output unset");
}

#[test]
fn sub_normal_requires_written_shift_input() {
    // SUBPIECE whose input 0 is a free (unwritten) varnode -> 0.
    let mut fd = build_fd();
    let mut rule = RuleSubNormal::new("g");
    let sub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_SUBPIECE);
    let a = mk_vn(&mut fd, 0x10, 8); // free, not written
    set_in(&mut fd, sub, a, 0);
    let c = mk_const(&mut fd, 4, 1);
    set_in(&mut fd, sub, c, 1);
    let _o = give_output(&mut fd, sub, 0x20, 4);
    assert_eq!(rule.apply_op(sub, &mut fd), 0);
}

#[test]
fn sub_normal_requires_shift_def() {
    // SUBPIECE whose input 0 is written by a COPY (not a shift) -> 0.
    let mut fd = build_fd();
    let mut rule = RuleSubNormal::new("g");
    let cpy = mk_op(&mut fd, 1, 0x90, OpCode::CPUI_COPY);
    let cpyin = mk_vn(&mut fd, 0x08, 8);
    set_in(&mut fd, cpy, cpyin, 0);
    let cpyout = give_output(&mut fd, cpy, 0x10, 8);
    let sub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, cpyout, 0);
    let c = mk_const(&mut fd, 4, 1);
    set_in(&mut fd, sub, c, 1);
    let _o = give_output(&mut fd, sub, 0x20, 4);
    assert_eq!(rule.apply_op(sub, &mut fd), 0, "shift input must be defined by a right shift");
}

#[test]
fn sub_normal_unnecessary_shift_collapses_to_subpiece() {
    // sub( (a >> n), c ) where n == k*8 and after folding n becomes 0:
    //   a: size 8, shift right by n=8 (k=1), SUB c=1, out size 4.
    //   total: n+8c+8*out = 8+8+32 = 48 >= 8*insize=64? no -> but n==k*8 so the
    //   first guard's `&& (n != k*8)` is false -> not rejected.
    //   k+c+out = 1+1+4 = 6 < insize 8 -> skip the extension branch.
    //   c += k -> 2 ; n -= k*8 -> 0  => the shift is unnecessary:
    //     opSetInput(op, a, 0); opSetInput(op, newConstant(4, 2), 1); return 1.
    // This whole path uses ONLY available API (opSetInput/newConstant), so it
    // commits — a positive transformation test.
    let mut fd = build_fd();
    let mut rule = RuleSubNormal::new("g");

    let adef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    let shift = mk_op(&mut fd, 2, 0x84, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, a, 0);
    let n = mk_const(&mut fd, 4, 8); // n = 8 -> k = 1
    set_in(&mut fd, shift, n, 1);
    let shiftout = give_output(&mut fd, shift, 0x20, 8);

    let sub = mk_op(&mut fd, 2, 0x88, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, shiftout, 0);
    let c = mk_const(&mut fd, 4, 1);
    set_in(&mut fd, sub, c, 1);
    let _o = give_output(&mut fd, sub, 0x30, 4);

    assert_eq!(rule.apply_op(sub, &mut fd), 1, "unnecessary-shift fold commits");
    // op now reads a directly, with constant 2 in slot 1.
    assert_eq!(fd.obank().get(sub).unwrap().get_in(0), Some(a));
    let new_c = fd.obank().get(sub).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(new_c).unwrap().is_constant());
    assert_eq!(fd.vbank().get(new_c).unwrap().get_offset(), 2);
    // The op is still a SUBPIECE (opcode unchanged on this path).
    assert_eq!(fd.obank().get(sub).unwrap().code(), OpCode::CPUI_SUBPIECE);
}

#[test]
fn div_term_add2_requires_shift_by_one() {
    // INT_RIGHT by 2 (not 1) -> 0.
    let mut fd = build_fd();
    let mut rule = RuleDivTermAdd2::new("g");
    let shift = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_RIGHT);
    let adef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    set_in(&mut fd, shift, a, 0);
    let two = mk_const(&mut fd, 4, 2);
    set_in(&mut fd, shift, two, 1);
    let _o = give_output(&mut fd, shift, 0x20, 8);
    assert_eq!(rule.apply_op(shift, &mut fd), 0, "must be >> 1");
}

#[test]
fn div_term_add2_non_const_shift_amount() {
    // INT_RIGHT by a non-constant -> 0.
    let mut fd = build_fd();
    let mut rule = RuleDivTermAdd2::new("g");
    let shift = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_RIGHT);
    let adef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    set_in(&mut fd, shift, a, 0);
    let amt = mk_vn(&mut fd, 0x18, 4); // non-constant
    set_in(&mut fd, shift, amt, 1);
    let _o = give_output(&mut fd, shift, 0x20, 8);
    assert_eq!(rule.apply_op(shift, &mut fd), 0);
}

#[test]
fn extension_push_const_input_bails() {
    // INT_ZEXT of a constant -> 0 (first guard).
    let mut fd = build_fd();
    let mut rule = RuleExtensionPush::new("g");
    let zext = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_ZEXT);
    let c = mk_const(&mut fd, 4, 0x10);
    set_in(&mut fd, zext, c, 0);
    let _o = give_output(&mut fd, zext, 0x20, 8);
    assert_eq!(rule.apply_op(zext, &mut fd), 0);
}

#[test]
fn extension_push_single_descendant_bails() {
    // INT_ZEXT with exactly one PTRADD descendant -> (addcount+ptrcount)<=1 -> 0.
    let mut fd = build_fd();
    let mut rule = RuleExtensionPush::new("g");
    let xdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    let zext = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_ZEXT);
    set_in(&mut fd, zext, x, 0);
    let zout = give_output(&mut fd, zext, 0x20, 8);
    let ptradd = mk_op(&mut fd, 3, 0x104, OpCode::CPUI_PTRADD);
    set_in(&mut fd, ptradd, zout, 0);
    let idx = mk_vn(&mut fd, 0x28, 8);
    set_in(&mut fd, ptradd, idx, 1);
    let elt = mk_const(&mut fd, 8, 4);
    set_in(&mut fd, ptradd, elt, 2);
    assert_eq!(rule.apply_op(zext, &mut fd), 0, "single pointer use -> no duplicate");
}

#[test]
fn extension_push_non_pointer_descendant_bails() {
    // INT_ZEXT feeding a plain COPY (neither PTRADD nor qualifying INT_ADD) -> 0.
    let mut fd = build_fd();
    let mut rule = RuleExtensionPush::new("g");
    let xdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let x = give_output(&mut fd, xdef, 0x10, 4);
    let zext = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_INT_ZEXT);
    set_in(&mut fd, zext, x, 0);
    let zout = give_output(&mut fd, zext, 0x20, 8);
    let cpy = mk_op(&mut fd, 1, 0x104, OpCode::CPUI_COPY);
    set_in(&mut fd, cpy, zout, 0);
    assert_eq!(rule.apply_op(zext, &mut fd), 0);
}

#[test]
fn ptradd_undo_skips_without_type_recovery() {
    // hasTypeRecoveryStarted() is false on a fresh Funcdata -> 0 immediately.
    let mut fd = build_fd();
    let mut rule = RulePtraddUndo::new("g");
    assert!(!fd.has_type_recovery_started());
    let ptradd = mk_op(&mut fd, 3, 0x100, OpCode::CPUI_PTRADD);
    let base = mk_vn(&mut fd, 0x10, 8);
    let idx = mk_vn(&mut fd, 0x18, 8);
    let elt = mk_const(&mut fd, 8, 4);
    set_in(&mut fd, ptradd, base, 0);
    set_in(&mut fd, ptradd, idx, 1);
    set_in(&mut fd, ptradd, elt, 2);
    let _o = give_output(&mut fd, ptradd, 0x20, 8);
    assert_eq!(rule.apply_op(ptradd, &mut fd), 0);
}

#[test]
fn sub_right_special_printing_bails() {
    // doesSpecialPrinting() set on the op -> 0 (first guard).  We set the marker
    // through the op flag the C++ checks.
    let mut fd = build_fd();
    let mut rule = RuleSubRight::new("g");
    let sub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_SUBPIECE);
    let adef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    set_in(&mut fd, sub, a, 0);
    let c = mk_const(&mut fd, 4, 4);
    set_in(&mut fd, sub, c, 1);
    let _o = give_output(&mut fd, sub, 0x20, 4);
    // Mark special printing (the guard the C++ checks first).
    fd.obank_mut()
        .get_mut(sub)
        .unwrap()
        .set_additional_flag(crate::op::pcodeop_addlflags::special_print);
    assert_eq!(rule.apply_op(sub, &mut fd), 0);
}

#[test]
fn sub_right_least_significant_bails() {
    // c == 0: the SUBPIECE already truncates to the low bytes, so it prints as a
    // cast and needs no shift.  The offset input must survive untouched.
    let mut fd = build_fd();
    let mut rule = RuleSubRight::new("g");
    let sub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_SUBPIECE);
    let adef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    set_in(&mut fd, sub, a, 0);
    let c = mk_const(&mut fd, 4, 0);
    set_in(&mut fd, sub, c, 1);
    let _o = give_output(&mut fd, sub, 0x20, 4);
    assert_eq!(rule.apply_op(sub, &mut fd), 0);
    assert_eq!(fd.obank().get(sub).unwrap().get_in(1), Some(c));
}

#[test]
fn sub_right_addr_tied_marker_bails() {
    // Output and input both address-tied and overlapping at exactly c: this
    // SUBPIECE is the storage marker ActionCopyMarker converts, so the rewrite
    // must decline and leave the op untouched (dropping this guard would turn
    // stack-piece writes back into shift chains).
    let mut fd = build_fd();
    let mut rule = RuleSubRight::new("g");
    let sub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_SUBPIECE);
    let adef = mk_op(&mut fd, 0, 0x90, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    set_in(&mut fd, sub, a, 0);
    let c = mk_const(&mut fd, 4, 4);
    set_in(&mut fd, sub, c, 1);
    let o = give_output(&mut fd, sub, 0x14, 4); // ram 0x14 is byte 4 of ram 0x10
    for vn in [a, o] {
        fd.vbank_mut()
            .get_mut(vn)
            .unwrap()
            .set_flags_pub(varnode_flags::mapped | varnode_flags::addrtied);
    }
    assert!(fd.vbank().get(o).unwrap().is_addr_tied());
    assert!(fd.vbank().get(a).unwrap().is_addr_tied());
    assert_eq!(rule.apply_op(sub, &mut fd), 0);
    assert_eq!(fd.obank().get(sub).unwrap().get_in(1), Some(c));
}

// =============================================================================
// specs() — registration list shape (C++ definition order, exact names)
// =============================================================================

#[test]
fn specs_lists_all_fourteen_in_definition_order() {
    let s = specs();
    assert_eq!(s.len(), 14);
    let groups: Vec<&str> = s.iter().map(|r| r.group).collect();
    assert_eq!(
        groups,
        vec![
            "ptraddundo",
            "ptrsubundo",
            "multnegone",
            "addunsigned",
            "2comp2sub",
            "subright",
            "ptrsubcharconstant",
            "extensionpush",
            "piecestructure",
            "subnormal",
            "positivediv",
            "divtermadd",
            "divtermadd2",
            "divopt",
        ]
    );
}

#[test]
fn clone_rule_filters_on_group() {
    let rule = RuleMultNegOne::new("analysis");
    let gl = ActionGroupList::from_names(["analysis"]);
    assert!(rule.clone_rule(&gl).is_some());
    let other = ActionGroupList::from_names(["somethingelse"]);
    assert!(rule.clone_rule(&other).is_none());
}
