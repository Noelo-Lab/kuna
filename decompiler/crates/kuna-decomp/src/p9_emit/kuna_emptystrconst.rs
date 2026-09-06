//! (kuna) The empty-string-constant sub-stage — no upstream Ghidra equivalent.
//!
//! `PrintC::pushConstant`'s pointer arm hands a constant whose pointed-to type is
//! character-printable to `pushPtrCharConstant`, which renders the readonly bytes
//! at that address as a quoted literal instead of the address.  The trade is
//! deliberate: `strcpy(dst,"usage: %s\n")` reads better than
//! `strcpy(dst,(char *)0x403550)`.
//!
//! It stops being a trade when the literal is empty.  `StringManager::isString`
//! answers yes for any readonly location whose first byte is a NUL — the encoding
//! check walks to the first terminator and therefore validates *zero* characters —
//! so a pointer into a binary blob that happens to open with a zero row renders as
//! `p = ""`.  That token names no byte of the image, and the address it displaced
//! was the only thing in the statement a reader could follow.
//!
//! Not every empty literal is a mistake, though.  `setlocale(6,"")` is idiomatic
//! C, and a linker that merges string constants stores the program's only `""`
//! as the tail NUL of some other literal - so the empty string is *real* there,
//! and the address would be the worse render.  The two cases separate on the
//! bytes past the terminator: a genuine `""` is followed by the next literal in
//! the table (`00 00 00 65 72 72 6f 72 20 69 6e 20 72 65 67 75`), a blob pointer
//! by bytes no C string holds (`00 00 00 00 00 00 00 00 00 77 df 77 ff fd ff
//! 7f`).  So the literal is declined only where those bytes positively
//! contradict string data.
//!
//! Following the kuna-option idiom ([`crate::kuna_arraycoverwidth::OptionArrayCoverWidth`]),
//! this module owns the option struct that flips
//! [`crate::printc::PrintCOptions::empty_str_const`] plus the predicate the printer
//! consults; the caller (`Architecture::set_kuna_option`) writes the live flag.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::{int4, uint1};

use crate::options::on_or_off;

/// Marshaling element `<emptystrconst>` (kuna 4000+ range; 4147 = the previous
/// highest kuna id).
pub const ELEM_EMPTYSTRCONST: ElementId = ElementId::new("emptystrconst", 4148);

/// (kuna) Toggle the zero-character string-literal render of a constant pointer:
/// `emptystrconst on|off`.
///
/// "off" keeps the upstream literal (`p = ""`); "on" (the kuna default) declines
/// the literal so the pointer constant prints as its address.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionEmptyStrConst;

impl OptionEmptyStrConst {
    /// The option name.
    pub const NAME: &'static str = "emptystrconst";

    /// Parse + validate the `on`/`off` value; the caller performs the printer
    /// write ([`crate::printc::PrintCOptions::set_empty_str_const`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Empty string-constant suppression turned {prop}")))
    }
}

/// How many bytes at the constant's address are inspected for string evidence.
///
/// Sized off the two witnesses: it has to reach past a genuine `""` into
/// whatever literal follows it (three NULs, in the coreutils images), and past
/// the leading zero ROW of a packed blob (nine, in the maze), while staying
/// short enough to sit inside one run of literals.
pub const WINDOW: usize = 16;

/// Does `buf` read as string data?
///
/// Deliberately a FALSIFICATION test, not a confirmation one: it answers no only
/// when the bytes positively contradict "these are C strings", so anything it
/// cannot judge keeps the upstream literal.  Skip the terminator run, then walk
/// the next run up to its NUL; if any byte there is one no C string holds -
/// outside printable ASCII and the three whitespace controls a literal spells -
/// this is not a string table.  A window of nothing but NULs is padding at the
/// end of a section and says nothing either way.  The charset is the analysis
/// tier's string recognizer's, so "is this string data" gets the same answer at
/// both ends of the pipeline.
pub fn reads_as_string_data(buf: &[uint1]) -> bool {
    let legal = |b: uint1| b == b'\t' || b == b'\n' || b == b'\r' || (0x20..0x7f).contains(&b);
    let mut i = 0;
    while i < buf.len() && buf[i] == 0 {
        i += 1;
    }
    while i < buf.len() && buf[i] != 0 {
        if !legal(buf[i]) {
            return false;
        }
        i += 1;
    }
    true
}

/// Should the string-literal render of a constant pointer be declined?
///
/// Declined only when the option is on, the literal the printer just escaped
/// holds no characters at all, AND the bytes at the address do not read as
/// string data - so every non-empty literal, however short, and every empty one
/// sitting inside a run of real strings keep their upstream rendering.  `window`
/// is `None` when the neighbourhood could not be read, which is not evidence
/// against the literal and therefore keeps it.
pub fn declines_literal(enabled: bool, chars_emitted: int4, window: Option<&[uint1]>) -> bool {
    if !enabled || chars_emitted != 0 {
        return false;
    }
    match window {
        Some(w) => !reads_as_string_data(w),
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_emptystrconst/tests.rs"]
mod tests;
