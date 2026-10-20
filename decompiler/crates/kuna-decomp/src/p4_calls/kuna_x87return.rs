//! Recover the scalar type of a finalized, otherwise untyped native x87 result.

use crate::dtype::{type_class, type_metatype};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub fn type_returns(data: &mut Funcdata, active: &ParamActive) {
    if data.get_func_proto().is_output_locked() {
        return;
    }
    let used: Vec<_> = (0..active.get_num_trials())
        .map(|index| active.get_trial(index))
        .filter(|trial| trial.is_used())
        .collect();
    if used.len() != 1 || used[0].get_size() != 10 {
        return;
    }
    let manager = data.get_arch().manage();
    let Some(storage) = manager
        .register_lookup()
        .and_then(|lookup| lookup.probe_register("ST0"))
    else {
        return;
    };
    let Some(space) = storage.space.as_ref().filter(|_| storage.size == 10) else {
        return;
    };
    let address = used[0].get_address();
    if address.get_space().map(|s| s.get_index()) != Some(space.get_index())
        || address.get_offset() != storage.offset
        || !data
            .get_func_proto()
            .model()
            .output()
            .get_entry()
            .iter()
            .any(|entry| {
                entry.get_type() == type_class::TYPECLASS_FLOAT
                    && entry.justified_contain(address, 10) == 0
            })
    {
        return;
    }
    let mut values = Vec::new();
    for id in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let Some(op) = data
            .obank()
            .get(id)
            .filter(|op| !op.is_dead() && op.get_halt_type() == 0)
        else {
            continue;
        };
        if op.num_input() != 2 {
            return;
        }
        let Some(value) = op.get_in(1).and_then(|vn| data.vbank().get(vn)) else {
            return;
        };
        let from_call = value
            .get_def()
            .and_then(|id| data.obank().get(id))
            .is_some_and(|def| {
                def.is_indirect_creation()
                    || matches!(def.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
            });
        if value.get_size() != 10
            || from_call
            || value.is_type_lock()
            || value.get_type().get_metatype() != type_metatype::TYPE_UNKNOWN
        {
            return;
        }
        values.push(op.get_in(1).unwrap());
    }
    if values.is_empty() {
        return;
    }
    let Some(types) = data.get_arch().types() else {
        return;
    };
    let Ok(datatype) = types.get_base(10, type_metatype::TYPE_FLOAT) else {
        return;
    };
    for value in values {
        data.vbank_mut()
            .get_mut(value)
            .unwrap()
            .update_type_locked(datatype.clone(), true, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::ArchContext,
        dtype::{Datatype, TypeFactoryImpl},
        fspec::{ParamEntry, ProtoModel},
    };
    use kuna_base::{
        address::Address,
        error::{KunaError, KunaResult},
        space::{
            spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, RegisterLookup, UniqueSpace,
            VarnodeStorage,
        },
    };
    use std::rc::Rc;

    struct Lookup(Rc<AddrSpace>);
    impl RegisterLookup for Lookup {
        fn get_register(&self, name: &str) -> KunaResult<VarnodeStorage> {
            if name != "ST0" {
                return Err(KunaError::lowlevel("unknown register"));
            }
            Ok(VarnodeStorage {
                space: Some(self.0.clone()),
                offset: 0x100,
                size: 10,
            })
        }
        fn get_register_name(&self, _: &Rc<AddrSpace>, _: u64, _: i32) -> String {
            String::new()
        }
        fn get_exact_register_name(&self, _: &Rc<AddrSpace>, _: u64, _: i32) -> String {
            String::new()
        }
    }

    fn sample(
        offset: u64,
        class: type_class,
        metatype: type_metatype,
        locked: bool,
    ) -> (Funcdata, ParamActive, crate::context::VarnodeId) {
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
            0,
            0,
            0,
        ));
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
        manager.insert_space(ram.clone()).unwrap();
        manager.insert_space(register.clone()).unwrap();
        manager.set_register_lookup(Rc::new(Lookup(register.clone())));
        let mut model = ProtoModel::new(&manager);
        model.build_param_list("standard").unwrap();
        model.output_mut().push_entry(
            ParamEntry::seed(
                0,
                class,
                register.clone(),
                offset,
                10,
                1,
                0,
                0,
                false,
                false,
                &[],
                &manager,
            )
            .unwrap(),
        );
        let mut arch = ArchContext::new(manager);
        let types = TypeFactoryImpl::new();
        types.set_max_basetype_size(16);
        types.set_default_alignment_map();
        arch.types = Some(Rc::new(types));
        let mut data = Funcdata::new(
            "sample",
            "sample",
            Rc::new(arch),
            Address::new(ram, 0x1000),
            0x10000000,
            0x40,
        )
        .unwrap();
        data.get_func_proto_mut().set_internal(
            Rc::new(model),
            Rc::new(Datatype::new_with_align(0, 1, type_metatype::TYPE_VOID)),
        );
        let address = Address::new(register, offset);
        let value = data.new_varnode(10, &address, None);
        data.vbank_mut().get_mut(value).unwrap().update_type_locked(
            Rc::new(Datatype::new_with_align(10, 1, metatype)),
            locked,
            true,
        );
        if locked {
            data.vbank_mut()
                .get_mut(value)
                .unwrap()
                .set_flags_pub(crate::varnode::varnode_flags::typelock);
        }
        assert_eq!(data.vbank().get(value).unwrap().is_type_lock(), locked);
        let ret = data.new_op(2, data.get_address().clone());
        data.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
        let zero = data.new_constant(4, 0);
        data.op_set_input(ret, zero, 0).unwrap();
        data.op_set_input(ret, value, 1).unwrap();
        data.obank_mut().mark_alive(ret);
        let mut active = ParamActive::new(false);
        active.register_trial(&address, 10);
        active.get_trial_mut(0).mark_used();
        (data, active, value)
    }

    #[test]
    fn finalized_native_float_result_supplies_its_type() {
        let (mut data, active, value) = sample(
            0x100,
            type_class::TYPECLASS_FLOAT,
            type_metatype::TYPE_UNKNOWN,
            false,
        );
        type_returns(&mut data, &active);
        assert_eq!(
            data.vbank().get(value).unwrap().get_type().get_metatype(),
            type_metatype::TYPE_FLOAT
        );
    }

    #[test]
    fn a_result_left_by_a_call_keeps_its_callee_type() {
        let (mut data, active, value) = sample(
            0x100,
            type_class::TYPECLASS_FLOAT,
            type_metatype::TYPE_UNKNOWN,
            false,
        );
        let call = data.new_op(1, data.get_address().clone());
        data.op_set_opcode_code(call, OpCode::CPUI_CALLIND);
        let target = data.new_constant(4, 0x2000);
        data.op_set_input(call, target, 0).unwrap();
        data.op_set_output(call, value).unwrap();
        data.obank_mut().mark_alive(call);
        type_returns(&mut data, &active);
        assert_eq!(
            data.vbank().get(value).unwrap().get_type().get_metatype(),
            type_metatype::TYPE_UNKNOWN
        );
    }

    #[test]
    fn unrelated_storage_and_declared_values_are_preserved() {
        for (offset, class, metatype, locked) in [
            (
                0x110,
                type_class::TYPECLASS_FLOAT,
                type_metatype::TYPE_UNKNOWN,
                false,
            ),
            (
                0x100,
                type_class::TYPECLASS_GENERAL,
                type_metatype::TYPE_UNKNOWN,
                false,
            ),
            (
                0x100,
                type_class::TYPECLASS_FLOAT,
                type_metatype::TYPE_INT,
                false,
            ),
            (
                0x100,
                type_class::TYPECLASS_FLOAT,
                type_metatype::TYPE_UNKNOWN,
                true,
            ),
        ] {
            let (mut data, active, value) = sample(offset, class, metatype, locked);
            type_returns(&mut data, &active);
            assert_eq!(
                data.vbank().get(value).unwrap().get_type().get_metatype(),
                metatype
            );
        }
    }
}
