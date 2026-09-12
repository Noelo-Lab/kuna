use super::*;
use std::rc::Rc;
use kuna_base::{address::Address, space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace, UniqueSpace}};
use crate::context::{ArchContext, TypeOp};

fn fixture() -> (Funcdata, VarnodeStorage, VarnodeId) {
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    manage.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    manage.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 4, 1, 4, addrspace_flags::hasphysical, 1, 1));
    manage.insert_space(Rc::clone(&ram)).unwrap();
    let ctx = Rc::new(ArchContext::new(manage));
    let mut fd = Funcdata::new("frame", "frame", ctx, Address::new(Rc::clone(&ram), 0x1000), 0x10000000, 0x40).unwrap();
    let sp = VarnodeStorage { space: Some(Rc::clone(&ram)), offset: 0x80, size: 4 };
    let base = fd.new_varnode(4, &Address::new(ram, 0x80), None);
    let base = fd.set_input_varnode(base).unwrap();
    (fd, sp, base)
}

fn operation(fd: &mut Funcdata, code: OpCode, ins: &[VarnodeId], width: int4) -> VarnodeId {
    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
    let op = fd.new_op(ins.len() as int4, Address::new(ram, 0x1020));
    fd.op_set_opcode(op, TypeOp::new(code, 0, format!("{code:?}")));
    for (slot, &vn) in ins.iter().enumerate() { fd.op_set_input(op, vn, slot as int4).unwrap(); }
    fd.new_unique_out(width, op).unwrap()
}

#[test]
fn copied_frame_pointer_and_negative_displacement_keep_the_sp_origin() {
    let (mut fd, sp, base) = fixture();
    let eight = fd.new_constant(4, 8);
    let fp = operation(&mut fd, OpCode::CPUI_INT_ADD, &[base, eight], 4);
    let copied = operation(&mut fd, OpCode::CPUI_COPY, &[fp], 4);
    let twenty_four = fd.new_constant(4, 24);
    let slot = operation(&mut fd, OpCode::CPUI_INT_SUB, &[copied, twenty_four], 4);
    assert_eq!(frame_slot(&fd, slot, &sp), Some((base, 0xfffffff0)));
    let negative = fd.new_constant(4, 0xfffffff0);
    let equivalent = operation(&mut fd, OpCode::CPUI_INT_ADD, &[negative, base], 4);
    assert_eq!(frame_slot(&fd, equivalent, &sp), frame_slot(&fd, slot, &sp));
}

#[test]
fn unknown_bases_phi_nodes_width_changes_and_long_chains_are_declined() {
    let (mut fd, sp, base) = fixture();
    let ram = Rc::clone(sp.space.as_ref().unwrap());
    let unknown = fd.new_varnode(4, &Address::new(ram, 0x90), None);
    assert_eq!(frame_slot(&fd, unknown, &sp), None);
    let phi = operation(&mut fd, OpCode::CPUI_MULTIEQUAL, &[base, unknown], 4);
    assert_eq!(frame_slot(&fd, phi, &sp), None);
    let wide = operation(&mut fd, OpCode::CPUI_COPY, &[base], 8);
    assert_eq!(frame_slot(&fd, wide, &sp), None);
    let mut ptr = base;
    for _ in 0..65 { ptr = operation(&mut fd, OpCode::CPUI_COPY, &[ptr], 4); }
    assert_eq!(frame_slot(&fd, ptr, &sp), None);
}

#[test]
fn reload_requires_the_same_wrapped_slot_and_value_width() {
    let (mut fd, _, base) = fixture();
    let displacement = fd.new_constant(4, 16);
    let ptr = operation(&mut fd, OpCode::CPUI_INT_SUB, &[base, displacement], 4);
    let space = fd.new_constant(4, 0);
    operation(&mut fd, OpCode::CPUI_LOAD, &[space, ptr], 4);
    assert!(has_later_reload(&fd, base, 0xfffffff0, 4, 0x1000));
    assert!(!has_later_reload(&fd, base, 0xfffffff0, 8, 0x1000));
    assert!(!has_later_reload(&fd, base, 0xfffffff4, 4, 0x1000));
}
