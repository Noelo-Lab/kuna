//! Preserve proven volatile loads and discarded native reads through unresolved pointers.
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

/// Whether a LOAD repeats an operand read already represented at this instruction:
/// another live read at the same instruction address accesses the same storage
/// and either `kept` holds for it or it was lifted first, so one read per
/// address per instruction survives however often the SLEIGH flag macros
/// re-load an operand.
pub(crate) fn rereads(
    data: &Funcdata,
    id: OpId,
    memo: &mut Memo,
    kept: impl Fn(&Funcdata, OpId) -> bool,
) -> bool {
    let Some(op) = data.obank().get(id) else {
        return false;
    };
    let want = target(data, id, memo);
    if want.is_none()
        && op.get_addlflags() & crate::op::pcodeop_addlflags::kuna_native_load == 0
        && !is_volatile_with(data, id, memo)
    {
        return false;
    }
    let time = op.get_time();
    data.obank()
        .iter_at(op.get_addr())
        .map(|(_, sibling)| sibling)
        .filter(|&sibling| sibling != id)
        .any(|sibling| {
            data.obank().get(sibling).is_some_and(|o| {
                let same = if let Some(want) = want {
                    target(data, sibling, memo) == Some(want)
                } else {
                    let space_slot = if o.code() == OpCode::CPUI_LOAD {
                        Some(0)
                    } else if o.code() == OpCode::CPUI_CALLOTHER
                        && o.get_in(0).and_then(|v| data.vbank().get(v))
                            .is_some_and(|v| v.get_offset() == crate::userop::BUILTIN_VOLATILE_READ as u64)
                        && o.num_input() == 3
                    {
                        Some(2)
                    } else {
                        None
                    };
                    space_slot.is_some()
                        && o.get_in(1) == op.get_in(1)
                        && space_slot.and_then(|slot| o.get_in(slot))
                            .and_then(|vn| data.vbank().get(vn)).map(|vn| vn.get_offset())
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

/// The dynamic address and access width of a retained native read effect.
pub(crate) fn dynamic_read(data: &Funcdata, id: OpId) -> Option<(VarnodeId, i32)> {
    let op = data.obank().get(id)?;
    if op.code() != OpCode::CPUI_CALLOTHER || op.num_input() != 3
        || op.get_addlflags() & crate::op::pcodeop_addlflags::kuna_native_load == 0
        || data.vbank().get(op.get_in(0)?)?.get_offset() != crate::userop::BUILTIN_VOLATILE_READ as u64
    {
        return None;
    }
    let address = op.get_in(1)?;
    if data.vbank().get(address)?.is_annotation() {
        return None;
    }
    Some((address, data.vbank().get(op.get_out()?)?.get_size()))
}

// Spacebase/input flags can be established after the first removal pass. The
// ABI stack register itself already proves ordinary fixed-offset frame reads.
fn stack_register_address(
    data: &Funcdata,
    id: VarnodeId,
    register: (i32, u64, i32),
    budget: &mut u32,
) -> bool {
    if *budget == 0 { return false; }
    *budget -= 1;
    let Some(value) = data.vbank().get(id) else { return false };
    if value.get_space().get_index() == register.0
        && value.get_offset() == register.1 && value.get_size() == register.2
    {
        return true;
    }
    let Some(op) = value.get_def().and_then(|id| data.obank().get(id)) else { return false };
    let Some(first) = op.get_in(0) else { return false };
    let constant_input = |slot| op.get_in(slot).and_then(|v| data.vbank().get(v))
        .is_some_and(|v| v.is_constant());
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
            data.vbank().get(first).is_some_and(|v| v.get_size() == value.get_size())
                && stack_register_address(data, first, register, budget)
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB
            if constant_input(1) => stack_register_address(data, first, register, budget),
        OpCode::CPUI_INT_ADD if constant_input(0) => op.get_in(1)
            .is_some_and(|v| stack_register_address(data, v, register, budget)),
        OpCode::CPUI_PTRADD if constant_input(1) && constant_input(2) =>
            stack_register_address(data, first, register, budget),
        _ => false,
    }
}

/// Preserve an unread native scalar access through an unresolved external
/// pointer. Its address may denote MMIO even without recovered C qualifiers.
/// Fixed storage and proven frame reads keep their existing removal rules.
pub(crate) fn preserve_discarded(data: &mut Funcdata, id: OpId, memo: &mut Memo) -> bool {
    if data.is_jumptable_recovery_on() {
        return false;
    }
    let Some(op) = data.obank().get(id) else { return false };
    if op.code() != OpCode::CPUI_LOAD
        || op.get_addlflags() & crate::op::pcodeop_addlflags::kuna_native_load == 0
        || op.uses_spacebase_ptr()
    {
        return false;
    }
    let (Some(pointer), Some(space), Some(output)) = (op.get_in(1), op.get_in(0), op.get_out()) else {
        return false;
    };
    let Some(size) = data.vbank().get(output).map(|v| v.get_size()) else { return false };
    if !matches!(size, 1 | 2 | 4 | 8) {
        return false;
    }
    let Some(space_index) = data.vbank().get(space).map(|v| v.get_offset()) else { return false };
    let manage = data.get_arch().manage();
    if space_index >= manage.num_spaces() as u64
        || manage.get_space(space_index as i32).is_none_or(|s| {
            s.get_word_size() != 1 || s.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR
        })
    {
        return false;
    }
    let mut budget = BUDGET;
    if manage.get_stack_space().and_then(|s| s.get_spacebase(0).ok())
        .and_then(|base| base.space.map(|s| (s.get_index(), base.offset, base.size as i32)))
        .is_some_and(|register| stack_register_address(data, pointer, register, &mut budget))
    {
        return false;
    }
    budget = BUDGET;
    if constant(data, pointer, memo, &mut BTreeSet::new(), &mut budget).is_some()
        || crate::kuna_stackranges::frame_address(data, pointer).is_some()
        || rereads(data, id, memo, |data, other| {
            data.obank().get(other).and_then(|op| op.get_out())
                .and_then(|v| data.vbank().get(v))
                .is_some_and(|v| !v.has_no_descend() || v.is_auto_live()
                    || (v.is_consume_vacuous() && v.get_consume() != 0))
        })
    {
        return false;
    }
    // Keep the address space as metadata, separate from the dynamic address.
    let builtin = data.new_constant(4, crate::userop::BUILTIN_VOLATILE_READ as u64);
    data.op_insert_input(id, space, 2).expect("retained read space");
    data.op_set_input(id, builtin, 0).expect("retained read builtin");
    data.op_set_opcode_code(id, OpCode::CPUI_CALLOTHER);
    data.obank_mut().get_mut(id).expect("retained read").set_hold_output();
    true
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
