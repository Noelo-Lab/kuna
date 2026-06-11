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
    RuleLoadVarnode, RuleShiftAnd, RuleStoreVarnode, RuleSubExtComm,
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
