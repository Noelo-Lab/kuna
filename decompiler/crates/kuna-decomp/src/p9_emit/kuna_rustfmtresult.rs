//! Preserve the hidden String destination of verified Rust format_inner calls.

use super::kuna_rustfmtargs::{self, MacroKind};
use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) struct Plan {
    pub destination: VarnodeId,
    pub pointer_bridge: bool,
    pub format: kuna_rustfmtargs::Plan,
}

pub(crate) fn recover(fd: &Funcdata, arch: &Architecture, id: OpId) -> Option<Plan> {
    if !arch.rust_fmt_result
        || !arch.rust_fmt_args
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
        || !matches!(arch.rustc_stable_version, Some((1, 93, 0 | 1)))
    {
        return None;
    }
    let call = fd.obank().get(id)?;
    if call.code() != OpCode::CPUI_CALL || call.num_input() != 4 || call.get_out().is_some() {
        return None;
    }
    let spec = fd.get_call_specs(fd.get_call_specs_index(id)?);
    match spec.get_name() {
        "alloc::fmt::format::format_inner" => (),
        "format_inner" => {
            let (leaf, offset, _, scope) = fd
                .get_arch()
                .global_name_scope()?
                .name_for_varnode_scoped(spec.get_entry_address(), 1, call.get_addr())?;
            if offset != 0 || leaf != "format_inner" || scope != ["format", "fmt", "alloc"] {
                return None;
            }
        }
        _ => return None,
    }
    let destination = call.get_in(1)?;
    let value = fd.vbank().get(destination)?;
    if value.get_size() != 8 {
        return None;
    }
    let pointer_bridge = match value.get_type_read_facing(id).get_metatype() {
        type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => true,
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => false,
        _ => return None,
    };
    if !entry_value(fd, destination) {
        return None;
    }
    let format = kuna_rustfmtargs::recover_at(fd, arch, id, 2, 3, MacroKind::Format)?;
    Some(Plan {
        destination,
        pointer_bridge,
        format,
    })
}

fn entry_value(fd: &Funcdata, mut id: VarnodeId) -> bool {
    for _ in 0..8 {
        let Some(value) = fd.vbank().get(id) else {
            return false;
        };
        if value.get_size() != 8 || value.is_constant() {
            return false;
        }
        if value.is_input() {
            return !value.is_spacebase() && !value.is_spacebase_placeholder();
        }
        let Some(op) = value.get_def().and_then(|id| fd.obank().get(id)) else {
            return false;
        };
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return false;
        }
        let Some(input) = op.get_in(0) else {
            return false;
        };
        id = input;
    }
    false
}
