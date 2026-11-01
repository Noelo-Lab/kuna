//! Explicit extensions from an ABI parameter slot to its wider physical register.
use crate::{
    fspec::{param_entry_flags, ParamEntry},
    funcdata::Funcdata,
};
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};

#[derive(Clone, Debug)]
pub(crate) struct Extension {
    pub input: VarnodeData,
    output: VarnodeData,
    opcode: OpCode,
}

fn overlaps(extension: &Extension, entry: &ParamEntry) -> bool {
    let container = extension.output.get_addr();
    let overlaps_range = |address: Address, size: i32| {
        container.overlap(0, &address, size) >= 0
            || address.overlap(0, &container, extension.output.size as i32) >= 0
    };
    if let Some(join) = entry.get_join_record() {
        (0..join.num_pieces()).any(|i| {
            let piece = join.get_piece(i);
            overlaps_range(piece.get_addr(), piece.size as i32)
        })
    } else {
        overlaps_range(
            Address::new(entry.get_space().clone(), entry.get_base()),
            entry.get_size(),
        )
    }
}

fn extensions(entries: &[ParamEntry]) -> impl Iterator<Item = &Extension> {
    entries.iter().enumerate().filter_map(|(index, entry)| {
        let extension = entry.register_extension.as_ref()?;
        (!entries
            .iter()
            .enumerate()
            .any(|(other, peer)| other != index && overlaps(extension, peer)))
        .then_some(extension)
    })
}

/// `registerextension` states only the slot-to-register extension, independently
/// of how a smaller C type fills the slot. Existing fixed `extension` declarations
/// continue to cover both extents.
pub(crate) fn decode(
    element: &kuna_base::xml::Element,
    register: Option<VarnodeData>,
    size: i32,
    alignment: i32,
    flags: u32,
) -> KunaResult<Option<Extension>> {
    let explicit = element.get_attribute_value("registerextension").ok();
    let flags = match explicit {
        None => flags,
        Some(b"sign") => param_entry_flags::SMALLSIZE_SEXT,
        Some(b"zero") => param_entry_flags::SMALLSIZE_ZEXT,
        _ => {
            return Err(KunaError::lowlevel(
                "Bad <pentry> registerextension attribute",
            ))
        }
    };
    let extension = declared(register, size, alignment, flags);
    if explicit.is_some() && extension.is_none() {
        return Err(KunaError::lowlevel(
            "registerextension requires a slot in a wider register",
        ));
    }
    Ok(extension)
}

pub(crate) fn assumed(
    entries: &[ParamEntry],
    addr: &Address,
    size: i32,
    result: &mut VarnodeData,
) -> Option<OpCode> {
    let extension =
        extensions(entries).find(|e| e.input.size as i32 == size && e.input.get_addr() == *addr)?;
    *result = extension.output.clone();
    Some(extension.opcode)
}

/// `maxsize` remains the slot width; an explicit fixed extension describes the
/// rest of a wider named register. No extension is inferred from register aliases.
pub(crate) fn declared(
    register: Option<VarnodeData>,
    size: i32,
    alignment: i32,
    flags: u32,
) -> Option<Extension> {
    let output = register?;
    if size <= 0 || output.size <= size as u32 || alignment != 0 {
        return None;
    }
    let opcode =
        match flags & (param_entry_flags::SMALLSIZE_SEXT | param_entry_flags::SMALLSIZE_ZEXT) {
            param_entry_flags::SMALLSIZE_SEXT => OpCode::CPUI_INT_SEXT,
            param_entry_flags::SMALLSIZE_ZEXT => OpCode::CPUI_INT_ZEXT,
            _ => return None,
        };
    let space = output.space.as_ref()?;
    if space.is_reverse_justified() {
        return None;
    }
    let mut input = output.clone();
    input.size = size as u32;
    if space.is_big_endian() {
        input.offset += u64::from(output.size - input.size);
    }
    Some(Extension {
        input,
        output,
        opcode,
    })
}

/// Canonicalize incoming argument words before SSA, for both inferred and locked
/// prototypes. Entries sharing the physical container cannot state this unconditionally.
pub(crate) fn apply(data: &mut Funcdata) -> KunaResult<()> {
    if data.kuna_call_mechanisms.register_slots
        || data.bblocks_get_size() == 0
        || data.get_func_proto().has_custom_storage()
    {
        return Ok(());
    }
    let Some(model) = crate::kuna_callmechanisms::evaluation_model(data) else {
        return Ok(());
    };
    let Some(input) = model.input_opt() else {
        return Ok(());
    };
    let entries = input.get_entry();
    let block = data.bblocks_get_block(0);
    let start = data.bblocks_block_start(block);
    for extension in extensions(entries) {
        let container = extension.output.get_addr();
        let op = data.new_op(1, start.clone());
        data.new_varnode_out(extension.output.size as i32, &container, op)?;
        let input = data.new_varnode(
            extension.input.size as i32,
            &extension.input.get_addr(),
            None,
        );
        data.op_set_opcode_code(op, extension.opcode);
        data.op_set_input(op, input, 0)?;
        data.op_insert_begin(op, block);
    }
    data.kuna_call_mechanisms.register_slots = true;
    Ok(())
}

/// Scalar slots use the existing assumed-extension query. A joined return needs
/// the same extension on each physical piece; a narrow result may first be
/// extended to its slot by the existing type-aware rule. Install before that
/// rule so its word definition precedes the physical-register definition.
pub(crate) fn apply_return(data: &mut Funcdata, index: i32) -> KunaResult<()> {
    let call = data.get_call_specs(index);
    let proto = call.proto();
    if !proto.has_model()
        || !proto.is_output_locked()
        || proto.has_custom_storage()
        || proto.is_no_return()
    {
        return Ok(());
    }
    let model = proto.model();
    if extensions(model.output().get_entry()).next().is_none() {
        return Ok(());
    }
    let output = proto.get_output();
    let addr = output.get_address();
    let size = output.get_size();
    if size == 0 {
        return Ok(());
    }
    let model = model.clone();
    let mut filled = VarnodeData::default();
    let (opcode, _) = crate::kuna_narrowext::or_model(
        data.get_arch().narrow_ext.output,
        model.output(),
        &addr,
        size,
        output.get_type().map(|ty| &**ty),
        &mut filled,
    );
    let join = if addr.is_join() {
        Some(data.get_arch().manage().find_join(addr.get_offset())?)
    } else {
        None
    };
    let op = call.get_op();
    let pc = data
        .obank()
        .get(op)
        .expect("return-slot call")
        .get_addr()
        .clone();
    for extension in extensions(model.output().get_entry()) {
        let slot = extension.input.get_addr();
        let in_join = join.as_ref().is_some_and(|join| {
            (0..join.num_pieces()).any(|i| {
                let piece = join.get_piece(i);
                piece.get_addr().justified_contain(
                    piece.size as i32,
                    &slot,
                    extension.input.size as i32,
                    false,
                ) >= 0
            })
        });
        let filled_word = opcode != OpCode::CPUI_COPY
            && filled.size == extension.input.size
            && filled.get_addr() == slot;
        if !in_join && !filled_word {
            continue;
        }
        let extend = data.new_op(1, pc.clone());
        data.new_varnode_out(
            extension.output.size as i32,
            &extension.output.get_addr(),
            extend,
        )?;
        let input = data.new_varnode(extension.input.size as i32, &slot, None);
        data.op_set_opcode_code(extend, extension.opcode);
        data.op_set_input(extend, input, 0)?;
        data.op_insert_after(extend, op);
    }
    Ok(())
}

/// Recover the logical return trial when heritage groups its read with the
/// wider register. The upper bytes are defined by the compiler declaration,
/// without enabling the generic overlap heuristic or widening the trial.
pub(crate) fn guard_output(
    data: &mut Funcdata,
    call: &mut crate::fspec::FuncCallSpecs,
    addr: &Address,
    translated: &Address,
    size: i32,
    write: &mut Vec<crate::context::VarnodeId>,
) -> bool {
    let Some(extension) = extensions(call.proto().model().output().get_entry())
        .find(|e| e.output.size as i32 == size && e.output.get_addr() == *translated)
        .cloned()
    else {
        return false;
    };
    let slot = addr + extension.input.offset.wrapping_sub(extension.output.offset) as i64;
    let slot_size = extension.input.size as i32;
    if call.get_active_output().which_trial(&slot, slot_size) >= 0 {
        return false;
    }
    let op = call.get_op();
    let creation = data.new_indirect_creation(op, &slot, slot_size, true);
    let input = data
        .obank()
        .get(creation)
        .and_then(|op| op.get_out())
        .expect("return-slot creation");
    let pc = data
        .obank()
        .get(op)
        .expect("return-slot call")
        .get_addr()
        .clone();
    let extend = data.new_op(1, pc);
    let output = data
        .new_varnode_out(size, addr, extend)
        .expect("return-slot extension");
    data.op_set_opcode_code(extend, extension.opcode);
    data.op_set_input(extend, input, 0)
        .expect("return-slot input");
    data.op_insert_after(extend, op);
    data.vbank_mut()
        .get_mut(output)
        .expect("return-slot output")
        .set_active_heritage();
    write.push(output);
    call.get_active_output().register_trial(&slot, slot_size);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::{spacetype, AddrSpace};
    use std::rc::Rc;

    fn register(big: bool, size: u32) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "register",
                big,
                4,
                1,
                4,
                0,
                0,
                0,
            ))),
            offset: 0x20,
            size,
        }
    }

    #[test]
    fn register_extension_does_not_invent_a_subword_extension() {
        use crate::{
            dtype::type_class,
            fspec::{ParamListKind, ParamListStandard},
        };
        use kuna_base::{space::AddrSpaceManager, xml::DocumentStorage};
        let mut storage = DocumentStorage::new();
        let document = storage
            .parse_document(b"<pentry registerextension=\"sign\"/>")
            .unwrap();
        for big in [false, true] {
            let reg = register(big, 8);
            let extension = decode(document.get_root(), Some(reg.clone()), 4, 0, 0)
                .unwrap()
                .unwrap();
            let slot = extension.input.get_addr();
            let mut entry = ParamEntry::seed(
                0,
                type_class::TYPECLASS_GENERAL,
                reg.space.clone().unwrap(),
                slot.get_offset(),
                4,
                1,
                0,
                0,
                true,
                false,
                &[],
                &AddrSpaceManager::new(),
            )
            .unwrap();
            entry.register_extension = Some(extension);
            let mut model = ParamListStandard::new(ParamListKind::StandardOut);
            model.push_entry(entry);
            model.finish_decode();
            let mut result = VarnodeData::default();
            assert_eq!(
                model.assumed_extension(&slot, 4, &mut result),
                OpCode::CPUI_INT_SEXT
            );
            assert_eq!(result.get_addr(), reg.get_addr());
            assert_eq!(result.size, 8);
            let byte = &slot + if big { 3 } else { 0 };
            assert_eq!(
                model.assumed_extension(&byte, 1, &mut result),
                OpCode::CPUI_COPY
            );
        }
        assert!(decode(document.get_root(), Some(register(false, 4)), 4, 0, 0).is_err());
        assert!(decode(document.get_root(), None, 4, 0, 0).is_err());
        let invalid = storage
            .parse_document(b"<pentry registerextension=\"inttype\"/>")
            .unwrap();
        assert!(decode(invalid.get_root(), Some(register(false, 8)), 4, 0, 0).is_err());
    }

    #[test]
    fn word_slots_keep_their_physical_container_in_both_byte_orders() {
        for big in [false, true] {
            for (flag, opcode) in [
                (param_entry_flags::SMALLSIZE_SEXT, OpCode::CPUI_INT_SEXT),
                (param_entry_flags::SMALLSIZE_ZEXT, OpCode::CPUI_INT_ZEXT),
            ] {
                let extension = declared(Some(register(big, 8)), 4, 0, flag).unwrap();
                assert_eq!(extension.input.offset, if big { 0x24 } else { 0x20 });
                assert_eq!(extension.input.size, 4);
                assert_eq!(extension.output.offset, 0x20);
                assert_eq!(extension.output.size, 8);
                assert_eq!(extension.opcode, opcode);
            }
        }
    }

    #[test]
    fn aliases_and_conditional_or_missing_extensions_do_not_invent_upper_bits() {
        for big in [false, true] {
            assert!(declared(
                Some(register(big, 4)),
                4,
                0,
                param_entry_flags::SMALLSIZE_SEXT
            )
            .is_none());
            for flag in [
                0,
                param_entry_flags::SMALLSIZE_INTTYPE,
                param_entry_flags::SMALLSIZE_FLOATEXT,
            ] {
                assert!(declared(Some(register(big, 8)), 4, 0, flag).is_none());
            }
            assert!(declared(
                Some(register(big, 8)),
                4,
                4,
                param_entry_flags::SMALLSIZE_SEXT
            )
            .is_none());
            assert!(declared(None, 4, 0, param_entry_flags::SMALLSIZE_SEXT).is_none());
        }
    }

    #[test]
    fn alternative_entries_may_overlap_either_end_of_the_register() {
        use crate::dtype::type_class;
        use kuna_base::space::AddrSpaceManager;
        let register = register(false, 8);
        let extension = declared(
            Some(register.clone()),
            4,
            0,
            param_entry_flags::SMALLSIZE_SEXT,
        )
        .unwrap();
        for (base, size, expected) in [
            (0x1c, 4, false),
            (0x1c, 8, true),
            (0x20, 8, true),
            (0x24, 8, true),
            (0x28, 4, false),
        ] {
            let entry = ParamEntry::seed(
                0,
                type_class::TYPECLASS_GENERAL,
                register.space.clone().unwrap(),
                base,
                size,
                1,
                0,
                0,
                true,
                false,
                &[],
                &AddrSpaceManager::new(),
            )
            .unwrap();
            assert_eq!(overlaps(&extension, &entry), expected);
        }
    }
}
