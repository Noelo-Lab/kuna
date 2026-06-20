//! INDEPENDENT-VERIFIER adversarial tests for `rport/w10-divmod` (round 2).
//!
//! Written by the verifier (NOT the porter), separate from the porter's
//! `verify_w10_divmod.rs`.  These target branches the porter's own tests did NOT
//! cover, and pin each expected value against an oracle computed independently
//! from the C++ algorithm (`RuleDivOpt::calcDivisor`, ruleaction.cc:8172-8213),
//! not from a divopt magic table:
//!
//!   A. GUARD A's *bare-constant* leg.  The C++ guard
//!      `if (!inVn->isWritten()) return 0;` (ruleaction.cc:8113) rejects a
//!      MULT-input-0 that is a bare CONSTANT (a constant is `isConstant`, never
//!      `isWritten`), so findForm declines rather than treating the constant as
//!      `y`.  The porter's F1 test covers a *free* input-0; this covers the
//!      constant input-0 leg the comment also claims.
//!   B. calcDivisor's three reject guards (`y <= 1`, `n > 127`, `xsize > 64`)
//!      and the maxx-failure path all return 0 — oracle-pinned.
//!   C. calcDivisor recovers a divisor for a *different* general magic (`/9`)
//!      and rejects a magic whose maxx check fails (`0x24924925 n=34 x=32 -> 0`),
//!      both oracle values from the standalone C++ algorithm; a hardcoded
//!      {81,89,91,125} table could produce neither.
//!   D. RuleDoubleShift::clone_rule fires when the live group "analysis" is in
//!      the grouplist (the bug the porter fixed: the rule's runtime group is
//!      "analysis", coreaction.cc:5807, not the placeholder name "doubleshift"),
//!      and declines for an unrelated group.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::{ActionGroupList, Rule};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_2::RuleDoubleShift;
use kuna_decomp::ruleaction_6::RuleDivOpt;
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

// -----------------------------------------------------------------------------
// fixtures (public-API only, mirroring the porter's verify_w10_divmod.rs)
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
// A — GUARD A, bare-constant input-0 leg.
// =============================================================================
//
// `INT_RIGHT(INT_MULT(#magic, #other), #s)`: getIn(0) of the MULT is a bare
// CONSTANT.  In C++, GUARD A (`if (!inVn->isWritten()) return 0;`) fires FIRST
// because a constant is never `isWritten`, so findForm returns 0 before the
// isConstantExtended branch can pick the constant up as `y` (which would then
// dereference a non-existent def for getIn(1)).  A faithful port returns None.
#[test]
fn w10_indep_findform_const_mult_input0_declines() {
    let mut fd = build_fd();

    let mult = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
    // input 0: a bare CONSTANT (isConstant, NOT isWritten) — GUARD A rejects it.
    let cmagic = mk_const(&mut fd, 8, 0xaaaaaaaaaaaaaaab);
    set_in(&mut fd, mult, cmagic, 0);
    // input 1: another constant (so the else-if would otherwise pick a `y`).
    let cother = mk_const(&mut fd, 8, 7);
    set_in(&mut fd, mult, cother, 1);
    let multout = give_output(&mut fd, mult, 0x20, 8);

    let shift = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, multout, 0);
    let s = mk_const(&mut fd, 4, 3);
    set_in(&mut fd, shift, s, 1);
    let _o = give_output(&mut fd, shift, 0x30, 8);

    assert!(
        RuleDivOpt::find_form(&fd, shift).is_none(),
        "GUARD A (ruleaction.cc:8113) declines a bare-constant MULT-input-0; \
         the port must return None"
    );
}

// =============================================================================
// B — calcDivisor reject guards (oracle-pinned to the standalone C++ algorithm).
// =============================================================================
#[test]
fn w10_indep_calc_divisor_guards_return_zero() {
    // y <= 1 boundary (ulessequal128(y, 1)) -> 0.  (C++ oracle: 0)
    assert_eq!(RuleDivOpt::calc_divisor(33, &[1, 0], 32), 0, "y=1 boundary -> 0");
    assert_eq!(RuleDivOpt::calc_divisor(33, &[0, 0], 32), 0, "y=0 boundary -> 0");
    // n > 127 precision guard -> 0.  (C++ oracle: 0)
    assert_eq!(RuleDivOpt::calc_divisor(128, &[0xaaaaaaab, 0], 32), 0, "n>127 -> 0");
    // xsize > 64 precision guard -> 0.  (C++ oracle: 0)
    assert_eq!(RuleDivOpt::calc_divisor(33, &[0xaaaaaaab, 0], 65), 0, "xsize>64 -> 0");
}

// =============================================================================
// C — general magic recovery + a maxx-failure reject (both oracle-pinned).
// =============================================================================
//
// Oracle values from the standalone C++ calcDivisor (verified by compiling the
// upstream algorithm against these exact inputs):
//   0x38e38e39 n=33 x=32 -> 9   (uint32 `/9` reciprocal: general math recovers 9)
//   0x24924925 n=34 x=32 -> 0   (a real reciprocal-shaped magic whose maxx check
//                                fails at this (n,xsize) -> declines, NOT a
//                                fabricated divisor)
// A hardcoded {81,89,91,125} table could produce neither result.
#[test]
fn w10_indep_calc_divisor_div9_recovers_and_maxx_fail_rejects() {
    assert_eq!(
        RuleDivOpt::calc_divisor(33, &[0x38e38e39, 0], 32),
        9,
        "general reciprocal 0x38e38e39 must derive 9 (not from any corpus table)"
    );
    assert_eq!(
        RuleDivOpt::calc_divisor(34, &[0x24924925, 0], 32),
        0,
        "reciprocal-shaped magic whose maxx check fails must decline (oracle 0), \
         not return a fabricated divisor"
    );
}

// =============================================================================
// D — RuleDoubleShift::clone_rule fires for the live "analysis" group.
// =============================================================================
//
// C++ `RuleDoubleShift(g) : Rule(g,0,"doubleshift")` with `g == "analysis"`
// (coreaction.cc:5807); its clone tests `grouplist.contains(getGroup())` =
// contains("analysis").  The Rust unit struct previously tested only
// "doubleshift" (the rule NAME), dropping the rule from every live pool.  The
// fixed clone_rule must return Some when "analysis" is enabled, and None for an
// unrelated group.
#[test]
fn w10_indep_doubleshift_clone_fires_for_analysis_group() {
    let rule = RuleDoubleShift;
    let analysis = ActionGroupList::from_names(["analysis"]);
    assert!(
        rule.clone_rule(&analysis).is_some(),
        "RuleDoubleShift must clone into a pool whose enabled group is \"analysis\" \
         (its real runtime group, coreaction.cc:5807)"
    );

    let unrelated = ActionGroupList::from_names(["stackvars"]);
    assert!(
        RuleDoubleShift.clone_rule(&unrelated).is_none(),
        "RuleDoubleShift must decline when neither \"analysis\" nor the \
         \"doubleshift\" placeholder is enabled"
    );

    // The placeholder-`specs()` contract still holds (so the per-op spec table,
    // which keys on the name, also clones).
    let placeholder = ActionGroupList::from_names(["doubleshift"]);
    assert!(
        RuleDoubleShift.clone_rule(&placeholder).is_some(),
        "the placeholder \"doubleshift\" group must still clone (specs() contract)"
    );
}
