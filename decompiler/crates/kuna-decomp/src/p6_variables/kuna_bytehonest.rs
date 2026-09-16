//! (kuna `bytehonest`) How an uncommitted BYTE is spelled on the
//! `decompile-all --json` `variables` surface (P6, JSON only).
//!
//! A one-byte value the type system never committed to is carried as a size-1
//! `TYPE_UNKNOWN` (Ghidra's `undefined1`, spelled `xunknown1` in kuna's own
//! vocabulary).  `realtypes` -- default-ON, and the rendering every real
//! decompile runs at -- renders that carrier as `char`, which is right for the C
//! text (`char` is the only one-byte C type that reads as a value) but asserts on
//! the machine-readable surface a fact the recovery never established: the
//! program's own type there may as well be `_Bool`, `unsigned char`, `int8_t` or
//! one byte of a struct nobody split.  Report the width instead --
//! `undefined1`, the same spelling Ghidra uses for the same fact, and the
//! analogue of IDA's `_BYTE`.
//!
//! The predicate is deliberately narrower than the `framelayout` slot speller
//! ([`crate::decompile_drive`]'s `frame_slot_type_name`): it is the datatype's own
//! metatype and size, size 1 ONLY, and never an array.  An array of unknown is a
//! `char[264]` buffer whose element type is the honest part of the answer -- the
//! width-only `undefined264` would drop the array-ness the consumer needs.
//!
//! JSON surface only.  The emitted C is untouched: the printer still spells the
//! same carrier `char`, so the same variable can read `char` in the `.c` text and
//! `undefined1` in `variables[]`.  That divergence is intentional -- the C text is
//! a rendering that must be legal C, `variables[]` is a report of what was
//! recovered -- and is written down in `docs/spec/06-variables-and-merge.md`.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype};
use crate::infra::architecture::Architecture;

/// A one-byte value the type system never committed to: the carrier the
/// `variables` surface must report by width rather than by a fabricated element
/// type.  Reads the datatype, never its spelling, so it is independent of
/// `realtypes`/`ctypes` (with `realtypes` off the same row exports the internal
/// `xunknown1`, which matches nothing at all).
pub fn is_uncommitted_byte(dt: &Rc<Datatype>) -> bool {
    dt.get_metatype() == type_metatype::TYPE_UNKNOWN && dt.get_size() == 1
}

/// How an exported `variables[]` row spells its data type: the width-only
/// `undefined1` for an uncommitted byte, and whatever the printer says for every
/// other type.  The exported `size` is the datatype's size either way.
pub fn exported_type_name(arch: &Architecture, dt: &Rc<Datatype>) -> String {
    if arch.byte_honest && is_uncommitted_byte(dt) {
        return "undefined1".to_string();
    }
    crate::printc::type_to_c_string(arch, dt)
}

#[cfg(test)]
mod tests;
