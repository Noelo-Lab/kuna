use super::*;
use crate::action::Rule;
use crate::context::ArchContext;
use crate::p3_dataflow::ruleaction_4::RuleStoreVarnode;
use kuna_base::address::{calc_mask, Address};
use kuna_base::space::{
    addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace,
};
use std::rc::Rc;

fn function(big: bool) -> Funcdata {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    manager
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            big,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    manager.insert_space(Rc::new(IopSpace::new(3))).unwrap();
    let arch = Rc::new(ArchContext::new(manager));
    let addr = Address::new(
        arch.manage().get_space_by_name("ram").unwrap().clone(),
        0x1000,
    );
    Funcdata::new("store", "store", arch, addr, 0x10000000, 0x40).unwrap()
}

fn value(data: &Funcdata, vn: VarnodeId) -> u64 {
    let v = data.vbank().get(vn).unwrap();
    if v.is_constant() {
        return v.get_offset();
    }
    let op = data.obank().get(v.get_def().unwrap()).unwrap();
    let first = value(data, op.get_in(0).unwrap());
    let computed = match op.code() {
        OpCode::CPUI_COPY => first,
        OpCode::CPUI_SUBPIECE => first >> (8 * value(data, op.get_in(1).unwrap())),
        OpCode::CPUI_PIECE => {
            let second = op.get_in(1).unwrap();
            (first << (8 * data.vbank().get(second).unwrap().get_size())) | value(data, second)
        }
        code => panic!("unexpected computation {code:?}"),
    };
    computed & calc_mask(v.get_size())
}

#[test]
fn store_conversion_preserves_every_overlapping_byte_in_both_endiannesses() {
    for big in [false, true] {
        for (write_off, write_size, effect_off, effect_size) in [
            (0x100, 8, 0x100, 8),
            (0x100, 8, 0x102, 4),
            (0x100, 4, 0x100, 8),
            (0x104, 4, 0x100, 8),
            (0x103, 2, 0x100, 8),
            (0xfd, 6, 0x100, 8),
            (0x106, 6, 0x100, 8),
            (0x110, 4, 0x100, 8),
        ] {
            let mut data = function(big);
            let ram = data
                .get_arch()
                .manage()
                .get_space_by_name("ram")
                .unwrap()
                .clone();
            let root = data.bblocks_ref().root.unwrap();
            let block = data.bblocks_mut().new_block_basic(root);
            let pc = data.get_address().clone();
            let store = data.new_op(3, pc.clone());
            data.op_set_opcode(store, crate::typeop::type_op_for(OpCode::CPUI_STORE));
            let space = data.new_constant(8, ram.get_index() as u64);
            let address = data.new_constant(8, write_off);
            let written_value = 0x6f5e4d3c2b1a0908 & calc_mask(write_size);
            let written = data.new_constant(write_size, written_value);
            data.op_set_input(store, space, 0).unwrap();
            data.op_set_input(store, address, 1).unwrap();
            data.op_set_input(store, written, 2).unwrap();
            data.op_insert(store, block, None);
            let effect =
                data.new_indirect_op(store, &Address::new(ram, effect_off), effect_size, 0);
            let old_value = 0x8877665544332211 & calc_mask(effect_size);
            let old = data.new_constant(effect_size, old_value);
            data.op_set_input(effect, old, 0).unwrap();
            let out = data.obank().get(effect).unwrap().get_out().unwrap();
            let reader = data.new_op(1, pc);
            data.op_set_opcode(reader, crate::typeop::type_op_for(OpCode::CPUI_COPY));
            let result = data.new_unique_out(effect_size, reader).unwrap();
            data.op_set_input(reader, out, 0).unwrap();
            data.op_insert(reader, block, None);
            assert_eq!(RuleStoreVarnode::new().apply_op(store, &mut data), 1);
            let mut expected = old_value;
            for byte in 0..effect_size {
                let address = effect_off + byte as u64;
                if address >= write_off && address < write_off + write_size as u64 {
                    let write_byte = (address - write_off) as i32;
                    let source_shift = 8 * if big {
                        write_size - write_byte - 1
                    } else {
                        write_byte
                    };
                    let target_shift = 8 * if big { effect_size - byte - 1 } else { byte };
                    expected = (expected & !(0xffu64 << target_shift))
                        | (((written_value >> source_shift) & 0xff) << target_shift);
                }
            }
            assert_eq!(value(&data, result), expected, "big={big}, write=({write_off:x},{write_size}), effect=({effect_off:x},{effect_size})");
            if let Some(marker) = data.obank().get(effect) {
                if !marker.is_dead() {
                    assert_ne!(marker.code(), OpCode::CPUI_INDIRECT);
                    let ops = data.bb_ops(block);
                    assert!(
                        ops.iter().position(|&op| op == store)
                            < ops.iter().position(|&op| op == effect)
                    );
                    assert!(
                        ops.iter().position(|&op| op == effect)
                            < ops.iter().position(|&op| op == reader)
                    );
                }
            }
        }
    }
}

#[test]
fn neighboring_indirects_for_another_operation_remain_unchanged() {
    let mut data = function(false);
    let ram = data
        .get_arch()
        .manage()
        .get_space_by_name("ram")
        .unwrap()
        .clone();
    let root = data.bblocks_ref().root.unwrap();
    let block = data.bblocks_mut().new_block_basic(root);
    let pc = data.get_address().clone();
    let other = data.new_op(0, pc.clone());
    data.op_set_opcode(other, crate::typeop::type_op_for(OpCode::CPUI_CALL));
    data.op_insert(other, block, None);
    let copy = data.new_op(1, pc);
    data.op_set_opcode(copy, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    let written = data.new_constant(4, 9);
    data.op_set_input(copy, written, 0).unwrap();
    data.new_varnode_out(4, &Address::new(ram.clone(), 0x100), copy)
        .unwrap();
    data.op_insert(copy, block, None);
    let unrelated = data.new_indirect_op(other, &Address::new(ram, 0x100), 4, 0);
    data.op_uninsert(unrelated);
    data.op_insert_before(unrelated, copy);
    let old = data.obank().get(unrelated).unwrap().get_in(0);
    collapse(&mut data, copy);
    assert_eq!(
        data.obank().get(unrelated).unwrap().code(),
        OpCode::CPUI_INDIRECT
    );
    assert_eq!(data.obank().get(unrelated).unwrap().get_in(0), old);
}
