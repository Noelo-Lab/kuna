//! A frame's byte-array view is storage metadata, not a stored value's type.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

pub(crate) fn backing_store(
    fd: &Funcdata,
    op: OpId,
    inslot: i32,
    outslot: i32,
    input: VarnodeId,
    ty: &Rc<Datatype>,
) -> bool {
    if !fd.get_arch().stack_views
        || inslot != 2
        || outslot != 1
        || fd
            .obank()
            .get(op)
            .is_none_or(|op| op.code() != OpCode::CPUI_STORE)
    {
        return false;
    }
    let Some(value) = fd.vbank().get(input) else {
        return false;
    };
    if !matches!(value.get_size(), 1 | 2 | 4 | 8) {
        return false;
    }
    let generated = |ty: &Rc<Datatype>| {
        ty.get_partial_base()
            .unwrap_or_else(|| Rc::clone(ty))
            .get_name()
            .starts_with("stack_views_")
    };
    generated(ty)
        || generated(value.get_type())
        || value
            .get_high()
            .and_then(|id| fd.high_bank().get(id))
            .and_then(|high| high.kuna_symbol_type())
            .is_some_and(generated)
        || fd.get_scope_local().is_some_and(|scope| {
            scope
                .query_container_for_link_width(
                    value.get_addr(),
                    value.get_size(),
                    &Address::new_invalid(),
                )
                .and_then(|entry| entry.sym_type)
                .is_some_and(|ty| generated(&ty))
        })
}

pub(crate) fn stored_word(fd: &Funcdata, input: VarnodeId, resolved: Rc<Datatype>) -> Rc<Datatype> {
    let Some(factory) = fd.get_arch().types() else {
        return resolved;
    };
    let size = fd.vbank().get(input).unwrap().get_size();
    crate::kuna_stackviews::scalar_piece(factory, Rc::clone(&resolved), 0, size)
        .or_else(|| factory.get_base(size, type_metatype::TYPE_UINT).ok())
        .unwrap_or(resolved)
}
