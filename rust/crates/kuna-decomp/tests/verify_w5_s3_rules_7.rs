//! Independent VERIFIER adversarial tests for item `w5-s3-rules-7`
//! (`ruleaction.cc` lines 8374-9803, classes `RuleSignDiv2` .. `RuleIgnoreNan`).
//!
//! These were written by the verifier (NOT the porter) and target the spots the
//! mandatory hunt list flagged as most fragile for this batch:
//!
//!   * **Missing wrapping on variable shift counts.** Three rules shift a
//!     `uintb` by a count derived from a p-code constant offset with no
//!     in-range guard:
//!       - `RuleDivChain`   `val1 <<= sa`        (sa = inner INT_RIGHT shift)
//!       - `RuleSignMod2nOpt` `mask = (mask<<n)-1` (n = a.size*8 - shiftAmt)
//!     The C++ builds with x86 shift semantics (`shift & 63`); the Rust port
//!     uses a *raw* `<<`, which **panics in debug** for shift >= 64.  These
//!     tests drive exactly that count and assert the C++ oracle's
//!     fire/no-fire outcome (NOT a panic).  Each `#[should_panic]` documents
//!     the confirmed divergence so the suite stays green while the finding is
//!     visible.
//!   * **Unsigned/signed overflow boundary in `RuleDivChain`** (`bitcount`
//!     vs `sz*8` for DIV, `sz*8-2` for SDIV).
//!
//! C++ oracle: `decompiler/cpp/ruleaction.cc`:
//!   RuleDivChain 8434-8476, RuleSignMod2nOpt 8698-8784.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};

use kuna_decomp::action::Rule;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_7::{RuleDivChain, RuleSignMod2nOpt};
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};
use kuna_num::opcodes::OpCode;

// ---------------------------------------------------------------------------
// Fixtures (mirror crate-internal ruleaction_7/tests.rs, public API only)
// ---------------------------------------------------------------------------

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

fn make_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), None);
    fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
}

fn konst(fd: &mut Funcdata, size: int4, val: uintb) -> VarnodeId {
    fd.new_constant(size, val)
}

fn make_op(
    fd: &mut Funcdata,
    off: u64,
    opc: OpCode,
    num_in: int4,
    outsize: int4,
) -> (OpId, VarnodeId) {
    let r = ram(fd);
    let pc = Address::new(Rc::clone(&r), off);
    let op = fd.obank_mut().create_at(num_in, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    let out = fd.new_varnode(outsize, &Address::new(r, off), Some(unk(outsize)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let out = fd.vbank_mut().set_def(out, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
    (op, out)
}

fn make_op_noout(fd: &mut Funcdata, off: u64, opc: OpCode, num_in: int4) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(num_in, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn op_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

fn in_off(fd: &Funcdata, op: OpId, slot: int4) -> uintb {
    let vn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
    fd.vbank().get(vn).unwrap().get_offset()
}

// ===========================================================================
// F1 — RuleDivChain: `val1 <<= sa` raw shift with sa >= 64
//
// Pattern that enters the INT_RIGHT (unsigned) branch:
//   inner = x INT_RIGHT sa   (sz 8, sa = 64)   -- a legal but degenerate shift
//   root  = inner INT_DIV c2 (sz 8)
// C++ (ruleaction.cc:8455-8458):
//   int4 sa = constVn1->getOffset(); val1 = 1; val1 <<= sa;   // x86: 1<<(64&63)=1
// then resval = (1 * c2) & mask, etc.  The C++ does NOT crash; it computes a
// (wrapped) val1 and proceeds.  The Rust port's raw `1u64 << 64` PANICS first.
// ===========================================================================

fn build_divchain_right(fd: &mut Funcdata, sa: uintb, c2: uintb) -> OpId {
    let x = make_input(fd, 0x100, 8);
    let (inner, innerout) = make_op(fd, 0x200, OpCode::CPUI_INT_RIGHT, 2, 8);
    wire(fd, x, inner, 0);
    let csa = konst(fd, 8, sa);
    wire(fd, csa, inner, 1);
    // root = innerout INT_DIV c2  (innerout lone-descends to root)
    let root = make_op_noout(fd, 0x210, OpCode::CPUI_INT_DIV, 2);
    wire(fd, innerout, root, 0);
    let cc2 = konst(fd, 8, c2);
    wire(fd, cc2, root, 1);
    root
}

#[test]
fn w5s3_divchain_right_small_shift_is_faithful() {
    // Sanity anchor: a small in-range shift behaves exactly like C++.
    //   inner = x >> 2  (val1 = 1<<2 = 4),  root = inner / 3  =>  x / 12.
    // bitcount = msb(4)=2 + msb(3)=1 + 2 = 5 <= 64  -> fires.
    let mut fd = build_fd();
    let root = build_divchain_right(&mut fd, 2, 3);
    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "small-shift divchain must fire");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_DIV, "root stays INT_DIV");
    assert_eq!(in_off(&fd, root, 1), 12, "collapsed constant = (1<<2)*3 = 12");
}

#[test]
fn w5s3_divchain_right_shift64_wraps_like_cpp() {
    // REPAIRED (F1): C++ `val1 <<= 64` wraps under x86 shift-count masking
    // (`1 << (64 & 63) = 1`) and the rule continues: resval = 1*c2 = 3 (nonzero),
    // bitcount = msb(1)+msb(3)+2 = 0+1+2 = 3 <= 64 -> FIRES, collapsing
    // `(x >> 64) / 3` to `x / 3`. The port now uses `wshl`, so it matches the C++
    // oracle (fire + collapse) instead of panicking on the raw `1u64 << 64`.
    let mut fd = build_fd();
    let root = build_divchain_right(&mut fd, 64, 3);
    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root, &mut fd), 1, "shift-64 divchain fires (wrapped val1=1)");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_DIV, "root stays INT_DIV");
    assert_eq!(in_off(&fd, root, 1), 3, "collapsed constant = (1<<64 wrapped to 1)*3 = 3");
}

// ===========================================================================
// F2 — RuleSignMod2nOpt: `mask = (mask << n) - 1` raw shift with n == 64
//
// n = a.getSize()*8 - shiftAmt.  With an 8-byte sign-extracted `a` and the root
// INT_RIGHT shiftAmt == 0, n == 64.  C++ (ruleaction.cc:8706-8708):
//   int4 n = a->getSize()*8 - shiftAmt;  uintb mask=1; mask=(mask<<n)-1;
// x86: 1<<64 = 1, mask = 0; the descend loop then finds no INT_MULT and the
// rule returns 0 (no fire) WITHOUT crashing.  Rust's raw `mask << 64` panics.
//
// Minimal reachable graph: root = (s>>0)  where its in0 is a valid sign
// extraction (a s>> 63) so checkSignExtraction succeeds and a is non-free.
// shiftAmt = 0 makes n = 64.  No INT_MULT descends from the root output, so the
// C++ falls out of the loop and returns 0.
// ===========================================================================

fn build_signmod2nopt_shiftamt0(fd: &mut Funcdata) -> OpId {
    let a = make_input(fd, 0x100, 8);
    // sign extraction: a s>> 63   (checkSignExtraction: in1 == insize*8-1 == 63)
    let (signop, signout) = make_op(fd, 0x200, OpCode::CPUI_INT_SRIGHT, 2, 8);
    wire(fd, a, signop, 0);
    let c63 = konst(fd, 8, 63);
    wire(fd, c63, signop, 1);
    // root = signout INT_RIGHT 0   (shiftAmt == 0 -> n == 8*8 - 0 == 64)
    let (root, _rootout) = make_op(fd, 0x210, OpCode::CPUI_INT_RIGHT, 2, 8);
    wire(fd, signout, root, 0);
    let c0 = konst(fd, 8, 0);
    wire(fd, c0, root, 1);
    root
}

#[test]
fn w5s3_signmod2nopt_in_range_is_faithful() {
    // Sanity anchor: shiftAmt = 1 (in range) -> n = 63, mask = (1<<63)-1.  No
    // INT_MULT descends from the root, so C++ and Rust both return 0 cleanly.
    let mut fd = build_fd();
    let a = make_input(&mut fd, 0x100, 8);
    let (signop, signout) = make_op(&mut fd, 0x200, OpCode::CPUI_INT_SRIGHT, 2, 8);
    wire(&mut fd, a, signop, 0);
    let c63 = konst(&mut fd, 8, 63);
    wire(&mut fd, c63, signop, 1);
    let (root, _o) = make_op(&mut fd, 0x210, OpCode::CPUI_INT_RIGHT, 2, 8);
    wire(&mut fd, signout, root, 0);
    let c1 = konst(&mut fd, 8, 1);
    wire(&mut fd, c1, root, 1);

    let mut rule = RuleSignMod2nOpt::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "no INT_MULT descend -> no fire");
}

#[test]
fn w5s3_signmod2nopt_shiftamt0_returns_zero_like_cpp() {
    // REPAIRED (F2): with n == 64 the C++ computes mask via a wrapped shift
    // (`1 << (64 & 63) = 1`, then `mask = 1 - 1 = 0`) and returns 0 (no INT_MULT
    // descends from the root output). The port now uses `wshl`, so it returns 0
    // matching the C++ oracle instead of panicking on the raw `mask << 64`.
    let mut fd = build_fd();
    let root = build_signmod2nopt_shiftamt0(&mut fd);
    let mut rule = RuleSignMod2nOpt::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "n==64 wraps mask to 0 -> no fire");
}

// ===========================================================================
// F3 (boundary, faithful) — RuleDivChain unsigned vs signed overflow split.
//
// DIV uses `bitcount > sz*8`; SDIV uses `bitcount > sz*8 - 2`.  This pins the
// 1-LSB difference in the limit so a future edit to the comparator is caught.
// sz = 4 (32 bits).  c1 = 0x40000000 (bit 30), c2 = 4 (bit 2):
//   bitcount = 30 + 2 + 2 = 34.
//   DIV  limit = 32  -> 34 > 32  -> reject (no fire)
//   SDIV limit = 30  -> 34 > 30  -> reject (no fire)
// Use c1 = 0x40000000, c2 = 1 (bit 0): bitcount = 30+0+2 = 32.
//   DIV  limit = 32 -> 32 > 32 is false -> FIRES
//   SDIV limit = 30 -> 32 > 30 -> reject
// This asymmetry is the boundary under test.
// ===========================================================================

fn build_divchain_same(fd: &mut Funcdata, opc: OpCode, c1: uintb, c2: uintb) -> OpId {
    let x = make_input(fd, 0x100, 4);
    let (inner, innerout) = make_op(fd, 0x200, opc, 2, 4);
    wire(fd, x, inner, 0);
    let cc1 = konst(fd, 4, c1);
    wire(fd, cc1, inner, 1);
    let root = make_op_noout(fd, 0x210, opc, 2);
    wire(fd, innerout, root, 0);
    let cc2 = konst(fd, 4, c2);
    wire(fd, cc2, root, 1);
    root
}

#[test]
fn w5s3_divchain_div_at_limit_fires_sdiv_rejects() {
    // bitcount == 32 exactly. c1 = 0x40000000 (msb bit30), c2 = 1 (msb bit0).
    // DIV: 32 > 32 false -> fires.  SDIV: 32 > 30 true -> rejects.
    let mut fd = build_fd();
    let root_div = build_divchain_same(&mut fd, OpCode::CPUI_INT_DIV, 0x4000_0000, 1);
    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root_div, &mut fd), 1, "DIV at limit fires");

    let mut fd2 = build_fd();
    let root_sdiv = build_divchain_same(&mut fd2, OpCode::CPUI_INT_SDIV, 0x4000_0000, 1);
    let mut rule2 = RuleDivChain::new();
    assert_eq!(rule2.apply_op(root_sdiv, &mut fd2), 0, "SDIV at same bitcount rejects");
}

// ===========================================================================
// F4 (sanity) — calc_mask(-1)/-1 constant handling in the resval==0 guard.
//
// resval = (val1 * val2) & calc_mask(sz).  If the product masks to 0 the rule
// must reject (C++ 8465: `if (resval == 0) return 0;`).  sz = 4, c1 = 0x10000
// (1<<16), c2 = 0x10000 -> product 1<<32, & mask(4)=0xffffffff -> 0 -> reject.
// (No shift involved; pure wrapping-mult boundary.)
// ===========================================================================

#[test]
fn w5s3_divchain_resval_zero_rejects() {
    let mut fd = build_fd();
    let root = build_divchain_same(&mut fd, OpCode::CPUI_INT_DIV, 0x1_0000, 0x1_0000);
    let mut rule = RuleDivChain::new();
    assert_eq!(rule.apply_op(root, &mut fd), 0, "resval masks to 0 -> reject");
    assert_eq!(op_of(&fd, root), OpCode::CPUI_INT_DIV, "unchanged");
    // calc_mask sanity (mask of 4 bytes = 0xffffffff).
    assert_eq!(calc_mask(4), 0xffff_ffff);
}
