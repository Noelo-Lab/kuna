//! Address-width normalization must preserve independent full-width values.
use super::*;
use crate::context::ArchContext;
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use std::rc::Rc;

fn fixture(big_endian: bool) -> (Funcdata, BlockId) {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, big_endian)))
        .unwrap();
    for (name, index) in [("ram", 2), ("register", 3), ("wide", 4)] {
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                name,
                big_endian,
                8,
                1,
                index,
                addrspace_flags::hasphysical,
                1,
                1,
            )))
            .unwrap();
    }
    manager.set_default_code_space(2).unwrap();
    manager.truncate_space("ram", 4).unwrap();
    let entry = Address::new(Rc::clone(manager.get_default_code_space().unwrap()), 0x1000);
    let mut fd = Funcdata::new(
        "probe",
        "probe",
        Rc::new(ArchContext::new(manager)),
        entry,
        0x10000000,
        0x40,
    )
    .unwrap();
    let root = fd.bblocks_ref().root.unwrap();
    let block = fd.bblocks_mut().new_block_basic(root);
    (fd, block)
}

fn input(fd: &mut Funcdata, storage: &str, size: i32) -> VarnodeId {
    if storage == "constant" {
        return fd.new_constant(size, 0xffff_ffff_8000_1234);
    }
    let space = Rc::clone(fd.get_arch().manage().get_space_by_name(storage).unwrap());
    let vn = fd.new_varnode(size, &Address::new(space, 0x100), None);
    fd.set_input_varnode(vn).unwrap()
}

fn access(
    fd: &mut Funcdata,
    block: BlockId,
    opcode: OpCode,
    pointer: VarnodeId,
    space: &str,
) -> (OpId, i32) {
    let slot = i32::from(matches!(opcode, OpCode::CPUI_LOAD | OpCode::CPUI_STORE));
    let n = if opcode == OpCode::CPUI_STORE {
        3
    } else {
        slot + 1
    };
    let op = fd.new_op(n, fd.get_address().clone());
    fd.op_set_opcode_code(op, opcode);
    if slot == 1 {
        let index = fd
            .get_arch()
            .manage()
            .get_space_by_name(space)
            .unwrap()
            .get_index();
        let id = fd.new_constant(8, index as u64);
        fd.op_set_input(op, id, 0).unwrap();
    }
    fd.op_set_input(op, pointer, slot).unwrap();
    if opcode == OpCode::CPUI_STORE {
        let value = fd.new_constant(8, 0x123456789abcdef0);
        fd.op_set_input(op, value, 2).unwrap();
    } else if opcode == OpCode::CPUI_LOAD {
        fd.new_unique_out(8, op).unwrap();
    }
    fd.op_insert_begin(op, block);
    (op, slot)
}

#[test]
fn address_uses_take_low_bytes_without_rewriting_other_uses() {
    for be in [false, true] {
        for storage in ["register", "unique", "constant"] {
            for opcode in [
                OpCode::CPUI_LOAD,
                OpCode::CPUI_STORE,
                OpCode::CPUI_CALLIND,
                OpCode::CPUI_BRANCHIND,
            ] {
                let (mut fd, block) = fixture(be);
                let full = input(&mut fd, storage, 8);
                let (op, slot) = access(&mut fd, block, opcode, full, "ram");
                let observer = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(observer, OpCode::CPUI_COPY);
                fd.op_set_input(observer, full, 0).unwrap();
                fd.new_unique_out(8, observer).unwrap();
                fd.op_insert_after(observer, op);
                let mut rule = RulePtrFlow::new();
                assert_eq!(rule.apply_op(op, &mut fd), 1);
                let narrow = fd.obank().get(op).unwrap().get_in(slot).unwrap();
                let value = fd.vbank().get(narrow).unwrap();
                assert_eq!(value.get_size(), 4);
                if storage == "constant" {
                    assert!(value.is_constant());
                    assert_eq!(value.get_offset(), 0x80001234);
                } else {
                    let trunc = fd.obank().get(value.get_def().unwrap()).unwrap();
                    assert_eq!(trunc.code(), OpCode::CPUI_SUBPIECE);
                    assert_eq!(trunc.get_in(0), Some(full));
                    assert_eq!(
                        fd.vbank()
                            .get(trunc.get_in(1).unwrap())
                            .unwrap()
                            .get_offset(),
                        0
                    );
                    if storage == "register" {
                        assert_eq!(value.get_offset(), if be { 0x104 } else { 0x100 });
                    }
                }
                let observed = fd.obank().get(observer).unwrap().get_in(0).unwrap();
                if storage == "constant" {
                    assert_eq!(
                        fd.vbank().get(observed).unwrap().get_offset(),
                        0xffffffff80001234
                    );
                    assert_eq!(fd.vbank().get(observed).unwrap().get_size(), 8);
                } else {
                    assert_eq!(observed, full, "{be} {storage} {opcode:?}");
                }
                assert_eq!(fd.vbank().get(full).unwrap().get_size(), 8);
                assert_eq!(rule.apply_op(op, &mut fd), 0, "repeated normalization");
            }
        }
    }
}

#[test]
fn each_access_uses_its_space_width_without_widening_small_pointers() {
    for (space, pointer_size) in [("wide", 8), ("ram", 4), ("ram", 2)] {
        let (mut fd, block) = fixture(false);
        let pointer = input(&mut fd, "register", pointer_size);
        let (op, slot) = access(&mut fd, block, OpCode::CPUI_LOAD, pointer, space);
        RulePtrFlow::new().apply_op(op, &mut fd);
        assert_eq!(fd.obank().get(op).unwrap().get_in(slot), Some(pointer));
        assert_eq!(fd.vbank().get(pointer).unwrap().get_size(), pointer_size);
    }
}

#[test]
fn pointer_subpieces_do_not_narrow_full_width_consumers() {
    use crate::subflow::RuleSubvarSubpiece;

    for be in [false, true] {
        for consumer in [
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_RETURN,
        ] {
            let (mut fd, block) = fixture(be);
            let full = input(&mut fd, "register", 8);
            fd.vbank_mut().get_mut(full).unwrap().set_consume(u64::MAX);
            let (load, _) = access(&mut fd, block, OpCode::CPUI_LOAD, full, "ram");
            let observer = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(observer, consumer);
            let other = fd.new_constant(
                8,
                if consumer == OpCode::CPUI_RETURN {
                    0
                } else {
                    7
                },
            );
            let slot = i32::from(consumer == OpCode::CPUI_RETURN);
            fd.op_set_input(observer, full, slot).unwrap();
            fd.op_set_input(observer, other, 1 - slot).unwrap();
            if consumer != OpCode::CPUI_RETURN {
                fd.new_unique_out(1, observer).unwrap();
            }
            fd.op_insert_after(observer, load);
            RulePtrFlow::new().apply_op(load, &mut fd);
            let narrow = fd.obank().get(load).unwrap().get_in(1).unwrap();
            assert!(fd.vbank().get(narrow).unwrap().is_ptr_flow());
            let trunc = fd.vbank().get(narrow).unwrap().get_def().unwrap();
            assert_eq!(
                RuleSubvarSubpiece::new("subvar").apply_op(trunc, &mut fd),
                0
            );
            assert_eq!(fd.obank().get(observer).unwrap().get_in(slot), Some(full));
            assert_eq!(fd.vbank().get(full).unwrap().get_size(), 8);
        }
    }
}

#[test]
fn extended_pointer_comparison_keeps_the_other_operands_upper_bits() {
    use crate::subflow::RuleSubvarZext;

    let (mut fd, block) = fixture(true);
    let low = input(&mut fd, "register", 4);
    fd.vbank_mut().get_mut(low).unwrap().set_ptr_flow();
    let zext = fd.new_op(1, fd.get_address().clone());
    fd.op_set_opcode_code(zext, OpCode::CPUI_INT_ZEXT);
    fd.op_set_input(zext, low, 0).unwrap();
    let full = fd.new_unique_out(8, zext).unwrap();
    fd.vbank_mut().get_mut(full).unwrap().set_consume(u64::MAX);
    fd.op_insert_begin(zext, block);
    let other = input(&mut fd, "unique", 8);
    fd.vbank_mut().get_mut(other).unwrap().set_consume(u64::MAX);
    let compare = fd.new_op(2, fd.get_address().clone());
    fd.op_set_opcode_code(compare, OpCode::CPUI_INT_EQUAL);
    fd.op_set_input(compare, full, 0).unwrap();
    fd.op_set_input(compare, other, 1).unwrap();
    fd.new_unique_out(1, compare).unwrap();
    fd.op_insert_after(compare, zext);
    assert_eq!(RuleSubvarZext::new("subvar").apply_op(zext, &mut fd), 0);
    assert_eq!(fd.obank().get(compare).unwrap().get_in(1), Some(other));
    assert_eq!(fd.vbank().get(other).unwrap().get_size(), 8);
}
