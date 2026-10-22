//! Pointer and integer storage members require explicit C representation conversions.

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(super) fn target(
    fd: &Funcdata,
    op: OpId,
    decl_high_type: bool,
    printed: impl FnOnce() -> Option<Rc<Datatype>>,
) -> Option<Rc<Datatype>> {
    let operation = fd.obank().get(op)?;
    if !matches!(
        operation.code(),
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB
    ) {
        return None;
    }
    let input = operation.get_in(0)?;
    let output = operation.get_out()?;
    // A CAST expression prints its result type, rather than its input type.
    let source = if operation.code() == OpCode::CPUI_CAST {
        fd.vn_type_def_facing(output)
    } else {
        fd.vn_type_read_facing(input, op)
    };
    if !matches!(
        source.get_metatype(),
        type_metatype::TYPE_PTR
            | type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT
            | type_metatype::TYPE_UNKNOWN
    ) {
        return None;
    }
    if operation.code() == OpCode::CPUI_COPY
        && source.get_metatype() == type_metatype::TYPE_PTR
        && crate::printc::declared_variable_type(fd, decl_high_type, input)
            .is_some_and(|declared| {
                matches!(
                    declared.get_metatype(),
                    type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
                ) && declared.get_size() == source.get_size()
            })
    {
        return None;
    }
    let value = fd.vbank().get(output)?;
    let target = printed().or_else(|| {
        let high = value.get_high().and_then(|id| fd.high_bank().get(id));
        let declared = high
            .and_then(|high| high.kuna_symbol_type())
            .filter(|ty| {
                matches!(
                    ty.get_metatype(),
                    type_metatype::TYPE_STRUCT
                        | type_metatype::TYPE_ARRAY
                        | type_metatype::TYPE_UNION
                )
            })
            .cloned()
            .or_else(|| super::kuna_bitcast::declared_member_type(fd, decl_high_type, output))?;
        let offset = high
            .map(|high| high.kuna_symbol_offset().max(0))
            .unwrap_or(0);
        crate::kuna_stackviews::scalar_piece(
            fd.get_arch().types()?,
            declared,
            offset,
            value.get_size(),
        )
    })?;
    let pointer_to_word = source.get_metatype() == type_metatype::TYPE_PTR
        && matches!(
            target.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        );
    let word_to_pointer = target.get_metatype() == type_metatype::TYPE_PTR
        && matches!(
            source.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        );
    ((pointer_to_word || word_to_pointer)
        && target.get_size() == source.get_size()
        && target.get_size() == value.get_size())
    .then_some(target)
}

/// A null pointer copied into integer storage prints as the integer `0`.
pub(super) fn null_source(fd: &Funcdata, op: OpId) -> bool {
    fd.obank()
        .get(op)
        .filter(|operation| operation.code() == OpCode::CPUI_COPY)
        .and_then(|operation| operation.get_in(0))
        .and_then(|input| fd.vbank().get(input))
        .is_some_and(|input| input.is_constant() && input.get_offset() == 0)
}
