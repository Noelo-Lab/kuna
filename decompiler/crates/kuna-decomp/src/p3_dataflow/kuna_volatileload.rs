//! Keep a proven volatile load live while its constant address is still an SSA expression.
use crate::{
    context::{OpId, VarnodeId},
    funcdata::Funcdata,
    varnode::varnode_flags,
};
use kuna_base::{
    address::{calc_mask, Address},
    space::AddrSpace,
};
use kuna_num::opcodes::OpCode;
use std::collections::BTreeMap;

fn constant(
    data: &Funcdata,
    id: VarnodeId,
    memo: &mut BTreeMap<VarnodeId, Option<u64>>,
) -> Option<u64> {
    if let Some(value) = memo.get(&id) {
        return *value;
    }
    if memo.len() >= 64 {
        return None;
    }
    memo.insert(id, None);
    let node = data.vbank().get(id)?;
    if !(1..=8).contains(&node.get_size()) {
        return None;
    }
    let value = if node.is_constant() {
        node.get_offset()
    } else {
        let op = data.obank().get(node.get_def()?)?;
        let input = op.get_in(0)?;
        let a = constant(data, input, memo)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => a,
            OpCode::CPUI_INT_SEXT => {
                let bits = data.vbank().get(input)?.get_size() * 8;
                ((a << (64 - bits)) as i64 >> (64 - bits)) as u64
            }
            OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_MULT => {
                let b = constant(data, op.get_in(1)?, memo)?;
                match op.code() {
                    OpCode::CPUI_INT_ADD => a.wrapping_add(b),
                    OpCode::CPUI_INT_SUB => a.wrapping_sub(b),
                    OpCode::CPUI_INT_AND => a & b,
                    OpCode::CPUI_INT_OR => a | b,
                    OpCode::CPUI_INT_XOR => a ^ b,
                    OpCode::CPUI_INT_LEFT => {
                        if b >= 64 {
                            0
                        } else {
                            a << b
                        }
                    }
                    OpCode::CPUI_INT_RIGHT => {
                        if b >= 64 {
                            0
                        } else {
                            a >> b
                        }
                    }
                    _ => a.wrapping_mul(b),
                }
            }
            _ => return None,
        }
    } & calc_mask(node.get_size());
    memo.insert(id, Some(value));
    Some(value)
}

/// Prove the access address without reading memory or guessing an input register.
pub(crate) fn is_volatile(data: &Funcdata, id: OpId) -> bool {
    let Some(op) = data
        .obank()
        .get(id)
        .filter(|o| o.code() == OpCode::CPUI_LOAD)
    else {
        return false;
    };
    let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    let Some(space) = op
        .get_in(0)
        .and_then(|v| data.vbank().get(v))
        .filter(|v| v.is_constant())
    else {
        return false;
    };
    let manager = data.get_arch().manage();
    if space.get_offset() >= manager.num_spaces() as u64 {
        return false;
    }
    let Some(space) = manager.get_space(space.get_offset() as i32) else {
        return false;
    };
    let Some(pointer) = op.get_in(1) else {
        return false;
    };
    let Some(offset) = constant(data, pointer, &mut BTreeMap::new()) else {
        return false;
    };
    let addr = Address::new(
        space.clone(),
        AddrSpace::address_to_byte(offset, space.get_word_size()),
    );
    let properties = data.query_local_properties(&addr, out.get_size(), op.get_addr())
        | data
            .get_arch()
            .query_global_properties(&addr, out.get_size(), op.get_addr());
    properties & varnode_flags::volatil != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{ArchContext, GlobalQuery};
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use std::rc::Rc;

    fn function(word_size: u32) -> Funcdata {
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
        manager.insert_space(ram.clone()).unwrap();
        let mut arch = ArchContext::new(manager);
        let mut query = GlobalQuery::default();
        *query.flagbase.split(&Address::new(ram.clone(), 0x50000004)) = 0;
        *query.flagbase.split(&Address::new(ram.clone(), 0x50000000)) = varnode_flags::volatil;
        arch.global_query = Some(Rc::new(query));
        Funcdata::new(
            "read",
            "read",
            Rc::new(arch),
            Address::new(ram, 0x1000),
            0x10000000,
            0x40,
        )
        .unwrap()
    }
    fn operation(
        fd: &mut Funcdata,
        opcode: OpCode,
        size: i32,
        inputs: &[VarnodeId],
    ) -> (OpId, VarnodeId) {
        let op = fd.new_op(inputs.len() as i32, fd.get_address().clone());
        fd.op_set_opcode_code(op, opcode);
        for (i, &vn) in inputs.iter().enumerate() {
            fd.op_set_input(op, vn, i as i32).unwrap();
        }
        let vn = fd.new_unique_out(size, op).unwrap();
        (op, vn)
    }
    #[test]
    fn constant_proof_respects_widths_unknowns_cycles_and_budget() {
        let mut fd = function(1);
        let a = fd.new_constant(4, 0xfffffff0);
        let b = fd.new_constant(4, 0x50000010);
        let (_, mut value) = operation(&mut fd, OpCode::CPUI_INT_ADD, 4, &[a, b]);
        assert_eq!(constant(&fd, value, &mut BTreeMap::new()), Some(0x50000000));
        let neg = fd.new_constant(1, 0x80);
        let (_, sext) = operation(&mut fd, OpCode::CPUI_INT_SEXT, 8, &[neg]);
        assert_eq!(
            constant(&fd, sext, &mut BTreeMap::new()),
            Some(0xffffffffffffff80)
        );
        let unknown = fd.new_unique(4, None);
        let (_, unproved) = operation(&mut fd, OpCode::CPUI_INT_ADD, 4, &[value, unknown]);
        assert_eq!(constant(&fd, unproved, &mut BTreeMap::new()), None);
        let (cycle, cyclic) = operation(&mut fd, OpCode::CPUI_COPY, 4, &[value]);
        fd.op_set_input(cycle, cyclic, 0).unwrap();
        assert_eq!(constant(&fd, cyclic, &mut BTreeMap::new()), None);
        for _ in 0..64 {
            value = operation(&mut fd, OpCode::CPUI_COPY, 4, &[value]).1;
        }
        assert_eq!(constant(&fd, value, &mut BTreeMap::new()), None);
    }
    #[test]
    fn only_proven_volatile_loads_are_kept_with_word_addresses_converted() {
        for word_size in [1, 2] {
            let mut fd = function(word_size);
            let space = fd.new_constant(4, 2);
            for (offset, expected) in [(0x50000000, true), (0x50000004, false)] {
                let ptr = fd.new_constant(4, offset / word_size as u64);
                let (load, value) = operation(&mut fd, OpCode::CPUI_LOAD, 4, &[space, ptr]);
                assert_eq!(is_volatile(&fd, load), expected);
                assert_eq!(constant(&fd, value, &mut BTreeMap::new()), None);
            }
            let ptr = fd.new_unique(4, None);
            let (load, _) = operation(&mut fd, OpCode::CPUI_LOAD, 4, &[space, ptr]);
            assert!(!is_volatile(&fd, load));
            let other = fd.new_unique(4, None);
            let (copy, _) = operation(&mut fd, OpCode::CPUI_COPY, 4, &[other]);
            assert!(!is_volatile(&fd, copy));
        }
    }
}
