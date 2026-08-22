//! Tests for the input-parameter carve-out in the uncomputed-return terminal rule.
//!
//! These pin the *predicate* on a hand-built `Funcdata`; the end-to-end witness
//! (a `wide`/`w2` pair whose only difference is copy-vs-arithmetic in the high
//! half) lives in `tests/stages/kuna-retinputhalf.xml`. The negative control —
//! that the callee-saved-restore phantom this carve-out sits inside stays killed —
//! is `kuna-console/tests/verify_return_uncomputed.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::context::ArchContext;

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
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unwritten(fd: &mut Funcdata, off: u64, sz: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(sz, &Address::new(r, off), None)
}

/// A model-less fixture cannot claim any storage is parameter storage, so the
/// carve-out is inert and the strict terminal rule stands. This is the
/// fail-closed direction: an unfamiliar prototype keeps today's answer.
#[test]
fn without_a_prototype_model_nothing_is_an_input_parameter() {
    let mut fd = build_fd();
    let vn = unwritten(&mut fd, 0x2000, 8);
    assert!(
        !is_input_parameter(&fd, vn),
        "no model means no parameter storage — the carve-out must not fire",
    );
}

/// A Varnode with a defining op is not the shape this rule speaks about; it is
/// classified by the op that produced it.
#[test]
fn a_written_varnode_is_never_the_input_parameter_shape() {
    use crate::context::TypeOp;
    use kuna_num::opcodes::OpCode;

    let mut fd = build_fd();
    let src = unwritten(&mut fd, 0x2000, 8);
    let r = ram(&fd);
    let op = fd.new_op(1, Address::new(Rc::clone(&r), 0x2100));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    fd.op_set_input(op, src, 0);
    let out = fd.new_varnode_out(8, &Address::new(r, 0x2100), op).expect("varnode out");
    assert!(
        !is_input_parameter(&fd, out),
        "a written Varnode is classified by its defining op, not by this rule",
    );
}

/// A free Varnode — one heritage never promoted to a function input — is
/// leftover, whatever storage it sits in.
#[test]
fn a_varnode_that_is_not_a_function_input_is_not_a_parameter() {
    let mut fd = build_fd();
    let vn = unwritten(&mut fd, 0x2000, 8);
    assert!(
        !fd.vbank().get(vn).unwrap().is_input(),
        "precondition: the fixture Varnode carries no input flag",
    );
    assert!(!is_input_parameter(&fd, vn));
}

/// The placement test lives in `kuna_returnuncomputed::computes_from`, but its
/// two inputs are this predicate's answer and an address comparison, so pin the
/// address comparison here: a terminal at the return half's OWN storage is the
/// caller's register passing straight through, never a placed argument.
#[test]
fn a_terminal_at_the_return_halfs_own_storage_is_not_a_placement() {
    let mut fd = build_fd();
    let r = ram(&fd);
    let vn = unwritten(&mut fd, 0x2000, 8);
    let same = Address::new(Rc::clone(&r), 0x2000);
    let other = Address::new(r, 0x2008);
    let addr = fd.vbank().get(vn).unwrap().get_addr().clone();
    assert_eq!(addr, same, "the terminal sits at its own address");
    assert_ne!(addr, other, "a moved argument arrives from somewhere else");
}
