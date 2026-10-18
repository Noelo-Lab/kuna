//! Emit changed bytes without reading a shared backing's preserved bytes.

use crate::context::{OpId, VarnodeId};
use crate::database::SymbolId;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

#[derive(Clone, Copy)]
pub(crate) enum Byte {
    Constant(u8),
    Value { source: VarnodeId, byte: i32 },
}

pub(crate) struct Range {
    pub offset: i32,
    pub bytes: Vec<Byte>,
}

pub(crate) struct Assignment {
    pub output: VarnodeId,
    pub name: String,
    pub offset: i32,
    pub pointer_size: i32,
    pub ranges: Vec<Range>,
    pub reemit_hidden_constant_copy: bool,
}

pub(crate) fn assignment(fd: &Funcdata, op: OpId) -> Option<Assignment> {
    if !crate::kuna_stackviews::enabled(fd) {
        return None;
    }
    let operation = fd.obank().get(op)?;
    if operation.is_marker() {
        return None;
    }
    let output = operation.get_out()?;
    let value = fd.vbank().get(output)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    if value.get_space().get_index() != stack.get_index() {
        return None;
    }
    let size = value.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    let info = fd.get_scope_local()?.query_container_for_link_width(
        value.get_addr(),
        size,
        &Address::new_invalid(),
    )?;
    if info.is_name_undefined {
        return None;
    }
    let mut bytes = expression(fd, output, true, 32)?;
    let scalar_type = fd.vn_type_def_facing(output).get_metatype();
    // View merging can hide a literal integer write to its generated backing.
    // Keep this exception to complete constant-only copies; loads, pointers and
    // other expressions must still use the ordinary statement path.
    let hidden_constant_copy = (operation.not_printed()
        || fd
            .vbank()
            .get(output)
            .is_some_and(|value| value.is_implied()))
        && operation.code() == OpCode::CPUI_COPY
        && matches!(size, 1 | 2 | 4 | 8)
        && matches!(
            scalar_type,
            type_metatype::TYPE_BOOL | type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        )
        && info
            .sym_type
            .as_ref()
            .is_some_and(|ty| ty.get_name().starts_with("stack_views_"))
        && operation
            .get_in(0)
            .is_some_and(|source| !contains_pointer_constant(fd, source, op, 32))
        && bytes.len() == size as usize
        && bytes.iter().all(|byte| matches!(byte, Byte::Constant(_)));
    if stack.is_big_endian() {
        bytes.reverse();
    }
    let preserved = |index: usize, byte: Byte| {
        let Byte::Value { source, byte } = byte else {
            return false;
        };
        let Some(source_value) = fd.vbank().get(source) else {
            return false;
        };
        let displacement = if stack.is_big_endian() {
            source_value.get_size() - 1 - byte
        } else {
            byte
        };
        source_value.get_space().get_index() == stack.get_index()
            && source_value.get_offset().wrapping_add(displacement as u64)
                == value.get_offset().wrapping_add(index as u64)
            && same_backing(fd, source, info.symbol)
    };
    let keep: Vec<_> = bytes
        .iter()
        .enumerate()
        .map(|(index, byte)| preserved(index, *byte))
        .collect();
    if matches!(size, 1 | 2 | 4 | 8) && !keep.iter().any(|keep| *keep) {
        if !matches!(
            fd.vn_type_def_facing(output).get_metatype(),
            type_metatype::TYPE_ARRAY | type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
        ) && !hidden_constant_copy
        {
            return None;
        }
    }
    let mut ranges: Vec<Range> = Vec::new();
    for (index, byte) in bytes.into_iter().enumerate() {
        if keep[index] {
            continue;
        }
        if let Byte::Value { source, .. } = byte {
            let source_value = fd.vbank().get(source)?;
            if source_value.is_implied()
                || same_backing(fd, source, info.symbol)
                || !matches!(source_value.get_size(), 1 | 2 | 4 | 8)
                || !matches!(
                    fd.vn_type_read_facing(source, op).get_metatype(),
                    type_metatype::TYPE_BOOL
                        | type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_UNKNOWN
                        | type_metatype::TYPE_PTR
                )
            {
                return None;
            }
        }
        let index = index as i32;
        if let Some(last) = ranges
            .last_mut()
            .filter(|range| range.offset + range.bytes.len() as i32 == index)
        {
            last.bytes.push(byte);
        } else {
            ranges.push(Range {
                offset: index,
                bytes: vec![byte],
            });
        }
    }
    if ranges.is_empty() {
        return None;
    }
    Some(Assignment {
        output,
        name: info.display_name,
        offset: info.sym_off,
        pointer_size: stack.get_addr_size() as i32,
        ranges,
        reemit_hidden_constant_copy: hidden_constant_copy,
    })
}

fn same_backing(fd: &Funcdata, value: VarnodeId, symbol: SymbolId) -> bool {
    let Some(value) = fd.vbank().get(value) else {
        return false;
    };
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    // HighVariable identity alone is not storage identity: another space can
    // share a HighVariable with the generated frame backing after a merge.
    value.get_space().get_index() == stack.get_index()
        && value
            .get_high()
            .and_then(|high| fd.high_bank().get(high))
            .is_some_and(|high| high.kuna_link_symbol() == Some(symbol))
}

fn contains_pointer_constant(fd: &Funcdata, id: VarnodeId, use_op: OpId, depth: u8) -> bool {
    let Some(value) = fd.vbank().get(id) else {
        return true;
    };
    if value.is_constant() {
        let pointer_type = |metatype| {
            matches!(
                metatype,
                type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL | type_metatype::TYPE_CODE
            )
        };
        return pointer_type(value.get_type().get_metatype())
            || pointer_type(fd.vn_type_read_facing(id, use_op).get_metatype());
    }
    let Some(depth) = depth.checked_sub(1) else {
        return true;
    };
    let Some(definition) = value.get_def() else {
        return false;
    };
    let Some(operation) = fd.obank().get(definition) else {
        return true;
    };
    (0..operation.num_input()).any(|slot| {
        operation
            .get_in(slot)
            .is_some_and(|input| contains_pointer_constant(fd, input, definition, depth))
    })
}

fn expression(fd: &Funcdata, id: VarnodeId, top: bool, depth: u8) -> Option<Vec<Byte>> {
    let depth = depth.checked_sub(1)?;
    let value = fd.vbank().get(id)?;
    let size = value.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    if value.is_constant() {
        return Some(
            (0..size)
                .map(|byte| Byte::Constant((value.get_offset() >> (byte * 8)) as u8))
                .collect(),
        );
    }
    if top || value.is_implied() {
        let op = fd.obank().get(value.get_def()?)?;
        let input = op.get_in(0)?;
        let literal = |slot| {
            let value = fd.vbank().get(op.get_in(slot)?)?;
            value.is_constant().then_some(value.get_offset())
        };
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                if fd.vbank().get(input)?.get_size() == size {
                    return expression(fd, input, false, depth);
                }
            }
            OpCode::CPUI_PIECE => {
                let mut low = expression(fd, op.get_in(1)?, false, depth)?;
                low.extend(expression(fd, input, false, depth)?);
                return (low.len() == size as usize).then_some(low);
            }
            OpCode::CPUI_SUBPIECE => {
                let offset: usize = literal(1)?.try_into().ok()?;
                let bytes = expression(fd, input, false, depth)?;
                return bytes
                    .get(offset..offset.checked_add(size as usize)?)
                    .map(|bytes| bytes.to_vec());
            }
            OpCode::CPUI_INT_ZEXT => {
                let mut bytes = expression(fd, input, false, depth)?;
                if bytes.len() > size as usize {
                    return None;
                }
                bytes.resize(size as usize, Byte::Constant(0));
                return Some(bytes);
            }
            OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT => {
                let shift = literal(1)?;
                if shift % 8 != 0 {
                    return None;
                }
                let bytes = expression(fd, input, false, depth)?;
                if bytes.len() != size as usize {
                    return None;
                }
                let shift: usize = (shift / 8).try_into().ok()?;
                return Some(
                    (0..size as usize)
                        .map(|byte| {
                            let source = if op.code() == OpCode::CPUI_INT_LEFT {
                                byte.checked_sub(shift)
                            } else {
                                byte.checked_add(shift)
                            };
                            source
                                .and_then(|index| bytes.get(index).copied())
                                .unwrap_or(Byte::Constant(0))
                        })
                        .collect(),
                );
            }
            _ => (),
        }
        return None;
    }
    Some(
        (0..size)
            .map(|byte| Byte::Value { source: id, byte })
            .collect(),
    )
}
