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
use std::collections::{BTreeMap, BTreeSet};

/// Values proven for written varnodes: `Some` is the constant, `None` a
/// varnode proven not to be one. Valid while the op graph is unchanged.
pub(crate) type Memo = BTreeMap<VarnodeId, Option<u64>>;

/// Written varnodes one proof may evaluate beyond those already in the memo.
const BUDGET: u32 = 64;

/// Fold `id` through constant integer operations. Constant leaves are free;
/// a failure caused by the exhausted budget or a cycle is not memoized.
fn constant(
    data: &Funcdata,
    id: VarnodeId,
    memo: &mut Memo,
    visiting: &mut BTreeSet<VarnodeId>,
    budget: &mut u32,
) -> Option<u64> {
    if let Some(value) = memo.get(&id) {
        return *value;
    }
    let node = data.vbank().get(id)?;
    let size = node.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    if node.is_constant() {
        return Some(node.get_offset() & calc_mask(size));
    }
    if *budget == 0 || !visiting.insert(id) {
        return None;
    }
    *budget -= 1;
    let value = node
        .get_def()
        .and_then(|def| data.obank().get(def))
        .and_then(|op| {
            let input = op.get_in(0)?;
            let a = constant(data, input, memo, visiting, budget)?;
            let value = match op.code() {
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
                    let b = constant(data, op.get_in(1)?, memo, visiting, budget)?;
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
            };
            Some(value & calc_mask(size))
        });
    visiting.remove(&id);
    if value.is_some() || *budget > 0 {
        memo.insert(id, value);
    }
    value
}

/// The volatile storage `(space index, byte offset, size)` a LOAD reads, when
/// its pointer folds to a constant without reading memory or guessing an input.
fn target(data: &Funcdata, id: OpId, memo: &mut Memo) -> Option<(i32, u64, i32)> {
    let op = data
        .obank()
        .get(id)
        .filter(|o| o.get_opcode().map(|t| t.get_opcode()) == Some(OpCode::CPUI_LOAD))?;
    let size = data.vbank().get(op.get_out()?)?.get_size();
    let space = data
        .vbank()
        .get(op.get_in(0)?)
        .filter(|v| v.is_constant())?;
    let manager = data.get_arch().manage();
    if space.get_offset() >= manager.num_spaces() as u64 {
        return None;
    }
    let space = manager.get_space(space.get_offset() as i32)?;
    let mut budget = BUDGET;
    let offset = constant(data, op.get_in(1)?, memo, &mut BTreeSet::new(), &mut budget)?;
    let addr = Address::new(
        space.clone(),
        AddrSpace::address_to_byte(offset, space.get_word_size()),
    );
    let properties = data.query_local_properties(&addr, size, op.get_addr())
        | data
            .get_arch()
            .query_global_properties(&addr, size, op.get_addr());
    (properties & varnode_flags::volatil != 0).then(|| (space.get_index(), addr.get_offset(), size))
}

/// Whether a LOAD provably reads volatile storage.
pub(crate) fn is_volatile(data: &Funcdata, id: OpId) -> bool {
    is_volatile_with(data, id, &mut Memo::new())
}

/// [`is_volatile`] sharing `memo` across the LOADs of one unchanged op graph.
pub(crate) fn is_volatile_with(data: &Funcdata, id: OpId, memo: &mut Memo) -> bool {
    if let Some(pointer) = data.get_arch().types()
        .filter(|types| types.has_volatile_types())
        .and(data.obank().get(id))
        .filter(|op| op.code() == OpCode::CPUI_LOAD)
        .and_then(|op| op.get_in(1))
    {
        let node = data.vbank().get(pointer);
        if node.and_then(|node| node.get_type().get_ptr_to())
            .is_some_and(|ty| crate::kuna_typequal::is_volatile(&ty))
        {
            return true;
        }
        let mut budget = BUDGET;
        if crate::kuna_typequal::address_type(data, pointer, &mut budget)
            .and_then(|ty| ty.get_ptr_to())
            .is_some_and(|ty| crate::kuna_typequal::is_volatile(&ty))
        {
            return true;
        }
    }
    target(data, id, memo).is_some()
}

/// Whether a volatile LOAD repeats a read its instruction already makes:
/// another live LOAD at the same instruction address reads the same storage
/// and either `kept` holds for it or it was lifted first, so one read per
/// address per instruction survives however often the SLEIGH flag macros
/// re-load an operand.
pub(crate) fn rereads(
    data: &Funcdata,
    id: OpId,
    memo: &mut Memo,
    kept: impl Fn(&Funcdata, OpId) -> bool,
) -> bool {
    let want = target(data, id, memo);
    if want.is_none() && !is_volatile_with(data, id, memo) {
        return false;
    }
    let Some(op) = data.obank().get(id) else {
        return false;
    };
    let time = op.get_time();
    let siblings: Vec<OpId> = data
        .obank()
        .iter_at(op.get_addr())
        .map(|(_, sibling)| sibling)
        .filter(|&sibling| sibling != id)
        .collect();
    siblings.into_iter().any(|sibling| {
        data.obank().get(sibling).is_some_and(|o| {
            let same = if let Some(want) = want {
                target(data, sibling, memo) == Some(want)
            } else {
                o.code() == OpCode::CPUI_LOAD
                    && o.get_in(1) == op.get_in(1)
                    && o.get_in(0).and_then(|vn| data.vbank().get(vn)).map(|vn| vn.get_offset())
                        == op.get_in(0).and_then(|vn| data.vbank().get(vn)).map(|vn| vn.get_offset())
                    && o.get_out().and_then(|vn| data.vbank().get(vn)).map(|vn| vn.get_size())
                        == op.get_out().and_then(|vn| data.vbank().get(vn)).map(|vn| vn.get_size())
            };
            !o.is_dead()
                && same
                && (kept(data, sibling) || o.get_time() < time)
        })
    })
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
    fn fold(fd: &Funcdata, id: VarnodeId) -> Option<u64> {
        let mut budget = BUDGET;
        constant(fd, id, &mut Memo::new(), &mut BTreeSet::new(), &mut budget)
    }
    #[test]
    fn constant_proof_respects_widths_unknowns_cycles_and_budget() {
        let mut fd = function(1);
        let a = fd.new_constant(4, 0xfffffff0);
        let b = fd.new_constant(4, 0x50000010);
        let (_, mut value) = operation(&mut fd, OpCode::CPUI_INT_ADD, 4, &[a, b]);
        assert_eq!(fold(&fd, value), Some(0x50000000));
        let neg = fd.new_constant(1, 0x80);
        let (_, sext) = operation(&mut fd, OpCode::CPUI_INT_SEXT, 8, &[neg]);
        assert_eq!(fold(&fd, sext), Some(0xffffffffffffff80));
        let unknown = fd.new_unique(4, None);
        let (_, unproved) = operation(&mut fd, OpCode::CPUI_INT_ADD, 4, &[value, unknown]);
        assert_eq!(fold(&fd, unproved), None);
        let (cycle, cyclic) = operation(&mut fd, OpCode::CPUI_COPY, 4, &[value]);
        fd.op_set_input(cycle, cyclic, 0).unwrap();
        assert_eq!(fold(&fd, cyclic), None);
        let mut sum = value;
        for _ in 0..63 {
            let four = fd.new_constant(4, 4);
            sum = operation(&mut fd, OpCode::CPUI_INT_ADD, 4, &[sum, four]).1;
        }
        assert_eq!(fold(&fd, sum), Some(0x50000000 + 63 * 4));
        for _ in 0..64 {
            value = operation(&mut fd, OpCode::CPUI_COPY, 4, &[value]).1;
        }
        assert_eq!(fold(&fd, value), None);
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
                assert_eq!(fold(&fd, value), None);
            }
            let ptr = fd.new_unique(4, None);
            let (load, _) = operation(&mut fd, OpCode::CPUI_LOAD, 4, &[space, ptr]);
            assert!(!is_volatile(&fd, load));
            let other = fd.new_unique(4, None);
            let (copy, _) = operation(&mut fd, OpCode::CPUI_COPY, 4, &[other]);
            assert!(!is_volatile(&fd, copy));
        }
    }
    #[test]
    fn a_second_load_of_one_address_in_an_instruction_is_a_reread() {
        let mut fd = function(1);
        let space = fd.new_constant(4, 2);
        let load = |fd: &mut Funcdata, offset: u64| {
            let ptr = fd.new_constant(4, offset);
            let (op, _) = operation(fd, OpCode::CPUI_LOAD, 4, &[space, ptr]);
            fd.obank_mut().mark_alive(op);
            op
        };
        let first = load(&mut fd, 0x50000000);
        let second = load(&mut fd, 0x50000000);
        let plain = load(&mut fd, 0x50000004);
        let memo = &mut Memo::new();
        assert!(!rereads(&fd, first, memo, |_, _| false));
        assert!(rereads(&fd, second, memo, |_, _| false));
        assert!(rereads(&fd, first, memo, |_, op| op == second));
        assert!(!rereads(&fd, plain, memo, |_, _| true));
    }
}
