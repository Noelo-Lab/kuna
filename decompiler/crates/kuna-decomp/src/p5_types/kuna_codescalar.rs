//! A `code` pointee never types a dereferenced value (kuna `codescalar`, P5).
//!
//! `TypeFactory::getTypeCode` builds the generic `code` data-type with **size 1**
//! (`type.cc`, `TypeCode()`), so that `code *` arithmetic advances one byte per
//! step.  That size is a convenience for pointer arithmetic, not a claim that a
//! byte-wide value can hold a function body — but the pointer-dereference rule in
//! `TypeOp::propagateFromPointer` (`typeop.cc:207`) reads it as one, because its
//! only test is `ptrto->getSize() == sz`:
//!
//! ```text
//!   v1 = *(code **)(v3 + 0xd4);   // an INT3-patch check
//!   if (*v1 != 0xcc)              //   LOAD  through code * ->  1-byte value
//!     v5 = (*v1)();
//!   *v1 = sub_44ac7a();           //   STORE through code * ->  1-byte value
//! ```
//!
//! Both accesses adopt `code` as the *value* type, `INT_EQUAL` spreads it to the
//! compared constant, and the merged high renders `code v4; // al` — printed
//! `void v4;` by the C back-end, then cast back to `char` for the arithmetic that
//! follows.  A scalar with no width cannot be assigned, stored or added, so the
//! function's C is not compilable and the byte the code actually manipulates has
//! no visible type.
//!
//! [`blocks_value_type`] is the whole feature: a `code` pointee is refused as the
//! dereferenced type, both where inference would adopt it and where the printer
//! would cast a stored value to it.  The pointer keeps its own `code *` type, so
//! the indirect call still renders `(*v1)()`; only the value side falls back to
//! the size-correct default (`undefined1`/`char`).
//!
//! Gated by [`Architecture::codescalar`](crate::architecture::Architecture)
//! (option `codescalar on|off`); with the option off the upstream reading is
//! restored exactly.

use crate::dtype::{type_metatype, Datatype};

/// Whether `ptrto` — the target of a pointer being dereferenced — must not be
/// adopted as the value's data-type.
///
/// True for exactly the generic `code` type and any `code` carrying a prototype;
/// everything else, including `code *` (metatype `TYPE_PTR`), is unaffected.
pub fn blocks_value_type(ptrto: &Datatype) -> bool {
    ptrto.get_metatype() == type_metatype::TYPE_CODE
}

#[cfg(test)]
mod tests;
