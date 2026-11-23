//! Scope the blocking receive prototype to a proven status-only Go wrapper.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::dtype::type_metatype;
use crate::fspec::{FuncProto, ProtoParameter, PrototypePieces};
use crate::funcdata::Funcdata;

const HELPER: &str = "runtime.chanrecv2";

/// Verify the guarded hot path preserves RAX, passes a private word in RBX,
/// and returns with AL untouched after a direct receive.
pub fn status_site(arch: &Architecture, entry: u64) -> Option<u64> {
    let target = arch.go_chanrecv2?;
    if arch.archid != "x86:LE:64:default:golang"
        || !arch.source_lang_detected.is_go()
        || !arch.source_lang().is_go()
    {
        return None;
    }
    const PREFIX: [u8; 19] = [
        0x49, 0x3b, 0x66, 0x10, 0x76, 0x18, 0x55, 0x48, 0x89, 0xe5, 0x48, 0x83, 0xec, 0x18, 0x48,
        0x8d, 0x5c, 0x24, 0x10,
    ];
    const RESTORE: [u8; 6] = [0x48, 0x83, 0xc4, 0x18, 0x5d, 0xc3];
    let mut bytes = [0; 30];
    let end = entry.checked_add(bytes.len() as u64)?;
    let image = arch
        .translate()
        .loader_rc()
        .try_borrow()
        .ok()?
        .shared_bytes()?;
    if !image.mapped_covers(entry, end) || image.fill_span(&mut bytes, entry) != 0 {
        return None;
    }
    let (prefix, call_and_restore) = bytes.split_at(PREFIX.len());
    let [0xe8, a, b, c, d, restore @ ..] = call_and_restore else {
        return None;
    };
    if prefix != PREFIX || restore != RESTORE {
        return None;
    }
    let site = entry.checked_add(PREFIX.len() as u64)?;
    let relative = i64::from(i32::from_le_bytes([*a, *b, *c, *d]));
    (site.checked_add(5)?.checked_add_signed(relative)? == target).then_some(site)
}

fn in_register(arch: &Architecture, slot: &dyn ProtoParameter, name: &[u8], width: int4) -> bool {
    arch.get_register_varnode(name).ok().is_some_and(|storage| {
        storage.size as int4 == width
            && slot.get_size() == width
            && slot.get_address() == storage.get_addr()
    })
}

pub(crate) fn seed(arch: &Architecture, data: &mut Funcdata) {
    let Some((index, site, mut proto)) = contract(arch, data) else {
        return;
    };
    proto.set_override(true);
    data.get_call_specs_mut(index).proto_mut().copy(&proto);
    data.get_override_mut()
        .insert_proto_override(site, Box::new(proto));
}

fn contract(arch: &Architecture, data: &Funcdata) -> Option<(int4, Address, FuncProto)> {
    let caller_entry = data.get_address();
    let site = *arch
        .go_chanrecv_status_sites
        .get(&caller_entry.get_offset())?;
    let space = arch.manage().get_default_code_space()?;
    if caller_entry.get_space()?.get_index() != space.get_index()
        || status_site(arch, caller_entry.get_offset())? != site
    {
        return None;
    }
    let site = Address::new(Rc::clone(space), site);
    let target = Address::new(Rc::clone(space), arch.go_chanrecv2?);
    let caller = data.get_func_proto();
    if !caller.has_model()
        || !caller.has_store()
        || caller.get_model_name() != "abi-internal"
        || !caller.is_output_locked()
        || !caller.is_input_locked()
        || caller.is_dotdotdot()
        || caller.num_params() != 1
        || data.get_override().find_proto_override(&site).is_some()
        || arch
            .symboltab
            .function_proto_pieces_across_scopes(&target)
            .is_some()
    {
        return None;
    }
    let output = caller.get_output();
    let bool_type = output.get_type()?.clone();
    let input = caller.get_param(0)?;
    if bool_type.get_metatype() != type_metatype::TYPE_BOOL
        || input.get_type()?.get_metatype() != type_metatype::TYPE_PTR
        || !in_register(arch, output, b"AL", 1)
        || !in_register(arch, input, b"RAX", 8)
    {
        return None;
    }
    let mut calls = (0..data.num_calls()).filter(|&index| {
        let fc = data.get_call_specs(index);
        fc.get_entry_address() == &target
            && fc.get_name() == HELPER
            && !fc.proto().has_model()
            && !fc.proto().is_inline()
            && !fc.proto().is_no_return()
            && fc
                .callee_reg_model()
                .is_none_or(|model| model.get_name() == "abi-internal")
            && data.obank().get(fc.get_op()).is_some_and(|op| {
                !op.is_dead() && op.code() == OpCode::CPUI_CALL && op.get_addr() == &site
            })
    });
    let index = calls.next()?;
    if calls.next().is_some() {
        return None;
    }
    let model = arch
        .default_fp()
        .filter(|model| model.get_name() == "abi-internal")?
        .clone();
    let types = arch.types();
    let void = types.get_type_void().ok()?;
    let channel = types.get_type_pointer(8, Rc::clone(&void), 1).ok()?;
    let word = types.get_base(8, type_metatype::TYPE_UINT).ok()?;
    let scratch = types.get_type_pointer(8, word, 1).ok()?;
    let pieces = PrototypePieces {
        name: HELPER.to_string(),
        outtype: Some(bool_type),
        intypes: vec![channel, scratch],
        innames: vec!["channel".to_string(), "scratch".to_string()],
        first_var_arg_slot: -1,
        ..PrototypePieces::default()
    };
    let mut proto = FuncProto::new();
    proto
        .seed_locked_from_pieces(&pieces, model, void, types, arch.manage())
        .ok()?;
    if !in_register(arch, proto.get_output(), b"AL", 1)
        || !in_register(arch, proto.get_param(0)?, b"RAX", 8)
        || !in_register(arch, proto.get_param(1)?, b"RBX", 8)
    {
        return None;
    }
    Some((index, site, proto))
}
