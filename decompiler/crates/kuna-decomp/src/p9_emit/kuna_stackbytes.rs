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
}

pub(crate) fn assignment(fd: &Funcdata, op: OpId) -> Option<Assignment> {
    let trace = std::env::var_os("KUNA_DEBUG_STACK_BYTES").is_some();
    macro_rules! reject {
        ($($arg:tt)*) => {{
            if trace {
                eprintln!("[stackbytes] {}", format!($($arg)*));
            }
            return None;
        }};
    }
    if !fd.get_arch().stack_views {
        reject!("stackviews off");
    }
    let Some(operation) = fd.obank().get(op) else {
        reject!("stale op");
    };
    if operation.is_marker() {
        reject!("marker");
    }
    let Some(output) = operation.get_out() else {
        reject!("no output");
    };
    let Some(value) = fd.vbank().get(output) else {
        reject!("missing output varnode");
    };
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        reject!("no stack space");
    };
    if value.get_space().get_index() != stack.get_index() {
        reject!("output is not stack space");
    }
    let size = value.get_size();
    if !(1..=8).contains(&size) {
        reject!("output size {size}");
    }
    let Some(scope) = fd.get_scope_local() else {
        reject!("no local scope");
    };
    let Some(info) =
        scope.query_container_for_link_width(value.get_addr(), size, &Address::new_invalid())
    else {
        reject!(
            "no stack container for offset {} size {size}",
            value.get_offset()
        );
    };
    if info.is_name_undefined {
        reject!("unnamed stack container");
    }
    let Some(mut bytes) = expression(fd, output, true, 32, op) else {
        reject!("unsupported source expression, output size {size}");
    };
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
    let stackview_scalar_copy = operation.code() == OpCode::CPUI_COPY
        && info
            .sym_type
            .as_ref()
            .is_some_and(|ty| ty.get_name().starts_with("stack_views_"));
    if matches!(size, 1 | 2 | 4 | 8) && !keep.iter().any(|keep| *keep) {
        if !matches!(
            fd.vn_type_def_facing(output).get_metatype(),
            type_metatype::TYPE_ARRAY | type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
        ) && !stackview_scalar_copy
        {
            reject!(
                "no preserved bytes and scalar output type {:?}; op={:?}; internal={}; container={}",
                fd.vn_type_def_facing(output).get_metatype(),
                operation.code(),
                operation.not_printed(),
                info.sym_type.as_ref().map_or("<none>", |ty| ty.get_name())
            );
        }
    }
    let mut ranges: Vec<Range> = Vec::new();
    for (index, byte) in bytes.into_iter().enumerate() {
        if keep[index] {
            continue;
        }
        // `keep` already removes each byte that is literally the same stack
        // storage byte. A different byte in the same generated backing still
        // has to be read and copied, even when both Varnodes share one HighVariable.
        if let Byte::Value { source, .. } = byte {
            let Some(source_value) = fd.vbank().get(source) else {
                reject!("missing source varnode");
            };
            let shares_stackview_high = source_value
                .get_high()
                .and_then(|high| fd.high_bank().get(high))
                .is_some_and(|high| high.kuna_link_symbol() == Some(info.symbol));
            let safe_load = safe_load_op(fd, source, op).is_some();
            if (source_value.is_implied() || shares_stackview_high)
                && !same_backing(fd, source, info.symbol)
                && !safe_load
            {
                reject!(
                    "source cannot be safely re-read after stackview merge (implied {}, shared high {}, size {}, readonly {}, volatile {}, def {:?})",
                    source_value.is_implied(),
                    shares_stackview_high,
                    source_value.get_size(),
                    source_value.is_read_only(),
                    source_value.is_volatile(),
                    source_value
                        .get_def()
                        .and_then(|def| fd.obank().get(def))
                        .map(|op| op.code())
                );
            }
            if !matches!(source_value.get_size(), 1 | 2 | 4 | 8) {
                reject!("source size {} unsupported", source_value.get_size());
            }
            let read_type = fd.vn_type_read_facing(source, op).get_metatype();
            if !matches!(
                read_type,
                type_metatype::TYPE_BOOL
                    | type_metatype::TYPE_INT
                    | type_metatype::TYPE_UINT
                    | type_metatype::TYPE_UNKNOWN
                    | type_metatype::TYPE_PTR
            ) {
                reject!("source read type {read_type:?} rejected");
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
        reject!("no output ranges after preserved bytes");
    }
    Some(Assignment {
        output,
        name: info.display_name,
        offset: info.sym_off,
        pointer_size: stack.get_addr_size() as i32,
        ranges,
    })
}

fn same_backing(fd: &Funcdata, value: VarnodeId, symbol: SymbolId) -> bool {
    let Some(value) = fd.vbank().get(value) else {
        return false;
    };
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    // HighVariable identity alone is not storage identity: a global or register
    // Varnode can be coalesced with the stackview backing after a merge.
    value.get_space().get_index() == stack.get_index()
        && value
            .get_high()
            .and_then(|high| fd.high_bank().get(high))
            .is_some_and(|high| high.kuna_link_symbol() == Some(symbol))
}

pub(crate) fn safe_load_op(fd: &Funcdata, value: VarnodeId, use_op: OpId) -> Option<OpId> {
    let mut current = value;
    for _ in 0..=32 {
        let current_value = fd.vbank().get(current)?;
        let def = current_value.get_def()?;
        let operation = fd.obank().get(def)?;
        match operation.code() {
            OpCode::CPUI_LOAD => {
                if current_value.is_volatile() {
                    return None;
                }
                if current_value.is_read_only() || reemit_load_is_safe(fd, def, use_op) {
                    return Some(def);
                }
                return None;
            }
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                let input = operation.get_in(0)?;
                if fd.vbank().get(input)?.get_size() != current_value.get_size() {
                    return None;
                }
                current = input;
            }
            _ => return None,
        }
    }
    None
}

fn reemit_load_is_safe(fd: &Funcdata, load: OpId, use_op: OpId) -> bool {
    let (Some(load_op), Some(use_operation)) = (fd.obank().get(load), fd.obank().get(use_op))
    else {
        return false;
    };
    let (Some(block), Some(use_block)) = (load_op.get_parent(), use_operation.get_parent()) else {
        return false;
    };
    if block != use_block {
        return false;
    }
    let mut next = fd.bb_op_next(load);
    let mut steps = 0;
    while let Some(op) = next {
        if op == use_op {
            return true;
        }
        let Some(operation) = fd.obank().get(op) else {
            return false;
        };
        if operation.is_call()
            || matches!(
                operation.code(),
                OpCode::CPUI_STORE
                    | OpCode::CPUI_INDIRECT
                    | OpCode::CPUI_CALLOTHER
            )
        {
            return false;
        }
        steps += 1;
        if steps > 4096 {
            return false;
        }
        next = fd.bb_op_next(op);
    }
    false
}

fn expression(
    fd: &Funcdata,
    id: VarnodeId,
    top: bool,
    depth: u8,
    use_op: OpId,
) -> Option<Vec<Byte>> {
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
                    return expression(fd, input, false, depth, use_op);
                }
            }
            OpCode::CPUI_PIECE => {
                let mut low = expression(fd, op.get_in(1)?, false, depth, use_op)?;
                low.extend(expression(fd, input, false, depth, use_op)?);
                return (low.len() == size as usize).then_some(low);
            }
            OpCode::CPUI_SUBPIECE => {
                let offset: usize = literal(1)?.try_into().ok()?;
                let bytes = expression(fd, input, false, depth, use_op)?;
                return bytes
                    .get(offset..offset.checked_add(size as usize)?)
                    .map(|bytes| bytes.to_vec());
            }
            OpCode::CPUI_INT_ZEXT => {
                let mut bytes = expression(fd, input, false, depth, use_op)?;
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
                let bytes = expression(fd, input, false, depth, use_op)?;
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
            OpCode::CPUI_LOAD if safe_load_op(fd, id, use_op).is_some() => {
                return Some(
                    (0..size)
                        .map(|byte| Byte::Value { source: id, byte })
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
