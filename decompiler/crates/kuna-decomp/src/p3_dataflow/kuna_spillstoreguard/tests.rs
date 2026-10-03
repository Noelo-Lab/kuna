use super::*;
use crate::context::ArchContext;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace,
    SpacebaseSpace, UniqueSpace, VarnodeStorage,
};

fn function(word_size: u32) -> (Funcdata, Rc<AddrSpace>, VarnodeId) {
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
        word_size,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&ram)).unwrap();
    manager.insert_space(Rc::new(IopSpace::new(3))).unwrap();
    let register = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        4,
        0,
        0,
        0,
    ));
    manager.insert_space(Rc::clone(&register)).unwrap();
    let stack = Rc::new(SpacebaseSpace::new("stack", 5, 4, &ram, 1, true, false));
    manager
        .add_spacebase_pointer(
            &stack,
            &VarnodeStorage {
                space: Some(Rc::clone(&register)),
                offset: 0,
                size: 4,
            },
            4,
            true,
        )
        .unwrap();
    manager.insert_space(Rc::clone(&stack)).unwrap();
    let arch = Rc::new(ArchContext::new(manager));
    let mut data = Funcdata::new(
        "spill",
        "spill",
        arch,
        Address::new(ram, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    let root = data.bblocks_ref().root.unwrap();
    data.bblocks_mut().new_block_basic(root);
    let sp = data.new_varnode(4, &Address::new(register, 0), None);
    let sp = data.set_input_varnode(sp).unwrap();
    data.vbank_mut().get_mut(sp).unwrap().set_spacebase();
    (data, stack, sp)
}

fn expression(data: &mut Funcdata, code: OpCode, inputs: &[VarnodeId], size: int4) -> VarnodeId {
    let op = data.new_op(inputs.len() as int4, data.get_address().clone());
    data.op_set_opcode(op, crate::typeop::type_op_for(code));
    let out = data.new_unique_out(size, op).unwrap();
    for (slot, &input) in inputs.iter().enumerate() {
        data.op_set_input(op, input, slot as int4).unwrap();
    }
    let block = data.bblocks_ref().root.unwrap();
    let block = data.bblocks_ref().sub_block(block, 0).unwrap();
    data.op_insert(op, block, None);
    out
}

#[test]
fn stack_destination_proof_masks_full_width_affine_pointers_only() {
    let (mut data, stack, sp) = function(1);
    for displacement in [0, 0x20, 0xfffffff0] {
        let c = data.new_constant(4, displacement);
        let ptr = expression(&mut data, OpCode::CPUI_INT_ADD, &[sp, c], 4);
        assert_eq!(offset(&data, ptr, &stack, 64), Some(displacement));
    }
    let minus = data.new_constant(4, 0xfffffff0);
    let ptr = expression(&mut data, OpCode::CPUI_INT_ADD, &[sp, minus], 4);
    let plus = data.new_constant(4, 0x20);
    let wrapped = expression(&mut data, OpCode::CPUI_INT_ADD, &[ptr, plus], 4);
    assert_eq!(offset(&data, wrapped, &stack, 64), Some(0x10));
    let short = expression(&mut data, OpCode::CPUI_COPY, &[sp], 2);
    assert_eq!(offset(&data, short, &stack, 64), None);
    for incoming in [[sp, sp], [sp, ptr]] {
        let phi = expression(&mut data, OpCode::CPUI_MULTIEQUAL, &incoming, 4);
        assert_eq!(offset(&data, phi, &stack, 64), None);
    }
    let external = expression(&mut data, OpCode::CPUI_CALL, &[], 4);
    let changed = expression(&mut data, OpCode::CPUI_COPY, &[external], 4);
    assert_eq!(offset(&data, changed, &stack, 64), None);
    let (data, stack, sp) = function(2);
    assert_eq!(offset(&data, sp, &stack, 64), None);
}

#[test]
fn provisional_guards_keep_only_exact_nonwrapping_stack_overlaps() {
    for (destination, guarded, kept) in [
        (0xfffffff0, 0xfffffff0, true),
        (0xfffffff0, 0, false),
        (0x20, 0x20, true),
        (0x20, 0x1c, false),
        (0x20, 0x22, true),
        (0xfffffffe, 0, false),
    ] {
        let (mut data, stack, sp) = function(1);
        let c = data.new_constant(4, destination);
        let ptr = expression(&mut data, OpCode::CPUI_INT_ADD, &[sp, c], 4);
        let store = data.new_op(3, data.get_address().clone());
        data.op_set_opcode(store, crate::typeop::type_op_for(OpCode::CPUI_STORE));
        let ram = data.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
        let value = data.new_constant(4, 9);
        for (slot, input) in [ram, ptr, value].into_iter().enumerate() {
            data.op_set_input(store, input, slot as int4).unwrap();
        }
        let block = data.bblocks_ref().root.unwrap();
        let block = data.bblocks_ref().sub_block(block, 0).unwrap();
        data.op_insert(store, block, None);
        let mut guards = Vec::new();
        build(
            &mut data,
            &Address::new(stack, guarded),
            4,
            &[store],
            &[],
            &std::collections::HashMap::new(),
            &mut guards,
            &mut Vec::new(),
        );
        assert_eq!(guards.len(), 1);
        let indirect = guards[0].0;
        prune(&mut data, &guards);
        assert_eq!(
            data.obank().get(indirect).is_some_and(|op| !op.is_dead()),
            kept,
            "destination={destination:x}, guarded={guarded:x}"
        );
    }
}

fn spilled_store(
    data: &mut Funcdata,
    stack: &Rc<AddrSpace>,
    sp: VarnodeId,
    slot: u64,
    destination: u64,
) -> OpId {
    let count = data.new_constant(4, destination);
    let pointer = expression(data, OpCode::CPUI_INT_ADD, &[sp, count], 4);
    let copy = data.new_op(1, data.get_address().clone());
    data.op_set_opcode(copy, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    data.new_varnode_out(4, &Address::new(Rc::clone(stack), slot), copy)
        .unwrap();
    data.op_set_input(copy, pointer, 0).unwrap();
    let block = data
        .bblocks_ref()
        .sub_block(data.bblocks_ref().root.unwrap(), 0)
        .unwrap();
    data.op_insert(copy, block, None);
    let store = data.new_op(3, data.get_address().clone());
    data.op_set_opcode(store, crate::typeop::type_op_for(OpCode::CPUI_STORE));
    let space = data.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
    let reload = data.new_varnode(4, &Address::new(Rc::clone(stack), slot), None);
    let value = data.new_constant(4, 9);
    for (i, vn) in [space, reload, value].into_iter().enumerate() {
        data.op_set_input(store, vn, i as int4).unwrap();
    }
    data.op_insert(store, block, None);
    store
}

#[test]
fn closed_frame_prefilter_excludes_pointer_slot_mutation_and_partial_writes() {
    let (mut data, stack, sp) = function(1);
    let stores: Vec<_> = (0..16)
        .map(|i| spilled_store(&mut data, &stack, sp, 0x100 + 4 * i, 0x400 + 4 * i))
        .collect();
    let limited = bounds(&data, &stores);
    assert_eq!(limited.len(), 16);
    let mut guards = Vec::new();
    build(
        &mut data,
        &Address::new(Rc::clone(&stack), 0x400),
        4,
        &stores,
        &[],
        &limited,
        &mut guards,
        &mut Vec::new(),
    );
    assert_eq!(guards.len(), 1);
    let patch = data.new_op(1, data.get_address().clone());
    data.op_set_opcode(patch, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    data.new_varnode_out(2, &Address::new(Rc::clone(&stack), 0x102), patch)
        .unwrap();
    let value = data.new_constant(2, 0);
    data.op_set_input(patch, value, 0).unwrap();
    let block = data
        .bblocks_ref()
        .sub_block(data.bblocks_ref().root.unwrap(), 0)
        .unwrap();
    data.op_insert(patch, block, None);
    assert!(bounds(&data, &stores).is_empty());

    let (mut data, stack, sp) = function(1);
    let mut stores: Vec<_> = (0..16)
        .map(|i| spilled_store(&mut data, &stack, sp, 0x100 + 4 * i, 0x400 + 4 * i))
        .collect();
    stores.push(spilled_store(&mut data, &stack, sp, 0x180, 0x100));
    assert!(bounds(&data, &stores).is_empty());

    let (mut data, stack, sp) = function(1);
    let mut stores: Vec<_> = (0..16)
        .map(|i| spilled_store(&mut data, &stack, sp, 0x100 + 4 * i, 0x400 + 4 * i))
        .collect();
    stores.push(spilled_store(&mut data, &stack, sp, 0x100, 0x800));
    assert!(bounds(&data, &stores).is_empty());
}

#[test]
fn unresolved_large_frames_decline_the_whole_new_guard_set() {
    for barrier in [
        OpCode::CPUI_CALL,
        OpCode::CPUI_CALLIND,
        OpCode::CPUI_CALLOTHER,
    ] {
        let (mut data, stack, sp) = function(1);
        let stores: Vec<_> = (0..256)
            .map(|i| spilled_store(&mut data, &stack, sp, 0x1000 + 4 * i, 0x4000 + 4 * i))
            .collect();
        let limited = bounds(&data, &stores);
        assert_eq!(limited.len(), 256);
        assert!(within_budget(&data, &stores, &limited));
        expression(&mut data, barrier, &[], 4);
        assert!(bounds(&data, &stores).is_empty(), "{barrier:?}");
        assert!(!within_budget(
            &data,
            &stores,
            &std::collections::HashMap::new()
        ));
    }
}
