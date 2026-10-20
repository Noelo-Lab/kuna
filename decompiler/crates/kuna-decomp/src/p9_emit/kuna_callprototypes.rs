//! C declarations for named calls with complete locked prototypes.
use crate::dtype::{type_metatype, Datatype, DatatypeKind};
use crate::{architecture::Architecture, funcdata::Funcdata};
use kuna_num::opcodes::OpCode;
use std::{collections::BTreeSet, rc::Rc};

pub fn declarations(arch: &Architecture, fd: &Funcdata) -> Vec<String> {
    let mut declarations = BTreeSet::new();
    let rt = crate::printc::RealTypeCtx::from_arch(arch, arch.print().out_lang());
    for index in 0..fd.num_calls() {
        let call = fd.get_call_specs(index);
        let Some(op) = fd.obank().get(call.get_op()) else {
            continue;
        };
        if op.is_dead() || op.code() != OpCode::CPUI_CALL {
            continue;
        }
        let proto = call.proto();
        if !proto.has_store() || !proto.is_input_locked() || !proto.is_output_locked() {
            continue;
        }
        let name = call.fspec_printed_name(arch.kuna_name_style());
        let mut bytes = name.bytes();
        if !bytes
            .next()
            .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
            || !bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            continue;
        }
        let mut snapshot = crate::fspec::FuncProto::new();
        snapshot.copy(proto);
        let parked = fd.get_arch().callee_proto_pieces(call.get_entry_address());
        if let Some(pieces) = parked.filter(|pieces| pieces.first_var_arg_slot >= 0) {
            if snapshot
                .set_pieces(&pieces, None, arch.types(), arch.manage())
                .is_err()
            {
                continue;
            }
        } else if call.format_arity().is_some() {
            continue;
        }
        if snapshot.get_output_type().is_some_and(|t| {
            matches!(t.get_metatype(), type_metatype::TYPE_ARRAY | type_metatype::TYPE_CODE)
        }) {
            continue;
        }
        let mut code = Datatype::new(1, type_metatype::TYPE_CODE);
        code.kind = DatatypeKind::Code {
            proto: Some(Rc::new(snapshot)),
        };
        declarations.insert(format!(
            "{};",
            crate::printc::declaration_text(&Rc::new(code), &name, rt)
        ));
    }
    declarations.into_iter().collect()
}

/// Project-only dependencies; ordinary per-function type inventories stay local.
pub fn type_definitions(
    arch: &Architecture,
    fd: &Funcdata,
) -> Vec<crate::decompile_drive::TypeInfo> {
    if !arch.print().options.struct_defs()
        || arch.print().out_lang() != crate::kuna_lang::OutLang::C
    {
        return Vec::new();
    }
    let rt = crate::printc::RealTypeCtx::from_arch(arch, arch.print().out_lang());
    let types = crate::kuna_structdefs::referenced_call_types(fd);
    crate::kuna_structdefs::dedup_by_name(&types)
        .iter()
        .map(|ty| crate::decompile_drive::TypeInfo {
            name: ty.get_name().to_string(),
            definition: crate::kuna_structdefs::definition_text(ty, rt),
            size: ty.get_size() as i64,
        })
        .collect()
}
