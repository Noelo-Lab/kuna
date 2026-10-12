//! Opt-in layout of call-escaped integer storage as one frame-bounded array.

use std::collections::HashSet;
use std::rc::Rc;

use kuna_base::address::sign_extend;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::varmap::{MapState, RangeHint, RangeType, COPY_CONSTANT};

const MAX_BYTES: intb = 4096;

fn same_integer_type(left: &Datatype, right: &Datatype) -> bool {
    left.get_metatype() == right.get_metatype()
        && left.get_size() == right.get_size()
        && left.get_align_size() == right.get_align_size()
        && left.is_enum_type() == right.is_enum_type()
}

/// The integer pointee declared at every direct-call use of a plain frame address.
fn call_type(fd: &Funcdata, base: VarnodeId) -> Option<(Rc<Datatype>, Vec<OpId>)> {
    let mut ty: Option<Rc<Datatype>> = None;
    let mut calls = Vec::new();
    for use_op in fd.vbank().get(base)?.descend_iter() {
        if calls.contains(&use_op) {
            continue;
        }
        let op = fd.obank().get(use_op)?;
        if op.is_dead() {
            continue;
        }
        if op.code() != OpCode::CPUI_CALL {
            return None;
        }
        let fc = fd.get_call_specs(fd.get_call_specs_index(use_op)?);
        if fc.format_arity().is_some()
            || fd
                .get_override()
                .find_proto_override(op.get_addr())
                .is_some()
        {
            return None;
        }
        let slots: Vec<_> = (1..op.num_input())
            .filter(|slot| op.get_in(*slot) == Some(base))
            .collect();
        if slots.is_empty() {
            return None;
        }
        for slot in slots {
            let param = fc.proto().get_param(slot - 1)?;
            if !param.is_type_locked() {
                return None;
            }
            let ptr = param.get_type()?;
            if ptr.get_metatype() != type_metatype::TYPE_PTR {
                return None;
            }
            let inner = ptr.get_ptr_to()?;
            if !matches!(
                inner.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT
            ) || !matches!(inner.get_size(), 4 | 8)
                || inner.is_enum_type()
                || inner.get_align_size() != inner.get_size()
            {
                return None;
            }
            if ty.as_ref().is_some_and(|t| !same_integer_type(t, &inner)) {
                return None;
            }
            ty = Some(inner);
        }
        calls.push(use_op);
    }
    Some((ty?, calls))
}

fn call_indirect_input(fd: &Funcdata, vn: VarnodeId, calls: &[OpId]) -> Option<VarnodeId> {
    let Some(def) = fd.vbank().get(vn).and_then(|v| v.get_def()) else {
        return None;
    };
    let Some(op) = fd.obank().get(def) else {
        return None;
    };
    (op.code() == OpCode::CPUI_INDIRECT
        && op
            .get_in(1)
            .and_then(|id| fd.vbank().get(id))
            .is_some_and(|iop| {
                calls.contains(&crate::funcdata_varnode::op_iop_decode(iop.get_offset()))
            }))
    .then(|| op.get_in(0))
    .flatten()
}

fn is_call_indirect(fd: &Funcdata, vn: VarnodeId, calls: &[OpId]) -> bool {
    call_indirect_input(fd, vn, calls).is_some()
}

/// Does a later frame slot retain a read of an INDIRECT caused by this call?
fn call_read(
    fd: &Funcdata,
    space: &Rc<AddrSpace>,
    start: intb,
    minimum: intb,
    end: intb,
    calls: &[OpId],
) -> bool {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    fd.vbank().iter_loc().any(|vn| {
        let Some(v) = fd.vbank().get(vn) else {
            return false;
        };
        if v.get_space().get_index() != space.get_index() {
            return false;
        }
        let offset = sign_extend(v.get_offset() as intb, bits);
        if offset <= start
            || offset < minimum
            || offset + v.get_size() as intb > end
            || !super::kuna_storereach::is_read(fd, vn)
        {
            return false;
        }
        is_call_indirect(fd, vn, calls)
    })
}

/// Does the proposed array contain storage independently defined by the caller?
fn has_independent_value(fd: &Funcdata, space: &Rc<AddrSpace>, start: intb, end: intb) -> bool {
    let bits = space.get_addr_size() as int4 * 8 - 1;
    fd.vbank().iter_loc().any(|vn| {
        let Some(v) = fd.vbank().get(vn) else {
            return false;
        };
        if v.get_space().get_index() != space.get_index() {
            return false;
        }
        let offset = sign_extend(v.get_offset() as intb, bits);
        if offset < start
            || offset + v.get_size() as intb > end
            || !super::kuna_storereach::is_read(fd, vn)
        {
            return false;
        }
        let mut pending = vec![vn];
        let mut seen = HashSet::new();
        while let Some(input_id) = pending.pop() {
            if !seen.insert(input_id) {
                continue;
            }
            if seen.len() > 256 {
                return true;
            }
            let Some(input) = fd.vbank().get(input_id) else {
                return true;
            };
            if input.get_space().get_index() != v.get_space().get_index()
                || input.get_offset() != v.get_offset()
                || input.get_size() != v.get_size()
            {
                return true;
            }
            let Some(def) = input.get_def().and_then(|id| fd.obank().get(id)) else {
                continue;
            };
            match def.code() {
                OpCode::CPUI_INDIRECT | OpCode::CPUI_COPY => {
                    let Some(next) = def.get_in(0) else {
                        return true;
                    };
                    pending.push(next);
                }
                OpCode::CPUI_MULTIEQUAL => {
                    if def.num_input() == 0 {
                        return true;
                    }
                    for slot in 0..def.num_input() {
                        let Some(next) = def.get_in(slot) else {
                            return true;
                        };
                        pending.push(next);
                    }
                }
                _ => return true,
            }
        }
        false
    })
}

fn capacity(start: intb, end: intb, width: intb) -> Option<int4> {
    let bytes = end.checked_sub(start)?;
    (start < 0
        && end <= 0
        && width > 0
        && bytes > width
        && bytes <= MAX_BYTES
        && bytes % width == 0)
        .then(|| (bytes / width) as int4)
}

fn minimum_end(h: &RangeHint) -> Option<intb> {
    let elements = if h.range_type == RangeType::Open && h.highind >= 0 {
        h.highind as intb + 1
    } else {
        1
    };
    h.sstart
        .checked_add((h.size as intb).checked_mul(elements)?)
}

fn compatible(h: &RangeHint, start: intb, end: intb, width: intb) -> bool {
    let ty = h
        .type_
        .get_array_base()
        .unwrap_or_else(|| Rc::clone(&h.type_));
    !h.is_type_lock()
        && h.sstart >= start
        && minimum_end(h).is_some_and(|last| last <= end)
        && (h.sstart - start) % width == 0
        && ty.get_size() as intb == width
        && !ty.is_enum_type()
        && matches!(
            ty.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
}

fn compatible_parent_hint(h: &RangeHint, start: intb, last: intb, width: intb) -> bool {
    compatible(h, start, last, width)
        || (!h.is_type_lock()
            && h.flags & COPY_CONSTANT != 0
            && h.sstart >= start
            && minimum_end(h).is_some_and(|end| end <= last)
            && (h.sstart - start) % width == 0
            && h.size as intb >= width
            && h.size as intb % width == 0
            && matches!(
                h.type_.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            ))
}

fn is_parent_root(hint: &RangeHint, indexed: bool) -> bool {
    indexed
        || hint.type_.num_elements().is_some_and(|count| count > 1)
        || (hint.range_type == RangeType::Open && hint.highind > 0)
}

fn joined_parent_end(
    hints: &[RangeHint],
    root: usize,
    width: intb,
    inferred_end: intb,
) -> Option<intb> {
    let first = hints.get(root)?;
    let start = first.sstart;
    let mut end = minimum_end(first)?;
    while end < inferred_end {
        let previous = end;
        for h in hints {
            let Some(last) = minimum_end(h) else { continue };
            if h.sstart >= start
                && h.sstart <= end
                && last > end
                && compatible_parent_hint(h, start, last, width)
            {
                end = last.min(inferred_end);
            }
        }
        if end == previous {
            break;
        }
    }
    Some(end)
}

/// Preserve shared storage up to the next address-taken base or the local frame end.
pub(super) fn coalesce(fd: &Funcdata, state: &mut MapState, space: &Rc<AddrSpace>) {
    if !fd.get_arch().call_array_extent {
        return;
    }
    let Some(types) = fd.get_arch().types_rc() else {
        return;
    };
    let frame_end = state.end_sstart().unwrap_or(0);
    let refs: Vec<(uintb, Option<VarnodeId>, VarnodeId)> = {
        let checker = state.checker_mut();
        checker
            .get_alias()
            .iter()
            .copied()
            .zip(checker.get_add_base())
            .map(|(offset, base)| (offset, base.index, base.base))
            .collect()
    };
    let indexed_offsets: Vec<_> = refs
        .iter()
        .filter(|(_, index, _)| index.is_some())
        .map(|(offset, _, _)| *offset)
        .collect();
    let refs: Vec<_> = refs
        .into_iter()
        .map(|(offset, index, base)| {
            (
                offset,
                if index.is_none() {
                    call_type(fd, base)
                } else {
                    None
                },
            )
        })
        .collect();
    let offsets: Vec<_> = refs.iter().map(|(offset, _)| *offset).collect();
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let signed = |offset: uintb| sign_extend(space.wrap_offset(offset) as intb, bits);
    for (offset, candidate) in &refs {
        let Some((ty, calls)) = candidate else {
            continue;
        };
        if refs
            .iter()
            .filter(|(other, _)| other == offset)
            .any(|(_, other)| {
                other
                    .as_ref()
                    .is_none_or(|(t, _)| !same_integer_type(t, ty))
            })
        {
            continue;
        }
        let offset = *offset;
        let call_start = signed(offset);
        let end = super::kuna_arrayextent::next_base(space, &offsets, offset)
            .map(signed)
            .unwrap_or(frame_end)
            .min(frame_end)
            .min(0);
        let width = ty.get_size() as intb;
        if space.get_word_size() != 1 {
            continue;
        }
        let (start, known_end) = {
            let hints = state.hints_mut();
            let mut start = call_start;
            let mut known_end = call_start;
            loop {
                let previous = start;
                for (index, h) in hints.iter().enumerate() {
                    let Some(mut last) = minimum_end(h) else {
                        continue;
                    };
                    let indexed_parent = indexed_offsets
                        .iter()
                        .any(|offset| signed(*offset) == h.sstart);
                    let parent_root = is_parent_root(h, indexed_parent);
                    let inferred_parent = if h.sstart < start && parent_root {
                        let Some(joined) = joined_parent_end(hints, index, width, call_start)
                        else {
                            continue;
                        };
                        last = joined;
                        start <= last
                    } else {
                        false
                    };
                    if h.sstart <= start
                        && (start < last || inferred_parent)
                        && (h.sstart == start || inferred_parent || indexed_parent)
                    {
                        start = start.min(h.sstart);
                        known_end = known_end.max(last);
                    }
                }
                if start == previous {
                    break;
                }
            }
            (start, known_end)
        };
        let Some(items) = capacity(start, end, width) else {
            continue;
        };
        let common_offset = space.wrap_offset(start as uintb);
        if known_end > end
            || !call_read(fd, space, call_start, known_end, end, calls)
            || !state.covers(common_offset, (end - start) as int4)
            || has_independent_value(fd, space, known_end, end)
        {
            continue;
        }
        let hints = state.hints_mut();
        let overlaps =
            |h: &RangeHint| h.sstart < end && minimum_end(h).is_none_or(|last| start < last);
        if hints.iter().filter(|h| overlaps(h)).any(|h| {
            let in_parent = minimum_end(h).is_some_and(|last| last <= known_end);
            if in_parent {
                !compatible_parent_hint(h, start, known_end, width)
            } else {
                !compatible(h, start, end, width)
            }
        }) {
            continue;
        }
        let Ok(array) = types.get_type_array(items, Rc::clone(ty)) else {
            continue;
        };
        hints.retain(|h| !overlaps(h));
        state.add_range_pub(common_offset, Some(array), 0, RangeType::Fixed, -1);
    }
}

#[cfg(test)]
mod tests;
