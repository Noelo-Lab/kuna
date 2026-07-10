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
    RuleBooleanUndistribute, RuleDoubleShift, RuleFloatRange, RuleShiftCompare, RuleZextEliminate,
};
use kuna_decomp::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};
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
// F1 (was BLOCKER, now REPAIRED): RuleZextEliminate `val >> (8*smallsize)` with
// an 8-byte ZEXT input (smallsize==8) has a shift count of 64.
//
// C++ ruleaction.cc:2535:  if ((val>>(8*smallsize))==0) ...
// The C++ shift by 64 is x86-masked (count & 0x3f == 0, so `val >> 64 == val`);
// the repaired port uses `wshr` (ADR-0003) to replicate that masking instead of
// debug-panicking. For a non-zero constant the guard `(val>>64)==val` is
// non-zero, so the rule DECLINES (returns 0) and leaves the op untouched —
// exactly what the C++ oracle does. (Previously a passing `#[should_panic]`
// evidencing the panic divergence; now asserts the masked-oracle result.)
// =============================================================================

#[test]
fn w5_s3_rules_2_zexteliminate_wide_input_masked_f1() {
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

    // (val >> 64) is x86-masked to (val >> 0) == 0x1234 != 0, so the guard
    // `(val>>(8*smallsize))==0` is false and the rule declines without mutating.
    assert_eq!(RuleZextEliminate.apply_op(eq, &mut fd), 0);
    assert_eq!(code_of(&fd, eq), OpCode::CPUI_INT_EQUAL);
    assert_eq!(in_of(&fd, eq, 0), Some(zext_out)); // ZEXT input unchanged
    assert_eq!(in_of(&fd, eq, 1), Some(c)); // constant unchanged
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

// =============================================================================
// ROUND 2 adversarial tests (re-verify the F1 repair is systemic, not just the
// one RuleZextEliminate site).  Round 1 REJECTed on bare `>>`/`<<` value-word
// shifts whose count can reach >= 64; the repair routed every such site through
// `wshl`/`wshr` (ADR-0003).  These exercise the OTHER repaired sites
// (RuleShiftCompare, RuleDoubleShift) plus the RuleBooleanUndistribute
// `BooleanMatch::evaluate` stub (a strictly-conservative loss).
// =============================================================================

// RuleShiftCompare LEFT info-loss branch (ruleaction.cc:2139-2156): `(V << c) == d`
// where the shift discards high bits of V's nz-mask.  The `(nzmask << sa)` and
// `((uintb)1) << (8*size - sa)` shifts are the repaired sites.  Pick a 1-byte V
// (nzmask 0xff) shifted left by 4 inside an INT_EQUAL against 0x30:
//   newconst = 0x30 >> 4 = 3; (3 << 4)=0x30==constval OK.
//   tmp = (0xff << 4) & 0xff = 0xf0; (0xf0 >> 4)=0x0f != 0xff  => info-loss branch.
//   sa becomes 8*1 - 4 = 4; mask = (1<<4)-1 = 0x0f.
//   Result: (V & 0x0f) == 3, with the AND inserted before the shift op.
#[test]
fn w5_s3_rules_2_r2_shiftcompare_left_info_loss_andmask() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 1); // nzmask 0xff
    let sa = mk_const(&mut fd, 1, 4);
    let (shl_out, shlop) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_INT_LEFT, &[v, sa]);
    let d = mk_const(&mut fd, 1, 0x30);
    let eq = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(eq, shl_out, 0).unwrap();
    fd.op_set_input(eq, d, 1).unwrap();
    fd.op_insert(eq, bl, None);

    assert_eq!(RuleShiftCompare.apply_op(eq, &mut fd), 1);
    // op stays INT_EQUAL; slot1 const becomes newconst = 3.
    assert_eq!(code_of(&fd, eq), OpCode::CPUI_INT_EQUAL);
    assert_eq!(const_val(&fd, in_of(&fd, eq, 1).unwrap()), 3);
    // slot0 now reads a freshly inserted INT_AND(V, 0x0f) placed before the shift.
    let andin = in_of(&fd, eq, 0).unwrap();
    let anddef = fd.vbank().get(andin).unwrap().get_def().unwrap();
    assert_eq!(code_of(&fd, anddef), OpCode::CPUI_INT_AND);
    assert_eq!(in_of(&fd, anddef, 0), Some(v));
    assert_eq!(const_val(&fd, in_of(&fd, anddef, 1).unwrap()), 0x0f);
    let _ = shlop;
}

// RuleDoubleShift same-direction OVERSHIFT (ruleaction.cc:1882-1887): `(V << 24) << 16`
// on a 4-byte value.  sa1+sa2 = 40 >= 8*4 = 32  =>  the whole thing zeroes:
// op becomes COPY of constant 0, input 1 removed.  Stresses the `sa1 + sa2 < 8*size`
// signed comparison (no overflow on realistic amounts) and the COPY#0 emit.
#[test]
fn w5_s3_rules_2_r2_doubleshift_same_dir_overshift_to_zero() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_reg(&mut fd, 0x100, 4);
    let s_inner = mk_const(&mut fd, 4, 24);
    let (left_out, _l) = mk_written(&mut fd, bl, 0x10, 4, OpCode::CPUI_INT_LEFT, &[v, s_inner]);
    let s_outer = mk_const(&mut fd, 4, 16);
    let outer = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_LEFT);
    fd.op_set_input(outer, left_out, 0).unwrap();
    fd.op_set_input(outer, s_outer, 1).unwrap();
    fd.op_insert(outer, bl, None);

    assert_eq!(RuleDoubleShift.apply_op(outer, &mut fd), 1);
    assert_eq!(code_of(&fd, outer), OpCode::CPUI_COPY);
    // input 1 was removed; the COPY now has exactly one input (the zero const).
    assert_eq!(fd.obank().get(outer).unwrap().num_input(), 1_i32);
    let z = in_of(&fd, outer, 0).unwrap();
    assert_eq!(const_val(&fd, z), 0);
    assert_eq!(vn_size_of(&fd, z), 4);
}

// RuleBooleanUndistribute stub (BooleanMatch::evaluate reduced to its `vn1==vn2`
// head): `(A && B) != (A && C)` with the *same* varnode A shared verbatim must
// still fire (stub returns `same` for identical varnodes).  centralEqual starts
// false (INT_NOTEQUAL), no OR flips, so combineOpc = BOOL_AND and the inner op is
// INT_NOTEQUAL(B,C).  Result:  A && (B != C).
#[test]
fn w5_s3_rules_2_r2_booleanundistribute_identical_shared_clause_fires() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_reg(&mut fd, 0x100, 1);
    let b = mk_reg(&mut fd, 0x108, 1);
    let c = mk_reg(&mut fd, 0x110, 1);
    let (and0, _op0) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_BOOL_AND, &[a, b]);
    let (and1, _op1) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_BOOL_AND, &[a, c]);
    let ne = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(ne, and0, 0).unwrap();
    fd.op_set_input(ne, and1, 1).unwrap();
    fd.op_insert(ne, bl, None);

    assert_eq!(RuleBooleanUndistribute.apply_op(ne, &mut fd), 1);
    // Outer op becomes BOOL_AND; slot0 = finalA = A (not flipped); slot1 = inner.
    assert_eq!(code_of(&fd, ne), OpCode::CPUI_BOOL_AND);
    assert_eq!(in_of(&fd, ne, 0), Some(a));
    let inner = in_of(&fd, ne, 1).unwrap();
    let innerdef = fd.vbank().get(inner).unwrap().get_def().unwrap();
    assert_eq!(code_of(&fd, innerdef), OpCode::CPUI_INT_NOTEQUAL);
    // inner compares the two non-shared clauses B and C (order: finalB, finalC).
    let i0 = in_of(&fd, innerdef, 0).unwrap();
    let i1 = in_of(&fd, innerdef, 1).unwrap();
    assert!((i0 == b && i1 == c) || (i0 == c && i1 == b));
}

// RuleBooleanUndistribute stub LOSS witness: a structurally-equal-but-distinct
// shared clause (two separate COPY-of-A varnodes) is `uncorrelated` under the
// stub (the recursive BooleanMatch is stubbed), so the rule DECLINES where the
// full C++ `BooleanMatch::evaluate` would match and transform.  Strictly
// conservative — never a wrong rewrite, only a missed one.
#[test]
fn w5_s3_rules_2_r2_booleanundistribute_stub_declines_structural_match() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_reg(&mut fd, 0x100, 1);
    let b = mk_reg(&mut fd, 0x108, 1);
    let c = mk_reg(&mut fd, 0x110, 1);
    // Two distinct COPY(A) varnodes: structurally the same boolean value, but
    // different VarnodeIds, so the stub's `vn1==vn2` head misses them.
    let (acopy0, _a0) = mk_written(&mut fd, bl, 0x08, 1, OpCode::CPUI_COPY, &[a]);
    let (acopy1, _a1) = mk_written(&mut fd, bl, 0x0c, 1, OpCode::CPUI_COPY, &[a]);
    let (and0, _op0) = mk_written(&mut fd, bl, 0x10, 1, OpCode::CPUI_BOOL_AND, &[acopy0, b]);
    let (and1, _op1) = mk_written(&mut fd, bl, 0x20, 1, OpCode::CPUI_BOOL_AND, &[acopy1, c]);
    let ne = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(ne, and0, 0).unwrap();
    fd.op_set_input(ne, and1, 1).unwrap();
    fd.op_insert(ne, bl, None);

    // Stub declines (returns 0); the op graph is left untouched.
    assert_eq!(RuleBooleanUndistribute.apply_op(ne, &mut fd), 0);
    assert_eq!(code_of(&fd, ne), OpCode::CPUI_INT_NOTEQUAL);
    assert_eq!(in_of(&fd, ne, 0), Some(and0));
    assert_eq!(in_of(&fd, ne, 1), Some(and1));
}
