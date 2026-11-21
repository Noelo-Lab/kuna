//! (kuna `oslog`) Apple unified-logging call sites print their arguments.
//!
//! The `os_log*` macros compile to a call into libsystem_trace with the
//! arguments packed into a stack buffer the caller fills right before the call:
//! byte 0 summary flags, byte 1 the argument count, then per argument a
//! descriptor byte (type in the high nibble, privacy in the low one), a size
//! byte and the value bytes. Read raw, the call is
//! `_os_log_impl(0x100000000,log,0x10,"fmt",&v1,8)` after `v1 = 0x4000100;
//! v8 = v2;`.
//!
//! When on, a call whose buffer is decoded exactly prints the values in the
//! buffer's place, `_os_log_impl(&__mh_execute_header,log,0x10,"fmt",v2)`, and
//! the statements that only fill the buffer are not printed. The first
//! argument, the image's `__dso_handle`, prints as the Mach-O header symbol at
//! that address on these calls and on the kernel's `_os_log_internal`.
//!
//! The decode is refused unless the size argument is a constant, the buffer is
//! a constant stack offset, every buffer byte is set by a value the call's
//! INDIRECT effects show, every header, descriptor and size byte is a constant,
//! each non-constant value fills exactly its own slot, and the items end at the
//! buffer's size.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_BUFFER: u64 = 1024;
const MAX_ITEMS: usize = 48;
const MAX_WALK: usize = 256;
const MAX_BLOCK_OPS: usize = 1 << 16;

/// One decoded buffer item.
pub(crate) enum Item {
    Value(VarnodeId),
    Constant { value: u64, size: i32 },
}

/// What the printer does at one call.
pub(crate) struct Plan {
    /// `&<name>` for the dso argument.
    pub dso: Option<String>,
    /// The fixed inputs printed before the items (`in[1..=fixed]`).
    pub fixed: i32,
    /// The decoded buffer items, `None` to print every input as is.
    pub items: Option<Vec<Item>>,
}

struct Shape {
    fixed: i32,
    buffer: Option<(i32, i32)>,
}

fn shape(name: &str) -> Option<Shape> {
    match name.trim_start_matches('_') {
        "os_log_impl" | "os_log_error_impl" | "os_log_fault_impl" | "os_log_debug_impl" => {
            Some(Shape {
                fixed: 4,
                buffer: Some((5, 6)),
            })
        }
        "os_signpost_emit_with_name_impl" => Some(Shape {
            fixed: 6,
            buffer: Some((7, 8)),
        }),
        "os_log_internal" => Some(Shape {
            fixed: 4,
            buffer: None,
        }),
        _ => None,
    }
}

fn callee_shape(fd: &Funcdata, call: OpId) -> Option<Shape> {
    let op = fd.obank().get(call)?;
    if op.code() != OpCode::CPUI_CALL {
        return None;
    }
    let index = fd.get_call_specs_index(call)?;
    shape(fd.get_call_specs(index).get_name())
}

/// The decode at `call`, or `None` when the call is not an os_log entry point or
/// nothing about it can be shown better.
pub(crate) fn recover(fd: &Funcdata, arch: &Architecture, call: OpId) -> Option<Plan> {
    if !arch.os_log_args {
        return None;
    }
    let shape = callee_shape(fd, call)?;
    let op = fd.obank().get(call)?;
    if op.num_input() <= shape.fixed {
        return None;
    }
    let dso = header_name(fd, arch, op.get_in(1)?);
    let items = match shape.buffer {
        Some((buffer, size)) if op.num_input() == size + 1 => {
            decode(fd, call, op.get_in(buffer)?, op.get_in(size)?).map(|(items, _)| items)
        }
        _ => None,
    };
    if dso.is_none() && items.is_none() {
        return None;
    }
    Some(Plan {
        dso,
        fixed: shape.fixed,
        items,
    })
}

/// The buffer-filling statements of every decoded call in `fd`.
pub(crate) fn hidden_ops(fd: &Funcdata, arch: &Architecture) -> BTreeSet<OpId> {
    let mut hidden = BTreeSet::new();
    if !arch.os_log_args {
        return hidden;
    }
    for call in fd.obank().iter_alive() {
        let Some(shape) = callee_shape(fd, call) else {
            continue;
        };
        let Some((buffer, size)) = shape.buffer else {
            continue;
        };
        let Some(op) = fd.obank().get(call) else {
            continue;
        };
        if op.num_input() != size + 1 {
            continue;
        }
        let (Some(buffer), Some(size)) = (op.get_in(buffer), op.get_in(size)) else {
            continue;
        };
        if let Some((_, fills)) = decode(fd, call, buffer, size) {
            hidden.extend(fills);
        }
    }
    hidden
}

/// The name of the Mach-O header at the constant `id`: the image's own symbol
/// there (`__mh_*`, `__dso_handle`), else the one the linker defines for the
/// header's file type, when the bytes there are a 64-bit Mach-O header.
fn header_name(fd: &Funcdata, arch: &Architecture, id: VarnodeId) -> Option<String> {
    let value = constant(fd, id)?;
    let space = fd.get_arch().manage().get_default_data_space()?;
    let address = Address::new(Rc::clone(space), value);
    if let Some(global) = arch.symboltab.get_global_scope() {
        if let Some(entry) = arch.symboltab.find_addr(global, &address, &Address::new_invalid()) {
            let symbol = arch.symboltab.entry(global, entry).symbol;
            let name = arch.symboltab.symbol(symbol).get_name();
            if name.starts_with("__mh_") || name == "__dso_handle" {
                return Some(name.to_string());
            }
        }
    }
    let mut header = [0u8; 16];
    fd.get_arch().loader_fill(&mut header, &address).ok()?;
    if header[..4] != [0xcf, 0xfa, 0xed, 0xfe] {
        return None;
    }
    let name = match u32::from_le_bytes(header[12..16].try_into().ok()?) {
        2 => "__mh_execute_header",
        6 => "__mh_dylib_header",
        7 => "__mh_dylinker_header",
        8 => "__mh_bundle_header",
        _ => "__dso_handle",
    };
    Some(name.to_string())
}

fn constant(fd: &Funcdata, mut id: VarnodeId) -> Option<u64> {
    for _ in 0..8 {
        let value = fd.vbank().get(id)?;
        if value.is_constant() {
            return Some(value.get_offset());
        }
        let op = fd.obank().get(value.get_def()?)?;
        match op.code() {
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT => id = op.get_in(0)?,
            _ => return None,
        }
    }
    None
}

/// The bytes of `id` that are constant, least significant first: through
/// copies, zero extension, PIECE, and AND/OR with a constant mask.
fn known_bytes(fd: &Funcdata, id: VarnodeId, depth: usize) -> Vec<Option<u8>> {
    let Some(value) = fd.vbank().get(id) else {
        return Vec::new();
    };
    let size = (value.get_size().max(0) as usize).min(8);
    let unknown = vec![None; size];
    if value.is_constant() {
        return (0..size)
            .map(|i| Some((value.get_offset() >> (8 * i)) as u8))
            .collect();
    }
    if depth >= 8 {
        return unknown;
    }
    let Some(op) = value.get_def().and_then(|def| fd.obank().get(def)) else {
        return unknown;
    };
    let input = |slot: i32| op.get_in(slot).map(|vn| known_bytes(fd, vn, depth + 1));
    let mask = |slot: i32| {
        op.get_in(slot)
            .and_then(|vn| fd.vbank().get(vn))
            .filter(|vn| vn.is_constant())
            .map(|vn| vn.get_offset())
    };
    let mut out = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => input(0).unwrap_or_default(),
        OpCode::CPUI_INT_ZEXT => {
            let mut low = input(0).unwrap_or_default();
            low.resize(size, Some(0));
            low
        }
        OpCode::CPUI_PIECE => {
            let mut low = input(1).unwrap_or_default();
            let low_size = op
                .get_in(1)
                .and_then(|vn| fd.vbank().get(vn))
                .map(|vn| vn.get_size().max(0) as usize)
                .unwrap_or(0);
            low.resize(low_size, None);
            low.extend(input(0).unwrap_or_default());
            low
        }
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR => {
            let Some(constant) = mask(1) else {
                return unknown;
            };
            let mut other = input(0).unwrap_or_default();
            other.resize(size, None);
            let and = op.code() == OpCode::CPUI_INT_AND;
            (0..size)
                .map(|i| {
                    let c = (constant >> (8 * i)) as u8;
                    match (and, c, other[i]) {
                        (true, 0, _) => Some(0),
                        (false, 0xff, _) => Some(0xff),
                        (true, _, Some(byte)) => Some(byte & c),
                        (false, _, Some(byte)) => Some(byte | c),
                        _ => None,
                    }
                })
                .collect()
        }
        _ => unknown,
    };
    out.resize(size, None);
    out
}

struct Byte {
    cell: usize,
    known: Option<u8>,
}

struct Fill {
    op: OpId,
    root: VarnodeId,
    base: u64,
    hideable: bool,
}

struct Cell {
    start: i64,
    size: u64,
    input: VarnodeId,
    fill: Option<Fill>,
}

/// Decode the buffer `call` reads: the items and the statements that only fill
/// it.
fn decode(
    fd: &Funcdata,
    call: OpId,
    buffer: VarnodeId,
    size: VarnodeId,
) -> Option<(Vec<Item>, Vec<OpId>)> {
    let length = constant(fd, size)?;
    if !(2..=MAX_BUFFER).contains(&length) {
        return None;
    }
    let base = super::kuna_rustfmtargs::stack_offset(fd, buffer)?;
    let end = base.checked_add(length as i64)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    let big_endian = stack.is_big_endian();
    let mut cells: Vec<Cell> = Vec::new();
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id)?;
        if !effect_of(fd, id, call) {
            continue;
        }
        let output = fd.vbank().get(op.get_out()?)?;
        if output.get_space().get_index() != stack.get_index() {
            continue;
        }
        let start = output.get_offset() as i64;
        let stop = start.checked_add(output.get_size() as i64)?;
        if start >= end || stop <= base {
            continue;
        }
        if cells.len() >= MAX_WALK {
            return None;
        }
        let input = op.get_in(0)?;
        let inside = start >= base && stop <= end;
        cells.push(Cell {
            start: start.checked_sub(base)?,
            size: output.get_size() as u64,
            input,
            fill: fill_op(fd, input, op.get_out()?, inside),
        });
    }
    let mut bytes: Vec<Option<Byte>> = (0..length).map(|_| None).collect();
    for (index, cell) in cells.iter().enumerate() {
        let known = known_bytes(fd, cell.input, 0);
        for offset in 0..cell.size {
            let at = cell.start.checked_add(offset as i64)?;
            if at < 0 || at >= length as i64 {
                continue;
            }
            let slot = bytes.get_mut(at as usize)?;
            if slot.is_some() {
                return None;
            }
            let significance = if big_endian {
                cell.size - 1 - offset
            } else {
                offset
            };
            *slot = Some(Byte {
                cell: index,
                known: known.get(significance as usize).copied().flatten(),
            });
        }
    }
    if bytes.iter().any(|byte| byte.is_none()) {
        return None;
    }
    let constant_byte = |at: u64| bytes.get(at as usize)?.as_ref()?.known;
    let count = constant_byte(1)? as usize;
    constant_byte(0)?;
    if count > MAX_ITEMS {
        return None;
    }
    let mut items = Vec::with_capacity(count);
    let mut fills = Vec::new();
    let mut position = 2u64;
    for _ in 0..count {
        let descriptor = constant_byte(position)?;
        let width = constant_byte(position + 1)? as u64;
        if descriptor >> 4 > 7 || !matches!(width, 1 | 2 | 4 | 8) {
            return None;
        }
        let start = position + 2;
        let stop = start.checked_add(width)?;
        if stop > length {
            return None;
        }
        let mut parts: Vec<usize> = Vec::new();
        let mut constants = 0;
        for at in start..stop {
            let byte = bytes.get(at as usize)?.as_ref()?;
            if byte.known.is_some() {
                constants += 1;
            }
            if parts.last() != Some(&byte.cell) {
                parts.push(byte.cell);
            }
        }
        if constants == width {
            let mut value = 0u64;
            for offset in 0..width {
                let byte = constant_byte(start + offset)? as u64;
                let shift = if big_endian {
                    width - 1 - offset
                } else {
                    offset
                };
                value |= byte << (shift * 8);
            }
            items.push(Item::Constant {
                value,
                size: width as i32,
            });
        } else if parts.iter().any(|&index| {
            let cell = &cells[index];
            cell.start < start as i64 || cell.start + cell.size as i64 > stop as i64
        }) {
            return None;
        } else {
            items.push(value_item(
                fd, call, &cells, &parts, start, width, big_endian, &mut fills,
            )?);
        }
        position = stop;
    }
    if position != length {
        return None;
    }
    for cell in &cells {
        if let Some(fill) = cell.fill.as_ref().filter(|fill| fill.hideable) {
            if constant(fd, cell.input).is_some() {
                fills.push(fill.op);
            }
        }
    }
    Some((items, fills))
}

/// The value one item's `parts` (cells inside its slot, in order) hold: the
/// source they were all copied from when it still holds that value at `call`,
/// else the single cell's own variable.
#[allow(clippy::too_many_arguments)]
fn value_item(
    fd: &Funcdata,
    call: OpId,
    cells: &[Cell],
    parts: &[usize],
    start: u64,
    width: u64,
    big_endian: bool,
    fills: &mut Vec<OpId>,
) -> Option<Item> {
    let root = cells[*parts.first()?].fill.as_ref().map(|fill| fill.root);
    let whole = root.is_some_and(|root| {
        fd.vbank()
            .get(root)
            .is_some_and(|value| value.get_size() as u64 == width)
            && (parts.len() == 1 || !big_endian)
            && parts.iter().all(|&index| {
                let cell = &cells[index];
                cell.fill.as_ref().is_some_and(|fill| {
                    fill.root == root
                        && cell.start.checked_sub(start as i64) == Some(fill.base as i64)
                        && reaches_call(fd, fill.op, fill.root, call)
                })
            })
    });
    if whole {
        let hideable = parts
            .iter()
            .all(|&index| cells[index].fill.as_ref().is_some_and(|fill| fill.hideable));
        if hideable {
            fills.extend(
                parts
                    .iter()
                    .filter_map(|&index| cells[index].fill.as_ref().map(|fill| fill.op)),
            );
        }
        return Some(Item::Value(root?));
    }
    match parts {
        [index] if cells[*index].start == start as i64 && cells[*index].size == width => {
            Some(Item::Value(cells[*index].input))
        }
        _ => None,
    }
}

/// True when `source`, copied into the buffer by `fill`, still holds that value
/// as its variable at `call`: a constant, or an explicit variable no statement
/// between the two in their shared block assigns.
fn reaches_call(fd: &Funcdata, fill: OpId, source: VarnodeId, call: OpId) -> bool {
    let Some(value) = fd.vbank().get(source) else {
        return false;
    };
    if value.is_constant() {
        return true;
    }
    if value.is_implied() {
        return false;
    }
    let Some(high) = value.get_high() else {
        return false;
    };
    if let Some(variable) = fd.high_bank().get(high) {
        let assigned_once = (0..variable.num_instances()).all(|index| {
            let instance = variable.get_instance(index);
            instance == source || fd.vbank().get(instance).is_some_and(|v| !v.is_written())
        });
        if assigned_once {
            return true;
        }
    }
    let block = fd.obank().get(fill).and_then(|op| op.get_parent());
    if block.is_none() || block != fd.obank().get(call).and_then(|op| op.get_parent()) {
        return false;
    }
    let mut cursor = fd.bb_op_next(fill);
    for _ in 0..MAX_BLOCK_OPS {
        let Some(id) = cursor else {
            return false;
        };
        if id == call {
            return true;
        }
        let writes = fd.obank().get(id).is_some_and(|op| {
            !effect_of(fd, id, call)
                && op
                    .get_out()
                    .and_then(|out| fd.vbank().get(out))
                    .is_some_and(|out| out.get_high() == Some(high))
        });
        if writes {
            return false;
        }
        cursor = fd.bb_op_next(id);
    }
    false
}

/// True when `id` is an INDIRECT standing for `call`'s effect.
fn effect_of(fd: &Funcdata, id: OpId, call: OpId) -> bool {
    fd.obank().get(id).is_some_and(|op| {
        op.code() == OpCode::CPUI_INDIRECT
            && !op.is_indirect_creation()
            && op
                .get_in(1)
                .and_then(|v| fd.vbank().get(v))
                .is_some_and(|v| OpId::from(slotmap::KeyData::from_ffi(v.get_offset())) == call)
    })
}

/// The COPY or SUBPIECE that sets `input`, when the buffer slot it writes is
/// read by nothing but this and later calls' effects.
fn fill_op(fd: &Funcdata, input: VarnodeId, effect: VarnodeId, inside: bool) -> Option<Fill> {
    let value = fd.vbank().get(input)?;
    let def = value.get_def()?;
    let single = value.num_descend() == 1;
    let mut current = input;
    let mut direct = true;
    let mut steps = 0;
    let (root, base) = loop {
        let Some(at) = fd.vbank().get(current)?.get_def() else {
            break (current, 0);
        };
        let op = fd.obank().get(at)?;
        match op.code() {
            OpCode::CPUI_COPY if steps < 8 => {
                let source = op.get_in(0)?;
                if steps == 0 {
                    if let Some(piece) = piece_of(fd, source).filter(|_| {
                        fd.vbank()
                            .get(source)
                            .is_some_and(|value| value.is_implied())
                    }) {
                        break piece;
                    }
                } else {
                    direct = false;
                }
                steps += 1;
                current = source;
                if !transit(fd, source) {
                    break (source, 0);
                }
            }
            OpCode::CPUI_SUBPIECE => {
                if steps != 0 {
                    direct = false;
                }
                break piece_of(fd, current)?;
            }
            _ if steps == 0 => return None,
            _ => break (current, 0),
        }
    };
    let hideable = inside && single && direct && dead_after(fd, effect);
    Some(Fill {
        op: def,
        root,
        base,
        hideable,
    })
}

/// True when `id` is a frame or temporary copy the buffer value passes through
/// on its way from the variable that computed it.
fn transit(fd: &Funcdata, id: VarnodeId) -> bool {
    let Some(value) = fd.vbank().get(id) else {
        return false;
    };
    let manage = fd.get_arch().manage();
    let index = value.get_space().get_index();
    value.is_written()
        && (manage
            .get_stack_space()
            .is_some_and(|space| space.get_index() == index)
            || manage
                .get_unique_space()
                .is_some_and(|space| space.get_index() == index))
}

/// True when the buffer slot `effect` holds after the call is read by nothing
/// but later calls' effects and merges of them.
fn dead_after(fd: &Funcdata, effect: VarnodeId) -> bool {
    let mut pending = vec![effect];
    let mut seen = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        if seen.len() > MAX_WALK {
            return false;
        }
        let Some(value) = fd.vbank().get(id) else {
            return false;
        };
        for reader in value.descend_iter() {
            let Some(reader) = fd.obank().get(reader) else {
                return false;
            };
            if !matches!(
                reader.code(),
                OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL
            ) {
                return false;
            }
            let Some(out) = reader.get_out() else {
                return false;
            };
            pending.push(out);
        }
    }
    true
}

/// `(whole, byte)` when `id` is a SUBPIECE of `whole` at `byte`, seen through
/// a logical right shift by whole bytes and casts of the whole.
fn piece_of(fd: &Funcdata, id: VarnodeId) -> Option<(VarnodeId, u64)> {
    let value = fd.vbank().get(id)?;
    let width = value.get_size() as u64;
    let op = fd.obank().get(value.get_def()?)?;
    if op.code() != OpCode::CPUI_SUBPIECE {
        return None;
    }
    let offset = fd.vbank().get(op.get_in(1)?)?;
    if !offset.is_constant() {
        return None;
    }
    let mut byte = offset.get_offset();
    let mut whole = op.get_in(0)?;
    for _ in 0..8 {
        let Some(def) = fd.vbank().get(whole)?.get_def() else {
            break;
        };
        let op = fd.obank().get(def)?;
        match op.code() {
            OpCode::CPUI_CAST | OpCode::CPUI_COPY => whole = op.get_in(0)?,
            OpCode::CPUI_INT_RIGHT => {
                let shift = fd.vbank().get(op.get_in(1)?)?;
                if !shift.is_constant() || shift.get_offset() % 8 != 0 {
                    break;
                }
                let source = op.get_in(0)?;
                let size = fd.vbank().get(source)?.get_size() as u64;
                let skipped = byte.checked_add(shift.get_offset() / 8)?;
                if skipped.checked_add(width)? > size {
                    return None;
                }
                byte = skipped;
                whole = source;
            }
            _ => break,
        }
    }
    let size = fd.vbank().get(whole)?.get_size() as u64;
    (byte.checked_add(width)? <= size).then_some((whole, byte))
}

#[cfg(test)]
mod tests {
    use super::shape;

    /// The entry points are matched with any number of leading underscores, and
    /// only those that take a buffer carry its slots.
    #[test]
    fn entry_point_shapes() {
        for name in ["_os_log_impl", "__os_log_impl", "os_log_error_impl", "_os_log_fault_impl"] {
            let found = shape(name).expect(name);
            assert_eq!((found.fixed, found.buffer), (4, Some((5, 6))), "{name}");
        }
        let signpost = shape("_os_signpost_emit_with_name_impl").unwrap();
        assert_eq!((signpost.fixed, signpost.buffer), (6, Some((7, 8))));
        let kernel = shape("_os_log_internal").unwrap();
        assert_eq!((kernel.fixed, kernel.buffer), (4, None));
        for other in ["os_log_type_enabled", "_os_log_create", "os_log_implx", ""] {
            assert!(shape(other).is_none(), "{other}");
        }
    }
}
