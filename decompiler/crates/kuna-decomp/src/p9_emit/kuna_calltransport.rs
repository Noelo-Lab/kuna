//! Distinguish outgoing argument shuffles from homes of local values.

use std::collections::HashSet;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::Containment;
use crate::funcdata::Funcdata;

impl Funcdata {
    pub(crate) fn kuna_record_call_transport(&mut self, first: VarnodeId) {
        self.kuna_mark_call_transport(first);
        let Some(value) = self.vbank().get(first) else {
            return;
        };
        let carrier = value.get_addr().clone();
        let size = value.get_size();
        let mut pending = vec![first];
        let mut visited = HashSet::new();
        while let Some(id) = pending.pop() {
            if !visited.insert(id) || visited.len() > 64 {
                continue;
            }
            let uses: Vec<_> = self
                .vbank()
                .get(id)
                .into_iter()
                .flat_map(|v| v.descend_iter())
                .collect();
            for use_id in uses {
                let Some(operation) = self.obank().get(use_id) else {
                    continue;
                };
                let Some(output) = operation.get_out() else {
                    continue;
                };
                let Some(node) = self.vbank().get(output) else {
                    continue;
                };
                let transparent = match operation.code() {
                    OpCode::CPUI_COPY
                    | OpCode::CPUI_CAST
                    | OpCode::CPUI_INT_ZEXT
                    | OpCode::CPUI_INT_SEXT => operation.get_in(0) == Some(id),
                    OpCode::CPUI_PIECE => {
                        operation.get_in(1) == Some(id)
                            && operation
                                .get_in(0)
                                .and_then(|v| self.vbank().get(v))
                                .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
                    }
                    _ => false,
                };
                if !transparent
                    || node.is_persist()
                    || (node.get_space().get_type() != spacetype::IPTR_INTERNAL
                        && node.get_addr() != &carrier)
                {
                    continue;
                }
                if node.get_addr() == &carrier && node.get_size() > size {
                    self.kuna_mark_call_transport_projection(output, size);
                }
                pending.push(output);
            }
        }
    }

    /// A register copy is transport only when an earlier home exists and all
    /// remaining uses forward it through that register to ordinary call arguments.
    pub fn kuna_is_call_transport(&self, vn: VarnodeId) -> bool {
        if !self.get_arch().name_style_angr {
            return false;
        }
        let Some(value) = self.vbank().get(vn) else {
            return false;
        };
        if value.is_persist() || self.kuna_storage_write_has_local_use(vn, value.get_size()) {
            return false;
        }
        if self.kuna_was_call_transport(vn) {
            return value.has_no_descend()
                || self.kuna_only_argument_uses(vn, value.get_addr(), value.get_size());
        }
        if !self.kuna_is_register_storage(value.get_addr(), value.get_size()) {
            return false;
        }
        let Some(definition) = value.get_def().and_then(|id| self.obank().get(id)) else {
            return false;
        };
        if definition.code() != OpCode::CPUI_COPY {
            return false;
        }
        let Some(input) = definition.get_in(0) else {
            return false;
        };
        if self
            .vbank()
            .get(input)
            .is_none_or(|v| v.get_size() != value.get_size())
            || !self.kuna_has_prior_storage(input, value.get_addr(), definition.get_addr())
        {
            return false;
        }
        self.kuna_only_argument_uses(vn, value.get_addr(), value.get_size())
    }

    pub(crate) fn kuna_is_call_transport_projection(
        &self,
        vn: VarnodeId,
        byte_offset: int4,
        byte_size: int4,
    ) -> bool {
        if byte_offset != 0 {
            return false;
        }
        let Some(value) = self.vbank().get(vn) else {
            return false;
        };
        if value.is_persist() || self.kuna_storage_write_has_local_use(vn, byte_size) {
            return false;
        }
        if !value.is_persist()
            && self
                .kuna_call_transport_projection_size(vn)
                .is_some_and(|size| byte_size <= size && byte_size < value.get_size())
        {
            return value.has_no_descend()
                || self.kuna_only_argument_uses(vn, value.get_addr(), byte_size);
        }
        let Some(definition) = value.get_def().and_then(|id| self.obank().get(id)) else {
            return false;
        };
        if !matches!(
            definition.code(),
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT
        ) {
            return false;
        }
        let Some(input) = definition.get_in(0) else {
            return false;
        };
        self.vbank().get(input).is_some_and(|v| {
            (v.get_size() == byte_size
                && value.get_size() > byte_size
                && v.get_addr() == value.get_addr()
                && self.kuna_is_call_transport(input))
                || (v.get_size() >= byte_size
                    && value.get_size() > v.get_size()
                    && v.get_space().get_type() == spacetype::IPTR_SPACEBASE
                    && v.get_def().is_some()
                    && self.kuna_is_register_storage(value.get_addr(), value.get_size())
                    && self.kuna_only_argument_uses(vn, value.get_addr(), byte_size))
        })
    }

    fn kuna_is_register_storage(&self, address: &Address, size: int4) -> bool {
        let Some(space) = address.get_space() else {
            return false;
        };
        self.get_arch()
            .manage()
            .register_lookup()
            .is_some_and(|lookup| {
                !lookup
                    .get_register_name(space, address.get_offset(), size)
                    .is_empty()
            })
    }

    pub(crate) fn kuna_has_local_storage_use(&self, first: VarnodeId) -> bool {
        let Some(value) = self.vbank().get(first) else {
            return false;
        };
        if !self.kuna_is_register_storage(value.get_addr(), value.get_size()) {
            return false;
        }
        let carrier = value.get_addr();
        let mut pending = vec![first];
        let mut visited = HashSet::new();
        while let Some(id) = pending.pop() {
            if !visited.insert(id) || visited.len() > 64 {
                continue;
            }
            let Some(value) = self.vbank().get(id) else {
                continue;
            };
            for op_id in value.descend_iter() {
                let Some(operation) = self.obank().get(op_id) else {
                    continue;
                };
                for slot in 0..operation.num_input() {
                    if operation.get_in(slot) != Some(id) {
                        continue;
                    }
                    if matches!(operation.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                        if slot == 0 {
                            return true;
                        }
                        continue;
                    }
                    if matches!(
                        operation.code(),
                        OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL
                    ) {
                        continue;
                    }
                    let transparent = match operation.code() {
                        OpCode::CPUI_COPY
                        | OpCode::CPUI_CAST
                        | OpCode::CPUI_INT_ZEXT
                        | OpCode::CPUI_INT_SEXT => slot == 0,
                        OpCode::CPUI_PIECE => {
                            slot == 1
                                && operation
                                    .get_in(0)
                                    .and_then(|v| self.vbank().get(v))
                                    .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
                        }
                        OpCode::CPUI_SUBPIECE => {
                            slot == 0
                                && operation
                                    .get_in(1)
                                    .and_then(|v| self.vbank().get(v))
                                    .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
                        }
                        _ => false,
                    };
                    if !transparent {
                        return true;
                    }
                    if let Some(output) = operation
                        .get_out()
                        .and_then(|v| self.vbank().get(v).map(|node| (v, node)))
                    {
                        if output.1.is_persist()
                            || (output.1.get_space().get_type() != spacetype::IPTR_INTERNAL
                                && output.1.get_addr() != carrier)
                        {
                            return true;
                        }
                        pending.push(output.0);
                    }
                }
            }
        }
        false
    }

    fn kuna_has_prior_storage(
        &self,
        first: VarnodeId,
        carrier: &Address,
        instruction: &Address,
    ) -> bool {
        if self.kuna_materializes_computation(first, instruction) {
            return false;
        }
        let mut pending = vec![first];
        let mut visited = HashSet::new();
        while let Some(id) = pending.pop() {
            if !visited.insert(id) {
                continue;
            }
            if visited.len() > 64 {
                return false;
            }
            let Some(value) = self.vbank().get(id) else {
                continue;
            };
            if value.is_constant() || value.is_annotation() {
                continue;
            }
            if value.get_space().get_type() == spacetype::IPTR_INTERNAL
                && value
                    .get_def()
                    .and_then(|op| self.obank().get(op))
                    .is_some_and(|op| {
                        op.code() != OpCode::CPUI_COPY && op.get_addr() == instruction
                    })
            {
                continue;
            }
            let address = value.get_addr();
            if address != carrier
                && !self.kuna_was_call_transport(id)
                && (self.kuna_is_register_storage(address, value.get_size())
                    || value.get_space().get_type() == spacetype::IPTR_SPACEBASE)
            {
                return true;
            }
            if self.kuna_storage_sources(id).iter().any(|source| {
                source.address != *carrier
                    && source.size == value.get_size()
                    && source.write_address.as_ref() != Some(instruction)
            }) {
                return true;
            }
            let Some(definition) = value.get_def().and_then(|op| self.obank().get(op)) else {
                continue;
            };
            if definition.code() == OpCode::CPUI_COPY {
                if let Some(input) = definition.get_in(0) {
                    if self
                        .vbank()
                        .get(input)
                        .is_some_and(|v| v.get_size() == value.get_size())
                    {
                        pending.push(input);
                    }
                }
            }
        }
        false
    }

    fn kuna_materializes_computation(&self, first: VarnodeId, instruction: &Address) -> bool {
        let mut current = first;
        let mut visited = HashSet::new();
        while visited.insert(current) && visited.len() <= 64 {
            let Some(value) = self.vbank().get(current) else {
                return false;
            };
            if value.get_space().get_type() != spacetype::IPTR_INTERNAL {
                return false;
            }
            let Some(operation) = value.get_def().and_then(|op| self.obank().get(op)) else {
                return false;
            };
            if operation.get_addr() != instruction {
                return false;
            }
            if matches!(
                operation.code(),
                OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT
            ) {
                return false;
            }
            if !matches!(operation.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
                return true;
            }
            let Some(input) = operation.get_in(0) else {
                return false;
            };
            if self
                .vbank()
                .get(input)
                .is_none_or(|v| v.get_size() != value.get_size())
            {
                return true;
            }
            current = input;
        }
        true
    }

    fn kuna_only_argument_uses(&self, first: VarnodeId, carrier: &Address, size: int4) -> bool {
        let mut pending = vec![first];
        let mut visited = HashSet::new();
        let mut saw_argument = false;
        while let Some(id) = pending.pop() {
            if !visited.insert(id) {
                continue;
            }
            if visited.len() > 64 {
                return false;
            }
            let Some(value) = self.vbank().get(id) else {
                return false;
            };
            if value.has_no_descend() {
                return false;
            }
            for use_id in value.descend_iter() {
                let Some(operation) = self.obank().get(use_id) else {
                    return false;
                };
                for slot in 0..operation.num_input() {
                    if operation.get_in(slot) != Some(id) {
                        continue;
                    }
                    if matches!(operation.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                        if slot == 0
                            || !self.kuna_argument_storage_matches(use_id, slot, carrier, size)
                        {
                            return false;
                        }
                        saw_argument = true;
                        continue;
                    }
                    let transparent = match operation.code() {
                        OpCode::CPUI_COPY | OpCode::CPUI_CAST => slot == 0,
                        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => slot == 0,
                        OpCode::CPUI_PIECE => {
                            slot == 1
                                && operation
                                    .get_in(0)
                                    .and_then(|v| self.vbank().get(v))
                                    .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
                        }
                        OpCode::CPUI_SUBPIECE => {
                            slot == 0
                                && operation
                                    .get_in(1)
                                    .and_then(|v| self.vbank().get(v))
                                    .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
                        }
                        _ => false,
                    };
                    let Some(output) = operation
                        .get_out()
                        .and_then(|v| self.vbank().get(v).map(|node| (v, node)))
                    else {
                        return false;
                    };
                    if !transparent
                        || output.1.is_persist()
                        || output.1.get_space().get_type() == spacetype::IPTR_SPACEBASE
                        || (output.1.get_space().get_type() != spacetype::IPTR_INTERNAL
                            && output.1.get_addr() != carrier)
                    {
                        return false;
                    }
                    pending.push(output.0);
                }
            }
        }
        saw_argument
    }

    fn kuna_argument_storage_matches(
        &self,
        op: OpId,
        slot: int4,
        address: &Address,
        size: int4,
    ) -> bool {
        let Some(index) = self.get_call_specs_index(op) else {
            return false;
        };
        let call = self.get_call_specs(index);
        let parameter = (slot - 1) as usize;
        if call.proto().is_input_locked() {
            return call.proto().get_param(slot - 1).is_some_and(|param| {
                let home = param.get_address();
                home.justified_contain(param.get_size(), address, size, false) == 0
                    || address.justified_contain(size, &home, param.get_size(), false) == 0
            });
        }
        if !call.final_input_storage().is_empty()
            && self.obank().get(op).is_some_and(|operation| {
                operation.num_input() as usize == call.final_input_storage().len() + 1
            })
        {
            return call
                .final_input_storage()
                .get(parameter)
                .is_some_and(|(home, width)| {
                    home.justified_contain(*width, address, size, false) == 0
                        || address.justified_contain(size, home, *width, false) == 0
                });
        }
        if let Some(param) = call.proto().get_param(slot - 1) {
            let home = param.get_address();
            return home.justified_contain(param.get_size(), address, size, false) == 0
                || address.justified_contain(size, &home, param.get_size(), false) == 0;
        }
        call.proto().has_model()
            && matches!(
                call.proto().characterize_as_input_param(address, size),
                Containment::ContainsJustified | Containment::ContainedBy
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::error::{KunaError, KunaResult};
    use kuna_base::space::{
        addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
        RegisterLookup, UniqueSpace, VarnodeStorage,
    };

    use crate::action::Rule;
    use crate::context::ArchContext;
    use crate::fspec::FuncCallSpecs;
    use crate::ruleaction_3::RulePropagateCopy;

    struct Registers;

    impl RegisterLookup for Registers {
        fn get_register(&self, _: &str) -> KunaResult<VarnodeStorage> {
            Err(KunaError::lowlevel("test registers resolve by storage"))
        }

        fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: int4) -> String {
            if base.get_name() == "register" && matches!(size, 1 | 2 | 4 | 8) {
                format!("r{off}_{size}")
            } else {
                String::new()
            }
        }

        fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: int4) -> String {
            self.get_register_name(base, off, size)
        }
    }

    fn function() -> Funcdata {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        manager.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        manager.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
        for (name, index) in [("ram", 4), ("register", 5)] {
            manager
                .insert_space(Rc::new(AddrSpace::new(
                    spacetype::IPTR_PROCESSOR,
                    name,
                    false,
                    8,
                    1,
                    index,
                    addrspace_flags::hasphysical,
                    1,
                    1,
                )))
                .unwrap();
        }
        manager.set_register_lookup(Rc::new(Registers));
        let architecture = Rc::new(ArchContext::new(manager));
        let memory = Rc::clone(architecture.manage().get_space_by_name("ram").unwrap());
        Funcdata::new(
            "func",
            "func",
            architecture,
            Address::new(memory, 0x1000),
            0x10000000,
            0x40,
        )
        .unwrap()
    }

    fn register(fd: &Funcdata, offset: u64) -> Address {
        Address::new(
            Rc::clone(
                fd.get_arch()
                    .manage()
                    .get_space_by_name("register")
                    .unwrap(),
            ),
            offset,
        )
    }

    fn operation(fd: &mut Funcdata, opcode: OpCode, inputs: &[VarnodeId]) -> OpId {
        let address = fd.get_address() + (fd.obank().iter_all().count() * 4) as i64;
        let id = fd.new_op(inputs.len() as int4, address);
        fd.op_set_opcode_code(id, opcode);
        for (slot, input) in inputs.iter().enumerate() {
            fd.op_set_input(id, *input, slot as int4).unwrap();
        }
        id
    }

    fn input(fd: &mut Funcdata, offset: u64, size: int4) -> VarnodeId {
        let address = register(fd, offset);
        let id = fd.new_varnode(size, &address, None);
        fd.set_input_varnode(id).unwrap()
    }

    fn copy(fd: &mut Funcdata, source: VarnodeId, offset: u64) -> VarnodeId {
        let size = fd.vbank().get(source).unwrap().get_size();
        let id = operation(fd, OpCode::CPUI_COPY, &[source]);
        let address = register(fd, offset);
        fd.new_varnode_out(size, &address, id).unwrap()
    }

    fn call(fd: &mut Funcdata, opcode: OpCode, argument: VarnodeId, abi_offset: u64) -> OpId {
        let target = fd.new_constant(8, 0x2000);
        let id = operation(fd, opcode, &[target, argument]);
        let mut specs = FuncCallSpecs::new(id, fd.get_address().clone());
        specs.set_final_input_storage(vec![(
            register(fd, abi_offset),
            fd.vbank().get(argument).unwrap().get_size(),
        )]);
        fd.push_call_specs(specs);
        id
    }

    #[test]
    fn outgoing_copy_is_excluded_before_propagation_and_after_teardown() {
        for opcode in [OpCode::CPUI_CALL, OpCode::CPUI_CALLIND] {
            let mut fd = function();
            let source = input(&mut fd, 0x80, 4);
            let carrier = copy(&mut fd, source, 0x10);
            let call = call(&mut fd, opcode, carrier, 0x10);
            assert!(fd.kuna_is_call_transport(carrier));
            assert_eq!(RulePropagateCopy.apply_op(call, &mut fd), 1);
            assert_eq!(fd.obank().get(call).unwrap().get_in(1), Some(source));
            assert!(fd.kuna_storage_sources(source).is_empty());
            assert!(fd.kuna_is_call_transport(carrier));
            let cloned = fd.clone_varnode(carrier);
            assert!(fd.kuna_is_call_transport(cloned));
            fd.delete_varnode(cloned).unwrap();
            assert!(!fd.kuna_was_call_transport(cloned));
            fd.clear();
            assert!(!fd.kuna_was_call_transport(carrier));
        }
    }

    #[test]
    fn argument_slot_must_match_abi_and_call_target_remains_a_use() {
        let mut fd = function();
        let source = input(&mut fd, 0x80, 8);
        let durable = copy(&mut fd, source, 0x90);
        call(&mut fd, OpCode::CPUI_CALL, durable, 0x10);
        assert!(!fd.kuna_is_call_transport(durable));

        let target = copy(&mut fd, source, 0x10);
        let indirect = call(&mut fd, OpCode::CPUI_CALLIND, target, 0x10);
        fd.op_set_input(indirect, target, 0).unwrap();
        assert!(!fd.kuna_is_call_transport(target));
    }

    #[test]
    fn real_definitions_return_values_and_local_uses_remain_homes() {
        for use_opcode in [
            OpCode::CPUI_RETURN,
            OpCode::CPUI_CALLOTHER,
            OpCode::CPUI_INT_ADD,
        ] {
            let mut fd = function();
            let source = input(&mut fd, 0x80, 4);
            let carrier = copy(&mut fd, source, 0x10);
            call(&mut fd, OpCode::CPUI_CALL, carrier, 0x10);
            let other = fd.new_constant(4, 1);
            operation(&mut fd, use_opcode, &[carrier, other]);
            assert!(!fd.kuna_is_call_transport(carrier));
            fd.kuna_inherit_storage(source, carrier);
            assert_eq!(
                fd.kuna_storage_sources(source)[0].address,
                register(&fd, 0x10)
            );
        }

        let mut fd = function();
        let pointer = input(&mut fd, 0x80, 8);
        let memory = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let space = fd.new_varnode_space(&memory);
        let load = operation(&mut fd, OpCode::CPUI_LOAD, &[space, pointer]);
        let loaded = fd.new_unique_out(4, load).unwrap();
        let first_home = copy(&mut fd, loaded, 0x10);
        call(&mut fd, OpCode::CPUI_CALL, first_home, 0x10);
        assert!(!fd.kuna_is_call_transport(first_home));
        fd.kuna_inherit_storage(loaded, first_home);
        assert_eq!(
            fd.kuna_storage_sources(loaded)[0].address,
            register(&fd, 0x10)
        );
    }

    #[test]
    fn extension_teardown_does_not_restore_outgoing_alias_but_keeps_computed_homes() {
        let mut fd = function();
        let source = input(&mut fd, 0x80, 4);
        let carrier = copy(&mut fd, source, 0x10);
        let extension = operation(&mut fd, OpCode::CPUI_INT_ZEXT, &[carrier]);
        let address = register(&fd, 0x10);
        let wide = fd.new_varnode_out(8, &address, extension).unwrap();
        call(&mut fd, OpCode::CPUI_CALL, wide, 0x10);
        assert!(fd.kuna_is_call_transport(carrier));
        assert!(!fd.kuna_is_call_transport(wide));
        fd.op_unset_output(extension);
        assert!(fd.kuna_storage_sources(carrier).is_empty());
        fd.kuna_inherit_storage(source, carrier);
        assert!(fd.kuna_storage_sources(source).is_empty());

        let mut fd = function();
        let source = input(&mut fd, 0x80, 1);
        let extension = operation(&mut fd, OpCode::CPUI_INT_ZEXT, &[source]);
        let address = register(&fd, 0x10);
        let computed = fd.new_varnode_out(4, &address, extension).unwrap();
        call(&mut fd, OpCode::CPUI_CALL, computed, 0x10);
        fd.op_unset_output(extension);
        assert_eq!(fd.kuna_storage_sources(source)[0].address, address);
    }

    #[test]
    fn already_approved_homes_survive_transport_and_unknown_uses_are_kept() {
        let mut fd = function();
        let source = input(&mut fd, 0x80, 4);
        let genuine = input(&mut fd, 0x90, 4);
        let carrier = copy(&mut fd, source, 0x10);
        fd.kuna_inherit_storage(carrier, genuine);
        call(&mut fd, OpCode::CPUI_CALL, carrier, 0x10);
        fd.total_replace(carrier, source).unwrap();
        assert_eq!(fd.kuna_storage_sources(source).len(), 1);
        assert_eq!(
            fd.kuna_storage_sources(source)[0].address,
            register(&fd, 0x90)
        );

        let no_uses = copy(&mut fd, source, 0x20);
        assert!(!fd.kuna_is_call_transport(no_uses));
        let persistent = copy(&mut fd, source, 0x10);
        call(&mut fd, OpCode::CPUI_CALL, persistent, 0x10);
        fd.vbank_mut()
            .get_mut(persistent)
            .unwrap()
            .set_flags_pub(crate::varnode::varnode_flags::persist);
        assert!(!fd.kuna_is_call_transport(persistent));
        let durable = copy(&mut fd, source, 0x30);
        let outgoing = copy(&mut fd, durable, 0x10);
        call(&mut fd, OpCode::CPUI_CALL, outgoing, 0x10);
        assert!(!fd.kuna_is_call_transport(durable));
    }

    #[test]
    fn another_definition_at_the_same_register_does_not_inherit_transport_role() {
        let mut fd = function();
        let constant = fd.new_constant(4, 7);
        let computation = operation(&mut fd, OpCode::CPUI_INT_ADD, &[constant, constant]);
        let address = register(&fd, 0x10);
        let genuine = fd.new_varnode_out(4, &address, computation).unwrap();
        let saved = copy(&mut fd, genuine, 0x80);
        let outgoing = copy(&mut fd, saved, 0x10);
        call(&mut fd, OpCode::CPUI_CALL, outgoing, 0x10);
        assert!(fd.kuna_is_call_transport(outgoing));
        fd.total_replace(outgoing, genuine).unwrap();
        assert!(!fd.kuna_is_call_transport(genuine));
        assert!(!fd.kuna_was_call_transport(genuine));

        let second = copy(&mut fd, saved, 0x10);
        call(&mut fd, OpCode::CPUI_CALL, second, 0x10);
        fd.kuna_inherit_storage(saved, second);
        assert!(fd.kuna_was_call_transport(second));
        let definition = fd.vbank().get(second).unwrap().get_def().unwrap();
        fd.op_set_opcode_code(definition, OpCode::CPUI_INT_ADD);
        assert!(!fd.kuna_was_call_transport(second));

        let third = copy(&mut fd, saved, 0x10);
        call(&mut fd, OpCode::CPUI_CALL, third, 0x10);
        fd.kuna_inherit_storage(saved, third);
        assert!(fd.kuna_was_call_transport(third));
        let constant = fd.new_constant(4, 9);
        let compute = operation(&mut fd, OpCode::CPUI_INT_ADD, &[constant, constant]);
        let unique = fd.new_unique_out(4, compute).unwrap();
        let materialize = operation(&mut fd, OpCode::CPUI_COPY, &[unique]);
        fd.op_set_output(materialize, third).unwrap();
        assert!(!fd.kuna_was_call_transport(third));
        assert!(!fd.kuna_is_call_transport(third));
    }

    #[test]
    fn late_proof_retracts_only_its_native_write_and_preserves_removed_local_reads() {
        let mut fd = function();
        let source = input(&mut fd, 0x80, 4);
        let zero = fd.new_constant(4, 0);
        let computation = operation(&mut fd, OpCode::CPUI_INT_ADD, &[source, zero]);
        let address = register(&fd, 0x10);
        let genuine = fd.new_varnode_out(4, &address, computation).unwrap();
        let genuine_write = fd.obank().get(computation).unwrap().get_addr().clone();
        fd.kuna_inherit_storage(source, genuine);

        let outgoing = copy(&mut fd, source, 0x10);
        fd.kuna_inherit_storage(source, outgoing);
        assert_eq!(fd.kuna_storage_sources(source).len(), 2);
        call(&mut fd, OpCode::CPUI_CALL, outgoing, 0x10);
        fd.total_replace(outgoing, source).unwrap();
        assert_eq!(fd.kuna_storage_sources(source).len(), 1);
        assert_eq!(
            fd.kuna_storage_sources(source)[0].write_address.as_ref(),
            Some(&genuine_write)
        );

        let meaningful = copy(&mut fd, source, 0x10);
        let memory = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let space = fd.new_varnode_space(&memory);
        let pointer = input(&mut fd, 0x90, 8);
        let store = operation(&mut fd, OpCode::CPUI_STORE, &[space, pointer, meaningful]);
        call(&mut fd, OpCode::CPUI_CALL, meaningful, 0x10);
        assert!(!fd.kuna_is_call_transport(meaningful));
        assert_eq!(RulePropagateCopy.apply_op(store, &mut fd), 1);
        assert!(!fd.kuna_is_call_transport(meaningful));
        let write = fd.vbank().get(meaningful).unwrap().get_def().unwrap();
        let instruction = fd.obank().get(write).unwrap().get_addr();
        assert!(fd
            .kuna_storage_sources(source)
            .iter()
            .any(|home| home.write_address.as_ref() == Some(instruction)));
    }

    #[test]
    fn zero_upper_pieces_project_transport_without_losing_computed_first_homes() {
        let mut fd = function();
        let source = input(&mut fd, 0x80, 4);
        let carrier = copy(&mut fd, source, 0x10);
        let zero = fd.new_constant(4, 0);
        let piece = operation(&mut fd, OpCode::CPUI_PIECE, &[zero, carrier]);
        let address = register(&fd, 0x10);
        let wide = fd.new_varnode_out(8, &address, piece).unwrap();
        call(&mut fd, OpCode::CPUI_CALL, wide, 0x10);
        assert!(fd.kuna_is_call_transport(carrier));
        assert_eq!(RulePropagateCopy.apply_op(piece, &mut fd), 1);
        assert!(fd.kuna_is_call_transport_projection(wide, 0, 4));
        assert!(!fd.kuna_is_call_transport(wide));
        assert!(!fd.kuna_is_call_transport_projection(wide, 0, 8));
        let cloned = fd.clone_varnode(wide);
        assert!(fd.kuna_is_call_transport_projection(cloned, 0, 4));
        fd.delete_varnode(cloned).unwrap();
        assert!(fd.kuna_call_transport_projection_size(cloned).is_none());

        for opcode in [OpCode::CPUI_LOAD, OpCode::CPUI_INT_ADD] {
            for wrapped in [false, true] {
                let mut fd = function();
                let source = input(&mut fd, 0x80, 4);
                let other = fd.new_constant(4, 1);
                let computed = operation(&mut fd, opcode, &[source, other]);
                let value = fd.new_unique_out(4, computed).unwrap();
                let instruction = fd.obank().get(computed).unwrap().get_addr().clone();
                let value = if wrapped {
                    let wrapper = fd.new_op(1, instruction.clone());
                    fd.op_set_opcode_code(wrapper, OpCode::CPUI_COPY);
                    fd.op_set_input(wrapper, value, 0).unwrap();
                    fd.new_unique_out(4, wrapper).unwrap()
                } else {
                    value
                };
                let materialize = fd.new_op(1, instruction);
                fd.op_set_opcode_code(materialize, OpCode::CPUI_COPY);
                fd.op_set_input(materialize, value, 0).unwrap();
                let address = register(&fd, 0x10);
                let first_home = fd.new_varnode_out(4, &address, materialize).unwrap();
                call(&mut fd, OpCode::CPUI_CALL, first_home, 0x10);
                let later_home = copy(&mut fd, first_home, 0x90);
                fd.kuna_inherit_storage(value, later_home);
                assert!(!fd.kuna_is_call_transport(first_home));
                fd.kuna_inherit_storage(value, first_home);
                assert!(fd
                    .kuna_storage_sources(value)
                    .iter()
                    .any(|home| home.address == address));
            }
        }
    }
}
