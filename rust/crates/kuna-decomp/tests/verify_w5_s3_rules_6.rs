//! Adversarial verifier tests for item `w5-s3-rules-6` (ruleaction.cc:6931-8373,
//! the 14 simplification rules RulePtraddUndo..RuleDivOpt).
//!
//! These were written by the *independent verifier* (not the porter) and target
//! the spots the hunt list flagged as most fragile for this item:
//!
//!   1. `RuleDivOpt::calc_divisor` — the off-by-one `r >= q` adjustment branch and
//!      its `diff` accumulation (128-bit reciprocal-divisor math, wrap-prone).
//!      Oracle values computed independently in Python against the C++ algorithm.
//!   2. `RulePtrsubUndo::get_extra_offset` — the final
//!      `sign_extend(extra, 8*outvn->getSize()-1)` boundary: a *positive* uintb
//!      accumulator (0x80000000) sign-extends to a *negative* int8 at a 4-byte
//!      output width (signedness / width-truncation hunt).
//!   3. `RuleDivOpt::move_sign_bit_extraction` — the grow-the-`testList`
//!      COPY-chaining path together with the INT_AND mask-subset guard
//!      (`getOffset() != (getOffset() & otherVn->getOffset())`), an
//!      order-sensitive traversal that erases-while-iterating in C++.
//!   4. `RuleSubNormal` — the `k = insize-c-outsize` fall-through that then
//!      collapses to a least-significant SUBPIECE (`n==0` commit) — a *distinct*
//!      committing path from the porter's existing `unnecessary_shift` test.
//!
//! The Funcdata-graph fixtures mirror the in-crate `ruleaction_6::tests` helpers
//! but are rebuilt here against the crate's public API only.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_6::{RuleDivOpt, RulePtrsubUndo, RuleSubNormal};
use kuna_decomp::action::Rule;
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

// -----------------------------------------------------------------------------
// fixtures (mirror ruleaction_6::tests, public-API only)
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
// 1. calc_divisor — the off-by-one (r >= q) adjustment branch
// =============================================================================

/// `calcDivisor` for a magic whose initial division lands with `r >= q`, forcing
/// the `q += 1; r = r - y; r += q` adjustment and the `diff = q[0]` accumulation
/// (ruleaction.cc:8190-8198, 8206).
///
/// Both probes go through the adjustment branch (q=2, r>=q at shift 35).  The
/// oracle is the *fixed-width 128-bit / u64* result, NOT the arbitrary-precision
/// "ideal": the final `diff += q[0] - r[0]` is a **u64** subtraction whose wrap
/// when `r[0] > q[0]` is load-bearing.  `0x2aaaaaaac` recovers divisor 3;
/// `0x2aaaaaaad` (one higher) lands a larger `r[0]`, the u64 `q[0]-r[0]` wraps to
/// a huge denom, and the maxx feasibility check rejects it -> 0.  A port that
/// used widening (Python-style bignum) arithmetic here would return 3 for both,
/// so this pins the C++ fixed-width semantics exactly.
#[test]
fn w5s3r6_calc_divisor_off_by_one_adjustment_branch() {
    // y = 0x2aaaaaaac, n = 35, xsize = 32: adjustment branch -> divisor 3.
    assert_eq!(
        RuleDivOpt::calc_divisor(35, &[0x2aaaaaaac, 0], 32),
        3,
        "off-by-one adjustment path recovers divisor 3 (fixed-width)"
    );
    // y one higher: same branch, but the u64 q[0]-r[0] wrap fails the maxx check.
    assert_eq!(
        RuleDivOpt::calc_divisor(35, &[0x2aaaaaaad, 0], 32),
        0,
        "fixed-width u64 (q[0]-r[0]) wrap must reject this neighbour (NOT bignum 3)"
    );
}

/// Guards/rejections around the adjustment math: y just past the `y < q` and
/// boundary rejections must return 0, not a spurious divisor.  Probes the
/// `y <= 1`, `n > 127`, and `xsize > 64` precision guards together with a
/// non-magic multiplier that fails the final `tmp <= maxx` check.
#[test]
fn w5s3r6_calc_divisor_rejections_return_zero() {
    assert_eq!(RuleDivOpt::calc_divisor(35, &[1, 0], 32), 0, "y == 1 is wrong form");
    assert_eq!(RuleDivOpt::calc_divisor(128, &[0x2aaaaaad, 0], 32), 0, "n > 127");
    assert_eq!(RuleDivOpt::calc_divisor(35, &[0x2aaaaaad, 0], 65), 0, "xsize > 64");
    // A multiplier that is NOT a valid reciprocal encoding for any small divisor
    // at this shift: the maxx feasibility check fails -> 0.
    assert_eq!(RuleDivOpt::calc_divisor(8, &[3, 0], 32), 0, "non-magic multiplier rejected");
}

// =============================================================================
// 2. get_extra_offset — positive uintb accumulator sign-extends to negative int8
// =============================================================================

/// `RulePtrsubUndo::getExtraOffset` ends in
/// `extra = sign_extend(extra, 8*outvn->getSize()-1)` (ruleaction.cc:7058).  Feed
/// a PTRSUB whose lone descendant is `INT_ADD(out, 0x80000000)` with a 4-byte
/// INT_ADD output: the accumulated `extra` is the *positive* uintb 0x80000000,
/// but at a 31-bit sign position it must come back as the *negative* int8
/// 0xffffffff80000000.  A width/signedness slip (extending at the wrong width, or
/// treating the accumulator as unsigned) would yield +0x80000000 instead.
#[test]
fn w5s3r6_get_extra_offset_sign_extends_to_negative() {
    let mut fd = build_fd();

    // base varnode (written) for the PTRSUB
    let bdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let base = give_output(&mut fd, bdef, 0x10, 8);

    // PTRSUB(base, #5) -> ptrout (8 bytes)
    let ptrsub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_PTRSUB);
    set_in(&mut fd, ptrsub, base, 0);
    let off5 = mk_const(&mut fd, 8, 5);
    set_in(&mut fd, ptrsub, off5, 1);
    let ptrout = give_output(&mut fd, ptrsub, 0x20, 8);

    // INT_ADD(ptrout, #0x80000000) -> addout (4 bytes) ; ptrout is the LONE
    // descendant of the PTRSUB output, addout has no descendant -> loop stops.
    let add = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD);
    set_in(&mut fd, add, ptrout, 0);
    let big = mk_const(&mut fd, 4, 0x8000_0000);
    set_in(&mut fd, add, big, 1);
    let _addout = give_output(&mut fd, add, 0x30, 4);

    let (extra, multiplier) = RulePtrsubUndo::get_extra_offset(&fd, ptrsub);
    assert_eq!(
        extra, -2_147_483_648i64,
        "0x80000000 at a 4-byte (31-bit) sign position must sign-extend negative"
    );
    assert_eq!(multiplier, 0, "no INT_MULT/PTRADD multiplier present");
}

/// Companion to the above with an 8-byte INT_ADD output: the *same* 0x80000000
/// accumulator now sits below the 63-bit sign position and stays positive.  This
/// pins that the sign-extension width is genuinely `outvn`-driven (the loop's
/// last `outvn`), not a fixed 32-bit assumption.
#[test]
fn w5s3r6_get_extra_offset_wide_output_stays_positive() {
    let mut fd = build_fd();
    let bdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let base = give_output(&mut fd, bdef, 0x10, 8);
    let ptrsub = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_PTRSUB);
    set_in(&mut fd, ptrsub, base, 0);
    let off5 = mk_const(&mut fd, 8, 5);
    set_in(&mut fd, ptrsub, off5, 1);
    let ptrout = give_output(&mut fd, ptrsub, 0x20, 8);

    let add = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD);
    set_in(&mut fd, add, ptrout, 0);
    let big = mk_const(&mut fd, 8, 0x8000_0000);
    set_in(&mut fd, add, big, 1);
    let _addout = give_output(&mut fd, add, 0x30, 8); // 8-byte output

    let (extra, _m) = RulePtrsubUndo::get_extra_offset(&fd, ptrsub);
    assert_eq!(extra, 0x8000_0000i64, "below the 63-bit sign position -> positive");
}

// =============================================================================
// 3. move_sign_bit_extraction — COPY chaining + INT_AND mask-subset guard
// =============================================================================

/// `moveSignBitExtraction` grows `testList` through COPY descendants
/// (ruleaction.cc:8264-8266) and, for shift amounts written by an INT_AND, only
/// accepts the shift if the AND is a *mask-superset* of the constant
/// (`constVn->getOffset() == (constVn->getOffset() & otherVn->getOffset())`,
/// 8254).  Build: first (4 bytes) -> COPY -> cpyout, and `cpyout s>> (31 & 0xff)`
/// where the shift amount is `INT_AND(#31, #0xff)`.  31 & 0xff == 31 so the guard
/// passes, the COPY pushes cpyout onto testList, and the shift's slot 0 is
/// redirected to `replace`.
#[test]
fn w5s3r6_move_sign_bit_through_copy_with_and_masked_amount() {
    let mut fd = build_fd();
    let fdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let first = give_output(&mut fd, fdef, 0x10, 4);
    let rdef = mk_op(&mut fd, 0, 0x84, OpCode::CPUI_COPY);
    let replace = give_output(&mut fd, rdef, 0x18, 4);

    // first -> COPY -> cpyout
    let copy = mk_op(&mut fd, 1, 0x88, OpCode::CPUI_COPY);
    set_in(&mut fd, copy, first, 0);
    let cpyout = give_output(&mut fd, copy, 0x20, 4);

    // shift amount = INT_AND(#31, #0xff)  (31 == 31 & 0xff -> subset guard passes)
    let andop = mk_op(&mut fd, 2, 0x8c, OpCode::CPUI_INT_AND);
    let c31 = mk_const(&mut fd, 4, 31);
    let cff = mk_const(&mut fd, 4, 0xff);
    set_in(&mut fd, andop, c31, 0);
    set_in(&mut fd, andop, cff, 1);
    let andout = give_output(&mut fd, andop, 0x28, 4);

    // cpyout s>> andout  -- the sign-bit extraction reached via the COPY chain
    let shift = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_SRIGHT);
    set_in(&mut fd, shift, cpyout, 0);
    set_in(&mut fd, shift, andout, 1);
    let _so = give_output(&mut fd, shift, 0x40, 4);

    RuleDivOpt::move_sign_bit_extraction(&mut fd, first, replace);
    assert_eq!(
        fd.obank().get(shift).unwrap().get_in(0),
        Some(replace),
        "sign-bit shift reached via COPY chain, with AND-masked amount, must be redirected"
    );
}

/// Negative companion: if the INT_AND is NOT a mask-superset (e.g. `30 & 0xff`,
/// where the shift constant 31 differs), or the amount is simply not the sign
/// position, the shift must be left untouched.  Here `INT_AND(#30, #0xff)` yields
/// the C++ `constVn = getIn(0) = #30`, and `30 != size*8-1 == 31` -> no redirect.
#[test]
fn w5s3r6_move_sign_bit_and_wrong_amount_no_redirect() {
    let mut fd = build_fd();
    let fdef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let first = give_output(&mut fd, fdef, 0x10, 4);
    let rdef = mk_op(&mut fd, 0, 0x84, OpCode::CPUI_COPY);
    let replace = give_output(&mut fd, rdef, 0x18, 4);

    let andop = mk_op(&mut fd, 2, 0x8c, OpCode::CPUI_INT_AND);
    let c30 = mk_const(&mut fd, 4, 30); // not the sign bit (31)
    let cff = mk_const(&mut fd, 4, 0xff);
    set_in(&mut fd, andop, c30, 0);
    set_in(&mut fd, andop, cff, 1);
    let andout = give_output(&mut fd, andop, 0x28, 4);

    let shift = mk_op(&mut fd, 2, 0x90, OpCode::CPUI_INT_SRIGHT);
    set_in(&mut fd, shift, first, 0);
    set_in(&mut fd, shift, andout, 1);
    let _so = give_output(&mut fd, shift, 0x40, 4);

    RuleDivOpt::move_sign_bit_extraction(&mut fd, first, replace);
    assert_eq!(
        fd.obank().get(shift).unwrap().get_in(0),
        Some(first),
        "AND amount 30 != sign position 31 -> shift must be untouched"
    );
}

// =============================================================================
// 4. RuleSubNormal — the `k = insize - c - outsize` fall-through then n==0 commit
// =============================================================================

/// A distinct committing path from the porter's `unnecessary_shift` test: take
/// `k+c+outsize > insize` so C++ enters the second `if`, the extension sub-path
/// guard fails (`n != k*8`), and we fall through to `k = insize - c - outsize`.
/// Then `c += k; n -= k*8` lands at `n == 0`, collapsing to a least-significant
/// SUBPIECE with input `a` and constant `c` (ruleaction.cc:7768-7800).
///
/// Construction: a = 8 bytes; `a >> n` with n = 8 (k = 1); SUBPIECE c = 3, out
/// size 4.
///   - guard1: n + 8c + 8*out = 8+24+32 = 64  < 8*insize = 64?  no -> not rejected.
///   - k+c+out = 1+3+4 = 8  >  insize 8?  no.  (Equality, so the second `if` is
///     NOT taken — verify the boundary is exclusive, matching C++ `>`.)
///   - c += k -> 4 ; n -= 8 -> 0  => collapse: op reads a, constant 4.
///
/// This pins the `>` (not `>=`) boundary on the `k+c+outsize > insize` test,
/// the exact off-by-one the hunt list calls out.
#[test]
fn w5s3r6_sub_normal_boundary_equal_collapses_not_extension() {
    let mut fd = build_fd();
    let mut rule = RuleSubNormal::new("g");

    let adef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    let shift = mk_op(&mut fd, 2, 0x84, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, a, 0);
    let n = mk_const(&mut fd, 4, 8); // n = 8 -> k = 1
    set_in(&mut fd, shift, n, 1);
    let shiftout = give_output(&mut fd, shift, 0x20, 8);

    let sub = mk_op(&mut fd, 2, 0x88, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, shiftout, 0);
    let c = mk_const(&mut fd, 4, 3);
    set_in(&mut fd, sub, c, 1);
    let _o = give_output(&mut fd, sub, 0x30, 4);

    assert_eq!(rule.apply_op(sub, &mut fd), 1, "n folds to 0 -> commit");
    assert_eq!(fd.obank().get(sub).unwrap().get_in(0), Some(a), "op now reads a directly");
    let new_c = fd.obank().get(sub).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(new_c).unwrap().is_constant());
    assert_eq!(
        fd.vbank().get(new_c).unwrap().get_offset(),
        4,
        "c becomes c+k = 3+1 = 4"
    );
    assert_eq!(fd.obank().get(sub).unwrap().code(), OpCode::CPUI_SUBPIECE);
}

/// The genuine `k = insize - c - outsize` fall-through: make `k+c+outsize >
/// insize` strictly, with the extension sub-path guard failing, so `k` is
/// shrunk and the result still collapses at `n==0`.
///   a = 8 bytes; n = 16 (k = 2); SUBPIECE c = 3, out size 4.
///   - guard1: n+8c+8out = 16+24+32 = 72 < 64? no.
///   - k+c+out = 2+3+4 = 9 > 8 -> enter second if.
///   - truncSize = insize-c-k = 8-3-2 = 3 ; n == k*8 (16==16) and truncSize 3 > 0
///     but popcount(3) == 2 != 1 -> extension guard FAILS -> k = 8-3-4 = 1.
///   - c += k -> 4 ; n -= 1*8 -> 8.  n != 0 and n(8) >= out*8(32)? no -> the
///     final newop/newUniqueOut path: that path is a W3/W6 seam -> returns 0.
///
/// So this exercises the shrink-k arithmetic and lands on the seam (no change),
/// which is the documented loss; the op must be left an unchanged SUBPIECE.
#[test]
fn w5s3r6_sub_normal_shrink_k_then_seam_no_change() {
    let mut fd = build_fd();
    let mut rule = RuleSubNormal::new("g");

    let adef = mk_op(&mut fd, 0, 0x80, OpCode::CPUI_COPY);
    let a = give_output(&mut fd, adef, 0x10, 8);
    let shift = mk_op(&mut fd, 2, 0x84, OpCode::CPUI_INT_RIGHT);
    set_in(&mut fd, shift, a, 0);
    let n = mk_const(&mut fd, 4, 16); // n = 16 -> k = 2
    set_in(&mut fd, shift, n, 1);
    let shiftout = give_output(&mut fd, shift, 0x20, 8);

    let sub = mk_op(&mut fd, 2, 0x88, OpCode::CPUI_SUBPIECE);
    set_in(&mut fd, sub, shiftout, 0);
    let c = mk_const(&mut fd, 4, 3);
    set_in(&mut fd, sub, c, 1);
    let subout = give_output(&mut fd, sub, 0x30, 4);

    assert_eq!(
        rule.apply_op(sub, &mut fd),
        0,
        "shrink-k path lands on the new-op/newUniqueOut seam -> no change"
    );
    // The op is structurally untouched: still SUBPIECE reading the shift output
    // with the original constant 3.
    assert_eq!(fd.obank().get(sub).unwrap().code(), OpCode::CPUI_SUBPIECE);
    assert_eq!(fd.obank().get(sub).unwrap().get_in(0), Some(shiftout));
    let still_c = fd.obank().get(sub).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(still_c).unwrap().get_offset(), 3);
    assert_eq!(fd.obank().get(sub).unwrap().get_out(), Some(subout));
}
