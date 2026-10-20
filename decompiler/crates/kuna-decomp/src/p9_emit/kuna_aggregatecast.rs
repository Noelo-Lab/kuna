//! Equal-width aggregate CAST operations transfer object representations.
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, DatatypeKind};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(crate) struct Transfer {
    pub input: VarnodeId,
    pub source: Rc<Datatype>,
    pub target: Rc<Datatype>,
}

pub(crate) fn storage_transfer(fd: &Funcdata, decl_high_type: bool, op: OpId) -> Option<Transfer> {
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_CAST {
        return None;
    }
    let input = operation.get_in(0)?;
    let output = operation.get_out()?;
    let size = fd.vbank().get(input)?.get_size();
    if size <= 0 || fd.vbank().get(output)?.get_size() != size {
        return None;
    }
    let aggregate = |ty: &Datatype| {
        matches!(
            ty.get_metatype(),
            type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
        )
    };
    if !aggregate(&fd.vn_type_read_facing(input, op)) && !aggregate(&fd.vn_type_def_facing(output))
    {
        return None;
    }
    let target = fd
        .vbank()
        .get(output)
        .filter(|value| !value.is_implied())
        .and_then(|_| {
            crate::kuna_stackviews::access_type(fd, output, op, -1)
                .or_else(|| crate::kuna_bitcast::declared_member_type(fd, decl_high_type, output))
        })
        .unwrap_or_else(|| fd.vn_type_def_facing(output));
    let source = crate::kuna_stackviews::access_type(fd, input, op, operation.get_slot(input))
        .or_else(|| crate::kuna_bitcast::declared_member_type(fd, decl_high_type, input))
        .unwrap_or_else(|| fd.vn_type_read_facing(input, op));
    if fd.vbank().get(input)?.is_constant()
        && matches!(
            source.get_metatype(),
            type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
        )
    {
        return None;
    }
    let supported = |ty: &Datatype| {
        !ty.is_incomplete()
            && padding_free(ty, 0)
            && ty.get_size() == size
            && (aggregate(ty)
                || matches!(
                    ty.get_metatype(),
                    type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_FLOAT
                        | type_metatype::TYPE_UNKNOWN
                ))
    };
    if !(aggregate(&source) || aggregate(&target)) || !supported(&source) || !supported(&target) {
        return None;
    }
    Some(Transfer {
        input,
        source,
        target,
    })
}

/// Value initialization must not discard padding subsequently read as bits.
fn padding_free(ty: &Datatype, depth: usize) -> bool {
    if depth > 32 || ty.is_incomplete() || ty.get_size() <= 0 {
        return false;
    }
    match &ty.kind {
        DatatypeKind::Struct { field, bitfield } => {
            if !bitfield.is_empty() {
                return false;
            }
            let mut end = 0;
            for item in field {
                if item.offset != end || !padding_free(&item.field_type, depth + 1) {
                    return false;
                }
                let Some(next) = end.checked_add(item.field_type.get_size()) else {
                    return false;
                };
                end = next;
            }
            end == ty.get_size()
        }
        DatatypeKind::Union { field } => {
            !field.is_empty()
                && field.iter().all(|item| {
                    item.offset == 0
                        && item.field_type.get_size() == ty.get_size()
                        && padding_free(&item.field_type, depth + 1)
                })
        }
        DatatypeKind::Base | DatatypeKind::Unknown => {
            matches!(
                ty.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            ) && matches!(ty.get_size(), 1 | 2 | 4 | 8)
                || ty.get_metatype() == type_metatype::TYPE_FLOAT && matches!(ty.get_size(), 4 | 8)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::TypeField;

    fn record(size: i32, offsets: &[i32]) -> Datatype {
        let word = Rc::new(Datatype::new(4, type_metatype::TYPE_UINT));
        let mut ty = Datatype::new(size, type_metatype::TYPE_STRUCT);
        ty.kind = DatatypeKind::Struct {
            field: offsets
                .iter()
                .enumerate()
                .map(|(i, offset)| TypeField::new(i as i32, *offset, format!("f{i}"), word.clone()))
                .collect(),
            bitfield: vec![],
        };
        ty
    }

    #[test]
    fn object_representation_must_have_no_unrepresented_bytes() {
        assert!(padding_free(&record(8, &[0, 4]), 0));
        assert!(!padding_free(&record(12, &[0, 8]), 0));
        assert!(!padding_free(&record(12, &[0, 4]), 0));
        assert!(!padding_free(&record(8, &[0, 0]), 0));
        let mut union = record(8, &[0, 0]);
        let DatatypeKind::Struct { field, .. } = union.kind else {
            unreachable!()
        };
        union.kind = DatatypeKind::Union { field };
        assert!(!padding_free(&union, 0));
    }
}
