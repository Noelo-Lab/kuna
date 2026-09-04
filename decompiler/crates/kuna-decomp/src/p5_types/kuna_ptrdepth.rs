//! Pointer-nesting cap for inferred data types (kuna `ptrdepthcap`, P5).
//!
//! `ActionInferTypes` solves a bidirectional lattice over the p-code graph.  A
//! small-string-optimized C++ object (MSVC `std::basic_string`, `std::ostringstream`'s
//! buffer) writes an *unsatisfiable* equation into that lattice:
//!
//! ```text
//!   p = &obj;                     // PTRSUB(spacebase, -0x298)  ->  ptr(typeof obj)
//!   if (16 <= obj.capacity)
//!       p = obj.ptr;              // LOAD(&obj)                 ->  typeof obj
//!   ... *(char *)p ...
//! ```
//!
//! The MULTIEQUAL demands `typeof obj == ptr(typeof obj)`, which has no finite
//! solution.  `propagateOneType` therefore adds exactly one pointer level per
//! `ActionInferTypes` pass and only stops at the empirical seven-pass ceiling,
//! so the object and every temporary that touches it is declared
//! `unsigned long long *****` / `char *****`.  Upstream Ghidra already refuses to
//! deepen a pointer this way at the one seam it noticed —
//! `TypeFactory::getTypePointerNoDepth` (`type.cc:1509`), used by
//! `TypeOpLoad`/`TypeOpStore::propagateType` — but the spacebase PTRSUB arm that
//! actually drives the escalation here never goes through it.
//!
//! [`cap_pointer_depth`] applies exactly the `getTypePointerNoDepth` rule to
//! **every** type the propagation is about to adopt: a pointer whose target is
//! itself a pointer-to-pointer collapses to `ptr(undefined<N>)`, and
//! `ptr(ptr(undefined<N>))` collapses to `ptr(undefined<N>)` when `N` is the
//! pointer size.  `ptr(undefined<N>)` is a fixed point, so the lattice settles
//! instead of running to the ceiling, and a genuine `char **` (`argv`, `envp`)
//! is left alone.
//!
//! Gated by [`Architecture::ptrdepthcap`](crate::architecture::Architecture)
//! (option `ptrdepthcap on|off`); inert when off.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, TypeFactory};

/// Deepest pointer nesting the propagation may adopt over a concrete base type.
/// Upstream `getTypePointerNoDepth` enforces the same value implicitly: it
/// rejects a target that is already a pointer-to-pointer.
pub const MAX_INFERRED_PTR_DEPTH: u32 = 2;

/// Apply `TypeFactory::getTypePointerNoDepth`'s rule to an already-built
/// candidate type.
///
/// Returns `None` when `dt` is acceptable as-is (the overwhelmingly common
/// case): anything that is not a pointer, and any pointer nested no deeper than
/// [`MAX_INFERRED_PTR_DEPTH`] over a concrete base.
pub fn cap_pointer_depth(tlst: &dyn TypeFactory, dt: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    if dt.get_metatype() != type_metatype::TYPE_PTR {
        return None;
    }
    let target = dt.get_ptr_to()?;
    if target.get_metatype() != type_metatype::TYPE_PTR {
        return None; // depth 1 -- always fine
    }
    let inner = target.get_ptr_to()?;
    match inner.get_metatype() {
        // depth >= 3 over anything: collapse the whole tail to one pointer to an
        // opaque word (C++ `pt = getBase(pt->getSize(), TYPE_UNKNOWN)`).
        type_metatype::TYPE_PTR => {
            let base = tlst.get_base(target.get_size(), type_metatype::TYPE_UNKNOWN).ok()?;
            let ws = dt.get_word_size().unwrap_or(1);
            tlst.get_type_pointer(dt.get_size(), base, ws).ok()
        }
        // `ptr(ptr(undefined<N>))` where N is the pointer width: the escalation
        // has already been collapsed once, so drop this level too and let
        // `ptr(undefined<N>)` be the fixed point (C++ `return pt`).
        type_metatype::TYPE_UNKNOWN if inner.get_size() == target.get_size() => Some(target),
        _ => None, // depth 2 over a concrete base -- `char **`, `int **`
    }
}

/// Pointer nesting depth of `dt`, saturating at `limit` (test/diagnostic helper).
pub fn pointer_depth(dt: &Rc<Datatype>, limit: u32) -> u32 {
    let mut depth = 0;
    let mut cur = Rc::clone(dt);
    while depth < limit && cur.get_metatype() == type_metatype::TYPE_PTR {
        match cur.get_ptr_to() {
            Some(next) => {
                depth += 1;
                cur = next;
            }
            None => break,
        }
    }
    depth
}

#[cfg(test)]
mod tests;
