//! Tests for the uncomputed-return-trial rule.
//!
//! These pin the *classification* — which value shapes count as computed — on a
//! hand-built `Funcdata`. The end-to-end witness (a real binary whose `main`
//! stops returning a 16-byte phantom) lives in
//! `kuna-console/tests/verify_return_uncomputed.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::{ArchContext, TypeOp};

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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// A Varnode at a ram address with no defining op — a function input, or a
/// location the function only ever reads.
fn unwritten(fd: &mut Funcdata, off: u64, sz: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(sz, &Address::new(r, off), None)
}

/// Build `out = <opc>(inputs...)` at a fresh ram address and return `out`.
fn mk_def(fd: &mut Funcdata, opc: OpCode, inputs: &[VarnodeId], out_off: u64) -> VarnodeId {
    let r = ram(fd);
    let op = fd.new_op(inputs.len() as int4, Address::new(Rc::clone(&r), out_off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(op, vn, i as int4);
    }
    fd.new_varnode_out(8, &Address::new(r, out_off), op).expect("varnode out")
}

#[test]
fn a_constant_is_a_computed_return_value() {
    let mut fd = build_fd();
    let k = fd.new_constant(8, 7);
    assert!(
        computes_a_value(&fd, k, 0),
        "`return 0;` is a real return — a literal must never be dropped",
    );
}

#[test]
fn an_unwritten_varnode_is_not_a_computed_return_value() {
    let mut fd = build_fd();
    let vn = unwritten(&mut fd, 0x2000, 8);
    assert!(
        !computes_a_value(&fd, vn, 0),
        "an unwritten Varnode carries whatever the caller left there",
    );
}

#[test]
fn a_copy_chases_through_to_its_source() {
    let mut fd = build_fd();

    let leftover = unwritten(&mut fd, 0x2000, 8);
    let copied = mk_def(&mut fd, OpCode::CPUI_COPY, &[leftover], 0x2100);
    assert!(!computes_a_value(&fd, copied, 0), "a copy of leftover is still leftover");

    let k = fd.new_constant(8, 5);
    let copied_k = mk_def(&mut fd, OpCode::CPUI_COPY, &[k], 0x2200);
    assert!(computes_a_value(&fd, copied_k, 0), "a copy of a literal is a real value");
}

#[test]
fn arithmetic_is_a_computed_return_value() {
    let mut fd = build_fd();
    // Both inputs unwritten, but INT_ADD is not a move — the walk stops here.
    // This is the guard that keeps a genuine struct return whose halves are built
    // from parameters.
    let a = unwritten(&mut fd, 0x2000, 8);
    let b = unwritten(&mut fd, 0x2008, 8);
    let sum = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 0x2100);
    assert!(computes_a_value(&fd, sum, 0), "arithmetic produces a value");
}

#[test]
fn a_load_is_a_computed_return_value() {
    let mut fd = build_fd();
    // `struct S get(struct S *p) { return *p; }` builds both halves with LOADs
    // through an (unwritten) parameter. A LOAD is not a move, so the walk stops
    // and the half is kept — this is the false positive this rule must not have.
    let spc = fd.new_constant(4, 0);
    let ptr = unwritten(&mut fd, 0x2000, 8);
    let loaded = mk_def(&mut fd, OpCode::CPUI_LOAD, &[spc, ptr], 0x2100);
    assert!(
        computes_a_value(&fd, loaded, 0),
        "a load through a pointer is a real value — a struct return reads this way",
    );
}

#[test]
fn a_phi_is_computed_when_any_arm_is() {
    let mut fd = build_fd();
    let leftover = unwritten(&mut fd, 0x2000, 8);
    let k = fd.new_constant(8, 3);
    let phi = mk_def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[leftover, k], 0x2100);
    assert!(
        computes_a_value(&fd, phi, 0),
        "one real arm is enough — the function returns a value on that path",
    );
}

#[test]
fn a_phi_of_only_leftover_is_not_computed() {
    let mut fd = build_fd();
    let a = unwritten(&mut fd, 0x2000, 8);
    let b = unwritten(&mut fd, 0x2008, 8);
    let phi = mk_def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[a, b], 0x2100);
    assert!(
        !computes_a_value(&fd, phi, 0),
        "leftover on every path is still leftover",
    );
}
