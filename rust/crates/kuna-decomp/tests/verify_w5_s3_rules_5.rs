//! Adversarial verifier tests for item `w5-s3-rules-5`
//! (`decompiler/cpp/ruleaction.cc` lines 5527-6930, RuleBoolNegate..RulePushPtr).
//!
//! These target the spots the hunt list flagged most fragile for this item:
//!
//!   * **RuleBoolNegate flip-opcode flag fidelity (F1).** `RuleBoolNegate`
//!     routes through `get_booleanflip`, which returns *eight* op-codes besides
//!     {COPY, INT_EQUAL, INT_NOTEQUAL, BOOL_NEGATE} (INT_SLESS<->INT_SLESSEQUAL,
//!     INT_LESS<->INT_LESSEQUAL, FLOAT_EQUAL/NOTEQUAL/LESS/LESSEQUAL).  The
//!     port's `type_op_seam` only knows the four; every other flip hits the
//!     `other => TypeOp::new(other, 0, ..)` fallback (flags = 0), so the flipped
//!     op loses its C++ `opflags` (`binary | booloutput`).  C++
//!     `opSetOpcode(flip_op, opc)` resolves `glb->inst[opc]`, which carries the
//!     correct opflags (typeop.cc).  After `setOpcode` (op.cc:299) clears the
//!     opflags mask and ORs `t_op->getFlags()`, the Rust op ends with
//!     `booloutput` cleared where C++ keeps it set.
//!
//!   * **RuleEqual2Zero constant-form wrap boundary.** `uintb_negate(off-1)`
//!     with `off == 0` underflows (`0 - 1` wraps to all-ones, size-masked).
//!
//!   * **RuleEqual2Constant INT_MULT(-1) negation.** `(-cvn_off) & mask`.
//!
//! C++ oracle: `decompiler/cpp/ruleaction.cc` (RuleBoolNegate 5533-5559,
//! RuleEqual2Zero 5872-5928, RuleEqual2Constant 5944-5994),
//! `decompiler/cpp/opcodes.cc` (`get_booleanflip` 94-135),
//! `decompiler/cpp/typeop.cc` (TypeOp opflags), `decompiler/cpp/op.cc`
//! (`PcodeOp::setOpcode` 299-308).

use std::rc::Rc;

use kuna_base::address::{calc_mask, uintb_negate, Address};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::ruleaction_5::{RuleBoolNegate, RuleEqual2Constant, RuleEqual2Zero};
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::Rule;

// --- harness (mirrors ruleaction_5/tests.rs) ---------------------------------

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

/// Real `typeop.cc` opflags for a def op (so the fixture op carries the flags
/// `glb->inst[opc]` would, before any rule rewrites it).
fn opcode_flags(opc: OpCode) -> u32 {
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
        OpCode::CPUI_INT_NEGATE => pcodeop_flags::unary,
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(s, Address::new(ram, off), dt(s))
}

fn mk_input(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(s, Address::new(ram, off), dt(s));
    fd.vbank_mut().set_input(vn, &mut |_b, _a, _c| Ok(())).expect("set_input")
}

fn mk_const(fd: &mut Funcdata, val: u64, s: int4) -> VarnodeId {
    fd.new_constant(s, val)
}

fn set_def(fd: &mut Funcdata, vn: VarnodeId, op: OpId) {
    let seqnum = fd.obank().get(op).unwrap().get_seq_num().clone();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    let info = DefOpInfo { id: op, seqnum };
    fd.vbank_mut().set_def(vn, info, &mut |_b, _a, _c| Ok(())).expect("set_def");
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

fn in_of(fd: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    fd.obank().get(op).unwrap().get_in(slot).unwrap()
}

// =============================================================================
// F1 — RuleBoolNegate flip-opcode flag fidelity
// =============================================================================

/// **THE BUG (F1).** Negate an `INT_SLESS`: `get_booleanflip(INT_SLESS)` returns
/// `INT_SLESSEQUAL` with `reorder = true`.  C++ sets the def op-code via
/// `glb->inst[CPUI_INT_SLESSEQUAL]`, whose opflags are `binary | booloutput`
/// (typeop.cc).  After `setOpcode`, the def op is STILL a boolean output.
///
/// The port's `type_op_seam(CPUI_INT_SLESSEQUAL)` hits the `other =>` fallback
/// (flags = 0), so the def op ends with `booloutput` cleared.  This asserts the
/// C++ contract (`is_bool_output()` after the flip); under the current port it
/// FAILS — the divergence trace backing the finding.
#[test]
fn boolnegate_sless_flip_keeps_booloutput_flag() {
    let mut fd = build_fd();
    // flip_op: INT_SLESS(a,b) -> vn ; op = BOOL_NEGATE(vn).
    let flip = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SLESS);
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x20, 4);
    fd.op_set_input(flip, a, 0).unwrap();
    fd.op_set_input(flip, b, 1).unwrap();
    let vn = mk_vn(&mut fd, 0x30, 1);
    set_def(&mut fd, vn, flip);

    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();

    let mut rule = RuleBoolNegate::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    // get_booleanflip: INT_SLESS -> INT_SLESSEQUAL, reorder = true.
    assert_eq!(code_of(&fd, flip), OpCode::CPUI_INT_SLESSEQUAL);
    // reorder swapped the inputs (a<->b).
    assert_eq!(in_of(&fd, flip, 0), b);
    assert_eq!(in_of(&fd, flip, 1), a);
    // negate collapses to COPY.
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);

    // C++ ORACLE: glb->inst[CPUI_INT_SLESSEQUAL] has opflags binary|booloutput.
    // The flipped INT_SLESSEQUAL op MUST still report as a boolean output.
    let flipped = fd.obank().get(flip).unwrap();
    assert!(
        flipped.is_bool_output(),
        "flipped INT_SLESSEQUAL lost its booloutput flag: flags={:#x} (type_op_seam fallback set flags=0)",
        flipped.get_flags()
    );
    // And it must retain its binary flag.
    assert_ne!(
        flipped.get_flags() & pcodeop_flags::binary,
        0,
        "flipped INT_SLESSEQUAL lost its binary flag"
    );
}

/// Companion: an `INT_LESS` flip -> `INT_LESSEQUAL` is the same fallback hole
/// (unsigned form).  Asserts the same C++ flag contract.
#[test]
fn boolnegate_less_flip_keeps_booloutput_flag() {
    let mut fd = build_fd();
    let flip = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x20, 4);
    fd.op_set_input(flip, a, 0).unwrap();
    fd.op_set_input(flip, b, 1).unwrap();
    let vn = mk_vn(&mut fd, 0x30, 1);
    set_def(&mut fd, vn, flip);
    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();

    let mut rule = RuleBoolNegate::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, flip), OpCode::CPUI_INT_LESSEQUAL);

    let flipped = fd.obank().get(flip).unwrap();
    assert!(
        flipped.is_bool_output(),
        "flipped INT_LESSEQUAL lost its booloutput flag: flags={:#x}",
        flipped.get_flags()
    );
}

/// Control: the `INT_EQUAL -> INT_NOTEQUAL` flip *is* in `type_op_seam`'s known
/// set, so the flipped op keeps `booloutput`.  This passes today and guards
/// against a regression that would also break the handled case.
#[test]
fn boolnegate_equal_flip_keeps_booloutput_flag_control() {
    let mut fd = build_fd();
    let flip = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x20, 4);
    fd.op_set_input(flip, a, 0).unwrap();
    fd.op_set_input(flip, b, 1).unwrap();
    let vn = mk_vn(&mut fd, 0x30, 1);
    set_def(&mut fd, vn, flip);
    let op = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BOOL_NEGATE);
    fd.op_set_input(op, vn, 0).unwrap();

    let mut rule = RuleBoolNegate::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, flip), OpCode::CPUI_INT_NOTEQUAL);
    assert!(fd.obank().get(flip).unwrap().is_bool_output());
}

// =============================================================================
// RuleEqual2Zero — constant-form wrap boundary
// =============================================================================

/// `0 == V + c  =>  V == -c`.  The C++ computes
/// `uintb_negate(vn2->getOffset()-1, vn2->getSize())`; for `c == 0` the inner
/// `0 - 1` underflows to all-ones then `uintb_negate` produces `~(mask) & mask`
/// path — net effect `-c` size-masked, i.e. `0`.  Verify the new input-1
/// constant equals `uintb_negate(c-1, size)` exactly at the underflow boundary.
#[test]
fn equal2zero_const_form_wrap_boundary_c_is_zero() {
    let mut fd = build_fd();
    // op: INT_EQUAL(const 0, addvn) where addvn = INT_ADD(posvn, const c).
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let zero = mk_const(&mut fd, 0, 4);

    let addop = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_ADD);
    let posvn = mk_input(&mut fd, 0x10, 4);
    let c = 0u64; // boundary: offset - 1 underflows
    let cvn = mk_const(&mut fd, c, 4);
    fd.op_set_input(addop, posvn, 0).unwrap();
    fd.op_set_input(addop, cvn, 1).unwrap();
    let addvn = mk_vn(&mut fd, 0x40, 4);
    set_def(&mut fd, addvn, addop);

    fd.op_set_input(op, zero, 0).unwrap();
    fd.op_set_input(op, addvn, 1).unwrap();

    let mut rule = RuleEqual2Zero::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    // input0 := posvn, input1 := the new -c constant.
    assert_eq!(in_of(&fd, op, 0), posvn);
    let new_c = in_of(&fd, op, 1);
    let expected = uintb_negate(c.wrapping_sub(1), 4);
    assert_eq!(
        fd.vbank().get(new_c).unwrap().get_offset(),
        expected,
        "RuleEqual2Zero const-form: -c mismatch at c=0 underflow boundary"
    );
    // Sanity: for c=0, -c size-masked is 0.
    assert_eq!(fd.vbank().get(new_c).unwrap().get_offset(), 0);
}

// =============================================================================
// RuleEqual2Constant — INT_MULT(-1) negation
// =============================================================================

/// `V * -1 == c  =>  V == -c`.  The transform requires the multiplier to equal
/// `calc_mask(size)` (i.e. all-ones = -1); the new constant is
/// `(-cvn_off) & calc_mask(otherconst->getSize())`.  Use a non-trivial `c` so a
/// sign/width slip would show.
#[test]
fn equal2constant_mult_minus1_negates_constant() {
    let mut fd = build_fd();
    // lhs = INT_MULT(a, const -1) ; op = INT_EQUAL(lhs, const c).
    let leftop = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_MULT);
    let a = mk_input(&mut fd, 0x10, 4);
    let minus1 = mk_const(&mut fd, calc_mask(4), 4); // -1 for size 4
    fd.op_set_input(leftop, a, 0).unwrap();
    fd.op_set_input(leftop, minus1, 1).unwrap();
    let lhs = mk_vn(&mut fd, 0x40, 4);
    set_def(&mut fd, lhs, leftop);

    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let cval = 0x12345678u64;
    let cvn = mk_const(&mut fd, cval, 4);
    fd.op_set_input(op, lhs, 0).unwrap();
    fd.op_set_input(op, cvn, 1).unwrap();

    let mut rule = RuleEqual2Constant::new("analysis");
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    assert_eq!(in_of(&fd, op, 0), a);
    let new_c = in_of(&fd, op, 1);
    // (-c) & calc_mask(size_of_minus1 = 4)
    let expected = cval.wrapping_neg() & calc_mask(4);
    assert_eq!(
        fd.vbank().get(new_c).unwrap().get_offset(),
        expected,
        "RuleEqual2Constant INT_MULT(-1): -c mismatch"
    );
    assert_eq!(expected, 0xEDCBA988); // -0x12345678 mod 2^32
}
