//! Adversarial verifier tests for item `w5-s3-subflow` (round 1).
//!
//! These exercise the spots the hunt list flagged as most fragile in the
//! `SubvariableFlow` discovery engine and its trigger rules
//! (`decompiler/cpp/subflow.cc`):
//!
//!   * F1 — unguarded shift in `RuleSubvarShift::applyOp` (`mask >> sa` /
//!     `(mask>>sa)<<sa`, subflow.cc:1694-1695) with a shift amount >= 64: the
//!     C++ is shift-past-width UB (x86-masked in practice, never crashes); the
//!     Rust uses a bare `>>` which PANICS in a debug build.  This pins the
//!     divergence as a panic boundary.
//!   * the forward bitsize==1 boolean-movement branch (subflow.cc:594-613) —
//!     the `booldir` inversion under INT_EQUAL vs INT_NOTEQUAL — and the
//!     backward INT_ADD single-bit→XOR vs multi-bit→ADD split
//!     (subflow.cc:720-729), through the public `do_trace` surface.
//!   * the duplicate-input CALL repeat-slot path (subflow.cc:616-623), which is
//!     W4-seamed: a CALL descendant must surface the seam (do_trace == Err via
//!     run-through) rather than silently succeeding.
//!
//! The tests reconstruct minimal hand-built Funcdata graphs (mirroring the
//! in-module `subflow::tests` scaffolding) so the discovery semantics run on
//! the executable trace paths.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::Rule;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::seams::{Architecture, TypeOp, VarnodeId};
use kuna_decomp::subflow::{RuleSubvarShift, SubvariableFlow};
use kuna_decomp::varnode::DefOpInfo;

// ---- scaffolding (mirrors subflow::tests) ----------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
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

fn no_replace(
) -> impl FnMut(&mut kuna_decomp::varnode::VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut kuna_decomp::varnode::VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> {
        Ok(())
    }
}

fn mk_op(fd: &mut Funcdata, off: u64, ninput: int4, opc: OpCode) -> kuna_decomp::seams::OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(ninput, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn mk_reg(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

fn mk_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let vn = mk_reg(fd, off, size);
    fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
}

fn wire_in(fd: &mut Funcdata, op: kuna_decomp::seams::OpId, vn: VarnodeId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn wire_out(fd: &mut Funcdata, defop: kuna_decomp::seams::OpId, vn: VarnodeId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

// ---- F1: RuleSubvarShift unguarded shift-by->=64 is a panic divergence ------
//
// C++ subflow.cc:1692-1695:
//   int4 sa = (int4)op->getIn(1)->getOffset();
//   uintb mask = vn->getNZMask();
//   if ((mask >> sa) != (uintb)1) return 0;   // <-- shift-past-width UB if sa>=64
//   mask = (mask >> sa) << sa;
// There is NO `sa >= sizeof(uintb)*8` guard here (unlike the trace functions).
// With a 1-byte register vn, getNZMask()==u64::MAX in this tree.  A constant
// shift of 64 makes the C++ `mask >> 64` UB (x86 masks to `>> 0`, so mask stays
// MAX, `!= 1` -> returns 0, no crash).  The Rust bare `>>` PANICS in debug.
//
// This pins the panic boundary (the W5-s3-subflow F1 divergence).  If the port
// later switches these to wrapping shifts (`wshr`/`wshl`, ADR 0003), this test
// should be flipped to assert `apply_op == 0` with no panic.
#[test]
#[should_panic(expected = "shift")]
fn w5s3_subvar_shift_amount_64_panics_where_cpp_is_masked_ub_f1() {
    let mut fd = build_fd();
    // 1-byte input register: get_size()==1 (passes the rule's size guard),
    // nz_mask == u64::MAX (registers are seeded MAX in this tree).
    let vn = mk_input(&mut fd, 0x40, 1);
    let cst = fd.new_constant(4, 64); // shift amount 64 (>= 64 -> C++ UB)
    let shr = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_RIGHT);
    let outvn = mk_reg(&mut fd, 0x80, 1);
    wire_in(&mut fd, shr, vn, 0);
    wire_in(&mut fd, shr, cst, 1);
    let _outvn = wire_out(&mut fd, shr, outvn);

    let mut rule = RuleSubvarShift::new("g");
    // C++: `mask >> 64` is UB-but-masked -> returns 0 (no crash).  Rust: panics.
    let _ = rule.apply_op(shr, &mut fd);
}

// A control: a *small* shift (sa < 64) on the same shape does NOT panic and
// returns 0 cleanly (nz_mask==MAX so `(MAX >> 3) != 1`).  This isolates the
// divergence to the shift width, not the rest of the rule.
#[test]
fn w5s3_subvar_shift_small_amount_no_panic_returns_zero() {
    let mut fd = build_fd();
    let vn = mk_input(&mut fd, 0x40, 1);
    let cst = fd.new_constant(4, 3);
    let shr = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_RIGHT);
    let outvn = mk_reg(&mut fd, 0x80, 1);
    wire_in(&mut fd, shr, vn, 0);
    wire_in(&mut fd, shr, cst, 1);
    let _outvn = wire_out(&mut fd, shr, outvn);

    let mut rule = RuleSubvarShift::new("g");
    assert_eq!(rule.apply_op(shr, &mut fd), 0);
}

// ---- F-check: forward INT_LESS aggressive gate (subflow.cc:560-575) ---------
//
// The forward comparison path gates on `(!aggressive) && ((nz|mask) != mask)`:
// a register has nz_mask==MAX in this tree, so a non-aggressive trace MUST bail
// (`MAX|0xff != 0xff` -> return false), while an aggressive trace skips that
// guard and proceeds into createCompareBridge (other side is a constant whose
// bits fit the mask).  This pins the aggressive flag's effect on the compare
// path and the `(mask | otherOffset) != mask` constant guard, exactly where a
// transposed condition (using `||` or dropping `!aggressive`) would flip the
// outcome.
#[test]
fn w5s3_forward_int_less_non_aggressive_bails_aggressive_proceeds() {
    // shared shape builder: root (4-byte input) used by `root < const`.
    fn build(aggr: bool) -> bool {
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 4);
        fd.vbank_mut().get_mut(root).unwrap().set_consume(0xff);
        // other side is a constant whose only set bits are within the mask 0xff.
        let cst = fd.new_constant(4, 0x12);
        let lessop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_LESS);
        let cmpout = mk_reg(&mut fd, 0x80, 1);
        wire_in(&mut fd, lessop, root, 0);
        wire_in(&mut fd, lessop, cst, 1);
        let _cmpout = wire_out(&mut fd, lessop, cmpout);

        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, aggr, false, false).unwrap();
        sf.do_trace(&mut fd).unwrap()
    }
    // Non-aggressive: nz(MAX)|mask != mask -> bail -> the only descendant is
    // unhandled -> do_trace false.
    assert!(!build(false), "non-aggressive INT_LESS must bail on nz!=mask");
    // Aggressive: skips the nz guard; the compare-bridge terminal bumps pullcount
    // -> do_trace true.  (Asymmetry proves the `!aggressive` guard is live.)
    assert!(build(true), "aggressive INT_LESS must proceed through the compare bridge");
}

// ---- F-check: backward INT_ADD single-bit -> XOR, input-flag guard ----------
//
// Backward through an INT_ADD with mask==1 builds a single-bit adder
// (createOp(CPUI_INT_XOR,2,...), subflow.cc:723-724) and links both operands at
// mask 1.  Here the operands are 1-byte INPUTS with bitsize 1, so
// setReplacement's input handling (`if (bitsize < 8) return 0`, subflow.cc:123)
// rejects them -> the backward links fail -> trace fails.  This exercises BOTH
// the mask==1 add->XOR branch selection and the faithful 1-bit-input-flag guard
// (a swapped `mask==1` -> ADD, or a missing input guard, would change the
// outcome).
#[test]
fn w5s3_backward_int_add_single_bit_input_flag_guard() {
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x40, 1);
    let b = mk_input(&mut fd, 0x48, 1);
    let addop = mk_op(&mut fd, 0x80, 2, OpCode::CPUI_INT_ADD);
    let sumv = mk_reg(&mut fd, 0xc0, 1);
    fd.vbank_mut().get_mut(sumv).unwrap().set_consume(0x1);
    wire_in(&mut fd, addop, a, 0);
    wire_in(&mut fd, addop, b, 1);
    let sumv = wire_out(&mut fd, addop, sumv);

    // sumv: 1-byte written, mask==1, not pinned (INT_ADD is not bool-output) ->
    // traced.  Backward INT_ADD reaches the XOR adder, then the 1-bit input
    // operands cannot become input flags -> createLink fails -> do_trace false.
    let mut sf = SubvariableFlow::new(&mut fd, sumv, 0x1, true, false, false).unwrap();
    assert!(!sf.do_trace(&mut fd).unwrap());
}

// ---- F-check: a CALL descendant routes through tryCallPull (seam now closed) -
//
// subflow.cc:616-623: a CALL descendant (slot != 0) routes through tryCallPull.
// With the call-site machinery now ported (w10-callsite-args), tryCallPull is no
// longer a W4 seam: `fd->getCallSpecs(op)` is queried, and a CALL op with **no
// registered FuncCallSpecs** (this hand-built fixture installs none) takes the
// `fc == 0` arm and returns `false` (no truncation) — so `do_trace` completes
// `Ok(false)` instead of surfacing an Err.  This pins the closed seam: the trace
// no longer aborts mid-flight on a CALL descendant.
#[test]
fn w5s3_forward_call_descendant_no_callspec_returns_false() {
    let mut fd = build_fd();
    let root = mk_input(&mut fd, 0x40, 4);
    fd.vbank_mut().get_mut(root).unwrap().set_consume(0xff);
    // CALL with the logical value in slot 1 (slot 0 is the call target).
    let target = fd.new_constant(8, 0x2000);
    let callop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_CALL);
    wire_in(&mut fd, callop, target, 0);
    wire_in(&mut fd, callop, root, 1);

    let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, true, false, false).unwrap();
    // tryCallPull queries getCallSpecs(op); no call spec is registered for this
    // fixture op -> `fc == 0` -> returns false -> do_trace completes Ok(false).
    let r = sf.do_trace(&mut fd);
    assert_eq!(
        r.ok(),
        Some(false),
        "a CALL(slot=1) descendant with no call spec returns Ok(false) (no truncation)"
    );
}
