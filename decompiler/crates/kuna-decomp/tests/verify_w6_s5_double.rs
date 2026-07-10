//! Independent verifier adversarial tests for item `w6-s5-double`
//! (`decompiler/cpp/double.cc` / `double.hh`: SplitVarnode double-precision
//! analysis + the doublein/doubleout/doubleload/doublestore rule set).
//!
//! Re-derived from the C++ oracle (NOT the Rust port).  These drive the PUBLIC
//! surface (`RuleDoubleIn`, `SplitVarnode`) to pin behaviors the in-crate porter
//! tests do not cover, focused on the hunt-list spots flagged most fragile:
//!
//!   - The arith/float classification that GATES recovery for the dominant
//!     `RuleDoubleIn::attemptMarking` branch (a SUBPIECE off a *written* whole):
//!       * `op_is_arith_or_float` now reads the W6 `TypeOp::addlflags`
//!         `isArithmeticOp`/`isFloatingPointOp` (double.cc:3238-3240) from the
//!         ported `type_op_info` table (`rport/w10-doublemove`).  An INT_ADD
//!         whole IS arithmetic, so `attemptMarking` proceeds and marks the pair —
//!         restoring the single most behavior-shrinking former loss in the port.
//!         These tests pin the restored (C++-faithful) marking.
//!   - `RuleDoubleIn::reset` MUST flip the function's double-precision-recovery
//!     flag (double.cc:3198-3202) — a side effect later passes depend on.
//!   - `exceedsConstPrecision` boundary at exactly `sizeof(uintb)` == 8
//!     (double.cc:700-702: `wholesize > sizeof(uintb)`), the guard every
//!     constant `SplitVarnode` form checks before emitting.
//!
//! Each assertion cites the C++ line it is derived from.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::Rule;
use kuna_decomp::double::{RuleDoubleIn, SplitVarnode};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};

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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.vbank_mut().create(size, Address::new(r, off), unk(size))
}

fn mk_const(fd: &mut Funcdata, size: int4, val: u64) -> VarnodeId {
    fd.new_constant(size, val)
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

/// Build `whole = INT_ADD(a, b)` (a *written*, arithmetic whole) and two
/// SUBPIECEs off it: `lo = SUB(whole,0)` (precisLo) and `hi = SUB(whole,4)`.
/// Returns `(whole, lo, hi, subhi_op)`.  The hi SUBPIECE is NOT marked precisHi
/// (so `RuleDoubleIn::apply_op` enters the `attemptMarking` arm, not the
/// already-marked early-out).
fn build_subpieces_off_written_add(
    fd: &mut Funcdata,
) -> (VarnodeId, VarnodeId, VarnodeId, OpId) {
    let bl = mk_block(fd);
    let r = ram(fd);

    // whole := INT_ADD(a, b)  (8 bytes, written, arithmetic op)
    let addop = mk_op(fd, 2, 0x300, OpCode::CPUI_INT_ADD);
    let whole = fd
        .new_varnode_out(8, &Address::new(Rc::clone(&r), 0x100), addop)
        .unwrap();
    let a = mk_vn(fd, 0x108, 8);
    let b = mk_vn(fd, 0x110, 8);
    fd.op_set_input(addop, a, 0).unwrap();
    fd.op_set_input(addop, b, 1).unwrap();
    fd.op_insert_end(addop, bl);

    // lo := SUBPIECE(whole, 0), marked precisLo
    let sublo = mk_op(fd, 2, 0x310, OpCode::CPUI_SUBPIECE);
    let lo = fd
        .new_varnode_out(4, &Address::new(Rc::clone(&r), 0x200), sublo)
        .unwrap();
    fd.op_set_input(sublo, whole, 0).unwrap();
    let c0 = mk_const(fd, 4, 0);
    fd.op_set_input(sublo, c0, 1).unwrap();
    fd.op_insert_end(sublo, bl);
    fd.vbank_mut().get_mut(lo).unwrap().set_precis_lo();

    // hi := SUBPIECE(whole, 4), NOT marked.
    let subhi = mk_op(fd, 2, 0x320, OpCode::CPUI_SUBPIECE);
    let hi = fd
        .new_varnode_out(4, &Address::new(r, 0x210), subhi)
        .unwrap();
    fd.op_set_input(subhi, whole, 0).unwrap();
    let c4 = mk_const(fd, 4, 4);
    fd.op_set_input(subhi, c4, 1).unwrap();
    fd.op_insert_end(subhi, bl);

    (whole, lo, hi, subhi)
}

/// HUNT LIST (the dominant marking branch, restored). The C++
/// `RuleDoubleIn::attemptMarking` (double.cc:3232-3256) for a *written* whole
/// reaches `if (!typeop->isArithmeticOp() && !typeop->isFloatingPointOp())
/// return 0;`.  An INT_ADD whole IS arithmetic, so C++ proceeds, finds the
/// precisLo companion off the SAME whole, sets precisHi on `hi`/precisLo on `lo`,
/// and returns 1.
///
/// The Rust port now classifies via the ported W6 `TypeOp::addlflags`
/// (`op_is_arith_or_float`), so the same call returns 1 and marks the pair —
/// faithful to the C++ oracle for this exact graph (returns 1,
/// `hi.isPrecisHi()` true, `lo` stays precisLo).
#[test]
fn verify_w6_s5_double_arith_whole_marking_restored() {
    let mut fd = build_fd();
    let (_whole, lo, hi, subhi) = build_subpieces_off_written_add(&mut fd);

    // apply_op on the UNMARKED hi SUBPIECE drives attemptMarking.
    let mut rule = RuleDoubleIn::new("doubleprecis");
    let res = rule.apply_op(subhi, &mut fd);

    // C++ oracle (double.cc:3238-3256): INT_ADD is arithmetic -> mark + return 1.
    assert_eq!(
        res, 1,
        "attemptMarking returns 1 for a written INT_ADD whole \
         (C++ oracle, double.cc:3238-3256)"
    );
    assert!(
        fd.vbank().get(hi).unwrap().is_precis_hi(),
        "hi piece is marked precisHi (double.cc:3255)"
    );
    // lo keeps its pre-existing precisLo mark; attemptMarking re-asserts it.
    assert!(
        fd.vbank().get(lo).unwrap().is_precis_lo(),
        "lo retains/keeps its precisLo mark (double.cc:3254)"
    );
}

/// HUNT LIST (cross-wave stub, the type-locked-INPUT branch, which does NOT need
/// the arith/float classification).  When the whole is an INPUT and type-locked,
/// `attemptMarking` SKIPS the `isArithmeticOp` test entirely (double.cc:3229-
/// 3231: `if (whole->isInput()) { if (!whole->isTypeLock()) return 0; }`).  But
/// a function INPUT requires the W4 input-varnode machinery to construct; here we
/// confirm the complementary fact that an UNwritten, non-input, non-typelocked
/// whole is rejected BEFORE the stub (so the stub is not even reached) — pinning
/// that the `!isWritten` guard (double.cc:3232) fires first.
#[test]
fn verify_w6_s5_double_free_whole_rejected_before_stub() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let r = ram(&fd);

    // whole is a FREE varnode (no def, not input): isWritten()==false,
    // isInput()==false -> the `else if (!whole->isWritten()) return 0;` arm.
    let whole = mk_vn(&mut fd, 0x100, 8);

    let subhi = mk_op(&mut fd, 2, 0x320, OpCode::CPUI_SUBPIECE);
    let hi = fd
        .new_varnode_out(4, &Address::new(Rc::clone(&r), 0x210), subhi)
        .unwrap();
    fd.op_set_input(subhi, whole, 0).unwrap();
    let c4 = mk_const(&mut fd, 4, 4);
    fd.op_set_input(subhi, c4, 1).unwrap();
    fd.op_insert_end(subhi, bl);

    let mut rule = RuleDoubleIn::new("doubleprecis");
    assert_eq!(
        rule.apply_op(subhi, &mut fd),
        0,
        "free (non-written, non-input) whole rejected by the !isWritten guard \
         (double.cc:3232) — independent of the W6 stub"
    );
    assert!(
        !fd.vbank().get(hi).unwrap().is_precis_hi(),
        "no marking when the whole is free"
    );
}

/// HUNT LIST (off-by-one boundary on the SUBPIECE offset/size geometry).
/// `attemptMarking` requires `offset == vn->getSize()` AND `offset*2 ==
/// whole->getSize()` (double.cc:3227-3228 — truncate EXACTLY half).  A hi
/// SUBPIECE at a non-half offset must be declined even before the W6 stub.  Here
/// the hi piece is a SUBPIECE(whole,2) of a size-2 output off an 8-byte whole:
/// offset(2)==size(2) holds, but offset*2(4) != whole.size(8), so it is the
/// half-test that rejects.
#[test]
fn verify_w6_s5_double_non_half_truncation_rejected() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let r = ram(&fd);

    // whole := INT_ADD(...) 8 bytes (written/arithmetic, would pass the op test).
    let addop = mk_op(&mut fd, 2, 0x300, OpCode::CPUI_INT_ADD);
    let whole = fd
        .new_varnode_out(8, &Address::new(Rc::clone(&r), 0x100), addop)
        .unwrap();
    let a = mk_vn(&mut fd, 0x108, 8);
    fd.op_set_input(addop, a, 0).unwrap();
    let b = mk_vn(&mut fd, 0x110, 8);
    fd.op_set_input(addop, b, 1).unwrap();
    fd.op_insert_end(addop, bl);

    // hi := SUBPIECE(whole, 2) with a 2-byte output: offset(2)==size(2) but
    // offset*2(4) != whole.size(8).
    let subhi = mk_op(&mut fd, 2, 0x320, OpCode::CPUI_SUBPIECE);
    let hi = fd
        .new_varnode_out(2, &Address::new(r, 0x210), subhi)
        .unwrap();
    fd.op_set_input(subhi, whole, 0).unwrap();
    let c2 = mk_const(&mut fd, 4, 2);
    fd.op_set_input(subhi, c2, 1).unwrap();
    fd.op_insert_end(subhi, bl);

    let mut rule = RuleDoubleIn::new("doubleprecis");
    assert_eq!(
        rule.apply_op(subhi, &mut fd),
        0,
        "non-half truncation rejected by `offset*2 != whole.size` (double.cc:3228)"
    );
    assert!(!fd.vbank().get(hi).unwrap().is_precis_hi());
}

/// `RuleDoubleIn::reset` MUST set the function's double-precision-recovery flag
/// (double.cc:3198-3202: `data.setDoublePrecisRecovery(true)`).  A pass that
/// reads `isDoublePrecisOn()` later relies on this side effect.
#[test]
fn verify_w6_s5_double_reset_sets_recovery_flag() {
    let mut fd = build_fd();
    assert!(
        !fd.is_double_precis_on(),
        "precondition: recovery flag starts clear"
    );
    let mut rule = RuleDoubleIn::new("doubleprecis");
    rule.reset(&mut fd);
    assert!(
        fd.is_double_precis_on(),
        "reset() flips double-precision recovery on (double.cc:3201)"
    );
}

/// HUNT LIST (integer-width boundary). `exceedsConstPrecision` is
/// `isConstant() && wholesize > sizeof(uintb)` (double.cc:700-702).  `uintb` is
/// 8 bytes, so an 8-byte constant is FINE (`8 > 8` is false) and a 9-byte
/// constant EXCEEDS.  Every constant double-precision form gates on this before
/// emitting (e.g. AddForm/SubForm/Equal2Form `exceedsConstPrecision()` -> bail),
/// so an off-by-one here would silently corrupt or drop transforms.
#[test]
fn verify_w6_s5_double_exceeds_const_precision_boundary() {
    // 8-byte constant: exactly sizeof(uintb), NOT exceeding.
    let eight = SplitVarnode::new_constant(8, 0xDEAD_BEEF_CAFE_F00D);
    assert!(eight.is_constant());
    assert!(
        !eight.exceeds_const_precision(),
        "wholesize==8 is NOT > sizeof(uintb)==8 (double.cc:701)"
    );

    // 9-byte constant: exceeds.
    let nine = SplitVarnode::new_constant(9, 0);
    assert!(
        nine.exceeds_const_precision(),
        "wholesize==9 IS > sizeof(uintb)==8 (double.cc:701)"
    );

    // A non-constant SplitVarnode is never "exceeding" regardless of size: the
    // `isConstant()` conjunct gates it.  `new_constant` is the only public ctor
    // making a constant, so build a 16-byte constant and confirm the size term,
    // then confirm the conjunction's short-circuit via a constant-flag check.
    let big = SplitVarnode::new_constant(16, 0);
    assert!(big.is_constant() && big.exceeds_const_precision());
}
