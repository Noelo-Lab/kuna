// Tests for the `ruleaction_2` rule batch (included into `mod tests` in
// ruleaction_2.rs).  Each test hand-builds the exact op pattern the C++
// `applyOp` keys on, runs the rule, and asserts the precise transformation
// (opcode changes, new ops/constants, input rewiring) per the upstream body, or
// pins an early-out guard for the negative tests.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, BlockId};
use crate::varnode::DefOpInfo;

// -----------------------------------------------------------------------------
// Fixture
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

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

/// A permissive opcode->TypeOp resolver for *test-input* ops (covers opcodes
/// the rule bodies never emit, so are absent from the production `op_typeop`
/// seam).  The flag word carries the standard `binary`/`unary`/`booloutput`
/// bits from `typeop.cc` so `is_cse_match` and friends behave; exact symbol
/// names are decorative here.
fn tst_typeop(opc: OpCode) -> TypeOp {
    use crate::op::pcodeop_flags as f;
    let flags = match opc {
        OpCode::CPUI_COPY => f::unary | f::nocollapse,
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_BOOL_NEGATE => f::unary,
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            f::binary | f::booloutput | f::commutative
        }
        OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_FLOAT_LESS
        | OpCode::CPUI_FLOAT_LESSEQUAL
        | OpCode::CPUI_FLOAT_EQUAL
        | OpCode::CPUI_FLOAT_NOTEQUAL => f::binary | f::booloutput,
        OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR => {
            f::binary | f::commutative | f::booloutput
        }
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
            f::binary | f::commutative
        }
        _ => f::binary,
    };
    TypeOp::new(opc, flags, format!("{opc:?}"))
}

/// Create an op (in the dead list) with `inputs` slots and a resolved opcode.
fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, tst_typeop(opc));
    op
}

/// A function-input register varnode at a ram address: non-free and
/// heritage-known (promoted via `set_input`/`xref`), so it survives the
/// `isFree()`/`isHeritageKnown()` guards the rules apply to leaf operands.
fn mk_reg(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(size, Address::new(ram, off), dt(size));
    let mut no_replace =
        |_b: &mut crate::varnode::VarnodeBank, _a: VarnodeId, _c: VarnodeId| Ok(());
    fd.vbank_mut().set_input(vn, &mut no_replace).expect("mk_reg set_input")
}

/// A constant varnode (free, in the constant space; heritage-known).
fn mk_const(fd: &mut Funcdata, size: int4, val: u64) -> VarnodeId {
    fd.new_constant(size, val)
}

/// Build a "written" varnode: the output of a defining op `opc` placed in block
/// `bl`, with the given inputs.  Returns (output_vn, def_op).  The output is
/// heritage-known (has the `insert` flag via set_def/xref).
fn mk_written(
    fd: &mut Funcdata,
    bl: BlockId,
    off: u64,
    out_size: int4,
    opc: OpCode,
    ins: &[VarnodeId],
) -> (VarnodeId, OpId) {
    let defop = mk_op(fd, ins.len() as int4, off, opc);
    for (i, &vn) in ins.iter().enumerate() {
        fd.op_set_input(defop, vn, i as int4).expect("mk_written set input");
    }
    let ram = ram_space(fd);
    let outvn = fd.new_varnode(out_size, &Address::new(ram, off + 0x800), Some(dt(out_size)));
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let mut no_replace =
        |_b: &mut crate::varnode::VarnodeBank, _a: VarnodeId, _c: VarnodeId| Ok(());
    let outvn = fd.vbank_mut().set_def(outvn, info, &mut no_replace).expect("mk_written set_def");
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(outvn));
    fd.op_insert(defop, bl, None);
    (outvn, defop)
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}
fn in_of(fd: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    fd.obank().get(op).unwrap().get_in(slot)
}
fn const_val(fd: &Funcdata, vn: VarnodeId) -> u64 {
    fd.vbank().get(vn).unwrap().get_offset()
}
fn is_const(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).unwrap().is_constant()
}

// =============================================================================
// RuleFloatRange  (intricate: slot normalization + dual comparison match)
// =============================================================================

#[test]
fn floatrange_less_or_equal_collapses_to_lessequal() {
    // (V f< C) || (V f== C)  =>  V f<= C
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let c1 = mk_const(&mut fd, 4, 5);
    let c2 = mk_const(&mut fd, 4, 5);
    // cmp1 = V f< C ; cmp2 = V f== C
    let (less_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_FLOAT_LESS, &[v, c1]);
    let (eq_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_FLOAT_EQUAL, &[v, c2]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, less_out, 0).unwrap();
    fd.op_set_input(or, eq_out, 1).unwrap();
    fd.op_insert(or, bl, None);

    let r = RuleFloatRange.apply_op(or, &mut fd);
    assert_eq!(r, 1);
    assert_eq!(code_of(&fd, or), OpCode::CPUI_FLOAT_LESSEQUAL);
    // Inputs collapse to (V, C): slot1 stays 0 (nvn1 = V at slot 0 of cmp1).
    assert_eq!(in_of(&fd, or, 0), Some(v));
    // cvn1 was a constant (5); a fresh constant of value 5 is placed in slot 1.
    let s1 = in_of(&fd, or, 1).unwrap();
    assert!(is_const(&fd, s1));
    assert_eq!(const_val(&fd, s1), 5);
}

#[test]
fn floatrange_lessequal_and_notequal_collapses_to_less() {
    // (V f<= C) && (V f!= C)  =>  V f< C
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let c1 = mk_const(&mut fd, 4, 9);
    let c2 = mk_const(&mut fd, 4, 9);
    let (le_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_FLOAT_LESSEQUAL, &[v, c1]);
    let (ne_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_FLOAT_NOTEQUAL, &[v, c2]);
    let and = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_AND);
    fd.op_set_input(and, le_out, 0).unwrap();
    fd.op_set_input(and, ne_out, 1).unwrap();
    fd.op_insert(and, bl, None);

    assert_eq!(RuleFloatRange.apply_op(and, &mut fd), 1);
    assert_eq!(code_of(&fd, and), OpCode::CPUI_FLOAT_LESS);
}

#[test]
fn floatrange_declines_mismatched_pair() {
    // (V f< C) || (V f!= C): FLOAT_LESS with FLOAT_NOTEQUAL under BOOL_OR is not
    // one of the recognized combinations -> resultopc stays COPY -> no change.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let c1 = mk_const(&mut fd, 4, 5);
    let c2 = mk_const(&mut fd, 4, 5);
    let (less_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_FLOAT_LESS, &[v, c1]);
    let (ne_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_FLOAT_NOTEQUAL, &[v, c2]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, less_out, 0).unwrap();
    fd.op_set_input(or, ne_out, 1).unwrap();
    fd.op_insert(or, bl, None);

    assert_eq!(RuleFloatRange.apply_op(or, &mut fd), 0);
    assert_eq!(code_of(&fd, or), OpCode::CPUI_BOOL_OR);
}

#[test]
fn floatrange_declines_unwritten_input() {
    // Early-out: in(0) not written.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_reg(&mut fd, 0x100, 1);
    let b = mk_reg(&mut fd, 0x104, 1);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, a, 0).unwrap();
    fd.op_set_input(or, b, 1).unwrap();
    fd.op_insert(or, bl, None);
    assert_eq!(RuleFloatRange.apply_op(or, &mut fd), 0);
}

// =============================================================================
// RuleDoubleSub  (sub(sub(V,c),d) => sub(V, c+d))
// =============================================================================

#[test]
fn doublesub_combines_offsets() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 8);
    let c2 = mk_const(&mut fd, 4, 2); // inner SUBPIECE offset
    let (inner_out, _i) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_SUBPIECE, &[v, c2]);
    let c1 = mk_const(&mut fd, 4, 1); // outer SUBPIECE offset
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(outer, inner_out, 0).unwrap();
    fd.op_set_input(outer, c1, 1).unwrap();
    fd.op_insert(outer, bl, None);

    assert_eq!(RuleDoubleSub.apply_op(outer, &mut fd), 1);
    assert_eq!(code_of(&fd, outer), OpCode::CPUI_SUBPIECE);
    assert_eq!(in_of(&fd, outer, 0), Some(v)); // middleman skipped
    let off = in_of(&fd, outer, 1).unwrap();
    assert!(is_const(&fd, off));
    assert_eq!(const_val(&fd, off), 3); // 1 + 2
}

#[test]
fn doublesub_declines_non_subpiece_inner() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 8);
    let (inner_out, _i) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_COPY, &[v]);
    let c1 = mk_const(&mut fd, 4, 1);
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(outer, inner_out, 0).unwrap();
    fd.op_set_input(outer, c1, 1).unwrap();
    fd.op_insert(outer, bl, None);
    assert_eq!(RuleDoubleSub.apply_op(outer, &mut fd), 0);
}

// =============================================================================
// RuleDoubleShift  (same-direction combine; opposite cancel)
// =============================================================================

#[test]
fn doubleshift_same_direction_adds() {
    // (V << 2) << 3  =>  V << 5
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let two = mk_const(&mut fd, 4, 2);
    let (sh_out, _s) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_LEFT, &[v, two]);
    let three = mk_const(&mut fd, 4, 3);
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_LEFT);
    fd.op_set_input(outer, sh_out, 0).unwrap();
    fd.op_set_input(outer, three, 1).unwrap();
    fd.op_insert(outer, bl, None);

    assert_eq!(RuleDoubleShift.apply_op(outer, &mut fd), 1);
    assert_eq!(code_of(&fd, outer), OpCode::CPUI_INT_LEFT);
    assert_eq!(in_of(&fd, outer, 0), Some(v)); // skip middle shift
    let sa = in_of(&fd, outer, 1).unwrap();
    assert_eq!(const_val(&fd, sa), 5); // 2 + 3
}

#[test]
fn doubleshift_same_direction_overshift_zeroes() {
    // (V << 30) << 30  on a 4-byte value (32 bits): 60 >= 32 => COPY #0
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let s1 = mk_const(&mut fd, 4, 30);
    let (sh_out, _s) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_LEFT, &[v, s1]);
    let s2 = mk_const(&mut fd, 4, 30);
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_LEFT);
    fd.op_set_input(outer, sh_out, 0).unwrap();
    fd.op_set_input(outer, s2, 1).unwrap();
    fd.op_insert(outer, bl, None);

    assert_eq!(RuleDoubleShift.apply_op(outer, &mut fd), 1);
    assert_eq!(code_of(&fd, outer), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(outer).unwrap().num_input(), 1);
    let z = in_of(&fd, outer, 0).unwrap();
    assert!(is_const(&fd, z));
    assert_eq!(const_val(&fd, z), 0);
}

#[test]
fn doubleshift_declines_non_constant_shift() {
    // Outer shift amount non-constant -> early-out.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let two = mk_const(&mut fd, 4, 2);
    let (sh_out, _s) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_LEFT, &[v, two]);
    let amt = mk_reg(&mut fd, 0x200, 4); // non-constant
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_LEFT);
    fd.op_set_input(outer, sh_out, 0).unwrap();
    fd.op_set_input(outer, amt, 1).unwrap();
    fd.op_insert(outer, bl, None);
    assert_eq!(RuleDoubleShift.apply_op(outer, &mut fd), 0);
}

// =============================================================================
// RuleTrivialArith  (V==V => true ; V^V => #0 ; V&V => COPY V)
// =============================================================================

#[test]
fn trivialarith_equal_same_input_is_true() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleTrivialArith.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
    let b = in_of(&fd, op, 0).unwrap();
    assert!(is_const(&fd, b));
    assert_eq!(const_val(&fd, b), 1); // boolean true
}

#[test]
fn trivialarith_notequal_same_input_is_false() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleTrivialArith.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    let b = in_of(&fd, op, 0).unwrap();
    assert_eq!(const_val(&fd, b), 0); // boolean false
}

#[test]
fn trivialarith_xor_same_input_is_sized_zero() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_XOR);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, v, 1).unwrap();
    // XOR needs an output (newConstant uses op->getOut()->getSize()).
    let (out, _d) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_XOR, &[v, v]);
    let _ = (op, out);
    // Use the written op so getOut() is valid.
    let xorop = _d;
    assert_eq!(RuleTrivialArith.apply_op(xorop, &mut fd), 1);
    assert_eq!(code_of(&fd, xorop), OpCode::CPUI_COPY);
    let z = in_of(&fd, xorop, 0).unwrap();
    assert!(is_const(&fd, z));
    assert_eq!(const_val(&fd, z), 0);
    assert_eq!(fd.vbank().get(z).unwrap().get_size(), 4); // same size 0
}

#[test]
fn trivialarith_and_same_input_is_identity_copy() {
    // V & V => COPY (vn == 0 branch: keep getIn(0)).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let (op, defop) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_AND, &[v, v]);
    let _ = op;
    assert_eq!(RuleTrivialArith.apply_op(defop, &mut fd), 1);
    assert_eq!(code_of(&fd, defop), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(defop).unwrap().num_input(), 1);
    assert_eq!(in_of(&fd, defop, 0), Some(v)); // identity: input preserved
}

#[test]
fn trivialarith_declines_distinct_unrelated_inputs() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_reg(&mut fd, 0x100, 4);
    let b = mk_reg(&mut fd, 0x104, 4);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    fd.op_insert(op, bl, None);
    assert_eq!(RuleTrivialArith.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
}

// =============================================================================
// RuleTrivialBool
// =============================================================================

#[test]
fn trivialbool_and_true_is_copy_of_v() {
    // V && true  =>  COPY V
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 1);
    let t = mk_const(&mut fd, 1, 1);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_BOOL_AND);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, t, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleTrivialBool.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(in_of(&fd, op, 0), Some(v));
}

#[test]
fn trivialbool_and_false_is_copy_false() {
    // V && false  =>  COPY #0
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 1);
    let f = mk_const(&mut fd, 1, 0);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_BOOL_AND);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, f, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleTrivialBool.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    let z = in_of(&fd, op, 0).unwrap();
    assert!(is_const(&fd, z));
    assert_eq!(const_val(&fd, z), 0);
}

#[test]
fn trivialbool_xor_true_is_negate() {
    // V ^^ true  =>  !V (BOOL_NEGATE, keeps in(0))
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 1);
    let t = mk_const(&mut fd, 1, 1);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_BOOL_XOR);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, t, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleTrivialBool.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_BOOL_NEGATE);
    assert_eq!(in_of(&fd, op, 0), Some(v));
}

#[test]
fn trivialbool_declines_non_constant_second() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 1);
    let w = mk_reg(&mut fd, 0x104, 1);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(op, v, 0).unwrap();
    fd.op_set_input(op, w, 1).unwrap();
    fd.op_insert(op, bl, None);
    assert_eq!(RuleTrivialBool.apply_op(op, &mut fd), 0);
}

// =============================================================================
// RuleSlessToLess  (uses nz-mask of constant operands to prove non-negative)
// =============================================================================

#[test]
fn slesstoless_nonnegative_constants_convert() {
    // Both operands constants with sign bit clear -> SLESS becomes LESS.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_const(&mut fd, 4, 0x10); // nzmask = 0x10, sign bit (bit 31) clear
    let b = mk_const(&mut fd, 4, 0x20);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_SLESS);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleSlessToLess.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
}

#[test]
fn slessequal_nonnegative_constants_convert() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_const(&mut fd, 4, 1);
    let b = mk_const(&mut fd, 4, 2);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_SLESSEQUAL);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleSlessToLess.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESSEQUAL);
}

#[test]
fn slesstoless_declines_when_signbit_possible() {
    // First operand has the 4-byte sign bit set in its nz-mask -> decline.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_const(&mut fd, 4, 0x8000_0000); // sign bit (bit 31) set
    let b = mk_const(&mut fd, 4, 1);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_SLESS);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    fd.op_insert(op, bl, None);
    assert_eq!(RuleSlessToLess.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SLESS);
}

// =============================================================================
// RuleAndZext  (sext(X) & mask => zext(X))
// =============================================================================

#[test]
fn andzext_sext_with_full_low_mask_becomes_zext() {
    // sext(X:2) & 0xffff  =>  zext(X)
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 2);
    let (sext_out, _s) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_SEXT, &[x]);
    let mask = mk_const(&mut fd, 4, 0xffff); // calc_mask(2)
    let (_anded, andop) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_AND, &[sext_out, mask]);

    assert_eq!(RuleAndZext.apply_op(andop, &mut fd), 1);
    assert_eq!(code_of(&fd, andop), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(andop).unwrap().num_input(), 1);
    assert_eq!(in_of(&fd, andop, 0), Some(x)); // rootvn becomes the sole input
}

#[test]
fn andzext_declines_wrong_mask() {
    // Mask not equal to calc_mask(rootvn size) -> decline.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 2);
    let (sext_out, _s) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_SEXT, &[x]);
    let mask = mk_const(&mut fd, 4, 0x7fff); // != 0xffff
    let (_anded, andop) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_AND, &[sext_out, mask]);
    assert_eq!(RuleAndZext.apply_op(andop, &mut fd), 0);
    assert_eq!(code_of(&fd, andop), OpCode::CPUI_INT_AND);
}

// =============================================================================
// RuleLessEqual  (V<W || V==W => V<=W ; V<W || V!=W => copy(V!=W))
// =============================================================================

#[test]
fn lessequal_less_or_equal_to_lessequal() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let (less_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_INT_LESS, &[v, w]);
    let (eq_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_INT_EQUAL, &[v, w]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, less_out, 0).unwrap();
    fd.op_set_input(or, eq_out, 1).unwrap();
    fd.op_insert(or, bl, None);

    assert_eq!(RuleLessEqual.apply_op(or, &mut fd), 1);
    assert_eq!(code_of(&fd, or), OpCode::CPUI_INT_LESSEQUAL);
    assert_eq!(in_of(&fd, or, 0), Some(v));
    assert_eq!(in_of(&fd, or, 1), Some(w));
}

#[test]
fn lessequal_less_or_notequal_is_copy_of_notequal() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let (less_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_INT_LESS, &[v, w]);
    let (ne_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_INT_NOTEQUAL, &[v, w]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, less_out, 0).unwrap();
    fd.op_set_input(or, ne_out, 1).unwrap();
    fd.op_insert(or, bl, None);

    assert_eq!(RuleLessEqual.apply_op(or, &mut fd), 1);
    assert_eq!(code_of(&fd, or), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(or).unwrap().num_input(), 1);
    assert_eq!(in_of(&fd, or, 0), Some(ne_out)); // takes the NOTEQUAL output
}

#[test]
fn lessequal_declines_when_comparands_differ() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let u = mk_reg(&mut fd, 0x110, 4);
    let (less_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_INT_LESS, &[v, w]);
    let (eq_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_INT_EQUAL, &[v, u]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, less_out, 0).unwrap();
    fd.op_set_input(or, eq_out, 1).unwrap();
    fd.op_insert(or, bl, None);
    assert_eq!(RuleLessEqual.apply_op(or, &mut fd), 0);
}

// =============================================================================
// RuleLessNotEqual  (V<=W && V!=W => V<W)
// =============================================================================

#[test]
fn lessnotequal_combines_to_less() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let (le_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_INT_LESSEQUAL, &[v, w]);
    let (ne_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_INT_NOTEQUAL, &[v, w]);
    let and = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_AND);
    fd.op_set_input(and, le_out, 0).unwrap();
    fd.op_set_input(and, ne_out, 1).unwrap();
    fd.op_insert(and, bl, None);

    assert_eq!(RuleLessNotEqual.apply_op(and, &mut fd), 1);
    assert_eq!(code_of(&fd, and), OpCode::CPUI_INT_LESS);
    assert_eq!(in_of(&fd, and, 0), Some(v));
    assert_eq!(in_of(&fd, and, 1), Some(w));
}

#[test]
fn lessnotequal_signed_variant() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let (le_out, _l) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_INT_SLESSEQUAL, &[v, w]);
    let (ne_out, _e) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_INT_NOTEQUAL, &[v, w]);
    let and = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_AND);
    fd.op_set_input(and, le_out, 0).unwrap();
    fd.op_set_input(and, ne_out, 1).unwrap();
    fd.op_insert(and, bl, None);

    assert_eq!(RuleLessNotEqual.apply_op(and, &mut fd), 1);
    assert_eq!(code_of(&fd, and), OpCode::CPUI_INT_SLESS);
}

// =============================================================================
// RuleZextEliminate  (zext(V) == c => V == c)
// =============================================================================

#[test]
fn zexteliminate_drops_extension_in_equal() {
    // zext(X:2) == 5  =>  X == 5  (5 fits in 2 bytes)
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 2);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 4, 5);
    let eq = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(eq, zext_out, 0).unwrap();
    fd.op_set_input(eq, c, 1).unwrap();
    fd.op_insert(eq, bl, None);

    assert_eq!(RuleZextEliminate.apply_op(eq, &mut fd), 1);
    assert_eq!(code_of(&fd, eq), OpCode::CPUI_INT_EQUAL); // opcode unchanged
    assert_eq!(in_of(&fd, eq, 0), Some(x)); // zext input promoted
    let nc = in_of(&fd, eq, 1).unwrap();
    assert!(is_const(&fd, nc));
    assert_eq!(const_val(&fd, nc), 5);
    assert_eq!(fd.vbank().get(nc).unwrap().get_size(), 2); // resized to smallsize
}

#[test]
fn zexteliminate_declines_when_constant_too_large() {
    // zext(X:2) == 0x1_0000: constant does not fit in 2 bytes -> decline.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 2);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 4, 0x1_0000);
    let eq = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(eq, zext_out, 0).unwrap();
    fd.op_set_input(eq, c, 1).unwrap();
    fd.op_insert(eq, bl, None);
    assert_eq!(RuleZextEliminate.apply_op(eq, &mut fd), 0);
}

// =============================================================================
// RuleConcatShift  (concat(V,W) >> c => zext(V) when c == size(W))
// =============================================================================

#[test]
fn concatshift_exact_cancellation_to_zext() {
    // concat(V:2, W:2) >> 16  =>  zext(V)  (sa == leastsize == 16)
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let hi = mk_reg(&mut fd, 0x100, 2);
    let lo = mk_reg(&mut fd, 0x104, 2);
    let (cat_out, _c) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_PIECE, &[hi, lo]);
    let sa = mk_const(&mut fd, 4, 16);
    let (_shifted, shop) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_RIGHT, &[cat_out, sa]);

    assert_eq!(RuleConcatShift.apply_op(shop, &mut fd), 1);
    assert_eq!(code_of(&fd, shop), OpCode::CPUI_INT_ZEXT);
    assert_eq!(fd.obank().get(shop).unwrap().num_input(), 1);
    assert_eq!(in_of(&fd, shop, 0), Some(hi)); // most significant part extended
}

#[test]
fn concatshift_partial_inserts_extension_op() {
    // concat(V:2, W:2) >> 24  =>  zext(V) << 8  (sa=24, leastsize=16, sa'=8)
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let hi = mk_reg(&mut fd, 0x100, 2);
    let lo = mk_reg(&mut fd, 0x104, 2);
    let (cat_out, _c) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_PIECE, &[hi, lo]);
    let sa = mk_const(&mut fd, 4, 24);
    let (_shifted, shop) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_RIGHT, &[cat_out, sa]);

    assert_eq!(RuleConcatShift.apply_op(shop, &mut fd), 1);
    // Op stays an INT_RIGHT; slot 0 now reads a freshly created ZEXT output,
    // slot 1 is the reduced shift amount 8.
    assert_eq!(code_of(&fd, shop), OpCode::CPUI_INT_RIGHT);
    let newin = in_of(&fd, shop, 0).unwrap();
    let extdef = fd.vbank().get(newin).unwrap().get_def().unwrap();
    assert_eq!(code_of(&fd, extdef), OpCode::CPUI_INT_ZEXT);
    assert_eq!(in_of(&fd, extdef, 0), Some(hi));
    let newsa = in_of(&fd, shop, 1).unwrap();
    assert_eq!(const_val(&fd, newsa), 8);
}

#[test]
fn concatshift_declines_shift_within_low_part() {
    // sa (8) < leastsize (16): does not throw away the whole least-sig part.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let hi = mk_reg(&mut fd, 0x100, 2);
    let lo = mk_reg(&mut fd, 0x104, 2);
    let (cat_out, _c) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_PIECE, &[hi, lo]);
    let sa = mk_const(&mut fd, 4, 8);
    let (_shifted, shop) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_RIGHT, &[cat_out, sa]);
    assert_eq!(RuleConcatShift.apply_op(shop, &mut fd), 0);
    assert_eq!(code_of(&fd, shop), OpCode::CPUI_INT_RIGHT);
}

// =============================================================================
// RuleZextSless  (zext(V) s< c => V < c)
// =============================================================================

#[test]
fn zextsless_drops_extension() {
    // zext(X:2) s< 5  =>  X < 5  (sign bit of the 2-byte value clear in 5)
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 2);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 4, 5);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_SLESS);
    fd.op_set_input(op, zext_out, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    fd.op_insert(op, bl, None);

    assert_eq!(RuleZextSless.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
    assert_eq!(in_of(&fd, op, 0), Some(x));
    let nc = in_of(&fd, op, 1).unwrap();
    assert_eq!(fd.vbank().get(nc).unwrap().get_size(), 2);
}

#[test]
fn zextsless_declines_when_signbit_of_small_set() {
    // c=0x8000: bit (8*2-1)=15 set -> sign bit of small value could be set.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 2);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 4, 0x8000);
    let op = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_SLESS);
    fd.op_set_input(op, zext_out, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    fd.op_insert(op, bl, None);
    assert_eq!(RuleZextSless.apply_op(op, &mut fd), 0);
}

// =============================================================================
// RuleDoubleArithShift  ((x s>> c) s>> d => x s>> (c+d), saturating)
// =============================================================================

#[test]
fn doublearithshift_adds_amounts() {
    // (x s>> 2) s>> 3  =>  x s>> 5
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 4);
    let c = mk_const(&mut fd, 4, 2);
    let (inner_out, _i) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_SRIGHT, &[x, c]);
    let d = mk_const(&mut fd, 4, 3);
    let (_o, outer) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_SRIGHT, &[inner_out, d]);

    assert_eq!(RuleDoubleArithShift.apply_op(outer, &mut fd), 1);
    assert_eq!(code_of(&fd, outer), OpCode::CPUI_INT_SRIGHT);
    assert_eq!(in_of(&fd, outer, 0), Some(x));
    let sa = in_of(&fd, outer, 1).unwrap();
    assert_eq!(const_val(&fd, sa), 5);
}

#[test]
fn doublearithshift_saturates() {
    // (x s>> 20) s>> 20 on a 4-byte (max shift 31): 40 saturates to 31.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_reg(&mut fd, 0x100, 4);
    let c = mk_const(&mut fd, 4, 20);
    let (inner_out, _i) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_SRIGHT, &[x, c]);
    let d = mk_const(&mut fd, 4, 20);
    let (_o, outer) = mk_written(&mut fd, bl, 0x20, 4, OpCode::CPUI_INT_SRIGHT, &[inner_out, d]);

    assert_eq!(RuleDoubleArithShift.apply_op(outer, &mut fd), 1);
    let sa = in_of(&fd, outer, 1).unwrap();
    assert_eq!(const_val(&fd, sa), 31); // saturated to size*8 - 1
}

// =============================================================================
// RuleBooleanUndistribute  (seam: BooleanMatch only handles the trivial case)
// =============================================================================

#[test]
fn booleanundistribute_matches_shared_clause() {
    // (A && B) != (A && C)  =>  A && (B != C)   [A == A is the trivial match]
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_reg(&mut fd, 0x100, 1);
    let b = mk_reg(&mut fd, 0x104, 1);
    let c = mk_reg(&mut fd, 0x108, 1);
    let (and0_out, _a0) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_BOOL_AND, &[a, b]);
    let (and1_out, _a1) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_BOOL_AND, &[a, c]);
    let ne = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(ne, and0_out, 0).unwrap();
    fd.op_set_input(ne, and1_out, 1).unwrap();
    fd.op_insert(ne, bl, None);

    assert_eq!(RuleBooleanUndistribute.apply_op(ne, &mut fd), 1);
    // central_equal starts false (NOTEQUAL) and neither clause is BOOL_OR, so
    // combine = BOOL_AND, inner = NOTEQUAL(B, C).  Slot 0 = A, slot 1 = inner.
    assert_eq!(code_of(&fd, ne), OpCode::CPUI_BOOL_AND);
    assert_eq!(in_of(&fd, ne, 0), Some(a)); // shared clause A (finalA)
    let inner = in_of(&fd, ne, 1).unwrap();
    let innerdef = fd.vbank().get(inner).unwrap().get_def().unwrap();
    assert_eq!(code_of(&fd, innerdef), OpCode::CPUI_INT_NOTEQUAL);
    // inner compares the two non-shared clauses B and C.
    let i0 = in_of(&fd, innerdef, 0).unwrap();
    let i1 = in_of(&fd, innerdef, 1).unwrap();
    assert!((i0 == b && i1 == c) || (i0 == c && i1 == b));
}

#[test]
fn booleanundistribute_declines_without_shared_clause() {
    // (A && B) != (C && D): no clause matches under the trivial seam -> decline.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_reg(&mut fd, 0x100, 1);
    let b = mk_reg(&mut fd, 0x104, 1);
    let c = mk_reg(&mut fd, 0x108, 1);
    let d = mk_reg(&mut fd, 0x10c, 1);
    let (and0_out, _a0) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_BOOL_AND, &[a, b]);
    let (and1_out, _a1) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_BOOL_AND, &[c, d]);
    let ne = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(ne, and0_out, 0).unwrap();
    fd.op_set_input(ne, and1_out, 1).unwrap();
    fd.op_insert(ne, bl, None);
    assert_eq!(RuleBooleanUndistribute.apply_op(ne, &mut fd), 0);
}

// =============================================================================
// specs() registration
// =============================================================================

#[test]
fn specs_are_in_cpp_definition_order() {
    let specs = specs();
    let groups: Vec<&str> = specs.iter().map(|s| s.group).collect();
    assert_eq!(
        groups,
        vec![
            "floatrange",
            "andcommute",
            "andpiece",
            "andzext",
            "andcompare",
            "doublesub",
            "doubleshift",
            "doublearithshift",
            "concatshift",
            "leftright",
            "shiftcompare",
            "lessequal",
            "lessnotequal",
            // RuleTrivialArith's C++ constructor group is "analysis"
            // (coreaction.cc:5786), not the rule name -- corrected so the rule
            // survives the real "decompile"/"jumptable" grouplists.
            "analysis",
            "trivialbool",
            "zexteliminate",
            "slesstoless",
            "zextsless",
            "bitundistribute",
            "booleanundistribute",
        ]
    );
}

#[test]
fn clone_rule_filters_by_group() {
    // Each spec's ctor produces a rule that clones iff its group is present.
    for spec in specs() {
        let rule = (spec.ctor)();
        let yes = ActionGroupList::from_names([spec.group]);
        let no = ActionGroupList::from_names(["some_other_group"]);
        assert!(rule.clone_rule(&yes).is_some(), "{} should clone in its group", spec.group);
        assert!(rule.clone_rule(&no).is_none(), "{} should not clone out of group", spec.group);
    }
}

#[test]
fn trivialarith_survives_the_real_grouplists_via_its_analysis_group() {
    // Regression pin for the `clone_rule` group-gating fix.  C++
    // `RuleTrivialArith::clone` gates on `getGroup()` == "analysis"
    // (coreaction.cc:5786 / ruleaction.hh:493).  The previous Rust port gated on
    // the literal rule NAME "trivialarith", which is NEVER in any root
    // grouplist, so the rule was silently dropped from the active pipeline (the
    // self-`INT_AND`/`INT_XOR` folds never ran -- the loaded byte in a `*ptr`
    // signed-compare cluster could not type `char`, and `Promotion on compare
    // #2` regressed).  The fix gates on the group, so the rule must:
    //   (a) survive the "decompile" grouplist (lists "analysis"),
    //   (b) survive the "jumptable" grouplist (lists "analysis"),
    //   (c) NOT be clonable by the bare rule name (the old, wrong gate).
    let rule = RuleTrivialArith;
    let decompile = ActionGroupList::from_names([
        "base", "analysis", "canonicalcompare", "typerecovery", "deadcode",
    ]);
    let jumptable = ActionGroupList::from_names(["base", "analysis", "deadcode"]);
    let by_name = ActionGroupList::from_names(["trivialarith"]);
    assert!(
        rule.clone_rule(&decompile).is_some(),
        "trivialarith must survive the decompile grouplist (it lists \"analysis\")"
    );
    assert!(
        rule.clone_rule(&jumptable).is_some(),
        "trivialarith must survive the jumptable grouplist (it lists \"analysis\")"
    );
    assert!(
        rule.clone_rule(&by_name).is_none(),
        "the bare rule NAME is not a group -- it must NOT clone the rule (the old bug)"
    );
}

/// Durability: `op_typeop` is the seam `RuleBitUndistribute` & co. retype ops
/// through — it must answer for *every* op-code.  The whitelist it replaced was
/// missing `CPUI_INT_SRIGHT`, so a shift-right undistribute panicked the whole
/// function away (no output at all for e.g. coreutils timeout/split `main`).
#[test]
fn op_typeop_answers_for_every_opcode() {
    for raw in 1..(OpCode::CPUI_MAX as i32) {
        let opc = match OpCode::from_i32(raw) {
            Some(o) => o,
            None => continue,
        };
        let t = op_typeop(opc);
        assert_eq!(t.get_opcode(), opc);
        assert_ne!(t.get_flags(), 0, "{opc:?}: seam produced a property-less TypeOp");
    }
    assert_eq!(op_typeop(OpCode::CPUI_INT_SRIGHT).get_flags(), crate::op::pcodeop_flags::binary);
}

#[test]
fn op_typeop_flags_match_typeop_cc() {
    // Spot-check the seam flag words against typeop.cc.
    use crate::op::pcodeop_flags as f;
    assert_eq!(op_typeop(OpCode::CPUI_COPY).get_flags(), f::unary | f::nocollapse);
    assert_eq!(op_typeop(OpCode::CPUI_INT_AND).get_flags(), f::binary | f::commutative);
    assert_eq!(
        op_typeop(OpCode::CPUI_INT_EQUAL).get_flags(),
        f::binary | f::booloutput | f::commutative
    );
    assert_eq!(op_typeop(OpCode::CPUI_INT_LESS).get_flags(), f::binary | f::booloutput);
    assert_eq!(op_typeop(OpCode::CPUI_INT_ZEXT).get_flags(), f::unary);
    assert_eq!(op_typeop(OpCode::CPUI_PIECE).get_flags(), f::binary);
    assert_eq!(op_typeop(OpCode::CPUI_BOOL_NEGATE).get_flags(), f::unary | f::booloutput);
}
