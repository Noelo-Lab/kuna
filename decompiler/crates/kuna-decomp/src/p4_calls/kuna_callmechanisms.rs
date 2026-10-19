//! (kuna) `callmechanisms` — compiler-declared p-code attached to prototype entry and return boundaries.
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{IdRegistry, XmlDecode};
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use crate::inject_sleigh::PcodeInjectLibrarySleigh;
use crate::pcodeinject::CALLMECHANISM_TYPE;

/// Register before snippet compilation; decoding models later only resolves IDs.
pub fn register(
    xml: Option<&str>,
    manager: &AddrSpaceManager,
    library: &mut PcodeInjectLibrarySleigh,
) -> KunaResult<()> {
    let Some(xml) = xml else { return Ok(()) };
    let mut documents = DocumentStorage::new();
    let root = documents.parse_document(xml.as_bytes())?.get_root().clone();
    let mut registry = IdRegistry::with_base_ids();
    crate::pcodeinject::register_ids(&mut registry);
    fn visit(
        element: &Rc<Element>,
        manager: &AddrSpaceManager,
        registry: &IdRegistry,
        library: &mut PcodeInjectLibrarySleigh,
    ) -> KunaResult<()> {
        if element.get_name() == "prototype" {
            let name = element.get_attribute_value("name").unwrap_or(b"__stdcall");
            for child in element
                .get_children()
                .iter()
                .filter(|e| e.get_name() == "pcode")
            {
                let kind = child.get_attribute_value("inject").unwrap_or(b"");
                if !matches!(kind, b"uponentry" | b"uponreturn") {
                    return Err(KunaError::lowlevel(
                        "Prototype p-code requires uponentry or uponreturn",
                    ));
                }
                let mut decoder = XmlDecode::new_with_root(manager, registry, child, 0);
                library.decode_inject(
                    b"Compiler prototype",
                    name,
                    CALLMECHANISM_TYPE,
                    &mut decoder,
                )?;
            }
        }
        for child in element.get_children() {
            visit(child, manager, registry, library)?;
        }
        Ok(())
    }
    visit(&root, manager, &registry, library)
}

pub fn ids(library: &PcodeInjectLibrarySleigh, name: &str) -> (i32, i32) {
    let lookup = |kind| {
        library.base.get_payload_id(
            CALLMECHANISM_TYPE,
            format!("{name}@@inject_{kind}").as_bytes(),
        )
    };
    (lookup("uponentry"), lookup("uponreturn"))
}

/// A payload with no locally compiled body (dynamic, or host-owned in ghidra mode) stays unresolved.
pub fn compiled_ids(library: &PcodeInjectLibrarySleigh, name: &str) -> (i32, i32) {
    let compiled = |id: i32| {
        if id >= 0 && library.get_tpl(id).is_some() {
            id
        } else {
            -1
        }
    };
    let (entry, upon_return) = ids(library, name);
    (compiled(entry), compiled(upon_return))
}

#[derive(Debug, Default)]
pub struct State {
    applied: std::collections::HashMap<crate::context::OpId, i32>,
    entry: Option<i32>,
    pub failure: Option<String>,
}

/// Changing a call's return mechanism requires rebuilding its pre-SSA effects.
pub fn requires_reflow(
    current: &crate::fspec::FuncProto,
    replacement: &crate::fspec::FuncProto,
) -> bool {
    let id = |proto: &crate::fspec::FuncProto| {
        if proto.has_model() {
            proto.get_inject_upon_return()
        } else {
            -1
        }
    };
    id(current) != id(replacement)
}

pub fn evaluation_model(data: &crate::funcdata::Funcdata) -> Option<Rc<crate::fspec::ProtoModel>> {
    if data.get_func_proto().is_model_locked() {
        return data
            .get_func_proto()
            .has_model()
            .then(|| data.get_func_proto().model().clone());
    }
    data.get_arch().eval_fp_current().cloned().or_else(|| {
        data.get_func_proto()
            .has_model()
            .then(|| data.get_func_proto().model().clone())
    })
}

/// Apply the callee model's declared return-boundary effects once per lifted call.
pub fn apply_return(data: &mut crate::funcdata::Funcdata, index: i32) -> KunaResult<()> {
    if !data.get_arch().call_mechanism_effects {
        return Ok(());
    }
    let call = data.get_call_specs(index);
    if !call.proto().has_model() || call.proto().is_no_return() {
        return Ok(());
    }
    let op = call.get_op();
    let id = call.proto().get_inject_upon_return();
    if let Some(previous) = data.kuna_call_mechanisms.applied.get(&op) {
        return if *previous == id {
            Ok(())
        } else {
            Err(KunaError::lowlevel(
                "Call mechanism changed after insertion; function requires reanalysis",
            ))
        };
    }
    if id < 0 {
        return Ok(());
    }
    let address = data
        .obank()
        .get(op)
        .ok_or_else(|| KunaError::lowlevel("Missing call operation for compiler mechanism"))?
        .get_addr()
        .clone();
    let mut context = crate::pcodeinject::InjectContext {
        baseaddr: Some(address.clone()),
        nextaddr: Some(address),
        ..Default::default()
    };
    let arch = data.get_arch().clone();
    let operations = arch
        .call_mechanisms
        .operations(id, arch.manage(), &mut context)?;
    install_after(data, op, &operations)?;
    data.kuna_call_mechanisms.applied.insert(op, id);
    Ok(())
}

/// Apply the current model's entry-boundary effects before the first operation.
pub fn apply_entry(data: &mut crate::funcdata::Funcdata) -> KunaResult<()> {
    if !data.get_arch().call_mechanism_effects || data.bblocks_get_size() == 0 {
        return Ok(());
    }
    let Some(model) = evaluation_model(data) else {
        return Ok(());
    };
    let id = model.get_inject_upon_entry();
    if let Some(previous) = data.kuna_call_mechanisms.entry {
        return if previous == id {
            Ok(())
        } else {
            Err(KunaError::lowlevel(
                "Entry mechanism changed after insertion; function requires reanalysis",
            ))
        };
    }
    if id < 0 {
        return Ok(());
    }
    let block = data.bblocks_get_block(0);
    let address = data.bblocks_block_start(block);
    let mut context = crate::pcodeinject::InjectContext {
        baseaddr: Some(address.clone()),
        nextaddr: Some(address),
        ..Default::default()
    };
    let arch = data.get_arch().clone();
    let operations = arch
        .call_mechanisms
        .operations(id, arch.manage(), &mut context)?;
    install(data, None, Some(block), &operations)?;
    data.kuna_call_mechanisms.entry = Some(id);
    Ok(())
}

/// One emitted operation before it is installed into function IR.
#[derive(Debug, Clone)]
pub struct Operation {
    pub address: kuna_base::address::Address,
    pub opcode: kuna_num::opcodes::OpCode,
    pub output: Option<kuna_num::pcoderaw::VarnodeData>,
    pub inputs: Vec<kuna_num::pcoderaw::VarnodeData>,
}

#[derive(Default)]
struct Operations(Vec<Operation>);

impl kuna_sleigh::translate::PcodeEmit for Operations {
    fn dump(
        &mut self,
        address: &kuna_base::address::Address,
        opcode: kuna_num::opcodes::OpCode,
        output: Option<&kuna_num::pcoderaw::VarnodeData>,
        inputs: &[kuna_num::pcoderaw::VarnodeData],
    ) {
        self.0.push(Operation {
            address: address.clone(),
            opcode,
            output: output.cloned(),
            inputs: inputs.to_vec(),
        });
    }
}

fn validate_operations(operations: &[Operation]) -> KunaResult<()> {
    use kuna_num::opcodes::OpCode::*;
    for op in operations {
        if matches!(
            op.opcode,
            CPUI_BRANCH
                | CPUI_CBRANCH
                | CPUI_BRANCHIND
                | CPUI_CALL
                | CPUI_CALLIND
                | CPUI_RETURN
                | CPUI_MULTIEQUAL
                | CPUI_INDIRECT
                | CPUI_MAX
        ) {
            return Err(KunaError::lowlevel(
                "Compiler prototype mechanism requires unsupported control flow",
            ));
        }
        if op.output.as_ref().is_some_and(|storage| {
            storage
                .space
                .as_ref()
                .is_some_and(|space| space.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
        }) {
            return Err(KunaError::lowlevel(
                "Compiler prototype mechanism writes constant storage",
            ));
        }
        for storage in op.inputs.iter().chain(op.output.iter()) {
            if storage.space.is_none()
                || storage.size == 0
                || storage.size > i32::MAX as u32
                || (storage.space.as_ref().is_some_and(|space| {
                    space.get_type() != kuna_base::space::spacetype::IPTR_CONSTANT
                }) && storage.offset.checked_add(storage.size as u64).is_none())
            {
                return Err(KunaError::lowlevel(
                    "Invalid compiler prototype mechanism storage",
                ));
            }
        }
    }
    Ok(())
}

/// Install straight-line p-code with a fresh temporary range for this invocation.
pub fn install_after(
    data: &mut crate::funcdata::Funcdata,
    previous: crate::context::OpId,
    operations: &[Operation],
) -> KunaResult<()> {
    install(data, Some(previous), None, operations)
}

fn install(
    data: &mut crate::funcdata::Funcdata,
    mut previous: Option<crate::context::OpId>,
    entry: Option<crate::context::BlockId>,
    operations: &[Operation],
) -> KunaResult<()> {
    use kuna_base::address::Address;
    use kuna_base::space::spacetype;
    validate_operations(operations)?;
    let mut temporary_range: Option<(u64, u64)> = None;
    for storage in operations
        .iter()
        .flat_map(|op| op.inputs.iter().chain(op.output.iter()))
    {
        if storage.space.as_ref().unwrap().get_type() == spacetype::IPTR_INTERNAL {
            let end = storage.offset + storage.size as u64;
            temporary_range = Some(match temporary_range {
                Some((lo, hi)) => (lo.min(storage.offset), hi.max(end)),
                None => (storage.offset, end),
            });
        }
    }
    let relocation = if let Some((lo, hi)) = temporary_range {
        let size = i32::try_from(hi - lo)
            .map_err(|_| KunaError::lowlevel("Compiler mechanism temporary range is too large"))?;
        let anchor = data.new_unique(size, None);
        let base = data.vbank().get(anchor).unwrap().get_addr().get_offset();
        data.delete_varnode(anchor)?;
        Some((lo, base))
    } else {
        None
    };
    let address = |storage: &kuna_num::pcoderaw::VarnodeData| {
        let space = storage.space.as_ref().unwrap().clone();
        let offset = if space.get_type() == spacetype::IPTR_INTERNAL {
            let (lo, base) = relocation.unwrap();
            base + (storage.offset - lo)
        } else {
            storage.offset
        };
        Address::new(space, offset)
    };
    for operation in operations {
        let op = data.new_op(operation.inputs.len() as i32, operation.address.clone());
        data.op_set_opcode_code(op, operation.opcode);
        if let Some(output) = operation.output.as_ref() {
            data.new_varnode_out(output.size as i32, &address(output), op)?;
        }
        for (slot, input) in operation.inputs.iter().enumerate() {
            let location = address(input);
            let vn = data.new_varnode_space_off(
                input.size as i32,
                location.get_space().unwrap().clone(),
                location.get_offset(),
            );
            data.op_set_input(op, vn, slot as i32)?;
        }
        if let Some(previous) = previous {
            data.op_insert_after(op, previous);
        } else {
            data.op_insert_begin(op, entry.expect("mechanism entry block"));
        }
        previous = Some(op);
    }
    Ok(())
}

fn validate_template(template: &kuna_sleigh::semantics::ConstructTpl) -> KunaResult<()> {
    use kuna_sleigh::semantics::ConstType;
    for op in template.get_opvec() {
        for storage in (0..op.num_input())
            .map(|index| op.get_in(index))
            .chain(op.get_out())
        {
            for value in [
                storage.get_space(),
                storage.get_offset(),
                storage.get_size(),
            ] {
                if matches!(
                    value.get_type(),
                    ConstType::JStart
                        | ConstType::JNext
                        | ConstType::JNext2
                        | ConstType::JRelative
                        | ConstType::JFlowref
                        | ConstType::JFlowrefSize
                        | ConstType::JFlowdest
                        | ConstType::JFlowdestSize
                ) {
                    return Err(KunaError::lowlevel(
                        "Compiler mechanism uses unsupported instruction context",
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Immutable compiled mechanisms shared by every function in an architecture.
#[derive(Debug, Default)]
pub struct Mechanisms {
    payloads: std::collections::BTreeMap<
        i32,
        (
            crate::inject_sleigh::SleighPayload,
            Option<kuna_sleigh::semantics::ConstructTpl>,
        ),
    >,
}

impl Mechanisms {
    pub fn from_library(library: &PcodeInjectLibrarySleigh) -> Self {
        let payloads = library
            .injection
            .iter()
            .enumerate()
            .filter(|(_, payload)| payload.as_payload().core().get_type() == CALLMECHANISM_TYPE)
            .map(|(id, payload)| {
                (
                    id as i32,
                    (payload.clone(), library.get_tpl(id as i32).cloned()),
                )
            })
            .collect();
        Self { payloads }
    }

    /// Collect and validate the complete payload before changing function IR.
    pub fn operations(
        &self,
        id: i32,
        manager: &AddrSpaceManager,
        context: &mut crate::pcodeinject::InjectContext,
    ) -> KunaResult<Vec<Operation>> {
        let mut output = Operations::default();
        self.emit(id, manager, context, &mut output)?;
        validate_operations(&output.0)?;
        Ok(output.0)
    }

    pub fn emit(
        &self,
        id: i32,
        manager: &AddrSpaceManager,
        context: &mut crate::pcodeinject::InjectContext,
        output: &mut dyn kuna_sleigh::translate::PcodeEmit,
    ) -> KunaResult<()> {
        let (payload, template) = self
            .payloads
            .get(&id)
            .ok_or_else(|| KunaError::lowlevel("Missing compiler prototype mechanism"))?;
        let template = template.as_ref().ok_or_else(|| {
            KunaError::lowlevel("Compiler prototype mechanism has no compiled p-code")
        })?;
        validate_template(template)?;
        let space = |value: Option<&Rc<kuna_base::space::AddrSpace>>| {
            value
                .cloned()
                .ok_or_else(|| KunaError::lowlevel("Missing mechanism address space"))
        };
        let engine = crate::inject_sleigh::SleighInjectEngine::new(
            space(manager.get_constant_space())?,
            space(manager.get_unique_space())?,
            space(manager.get_default_code_space())?,
        );
        engine.emit_payload(payload.as_payload(), template, context, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unasserted_function_uses_the_effective_default_entry_model() {
        use kuna_base::address::Address;
        use kuna_base::space::{spacetype, AddrSpace, ConstantSpace, UniqueSpace};
        use kuna_num::opcodes::OpCode::CPUI_COPY;
        use kuna_sleigh::semantics::{ConstTpl, ConstType, ConstructTpl, OpTpl, VarnodeTpl};
        let mut manager = AddrSpaceManager::new();
        let constant = Rc::new(ConstantSpace::new());
        manager.insert_space(constant.clone()).unwrap();
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
            0,
            1,
            1,
        ));
        manager.insert_space(ram.clone()).unwrap();
        manager.set_default_code_space(2).unwrap();
        let real = |value| ConstTpl::new_real(ConstType::Real, value);
        let mut operation = OpTpl::new(CPUI_COPY);
        operation.set_output(VarnodeTpl::new(
            ConstTpl::new_space(ram.clone()),
            real(0x20),
            real(8),
        ));
        operation.add_input(VarnodeTpl::new(
            ConstTpl::new_space(constant),
            real(7),
            real(8),
        ));
        let mut template = ConstructTpl::new();
        assert!(template.add_op(operation));
        let payload = crate::inject_sleigh::SleighPayload::Sleigh(
            crate::inject_sleigh::InjectPayloadSleigh::new(b"test", b"entry", CALLMECHANISM_TYPE),
        );
        let mechanisms = Mechanisms {
            payloads: [(0, (payload, Some(template)))].into(),
        };
        let mut model = crate::fspec::ProtoModel::new(&manager);
        model.set_inject_ids(0, -1);
        let mut context = crate::context::ArchContext::new(manager);
        context.evalfp_current = Some(Rc::new(model));
        context.call_mechanisms = Rc::new(mechanisms);
        context.call_mechanism_effects = true;
        let address = Address::new(ram.clone(), 0x1000);
        let mut data = crate::funcdata::Funcdata::new(
            "sample",
            "sample",
            Rc::new(context),
            address.clone(),
            0x10000000,
            0x40,
        )
        .unwrap();
        let root = data.bblocks_root_pub();
        let block = data.bblocks_mut().new_block_basic(root);
        if let crate::block::BlockKind::Basic(basic) = data.bblocks_mut().arena[block].kind_mut() {
            basic.cover.insert_range(ram.clone(), 0x1000, 0x1000);
        }
        let anchor = data.new_op(1, address);
        data.op_set_opcode_code(anchor, CPUI_COPY);
        let read = data.new_varnode_space_off(8, ram.clone(), 0x20);
        data.op_set_input(anchor, read, 0).unwrap();
        data.new_varnode_out(8, &Address::new(ram, 0x28), anchor)
            .unwrap();
        data.op_insert_end(anchor, block);
        assert!(!data.get_func_proto().has_model());
        apply_entry(&mut data).unwrap();
        let injected = data
            .obank()
            .get(anchor)
            .unwrap()
            .basic_neighbours()
            .0
            .unwrap();
        let input = data.obank().get(injected).unwrap().get_in(0).unwrap();
        assert_eq!(data.vbank().get(input).unwrap().get_addr().get_offset(), 7);
        apply_entry(&mut data).unwrap();
        assert_eq!(
            data.obank().get(anchor).unwrap().basic_neighbours().0,
            Some(injected)
        );
    }

    #[test]
    fn instruction_context_placeholders_are_rejected_before_emission() {
        use kuna_num::opcodes::OpCode::CPUI_COPY;
        use kuna_sleigh::semantics::{ConstTpl, ConstType, ConstructTpl, OpTpl, VarnodeTpl};
        for kind in [
            ConstType::JStart,
            ConstType::JNext,
            ConstType::JNext2,
            ConstType::JRelative,
            ConstType::JFlowref,
            ConstType::JFlowdest,
        ] {
            let mut operation = OpTpl::new(CPUI_COPY);
            operation.add_input(VarnodeTpl::new(
                ConstTpl::new(),
                ConstTpl::new_type(kind),
                ConstTpl::new_real(ConstType::Real, 8),
            ));
            let mut template = ConstructTpl::new();
            assert!(template.add_op(operation));
            assert!(validate_template(&template).is_err());
        }
    }

    #[test]
    fn late_prototype_changes_reflow_for_added_removed_or_different_effects() {
        let manager = AddrSpaceManager::new();
        let prototype = |id| {
            let mut model = crate::fspec::ProtoModel::new(&manager);
            model.set_inject_ids(-1, id);
            let mut proto = crate::fspec::FuncProto::new();
            proto.set_model(Some(Rc::new(model)));
            proto
        };
        let empty = crate::fspec::FuncProto::new();
        let ordinary = prototype(-1);
        let first = prototype(0);
        let second = prototype(1);
        assert!(!requires_reflow(&empty, &ordinary));
        assert!(!requires_reflow(&first, &first));
        for (old, new) in [
            (&empty, &first),
            (&ordinary, &first),
            (&first, &ordinary),
            (&first, &second),
            (&second, &first),
        ] {
            assert!(requires_reflow(old, new));
        }
    }

    #[test]
    fn repeated_installations_preserve_partial_temporary_aliases_without_sharing() {
        use kuna_base::address::Address;
        use kuna_base::space::{spacetype, AddrSpace, ConstantSpace, UniqueSpace};
        use kuna_num::opcodes::OpCode::CPUI_COPY;
        use kuna_num::pcoderaw::VarnodeData;
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
            0,
            1,
            1,
        ));
        manager.insert_space(ram.clone()).unwrap();
        manager.set_default_code_space(2).unwrap();
        let temporary = manager.get_unique_space().unwrap().clone();
        let context = Rc::new(crate::context::ArchContext::new(manager));
        let address = Address::new(ram.clone(), 0x1000);
        let mut data = crate::funcdata::Funcdata::new(
            "sample",
            "sample",
            context,
            address.clone(),
            0x10000000,
            0x40,
        )
        .unwrap();
        let root = data.bblocks_root_pub();
        let block = data.bblocks_mut().new_block_basic(root);
        let anchor = data.new_op(1, address.clone());
        data.op_set_opcode_code(anchor, CPUI_COPY);
        let input = data.new_constant(8, 3);
        data.op_set_input(anchor, input, 0).unwrap();
        data.new_varnode_out(8, &Address::new(ram.clone(), 0x20), anchor)
            .unwrap();
        data.op_insert_end(anchor, block);
        let storage = |space: Rc<AddrSpace>, offset, size| VarnodeData {
            space: Some(space),
            offset,
            size,
        };
        let operations = vec![
            Operation {
                address: address.clone(),
                opcode: CPUI_COPY,
                output: Some(storage(temporary.clone(), 0x100, 8)),
                inputs: vec![storage(ram.clone(), 0x20, 8)],
            },
            Operation {
                address,
                opcode: CPUI_COPY,
                output: Some(storage(ram, 0x28, 4)),
                inputs: vec![storage(temporary, 0x104, 4)],
            },
        ];
        let installed_range = |data: &crate::funcdata::Funcdata| {
            let first = data
                .obank()
                .get(anchor)
                .unwrap()
                .basic_neighbours()
                .1
                .unwrap();
            let second = data
                .obank()
                .get(first)
                .unwrap()
                .basic_neighbours()
                .1
                .unwrap();
            let output = data.obank().get(first).unwrap().get_out().unwrap();
            let input = data.obank().get(second).unwrap().get_in(0).unwrap();
            let lo = data.vbank().get(output).unwrap().get_addr().get_offset();
            assert_eq!(
                data.vbank().get(input).unwrap().get_addr().get_offset(),
                lo + 4
            );
            (lo, lo + 8)
        };
        install_after(&mut data, anchor, &operations).unwrap();
        let first = installed_range(&data);
        install_after(&mut data, anchor, &operations).unwrap();
        let second = installed_range(&data);
        assert!(first.1 <= second.0 || second.1 <= first.0);
    }

    #[test]
    fn collection_keeps_memory_effects_and_rejects_control_flow() {
        use kuna_base::address::Address;
        use kuna_base::space::{spacetype, AddrSpace, ConstantSpace};
        use kuna_num::opcodes::OpCode::*;
        use kuna_num::pcoderaw::VarnodeData;
        use kuna_sleigh::translate::PcodeEmit;
        let register = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            8,
            1,
            2,
            0,
            0,
            0,
        ));
        let constant = Rc::new(ConstantSpace::new());
        let address = Address::new(register.clone(), 0x100);
        let value = VarnodeData {
            space: Some(register),
            offset: 0x20,
            size: 8,
        };
        let displacement = VarnodeData {
            space: Some(constant),
            offset: u64::MAX,
            size: 8,
        };
        let mut output = Operations::default();
        output.dump(
            &address,
            CPUI_INT_ADD,
            Some(&value),
            &[value.clone(), displacement],
        );
        output.dump(
            &address,
            CPUI_STORE,
            None,
            &[value.clone(), value.clone(), value],
        );
        assert_eq!(output.0.len(), 2);
        assert!(validate_operations(&output.0).is_ok());
        for opcode in [
            CPUI_BRANCH,
            CPUI_CBRANCH,
            CPUI_BRANCHIND,
            CPUI_CALL,
            CPUI_CALLIND,
            CPUI_RETURN,
            CPUI_MULTIEQUAL,
            CPUI_INDIRECT,
        ] {
            output.dump(&address, opcode, None, &[]);
            assert!(validate_operations(&output.0).is_err());
            output.0.pop();
        }
        output.0[0].inputs[0].space = None;
        assert!(validate_operations(&output.0).is_err());
    }

    #[test]
    fn absent_or_uncompiled_mechanisms_are_errors() {
        let manager = AddrSpaceManager::new();
        let mut context = crate::pcodeinject::InjectContext::default();
        assert!(Mechanisms::default()
            .operations(0, &manager, &mut context)
            .is_err());
        let mut library = PcodeInjectLibrarySleigh::new(0x1000);
        register(
            Some(
                r#"<compiler_spec><prototype name="pair">
          <pcode inject="uponreturn"><body>R0=R1;</body></pcode>
        </prototype></compiler_spec>"#,
            ),
            &manager,
            &mut library,
        )
        .unwrap();
        let mechanisms = Mechanisms::from_library(&library);
        assert_eq!(mechanisms.payloads.len(), 1);
        assert!(mechanisms.operations(0, &manager, &mut context).is_err());
    }

    #[test]
    fn prototype_payloads_have_distinct_boundary_and_model_identities() {
        let manager = AddrSpaceManager::new();
        let mut library = PcodeInjectLibrarySleigh::new(0x1000);
        register(Some(r#"<compiler_spec>
          <default_proto><prototype name="first">
            <pcode inject="uponreturn"><body>R0=R1;</body></pcode>
            <pcode inject="uponentry"><body>R2=R3;</body></pcode>
          </prototype></default_proto>
          <prototype name="second"><pcode inject="uponreturn"><body>R4=R5;</body></pcode></prototype>
        </compiler_spec>"#), &manager, &mut library).unwrap();
        assert_eq!(ids(&library, "first"), (1, 0));
        assert_eq!(ids(&library, "second"), (-1, 2));
        assert_eq!(ids(&library, "missing"), (-1, -1));
        assert_eq!(compiled_ids(&library, "first"), (-1, -1));
        assert_eq!(compiled_ids(&library, "second"), (-1, -1));
        assert!(register(Some(r#"<compiler_spec><prototype name="bad"><pcode inject="other"><body>R0=R1;</body></pcode></prototype></compiler_spec>"#), &manager, &mut library).is_err());
    }
}
