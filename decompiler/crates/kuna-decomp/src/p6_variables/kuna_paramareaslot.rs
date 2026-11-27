//! An address-taken slot in the incoming-argument area that no parameter owns is a frame object.
//!
//! `ScopeLocal::restructure` leaves the prototype's parameter range to the parameter
//! symbols, so a slot there that the final prototype does not claim has no symbol: its
//! `&slot` reference renders as raw storage while its reads and writes render as a
//! separate variable. Once the prototype is final, such a slot gets a local symbol so
//! both spellings name one object.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, VarnodeId};
use crate::dtype::TypeFactory;
use crate::funcdata::Funcdata;

/// Whether the emitted program reads or writes this varnode as a value.
fn printed(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else {
        return false;
    };
    let defined = v
        .get_def()
        .and_then(|op| fd.obank().get(op))
        .is_some_and(|op| op.code() != OpCode::CPUI_INDIRECT);
    defined
        || v.descend_iter().any(|op| {
            fd.obank()
                .get(op)
                .is_some_and(|op| op.code() != OpCode::CPUI_INDIRECT)
        })
}

/// Whether every use of the address `out` hands it on whole: a call argument or a
/// stored value, never arithmetic that would reach past the slot.
fn passed_whole(fd: &Funcdata, out: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(out) else {
        return false;
    };
    v.descend_iter().all(|op| {
        fd.obank().get(op).is_some_and(|op| match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => op.get_slot(out) >= 1,
            OpCode::CPUI_STORE => op.get_slot(out) == 2,
            _ => false,
        })
    })
}

/// The stack addresses `PTRSUB(spacebase, #off)` references take in `space`, each
/// with whether every reference to it is passed whole.
fn referenced_slots(fd: &Funcdata, space: usize) -> Vec<(Address, bool)> {
    let mut slots: Vec<(Address, bool)> = Vec::new();
    for id in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(id) else {
            continue;
        };
        if op.code() != OpCode::CPUI_PTRSUB {
            continue;
        }
        let (Some(base), Some(offset), Some(out)) = (op.get_in(0), op.get_in(1), op.get_out()) else {
            continue;
        };
        if !fd.vbank().get(base).is_some_and(|v| v.is_spacebase())
            || !fd.vbank().get(offset).is_some_and(|v| v.is_constant())
        {
            continue;
        }
        let Some(addr) = fd.resolve_spacebase_ref_addr(offset) else {
            continue;
        };
        if addr.get_space().is_none_or(|s| s.get_index() as usize != space) {
            continue;
        }
        let whole = passed_whole(fd, out);
        match slots.iter_mut().find(|(slot, _)| *slot == addr) {
            Some((_, all)) => *all &= whole,
            None => slots.push((addr, whole)),
        }
    }
    slots
}

/// The width of the printed slot object starting at `addr` and the variables printing
/// it, or `None` when no printed varnode starts there or another varnode straddles
/// either end of it.
fn slot_object(fd: &Funcdata, addr: &Address) -> Option<(i32, Vec<HighVariableId>)> {
    let space = addr.get_space()?;
    let start = addr.get_offset();
    let window = Address::new(Rc::clone(space), start.saturating_sub(64));
    let mut width = 0;
    let mut highs = Vec::new();
    for vn in fd.vbank().iter_loc_addr_range(&window, &(addr + 1)) {
        let Some(v) = fd.vbank().get(vn) else {
            continue;
        };
        let Some(high) = v.get_high().filter(|_| !v.is_free()) else {
            continue;
        };
        let offset = v.get_offset();
        if offset != start {
            let end = offset.wrapping_add(v.get_size() as u64);
            if offset < start && end > start {
                return None;
            }
            continue;
        }
        width = width.max(v.get_size());
        if printed(fd, vn) && !highs.contains(&high) {
            highs.push(high);
        }
    }
    if highs.is_empty() || width <= 0 {
        return None;
    }
    let inside = fd
        .vbank()
        .iter_loc_addr_range(&(addr + 1), &(addr + i64::from(width)))
        .any(|vn| fd.vbank().get(vn).is_some_and(|v| !v.is_free()));
    (!inside).then_some((width, highs))
}

/// Map each unclaimed parameter-area slot whose address is only passed on whole and
/// which the body reads or writes as one scalar as a local of the width and type those
/// accesses use, when every variable printing it is named from the slot. A variadic
/// function's unclaimed slots are its variadic arguments, not scratch, and stay
/// unmapped. Run after the prototype is final and its parameter symbols are linked.
pub(crate) fn map(fd: &mut Funcdata) {
    if fd.get_func_proto().is_dotdotdot() {
        return;
    }
    let Some(space) = fd.get_scope_local().map(|lm| Rc::clone(lm.get_space_id())) else {
        return;
    };
    if fd
        .get_arch()
        .manage()
        .get_stack_space()
        .is_none_or(|stack| stack.get_index() != space.get_index())
    {
        return;
    }
    let candidates = referenced_slots(fd, space.get_index() as usize);
    if candidates.is_empty() {
        return;
    }
    let Some(types) = fd.get_arch().types_rc() else {
        return;
    };
    let invalid = Address::new_invalid();
    for (addr, whole) in &candidates {
        if !whole {
            continue;
        }
        let addr = addr.clone();
        let Some((width, highs)) = slot_object(fd, &addr) else {
            continue;
        };
        let highs_len = highs.len();
        let end = addr.get_offset().wrapping_add(width as u64);
        if candidates.iter().any(|(other, _)| other.get_offset() > addr.get_offset() && other.get_offset() < end) {
            continue;
        }
        if !fd.get_func_proto().get_param_range().in_range(&addr, width) {
            continue;
        }
        let Some(lm) = fd.get_scope_local() else {
            return;
        };
        if !lm.in_scope(&addr, width) || lm.database().find_overlap(lm.scope_id(), &addr, width).is_some() {
            continue;
        }
        let mut reps = Vec::with_capacity(highs.len());
        for high in highs {
            match fd.high_name_representative(high).and_then(|rep| fd.vbank().get(rep)) {
                Some(rep) if rep.get_addr() == &addr => reps.push(Rc::clone(rep.get_type())),
                _ => break,
            }
        }
        if reps.len() != highs_len {
            continue;
        }
        let declared = match reps.as_slice() {
            [only] if only.get_size() == width => types.concretize(Rc::clone(only)).ok(),
            _ => None,
        };
        let Some(ct) = declared.or_else(|| types.get_base(width, crate::dtype::type_metatype::TYPE_UNKNOWN).ok()) else {
            continue;
        };
        if let Some(lm) = fd.get_scope_local_mut() {
            let _ = lm.add_symbol("", ct, &addr, &invalid);
        }
    }
}
