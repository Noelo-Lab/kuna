//! (kuna) A pointer built to a record before the record was completed.
//!
//! Upstream completes a struct or union in place (`TypeFactory::setFields`), so a
//! pointer built to the incomplete stub sees the members once they arrive. kuna's
//! interned types are immutable: completing one mints a new `Rc`, and a pointer
//! built earlier still names the member-less stub, so every access through it
//! printed as offset arithmetic. A record's pointer to itself is always such a
//! pointer, since its member is built before the record is complete; so is a
//! pointer to a tag a C declaration named ahead of its body (`struct Node;`, a
//! sibling record first named inside a member list).
//!
//! [`resolve_completed_pointer`] gives the pointer to the completed record in
//! place of a pointer to its stub when the two are provably one type: the factory
//! holds the complete record under the stub's name, id and kind, and either the
//! record's own member points at that exact stub or a C declaration declared the
//! tag ahead ([`TypeFactory::kuna_declared_ahead`]). A stub matched to a
//! definition only by name, a DWARF declaration in one unit completed by a
//! definition from another, is left alone. The type-propagation edge of a LOAD or
//! STORE value and the load/store cast tokens of chapter 09 call it.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactory};

/// The completed record `stub` stands for, or `None` when `stub` is not an
/// incomplete struct or union or is not provably the stub of a completed one.
pub fn completed_record(types: &dyn TypeFactory, stub: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    let meta = stub.get_metatype();
    if !stub.is_incomplete() || (meta != type_metatype::TYPE_STRUCT && meta != type_metatype::TYPE_UNION) {
        return None;
    }
    let full = types.find_by_name(stub.get_name()).ok().flatten()?;
    if full.is_incomplete() || full.get_metatype() != meta || full.get_id() != stub.get_id() {
        return None;
    }
    (points_at(&full, stub) || types.kuna_declared_ahead(stub)).then_some(full)
}

/// Does a member of `record` (or an array member's element) point at `stub`?
fn points_at(record: &Datatype, stub: &Rc<Datatype>) -> bool {
    (0..record.num_depend()).filter_map(|i| record.get_field(i)).any(|f| {
        let mut ct = Rc::clone(&f.field_type);
        while let Some(elem) = ct.get_array_base() {
            ct = elem;
        }
        ct.get_ptr_to().is_some_and(|p| Rc::ptr_eq(&p, stub))
    })
}

/// The pointer to the completed record, when `ct` is a plain pointer to a
/// record's stub ([`completed_record`]); `None` for any other type.
pub fn resolve_completed_pointer(types: &dyn TypeFactory, ct: &Datatype) -> Option<Rc<Datatype>> {
    let DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize } = &ct.kind else {
        return None;
    };
    let full = completed_record(types, ptrto)?;
    types.get_type_pointer(ct.get_size(), full, *wordsize).ok()
}

#[cfg(test)]
mod tests;
