//! (kuna) The array-cover width sub-stage — no upstream Ghidra equivalent.
//!
//! A local whose storage is only ever indexed recovers as an array, and a
//! symbol-mapped access into it renders `name[index]` with
//! `index = symboloff / elementAlignSize`.  That index says nothing about how
//! WIDE the access is, so a sixteen-byte `movaps` transfer through a
//! `char v30[16]` bank rendered `v30[0] = v32[0]` — a one-byte lvalue for a
//! sixteen-byte copy, and a false statement about the program.
//!
//! An access that spans more than one element has no subscript that describes
//! it.  Ghidra's own `unnamedField` notation does: `v30._0_16_` names offset 0,
//! size 16.  kuna already emits it for a *partial* multi-element access (the
//! `v30._0_4_` reads in the same function); this sub-stage extends it to the
//! whole-array cover, which `PrintC::pushPartialSymbol`'s top-of-walk
//! whole-symbol break (printc.cc:2033) exits before the array arm can see.
//!
//! Following the kuna-option idiom ([`crate::kuna_arraynotation::OptionArrayNotation`]),
//! this module owns the option struct that flips
//! [`crate::printc::PrintCOptions::array_cover_width`] plus the width predicate
//! the printer consults; the caller (`Architecture::set_kuna_option`) writes the
//! live flag.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::dtype::{type_metatype, Datatype};
use crate::options::on_or_off;
use kuna_base::types::int4;

/// Marshaling element `<arraycoverwidth>` (kuna 4000+ range; 4144 = the previous
/// highest kuna id).
pub const ELEM_ARRAYCOVERWIDTH: ElementId = ElementId::new("arraycoverwidth", 4145);

/// (kuna) Toggle the width-carrying render of a multi-element array access:
/// `arraycoverwidth on|off`.
///
/// "off" keeps the upstream whole-symbol break (a bare name, or the caller's
/// `name[0]` subscript); "on" (the kuna default) renders `name._<off>_<size>_`.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionArrayCoverWidth;

impl OptionArrayCoverWidth {
    /// The option name.
    pub const NAME: &'static str = "arraycoverwidth";

    /// Parse + validate the `on`/`off` value; the caller performs the printer
    /// write ([`crate::printc::PrintCOptions::set_array_cover_width`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Array-cover width rendering turned {prop}")))
    }
}

/// Does an `sz`-byte access of `ct` span more than one array element?
///
/// True only for a TYPE_ARRAY whose element *aligned* size (the
/// `TypeArray::getSubEntry` stride, type.cc:1430) is strictly smaller than the
/// access — the one shape for which no subscript can carry the width.  A scalar,
/// a struct, a union and an access that fits inside one element all answer
/// false, so the whole-symbol break keeps its upstream behaviour for them.
pub fn spans_multiple_elements(ct: &Datatype, sz: int4) -> bool {
    if ct.get_metatype() != type_metatype::TYPE_ARRAY {
        return false;
    }
    match ct.get_array_base() {
        Some(elem) => (elem.get_align_size().max(1)) < sz,
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_arraycoverwidth/tests.rs"]
mod tests;
