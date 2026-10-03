use super::*;
use crate::context::ArchContext;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace,
    SpacebaseSpace, UniqueSpace,
};
use std::rc::Rc;

fn function() -> (Funcdata, Rc<AddrSpace>, VarnodeId) {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&ram)).unwrap();
    let register = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        0,
        0,
        0,
    ));
    manager.insert_space(Rc::clone(&register)).unwrap();
    manager.insert_space(Rc::new(IopSpace::new(4))).unwrap();
    let stack = Rc::new(SpacebaseSpace::new("stack", 5, 4, &ram, 1, true, false));
    manager.insert_space(Rc::clone(&stack)).unwrap();
    let mut data = Funcdata::new(
        "roots",
        "roots",
        Rc::new(ArchContext::new(manager)),
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    let root = data.bblocks_ref().root.unwrap();
    data.bblocks_mut().new_block_basic(root);
    let argument = data.new_varnode(4, &Address::new(register, 4), None);
    let argument = data.set_input_varnode(argument).unwrap();
    (data, stack, argument)
}

fn operation(data: &mut Funcdata, code: OpCode, inputs: &[VarnodeId]) -> (OpId, VarnodeId) {
    let op = data.new_op(inputs.len() as int4, data.get_address().clone());
    data.op_set_opcode_code(op, code);
    for (slot, &input) in inputs.iter().enumerate() {
        data.op_set_input(op, input, slot as int4).unwrap();
    }
    let output = data.new_unique_out(4, op).unwrap();
    (op, output)
}

#[test]
fn provisional_effects_do_not_supply_unknown_stack_pointer_provenance() {
    let (mut data, stack, argument) = function();
    let space = data.new_constant(4, 2);
    let store = data.new_op(3, data.get_address().clone());
    data.op_set_opcode_code(store, OpCode::CPUI_STORE);
    for (slot, input) in [space, argument, argument].into_iter().enumerate() {
        data.op_set_input(store, input, slot as int4).unwrap();
    }
    let root = data.bblocks_ref().root.unwrap();
    let block = data.bblocks_ref().sub_block(root, 0).unwrap();
    data.op_insert(store, block, None);
    let mut guards = Vec::new();
    let mut writes = Vec::new();
    crate::kuna_spillstoreguard::build(
        &mut data,
        &Address::new(stack, 0x20),
        4,
        &[store],
        &[],
        &HashMap::new(),
        &mut guards,
        &mut writes,
    );
    assert_eq!(guards.len(), 1);
    let indirect = guards[0].0;
    let pointer = writes[0];
    let unknown = data.obank().get(indirect).unwrap().get_in(0).unwrap();
    assert!(data.vbank().get(unknown).unwrap().is_free());
    let mut memo = HashMap::new();
    assert!(!from_input(&data, pointer, &mut memo));
    assert!(!from_input(&data, pointer, &mut memo));
    let (phi, cycle) = operation(&mut data, OpCode::CPUI_MULTIEQUAL, &[pointer, pointer]);
    data.op_set_input(phi, cycle, 1).unwrap();
    assert!(!from_input(&data, cycle, &mut memo));
    let register = data
        .vbank()
        .get(argument)
        .unwrap()
        .get_addr()
        .get_space()
        .unwrap()
        .clone();
    let base = data.new_varnode(4, &Address::new(register, 0), None);
    let base = data.set_input_varnode(base).unwrap();
    data.vbank_mut().get_mut(base).unwrap().set_spacebase();
    data.op_set_input(indirect, base, 0).unwrap();
    assert!(!from_input(&data, cycle, &mut HashMap::new()));
    data.op_set_input(indirect, argument, 0).unwrap();
    assert!(from_input(&data, cycle, &mut HashMap::new()));
}

#[test]
fn data_inputs_include_stack_arguments_but_exclude_spacebases() {
    let (mut data, stack, argument) = function();
    let register = data
        .vbank()
        .get(argument)
        .unwrap()
        .get_addr()
        .get_space()
        .unwrap()
        .clone();
    let base_addr = Address::new(register, 0);
    let base = data.new_varnode(4, &base_addr, None);
    let base = data.set_input_varnode(base).unwrap();
    data.vbank_mut().get_mut(base).unwrap().set_spacebase();
    let stack_argument = data.new_varnode(4, &Address::new(stack, 8), None);
    let stack_argument = data.set_input_varnode(stack_argument).unwrap();
    let offset = data.new_constant(4, 8);
    let frame_pointer = operation(&mut data, OpCode::CPUI_INT_ADD, &[base, offset]).1;
    let scaled = data.new_constant(4, 4);
    let indexed = operation(&mut data, OpCode::CPUI_PTRADD, &[base, argument, scaled]).1;
    let indexed_add = operation(&mut data, OpCode::CPUI_INT_ADD, &[base, argument]).1;
    let copied = operation(&mut data, OpCode::CPUI_COPY, &[indexed_add]).1;
    let (phi, cycle) = operation(&mut data, OpCode::CPUI_MULTIEQUAL, &[copied, copied]);
    let next = operation(&mut data, OpCode::CPUI_INT_ADD, &[cycle, argument]).1;
    data.op_set_input(phi, next, 1).unwrap();
    let mut memo = HashMap::new();
    for root in [
        base,
        frame_pointer,
        indexed,
        indexed_add,
        copied,
        cycle,
        next,
        base,
        frame_pointer,
    ] {
        assert!(!from_input(&data, root, &mut memo));
    }
    assert!(from_input(&data, argument, &mut memo));
    assert!(from_input(&data, stack_argument, &mut memo));
    let mixed = operation(&mut data, OpCode::CPUI_MULTIEQUAL, &[cycle, argument]).1;
    let shifted = operation(&mut data, OpCode::CPUI_INT_ADD, &[mixed, offset]).1;
    assert!(from_input(&data, mixed, &mut memo));
    assert!(from_input(&data, shifted, &mut memo));
    let (loop_op, mixed_loop) = operation(&mut data, OpCode::CPUI_INT_ADD, &[mixed, mixed]);
    data.op_set_input(loop_op, mixed_loop, 1).unwrap();
    assert!(from_input(&data, mixed_loop, &mut memo));
}

#[test]
fn repeated_memory_and_call_roots_do_not_reuse_positive_memo_entries() {
    let (mut data, _, argument) = function();
    let space = data.new_constant(4, 2);
    let userop = data.new_constant(4, 0);
    let load = operation(&mut data, OpCode::CPUI_LOAD, &[space, argument]).1;
    let call = operation(&mut data, OpCode::CPUI_CALL, &[argument]).1;
    let other = operation(&mut data, OpCode::CPUI_CALLOTHER, &[userop, argument]).1;
    let mut memo = HashMap::new();
    for root in [load, call, other, load, call, other] {
        assert!(!from_input(&data, root, &mut memo));
    }
    assert!(from_input(&data, argument, &mut memo));
}

#[test]
fn load_intervals_include_wrapped_bytes() {
    assert!(may_overlap((0xfffe, 0xffff), 2, 0, 1, 2, 2, 1));
    assert!(may_overlap((0x1010, 0x1020), 4, 0x1021, 0x1024, 2, 2, 1));
    assert!(!may_overlap((0x1010, 0x1020), 1, 0x1021, 0x1024, 2, 2, 1));
    assert!(!may_overlap((0xd11a, 0xd219), 2, 0, 4, 2, 2, 1));
    assert!(may_overlap((0x100, 0x100), 1, 0, 4, 2, 1, 1));
    assert!(may_overlap((0x100, 0x100), 1, 0, 4, 2, 2, 2));
}

#[test]
fn stale_or_missing_load_pointers_cannot_force_partial_stores() {
    let (mut data, _, argument) = function();
    let space = data.new_constant(4, 2);
    let (write, value) = operation(&mut data, OpCode::CPUI_COPY, &[argument]);
    let (converted, _) = operation(&mut data, OpCode::CPUI_LOAD, &[space, argument]);
    data.op_set_opcode_code(converted, OpCode::CPUI_COPY);
    let (missing, _) = operation(&mut data, OpCode::CPUI_LOAD, &[]);
    let root = data.bblocks_ref().root.unwrap();
    let block = data.bblocks_ref().sub_block(root, 0).unwrap();
    for op in [write, converted, missing] {
        data.op_insert(op, block, None);
    }
    let addr = data.get_address().clone();
    apply(
        &mut data,
        vec![Guard {
            addr: addr.clone(),
            size: 4,
            writes: vec![value],
            loads: vec![converted, missing],
        }],
    );
    assert!(!data.vbank().get(value).unwrap().is_addr_force());
    assert_eq!(data.vbank().get(value).unwrap().get_def(), Some(write));
    let (live, _) = operation(&mut data, OpCode::CPUI_LOAD, &[space, argument]);
    data.op_insert(live, block, None);
    apply(
        &mut data,
        vec![Guard {
            addr,
            size: 4,
            writes: vec![value],
            loads: vec![live],
        }],
    );
    assert!(data.vbank().get(value).unwrap().is_addr_force());
}
