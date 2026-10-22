//! A SUBPIECE into a byte-array variable copies a word of the source's storage bits.
//!
//! The array type cannot be a cast target, so the truncation prints as an unsigned word of
//! the destination's width taken from the source's bits; a floating source is reinterpreted
//! rather than converted.

use crate::context::OpId;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::p9_emit::kuna_floatslice::Slice;
use kuna_num::opcodes::OpCode;

/// The slice and whether the source needs a word cast before shifting (a pointer does).
pub(crate) fn recognize(fd: &Funcdata, decl_high_type: bool, op: OpId) -> Option<(Slice, bool)> {
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_SUBPIECE {
        return None;
    }
    let input = operation.get_in(0)?;
    let source = fd.vbank().get(input)?;
    let output = operation.get_out()?;
    let destination = fd.vbank().get(output)?;
    let offset = fd.vbank().get(operation.get_in(1)?)?;
    let (from_size, to_size) = (source.get_size(), destination.get_size());
    if !matches!(from_size, 2 | 4 | 8)
        || !matches!(to_size, 1 | 2 | 4)
        || !offset.is_constant()
        || offset.get_offset() + to_size as u64 > from_size as u64
    {
        return None;
    }
    let target = crate::kuna_stackviews::access_type(fd, output, op, -1)
        .unwrap_or_else(|| fd.vn_type_def_facing(output));
    let storage = target.get_partial_base().unwrap_or(target);
    if storage.get_size() < to_size || !super::kuna_bytearrayc::byte_array(&storage) {
        return None;
    }
    let from = crate::kuna_stackviews::access_type(fd, input, op, operation.get_slot(input))
        .or_else(|| crate::printc::declared_variable_type(fd, decl_high_type, input))
        .unwrap_or_else(|| fd.vn_type_read_facing(input, op));
    let from = from.get_stripped().unwrap_or(from);
    let types = fd.get_arch().types()?;
    let source_float = match from.get_metatype() {
        type_metatype::TYPE_INT
        | type_metatype::TYPE_UINT
        | type_metatype::TYPE_UNKNOWN
        | type_metatype::TYPE_BOOL
        | type_metatype::TYPE_PTR => false,
        type_metatype::TYPE_FLOAT
            if (from_size == 4 && types.get_size_of_float() == 4)
                || (from_size == 8 && types.get_size_of_double() == 8) =>
        {
            true
        }
        _ => return None,
    };
    if from.get_size() != from_size {
        return None;
    }
    let pointer = from.get_metatype() == type_metatype::TYPE_PTR;
    Some((
        Slice {
            input,
            source_float,
            source_word: types.get_base(from_size, type_metatype::TYPE_UINT).ok()?,
            target_word: types.get_base(to_size, type_metatype::TYPE_UINT).ok()?,
            shift: offset.get_offset() * 8,
        },
        pointer,
    ))
}
