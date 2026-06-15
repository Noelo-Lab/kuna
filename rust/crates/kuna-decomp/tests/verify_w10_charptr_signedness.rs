//! Adversarial verification tests for the `char/char*` follow-up of item
//! `w10-highvar-naming` (commit `rport/w10-highvar-naming: faithful
//! INT_SLESS/SLESSEQUAL propagateType gate`).
//!
//! Written by the INDEPENDENT verifier (round 1).  The follow-up split the
//! comparison `propagateType` dispatch so that `CPUI_INT_SLESS` /
//! `CPUI_INT_SLESSEQUAL` take the C++ STRICTER override
//! (`TypeOpIntSless::propagateType`, typeop.cc:1035-1041 / 1061-1067):
//!
//! ```text
//!   if ((inslot==-1)||(outslot==-1)) return (Datatype *)0; // input <-> input only
//!   if (alttype->getMetatype() != TYPE_INT) return (Datatype *)0; // signed only
//!   return alttype;
//! ```
//!
//! while `CPUI_INT_EQUAL/NOTEQUAL/LESS/LESSEQUAL` keep `propagateAcrossCompare`
//! (typeop.cc:947/1011/1087/1111).  These tests pin the THREE fragile edges the
//! hunt list flagged for this arm:
//!
//!   1. SIGNEDNESS gate — a `char` (a `TypeChar`, which is `TypeBase(1,TYPE_INT)`
//!      per dtype.rs:4430) propagates across a SIGNED compare; a `uint1`
//!      (`TYPE_UINT`) and a pointer (`TYPE_PTR`) do NOT.  This is the entire
//!      char-vs-char* signedness hint.
//!   2. input<->input ONLY — `inslot == -1` (output side) or `outslot == -1`
//!      declines, so a signed type never flows out across the boolean result.
//!   3. NO `propagateAcrossCompare` aliasing — the SIGNED arm must NOT carry the
//!      generic compare arm's spacebase / mid-struct-relptr behaviour; the
//!      contrast is that the UNSIGNED `INT_LESS` arm DOES propagate a `uint1`
//!      (via `propagateAcrossCompare`'s identity path), proving the two arms are
//!      genuinely distinct and the split is not a no-op rename.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use kuna_decomp::coreaction_infertypes::propagate_type_pub;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};

// ---------------------------------------------------------------------------
// harness (mirrors verify_w5x_helpers_completion.rs)
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

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode, flags: uint4) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, flags, format!("{opc:?}")));
    op
}

fn mk_input(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(1, Address::new(ram, off), unk(1));
    fd.vbank_mut()
        .set_input(vn, &mut |_: &mut kuna_decomp::varnode::VarnodeBank, _, _| Ok(()))
        .unwrap()
}

/// Build a 2-input boolean compare with both inputs and an output varnode.
/// Returns (op, in0, in1).
fn mk_compare(fd: &mut Funcdata, opc: OpCode) -> (OpId, VarnodeId, VarnodeId) {
    let op = mk_op(fd, 2, 0x100, opc, pcodeop_flags::binary | pcodeop_flags::booloutput);
    let a = mk_input(fd, 0x10);
    let b = mk_input(fd, 0x20);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    let ram = ram_space(fd);
    let out = fd.new_varnode(1, &Address::new(ram, 0x40), Some(unk(1)));
    fd.op_set_output(op, out).unwrap();
    (op, a, b)
}

/// A standalone, populated type factory (the SLESS/SLESSEQUAL arm only inspects
/// `alttype.get_metatype()`, so the data-types can come from any factory —
/// matching the in-module `propagate_type_tests` setup, which builds types from
/// a free `TypeFactoryImpl` rather than the (seam) architecture's).
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    // Register the `char` core type exactly as the real arch init does
    // (architecture.rs:1162: `set_core_type("char", 1, TYPE_INT, true)`), then
    // populate the core caches (`cache_core_types` fills `charcache`) so
    // `get_type_char(1)` yields a faithful `TypeChar` (TYPE_INT, size 1).
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).expect("register char");
    f.cache_core_types().expect("cache core types");
    f
}

/// A real signed 1-byte `char` (TypeChar -> TypeBase(1,TYPE_INT)).
fn make_char(f: &TypeFactoryImpl) -> Rc<Datatype> {
    f.get_type_char(1).expect("char type")
}

// ===========================================================================
// 1. SIGNEDNESS gate: char propagates, uint1 / pointer do not.
// ===========================================================================

/// A `char` (TYPE_INT, size 1) read by an INT_SLESS flows across to the other
/// operand — the keystone of the char/char* signedness hint.  C++
/// `TypeOpIntSless::propagateType` returns `alttype` because
/// `getMetatype()==TYPE_INT`.
#[test]
fn w10_charptr_slesss_propagates_a_signed_char() {
    let mut fd = build_fd();
    let f = factory();
    let (op, a, b) = mk_compare(&mut fd, OpCode::CPUI_INT_SLESS);
    let chr = make_char(&f);
    assert_eq!(chr.get_metatype(), type_metatype::TYPE_INT, "a char must be TYPE_INT");
    // Propagate FROM input slot 0 (a) TO input slot 1 (b).
    let out = propagate_type_pub(&mut fd, Rc::clone(&chr), op, a, b, 0, 1);
    let out = out.expect("a signed char propagates across INT_SLESS");
    assert!(Rc::ptr_eq(&out, &chr), "the SLESS arm returns the incoming type unchanged");
}

/// A `uint1` (TYPE_UINT) read by an INT_SLESS does NOT propagate — "Only
/// propagate signed things".  This is exactly why an UNSIGNED byte stays
/// distinct from the signed `char` flowing the other way.
#[test]
fn w10_charptr_sless_declines_an_unsigned_byte() {
    let mut fd = build_fd();
    let f = factory();
    let (op, a, b) = mk_compare(&mut fd, OpCode::CPUI_INT_SLESS);
    let u1 = f.get_base(1, type_metatype::TYPE_UINT).expect("uint1");
    assert_eq!(u1.get_metatype(), type_metatype::TYPE_UINT);
    let out = propagate_type_pub(&mut fd, u1, op, a, b, 0, 1);
    assert!(out.is_none(), "a uint1 must NOT propagate across a signed compare");
}

/// A pointer (`char *`, TYPE_PTR) read by an INT_SLESS does NOT propagate — the
/// signed-only gate keeps a pointer's type from leaking onto the other operand
/// of a *signed* relational test.
#[test]
fn w10_charptr_sless_declines_a_pointer() {
    let mut fd = build_fd();
    let f = factory();
    let (op, a, b) = mk_compare(&mut fd, OpCode::CPUI_INT_SLESS);
    let chr = make_char(&f);
    let cptr = f.get_type_pointer(8, chr, 1).expect("char *");
    assert_eq!(cptr.get_metatype(), type_metatype::TYPE_PTR);
    let out = propagate_type_pub(&mut fd, cptr, op, a, b, 0, 1);
    assert!(out.is_none(), "a pointer must NOT propagate across a signed compare");
}

/// INT_SLESSEQUAL shares the identical override — a char propagates, a uint1
/// declines.  (typeop.cc:1061-1067 is byte-identical to 1035-1041.)
#[test]
fn w10_charptr_slessequal_matches_sless_exactly() {
    let mut fd = build_fd();
    let f = factory();
    let (op, a, b) = mk_compare(&mut fd, OpCode::CPUI_INT_SLESSEQUAL);
    let chr = make_char(&f);
    let out = propagate_type_pub(&mut fd, Rc::clone(&chr), op, a, b, 0, 1);
    assert!(out.is_some(), "char propagates across INT_SLESSEQUAL");
    let u1 = f.get_base(1, type_metatype::TYPE_UINT).expect("uint1");
    let out_u = propagate_type_pub(&mut fd, u1, op, a, b, 0, 1);
    assert!(out_u.is_none(), "uint1 declines across INT_SLESSEQUAL");
}

// ===========================================================================
// 2. input<->input ONLY: output side declines.
// ===========================================================================

/// Even a `char` does NOT propagate when either slot is the output (-1): the
/// signed compare only relates its two *inputs* (typeop.cc:1038
/// `(inslot==-1)||(outslot==-1) -> 0`).  This guards the boolean result from
/// receiving a signed integer type.
#[test]
fn w10_charptr_sless_declines_when_a_slot_is_the_output() {
    let mut fd = build_fd();
    let f = factory();
    let (op, a, _b) = mk_compare(&mut fd, OpCode::CPUI_INT_SLESS);
    let out = fd.obank().get(op).unwrap().get_out().unwrap();
    let chr = make_char(&f);
    // input(0) -> output(-1): declines.
    assert!(
        propagate_type_pub(&mut fd, Rc::clone(&chr), op, a, out, 0, -1).is_none(),
        "char must not flow from an input to the boolean output"
    );
    // output(-1) -> input(1): declines.
    assert!(
        propagate_type_pub(&mut fd, chr, op, out, a, -1, 0).is_none(),
        "char must not flow from the boolean output to an input"
    );
}

// ===========================================================================
// 3. The split is REAL: the UNSIGNED INT_LESS arm is genuinely different.
// ===========================================================================

/// Contrast test — the SAME `uint1` that the SIGNED arm REJECTS is ACCEPTED by
/// the UNSIGNED `INT_LESS` arm, which still routes through
/// `propagateAcrossCompare` (identity propagation).  If the porter had merged
/// the arms (the pre-split bug) or made SLESS also call
/// `propagateAcrossCompare`, this divergence would vanish; the test proves the
/// two op-codes now behave distinctly, exactly as the C++ dispatch does.
#[test]
fn w10_charptr_unsigned_int_less_still_propagates_a_uint1() {
    let mut fd = build_fd();
    let f = factory();
    let (op, a, b) = mk_compare(&mut fd, OpCode::CPUI_INT_LESS);
    let u1 = f.get_base(1, type_metatype::TYPE_UINT).expect("uint1");
    let out = propagate_type_pub(&mut fd, Rc::clone(&u1), op, a, b, 0, 1);
    let out = out.expect("INT_LESS (unsigned) propagates a uint1 via propagateAcrossCompare");
    assert_eq!(
        out.get_metatype(),
        type_metatype::TYPE_UINT,
        "the generic compare arm passes the type through unchanged"
    );
    // And the SIGNED arm rejects the very same type — the split has teeth.
    let (sop, sa, sb) = mk_compare(&mut fd, OpCode::CPUI_INT_SLESS);
    assert!(
        propagate_type_pub(&mut fd, u1, sop, sa, sb, 0, 1).is_none(),
        "INT_SLESS rejects the uint1 the INT_LESS arm accepts — distinct behaviour"
    );
}
