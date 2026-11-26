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

/// The stack addresses `PTRSUB(spacebase, #off)` references take in `space`.
fn referenced_slots(fd: &Funcdata, space: usize) -> Vec<Address> {
    let mut slots: Vec<Address> = Vec::new();
    for id in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(id) else {
            continue;
        };
        if op.code() != OpCode::CPUI_PTRSUB {
            continue;
        }
        let (Some(base), Some(offset)) = (op.get_in(0), op.get_in(1)) else {
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
        if addr.get_space().is_some_and(|s| s.get_index() as usize == space) && !slots.contains(&addr) {
            slots.push(addr);
        }
    }
    slots
}

/// The width of the printed slot object starting at `addr` and the variables printing
/// it, or `None` when no printed varnode starts there or another varnode straddles its
/// start.
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
    (!highs.is_empty() && width > 0).then_some((width, highs))
}

/// Map each unclaimed address-taken parameter-area slot the body reads or writes as a
/// local of the width those accesses use, when every variable printing it is named
/// from the slot. Run after the prototype is final and its parameter symbols are linked.
pub(crate) fn map(fd: &mut Funcdata) {
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
    for addr in candidates {
        let Some((width, highs)) = slot_object(fd, &addr) else {
            continue;
        };
        if !fd.get_func_proto().get_param_range().in_range(&addr, width) {
            continue;
        }
        let Some(lm) = fd.get_scope_local() else {
            return;
        };
        if !lm.in_scope(&addr, width) || lm.database().find_overlap(lm.scope_id(), &addr, width).is_some() {
            continue;
        }
        let named_here = highs.into_iter().all(|high| {
            fd.high_name_representative(high)
                .and_then(|rep| fd.vbank().get(rep))
                .is_some_and(|rep| rep.get_addr() == &addr)
        });
        if !named_here {
            continue;
        }
        let Ok(ct) = types.get_base(width, crate::dtype::type_metatype::TYPE_UNKNOWN) else {
            continue;
        };
        if let Some(lm) = fd.get_scope_local_mut() {
            let _ = lm.add_symbol("", ct, &addr, &invalid);
        }
    }
}
