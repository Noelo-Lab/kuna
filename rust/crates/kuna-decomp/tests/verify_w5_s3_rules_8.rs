//! Adversarial verifier tests for item `w5-s3-rules-8`
//! (`decompiler/cpp/ruleaction.cc` 9804-11032, the eighth `Rule` batch).
//!
//! These target the spots the hunt list flagged as most fragile and that the
//! porter's own inline tests did NOT cover:
//!
//!   * `RuleThreeWayCompare::applyOp` — the full Form-1 (`(z+z)-1`) transform:
//!     the `form` bit-encoding, the 24-case switch, and the avn/bvn operand
//!     ordering (`bvn=getIn(0)`, `avn=getIn(1)`).  The porter only tested the
//!     `test_compare_equivalence` static helper, never the end-to-end rewrite.
//!     This is the single riskiest untested transform in the batch.
//!   * `RuleThreeWayCompare::testCompareEquivalence` — the deliberate ASYMMETRY
//!     between the a1/a2 block (inner `else return -1`) and the b1/b2 block
//!     (outer `else return -1`), ruleaction.cc:10008-10035.  A transposed
//!     condition or a moved early-out is silent corruption.
//!   * `RuleLzcountShiftBool` — the `popcount(max_return) != 1` power-of-2 guard
//!     (max_return = 8*size), an integer-width/`popcount` surface.
//!   * `RuleOrCompare` — the W3 `newUniqueOut` seam partial-state: on the seam
//!     `Err` the rule returns 0, but the two fresh `eq_V`/`eq_W` ops have already
//!     been linked as descendants of V and W.  Pins that documented divergence.
//!
//! C++ oracle anchors are cited inline.  All graphs are hand-built on a real
//! `Funcdata` through the public IR API (mirroring the crate's W3 verifier
//! fixtures); no porter-private helpers are used.

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
use kuna_decomp::ruleaction_8::{
    RuleLzcountShiftBool, RuleOrCompare, RuleThreeWayCompare,
};
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

// --- fixtures (public-API mirror of the inline `tests` module) --------------

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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

/// Skeleton TypeOp matching the small set the rule resolver writes; flagless for
/// fixture-only opcodes (INT_OR/INT_ADD/INT_RIGHT/...).  Mirrors the inline
/// `resolve_typeop_for_test`, but the rule bodies re-resolve flags through their
/// own W6 seam when they *write* an opcode, so a flagless fixture op is fine.
fn typeop_for(opc: OpCode) -> TypeOp {
    TypeOp::new(opc, 0, format!("{opc:?}"))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, bl: BlockId, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.op_set_opcode(op, typeop_for(opc));
    fd.op_insert(op, bl, None);
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

fn mk_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let v = mk_vn(fd, off, size);
    fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
}

fn mk_const(fd: &mut Funcdata, size: int4, val: u64) -> VarnodeId {
    fd.new_constant(size, val)
}

fn set_output(fd: &mut Funcdata, op: OpId, off: u64, ct: Rc<Datatype>) -> VarnodeId {
    let r = ram(fd);
    let size = ct.get_size();
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(ct));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn code(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

fn in_of(fd: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    fd.obank().get(op).unwrap().get_in(slot)
}

// ===========================================================================
// F-A: RuleThreeWayCompare full Form-1 transform (UNTESTED by the porter).
//
// Builds the canonical three-way `X = zext(a < b) + zext(a <= b) - 1` (Form 1:
// `(z + z) - 1`, ruleaction.cc:10058-10073) and the secondary comparison
// `X == 0` (INT_EQUAL with the three-way on slot 1).
//
// Encoding walk (ruleaction.cc:10164-10208), oracle:
//   constSlot: in0 is the threeway (not const) -> constSlot=1, val=0.
//   val=0 -> form=1 (val+1). isPartial=false. form<<=1 -> 2; constSlot==1 ->
//   form=3. lessform=INT_LESS. form<<=2 -> 12; op is INT_EQUAL -> form+=2 -> 14.
//   Case 14 ("threeway == 0 => a == b"): opSetOpcode = INT_EQUAL (integer form),
//   inputs (avn, bvn).  avn=lessop->getIn(1)=b, bvn=lessop->getIn(0)=a.
//   => result is INT_EQUAL(b_operand_at_slot0=a?, ...).  Concretely getIn(0)=avn=b,
//   getIn(1)=bvn=a.
// ===========================================================================

/// Build `zext(less_out)` of a `a OP b` comparison; returns (cmp_op, zext_out).
fn build_zext_cmp(
    fd: &mut Funcdata,
    bl: BlockId,
    a: VarnodeId,
    b: VarnodeId,
    opc: OpCode,
    base_off: u64,
) -> (OpId, VarnodeId) {
    let cmp = mk_op(fd, bl, 2, base_off, opc);
    wire(fd, a, cmp, 0);
    wire(fd, b, cmp, 1);
    let cmp_out = set_output(fd, cmp, base_off + 0x40, unk(1));
    let zext = mk_op(fd, bl, 1, base_off + 1, OpCode::CPUI_INT_ZEXT);
    wire(fd, cmp_out, zext, 0);
    let zext_out = set_output(fd, zext, base_off + 0x80, unk(4));
    (cmp, zext_out)
}

#[test]
fn w5s3r8_threeway_form1_eq_zero_collapses_to_int_equal() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x20, 4);

    // z1 = zext(a < b); z2 = zext(a <= b)
    let (less_op, z1) = build_zext_cmp(&mut fd, bl, a, b, OpCode::CPUI_INT_LESS, 0x100);
    let (_lesseq_op, z2) = build_zext_cmp(&mut fd, bl, a, b, OpCode::CPUI_INT_LESSEQUAL, 0x200);

    // inner = z1 + z2
    let inner = mk_op(&mut fd, bl, 2, 0x300, OpCode::CPUI_INT_ADD);
    wire(&mut fd, z1, inner, 0);
    wire(&mut fd, z2, inner, 1);
    let inner_out = set_output(&mut fd, inner, 0x340, unk(4));

    // threeway = inner + (-1)  [Form 1: (z+z) - 1, the const is -1 = mask]
    let minus1 = mk_const(&mut fd, 4, 0xffff_ffff);
    let three = mk_op(&mut fd, bl, 2, 0x380, OpCode::CPUI_INT_ADD);
    wire(&mut fd, inner_out, three, 0);
    wire(&mut fd, minus1, three, 1);
    let three_out = set_output(&mut fd, three, 0x3c0, unk(4));

    // secondary: op = (threeway == 0), threeway on slot 0, 0 on slot 1.
    // -> in0 not const => constSlot becomes 1, val = 0 (the 0 constant).
    let zero = mk_const(&mut fd, 4, 0);
    let op = mk_op(&mut fd, bl, 2, 0x400, OpCode::CPUI_INT_EQUAL);
    wire(&mut fd, three_out, op, 0);
    wire(&mut fd, zero, op, 1);
    let _ = set_output(&mut fd, op, 0x440, unk(1));

    let r = RuleThreeWayCompare.apply_op(op, &mut fd);
    assert_eq!(r, 1, "Form-1 threeway==0 must collapse");
    // Case 14 => INT_EQUAL, inputs (avn=b, bvn=a)  [avn=lessop.getIn(1)=b]
    assert_eq!(code(&fd, op), OpCode::CPUI_INT_EQUAL, "collapses to a==b");
    assert_eq!(in_of(&fd, op, 0), Some(b), "avn (lessop.getIn(1)) on slot 0");
    assert_eq!(in_of(&fd, op, 1), Some(a), "bvn (lessop.getIn(0)) on slot 1");
    // Sanity: the lessop fed the encoding (it is the `<` op, not the `<=`).
    assert_eq!(code(&fd, less_op), OpCode::CPUI_INT_LESS);
}

#[test]
fn w5s3r8_threeway_form1_sless_zero_collapses_to_a_lt_b() {
    // Same Form-1 threeway, but secondary is `threeway s< 0` (INT_SLESS).
    // Encoding: constSlot in0=threeway not const -> constSlot=1, val=0 -> form=1.
    // form<<=1 -> 2; constSlot==1 -> 3; lessform=INT_LESS; form<<=2 -> 12; op is
    // INT_SLESS (neither SLESSEQUAL/EQUAL/NOTEQUAL) -> form unchanged = 12.
    // Case 12 ("threeway s< 0 => a < b"): opcode=lessform=INT_LESS, (avn, bvn).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x20, 4);
    let (_less, z1) = build_zext_cmp(&mut fd, bl, a, b, OpCode::CPUI_INT_LESS, 0x100);
    let (_le, z2) = build_zext_cmp(&mut fd, bl, a, b, OpCode::CPUI_INT_LESSEQUAL, 0x200);
    let inner = mk_op(&mut fd, bl, 2, 0x300, OpCode::CPUI_INT_ADD);
    wire(&mut fd, z1, inner, 0);
    wire(&mut fd, z2, inner, 1);
    let inner_out = set_output(&mut fd, inner, 0x340, unk(4));
    let minus1 = mk_const(&mut fd, 4, 0xffff_ffff);
    let three = mk_op(&mut fd, bl, 2, 0x380, OpCode::CPUI_INT_ADD);
    wire(&mut fd, inner_out, three, 0);
    wire(&mut fd, minus1, three, 1);
    let three_out = set_output(&mut fd, three, 0x3c0, unk(4));
    let zero = mk_const(&mut fd, 4, 0);
    let op = mk_op(&mut fd, bl, 2, 0x400, OpCode::CPUI_INT_SLESS);
    wire(&mut fd, three_out, op, 0);
    wire(&mut fd, zero, op, 1);
    let _ = set_output(&mut fd, op, 0x440, unk(1));

    let r = RuleThreeWayCompare.apply_op(op, &mut fd);
    assert_eq!(r, 1);
    // a < b  => INT_LESS(avn=b on slot0, bvn=a on slot1)
    assert_eq!(code(&fd, op), OpCode::CPUI_INT_LESS, "case 12 => a < b");
    assert_eq!(in_of(&fd, op, 0), Some(b));
    assert_eq!(in_of(&fd, op, 1), Some(a));
}

// ===========================================================================
// F-B: testCompareEquivalence — the a/b block ASYMMETRY (ruleaction.cc:10008-35).
//
// a1/a2 block: the `else return -1` is INNER (only when offsets differ &&
//   twoLessThan && none of the +1 relations hold).
// b1/b2 block: the `else return -1` is OUTER — it fires whenever the outer
//   `(b1off != b2off) && twoLessThan` is FALSE, i.e. ALSO when b1==b2 or when
//   twoLessThan is already false.
//
// The trap: with LESS+LESSEQUAL (twoLessThan=false), a1==a2 but b1!=b2
// constants, the b-block's outer condition is false (twoLessThan==false) =>
// the OUTER else fires => return -1.  A port that mirrored the a-block (inner
// else) would instead fall through and return 0.  This pins the -1.
// ===========================================================================

#[test]
fn w5s3r8_test_compare_equivalence_bblock_outer_else_returns_minus1() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_input(&mut fd, 0x10, 4);
    // a1==a2 (same var), b1 != b2 constants. LESS + LESSEQUAL => twoLessThan=false.
    let b1 = mk_const(&mut fd, 4, 5);
    let b2 = mk_const(&mut fd, 4, 9); // not adjacent; but twoLessThan already false
    let less = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_LESS);
    wire(&mut fd, a, less, 0);
    wire(&mut fd, b1, less, 1);
    let lesseq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_LESSEQUAL);
    wire(&mut fd, a, lesseq, 0);
    wire(&mut fd, b2, lesseq, 1);
    // twoLessThan=false; a1==a2 skip; b1!=b2 -> outer cond (false&&...) false ->
    // OUTER else -> return -1.  (NOT 0.)
    assert_eq!(
        RuleThreeWayCompare::test_compare_equivalence(&fd, less, lesseq),
        -1,
        "b-block OUTER else fires when twoLessThan is already false and b1!=b2"
    );
}

#[test]
fn w5s3r8_test_compare_equivalence_ablock_nonadjacent_const_returns_minus1() {
    // Mirror trap on the a-block: two INT_LESS (twoLessThan=true), a1,a2 constants
    // that are NOT +/-1 adjacent => inner `else return -1`.  b1==b2 (same var) so
    // the b-block outer cond is false but its OUTER else also returns -1; to keep
    // the a-block as the decisive branch we still expect -1 either way, but the
    // a-path is reached first.  This guards the a-block inner else.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a1 = mk_const(&mut fd, 4, 5);
    let a2 = mk_const(&mut fd, 4, 20); // far apart -> not adjacent
    let b = mk_input(&mut fd, 0x30, 4);
    let less = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_LESS);
    wire(&mut fd, a1, less, 0);
    wire(&mut fd, b, less, 1);
    let lesseq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_LESS); // both LESS => twoLessThan
    wire(&mut fd, a2, lesseq, 0);
    wire(&mut fd, b, lesseq, 1);
    assert_eq!(
        RuleThreeWayCompare::test_compare_equivalence(&fd, less, lesseq),
        -1,
        "a-block inner else fires for non-adjacent constants under twoLessThan"
    );
}

#[test]
fn w5s3r8_test_compare_equivalence_ablock_adjacent_swaps_res1() {
    // a1+1 == a2 under twoLessThan => a-block sets twoLessThan=false AND res=1
    // (the swap signal), ruleaction.cc:10014-10017.  b1==b2 so the b-block is
    // skipped entirely (b1==b2 means the outer `if(b1!=b2)` is false => no else).
    // Final: twoLessThan=false => return res = 1.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a1 = mk_const(&mut fd, 4, 7);
    let a2 = mk_const(&mut fd, 4, 8); // a1 + 1 == a2  => res=1 branch
    let b = mk_input(&mut fd, 0x30, 4);
    let less = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_LESS);
    wire(&mut fd, a1, less, 0);
    wire(&mut fd, b, less, 1);
    let lesseq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_LESS);
    wire(&mut fd, a2, lesseq, 0);
    wire(&mut fd, b, lesseq, 1);
    assert_eq!(
        RuleThreeWayCompare::test_compare_equivalence(&fd, less, lesseq),
        1,
        "a1+1==a2 under twoLessThan must signal a swap (res=1)"
    );
}

// ===========================================================================
// F-C: RuleLzcountShiftBool power-of-2 guard (ruleaction.cc:10686-10693).
//   max_return = 8 * getIn(0)->getSize().  popcount(max_return) != 1 => return 0
//   with NO graph mutation.  size 3 -> 24 (popcount 2) blocks; size 4 -> 32
//   (popcount 1) proceeds to the per-reader scan.
// ===========================================================================

#[test]
fn w5s3r8_lzcount_non_power_of_two_size_blocks_without_mutation() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // size-3 input => max_return = 24, popcount(24)=2 => guard blocks.
    let x = mk_input(&mut fd, 0x10, 3);
    let lz = mk_op(&mut fd, bl, 1, 0x100, OpCode::CPUI_LZCOUNT);
    wire(&mut fd, x, lz, 0);
    let lz_out = set_output(&mut fd, lz, 0x40, unk(3));
    // a matching shift reader (would otherwise fire): (lzout >> 4)
    let four = mk_const(&mut fd, 4, 4);
    let shr = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_RIGHT);
    wire(&mut fd, lz_out, shr, 0);
    wire(&mut fd, four, shr, 1);
    let _ = set_output(&mut fd, shr, 0x80, unk(3));

    assert_eq!(RuleLzcountShiftBool.apply_op(lz, &mut fd), 0, "size 3 -> popcount(24)!=1 blocks");
    // The shift reader is untouched.
    assert_eq!(code(&fd, shr), OpCode::CPUI_INT_RIGHT);
}

#[test]
fn w5s3r8_lzcount_power_of_two_completes_int_equal_rewrite() {
    // size-4 input => max_return=32 (popcount 1) passes the guard; reader
    // `lzout >> 5` matches `32>>5==1`.  Since w4x-flow-linkage filled
    // newUniqueOut/opSetOutput, the FULL C++ transform now runs
    // (ruleaction.cc lzcount body): a fresh INT_EQUAL(x, 0) is inserted before
    // the shift, the shift loses its constant input and becomes INT_ZEXT (its
    // output is size 4 != 1) reading the new unique bool.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10, 4);
    let lz = mk_op(&mut fd, bl, 1, 0x100, OpCode::CPUI_LZCOUNT);
    wire(&mut fd, x, lz, 0);
    let lz_out = set_output(&mut fd, lz, 0x40, unk(4));
    let five = mk_const(&mut fd, 4, 5);
    let shr = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_RIGHT);
    wire(&mut fd, lz_out, shr, 0);
    wire(&mut fd, five, shr, 1);
    let _ = set_output(&mut fd, shr, 0x80, unk(4));

    assert_eq!(RuleLzcountShiftBool.apply_op(lz, &mut fd), 1, "transform completes since w4x");
    assert_eq!(code(&fd, shr), OpCode::CPUI_INT_ZEXT, "size-4 output -> INT_ZEXT (not COPY)");
    let shr_rec = fd.obank().get(shr).expect("shift survives as zext");
    assert_eq!(shr_rec.num_input(), 1, "constant shift-amount input removed");
    let bool_in = shr_rec.get_in(0).expect("zext input");
    let bool_rec = fd.vbank().get(bool_in).expect("unique bool varnode");
    assert_eq!(bool_rec.get_size(), 1);
    let eq = bool_rec.get_def().expect("defined by the fresh INT_EQUAL");
    let eq_rec = fd.obank().get(eq).expect("eq op");
    assert_eq!(eq_rec.code(), OpCode::CPUI_INT_EQUAL);
    assert_eq!(eq_rec.get_in(0), Some(x), "compares the original lzcount operand");
    let z = fd.vbank().get(eq_rec.get_in(1).expect("zero")).expect("const");
    assert!(z.is_constant() && z.get_offset() == 0 && z.get_size() == 4);
}

// ===========================================================================
// F-D: RuleOrCompare — formerly a partial-state seam pin; since
//   w4x-flow-linkage filled newUniqueOut, the C++ path COMPLETES
//   (ruleaction.cc:10829): INT_EQUAL(V|W, 0) -> BOOL_AND(INT_EQUAL(V,0),
//   INT_EQUAL(W,0)).  V/W each gain exactly one well-formed descendant (the
//   fresh comparison), not an orphan.
// ===========================================================================

#[test]
fn w5s3r8_orcompare_completes_and_links_v_w_descendants() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_input(&mut fd, 0x10, 4);
    let w = mk_input(&mut fd, 0x20, 4);
    let orop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_OR);
    wire(&mut fd, v, orop, 0);
    wire(&mut fd, w, orop, 1);
    let orout = set_output(&mut fd, orop, 0x30, unk(4));
    let zero = mk_const(&mut fd, 4, 0);
    let eq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_EQUAL);
    wire(&mut fd, orout, eq, 0);
    wire(&mut fd, zero, eq, 1);
    let _ = set_output(&mut fd, eq, 0x40, unk(1));

    assert_eq!(fd.vbank().get(v).unwrap().num_descend(), 1);
    assert_eq!(fd.vbank().get(w).unwrap().num_descend(), 1);

    assert_eq!(RuleOrCompare.apply_op(orop, &mut fd), 1, "rewrite completes since w4x");
    // The OR op itself is untouched; its READER became BOOL_AND.
    assert_eq!(code(&fd, orop), OpCode::CPUI_INT_OR);
    assert_eq!(code(&fd, eq), OpCode::CPUI_BOOL_AND);

    // V and W each gained exactly one descendant — a fresh, fully-linked
    // INT_EQUAL (with a unique bool output feeding the BOOL_AND), not an orphan.
    for (base, slot) in [(v, 0i32), (w, 1i32)] {
        assert_eq!(fd.vbank().get(base).unwrap().num_descend(), 2);
        let and_in = fd.obank().get(eq).unwrap().get_in(slot).expect("bool_and input");
        let and_in_rec = fd.vbank().get(and_in).expect("unique bool");
        assert_eq!(and_in_rec.get_size(), 1);
        let def = and_in_rec.get_def().expect("defined");
        let def_rec = fd.obank().get(def).expect("fresh INT_EQUAL");
        assert_eq!(def_rec.code(), OpCode::CPUI_INT_EQUAL);
        assert_eq!(def_rec.get_in(0), Some(base));
    }
}
