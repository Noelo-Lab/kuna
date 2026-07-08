//! Independent-verifier adversarial tests for item `rport/w10-divmod`
//! (the div/mod reciprocal-multiply reconstruction: `RuleDivOpt`,
//! `RuleDivTermAdd`, `RuleDivTermAdd2`, `RuleSubNormal` commit, the
//! `isConstantExtended` / `newExtendedConstant` seams, and the divopt corpus).
//!
//! Written by the verifier (NOT the porter).  Targets the spots the hunt list
//! flagged most fragile for this item:
//!
//!   F1. `RuleDivOpt::findForm` dropped the C++ GUARD A
//!       (`if (!inVn->isWritten()) return 0;` on `curOp->getIn(0)`,
//!       ruleaction.cc:8113).  When MULT input 0 is a *free* (non-written,
//!       non-constant) Varnode and input 1 is the constant, C++ declines
//!       cleanly; the Rust reaches `def_of(in_vn_cur).expect(...)` and PANICS.
//!   N.  No special-casing: `calc_divisor` must derive the divisor from the
//!       general 128-bit magic-number math for an arbitrary (non-divopt) magic,
//!       not from a hardcoded table.
//!   P.  REAL parity: the divopt corpus must render the EXACT oracle forms
//!       (`*divu = *divu / 81;`, `divu[1] = divu[1] / 89;`, the signed `divi`
//!       twins, and a modulo `% 81`) — not a coincidental `/ 81` substring.
//!   S.  SEXT `xsize` boundary: the `xsize -= 1` adjustment for SEXT and the
//!       `1u64 << xsize` in `calc_divisor` must not panic / wrap at the
//!       minimum signed numerand size.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_6::RuleDivOpt;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};
use kuna_base::error::KunaResult;

// -----------------------------------------------------------------------------
// fixtures (public-API only, mirroring verify_w5_s3_rules_6.rs)
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

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn mk_const(fd: &mut Funcdata, size: int4, val: uintb) -> VarnodeId {
    fd.new_constant(size, val)
}

/// A *free* Varnode (not written, not input, not constant) — the C++ `isFree()`
/// case that GUARD A at ruleaction.cc:8113 specifically rejects.
fn mk_free(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

fn give_output(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn set_in(fd: &mut Funcdata, op: OpId, vn: VarnodeId, slot: int4) {
    fd.op_set_input(op, vn, slot).unwrap();
}

// =============================================================================
// F1 — findForm dropped GUARD A: a free MULT-input-0 must DECLINE, not panic.
// =============================================================================
//
// C++ `RuleDivOpt::findForm` (ruleaction.cc:8112-8122):
//   Varnode *inVn = curOp->getIn(0);
//   if (!inVn->isWritten()) return 0;            // GUARD A
//   if (inVn->isConstantExtended(y)) { ... }
//   else if (!getIn(1)->isConstantExtended(y)) return 0;
//   PcodeOp *extOp = inVn->getDef();             // requires inVn written
//
// The Rust port omits GUARD A.  Construct the form `INT_RIGHT(INT_MULT(free, #c), #s)`:
// MULT input 0 is free (isConstantExtended -> None), input 1 is the constant, so
// the else-if branch keeps `in_vn_cur = free` and then calls
// `def_of(free).expect("findForm: in_vn has no def")` -> panic.
//
// A faithful port returns `None` (the C++ `return 0`).  This test asserts the
// faithful behaviour and therefore FAILS (panics) on the current port: it is the
// REJECT's failing test.
#[test]
fn w10_divmod_findform_free_mult_input0_declines_not_panics() {
    let mut fd = build_fd();

    // INT_MULT(free, #magic) -> multout (8 bytes).
    let mult = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
    let free = mk_free(&mut fd, 0x10, 8); // input 0: FREE (C++ GUARD A rejects)
    set_in(&mut fd, mult, free, 0);
    let cmagic = mk_const(&mut fd, 8, 0xcccccccccccccccd); // input 1: constant
    set_in(&mut fd, mult, cmagic, 1);
    let multout = give_output(&mut fd, mult, 0x20, 8);

    // INT_RIGHT(multout, #3) -> the findForm root.
    let shift = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, multout, 0);
    let s = mk_const(&mut fd, 4, 3);
    set_in(&mut fd, shift, s, 1);
    let _shiftout = give_output(&mut fd, shift, 0x30, 8);

    // Faithful C++ behaviour: GUARD A makes findForm return 0 (None here).
    // The current port instead panics in `def_of(...).expect(...)`.
    let res = RuleDivOpt::find_form(&fd, shift);
    assert!(
        res.is_none(),
        "C++ GUARD A (ruleaction.cc:8113) declines a free MULT-input-0; \
         the port must return None, not panic"
    );
}

// =============================================================================
// N — no special-casing: calc_divisor derives N from the general magic math.
// =============================================================================
//
// The divisor must come from `2^n / (y-1)` (ruleaction.cc:8183-8212), not a
// hardcoded divopt table.  Probe an ARBITRARY magic that is NOT one of the
// divopt divisors: the canonical `/3` reciprocal `0xaaaaaaab` at shift 33 on a
// 32-bit numerand recovers divisor 3.  A table keyed on divopt magics could not
// produce this.  Oracle computed independently against the C++ algorithm.
#[test]
fn w10_divmod_calc_divisor_general_magic_not_table() {
    // y = 0xaaaaaaab (the classic uint32 `/3` reciprocal), n = 33, xsize = 32.
    assert_eq!(
        RuleDivOpt::calc_divisor(33, &[0xaaaaaaab, 0], 32),
        3,
        "calc_divisor must derive 3 from 2^33/(0xaaaaaaab-1), proving general math"
    );
    // A different general magic: uint32 `/5` reciprocal 0xcccccccd at shift 34.
    assert_eq!(
        RuleDivOpt::calc_divisor(34, &[0xcccccccd, 0], 32),
        5,
        "calc_divisor must derive 5 from the general reciprocal math"
    );
    // None of the divopt-corpus divisors {81,89,91,...} is special-cased: an
    // arbitrary tiny magic that encodes no valid divisor returns 0, not a
    // fabricated corpus value.
    assert_eq!(
        RuleDivOpt::calc_divisor(8, &[3, 0], 16),
        0,
        "non-magic multiplier must reject (no hardcoded divisor fallback)"
    );
}

// =============================================================================
// S — SEXT xsize boundary: minimum signed numerand must not panic/wrap.
// =============================================================================
//
// In applyOp, SEXT does `xsize -= 1`.  findForm sets `xsize = inSize*8` for
// SEXT, so the minimum is 8 (1-byte input) -> 7 after the adjustment.  The
// `1u64 << xsize` in calc_divisor must be well-defined for that range (the
// concern: a 0 or negative xsize would shift-panic).  We drive calc_divisor
// directly across the SEXT-adjusted small-xsize range and assert it returns
// (some value, possibly 0) without panicking.
#[test]
fn w10_divmod_calc_divisor_sext_small_xsize_no_panic() {
    // xsize = 7 (the SEXT 1-byte minimum after `xsize -= 1`): must not panic.
    let _ = RuleDivOpt::calc_divisor(33, &[0xaaaaaaab, 0], 7);
    // xsize = 0 is the degenerate boundary (`1<<0 = 1`, maxx = 0): must not panic.
    let _ = RuleDivOpt::calc_divisor(33, &[0xaaaaaaab, 0], 0);
    // xsize = 63 (just under the 64 guard): `1u64 << 63` must not overflow-panic.
    let _ = RuleDivOpt::calc_divisor(64, &[0xaaaaaaab, 0], 63);
    // xsize = 64 takes the `maxx = 0` branch (no shift): must not panic.
    let _ = RuleDivOpt::calc_divisor(64, &[0xaaaaaaab, 0], 64);
}

// =============================================================================
// P — REAL parity: the divopt corpus renders the EXACT oracle div/mod forms.
// =============================================================================
//
// The new PASS must be genuine parity with the C++ oracle's <stringmatch>
// expectations (divopt.xml:224-...), not a coincidental `/ 81` substring.  The
// full assignment forms `*divu = *divu / 81;`, `divu[1] = divu[1] / 89;`, the
// signed `divi` twins, and a modulo `% 81` must ALL appear, and the raw
// reciprocal magic / wide `* -1` mask must be GONE.

mod corpus {
    include!("verify_w10_divmod_corpus.rs");
}
