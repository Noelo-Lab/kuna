//! Declare an address-only local from the referenced object, not the PTRSUB offset.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::context::HighVariableId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// A reference high has no storage of its own. Its constants encode an address
/// offset whose width says nothing about the size of the referenced scalar.
/// The address width is kept when [`splits_wider_object`] finds the frame map
/// has likely cut a wider object in two.
pub(crate) fn object_type(fd: &Funcdata, high: HighVariableId) -> Option<Rc<Datatype>> {
    if crate::kuna_paramrefdecl::references_parameter_symbol(fd, high) {
        return None;
    }
    let h = fd.high_bank().get(high)?;
    let object = h.kuna_symbol_type()?;
    if h.kuna_symbol_offset() < 0
        || h.num_instances() == 0
        || matches!(object.get_metatype(), type_metatype::TYPE_ARRAY | type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION)
        || !(0..h.num_instances()).all(|i| {
            fd.vbank().get(h.get_instance(i)).is_some_and(|v| v.is_constant())
        })
        || splits_wider_object(fd, high)
    {
        return None;
    }
    Some(object.clone())
}

/// Does another Symbol with direct storage accesses lie wholly inside the
/// reference constant's width from the start of the referenced Symbol? Then the
/// narrower Symbol is likely one piece of a wider object the program reaches
/// through the address, and declaring only that piece would drop the rest.
fn splits_wider_object(fd: &Funcdata, high: HighVariableId) -> bool {
    let (Some(lm), Some(h)) = (fd.get_scope_local(), fd.high_bank().get(high)) else {
        return false;
    };
    let (Some(sym), Some(width)) = (
        h.kuna_ref_symbol(),
        fd.vbank().get(h.get_instance(0)).map(|v| v.get_size() as u64),
    ) else {
        return false;
    };
    let db = lm.database();
    let scope = db.symbol(sym).scope;
    let Some(whole) = db.symbol(sym).mapentry.iter().map(|&e| db.entry(scope, e)).find(|e| e.offset == 0) else {
        return false;
    };
    if whole.size as u64 >= width {
        return false;
    }
    let Some(spc) = whole.get_addr().get_space().cloned() else {
        return false;
    };
    let start = whole.get_addr().get_offset();
    let inside = |off: u64, size: int4| {
        off.wrapping_sub(start).checked_add(size as u64).is_some_and(|end| end <= width)
    };
    let lo = Address::new(Rc::clone(&spc), start);
    let hi = Address::new(Rc::clone(&spc), spc.wrap_offset(start.wrapping_add(width)));
    fd.vbank().iter_loc_addr_range(&lo, &hi).any(|id| {
        let Some(v) = fd.vbank().get(id).filter(|v| !v.is_constant()) else {
            return false;
        };
        match db.find_container_ignore_usepoint(scope, v.get_addr(), 1).map(|e| db.entry(scope, e)) {
            Some(n) => n.symbol != sym && inside(n.get_addr().get_offset(), n.size),
            None => inside(v.get_offset(), v.get_size()),
        }
    })
}
