//! Adversarial verifier tests for item `w5-s3-rules-4`
//! (ruleaction.cc:4293-5526, RuleLoadVarnode..RuleCondNegate).
//!
//! These target the hunt-list spots most fragile for this batch:
//!
//!   * **Wrapping / shift-overflow (the bug).** Several rules shift a 64-bit
//!     `uintb` by a *data-derived* count taken straight from a constant operand
//!     (`sa = (int4)cvn->getOffset()`).  Upstream C++ uses raw `>>=`/`<<=`,
//!     which on the x86 target masks the shift count to `& 63`; ADR 0003 makes
//!     the wrapping-shift helpers (`wshl`/`wshr`) *mandatory* for exactly these
//!     spots.  The port uses bare `nzm >>= sa as u32`, which **panics** under
//!     the debug (overflow-checked) build `cargo test` runs in, for any shift
//!     constant `>= 64` (or a constant whose `(int4)` cast is negative).  The
//!     `RuleShiftAnd` tests below build a legal `(V & m) >> 64` p-code op and
//!     assert the rule returns the C++ result (0, AND not bypassed) *without
//!     panicking* — under the bug they abort.
//!
//!   * **Registration group provenance.**  `coreaction.cc:5939-5940` registers
//!     `RuleLoadVarnode`/`RuleStoreVarnode` with the group **`"stackvars"`**, not
//!     `"analysis"` (the other 17 are `"analysis"`, 5845-5876).  The port's
//!     `clone_rule` hardcodes `contains("analysis")` for all 19 and `specs()`
//!     tags both with `group:"analysis"` — so these two rules clone/route under
//!     the wrong group.  Asserted directly.
//!
//! C++ oracle: `decompiler/cpp/ruleaction.cc` (RuleShiftAnd 4935-4977,
//! RuleSubZext 5059-5120) and `decompiler/cpp/coreaction.cc` (5845-5876,
//! 5939-5940).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::action::{ActionGroupList, Rule};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_4::{
    RuleHumptyOr, RuleLoadVarnode, RuleShiftAnd, RuleStoreVarnode, RuleSubExtComm, RuleSubZext,
};
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;
use kuna_num::opcodes::OpCode;

// ---------------------------------------------------------------------------
// Fixtures (public-API mirror of the in-crate ruleaction_4/tests.rs helpers)
// ---------------------------------------------------------------------------

fn build_fd() -> Funcdata {
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
    let glb = Rc::new(Architecture::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn set_opcode(fd: &mut Funcdata, op: OpId, opc: OpCode) {
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    set_opcode(fd, op, opc);
    op
}

/// A non-free processor register Varnode (a heritaged input).
fn mk_vn(fd: &mut Funcdata, s: int4, off: u64) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.vbank_mut().create(s, Address::new(r, off), dt(s));
    fd.vbank_mut().set_input(vn, &mut |_, _, _| Ok(())).expect("set_input")
}

/// Fresh unique output of `op` (public-API mirror of `new_unique_out`).
fn mk_out(fd: &mut Funcdata, s: int4, op: OpId) -> VarnodeId {
    let seqnum = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum };
    let vn = fd
        .vbank_mut()
        .create_def_unique(s, dt(s), def, &mut |_, _, _| Ok(()))
        .expect("create_def_unique");
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

/// Build `(invn & mask_const) <shiftop> sa_const`, return the shift op.
/// `mask_val` is the AND mask constant; `sa_val` is the shift-amount constant.
/// `invn` is a size-`vsize` non-free register.  The shift op gets no output
/// (RuleShiftAnd never reads it).
fn build_shift_and(
    fd: &mut Funcdata,
    shift_opc: OpCode,
    vsize: int4,
    mask_val: u64,
    sa_val: u64,
) -> OpId {
    let bl = mk_block(fd);
    let invn = mk_vn(fd, vsize, 0x10);
    let mask = fd.new_constant(vsize, mask_val);
    // and: invn & mask  (output size vsize)
    let andop = mk_op(fd, 2, 0x20, OpCode::CPUI_INT_AND);
    fd.op_set_input(andop, invn, 0).unwrap();
    fd.op_set_input(andop, mask, 1).unwrap();
    fd.op_insert(andop, bl, None);
    let andout = mk_out(fd, vsize, andop);
    // shift: andout <shift_opc> sa
    let sa = fd.new_constant(4, sa_val);
    let shop = mk_op(fd, 2, 0x30, shift_opc);
    fd.op_set_input(shop, andout, 0).unwrap();
    fd.op_set_input(shop, sa, 1).unwrap();
    fd.op_insert(shop, bl, None);
    let _ = bl;
    shop
}

// ---------------------------------------------------------------------------
// F1 (BUG): RuleShiftAnd shift count >= 64 — C++ x86-masks; the port panics
// ---------------------------------------------------------------------------

// `(V & 0xff) >> 64`.  On the x86 target the C++ `nzm >>= sa` / `mask >>= sa`
// mask the count to `64 & 63 == 0`, leaving nzm/mask unchanged; with a size-1
// processor invn (nzm == u64::MAX) and mask 0xff, `(mask & nzm) == 0xff != MAX`
// so the rule declines (returns 0) — no crash.  The port shifts by `64u32`,
// which panics with overflow under the debug build `cargo test` uses.
#[test]
fn shiftand_right_shift_64_matches_cpp_no_panic() {
    let mut fd = build_fd();
    let shop = build_shift_and(&mut fd, OpCode::CPUI_INT_RIGHT, 1, 0xff, 64);
    // C++ oracle: returns 0 (AND not bypassed). Port: panics here.
    let res = RuleShiftAnd::new().apply_op(shop, &mut fd);
    assert_eq!(res, 0, "C++ x86-masked shift-by-64 declines; rule must not bypass");
}

// `(V & 0xff) << 64` on a size-1 value.  C++: `nzm <<= 64` -> nzm unchanged
// (x86 mask), then `nzm &= fullmask(0xff)`, `mask <<= 64` -> mask unchanged,
// `mask &= 0xff`.  With nzm==MAX&0xff==0xff and mask==0xff, `(mask&nzm)==nzm`
// so the AND IS bypassed (returns 1).  The port panics on `<< 64u32`.
#[test]
fn shiftand_left_shift_64_matches_cpp_no_panic() {
    let mut fd = build_fd();
    let shop = build_shift_and(&mut fd, OpCode::CPUI_INT_LEFT, 1, 0xff, 64);
    let res = RuleShiftAnd::new().apply_op(shop, &mut fd);
    // C++ oracle: 1 (AND bypassed, slot 0 now reads invn directly).
    assert_eq!(res, 1, "C++ x86-masked shift-by-64 LEFT bypasses the AND");
}

// A shift constant whose `(int4)` cast is negative: offset 0x80000000 -> int4
// -2147483648.  C++ `nzm >>= sa` with a negative `sa` is the same UB resolved on
// x86 to `sa & 63 == 0`.  The port computes `sa as u32 == 0x80000000` and shifts
// by that -> panic.  (Exercises the `(int4)` signedness path feeding the shift.)
#[test]
fn shiftand_negative_int4_shift_count_no_panic() {
    let mut fd = build_fd();
    let shop = build_shift_and(&mut fd, OpCode::CPUI_INT_RIGHT, 1, 0xff, 0x8000_0000);
    let res = RuleShiftAnd::new().apply_op(shop, &mut fd);
    // 0x80000000 & 63 == 0 -> nzm/mask unchanged -> declines (as the >>64 case).
    assert_eq!(res, 0, "negative-int4 shift count must x86-mask, not panic");
}

// ---------------------------------------------------------------------------
// F2 (MAJOR): RuleLoadVarnode / RuleStoreVarnode register under "stackvars"
// ---------------------------------------------------------------------------

// coreaction.cc:5939-5940 -> these two rules carry group "stackvars"; their C++
// `clone` gates on `grouplist.contains(getGroup())` == contains("stackvars").
// The port hardcodes contains("analysis"), so it clones under the wrong group
// (and `specs()` tags them "analysis").  Under a faithful port, asking to clone
// into a "stackvars"-only grouplist must succeed and into an "analysis"-only
// grouplist must decline.  The current port does the opposite.
#[test]
fn loadstore_varnode_clone_under_stackvars_group() {
    let stackvars = ActionGroupList::from_names(["stackvars"]);
    let analysis = ActionGroupList::from_names(["analysis"]);

    // C++ oracle: getGroup()=="stackvars" -> clones iff list has "stackvars".
    assert!(
        RuleLoadVarnode::new().clone_rule(&stackvars).is_some(),
        "RuleLoadVarnode is a stackvars-group rule (coreaction.cc:5939); must clone there"
    );
    assert!(
        RuleLoadVarnode::new().clone_rule(&analysis).is_none(),
        "RuleLoadVarnode must NOT clone into an analysis-only grouplist"
    );
    assert!(
        RuleStoreVarnode::new().clone_rule(&stackvars).is_some(),
        "RuleStoreVarnode is a stackvars-group rule (coreaction.cc:5940); must clone there"
    );
    assert!(
        RuleStoreVarnode::new().clone_rule(&analysis).is_none(),
        "RuleStoreVarnode must NOT clone into an analysis-only grouplist"
    );
}

// ---------------------------------------------------------------------------
// Positive control: RuleSubExtComm sub-misses-extended-bits corner (no bug)
// ---------------------------------------------------------------------------

// `sub( zext(V:2 -> :8), 0 )` with out-size 2 <= invn-size 2: the SUBPIECE never
// hits the extended bits, so C++ bypasses to read invn and, since invn-size ==
// out-size, rewrites to COPY (ruleaction.cc:4435-4442).  Confirms the harness +
// a non-buggy rule.
#[test]
fn subextcomm_sub_misses_ext_bits_to_copy() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 2, 0x10);
    let zextop = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(zextop, v, 0).unwrap();
    fd.op_insert(zextop, bl, None);
    let zout = mk_out(&mut fd, 8, zextop);
    // sub(zout, 0), out-size 2
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, zout, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let _ = mk_out(&mut fd, 2, subop);

    let res = RuleSubExtComm::new().apply_op(subop, &mut fd);
    assert_eq!(res, 1);
    // out-size(2) + subcut(0) <= invn-size(2) -> bypass; invn-size==out-size -> COPY.
    assert_eq!(fd.obank().get(subop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(subop).unwrap().get_in(0), Some(v));
}

// ===========================================================================
// ROUND 2 — independent re-verification of the F1 repair, second fix site,
// plus an adversarial slot-swap probe round 1 never touched.
// ===========================================================================

// ---------------------------------------------------------------------------
// R2-A (F1 second site): RuleSubZext INT_RIGHT branch shifts a `uintb` mask by a
// data-derived right-shift constant — `val >>= sa` at ruleaction.cc:5107, ported
// at ruleaction_4.rs:1494.  Round 1's three shift-overflow tests only covered
// RuleShiftAnd (ruleaction_4.rs:1212-1216); this is the *distinct* F1 fix site.
// Pattern: `zext( sub(V,0) >> 64 )`.  On the x86 target C++ masks the count to
// `64 & 63 == 0`, so `val = calc_mask(midsize)` is unchanged and the rule fires
// (returns 1), folding the SUBPIECE+RIGHT+ZEXT into a shifted INT_AND with the
// full truncated mask.  Under a bare `val >> 64u32` the debug build panics.
#[test]
fn subzext_right_shift_64_matches_cpp_no_panic() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // base size 8 == zext out size; sub(base,0) size 4 (midsize, mask 0xffffffff).
    let basevn = mk_vn(&mut fd, 8, 0x10);
    let off0 = fd.new_constant(4, 0);
    let subop = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_SUBPIECE);
    fd.op_set_input(subop, basevn, 0).unwrap();
    fd.op_set_input(subop, off0, 1).unwrap();
    fd.op_insert(subop, bl, None);
    let subout = mk_out(&mut fd, 4, subop);
    // (subout) >> 64  -- the legal-but-oversized p-code shift count.
    let d = fd.new_constant(4, 64);
    let shop = mk_op(&mut fd, 2, 0x28, OpCode::CPUI_INT_RIGHT);
    fd.op_set_input(shop, subout, 0).unwrap();
    fd.op_set_input(shop, d, 1).unwrap();
    fd.op_insert(shop, bl, None);
    let shout = mk_out(&mut fd, 4, shop);
    // zext(shout) out size 8 == base size.
    let zop = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(zop, shout, 0).unwrap();
    fd.op_insert(zop, bl, None);
    let _zout = mk_out(&mut fd, 8, zop);

    // C++ oracle: returns 1; op becomes INT_AND; the shift operand of the inner
    // INT_RIGHT becomes sa = (64 & 63) + 0*8 == 64; the AND mask is the unshifted
    // calc_mask(midsize=4) == 0xffffffff (since `64 & 63 == 0` masks nothing).
    let res = RuleSubZext::new().apply_op(zop, &mut fd);
    assert_eq!(res, 1, "x86-masked >>64 must fold (port must use wrapping shift, not panic)");
    assert_eq!(fd.obank().get(zop).unwrap().code(), OpCode::CPUI_INT_AND);
    let maskc = fd.obank().get(zop).unwrap().get_in(1).unwrap();
    assert_eq!(
        fd.vbank().get(maskc).unwrap().get_offset(),
        0xffff_ffff,
        "val >>= (64&63=0) leaves the full truncated mask"
    );
    // The inner shift's count was rewritten to the combined amount sa = 64.
    let new_sa = fd.obank().get(shop).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(new_sa).unwrap().get_offset(), 64);
}

// ---------------------------------------------------------------------------
// R2-B (F1 third route): RuleShiftAnd reached via INT_MULT, whose multiplier is a
// power of two so `leastsigbit_set` yields a *large* shift amount (40), which then
// feeds `nzm <<= sa` / `mask <<= sa` (the INT_LEFT treatment) at
// ruleaction_4.rs:1215-1216.  Round 1 drove INT_LEFT directly with sa==64; the
// MULT path (sa from leastsigbit_set, sa < 64 but non-trivial) is its own
// arithmetic provenance.  invn size 8 (fullmask MAX, nz==MAX); mask 0xffff,
// multiplier 1<<40.  C++: `nzm <<= 40` (MAX<<40), `mask <<= 40` (0xffff<<40),
// `nzm &= MAX`, `mask &= MAX`; `(mask & nzm) == mask == nzm`? nzm = MAX<<40 has
// the top 24 bits set; mask = 0xffff<<40 = bits[40..56].  `(mask & nzm) == mask`
// but nzm != mask, so `(mask & nzm) != nzm` -> returns 0 (AND NOT bypassed).
#[test]
fn shiftand_mult_power_of_two_large_sa_no_panic() {
    let mut fd = build_fd();
    let shop = build_shift_and(&mut fd, OpCode::CPUI_INT_MULT, 8, 0xffff, 1u64 << 40);
    // C++ oracle: leastsigbit_set(1<<40)=40, testval==offset ok -> treat as LEFT.
    // nzm(MAX)<<40 has bits[40..64]; mask(0xffff)<<40 has bits[40..56].
    // (mask & nzm) == mask != nzm -> declines.  Must not panic on `<< 40`.
    let res = RuleShiftAnd::new().apply_op(shop, &mut fd);
    assert_eq!(res, 0, "MULT->LEFT with sa=40: (mask&nzm)!=nzm declines, no panic");
}

// A companion guard on the same MULT route: a non-power-of-two multiplier must be
// rejected by the `testval != cvn->getOffset()` check (ruleaction.cc:4957-4959),
// proving the MULT branch's leastsigbit/testval gate is transcribed, not skipped.
#[test]
fn shiftand_mult_non_power_of_two_rejected() {
    let mut fd = build_fd();
    // multiplier 0b110 = 6 -> leastsigbit_set==1, testval(1<<1==2) != 6 -> return 0.
    let shop = build_shift_and(&mut fd, OpCode::CPUI_INT_MULT, 8, 0xffff, 6);
    assert_eq!(RuleShiftAnd::new().apply_op(shop, &mut fd), 0);
}

// ---------------------------------------------------------------------------
// R2-C (slot-swap hunt): RuleHumptyOr's `b == c` reassignment arm
// (ruleaction.cc:5376-5380 -> ruleaction_4.rs:1923-1927) is the one branch that
// *rewrites three of the four slots* (`b=a; a=c; c=d`).  A transposed assignment
// here would compute `totalbits` from the wrong pair and emit a wrong mask while
// still "succeeding".  Build `(W & V) | (V & X)` so the shared operand V sits in
// slot 1 of and1 (==`b`) and slot 0 of and2 (==`c`): the `b == c` arm.  After the
// swap a==V, b==W, c==X; totalbits = offset(W)|offset(X).  Choose W=0xff00,
// X=0x00ff over a size-2 V -> totalbits 0xffff == calc_mask(2) -> COPY of V.
// If the swap were transposed (e.g. produced a==W or c==b) the result varnode or
// the all-bits test would differ.
#[test]
fn humptyor_b_eq_c_swap_picks_shared_operand() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let v = mk_vn(&mut fd, 2, 0x10); // the shared operand (a after swap)
    let wconst = fd.new_constant(2, 0xff00); // and1 slot 0 == `a` pre-swap
    let xconst = fd.new_constant(2, 0x00ff); // and2 slot 1 == `d` pre-swap
    // and1 = (W & V): slot0=W(const), slot1=V  -> pre-swap a=W, b=V
    let and1 = mk_op(&mut fd, 2, 0x20, OpCode::CPUI_INT_AND);
    fd.op_set_input(and1, wconst, 0).unwrap();
    fd.op_set_input(and1, v, 1).unwrap();
    fd.op_insert(and1, bl, None);
    let and1out = mk_out(&mut fd, 2, and1);
    // and2 = (V & X): slot0=V, slot1=X  -> pre-swap c=V, d=X.  b==c is V==V.
    let and2 = mk_op(&mut fd, 2, 0x28, OpCode::CPUI_INT_AND);
    fd.op_set_input(and2, v, 0).unwrap();
    fd.op_set_input(and2, xconst, 1).unwrap();
    fd.op_insert(and2, bl, None);
    let and2out = mk_out(&mut fd, 2, and2);

    let orop = mk_op(&mut fd, 2, 0x30, OpCode::CPUI_INT_OR);
    fd.op_set_input(orop, and1out, 0).unwrap();
    fd.op_set_input(orop, and2out, 1).unwrap();
    fd.op_insert(orop, bl, None);
    let _orout = mk_out(&mut fd, 2, orop);

    // C++ oracle: a!=c, a!=d, b==c -> swap to a=V,b=W(0xff00),c=X(0x00ff).
    // totalbits = 0xff00 | 0x00ff = 0xffff == calc_mask(size 2) -> COPY of V.
    let res = RuleHumptyOr::new().apply_op(orop, &mut fd);
    assert_eq!(res, 1);
    assert_eq!(fd.obank().get(orop).unwrap().code(), OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(orop).unwrap().num_input(), 1);
    assert_eq!(
        fd.obank().get(orop).unwrap().get_in(0),
        Some(v),
        "the b==c swap must leave the *shared* operand V as the COPY source"
    );
}
