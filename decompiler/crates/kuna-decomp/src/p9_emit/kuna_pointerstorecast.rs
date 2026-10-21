//! Integer members require explicit conversions when an asserted pointer names the value.

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
    if operation.code() != OpCode::CPUI_COPY {
        return None;
    }
    let input = operation.get_in(0)?;
    let output = operation.get_out()?;
    let source = fd.vn_type_read_facing(input, op);
    if source.get_metatype() != type_metatype::TYPE_PTR {
        return None;
    }
    let value = fd.vbank().get(output)?;
    let target = printed().or_else(|| {
        let declared = super::kuna_bitcast::declared_member_type(fd, decl_high_type, output)?;
        let offset = value
            .get_high()
            .and_then(|id| fd.high_bank().get(id))
            .map(|high| high.kuna_symbol_offset().max(0))
            .unwrap_or(0);
        crate::kuna_stackviews::scalar_piece(
            fd.get_arch().types()?,
            declared,
            offset,
            value.get_size(),
        )
    })?;
    (matches!(
        target.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT
    ) && target.get_size() == source.get_size()
        && target.get_size() == value.get_size())
    .then_some(target)
}
