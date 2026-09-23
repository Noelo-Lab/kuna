//! Unit tests for the vararg-tail classification.
//!
//! These pin which operations hand a parameter's value on ([`moves_the_value`])
//! and the shapes that answer "not vararg-tail" without a call in front of them.
//! The real witnesses are binaries: openssh `xcalloc`'s third parameter (gcc's
//! `push %rdx` alignment filler at a variadic call) and gnulib `open_safer`'s
//! `mode`, both recorded in `docs/features/passthrough/dwarf-confirmation.md`.

use super::*;

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::{ArchContext, TypeOp};

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

fn mk_def(fd: &mut Funcdata, opc: OpCode, inputs: &[VarnodeId], out_off: u64) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs.len() as int4, Address::new(Rc::clone(&r), out_off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(op, vn, i as int4);
    }
    fd.new_varnode_out(8, &Address::new(r, out_off), op).expect("varnode out");
    op
}

#[test]
fn a_storage_with_no_input_varnode_is_not_vararg_tail() {
    let fd = build_fd();
    let r = ram(&fd);
    let addr = Address::new(r, 0x3000);
    assert!(
        !only_feeds_a_vararg_tail(&fd, &addr, 8),
        "a parameter the function has no incoming Varnode for keeps today's answer",
    );
}

#[test]
fn a_copy_and_a_mask_hand_the_value_on_but_arithmetic_does_not() {
    let mut fd = build_fd();
    let r = ram(&fd);
    let vn = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x3000), None);
    let k = fd.new_constant(8, 0xffff_ffff);
    let copy = mk_def(&mut fd, OpCode::CPUI_COPY, &[vn], 0x3100);
    let mask = mk_def(&mut fd, OpCode::CPUI_INT_AND, &[vn, k], 0x3200);
    let sum = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[vn, vn], 0x3300);
    assert!(moves_the_value(&fd, copy, vn, OpCode::CPUI_COPY), "a copy is the same value");
    assert!(
        moves_the_value(&fd, mask, vn, OpCode::CPUI_INT_AND),
        "`mode & 0xffffffff` is open_safer's va_arg read, still the parameter",
    );
    assert!(
        !moves_the_value(&fd, sum, vn, OpCode::CPUI_INT_ADD),
        "arithmetic is the function using the parameter for itself",
    );
}

#[test]
fn a_mask_by_a_second_live_value_is_not_a_hand_on() {
    let mut fd = build_fd();
    let r = ram(&fd);
    let vn = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x3000), None);
    let other = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x3008), None);
    let and = mk_def(&mut fd, OpCode::CPUI_INT_AND, &[vn, other], 0x3400);
    assert!(
        !moves_the_value(&fd, and, vn, OpCode::CPUI_INT_AND),
        "an AND of two live values computes something; only a constant mask is a width",
    );
}
