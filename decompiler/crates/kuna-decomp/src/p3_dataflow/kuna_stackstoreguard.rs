//! Keep indexed stack writes between a stack slot's write and its later reads:
//! every recorded indexed byte STORE guards a constant-initialized slot, and an
//! indexed STORE of any width guards a written slot inside the array it may write.

use kuna_base::address::Address;
use kuna_base::types::int4;
use std::rc::Rc;

pub(super) fn enabled(
    fd: &crate::funcdata::Funcdata,
    addr: &Address,
    writes: &[crate::context::VarnodeId],
) -> bool {
    fd.stack_store_guard()
        && addr
            .get_space()
            .zip(fd.get_arch().manage().get_stack_space())
            .is_some_and(|(space, stack)| Rc::ptr_eq(space, stack))
        && writes.iter().any(|&vn| constant_write(fd, vn))
}

/// Does a recorded byte store into `spc`, or another stack access, have a
/// pointer that does not resolve to stack offsets? The layout cannot keep such
/// a frame's guarded slots in one local, so the frame is not guarded.
pub(super) fn frame_unresolved(
    fd: &crate::funcdata::Funcdata,
    spc: &Rc<kuna_base::space::AddrSpace>,
    store_guard: &[super::heritage::LoadGuard],
) -> bool {
    use kuna_num::opcodes::OpCode;
    let stores: Vec<crate::context::OpId> = store_guard
        .iter()
        .filter(|guard| Rc::ptr_eq(&guard.spc, spc))
        .map(|guard| guard.op)
        .filter(|&op| {
            fd.obank().get(op).is_some_and(|o| {
                o.code() == OpCode::CPUI_STORE
                    && !o.is_dead()
                    && o.get_in(2).and_then(|v| fd.vbank().get(v)).is_some_and(|v| v.get_size() == 1)
            })
        })
        .collect();
    !stores.is_empty() && crate::p6_variables::kuna_storereach::frame_unresolved(fd, spc, &stores)
}

/// May an indexed stack STORE overwrite a value this function wrote to the
/// stack range at `addr`? The range must have a write that is not an
/// INDIRECT or MULTIEQUAL; an input-only range (a stack parameter) is left
/// to the existing policy.
pub(super) fn indexed_enabled(
    fd: &crate::funcdata::Funcdata,
    addr: &Address,
    writes: &[crate::context::VarnodeId],
) -> bool {
    use kuna_num::opcodes::OpCode;
    fd.stack_store_guard()
        && addr
            .get_space()
            .zip(fd.get_arch().manage().get_stack_space())
            .is_some_and(|(space, stack)| Rc::ptr_eq(space, stack))
        && writes.iter().any(|&vn| {
            fd.vbank()
                .get(vn)
                .and_then(|v| v.get_def())
                .and_then(|d| fd.obank().get(d))
                .is_some_and(|d| !matches!(d.code(), OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL))
        })
}

/// The fallback element count for an unbounded provisional STORE window.
const UNLOCKED_ELEMENTS: u128 = 4;

/// The bytes `[lo, last]` each indexed STORE may write by its pointer's own
/// bound (`kuna_storereach.rs (store_window)`), computed once per heritage pass.
pub(super) type StoreBounds = std::collections::HashMap<crate::context::OpId, Option<(u64, u64)>>;

fn store_bound(
    fd: &crate::funcdata::Funcdata,
    guard: &super::heritage::LoadGuard,
    bounds: &mut StoreBounds,
) -> Option<(u64, u64)> {
    *bounds
        .entry(guard.op)
        .or_insert_with(|| crate::p6_variables::kuna_storereach::store_window(fd, guard.op, &guard.spc))
}

/// Can this STORE overlap `[lo, lo + size)`? Unanalyzed guards may write
/// anywhere; locked ranges and independently bounded pointers retain their
/// inferred window. Otherwise, cover four elements from the greater of the
/// pointer base and inferred minimum, clipped to the inferred maximum. A range
/// entirely before the base retains its original start.
pub(super) fn window_overlaps(
    guard: &super::heritage::LoadGuard,
    store_size: int4,
    bound: Option<(u64, u64)>,
    lo: u64,
    size: int4,
) -> bool {
    let width = store_size.max(1) as u128;
    let min = guard.minimum_offset as u128;
    let window_last = guard.maximum_offset as u128 + width - 1;
    let (first, last) = match (guard.analysis_state, bound) {
        (0, _) => (0, u128::MAX),
        (2, _) | (_, Some(_)) => (min, window_last),
        _ => {
            let base = guard.pointer_base as u128;
            let start = if base <= window_last {
                min.max(base)
            } else {
                min
            };
            (
                start,
                window_last.min(start + UNLOCKED_ELEMENTS * width - 1),
            )
        }
    };
    let (first, last) = match bound {
        Some((b_lo, b_last)) => (first.max(b_lo as u128), last.min(b_last as u128)),
        None => (first, last),
    };
    (lo as u128) <= last && first < lo as u128 + size.max(1) as u128
}

/// The live indexed stack STOREs into `spc` (any width) that may write
/// `[addr, addr + size)`, other than those in `skip` and those through
/// dynamically allocated stack.
pub(super) fn indexed_stores(
    fd: &crate::funcdata::Funcdata,
    store_guard: &[super::heritage::LoadGuard],
    bounds: &mut StoreBounds,
    addr: &Address,
    size: int4,
    skip: &[crate::context::OpId],
) -> Vec<crate::context::OpId> {
    use kuna_num::opcodes::OpCode;
    let Some(spc) = addr.get_space() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for guard in store_guard {
        if !Rc::ptr_eq(&guard.spc, spc) || guard.dynamic_stack || skip.contains(&guard.op) {
            continue;
        }
        let Some(store_size) = fd
            .obank()
            .get(guard.op)
            .filter(|op| !op.is_dead() && op.code() == OpCode::CPUI_STORE)
            .and_then(|op| op.get_in(2))
            .and_then(|v| fd.vbank().get(v))
            .map(|v| v.get_size())
        else {
            continue;
        };
        let bound = store_bound(fd, guard, bounds);
        if window_overlaps(guard, store_size, bound, addr.get_offset(), size) {
            out.push(guard.op);
        }
    }
    out
}

/// Remove each `(INDIRECT, STORE)` guard whose STORE's analyzed window turned
/// out not to reach the INDIRECT's range.
pub(super) fn prune_indexed(
    fd: &mut crate::funcdata::Funcdata,
    store_guard: &[super::heritage::LoadGuard],
    bounds: &mut StoreBounds,
    guards: &[(crate::context::OpId, crate::context::OpId)],
) {
    use kuna_num::opcodes::OpCode;
    for &(indirect, store) in guards {
        let Some(guard) = store_guard.iter().find(|g| g.op == store) else {
            continue;
        };
        let Some(store_size) = fd
            .obank()
            .get(store)
            .filter(|op| !op.is_dead() && op.code() == OpCode::CPUI_STORE)
            .and_then(|op| op.get_in(2))
            .and_then(|v| fd.vbank().get(v))
            .map(|v| v.get_size())
        else {
            continue;
        };
        let Some(op) = fd
            .obank()
            .get(indirect)
            .filter(|op| !op.is_dead() && op.code() == OpCode::CPUI_INDIRECT)
        else {
            continue;
        };
        let (Some(input), Some(output)) = (op.get_in(0), op.get_out()) else {
            continue;
        };
        let Some((lo, size)) = fd.vbank().get(output).map(|v| (v.get_offset(), v.get_size())) else {
            continue;
        };
        let bound = store_bound(fd, guard, bounds);
        if window_overlaps(guard, store_size, bound, lo, size) {
            continue;
        }
        fd.total_replace(output, input).expect("remove indexed store guard");
        fd.op_destroy(indirect);
    }
}

fn constant_write(fd: &crate::funcdata::Funcdata, mut vn: crate::context::VarnodeId) -> bool {
    use kuna_num::opcodes::OpCode;
    for _ in 0..16 {
        let Some(value) = fd.vbank().get(vn) else {
            return false;
        };
        if value.is_constant() {
            return true;
        }
        let Some(op) = value.get_def().and_then(|id| fd.obank().get(id)) else {
            return false;
        };
        if !matches!(
            op.code(),
            OpCode::CPUI_COPY
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
        ) {
            return false;
        }
        let Some(input) = op.get_in(0) else {
            return false;
        };
        vn = input;
    }
    false
}

/// Keep a stack slot's STORE INDIRECT whole instead of narrowing it to a
/// piece that some read uses as one multi-byte value: that piece would print
/// as a separate local the store never reaches. Byte reads may still narrow,
/// since a byte piece prints as an element of the array the store indexes.
pub(super) fn keeps_store_indirect_whole(
    fd: &crate::funcdata::Funcdata,
    vn: crate::context::VarnodeId,
    store: crate::context::OpId,
) -> bool {
    use kuna_num::opcodes::OpCode;
    if !fd.stack_store_guard()
        || fd.obank().get(store).is_none_or(|op| op.code() != OpCode::CPUI_STORE)
    {
        return false;
    }
    let Some(slot) = fd.vbank().get(vn) else {
        return false;
    };
    fd.get_arch().manage().get_stack_space().is_some_and(|s| Rc::ptr_eq(s, slot.get_space()))
        && slot.descend_iter().any(|op| {
            let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()).and_then(|o| fd.vbank().get(o)) else {
                return false;
            };
            out.get_size() > 1
                && out.descend_iter().any(|use_op| {
                    fd.obank().get(use_op).is_some_and(|u| u.code() != OpCode::CPUI_INDIRECT)
                })
        })
}

#[cfg(test)]
#[path = "kuna_stackstoreguard/tests.rs"]
mod tests;
