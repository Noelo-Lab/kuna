//! Preserve integer SUBPIECE width and bit position when projecting an implied struct field.

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use std::rc::Rc;

pub(crate) struct Extraction {
    pub target: Rc<Datatype>,
    pub shift: u64,
}

pub(crate) fn plan(
    fd: &Funcdata,
    op: OpId,
    whole: &Datatype,
    index: i32,
    offset: i64,
) -> Option<Extraction> {
    if whole.is_incomplete() || whole.has_bitfields() || crate::kuna_typequal::is_volatile(whole) {
        return None;
    }
    let field = &whole.get_field(index)?.field_type;
    let operation = fd.obank().get(op)?;
    let input = fd.vbank().get(operation.get_in(0)?)?;
    let output = operation.get_out()?;
    let size = fd.vbank().get(output)?.get_size();
    let target = fd.vn_type_def_facing(output);
    if !matches!(field.get_size(), 1 | 2 | 4 | 8)
        || !matches!(size, 1 | 2 | 4 | 8)
        || target.get_size() != size
        || input.get_size() != whole.get_size()
        || field.is_enum_type()
        || target.is_enum_type()
        || crate::kuna_typequal::is_volatile(field)
        || !matches!(
            field.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        )
        || !matches!(
            target.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        )
        || offset < 0
        || offset.checked_add(i64::from(size))? > i64::from(field.get_size())
    {
        return None;
    }
    let low_byte = if input.get_addr().get_space()?.is_big_endian() {
        i64::from(field.get_size() - size) - offset
    } else {
        offset
    };
    let shift = u64::try_from(low_byte).ok()?.checked_mul(8)?;
    if shift == 0 && matches!(target.compare(field, 10), Ok(0)) {
        return None;
    }
    Some(Extraction { target, shift })
}

#[cfg(test)]
mod tests;
