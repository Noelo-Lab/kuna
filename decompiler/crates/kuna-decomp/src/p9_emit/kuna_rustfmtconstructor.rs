//! Transport a complete, image-verified Arguments result to its stderr consumer.

use super::kuna_rustfmtargs::{self, Argument, Plan};
use super::kuna_rustfmtflags::Trait;
use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::fspec::{FuncCallSpecs, FuncProto};
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub const NAME: &str = "core::fmt::Arguments::new";
pub const BODY: [u8; 17] = [
    0x48, 0x89, 0xf2, 0x48, 0x89, 0xf8, 0x48, 0x89, 0x44, 0x24, 0xf0, 0x48, 0x89, 0x54, 0x24, 0xf8,
    0xc3,
];

fn register(arch: &Architecture, address: &Address, size: i32, name: &str) -> bool {
    let Ok(expected) = arch.get_register_varnode(name.as_bytes()) else {
        return false;
    };
    size == 8
        && expected.size == 8
        && address.get_offset() == expected.offset
        && address
            .get_space()
            .zip(expected.space.as_ref())
            .is_some_and(|(actual, expected)| actual.get_index() == expected.get_index())
}

fn pair(arch: &Architecture, address: &Address, names: [&str; 2]) -> bool {
    if !address.is_join() {
        return false;
    }
    let Ok(join) = arch.manage().find_join(address.get_offset()) else {
        return false;
    };
    join.get_unified().size == 16
        && join.num_pieces() == 2
        && names.iter().enumerate().all(|(index, name)| {
            let piece = join.get_piece(index as i32);
            piece.space.as_ref().is_some_and(|space| {
                register(
                    arch,
                    &Address::new(space.clone(), piece.offset),
                    piece.size as i32,
                    name,
                )
            })
        })
}

fn arguments_type(ty: &Datatype) -> bool {
    ty.get_name() == "core::fmt::Arguments"
        && ty.get_metatype() == type_metatype::TYPE_STRUCT
        && ty.get_size() == 16
        && !ty.is_incomplete()
        && !crate::kuna_typequal::is_volatile(ty)
}

fn ordinary_call(proto: &FuncProto) -> bool {
    proto.has_store()
        && !proto.is_dotdotdot()
        && !proto.is_inline()
        && !proto.is_no_return()
        && proto.get_inject_id() < 0
}

fn input_contract(arch: &Architecture, proto: &FuncProto, logical_pair: bool) -> bool {
    if !ordinary_call(proto) {
        return false;
    }
    if logical_pair && proto.num_params() == 1 {
        return proto.get_param(0).is_some_and(|param| {
            !param.is_hidden_return()
                && !param.is_indirect_storage()
                && param.get_size() == 16
                && param.get_type().is_some_and(|ty| arguments_type(ty))
                && pair(arch, &param.get_address(), ["RSI", "RDI"])
        });
    }
    proto.num_params() == 2
        && ["RDI", "RSI"].iter().enumerate().all(|(index, name)| {
            proto.get_param(index as i32).is_some_and(|param| {
                !param.is_hidden_return()
                    && !param.is_indirect_storage()
                    && register(arch, &param.get_address(), param.get_size(), name)
                    && param
                        .get_type()
                        .is_some_and(|ty| !crate::kuna_typequal::is_volatile(ty))
            })
        })
}

fn finalized_sink_inputs(call: &FuncCallSpecs) -> Option<&[(Address, i32)]> {
    let proto = call.proto();
    if !ordinary_call(proto)
        || proto.num_params() != 0
        || proto.is_input_locked()
        || !proto.has_model()
        || proto.is_model_unknown()
        || proto.has_input_errors()
        || call.is_input_active()
        || call.get_paramshift() != 0
    {
        return None;
    }
    let storage = call.final_input_storage();
    (storage.len() == 2 && storage.iter().all(|(_, size)| *size == 8)).then_some(storage)
}

fn sink_contract(arch: &Architecture, call: &FuncCallSpecs) -> bool {
    input_contract(arch, call.proto(), true)
        || finalized_sink_inputs(call).is_some_and(|storage| {
            storage
                .iter()
                .zip(["RDI", "RSI"])
                .all(|((address, size), name)| register(arch, address, *size, name))
        })
}

fn authority_unchanged(fd: &Funcdata, call: OpId) -> Option<()> {
    let address = fd.obank().get(call)?.get_addr();
    (fd.get_override().find_proto_override(address).is_none()
        && fd.get_override().find_indirect_override(address).is_none())
    .then_some(())
}

fn result_owner(fd: &Funcdata, mut value: VarnodeId, offset: u64) -> Option<VarnodeId> {
    for _ in 0..8 {
        let node = fd.vbank().get(value)?;
        if node.get_size() != 8 || node.is_volatile() || node.is_persist() {
            return None;
        }
        let op = fd.obank().get(node.get_def()?)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST if op.num_input() == 1 => value = op.get_in(0)?,
            OpCode::CPUI_SUBPIECE if op.num_input() == 2 => {
                let position = fd.vbank().get(op.get_in(1)?)?;
                let owner = op.get_in(0)?;
                let whole = fd.vbank().get(owner)?;
                return (position.is_constant()
                    && position.get_offset() == offset
                    && whole.get_size() == 16
                    && !whole.is_volatile()
                    && !whole.is_persist())
                .then_some(owner);
            }
            _ => return None,
        }
    }
    None
}

fn producer(fd: &Funcdata, arch: &Architecture, owner: VarnodeId) -> Option<OpId> {
    let node = fd.vbank().get(owner)?;
    let id = node.get_def()?;
    let op = fd.obank().get(id)?;
    if op.code() != OpCode::CPUI_CALL || op.num_input() != 3 || op.get_out() != Some(owner) {
        return None;
    }
    authority_unchanged(fd, id)?;
    let spec = fd.get_call_specs(fd.get_call_specs_index(id)?);
    let entry = spec.get_entry_address();
    let profile = arch.rust_fmt_constructors.get(&entry.get_offset())?;
    if entry.is_invalid()
        || entry.get_space()?.get_index()
            != fd.get_arch().manage().get_default_code_space()?.get_index()
        || profile.address != entry.get_offset()
        || profile.bytes != BODY
        || !super::kuna_rustfmtio::spans_unchanged(fd, std::slice::from_ref(profile), None)
    {
        return None;
    }
    match spec.get_name() {
        NAME => (),
        "new" => {
            let (leaf, offset, _, scope) = fd
                .get_arch()
                .global_name_scope()?
                .name_for_varnode_scoped(entry, 1, op.get_addr())?;
            if leaf != "new" || offset != 0 || scope != ["Arguments", "fmt", "core"] {
                return None;
            }
        }
        _ => return None,
    }
    let proto = spec.proto();
    if !input_contract(arch, proto, false) || !proto.is_output_locked() {
        return None;
    }
    let output = proto.get_output();
    let ty = proto.get_output_type()?;
    (output.get_size() == 16
        && output.get_address() == *node.get_addr()
        && !output.is_hidden_return()
        && !output.is_indirect_storage()
        && arguments_type(ty)
        && matches!(ty.compare(&fd.vn_type_def_facing(owner), 10), Ok(0))
        && pair(arch, node.get_addr(), ["RDX", "RAX"]))
    .then_some(id)
}

fn no_effects_between(fd: &Funcdata, producer: OpId, sink: OpId) -> Option<()> {
    let block = fd.obank().get(producer)?.get_parent()?;
    if fd.obank().get(sink)?.get_parent()? != block {
        return None;
    }
    let mut seen = false;
    for id in fd.bb_ops(block) {
        if id == sink {
            return seen.then_some(());
        }
        if id == producer {
            seen = true;
            continue;
        }
        if !seen {
            continue;
        }
        let op = fd.obank().get(id)?;
        if (0..op.num_input())
            .filter_map(|slot| op.get_in(slot))
            .chain(op.get_out())
            .any(|value| {
                fd.vbank()
                    .get(value)
                    .is_none_or(|v| v.is_volatile() || v.is_persist())
            })
        {
            return None;
        }
        if op.code() != OpCode::CPUI_INDIRECT
            && op.get_out().is_some_and(|value| {
                fd.vbank().get(value).is_some_and(|v| {
                    v.get_space().get_type() == kuna_base::space::spacetype::IPTR_SPACEBASE
                })
            })
        {
            return None;
        }
        match op.code() {
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB => (),
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => {
                let effect = fd.vbank().get(op.get_in(1)?)?;
                let cause = OpId::from(slotmap::KeyData::from_ffi(effect.get_offset()));
                if cause != producer && cause != sink {
                    return None;
                }
            }
            _ => return None,
        }
    }
    None
}

fn frame_ranges(entry: i64, sink_entry: i64, descriptor: i64, payload: i64) -> Option<()> {
    if [entry, descriptor, payload]
        .iter()
        .any(|offset| offset.unsigned_abs() > 1024 * 1024)
        || [entry, descriptor, payload]
            .iter()
            .any(|offset| offset % 8 != 0)
    {
        return None;
    }
    if entry != sink_entry {
        return None;
    }
    let caller_sp = entry.checked_add(8)?;
    let scratch = (entry.checked_sub(16)?, caller_sp);
    let descriptor = (descriptor, descriptor.checked_add(16)?);
    let payload = (payload, payload.checked_add(8)?);
    if [descriptor, payload]
        .iter()
        .any(|range| range.0 < caller_sp || range.1 > 0)
    {
        return None;
    }
    let disjoint = |a: (i64, i64), b: (i64, i64)| a.1 <= b.0 || b.1 <= a.0;
    (disjoint(scratch, descriptor) && disjoint(scratch, payload) && disjoint(descriptor, payload))
        .then_some(())
}

pub(crate) fn recover(fd: &Funcdata, arch: &Architecture, id: OpId) -> Option<Plan> {
    if !arch.rust_fmt.recovers()
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
        || arch.rustc_stable_version != Some((1, 93, 1))
        || arch.rust_fmt_constructors.is_empty()
    {
        return None;
    }
    let call = fd.obank().get(id)?;
    if call.code() != OpCode::CPUI_CALL
        || call.num_input() != 3
        || call.get_out().is_some()
        || !super::kuna_rustfmtliteral::print_sink(fd, id)?
    {
        return None;
    }
    authority_unchanged(fd, id)?;
    let sink = fd.get_call_specs(fd.get_call_specs_index(id)?);
    if !sink_contract(arch, sink)
        || sink.proto().get_output_type()?.get_metatype() != type_metatype::TYPE_VOID
    {
        return None;
    }
    let owner = result_owner(fd, call.get_in(1)?, 0)?;
    if result_owner(fd, call.get_in(2)?, 8)? != owner {
        return None;
    }
    let constructor = producer(fd, arch, owner)?;
    no_effects_between(fd, constructor, id)?;
    let stack = arch.manage().get_stack_space()?;
    let space = arch.manage().get_default_data_space()?;
    let base = stack.get_spacebase(0).ok()?;
    let rsp = arch.get_register_varnode(b"RSP").ok()?;
    if stack.get_addr_size() != 8
        || stack.get_word_size() != 1
        || stack.is_big_endian()
        || space.get_addr_size() != 8
        || space.get_word_size() != 1
        || space.is_big_endian()
        || space.is_overlay()
        || space.is_overlay_base()
        || base.size != 8
        || rsp.size != 8
        || base.offset != rsp.offset
        || base.space.as_ref()?.get_index() != rsp.space.as_ref()?.get_index()
    {
        return None;
    }
    let entry = fd
        .get_call_specs(fd.get_call_specs_index(constructor)?)
        .verified_spacebase_offset()?;
    let descriptor = kuna_rustfmtargs::stack_offset(fd, fd.obank().get(constructor)?.get_in(2)?)?;
    let (arguments, formatters) =
        kuna_rustfmtargs::primitive_arguments(fd, arch, constructor, descriptor, 1)?;
    let [Argument::Primitive {
        pointer,
        primitive: "u64",
    }] = arguments.as_slice()
    else {
        return None;
    };
    if formatters[0].kind != Trait::Display {
        return None;
    }
    let payload = kuna_rustfmtargs::stack_offset(fd, *pointer)?;
    let sink_entry = fd
        .get_call_specs(fd.get_call_specs_index(id)?)
        .verified_spacebase_offset()?;
    frame_ranges(entry as i64, sink_entry as i64, descriptor, payload)?;
    let value = kuna_rustfmtargs::stack_cells(fd, constructor, payload, 1)?[0];
    if fd.vbank().get(value)?.is_volatile() || fd.vbank().get(value)?.get_size() != 8 {
        return None;
    }
    let plan = kuna_rustfmtargs::recover_single_default_at(fd, arch, constructor)?;
    match plan.arguments.as_slice() {
        [Argument::Primitive {
            pointer: decoded,
            primitive: "u64",
        }] if decoded == pointer => Some(plan),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{ArchContext, TypeOp};
    use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
    use std::rc::Rc;

    fn function() -> Funcdata {
        let mut spaces = AddrSpaceManager::new();
        spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        spaces
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
        spaces.insert_space(ram.clone()).unwrap();
        Funcdata::new(
            "observer",
            "observer",
            Rc::new(ArchContext::new(spaces)),
            Address::new(ram, 0x1000),
            0x10000000,
            64,
        )
        .unwrap()
    }

    fn operation(
        fd: &mut Funcdata,
        opcode: OpCode,
        inputs: &[VarnodeId],
        size: i32,
    ) -> (OpId, VarnodeId) {
        let address = fd.get_address().clone();
        let op = fd.new_op(inputs.len() as i32, address);
        fd.op_set_opcode(op, TypeOp::new(opcode, 0, "test"));
        for (slot, value) in inputs.iter().enumerate() {
            fd.op_set_input(op, *value, slot as i32).unwrap();
        }
        let output = fd.new_unique(size, None);
        fd.op_set_output(op, output).unwrap();
        (op, output)
    }

    fn piece(fd: &mut Funcdata, owner: VarnodeId, offset: u64, size: i32) -> VarnodeId {
        let offset = fd.new_constant(4, offset);
        operation(fd, OpCode::CPUI_SUBPIECE, &[owner, offset], size).1
    }

    #[test]
    fn finalized_sink_storage_requires_complete_unlocked_successful_recovery() {
        use crate::dtype::{TypeFactory, TypeFactoryImpl};
        use crate::fspec::{ParameterPieces, ProtoModel};
        let mut fd = function();
        let target = fd.new_constant(8, 0x3000);
        let op = operation(&mut fd, OpCode::CPUI_CALL, &[target], 16).0;
        let types = TypeFactoryImpl::new();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(4), 8, 8);
        let space = fd.get_address().get_space().unwrap().clone();
        let storage = vec![
            (Address::new(space.clone(), 0x38), 8),
            (Address::new(space, 0x30), 8),
        ];
        for case in 0..16 {
            let mut call = FuncCallSpecs::new(op, fd.get_address().clone());
            call.proto_mut()
                .attach_internal_store(types.get_type_void().unwrap());
            let mut model = ProtoModel::new(fd.get_arch().manage());
            model.build_param_list("standard").unwrap();
            call.proto_mut().set_model(Some(Rc::new(model)));
            call.set_final_input_storage(storage.clone());
            match case {
                0 => (),
                1 => call.set_final_input_storage(Vec::new()),
                2 => call
                    .set_final_input_storage(vec![(storage[0].0.clone(), 4), storage[1].clone()]),
                3 => call.set_final_input_storage(vec![storage[0].clone()]),
                4 => call.set_final_input_storage(vec![
                    storage[0].clone(),
                    storage[1].clone(),
                    storage[0].clone(),
                ]),
                5 => call.proto_mut().set_input_lock(true),
                6 => call.init_active_input(),
                7 => call.set_paramshift(1),
                8 => call.proto_mut().set_model(None),
                9 => {
                    let model = Rc::new(ProtoModel::new_unknown("unknown", call.proto().model()));
                    call.proto_mut().set_model(Some(model));
                }
                10 => call.proto_mut().set_dotdotdot(true),
                11 => call.proto_mut().set_inline(true),
                12 => call.proto_mut().set_no_return(true),
                13 => call.proto_mut().set_inject_id(1),
                14 => call.proto_mut().set_input_errors(true),
                15 => call.proto_mut().store_set_input(
                    0,
                    "stated",
                    &ParameterPieces {
                        addr: storage[0].0.clone(),
                        type_: Some(types.get_base(8, type_metatype::TYPE_UINT).unwrap()),
                        flags: 0,
                    },
                ),
                _ => unreachable!(),
            }
            assert_eq!(
                finalized_sink_inputs(&call).is_some(),
                case == 0,
                "case {case}"
            );
            if case == 0 {
                assert_eq!(finalized_sink_inputs(&call), Some(storage.as_slice()));
            }
        }
    }

    #[test]
    fn low_high_transport_keeps_one_complete_owner_and_other_uses() {
        let mut fd = function();
        let target = fd.new_constant(8, 0x3000);
        let (producer, owner) = operation(&mut fd, OpCode::CPUI_CALL, &[target], 16);
        let low = piece(&mut fd, owner, 0, 8);
        let high = piece(&mut fd, owner, 8, 8);
        let (_, low_copy) = operation(&mut fd, OpCode::CPUI_COPY, &[low], 8);
        let (_, high_cast) = operation(&mut fd, OpCode::CPUI_CAST, &[high], 8);
        let (_, other_use) = operation(&mut fd, OpCode::CPUI_COPY, &[owner], 16);
        assert_eq!(result_owner(&fd, low_copy, 0), Some(owner));
        assert_eq!(result_owner(&fd, high_cast, 8), Some(owner));
        assert_eq!(fd.obank().get(producer).unwrap().get_out(), Some(owner));
        assert!(fd.vbank().get(other_use).unwrap().get_def().is_some());
        assert_eq!(fd.vbank().get(owner).unwrap().num_descend(), 3);
    }

    #[test]
    fn transport_rejects_mixed_swapped_partial_phi_and_volatile_words() {
        let mut fd = function();
        let target = fd.new_constant(8, 0x3000);
        let owner = operation(&mut fd, OpCode::CPUI_CALL, &[target], 16).1;
        let other = operation(&mut fd, OpCode::CPUI_CALL, &[target], 16).1;
        let low = piece(&mut fd, owner, 0, 8);
        let high_other = piece(&mut fd, other, 8, 8);
        assert_ne!(result_owner(&fd, low, 0), result_owner(&fd, high_other, 8));
        assert_eq!(result_owner(&fd, low, 8), None);
        let short = piece(&mut fd, owner, 0, 4);
        assert_eq!(result_owner(&fd, short, 0), None);
        let partial_owner = fd.new_unique(8, None);
        let partial = piece(&mut fd, partial_owner, 0, 8);
        assert_eq!(result_owner(&fd, partial, 0), None);
        let (_, phi) = operation(&mut fd, OpCode::CPUI_MULTIEQUAL, &[low, high_other], 8);
        assert_eq!(result_owner(&fd, phi, 0), None);
        fd.vbank_mut()
            .get_mut(low)
            .unwrap()
            .set_flags_pub(crate::varnode::varnode_flags::volatil);
        assert_eq!(result_owner(&fd, low, 0), None);
    }

    #[test]
    fn call_entry_scratch_and_return_slot_are_byte_disjoint_from_live_storage() {
        assert_eq!(frame_ranges(-48, -48, -32, -40), Some(()));
        assert_eq!(frame_ranges(-48, -48, -16, -40), Some(()));
        assert_eq!(frame_ranges(-48, -48, -80, -40), None);
        assert_eq!(frame_ranges(-48, -48, -32, -72), None);
        assert_eq!(frame_ranges(-48, -48, -16, 8), None);
        assert_eq!(frame_ranges(-48, -40, -32, -40), None);
        assert_eq!(frame_ranges(-48, -56, -32, -40), None);
        for descriptor in [-64, -56, -48] {
            assert_eq!(frame_ranges(-48, -48, descriptor, -32), None);
        }
        for payload in [-64, -56, -48] {
            assert_eq!(frame_ranges(-48, -48, -32, payload), None);
        }
        assert_eq!(frame_ranges(-48, -48, -32, -24), None);
        assert_eq!(frame_ranges(-48, -48, -31, -40), None);
        assert_eq!(frame_ranges(i64::MIN, i64::MIN, 0, 32), None);
        assert_eq!(frame_ranges(0, 0, i64::MAX, 32), None);
    }

    #[test]
    fn intervening_call_store_load_and_unrelated_indirect_refuse_transport() {
        let mut fd = function();
        let root = fd.bblocks_ref().root.expect("bblocks root");
        let block = fd.bblocks_mut().new_block_basic(root);
        let value = fd.new_constant(8, 0);
        let (producer, _) = operation(&mut fd, OpCode::CPUI_CALL, &[value], 16);
        let (between, _) = operation(&mut fd, OpCode::CPUI_COPY, &[value], 8);
        let (sink, _) = operation(&mut fd, OpCode::CPUI_CALL, &[value], 8);
        for op in [producer, between, sink] {
            fd.bb_insert_op(op, block, None);
        }
        assert_eq!(no_effects_between(&fd, producer, sink), Some(()));
        assert_eq!(no_effects_between(&fd, sink, producer), None);
        for code in [
            OpCode::CPUI_CALL,
            OpCode::CPUI_STORE,
            OpCode::CPUI_LOAD,
            OpCode::CPUI_CALLOTHER,
            OpCode::CPUI_MULTIEQUAL,
            OpCode::CPUI_INDIRECT,
        ] {
            fd.op_set_opcode(between, TypeOp::new(code, 0, "test"));
            assert_eq!(no_effects_between(&fd, producer, sink), None);
        }
    }
}
