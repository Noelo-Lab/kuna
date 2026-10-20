//! Required stackview byte stores stay separate from their register value when
//! a later call has a write-effect INDIRECT for those bytes.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace,
    SpacebaseSpace, UniqueSpace,
};
use kuna_decomp::context::ArchContext;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::merge::Merge;
use kuna_decomp::varnode::varnode_flags;
use kuna_num::opcodes::OpCode;

fn fixture(
    with_write_guard: bool,
    join_call: bool,
    write_width: i32,
    stack_addr_size: u32,
) -> (
    Funcdata,
    kuna_decomp::context::HighVariableId,
    kuna_decomp::context::HighVariableId,
) {
    let mut spaces = AddrSpaceManager::new();
    spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    spaces
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    spaces.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    let register = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    spaces.insert_space(Rc::clone(&register)).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    spaces.insert_space(Rc::clone(&ram)).unwrap();
    let stack = Rc::new(SpacebaseSpace::new(
        "stack",
        5,
        stack_addr_size,
        &register,
        1,
        true,
        false,
    ));
    spaces.insert_space(Rc::clone(&stack)).unwrap();

    let mut arch = ArchContext::new(spaces);
    arch.stack_views = true;
    let mut fd = Funcdata::new(
        "build",
        "build",
        Rc::new(arch),
        Address::new(Rc::clone(&ram), 0x1000),
        0x1000,
        0x40,
    )
    .unwrap();
    let object = Address::new(Rc::clone(&stack), 0x20);
    let mut backing = Datatype::new(3, type_metatype::TYPE_UNION);
    backing.name = "stack_views_rgb".to_string();
    fd.get_scope_local_mut()
        .unwrap()
        .add_symbol(
            "backing",
            Rc::new(backing),
            &object,
            &Address::new_invalid(),
        )
        .unwrap();

    let root = fd.bblocks_ref().root.expect("function block root");
    let block = fd.bblocks_mut().new_block_basic(root);
    let call_block = if join_call {
        let join = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(block, join);
        join
    } else {
        block
    };

    let literal = fd.new_constant(write_width, 0x11);
    let source_op = fd.new_op(write_width, Address::new(Rc::clone(&ram), 0x1001));
    fd.op_set_opcode(
        source_op,
        kuna_decomp::typeop::type_op_for(OpCode::CPUI_COPY),
    );
    fd.op_set_input(source_op, literal, 0).unwrap();
    let source = fd.new_unique_out(write_width, source_op).unwrap();
    fd.op_insert(source_op, block, None);

    let store = fd.new_op(write_width, Address::new(Rc::clone(&ram), 0x1002));
    fd.op_set_opcode(store, kuna_decomp::typeop::type_op_for(OpCode::CPUI_COPY));
    fd.op_set_input(store, source, 0).unwrap();
    let stored = fd.new_varnode_out(write_width, &object, store).unwrap();
    fd.vbank_mut()
        .get_mut(stored)
        .unwrap()
        .set_flags_pub(varnode_flags::mapped | varnode_flags::addrtied);
    fd.vbank_mut().get_mut(stored).unwrap().set_stack_store();
    fd.op_insert(store, block, None);

    let call = fd.new_op(1, Address::new(Rc::clone(&ram), 0x1003));
    fd.op_set_opcode(call, kuna_decomp::typeop::type_op_for(OpCode::CPUI_CALL));
    let target = fd.new_code_ref(&Address::new(Rc::clone(&ram), 0x2000));
    fd.op_set_input(call, target, 0).unwrap();
    fd.op_insert(call, call_block, None);
    if with_write_guard {
        fd.new_indirect_op(call, &object, write_width, 0);
    }
    fd.set_high_level();

    let backing = fd.vbank().get(stored).unwrap().get_high().unwrap();
    let value = fd.vbank().get(source).unwrap().get_high().unwrap();
    (fd, backing, value)
}

#[test]
fn a_later_call_guard_keeps_a_stackview_store_explicit() {
    let (mut fd, backing, value) = fixture(true, false, 1, 8);
    assert!(
        !Merge::merge_test_required(&mut fd, backing, value),
        "a register value must not merge into frame bytes guarded across a writing call"
    );
}

#[test]
fn a_read_only_call_does_not_change_the_store_merge() {
    let (mut fd, backing, value) = fixture(false, false, 1, 8);
    assert!(
        Merge::merge_test_required(&mut fd, backing, value),
        "without a write-effect INDIRECT, the existing adjacent-store merge remains allowed"
    );
}

#[test]
fn a_joined_call_guard_keeps_a_predecessor_stackview_store_explicit() {
    let (mut fd, backing, value) = fixture(true, true, 1, 8);
    assert!(
        !Merge::merge_test_required(&mut fd, backing, value),
        "a reachable join-block call must keep its predecessor's frame store explicit"
    );
}

#[test]
fn a_store_as_wide_as_a_sixteen_bit_pointer_is_not_split() {
    let (mut fd, backing, value) = fixture(true, false, 2, 2);
    assert!(
        Merge::merge_test_required(&mut fd, backing, value),
        "a full-width pointer-sized stack write keeps the prior merge policy"
    );
}

#[test]
fn a_byte_store_on_a_sixteen_bit_stack_keeps_the_write_guard() {
    let (mut fd, backing, value) = fixture(true, false, 1, 2);
    assert!(
        !Merge::merge_test_required(&mut fd, backing, value),
        "a one-byte store narrower than the pointer width remains guarded"
    );
}
