//! Keep an explicitly mapped array base visible until constant-pointer recovery.
use crate::{
    context::{OpId, VarnodeId},
    dtype::type_metatype,
    funcdata::Funcdata,
    ruleaction_1::RuleCollectTerms,
};
use kuna_base::{address::Address, types::uintb};
use kuna_num::opcodes::OpCode;
use smallvec::{smallvec, SmallVec};

/// Defer folding a negative element displacement into an exact array base used
/// as a LOAD/STORE address. Other arithmetic and unmapped constants still fold.
pub(crate) fn preserve_array_base(data: &Funcdata, root: OpId) -> bool {
    if !data.get_arch().global_array_anchor
        || !data.get_arch().infer_pointers
        || !data.is_type_recovery_on()
        || data
            .kuna_global_array_anchor_passes
            .is_none_or(|passes| passes >= 4)
    {
        return false;
    }
    let Some((root, reader)) = memory_address_root(data, root) else {
        return false;
    };
    let Some(terms) = address_terms(data, root) else {
        return false;
    };
    let mut constants: SmallVec<[(VarnodeId, u64); 4]> = SmallVec::new();
    let mut variables: SmallVec<[VarnodeId; 4]> = SmallVec::new();
    for id in terms {
        if let Some(value) = literal_value(data, id, 8) {
            constants.push((id, value));
        } else {
            variables.push(id);
        }
    }
    if variables.is_empty() || constants.len() < 2 {
        return false;
    }
    let Some(output) = data.obank().get(root).and_then(|o| o.get_out()) else {
        return false;
    };
    let Some(reader) = data.obank().get(reader) else {
        return false;
    };
    if !matches!(reader.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
        || reader.get_in(1) != Some(output)
    {
        return false;
    }
    let arch = data.get_arch();
    let Some(space_id) = reader.get_in(0).and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    if !space_id.is_constant() || space_id.get_offset() >= arch.manage().num_spaces() as u64 {
        return false;
    }
    let Some(space) = arch.manage().get_space(space_id.get_offset() as i32) else {
        return false;
    };
    if space.get_word_size() != 1 {
        return false;
    }
    if !arch
        .infer_ptr_spaces()
        .iter()
        .any(|s| s.get_index() == space.get_index())
    {
        return false;
    }
    for (position, &(id, offset)) in constants.iter().enumerate() {
        let Some(value) = data.vbank().get(id) else {
            continue;
        };
        let mut compatible = arch.infer_ptr_spaces().iter().filter(|candidate| {
            let minimum = candidate.get_minimum_ptr_size();
            if minimum == 0 {
                value.get_size() == candidate.get_addr_size() as i32
            } else {
                value.get_size() >= minimum
            }
        });
        if compatible
            .next()
            .is_none_or(|candidate| candidate.get_index() != space.get_index())
            || compatible.next().is_some()
            || data.lone_descend(id).is_none()
            || value.is_spacebase()
            || value.get_size() > 8
            || value.is_ptr_check()
            || value.get_size() != space.get_addr_size() as i32
            || offset < space.get_pointer_lower_bound()
            || offset > space.get_pointer_upper_bound()
            || kuna_base::address::bit_transitions(offset, value.get_size()) < 3
        {
            continue;
        }
        if data
            .lone_descend(id)
            .and_then(|o| data.obank().get(o))
            .is_some_and(|op| {
                op.code() == OpCode::CPUI_INT_ADD
                    && (0..2).any(|slot| {
                        op.get_in(slot)
                            .and_then(|v| data.vbank().get(v))
                            .is_some_and(|v| v.is_spacebase())
                    })
            })
        {
            continue;
        }
        let address = Address::new(space.clone(), space.wrap_offset(offset));
        let Some(entry) = arch.query_container_global(&address, 1, &Address::new_invalid()) else {
            continue;
        };
        if entry.entry_addr != address {
            continue;
        }
        let Some(array) = entry
            .symbol_type
            .as_ref()
            .filter(|t| t.get_metatype() == type_metatype::TYPE_ARRAY)
        else {
            continue;
        };
        let Some(element) = array.get_array_base() else {
            continue;
        };
        let size = element.get_size();
        if size <= 0 {
            continue;
        }
        let mut displacement: uintb = 0;
        for (other_position, &(_, other)) in constants.iter().enumerate() {
            if other_position != position {
                displacement = displacement.wrapping_add(other);
            }
        }
        let mask = kuna_base::address::calc_mask(value.get_size());
        let magnitude = displacement.wrapping_neg() & mask;
        if magnitude == 0 || magnitude > array.get_size() as uintb || magnitude % size as uintb != 0
        {
            continue;
        }
        if variables.iter().all(|&id| {
            if data
                .vbank()
                .get(id)
                .is_some_and(|v| v.get_type().get_metatype() == type_metatype::TYPE_PTR)
            {
                return false;
            }
            let mut coefficient = 0;
            RuleCollectTerms::get_mult_coeff(data, id, &mut coefficient);
            if let Some(shift) = data
                .vbank()
                .get(id)
                .and_then(|v| v.get_def())
                .and_then(|o| data.obank().get(o))
            {
                if shift.code() == OpCode::CPUI_INT_LEFT {
                    if let Some(amount) = shift
                        .get_in(1)
                        .and_then(|v| data.vbank().get(v))
                        .filter(|v| v.is_constant())
                    {
                        coefficient = u32::try_from(amount.get_offset())
                            .ok()
                            .and_then(|shift| 1u64.checked_shl(shift))
                            .unwrap_or(0);
                    }
                }
            }
            let mask = data
                .vbank()
                .get(id)
                .map(|v| kuna_base::address::calc_mask(v.get_size()))
                .unwrap_or(0);
            coefficient &= mask;
            coefficient != 0 && coefficient % size as uintb == 0
        }) {
            return true;
        }
    }
    false
}

fn literal_value(data: &Funcdata, id: VarnodeId, depth: u8) -> Option<u64> {
    let value = data.vbank().get(id)?;
    if value.is_constant() {
        return Some(value.get_offset());
    }
    if depth == 0 {
        return None;
    }
    let op = data.obank().get(value.get_def()?)?;
    let first = op.get_in(0)?;
    if data.vbank().get(first)?.get_size() != value.get_size() {
        return None;
    }
    let a = literal_value(data, first, depth - 1)?;
    match op.code() {
        OpCode::CPUI_COPY => Some(a),
        OpCode::CPUI_INT_OR => {
            let second = op.get_in(1)?;
            if data.vbank().get(second)?.get_size() != value.get_size() {
                return None;
            }
            Some(a | literal_value(data, second, depth - 1)?)
        }
        _ => None,
    }
}

/// Collect additive terms through single-use copies without changing the IR.
fn address_terms(data: &Funcdata, root: OpId) -> Option<SmallVec<[VarnodeId; 8]>> {
    let output = data.obank().get(root)?.get_out()?;
    let width = data.vbank().get(output)?.get_size();
    let mut pending: SmallVec<[VarnodeId; 8]> = smallvec![output];
    let mut terms = SmallVec::new();
    for _ in 0..256 {
        let Some(id) = pending.pop() else {
            return Some(terms);
        };
        let value = data.vbank().get(id)?;
        if value.get_size() != width {
            return None;
        }
        if !value.is_constant() && address_reader(data, id).is_some() {
            if let Some(op) = value.get_def().and_then(|id| data.obank().get(id)) {
                match op.code() {
                    OpCode::CPUI_INT_ADD => {
                        pending.push(op.get_in(0)?);
                        pending.push(op.get_in(1)?);
                        continue;
                    }
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                        pending.push(op.get_in(0)?);
                        continue;
                    }
                    _ => {}
                }
            }
        }
        terms.push(id);
    }
    None
}

/// Ignore unused pure copies left behind when their readers are forwarded.
fn address_reader(data: &Funcdata, value: VarnodeId) -> Option<OpId> {
    let mut reader = None;
    for id in data.vbank().get(value)?.descend_iter() {
        let op = data.obank().get(id)?;
        if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST)
            && !op.is_indirect_source()
            && op
                .get_out()
                .and_then(|v| data.vbank().get(v))
                .is_some_and(|v| v.has_no_descend() && !v.is_auto_live())
        {
            continue;
        }
        if reader.replace(id).is_some() {
            return None;
        }
    }
    reader
}

fn memory_address_root(data: &Funcdata, mut root: OpId) -> Option<(OpId, OpId)> {
    for _ in 0..64 {
        let output = data.obank().get(root)?.get_out()?;
        let reader = address_reader(data, output)?;
        let op = data.obank().get(reader)?;
        match op.code() {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_COPY | OpCode::CPUI_CAST => root = reader,
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE if op.get_in(1) == Some(output) => {
                return Some((root, reader))
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Rule;
    use crate::context::{ArchContext, GlobalEntry, GlobalQuery};
    use crate::dtype::{Datatype, TypeFactory, TypeFactoryImpl};
    use crate::ruleaction_3::RuleCollapseConstants;
    use kuna_base::address::RangeList;
    use kuna_base::partmap::PartMap;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use std::rc::Rc;

    fn operation(data: &mut Funcdata, code: OpCode, inputs: &[VarnodeId]) -> (OpId, VarnodeId) {
        let op = data.new_op(inputs.len() as i32, data.get_address().clone());
        data.op_set_opcode(op, crate::typeop::type_op_for(code));
        for (slot, &input) in inputs.iter().enumerate() {
            data.op_set_input(op, input, slot as i32).unwrap();
        }
        let out = data.new_unique_out(8, op).unwrap();
        (op, out)
    }

    fn address_fixture() -> (Funcdata, OpId, VarnodeId) {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        manager.insert_space(ram.clone()).unwrap();
        let mut arch = ArchContext::new(manager);
        arch.global_array_anchor = true;
        arch.infer_pointers = true;
        arch.opbehaviors
            .resize_with(OpCode::CPUI_MAX as usize, || None);
        arch.opbehaviors[OpCode::CPUI_INT_ADD as usize] =
            Some(Rc::new(kuna_num::opbehavior::OpBehaviorIntAdd));
        arch.infer_ptr_spaces.push(ram.clone());
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        let array = types
            .get_type_array(2, Rc::new(Datatype::new(2, type_metatype::TYPE_INT)))
            .unwrap();
        arch.global_query = Some(Rc::new(GlobalQuery::new(
            vec![GlobalEntry {
                space_index: ram.get_index(),
                first: 0x400018,
                last: 0x40001b,
                size: 4,
                all_flags: 0,
                addrtied: true,
                uselimit: RangeList::new(),
                symbol_name: "flags".into(),
                symbol_offset: 0,
                symbol_type: Some(array),
                scope_path: Vec::new(),
                is_function: false,
                func_inject_id: -1,
                func_no_return: false,
                symbol_id: 0,
            }],
            RangeList::new(),
            PartMap::new(0),
        )));
        let mut data = Funcdata::new(
            "probe",
            "probe",
            Rc::new(arch),
            Address::new(ram.clone(), 0x10000),
            0x10000000,
            0x40,
        )
        .unwrap();
        data.set_type_recovery(true);
        data.kuna_global_array_anchor_passes = Some(0);
        let index = data.new_varnode(8, &Address::new(ram, 0x100), None);
        let two = data.new_constant(8, 2);
        let (_, scaled) = operation(&mut data, OpCode::CPUI_INT_MULT, &[index, two]);
        let base = data.new_constant(8, 0x400018);
        let negative = data.new_constant(8, u64::MAX - 1);
        let (bias, biased) = operation(&mut data, OpCode::CPUI_INT_ADD, &[base, negative]);
        let (_, copied) = operation(&mut data, OpCode::CPUI_COPY, &[biased]);
        let (_, address) = operation(&mut data, OpCode::CPUI_INT_ADD, &[copied, scaled]);
        let (_, copied_address) = operation(&mut data, OpCode::CPUI_CAST, &[address]);
        let space = data.new_constant(8, 2);
        operation(&mut data, OpCode::CPUI_LOAD, &[space, copied_address]);
        (data, bias, base)
    }

    #[test]
    fn constant_inner_bias_survives_copies_then_folds_after_recovery_budget() {
        let (mut data, bias, _) = address_fixture();
        assert!(preserve_array_base(&data, bias));
        assert_eq!(RuleCollapseConstants.apply_op(bias, &mut data), 0);
        data.kuna_global_array_anchor_passes = Some(4);
        assert_eq!(RuleCollapseConstants.apply_op(bias, &mut data), 1);
        let op = data.obank().get(bias).unwrap();
        assert_eq!(op.code(), OpCode::CPUI_COPY);
        assert_eq!(
            data.vbank()
                .get(op.get_in(0).unwrap())
                .unwrap()
                .get_offset(),
            0x400016
        );
    }

    #[test]
    fn a_declined_pointer_check_or_disabled_types_releases_the_bias() {
        let (mut data, bias, base) = address_fixture();
        assert!(preserve_array_base(&data, bias));
        data.set_type_recovery(false);
        assert!(!preserve_array_base(&data, bias));
        data.set_type_recovery(true);
        data.vbank_mut().get_mut(base).unwrap().set_ptr_check();
        assert_eq!(RuleCollapseConstants.apply_op(bias, &mut data), 1);
    }

    #[test]
    fn shared_address_temporary_does_not_hold_constants() {
        let (mut data, bias, _) = address_fixture();
        assert!(preserve_array_base(&data, bias));
        let output = data.obank().get(bias).unwrap().get_out().unwrap();
        let space = data.new_constant(8, 2);
        operation(&mut data, OpCode::CPUI_LOAD, &[space, output]);
        assert_eq!(RuleCollapseConstants.apply_op(bias, &mut data), 1);
    }

    #[test]
    fn unused_pure_copy_does_not_hide_the_memory_reader() {
        let (mut data, bias, _) = address_fixture();
        let output = data.obank().get(bias).unwrap().get_out().unwrap();
        operation(&mut data, OpCode::CPUI_COPY, &[output]);
        assert!(preserve_array_base(&data, bias));
        assert_eq!(RuleCollapseConstants.apply_op(bias, &mut data), 0);
    }

    #[test]
    fn shared_literal_temporary_is_not_held_for_pointer_recovery() {
        let (mut data, bias, base) = address_fixture();
        let (_, literal) = operation(&mut data, OpCode::CPUI_COPY, &[base]);
        data.op_set_input(bias, literal, 0).unwrap();
        let other = data.new_constant(8, 8);
        operation(&mut data, OpCode::CPUI_INT_MULT, &[literal, other]);
        assert!(!preserve_array_base(&data, bias));
    }

    #[test]
    fn a_filtered_out_pointer_action_does_not_hold_the_bias() {
        let (mut data, bias, _) = address_fixture();
        data.kuna_global_array_anchor_passes = None;
        assert_eq!(RuleCollapseConstants.apply_op(bias, &mut data), 1);
    }
}
