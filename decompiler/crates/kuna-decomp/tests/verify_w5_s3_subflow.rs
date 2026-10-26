//! Adversarial verifier tests for item `w5-s3-subflow` (round 1).
//!
//! These exercise the spots the hunt list flagged as most fragile in the
//! `SubvariableFlow` discovery engine and its trigger rules
//! (`decompiler/cpp/subflow.cc`):
//!
//!   * F1 — shifts beyond mask precision in `RuleSubvarShift::applyOp`
//!     decline without changing the IR instead of reproducing C++ shift UB.
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
use kuna_decomp::context::{ArchContext, TypeOp, VarnodeId};
use kuna_decomp::subflow::{RuleSubvarShift, SubfloatFlow, SubvariableFlow};
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

fn no_replace(
) -> impl FnMut(&mut kuna_decomp::varnode::VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut kuna_decomp::varnode::VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> {
        Ok(())
    }
}

fn mk_op(fd: &mut Funcdata, off: u64, ninput: int4, opc: OpCode) -> kuna_decomp::context::OpId {
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

fn wire_in(fd: &mut Funcdata, op: kuna_decomp::context::OpId, vn: VarnodeId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn wire_out(fd: &mut Funcdata, defop: kuna_decomp::context::OpId, vn: VarnodeId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

#[test]
fn w5s3_subvar_shift_amount_64_is_rejected_without_mutating_ir() {
    let mut fd = build_fd();
    let vn = mk_input(&mut fd, 0x40, 1);
    let cst = fd.new_constant(4, 64);
    let shr = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_RIGHT);
    let outvn = mk_reg(&mut fd, 0x80, 1);
    wire_in(&mut fd, shr, vn, 0);
    wire_in(&mut fd, shr, cst, 1);
    let outvn = wire_out(&mut fd, shr, outvn);
    let before = fd.vbank().num_varnodes();

    let mut rule = RuleSubvarShift::new("g");
    assert_eq!(rule.apply_op(shr, &mut fd), 0);
    assert_eq!(fd.obank().get(shr).unwrap().code(), OpCode::CPUI_INT_RIGHT);
    assert_eq!(fd.obank().get(shr).unwrap().get_in(0), Some(vn));
    assert_eq!(fd.obank().get(shr).unwrap().get_in(1), Some(cst));
    assert_eq!(fd.obank().get(shr).unwrap().get_out(), Some(outvn));
    assert_eq!(fd.vbank().num_varnodes(), before);
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

#[test]
fn subfloat_trace_keeps_wide_binary_arithmetic_from_both_directions() {
    for code in [
        OpCode::CPUI_FLOAT_ADD,
        OpCode::CPUI_FLOAT_SUB,
        OpCode::CPUI_FLOAT_MULT,
        OpCode::CPUI_FLOAT_DIV,
    ] {
        let mut glb = ArchContext::new(build_manager());
        glb.floatformats = vec![
            kuna_num::float::FloatFormat::new(4),
            kuna_num::float::FloatFormat::new(8),
        ];
        let glb = Rc::new(glb);
        let r = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let mut fd = Funcdata::new("func", "func", glb, Address::new(r, 0x1000), 0, 0x40).unwrap();
        let mut widened = Vec::new();
        for i in 0..2 {
            let input = mk_input(&mut fd, 0x40 + i * 4, 4);
            let widen = mk_op(&mut fd, 0x100 + i * 4, 1, OpCode::CPUI_FLOAT_FLOAT2FLOAT);
            wire_in(&mut fd, widen, input, 0);
            let output = mk_reg(&mut fd, 0x80 + i * 8, 8);
            widened.push(wire_out(&mut fd, widen, output));
        }
        let binary = mk_op(&mut fd, 0x108, 2, code);
        wire_in(&mut fd, binary, widened[0], 0);
        wire_in(&mut fd, binary, widened[1], 1);
        let output = mk_reg(&mut fd, 0x90, 8);
        let output = wire_out(&mut fd, binary, output);
        let narrow = mk_op(&mut fd, 0x10c, 1, OpCode::CPUI_FLOAT_FLOAT2FLOAT);
        wire_in(&mut fd, narrow, output, 0);
        let result = mk_reg(&mut fd, 0xa0, 4);
        wire_out(&mut fd, narrow, result);

        for root in [widened[0], output] {
            let mut trace = SubfloatFlow::new(&mut fd, root, 4);
            assert!(fd.get_arch().get_float_format(4).is_some());
            assert!(!trace.do_trace(&mut fd).unwrap(), "lowered {code:?}");
            for vn in [widened[0], widened[1], output] {
                assert!(!fd.vbank().get(vn).unwrap().is_mark());
                assert_eq!(fd.vbank().get(vn).unwrap().get_size(), 8);
            }
        }
    }
}

#[test]
fn subfloat_trace_preserves_integer_conversion_rounding() {
    for (integer_size, original_size, can_lower) in [
        (2, 8, true), (3, 8, true), (4, 8, false), (8, 8, false), (2, 16, true), (2, 10, true),
        (4, 10, false),
    ] {
        let mut context = ArchContext::new(build_manager());
        context.floatformats = [4, 8].into_iter().map(kuna_num::float::FloatFormat::new).collect();
    context.int_float_round = true;
        let context = Rc::new(context);
        let ram = Rc::clone(context.manage().get_space_by_name("ram").unwrap());
        let mut fd = Funcdata::new("convert", "convert", context, Address::new(ram, 0x1000), 0, 0x40).unwrap();
        let input = mk_input(&mut fd, 0x40, integer_size);
        let convert = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_FLOAT_INT2FLOAT);
        wire_in(&mut fd, convert, input, 0);
        let output = mk_reg(&mut fd, 0x80, original_size);
        let output = wire_out(&mut fd, convert, output);
        let narrow = mk_op(&mut fd, 0x104, 1, OpCode::CPUI_FLOAT_FLOAT2FLOAT);
        wire_in(&mut fd, narrow, output, 0);
        let result = mk_reg(&mut fd, 0x90, 4);
        wire_out(&mut fd, narrow, result);

        let mut trace = SubfloatFlow::new(&mut fd, output, 4);
        assert_eq!(trace.do_trace(&mut fd).unwrap(), can_lower, "integer size={integer_size}, original float size={original_size}");
        assert!(!fd.vbank().get(output).unwrap().is_mark());
        assert_eq!(fd.vbank().get(output).unwrap().get_size(), original_size);
    }
}

#[test]
fn subfloat_trace_does_not_round_integer_before_square_root() {
    let mut context = ArchContext::new(build_manager());
    context.floatformats = [4, 8].into_iter().map(kuna_num::float::FloatFormat::new).collect();
    context.int_float_round = true;
    let context = Rc::new(context);
    let ram = Rc::clone(context.manage().get_space_by_name("ram").unwrap());
    let mut fd = Funcdata::new("root", "root", context, Address::new(ram, 0x1000), 0, 0x40).unwrap();
    let input = mk_input(&mut fd, 0x40, 4);
    let convert = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_FLOAT_INT2FLOAT);
    wire_in(&mut fd, convert, input, 0);
    let converted = mk_reg(&mut fd, 0x80, 8);
    let converted = wire_out(&mut fd, convert, converted);
    let sqrt = mk_op(&mut fd, 0x104, 1, OpCode::CPUI_FLOAT_SQRT);
    wire_in(&mut fd, sqrt, converted, 0);
    let root = mk_reg(&mut fd, 0x90, 8);
    let root = wire_out(&mut fd, sqrt, root);
    let narrow = mk_op(&mut fd, 0x108, 1, OpCode::CPUI_FLOAT_FLOAT2FLOAT);
    wire_in(&mut fd, narrow, root, 0);
    let result = mk_reg(&mut fd, 0xa0, 4);
    wire_out(&mut fd, narrow, result);

    let mut trace = SubfloatFlow::new(&mut fd, root, 4);
    assert!(!trace.do_trace(&mut fd).unwrap());
    for vn in [converted, root] {
        assert!(!fd.vbank().get(vn).unwrap().is_mark());
        assert_eq!(fd.vbank().get(vn).unwrap().get_size(), 8);
    }
}
