//! Round-1 VERIFIER adversarial tests for `rport/w10-bitfield-absorb`.
//!
//! Authored by the independent verifier (per docs/rust-port/verification.md,
//! Adversarial pass).  Unlike the sibling `w10-bitfield-activate` round, the
//! transform IS wired here (`RuleInsertAbsorb`/`RulePullAbsorb::apply_op` now
//! delegate into `bitfield/absorb.rs`), so these drive the *real* rule
//! end-to-end on a hand-built p-code graph and assert both the positive
//! rewrite AND the negative guards (the spots a shortcut/special-case would
//! silently weaken).  Targets the hunt-list-flagged fragilities:
//!
//!  - A1 `RuleInsertAbsorb::absorbAnd` (bitfield.cc:2241-2252): the LSB-mask
//!    gate `(mask & val) == mask` is the WHOLE point — `INSERT(x & #m)` may only
//!    drop the AND when the AND mask covers *every* bit the INSERT keeps.  A
//!    partial mask (one bit short) must NOT absorb, or live bits are corrupted.
//!  - A2 `leftShiftVarnode` (bitfield.cc:2214-2233) via `absorbShiftAdd`: the
//!    INT_MULT arm matches `1<<sa`, NOT an arbitrary constant.  A multiply by a
//!    non-power-of-two (or the wrong power) must NOT be treated as a shift.
//!  - A3 `RuleInsertAbsorb::absorbRightLeft` (bitfield.cc:2257-2286): the
//!    shift-cancel `(x<<c)>>c` is only sound while `bitsize <= preserved bits`;
//!    a field wider than `containerbits - c` must NOT cancel.
//!  - A4 `RuleInsertAbsorb::absorbNestedAnd` (bitfield.cc:2333-2355): the
//!    coveringmask/popcount gate — a NON-contiguous AND mask, or a mask with
//!    `popcount < bitsize`, must NOT be absorbed.
//!  - A5 `RulePullAbsorb::absorbExt` (bitfield.cc:2070-2086): SEXT folds only a
//!    SPULL (signedness must match); a SEXT over a ZPULL must NOT fold.
//!
//! These are verifier-authored and land on the porter branch regardless of
//! verdict.  Marked `w10bfa_*` (the item id).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::action::Rule;
use kuna_decomp::bitfield::{RuleInsertAbsorb, RulePullAbsorb};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;
use kuna_num::opcodes::OpCode;

// --- harness (mirrors verify_w5_s3_rules_5.rs) -------------------------------

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

fn opcode_flags(opc: OpCode) -> u32 {
    match opc {
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESSEQUAL => pcodeop_flags::binary | pcodeop_flags::booloutput,
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR => pcodeop_flags::binary | pcodeop_flags::commutative,
        OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
            pcodeop_flags::binary
        }
        OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_SUBPIECE => pcodeop_flags::unary,
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

fn mk_input(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(s, Address::new(ram, off), dt(s));
    fd.vbank_mut()
        .set_input(vn, &mut |_b, _a, _c| Ok(()))
        .expect("set_input")
}

fn mk_vn(fd: &mut Funcdata, off: u64, s: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(s, Address::new(ram, off), dt(s))
}

fn mk_const(fd: &mut Funcdata, val: u64, s: int4) -> VarnodeId {
    fd.new_constant(s, val)
}

fn set_def(fd: &mut Funcdata, vn: VarnodeId, op: OpId) {
    let seqnum = fd.obank().get(op).unwrap().get_seq_num().clone();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    let info = DefOpInfo { id: op, seqnum };
    fd.vbank_mut()
        .set_def(vn, info, &mut |_b, _a, _c| Ok(()))
        .expect("set_def");
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

fn in_of(fd: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    fd.obank().get(op).unwrap().get_in(slot).unwrap()
}

/// Build a bare `INSERT(container, value, #pos, #numbits)` op (4 inputs); the
/// container is an unused live input, `value` is the def we want absorbed.
fn mk_insert(
    fd: &mut Funcdata,
    off: u64,
    container: VarnodeId,
    value: VarnodeId,
    pos: u64,
    numbits: u64,
) -> OpId {
    let ins = mk_op(fd, 4, off, OpCode::CPUI_INSERT);
    let pos_c = mk_const(fd, pos, 4);
    let nb_c = mk_const(fd, numbits, 4);
    fd.op_set_input(ins, container, 0).unwrap();
    fd.op_set_input(ins, value, 1).unwrap();
    fd.op_set_input(ins, pos_c, 2).unwrap();
    fd.op_set_input(ins, nb_c, 3).unwrap();
    // Give the INSERT an output so it is a live op.
    let out = mk_vn(fd, off + 0x800, container_size(fd, container));
    set_def(fd, out, ins);
    ins
}

fn container_size(fd: &Funcdata, vn: VarnodeId) -> int4 {
    fd.vbank().get(vn).unwrap().get_size()
}

// ===========================================================================
// A1: absorbAnd LSB-mask gate (bitfield.cc:2241-2252)
// ===========================================================================

/// FULL-coverage mask collapses: `INSERT(x & 0xff, p, n=8)` with mask covering
/// all 8 inserted bits => INSERT(x, ...).  `(mask & val)==mask` holds.
#[test]
fn w10bfa_insert_and_full_mask_absorbs() {
    let mut fd = build_fd();
    let cont = mk_input(&mut fd, 0x100, 4);
    let x = mk_input(&mut fd, 0x110, 4);
    let m = mk_const(&mut fd, 0xff, 4); // covers the 8 low bits
    let andop = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_AND);
    fd.op_set_input(andop, x, 0).unwrap();
    fd.op_set_input(andop, m, 1).unwrap();
    let andout = mk_vn(&mut fd, 0x210, 4);
    set_def(&mut fd, andout, andop);

    let ins = mk_insert(&mut fd, 0x300, cont, andout, 0, 8);
    assert_eq!(RuleInsertAbsorb.apply_op(ins, &mut fd), 1, "full mask must absorb");
    // INSERT value input is now `x` directly (the AND is gone).
    assert_eq!(in_of(&fd, ins, 1), x, "INSERT now feeds x directly");
}

/// PARTIAL mask must NOT collapse: `INSERT(x & 0x7f, p, n=8)` — the mask drops
/// bit 7 which the INSERT keeps, so `(mask & val=0x7f) != mask(=0xff)`.  A port
/// that dropped the gate would silently lose the high bit.
#[test]
fn w10bfa_insert_and_partial_mask_does_not_absorb() {
    let mut fd = build_fd();
    let cont = mk_input(&mut fd, 0x100, 4);
    let x = mk_input(&mut fd, 0x110, 4);
    let m = mk_const(&mut fd, 0x7f, 4); // one bit short of the 8-bit field
    let andop = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_AND);
    fd.op_set_input(andop, x, 0).unwrap();
    fd.op_set_input(andop, m, 1).unwrap();
    let andout = mk_vn(&mut fd, 0x210, 4);
    set_def(&mut fd, andout, andop);

    let ins = mk_insert(&mut fd, 0x300, cont, andout, 0, 8);
    assert_eq!(RuleInsertAbsorb.apply_op(ins, &mut fd), 0, "partial mask must NOT absorb");
    assert_eq!(in_of(&fd, ins, 1), andout, "INSERT still feeds the AND output");
    assert_eq!(code_of(&fd, andop), OpCode::CPUI_INT_AND, "AND survives");
}

// ===========================================================================
// A2: leftShiftVarnode INT_MULT power-of-two match (bitfield.cc:2214-2233)
//     exercised through absorbShiftAdd (the only caller).
// ===========================================================================

/// Build `INSERT(container, ((a*K0) + (b*K1)) >> sa, pos, numbits)` and run the
/// rule; `k0`/`k1` are the multiply constants, `sa` the right shift.
fn shiftadd_fixture(k0: u64, k1: u64, sa: u64, numbits: u64) -> (Funcdata, OpId, VarnodeId, VarnodeId) {
    let mut fd = build_fd();
    let cont = mk_input(&mut fd, 0x100, 4);
    let a = mk_input(&mut fd, 0x110, 4);
    let b = mk_input(&mut fd, 0x120, 4);
    // a * k0
    let mul0 = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_MULT);
    let k0c = mk_const(&mut fd, k0, 4);
    fd.op_set_input(mul0, a, 0).unwrap();
    fd.op_set_input(mul0, k0c, 1).unwrap();
    let mul0o = mk_vn(&mut fd, 0x210, 4);
    set_def(&mut fd, mul0o, mul0);
    // b * k1
    let mul1 = mk_op(&mut fd, 2, 0x220, OpCode::CPUI_INT_MULT);
    let k1c = mk_const(&mut fd, k1, 4);
    fd.op_set_input(mul1, b, 0).unwrap();
    fd.op_set_input(mul1, k1c, 1).unwrap();
    let mul1o = mk_vn(&mut fd, 0x230, 4);
    set_def(&mut fd, mul1o, mul1);
    // add
    let add = mk_op(&mut fd, 2, 0x240, OpCode::CPUI_INT_ADD);
    fd.op_set_input(add, mul0o, 0).unwrap();
    fd.op_set_input(add, mul1o, 1).unwrap();
    let addo = mk_vn(&mut fd, 0x250, 4);
    set_def(&mut fd, addo, add);
    // >> sa
    let rsh = mk_op(&mut fd, 2, 0x260, OpCode::CPUI_INT_RIGHT);
    let sac = mk_const(&mut fd, sa, 4);
    fd.op_set_input(rsh, addo, 0).unwrap();
    fd.op_set_input(rsh, sac, 1).unwrap();
    let rsho = mk_vn(&mut fd, 0x270, 4);
    set_def(&mut fd, rsho, rsh);

    let ins = mk_insert(&mut fd, 0x300, cont, rsho, 0, numbits);
    (fd, ins, a, b)
}

/// Powers of two `1<<sa` on both multiplies => shift cancels into `a + b`.
#[test]
fn w10bfa_shiftadd_poweroftwo_absorbs() {
    let sa = 4u64;
    let k = 1u64 << sa; // 16
    let (mut fd, ins, a, b) = shiftadd_fixture(k, k, sa, 4);
    let rsh = fd.vbank().get(in_of(&fd, ins, 1)).unwrap().get_def().unwrap();
    assert_eq!(RuleInsertAbsorb.apply_op(ins, &mut fd), 1, "power-of-two multiplies must absorb");
    // The right-shift op was rewritten to INT_ADD(a, b).
    assert_eq!(code_of(&fd, rsh), OpCode::CPUI_INT_ADD, "shift rewritten to ADD");
    assert_eq!(in_of(&fd, rsh, 0), a);
    assert_eq!(in_of(&fd, rsh, 1), b);
}

/// A multiply by a NON-power-of-two (`1<<sa` expected, `K=17` supplied) must NOT
/// match `leftShiftVarnode` — the rewrite is rejected.  A shortcut that accepted
/// any constant multiply would mis-cancel the shift.
#[test]
fn w10bfa_shiftadd_nonpoweroftwo_does_not_absorb() {
    let sa = 4u64;
    let good = 1u64 << sa; // 16
    let bad = good + 1; // 17, not 1<<sa
    let (mut fd, ins, _a, _b) = shiftadd_fixture(good, bad, sa, 4);
    let rsh = fd.vbank().get(in_of(&fd, ins, 1)).unwrap().get_def().unwrap();
    assert_eq!(
        RuleInsertAbsorb.apply_op(ins, &mut fd),
        0,
        "non-power-of-two multiply must NOT absorb"
    );
    assert_eq!(code_of(&fd, rsh), OpCode::CPUI_INT_RIGHT, "shift op unchanged");
}

// ===========================================================================
// A3: absorbRightLeft shift-cancel width guard (bitfield.cc:2257-2286)
// ===========================================================================

/// `INSERT((x << c) >> c, pos, numbits)` with `numbits <= containerbits - c`
/// cancels the shift pair; with `numbits` too wide it must NOT.
fn rightleft_fixture(c: u64, numbits: u64) -> (Funcdata, OpId, VarnodeId) {
    let mut fd = build_fd();
    let cont = mk_input(&mut fd, 0x100, 4);
    let x = mk_input(&mut fd, 0x110, 4);
    let lsh = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_LEFT);
    let cc0 = mk_const(&mut fd, c, 4);
    fd.op_set_input(lsh, x, 0).unwrap();
    fd.op_set_input(lsh, cc0, 1).unwrap();
    let lsho = mk_vn(&mut fd, 0x210, 4);
    set_def(&mut fd, lsho, lsh);
    let rsh = mk_op(&mut fd, 2, 0x220, OpCode::CPUI_INT_RIGHT);
    let cc1 = mk_const(&mut fd, c, 4);
    fd.op_set_input(rsh, lsho, 0).unwrap();
    fd.op_set_input(rsh, cc1, 1).unwrap();
    let rsho = mk_vn(&mut fd, 0x230, 4);
    set_def(&mut fd, rsho, rsh);

    // INSERT value input must be 4 bytes (insert in(1) width drives the guard).
    let ins = mk_insert(&mut fd, 0x300, cont, rsho, 0, numbits);
    (fd, ins, x)
}

#[test]
fn w10bfa_rightleft_narrow_field_absorbs() {
    // container in(1) = 4 bytes = 32 bits; c=8 => preserved = 24 bits; numbits=8 <= 24
    let (mut fd, ins, x) = rightleft_fixture(8, 8);
    assert_eq!(RuleInsertAbsorb.apply_op(ins, &mut fd), 1, "narrow field must cancel shifts");
    assert_eq!(in_of(&fd, ins, 1), x, "INSERT now feeds x directly");
}

#[test]
fn w10bfa_rightleft_wide_field_does_not_absorb() {
    // c=8 => preserved=24 bits; numbits=25 > 24 => MUST NOT cancel (would lose data)
    let (mut fd, ins, _x) = rightleft_fixture(8, 25);
    let before = in_of(&fd, ins, 1);
    assert_eq!(
        RuleInsertAbsorb.apply_op(ins, &mut fd),
        0,
        "field wider than preserved bits must NOT cancel"
    );
    assert_eq!(in_of(&fd, ins, 1), before, "INSERT value input unchanged");
}

// ===========================================================================
// A4: absorbNestedAnd coveringmask/popcount gate (bitfield.cc:2333-2355)
// ===========================================================================

/// `INSERT((x & mask) + y, pos, numbits=4)`; mask drives the gate.
fn nestedand_fixture(mask: u64, numbits: u64) -> (Funcdata, OpId, VarnodeId) {
    let mut fd = build_fd();
    let cont = mk_input(&mut fd, 0x100, 4);
    let x = mk_input(&mut fd, 0x110, 4);
    let y = mk_input(&mut fd, 0x120, 4);
    let andop = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_AND);
    let mc = mk_const(&mut fd, mask, 4);
    fd.op_set_input(andop, x, 0).unwrap();
    fd.op_set_input(andop, mc, 1).unwrap();
    let ando = mk_vn(&mut fd, 0x210, 4);
    set_def(&mut fd, ando, andop);
    let add = mk_op(&mut fd, 2, 0x220, OpCode::CPUI_INT_ADD);
    fd.op_set_input(add, ando, 0).unwrap();
    fd.op_set_input(add, y, 1).unwrap();
    let addo = mk_vn(&mut fd, 0x230, 4);
    set_def(&mut fd, addo, add);

    let ins = mk_insert(&mut fd, 0x300, cont, addo, 0, numbits);
    (fd, ins, x)
}

#[test]
fn w10bfa_nestedand_contiguous_lsb_mask_absorbs() {
    // mask 0xff = contiguous LSB run, popcount 8 >= numbits 4 => absorb
    let (mut fd, ins, x) = nestedand_fixture(0xff, 4);
    let add = fd.vbank().get(in_of(&fd, ins, 1)).unwrap().get_def().unwrap();
    assert_eq!(RuleInsertAbsorb.apply_op(ins, &mut fd), 1, "contiguous LSB mask must absorb");
    assert_eq!(in_of(&fd, add, 0), x, "ADD now feeds x directly (AND dropped)");
}

#[test]
fn w10bfa_nestedand_noncontiguous_mask_does_not_absorb() {
    // mask 0xf0 is NOT a covering LSB run (coveringmask(0xf0)=0xff != 0xf0), and
    // also (mask & 1)==0 — either guard rejects it.
    let (mut fd, ins, _x) = nestedand_fixture(0xf0, 4);
    assert_eq!(
        RuleInsertAbsorb.apply_op(ins, &mut fd),
        0,
        "non-contiguous / non-LSB mask must NOT absorb"
    );
}

#[test]
fn w10bfa_nestedand_mask_narrower_than_field_does_not_absorb() {
    // mask 0x7 (popcount 3) but INSERT keeps numbits=4 bits => AND still has an
    // effect on bit 3, so it must NOT be absorbed.
    let (mut fd, ins, _x) = nestedand_fixture(0x7, 4);
    assert_eq!(
        RuleInsertAbsorb.apply_op(ins, &mut fd),
        0,
        "mask popcount < bitsize must NOT absorb"
    );
}

// ===========================================================================
// A5: absorbExt signedness match (bitfield.cc:2070-2086) — RulePullAbsorb.
// ===========================================================================

/// Build `ext( PULL(container, pos, numbits) )` where `ext`/`pull` opcodes are
/// supplied; run RulePullAbsorb on the PULL.
fn ext_fixture(pull: OpCode, ext: OpCode) -> (Funcdata, OpId, OpId) {
    let mut fd = build_fd();
    let container = mk_input(&mut fd, 0x100, 4);
    let pullop = mk_op(&mut fd, 3, 0x200, pull);
    let pos = mk_const(&mut fd, 0, 4);
    let nb = mk_const(&mut fd, 4, 4);
    fd.op_set_input(pullop, container, 0).unwrap();
    fd.op_set_input(pullop, pos, 1).unwrap();
    fd.op_set_input(pullop, nb, 2).unwrap();
    let pullo = mk_vn(&mut fd, 0x210, 4);
    set_def(&mut fd, pullo, pullop);
    let extop = mk_op(&mut fd, 1, 0x220, ext);
    fd.op_set_input(extop, pullo, 0).unwrap();
    let exto = mk_vn(&mut fd, 0x230, 8);
    set_def(&mut fd, exto, extop);
    (fd, pullop, extop)
}

#[test]
fn w10bfa_ext_matching_sign_absorbs() {
    // SEXT over SPULL => fold (the EXT becomes a PULL).
    let (mut fd, pullop, extop) = ext_fixture(OpCode::CPUI_SPULL, OpCode::CPUI_INT_SEXT);
    assert_eq!(RulePullAbsorb.apply_op(pullop, &mut fd), 1, "SEXT/SPULL must fold");
    assert_eq!(code_of(&fd, extop), OpCode::CPUI_SPULL, "EXT rewritten to SPULL");
}

#[test]
fn w10bfa_ext_mismatched_sign_does_not_absorb() {
    // SEXT over ZPULL => signedness mismatch, MUST NOT fold.
    let (mut fd, pullop, extop) = ext_fixture(OpCode::CPUI_ZPULL, OpCode::CPUI_INT_SEXT);
    assert_eq!(
        RulePullAbsorb.apply_op(pullop, &mut fd),
        0,
        "SEXT over ZPULL signedness mismatch must NOT fold"
    );
    assert_eq!(code_of(&fd, extop), OpCode::CPUI_INT_SEXT, "EXT unchanged");
}
