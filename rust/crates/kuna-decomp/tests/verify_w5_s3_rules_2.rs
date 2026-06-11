//! Verifier adversarial tests for item `w5-s3-rules-2`
//! (ruleaction.cc:1444-2818, 20 simplification rules `RuleFloatRange` ..
//! `RuleBooleanUndistribute`).
//!
//! INDEPENDENT of the porter's in-module `ruleaction_2::tests`. These drive the
//! public `Rule` API and target the hunt-list spots the review flagged as most
//! fragile:
//!
//!  - F1 (BLOCKER): `RuleZextEliminate` (ruleaction_2.rs:1676) evaluates
//!    `val >> (8 * smallsize)` with a *plain* `>>`. When the ZEXT input is 8
//!    bytes (`smallsize == 8`) the shift count is exactly 64, which DEBUG
//!    PANICS in Rust, whereas the C++ relies on x86 shift-masking
//!    (`val >> 64 == val >> 0`) to produce a defined result. Same class of bug
//!    the `w3-ir-op` verifier already flagged a BLOCKER for.
//!  - `RuleFloatRange` slot normalization when the LESS op is in slot 1 AND the
//!    "other" comparand is a *non-constant* (`cvn1 != matchvn` path,
//!    ruleaction.cc:1506) — the porter's in-module tests only cover the
//!    constant-comparand path with the LESS op in slot 0.
//!  - `RuleZextEliminate` zext-in-slot-1 routing (zextslot/otherslot swap).
//!  - `RuleDoubleShift` opposite-direction partial cancel with `diffsa < 0`
//!    (the INT_RIGHT + sign-flip branch, ruleaction.cc:1918-1925).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::Rule;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::ruleaction_2::{
    RuleDoubleShift, RuleFloatRange, RuleZextEliminate,
};
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;

// -----------------------------------------------------------------------------
// Fixture (mirrors ruleaction_2/tests.rs but kept independent in this file)
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
    let glb = Rc::new(Architecture::new(manage));
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

fn tst_typeop(opc: OpCode) -> TypeOp {
    use pcodeop_flags as f;
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

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, tst_typeop(opc));
    op
}

fn mk_reg(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(size, Address::new(ram, off), dt(size));
    let mut no_replace =
        |_b: &mut kuna_decomp::varnode::VarnodeBank, _a: VarnodeId, _c: VarnodeId| Ok(());
    fd.vbank_mut().set_input(vn, &mut no_replace).expect("mk_reg set_input")
}

fn mk_const(fd: &mut Funcdata, size: int4, val: u64) -> VarnodeId {
    fd.new_constant(size, val)
}

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
        |_b: &mut kuna_decomp::varnode::VarnodeBank, _a: VarnodeId, _c: VarnodeId| Ok(());
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
fn vn_size_of(fd: &Funcdata, vn: VarnodeId) -> int4 {
    fd.vbank().get(vn).unwrap().get_size()
}

// =============================================================================
// F1 (BLOCKER): RuleZextEliminate `val >> (8*smallsize)` panics for smallsize==8
//
// C++ ruleaction.cc:2535:  if ((val>>(8*smallsize))==0) ...
// With an 8-byte ZEXT input, 8*smallsize == 64. The C++ shift by 64 is
// platform-masked (x86: `val >> 0 == val`); the Rust plain `>>` debug-panics.
// This test demonstrates the divergence: on a wide (8-byte) ZEXT input feeding
// a comparison, the Rust port PANICS instead of evaluating the guard.
// =============================================================================

#[test]
#[should_panic(expected = "shift")]
fn w5_s3_rules_2_zexteliminate_wide_input_panics_f1() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // ZEXT of an 8-byte value to a 16-byte value, compared (== ) against a
    // 16-byte constant. smallsize == 8  =>  8*smallsize == 64.
    let x = mk_reg(&mut fd, 0x100, 8);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 16, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 16, 0x1234);
    let eq = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(eq, zext_out, 0).unwrap();
    fd.op_set_input(eq, c, 1).unwrap();
    fd.op_insert(eq, bl, None);

    // C++ would compute (val >> 64) == val (x86 masking) and proceed; the Rust
    // plain shift panics here. The #[should_panic] records the confirmed
    // divergence (a debug-mode crash where upstream produces a value).
    let _ = RuleZextEliminate.apply_op(eq, &mut fd);
}

// Companion: the *narrow* (smallsize < 8) path the rule is meant for works and
// produces the C++ result — proving F1 is specifically the wide-input edge.
#[test]
fn w5_s3_rules_2_zexteliminate_narrow_input_ok() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // ZEXT of a 4-byte value to 8 bytes; 8*smallsize == 32, no panic.
    let x = mk_reg(&mut fd, 0x100, 4);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 8, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 8, 7); // fits in 4 bytes
    let eq = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(eq, zext_out, 0).unwrap();
    fd.op_set_input(eq, c, 1).unwrap();
    fd.op_insert(eq, bl, None);

    assert_eq!(RuleZextEliminate.apply_op(eq, &mut fd), 1);
    assert_eq!(in_of(&fd, eq, 0), Some(x)); // extension dropped
    let nc = in_of(&fd, eq, 1).unwrap();
    assert_eq!(const_val(&fd, nc), 7);
    assert_eq!(vn_size_of(&fd, nc), 4); // resized to smallsize
}

// =============================================================================
// RuleZextEliminate: ZEXT in slot 1 (the zextslot/otherslot swap path).
// C++ ruleaction.cc:2520-2525.
// =============================================================================

#[test]
fn w5_s3_rules_2_zexteliminate_zext_in_slot1() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // const(slot0) == zext(X:2)(slot1)  =>  const == X  (operands keep slots)
    let x = mk_reg(&mut fd, 0x100, 2);
    let (zext_out, _z) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_ZEXT, &[x]);
    let c = mk_const(&mut fd, 4, 9);
    let eq = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(eq, c, 0).unwrap(); // const in slot 0
    fd.op_set_input(eq, zext_out, 1).unwrap(); // zext in slot 1
    fd.op_insert(eq, bl, None);

    assert_eq!(RuleZextEliminate.apply_op(eq, &mut fd), 1);
    // zextslot == 1: the ZEXT input X lands in slot 1; the resized const in slot 0.
    assert_eq!(in_of(&fd, eq, 1), Some(x));
    let nc = in_of(&fd, eq, 0).unwrap();
    assert_eq!(const_val(&fd, nc), 9);
    assert_eq!(vn_size_of(&fd, nc), 2);
}

// =============================================================================
// RuleFloatRange: LESS op in slot 1, non-constant shared comparand.
// Exercises the cmp1/cmp2 swap (ruleaction.cc:1465-1469) AND the
// `cvn1 != matchvn` non-constant comparand path (ruleaction.cc:1506-1509),
// neither covered by the porter's constant/slot-0 in-module tests.
// =============================================================================

#[test]
fn w5_s3_rules_2_floatrange_swapped_slots_noncconst_comparand() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // slot0 = (V f== W), slot1 = (V f< W);  V and W both non-constant.
    // BOOL_OR  =>  V f<= W. cmp1 must be swapped to the FLOAT_LESS (slot1) op.
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let (eq_out, _e) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_FLOAT_EQUAL, &[v, w]);
    let (less_out, _l) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_FLOAT_LESS, &[v, w]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, eq_out, 0).unwrap(); // EQUAL in slot 0
    fd.op_set_input(or, less_out, 1).unwrap(); // LESS in slot 1
    fd.op_insert(or, bl, None);

    assert_eq!(RuleFloatRange.apply_op(or, &mut fd), 1);
    assert_eq!(code_of(&fd, or), OpCode::CPUI_FLOAT_LESSEQUAL);
    // nvn1 = V (slot 0 of cmp1=LESS), cvn1 = W (non-constant) goes to slot 1.
    assert_eq!(in_of(&fd, or, 0), Some(v));
    assert_eq!(in_of(&fd, or, 1), Some(w)); // the *same* W varnode, not a copy
}

// Negative: same shape but the shared non-constant comparand differs between
// the two comparisons (V f== W) vs (V f< U) -> `cvn1 != matchvn` => decline.
#[test]
fn w5_s3_rules_2_floatrange_declines_distinct_noncconst_comparand() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let w = mk_reg(&mut fd, 0x108, 4);
    let u = mk_reg(&mut fd, 0x110, 4);
    let (eq_out, _e) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_FLOAT_EQUAL, &[v, w]);
    let (less_out, _l) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_FLOAT_LESS, &[v, u]);
    let or = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_BOOL_OR);
    fd.op_set_input(or, eq_out, 0).unwrap();
    fd.op_set_input(or, less_out, 1).unwrap();
    fd.op_insert(or, bl, None);

    assert_eq!(RuleFloatRange.apply_op(or, &mut fd), 0);
    assert_eq!(code_of(&fd, or), OpCode::CPUI_BOOL_OR);
}

// =============================================================================
// RuleDoubleShift: opposite-direction partial cancel, diffsa < 0 => INT_RIGHT.
// (V << 8) >> 24  on a 4-byte value:
//   opc1 = INT_RIGHT, opc2 = INT_LEFT (opposite). Non-LEFT outer branch:
//   mask = (fullmask >> sa2=8) & fullmask = 0x00ffffff;  diffsa = sa2 - sa1
//   = 8 - 24 = -16 < 0  => finalopc = INT_RIGHT, diffsa = 16.
//   Result: (V & 0x00ffffff) >> 16.
// (ruleaction.cc:1901-1926) — the sign-flip branch the porter's in-module
// tests do not cover.
// =============================================================================

#[test]
fn w5_s3_rules_2_doubleshift_opposite_partial_negative_diffsa() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let s_inner = mk_const(&mut fd, 4, 8); // inner INT_LEFT by 8
    let (left_out, _l) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_LEFT, &[v, s_inner]);
    let s_outer = mk_const(&mut fd, 4, 24); // outer INT_RIGHT by 24
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_RIGHT);
    fd.op_set_input(outer, left_out, 0).unwrap();
    fd.op_set_input(outer, s_outer, 1).unwrap();
    fd.op_insert(outer, bl, None);

    assert_eq!(RuleDoubleShift.apply_op(outer, &mut fd), 1);
    // Outer op becomes INT_RIGHT by diffsa=16, slot0 reads a fresh INT_AND.
    assert_eq!(code_of(&fd, outer), OpCode::CPUI_INT_RIGHT);
    let sa = in_of(&fd, outer, 1).unwrap();
    assert_eq!(const_val(&fd, sa), 16);
    let andin = in_of(&fd, outer, 0).unwrap();
    let anddef = fd.vbank().get(andin).unwrap().get_def().unwrap();
    assert_eq!(code_of(&fd, anddef), OpCode::CPUI_INT_AND);
    assert_eq!(in_of(&fd, anddef, 0), Some(v));
    // mask = (0xffffffff >> 8) & 0xffffffff = 0x00ffffff.
    let maskvn = in_of(&fd, anddef, 1).unwrap();
    assert_eq!(const_val(&fd, maskvn), 0x00ff_ffff);
}
