//! Signed machine operations on Rust's unsigned primitive storage.

use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) struct SignedTextPlan {
    pub(crate) inputs: [VarnodeId; 2],
    pub(crate) casts: [Option<&'static str>; 2],
    pub(crate) constant_types: [Option<Rc<Datatype>>; 2],
    pub(crate) output: Option<Rc<Datatype>>,
}

fn signed_operand_name(ty: &Datatype) -> Option<&'static str> {
    if !ty.is_core_type()
        || !(ty.get_metatype() == type_metatype::TYPE_UINT
            || ty.get_metatype() == type_metatype::TYPE_INT && ty.is_char_print())
    {
        return None;
    }
    match ty.get_size() {
        1 => Some("i8"),
        2 => Some("i16"),
        4 => Some("i32"),
        8 => Some("i64"),
        _ => None,
    }
}

pub(crate) fn plan(fd: &Funcdata, op: OpId) -> Option<SignedTextPlan> {
    let operation = fd.obank().get(op)?;
    let shift = match operation.code() {
        OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => false,
        OpCode::CPUI_INT_SRIGHT => true,
        _ => return None,
    };
    if operation.num_input() != 2 {
        return None;
    }
    let inputs = [operation.get_in(0)?, operation.get_in(1)?];
    let width = fd.vbank().get(inputs[0])?.get_size();
    if !matches!(width, 1 | 2 | 4 | 8) {
        return None;
    }
    let mut casts = [None, None];
    let mut constant_types = [None, None];
    for slot in 0..if shift { 1 } else { 2 } {
        let ty = fd.vn_type_read_facing(inputs[slot], op);
        if !matches!(ty.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
            || !ty.is_core_type()
            || ty.get_size() != width || fd.vbank().get(inputs[slot])?.get_size() != width
        {
            return None;
        }
        casts[slot] = signed_operand_name(&ty);
        if casts[slot].is_some() && ty.get_metatype() == type_metatype::TYPE_UINT
            && fd.vbank().get(inputs[slot])?.is_constant()
        {
            constant_types[slot] = Some(ty);
        }
    }
    let output = if shift {
        let out = operation.get_out()?;
        let ty = fd.vn_type_def_facing(out);
        if ty.get_size() != width || fd.vbank().get(out)?.get_size() != width
            || !ty.is_core_type()
            || !matches!(ty.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
        {
            return None;
        }
        if ty.get_metatype() != type_metatype::TYPE_INT || ty.is_char_print() { Some(ty) } else { None }
    } else {
        None
    };
    if casts.iter().all(Option::is_none) && output.is_none() {
        return None;
    }
    Some(SignedTextPlan { inputs, casts, constant_types, output })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::flags;

    #[test]
    fn core_unsigned_and_signed_text_require_signed_interpretation() {
        let mut ty = Datatype::new_with_align(1, -1, type_metatype::TYPE_INT);
        ty.flags |= flags::chartype | flags::coretype;
        assert_eq!(signed_operand_name(&ty), Some("i8"));
        ty.flags &= !flags::chartype;
        assert_eq!(signed_operand_name(&ty), None);
        ty.flags |= flags::chartype;
        ty.flags &= !flags::coretype;
        assert_eq!(signed_operand_name(&ty), None);
        ty.flags |= flags::coretype;
        ty.metatype = type_metatype::TYPE_UINT;
        assert_eq!(signed_operand_name(&ty), Some("i8"));
        ty.metatype = type_metatype::TYPE_PTR;
        assert_eq!(signed_operand_name(&ty), None);
    }
}
