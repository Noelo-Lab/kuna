//! Method expressions for direct calls with an admitted receiver declaration.

use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

use super::kuna_langgo::identifier;

pub(super) struct DirectMethod<'a> {
    pub receiver: VarnodeId,
    pub type_name: String,
    pub member: String,
    qualified_type: &'a str,
}

impl DirectMethod<'_> {
    pub(super) fn has_selector_type(&self, ty: &Datatype) -> bool {
        ty.get_metatype() == type_metatype::TYPE_PTR
            && ty.get_typedef().is_none()
            && ty.get_ptr_to().is_some_and(|pointee| {
                pointee.get_metatype() == type_metatype::TYPE_STRUCT
                    && pointee.get_name() == self.qualified_type
            })
    }
}

pub(super) fn direct_method<'a>(
    fd: &Funcdata,
    arch: &'a Architecture,
    id: OpId,
) -> Option<DirectMethod<'a>> {
    if !arch.analysis_godwarfsig || !fd.get_arch().source_lang.is_go() {
        return None;
    }
    let op = fd.obank().get(id)?;
    if op.code() != OpCode::CPUI_CALL
        || op.num_input() != 3
        || fd
            .get_override()
            .find_proto_override(op.get_addr())
            .is_some()
    {
        return None;
    }
    let call = fd.get_call_specs(fd.get_call_specs_index(id)?);
    let receiver = arch
        .symboltab
        .go_pointer_receiver(call.get_entry_address())?;
    let package = super::kuna_gopackage::function_package(fd.get_name())?;
    if package != receiver.package
        || call.get_name() != receiver.symbol
        || !receiver.matches(arch, call.proto())
        || (1..3).any(|slot| {
            op.get_in(slot)
                .and_then(|vn| fd.vbank().get(vn))
                .is_none_or(|vn| vn.get_size() != 8)
        })
    {
        return None;
    }
    let name = super::kuna_gopackage::local_type_name(Some(package), &receiver.type_name);
    Some(DirectMethod {
        receiver: op.get_in(1)?,
        type_name: identifier(name),
        member: identifier(&receiver.method),
        qualified_type: &receiver.type_name,
    })
}
