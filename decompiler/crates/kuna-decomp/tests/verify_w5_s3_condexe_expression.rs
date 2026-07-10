//! Adversarial verifier tests for item `w5-s3-condexe-expression`.
//!
//! Independent (verifier-authored) tests targeting the spots the hunt list
//! flagged as most fragile for the `condexe.{cc,hh}` / `expression.{cc,hh}`
//! port — chosen because the in-crate unit tests do NOT exercise them:
//!
//!   * `BooleanMatch::sameOpComplement` (`expression.cc:58-87`) — the two
//!     *wrapping/signedness corner* rejections: the unsigned `val2 == 0`
//!     corner (which depends on `val1 + 1` wrapping to 0 in `uintb`) and the
//!     signed `signbit_negative(val2) && !signbit_negative(val1)` corner.  A
//!     `checked_add`/missing-wrap or a transposed sign test silently flips a
//!     pair from `uncorrelated` to `complementary`.
//!   * `functionalEqualityLevel` (`expression.cc:625-705`) — the commutative
//!     "prefer the original ordering vs. swap res2" tail (lines 698-704), the
//!     riskiest slot-bookkeeping in the file, where the returned `res1/res2`
//!     pair identity is load-bearing for callers.
//!   * `BooleanMatch::evaluate` (`expression.cc:112-217`) — double `BOOL_NEGATE`
//!     (same/complementary flip applied twice) and the `op1 == 0` ordering
//!     (vn1 unwritten while vn2 is `BOOL_NEGATE`): the C++ defers the
//!     `op1 == 0 -> uncorrelated` early-out until *after* the vn2 negate check,
//!     so a `!base` vs `free` pair must still recurse and resolve.
//!
//! C++ oracle anchors are in the inline comments per assertion.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::expression::{
    boolean_match, functional_equality_level, BooleanMatch,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;

// -----------------------------------------------------------------------------
// Fixture (mirrors crate-internal expression/tests.rs harness)
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

fn const_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space(0).unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn opcode_flags(opc: OpCode) -> kuna_base::types::uint4 {
    match opc {
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_RETURN => pcodeop_flags::returns,
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR => {
            pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_BOOL_NEGATE | OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_SLESS => {
            pcodeop_flags::booloutput
        }
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR => pcodeop_flags::commutative,
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(size, Address::new(ram, off), dt(size))
}

fn mk_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(size, Address::new(ram, off), dt(size));
    let mut noop_thunk = |_b: &mut kuna_decomp::varnode::VarnodeBank,
                          _o: VarnodeId,
                          _n: VarnodeId|
     -> kuna_base::error::KunaResult<()> {
        panic!("mk_input: distinct addresses must not collide")
    };
    fd.vbank_mut().set_input(vn, &mut noop_thunk).unwrap()
}

fn mk_const(fd: &mut Funcdata, val: u64, size: int4) -> VarnodeId {
    let c = const_space(fd);
    fd.vbank_mut().create(size, Address::new(c, val), dt(size))
}

fn mk_out(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum: seq };
    let vn = {
        let mut noop_thunk = |_b: &mut kuna_decomp::varnode::VarnodeBank,
                              _o: VarnodeId,
                              _n: VarnodeId|
         -> kuna_base::error::KunaResult<()> {
            panic!("mk_out: distinct addresses must not collide (xref replace fired)")
        };
        fd.vbank_mut()
            .create_def(size, Address::new(ram, off), dt(size), def, &mut noop_thunk)
            .unwrap()
    };
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn wire_read(fd: &mut Funcdata, reader: OpId, vn: VarnodeId, slot: int4) {
    fd.obank_mut().get_mut(reader).unwrap().set_input(Some(vn), slot);
    fd.vbank_mut().add_descend(vn, reader).unwrap();
}

// =============================================================================
// 1. sameOpComplement — unsigned `val2 == 0` wrapping corner (expression.cc:77-78)
// =============================================================================

#[test]
fn w5_s3_sameop_complement_unsigned_val2_zero_corner_is_uncorrelated() {
    // bin1 = INT_LESS(x, #0)  ; bin2 = INT_LESS(#0xFFFFFFFF, x)
    //
    // constslot=1 (bin1.getIn(1) is the constant). After the constslot!=0 swap:
    //   val1 = bin2.getIn(0) = 0xFFFFFFFF
    //   val2 = bin1.getIn(1) = 0
    // C++ line 77: `if (val1 + 1 != val2) return false;` — val1+1 WRAPS to 0 in
    // uintb, so 0 == val2 and we pass the wrap check.  Then line 78:
    //   `if ((val2 == 0) && (opcode==CPUI_INT_LESS)) return false;`  <-- rejection
    // So the pair is UNCORRELATED, not complementary.  A missing `wrapping_add`
    // would panic in debug; a missing `val2==0` guard would wrongly say
    // complementary.
    let mut fd = build_fd();
    let x = mk_input(&mut fd, 0x100, 4); // shared by both INT_LESS
    let zero = mk_const(&mut fd, 0, 4);
    let allones = mk_const(&mut fd, 0xFFFF_FFFF, 4);

    let l1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS); // x < 0
    let o1 = mk_out(&mut fd, l1, 0x300, 1);
    wire_read(&mut fd, l1, x, 0);
    wire_read(&mut fd, l1, zero, 1);

    let l2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_LESS); // 0xFFFFFFFF < x
    let o2 = mk_out(&mut fd, l2, 0x400, 1);
    wire_read(&mut fd, l2, allones, 0);
    wire_read(&mut fd, l2, x, 1);

    assert_eq!(
        BooleanMatch::evaluate(o1, o2, 1, fd.vbank(), fd.obank()),
        boolean_match::uncorrelated,
        "unsigned val2==0 corner must be rejected (expression.cc:78)"
    );
}

// =============================================================================
// 2. sameOpComplement — signed sign-boundary corner (expression.cc:79-83)
// =============================================================================

#[test]
fn w5_s3_sameop_complement_signed_sign_boundary_corner_is_uncorrelated() {
    // bin1 = INT_SLESS(x, #0x80) ; bin2 = INT_SLESS(#0x7F, x)   (size 1 byte)
    //
    // constslot=1. After swap: val1 = bin2.getIn(0) = 0x7F, val2 = bin1.getIn(1)
    // = 0x80.  val1+1 == val2 (0x80) so the wrap check passes; val2 != 0.
    // Then the signed corner (line 81): signbit_negative(0x80, 1) is true
    // (0x80 has the high bit of a 1-byte value set) and signbit_negative(0x7F, 1)
    // is false, so `signbit_negative(val2) && !signbit_negative(val1)` => return
    // false (UNCORRELATED).  A transposed sign test (val1<->val2 swapped) would
    // wrongly classify this as complementary.
    let mut fd = build_fd();
    let x = mk_input(&mut fd, 0x100, 1); // shared, 1-byte
    let c80 = mk_const(&mut fd, 0x80, 1);
    let c7f = mk_const(&mut fd, 0x7F, 1);

    let l1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_SLESS); // x s< 0x80
    let o1 = mk_out(&mut fd, l1, 0x300, 1);
    wire_read(&mut fd, l1, x, 0);
    wire_read(&mut fd, l1, c80, 1);

    let l2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_SLESS); // 0x7F s< x
    let o2 = mk_out(&mut fd, l2, 0x400, 1);
    wire_read(&mut fd, l2, c7f, 0);
    wire_read(&mut fd, l2, x, 1);

    assert_eq!(
        BooleanMatch::evaluate(o1, o2, 1, fd.vbank(), fd.obank()),
        boolean_match::uncorrelated,
        "signed sign-boundary corner must be rejected (expression.cc:79-83)"
    );

    // Control: the in-range signed pair x s< 8  vs  7 s< x IS complementary
    // (val1=7, val2=8; neither is sign-negative as 1-byte) — confirms the corner
    // guard is not over-rejecting.
    let mut fd2 = build_fd();
    let y = mk_input(&mut fd2, 0x100, 1);
    let c8 = mk_const(&mut fd2, 8, 1);
    let c7 = mk_const(&mut fd2, 7, 1);
    let m1 = mk_op(&mut fd2, 2, 0x1000, OpCode::CPUI_INT_SLESS);
    let mo1 = mk_out(&mut fd2, m1, 0x300, 1);
    wire_read(&mut fd2, m1, y, 0);
    wire_read(&mut fd2, m1, c8, 1);
    let m2 = mk_op(&mut fd2, 2, 0x1004, OpCode::CPUI_INT_SLESS);
    let mo2 = mk_out(&mut fd2, m2, 0x400, 1);
    wire_read(&mut fd2, m2, c7, 0);
    wire_read(&mut fd2, m2, y, 1);
    assert_eq!(
        BooleanMatch::evaluate(mo1, mo2, 1, fd2.vbank(), fd2.obank()),
        boolean_match::complementary,
        "in-range signed complement must still be detected"
    );
}

// =============================================================================
// 3. functionalEqualityLevel — commutative "swap res2" tail (expression.cc:698-704)
// =============================================================================

#[test]
fn w5_s3_funceqlevel_commutative_swap_prefers_then_swaps_ordering() {
    // op1 = INT_ADD(p, q) ; op2 = INT_ADD(r, s) with p,q,r,s all distinct,
    // non-free (input) varnodes of equal size.  Every level0 comparison is
    // contingent (1):
    //   testval  = level0(p,r) = 1
    //   testval2 = level0(q,s) = 1   => unmatchsize = 2
    // commutative, so it tries the flip:
    //   comm1 = level0(p,s) = 1 ; comm2 = level0(q,r) = 1
    // Both are 1 (not 0, not <0), so we fall to the final branch (line 698):
    //   `if (unmatchsize == 2) return 2;`  -> the ORIGINAL ordering is kept,
    // res2 is NOT swapped.  So res2 must still be [r, s].
    let mut fd = build_fd();
    let p = mk_input(&mut fd, 0x100, 4);
    let q = mk_input(&mut fd, 0x108, 4);
    let r = mk_input(&mut fd, 0x110, 4);
    let s = mk_input(&mut fd, 0x118, 4);
    let add1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let o1 = mk_out(&mut fd, add1, 0x300, 4);
    wire_read(&mut fd, add1, p, 0);
    wire_read(&mut fd, add1, q, 1);
    let add2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let o2 = mk_out(&mut fd, add2, 0x400, 4);
    wire_read(&mut fd, add2, r, 0);
    wire_read(&mut fd, add2, s, 1);

    let mut res1 = [o1, o1];
    let mut res2 = [o2, o2];
    let n = functional_equality_level(o1, o2, &mut res1, &mut res2, fd.vbank(), fd.obank());
    assert_eq!(n, 2, "two contingent pairs survive (expression.cc:699-700)");
    // Original ordering preferred => res1=[p,q], res2=[r,s] (NOT swapped).
    assert_eq!(res1, [p, q], "res1 unchanged");
    assert_eq!(
        res2,
        [r, s],
        "unmatchsize==2 keeps the original res2 ordering (expression.cc:699 'Prefer the original ordering')"
    );
}

#[test]
fn w5_s3_funceqlevel_commutative_locks_first_pair_via_flip() {
    // op1 = INT_ADD(p, q) ; op2 = INT_ADD(q, s)  (q shared, p/s distinct inputs).
    //   testval  = level0(p,q) = 1 (both inputs, distinct) ... actually p!=q ->
    //              contingent 1.
    //   testval2 = level0(q,s) = 1
    //   unmatchsize = 2
    //   flip: comm1 = level0(p,s) = 1 ; comm2 = level0(q,q) = 0 (identity!)
    //   comm1<0||comm2<0 ? no. comm1==0 ? no. comm2==0 ? YES (line 694):
    //     `res2[0] = res2[1]; return 1;`  -> single leftover pair (p, s).
    let mut fd = build_fd();
    let p = mk_input(&mut fd, 0x100, 4);
    let q = mk_input(&mut fd, 0x108, 4); // shared between op1 slot1 and op2 slot0
    let s = mk_input(&mut fd, 0x118, 4);
    let add1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let o1 = mk_out(&mut fd, add1, 0x300, 4);
    wire_read(&mut fd, add1, p, 0);
    wire_read(&mut fd, add1, q, 1);
    let add2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let o2 = mk_out(&mut fd, add2, 0x400, 4);
    wire_read(&mut fd, add2, q, 0); // q in slot 0
    wire_read(&mut fd, add2, s, 1);

    let mut res1 = [o1, o1];
    let mut res2 = [o2, o2];
    let n = functional_equality_level(o1, o2, &mut res1, &mut res2, fd.vbank(), fd.obank());
    assert_eq!(n, 1, "comm2==0 locks one leftover pair (expression.cc:694-696)");
    // Leftover unmatch is (res1[0], res2[0]) == (p, s) after `res2[0]=res2[1]`.
    assert_eq!(res1[0], p, "res1[0] still the first op1 input");
    assert_eq!(res2[0], s, "res2[0] rewritten to res2[1] (s) per line 695");
}

// =============================================================================
// 4. BooleanMatch::evaluate — double BOOL_NEGATE + unwritten-vn1 ordering
// =============================================================================

#[test]
fn w5_s3_evaluate_double_negate_round_trips_to_same() {
    // a = INT_LESS(x,y) ; n1 = BOOL_NEGATE(a) ; n2 = BOOL_NEGATE(n1).
    // evaluate(a, n2): n2 is BOOL_NEGATE -> recurse evaluate(a, n1) then flip.
    //   evaluate(a, n1): n1 is BOOL_NEGATE -> recurse evaluate(a, a)=same, flip
    //   to complementary.  Outer flip: complementary -> same.
    // So a vs !!a is SAME.  This exercises the flip-applied-twice path; an
    // inverted flip (same<->complementary mixed up) would yield complementary.
    let mut fd = build_fd();
    let x = mk_input(&mut fd, 0x100, 4);
    let y = mk_input(&mut fd, 0x108, 4);
    let less = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS);
    let a = mk_out(&mut fd, less, 0x300, 1);
    wire_read(&mut fd, less, x, 0);
    wire_read(&mut fd, less, y, 1);

    let neg1 = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_BOOL_NEGATE);
    let n1 = mk_out(&mut fd, neg1, 0x400, 1);
    wire_read(&mut fd, neg1, a, 0);

    let neg2 = mk_op(&mut fd, 1, 0x1008, OpCode::CPUI_BOOL_NEGATE);
    let n2 = mk_out(&mut fd, neg2, 0x500, 1);
    wire_read(&mut fd, neg2, n1, 0);

    assert_eq!(
        BooleanMatch::evaluate(a, n2, 1, fd.vbank(), fd.obank()),
        boolean_match::same,
        "!!a == a (double-negate flips twice back to same)"
    );
    // And a vs !a (single negate) is complementary — sanity that the flip fires.
    assert_eq!(
        BooleanMatch::evaluate(a, n1, 1, fd.vbank(), fd.obank()),
        boolean_match::complementary,
    );
}

#[test]
fn w5_s3_evaluate_unwritten_vn1_with_negate_vn2_recurses() {
    // vn1 = free varnode f (NOT written) ; vn2 = BOOL_NEGATE(f).
    // C++ (expression.cc:130-133): vn1 unwritten -> op1=0, opc1=CPUI_MAX, but we
    // do NOT return yet.  vn2 is BOOL_NEGATE (line 137) -> recurse
    // evaluate(f, BOOL_NEGATE.getIn(0)) = evaluate(f, f) = same, then flip ->
    // complementary.  The `op1 == 0 -> uncorrelated` early-out (line 148) is
    // *bypassed* by the negate branch.  A port that returned uncorrelated as
    // soon as vn1 is unwritten (mis-ordering the op1==null check) would give the
    // wrong answer here.
    let mut fd = build_fd();
    let f = mk_input(&mut fd, 0x100, 1); // unwritten, but shared (negate reads it)
    let neg = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_BOOL_NEGATE);
    let nf = mk_out(&mut fd, neg, 0x300, 1);
    wire_read(&mut fd, neg, f, 0);

    assert_eq!(
        BooleanMatch::evaluate(f, nf, 1, fd.vbank(), fd.obank()),
        boolean_match::complementary,
        "unwritten vn1 vs BOOL_NEGATE(vn1) is complementary (expression.cc:130-145)"
    );

    // Mirror: vn1 unwritten and vn2 unwritten-non-negate => uncorrelated (the
    // `else return uncorrelated` at line 146-147 for vn2 not written).
    let g = mk_vn(&mut fd, 0x200, 1); // free, not written, single descend ok
    assert_eq!(
        BooleanMatch::evaluate(f, g, 1, fd.vbank(), fd.obank()),
        boolean_match::uncorrelated,
        "two unwritten non-negate booleans are uncorrelated"
    );
}
