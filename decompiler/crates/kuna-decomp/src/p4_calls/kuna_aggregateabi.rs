//! Direct-field ABI classification and output joins that retain struct padding.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpaceManager;
use kuna_num::pcoderaw::VarnodeData;

use super::modelrules::Primitive;
use crate::dtype::{type_class, type_metatype, Datatype, DatatypeKind};
use crate::fspec::{AssignActionResponse, ParamListStandard, ParameterPieces};

pub(super) struct PrimitiveField {
    pub dt: Rc<Datatype>,
    pub offset: i32,
    pub alignment: i32,
    pub bitfield: bool,
}

/// Include the occupied bytes of bitfields in ABI primitive classification.
/// Overlapping containers contribute once; integer chunks never cross their
/// natural boundary. Bitfield positions do not impose ordinary-member alignment.
pub(super) fn primitive_fields(dt: &Datatype, offset: i32) -> Option<Vec<PrimitiveField>> {
    let DatatypeKind::Struct { field, bitfield } = &dt.kind else {
        return None;
    };
    let mut fields: Vec<_> = field
        .iter()
        .map(|f| PrimitiveField {
            dt: f.field_type.clone(),
            offset: f.offset,
            alignment: f.field_type.get_alignment(),
            bitfield: false,
        })
        .collect();
    let mut ranges = Vec::new();
    for f in bitfield.iter().filter(|f| f.num_bits > 0) {
        let mut bits = f.bits();
        bits.minimize_container();
        let end = bits.byte_offset.checked_add(bits.byte_size)?;
        if bits.byte_offset < 0 || end > dt.get_size() || bits.byte_size <= 0 {
            return None;
        }
        ranges.push((bits.byte_offset, end, f.field_type.get_alignment()));
    }
    ranges.sort_unstable();
    let mut merged: Vec<(i32, i32, i32)> = Vec::new();
    for (start, end, align) in ranges {
        if let Some(last) = merged.last_mut().filter(|r| start <= r.1) {
            if start == last.0 {
                last.2 = last.2.max(align);
            }
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end, align));
        }
    }
    for (mut start, end, mut align) in merged {
        while start < end {
            let mut size = 8;
            while size > end - start || (start + offset) % size != 0 {
                size /= 2;
            }
            fields.push(PrimitiveField {
                dt: Rc::new(Datatype::new_with_align(size, 1, type_metatype::TYPE_UINT)),
                offset: start,
                alignment: align,
                bitfield: true,
            });
            start += size;
            align = 1;
        }
    }
    fields.sort_by_key(|f| f.offset);
    Some(fields)
}

pub(super) fn direct_fields(dt: &Rc<Datatype>) -> Option<Vec<Primitive>> {
    let members = primitive_fields(dt, 0)?;
    if members.is_empty() {
        return None;
    }
    let mut end = 0;
    let mut fields = Vec::new();
    for field in members {
        if field.offset < end || field.dt.get_size() <= 0 {
            return None;
        }
        end = field.offset.checked_add(field.dt.get_size())?;
        if end > dt.get_size() {
            return None;
        }
        fields.push(Primitive::new(field.dt, field.offset));
    }
    Some(fields)
}

pub(super) fn float_fields(dt: &Rc<Datatype>, max: i32) -> bool {
    !dt.has_bitfields()
        && direct_fields(dt).is_some_and(|fields| {
            fields.len() <= max as usize
                && fields
                    .iter()
                    .all(|field| field.dt.get_metatype() == type_metatype::TYPE_FLOAT)
        })
}

fn physical_pieces(
    addr: &Address,
    size: u32,
    manager: &AddrSpaceManager,
    big_endian: bool,
) -> KunaResult<Vec<VarnodeData>> {
    if addr
        .get_space()
        .zip(manager.get_join_space())
        .is_some_and(|(a, b)| Rc::ptr_eq(a, b))
    {
        let join = manager.find_join(addr.get_offset())?;
        let mut pieces: Vec<_> = (0..join.num_pieces())
            .map(|i| {
                let piece = join.get_piece(i);
                VarnodeData {
                    space: piece.space.clone(),
                    offset: piece.offset,
                    size: piece.size,
                }
            })
            .collect();
        if !big_endian {
            pieces.reverse();
        }
        Ok(pieces)
    } else {
        Ok(vec![VarnodeData {
            space: addr.get_space().cloned(),
            offset: addr.get_offset(),
            size,
        }])
    }
}

fn append_padding(
    pieces: &mut Vec<VarnodeData>,
    manager: &AddrSpaceManager,
    size: u32,
) -> KunaResult<()> {
    if size > 0 {
        let space = manager
            .get_constant_space()
            .ok_or_else(|| KunaError::lowlevel("ABI padding requires a constant space"))?;
        pieces.push(VarnodeData {
            space: Some(space.clone()),
            offset: 0,
            size,
        });
    }
    Ok(())
}

/// Assign each direct field independently, keeping its byte offset in the struct.
/// ABI padding has no physical value. Zero pieces retain its logical offsets;
/// heritage discards writes to these pieces instead of creating register uses.
pub(super) fn assign_fields(
    dt: &Rc<Datatype>,
    class: type_class,
    status: &mut [i32],
    result: &mut ParameterPieces,
    resource: &ParamListStandard,
    manager: &AddrSpaceManager,
) -> KunaResult<AssignActionResponse> {
    use AssignActionResponse::{fail, success};
    let Some(fields) = direct_fields(dt) else {
        return Ok(fail);
    };
    let big_endian = resource.is_big_endian();
    let mut pending = status.to_vec();
    let mut field_pieces = Vec::new();
    for field in &fields {
        let mut param = ParameterPieces::default();
        if resource.assign_address_fallback(
            class,
            &field.dt,
            true,
            &mut pending,
            &mut param,
            manager,
        )? == fail
        {
            return Ok(fail);
        }
        field_pieces.push(physical_pieces(
            &param.addr,
            field.dt.get_size() as u32,
            manager,
            big_endian,
        )?);
    }
    let mut pieces = Vec::new();
    let mut end = 0;
    for (field, storage) in fields.iter().zip(field_pieces) {
        append_padding(&mut pieces, manager, (field.offset - end) as u32)?;
        pieces.extend(storage);
        end = field.offset + field.dt.get_size();
    }
    append_padding(&mut pieces, manager, (dt.get_size() - end) as u32)?;
    result.assign_address_from_pieces(&mut pieces, big_endian, manager)?;
    result.flags = 0;
    result.type_ = Some(dt.clone());
    status.copy_from_slice(&pending);
    Ok(success)
}
