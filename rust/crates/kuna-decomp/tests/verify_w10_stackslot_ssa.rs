//! Verifier gate for `rport/w10-stackslot-ssa`: the ported
//! `RuleMultiCollapse` (ruleaction.cc:3253) collapses a `MULTIEQUAL` whose
//! inputs are functionally equal — the spill/reload-fold keystone.
//!
//! The diagnosis (see the wave report) is that two per-branch `COPY(reg)`
//! inputs of a phi (the reloaded spilled register on two paths) must be CSE'd
//! into a single copy before the CFG can structure into `&&`/`||`.  C++
//! `RuleMultiCollapse` does this via `functionalEquality` + `cseFindInBlock`.
//! The Rust port of that rule body was a SEAM stub returning 0; this wave
//! un-seams it.  These tests prove the un-seamed rule:
//!   1. collapses a MULTIEQUAL of two IDENTICAL inputs (absolute equality)
//!      into a COPY of that input (the simplest spill/reload — no divergence);
//!   2. collapses a MULTIEQUAL whose two inputs are distinct `COPY(reg)` of the
//!      SAME register (functional equality) by hoisting a single dominant copy;
//!   3. DECLINES when one input genuinely differs (the condconst_conn shape —
//!      one path sets a constant), exactly as C++ does (`success=false`).
//!
//! No special-casing: the rule body is the verbatim C++ algorithm; these tests
//! only build the canonical p-code shapes and assert the C++ outcome.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::ruleaction_3::RuleMultiCollapse;
use kuna_decomp::action::Rule;
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;
use kuna_num::opcodes::OpCode;

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
/// A non-free processor register Varnode (heritaged input → isHeritageKnown).
fn mk_reg(fd: &mut Funcdata, s: int4, off: u64) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.vbank_mut().create(s, Address::new(r, off), dt(s));
    fd.vbank_mut().set_input(vn, &mut |_, _, _| Ok(())).expect("set_input")
}
/// Fresh unique output of `op`, marked heritage-known (`insert`), as a real
/// SSA def written into the op graph would be.
fn mk_out(fd: &mut Funcdata, s: int4, op: OpId) -> VarnodeId {
    let seqnum = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum };
    let vn = fd
        .vbank_mut()
        .create_def_unique(s, dt(s), def, &mut |_, _, _| Ok(()))
        .expect("create_def_unique");
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    // A written SSA def is heritage-known: set the `insert` flag the heritage
    // renamer would have set (mk_reg's set_input does this for inputs).
    fd.vbank_mut().get_mut(vn).unwrap().set_flags_pub(kuna_decomp::varnode::varnode_flags::insert);
    vn
}

/// Build a COPY(`src`) op in block `bl` at `off`, return (op, out varnode).
fn mk_copy(fd: &mut Funcdata, bl: BlockId, off: u64, src: VarnodeId, s: int4) -> (OpId, VarnodeId) {
    let op = mk_op(fd, 1, off, OpCode::CPUI_COPY);
    fd.op_set_input(op, src, 0).unwrap();
    fd.op_insert(op, bl, None);
    let out = mk_out(fd, s, op);
    (op, out)
}

// ===========================================================================
// AT1 — ABSOLUTE EQUALITY: phi(v, v) -> COPY(v).  The simplest spill/reload:
// the same value reaches the join on both paths, the MULTIEQUAL is redundant.
// ===========================================================================
#[test]
fn at1_multiequal_of_identical_inputs_collapses() {
    let mut fd = build_fd();
    let join = mk_block(&mut fd);
    let edi = mk_reg(&mut fd, 4, 0x40);
    // m = MULTIEQUAL(edi, edi)  -- both branches carry the same value
    let phi = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_MULTIEQUAL);
    fd.op_set_input(phi, edi, 0).unwrap();
    fd.op_set_input(phi, edi, 1).unwrap();
    fd.op_insert(phi, join, None);
    let _out = mk_out(&mut fd, 4, phi);

    let res = RuleMultiCollapse.apply_op(phi, &mut fd);
    assert_eq!(res, 1, "MULTIEQUAL(v,v) must collapse (absolute equality)");
    // The MULTIEQUAL op is destroyed; its output's reads now point at `edi`.
    assert!(
        fd.obank().get(phi).map(|o| o.is_dead()).unwrap_or(true),
        "the redundant MULTIEQUAL must be destroyed"
    );
}

// ===========================================================================
// AT2 — FUNCTIONAL EQUALITY: phi(COPY(edi), COPY(edi)) where the two copies are
// DISTINCT ops (the spill/reload on two branches).  C++ hoists a single
// dominant copy (cseFindInBlock) and rewrites; the rule reports a change.
// This is the exact dataflow the condconst_conn `v1=x` paths produce.
// ===========================================================================
#[test]
fn at2_multiequal_of_functionally_equal_copies_collapses() {
    let mut fd = build_fd();
    let b_then = mk_block(&mut fd);
    let b_else = mk_block(&mut fd);
    let join = mk_block(&mut fd);
    let edi = mk_reg(&mut fd, 4, 0x40);
    // Two distinct COPY(edi), one per branch.
    let (_c1, o1) = mk_copy(&mut fd, b_then, 0x100, edi, 4);
    let (_c2, o2) = mk_copy(&mut fd, b_else, 0x200, edi, 4);
    fd.bb_set_order(b_then);
    fd.bb_set_order(b_else);
    // m = MULTIEQUAL(o1, o2)
    let phi = mk_op(&mut fd, 2, 0x300, OpCode::CPUI_MULTIEQUAL);
    fd.op_set_input(phi, o1, 0).unwrap();
    fd.op_set_input(phi, o2, 1).unwrap();
    fd.op_insert(phi, join, None);
    let _out = mk_out(&mut fd, 4, phi);
    fd.bb_set_order(join);

    let res = RuleMultiCollapse.apply_op(phi, &mut fd);
    assert_eq!(
        res, 1,
        "MULTIEQUAL(COPY(edi),COPY(edi)) with distinct copies must collapse \
         (functional equality) — this is the spill/reload fold"
    );
}

// ===========================================================================
// AT3 — DECLINE: phi(COPY(edi), COPY(#0x14)) — one input genuinely differs.
// This is the condconst_conn shape (one path sets a constant 0x14, two set x).
// RuleMultiCollapse must NOT collapse it (C++ success=false): the differing
// constant input is real.  (The condconst_conn `&&`-fold instead needs the
// Merge/ActionDominantCopy phase — the wave's reported next blocker.)
// ===========================================================================
#[test]
fn at3_multiequal_with_a_genuinely_different_input_declines() {
    let mut fd = build_fd();
    let b_then = mk_block(&mut fd);
    let b_else = mk_block(&mut fd);
    let join = mk_block(&mut fd);
    let edi = mk_reg(&mut fd, 4, 0x40);
    let (_c1, o1) = mk_copy(&mut fd, b_then, 0x100, edi, 4);
    let k = fd.new_constant(4, 0x14);
    let (_c2, o2) = mk_copy(&mut fd, b_else, 0x200, k, 4);
    fd.bb_set_order(b_then);
    fd.bb_set_order(b_else);
    let phi = mk_op(&mut fd, 2, 0x300, OpCode::CPUI_MULTIEQUAL);
    fd.op_set_input(phi, o1, 0).unwrap();
    fd.op_set_input(phi, o2, 1).unwrap();
    fd.op_insert(phi, join, None);
    let _out = mk_out(&mut fd, 4, phi);
    fd.bb_set_order(join);

    let res = RuleMultiCollapse.apply_op(phi, &mut fd);
    assert_eq!(
        res, 0,
        "MULTIEQUAL with a genuinely different (constant) input must NOT collapse"
    );
    assert!(
        !fd.obank().get(phi).map(|o| o.is_dead()).unwrap_or(true),
        "the MULTIEQUAL must survive when an input genuinely differs"
    );
}
