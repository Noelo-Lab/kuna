//! Render a scalar's preserved upper bytes and replaced lower bytes as C masks.

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) struct PartialConcat {
    pub source: VarnodeId,
    pub low: VarnodeId,
    pub size: i32,
    pub low_mask: u64,
    pub high_mask: u64,
}

fn scalar(data: &Funcdata, vn: VarnodeId, reader: OpId) -> bool {
    let Some(vn) = data.vbank().get(vn) else {
        return false;
    };
    let ty = vn.get_type_read_facing(reader);
    !ty.is_enum_type()
        && matches!(
            ty.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
}

/// Match PIECE(SUBPIECE(source, low_size), low) without bypassing a named value.
pub(crate) fn preserved_prefix(data: &Funcdata, op: OpId) -> Option<PartialConcat> {
    let piece = data.obank().get(op)?;
    if piece.code() != OpCode::CPUI_PIECE || piece.num_input() != 2 {
        return None;
    }
    let output = piece.get_out()?;
    let size = data.vbank().get(output)?.get_size();
    if !matches!(size, 2 | 4 | 8) || !scalar(data, output, op) {
        return None;
    }
    let low = piece.get_in(1)?;
    let low_size = data.vbank().get(low)?.get_size();
    if !matches!(low_size, 1 | 2 | 4) || low_size >= size || !scalar(data, low, op) {
        return None;
    }
    let mut high = piece.get_in(0)?;
    let mut reader = op;
    for _ in 0..4 {
        let vn = data.vbank().get(high)?;
        if !vn.is_implied() || !scalar(data, high, reader) || vn.get_size() != size - low_size {
            return None;
        }
        let def = vn.get_def()?;
        let producer = data.obank().get(def)?;
        if matches!(producer.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            high = producer.get_in(0)?;
            reader = def;
            continue;
        }
        if producer.code() != OpCode::CPUI_SUBPIECE {
            return None;
        }
        let offset = data.vbank().get(producer.get_in(1)?)?;
        let source = producer.get_in(0)?;
        if !offset.is_constant()
            || data.vbank().get(source)?.get_size() != size
            || !scalar(data, source, def)
        {
            return None;
        }
        let mut source = if offset.get_offset() == low_size as u64 {
            source
        } else if offset.get_offset() == 0 {
            let shifted = data.vbank().get(source)?;
            if !shifted.is_implied() {
                return None;
            }
            let shift_id = shifted.get_def()?;
            let shift = data.obank().get(shift_id)?;
            if !matches!(
                shift.code(),
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT
            ) {
                return None;
            }
            let amount = data.vbank().get(shift.get_in(1)?)?;
            let original = shift.get_in(0)?;
            if !amount.is_constant()
                || amount.get_offset() != (low_size * 8) as u64
                || data.vbank().get(original)?.get_size() != size
                || !scalar(data, original, shift_id)
            {
                return None;
            }
            original
        } else {
            return None;
        };
        for _ in 0..4 {
            let vn = data.vbank().get(source)?;
            if !vn.is_implied() {
                break;
            }
            let Some(cast_id) = vn.get_def() else { break };
            let cast = data.obank().get(cast_id)?;
            if !matches!(cast.code(), OpCode::CPUI_CAST | OpCode::CPUI_COPY) {
                break;
            }
            let input = cast.get_in(0)?;
            if data.vbank().get(input)?.get_size() != size || !scalar(data, input, cast_id) {
                break;
            }
            source = input;
        }
        let low_mask = (1u64 << (low_size * 8)) - 1;
        let full_mask = u64::MAX >> ((8 - size) * 8);
        return Some(PartialConcat {
            source,
            low,
            size,
            low_mask,
            high_mask: full_mask ^ low_mask,
        });
    }
    None
}
