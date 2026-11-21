//! (kuna `cstringconst`) A pointer-wide constant inside a string-literal section
//! prints as the literal there, whatever type it reaches the printer with.
//!
//! `PrintC::pushConstant` renders a constant as a quoted string only when its
//! type is a pointer to a character type, and `ActionConstantPtr` gives a
//! constant that type only when a global symbol covers its address. The string
//! pass plants a `char[N]` at a literal of four or more characters, so `" ("`,
//! `".SH"` or `")"` in the middle of a `__cstring` section has none, and a
//! constant that reaches it as an argument of a call through an untyped function
//! pointer, or as a `void *` chosen between two values, prints as the integer
//! `0x28abe` or the address `&dat_28a62`. The linker says what those bytes are:
//! a Mach-O `__cstring` section (type `S_CSTRING_LITERALS`) holds nothing but
//! NUL-terminated string literals. This module keeps those sections' ranges and
//! answers, for a pointer-wide constant inside one, the character type the
//! literal is read with; the printer's string path then reads the bytes, and
//! declines as it always does when they are not a string. An ELF
//! `SHF_STRINGS|SHF_MERGE` section is not taken: rustc marks a whole `.rodata`
//! so with records inside it.
//!
//! The integer arm takes only an argument of a call (slot 1 and up), reached
//! directly or through copies and phis: a value compared, added or stored might
//! be a plain integer that happens to equal an address, while an argument that
//! holds the address of a literal is that literal.

use std::rc::Rc;

use kuna_base::types::uintb;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// The `[start, end)` ranges of the sections that hold only string literals,
/// out of `(name, kind, vma, size)` rows whose `kind` is the `object` crate's
/// `SectionKind` rendered with `Debug`: a Mach-O `__cstring` section of type
/// `S_CSTRING_LITERALS`. An ELF `SHF_STRINGS|SHF_MERGE` section is not taken:
/// rustc gives a whole `.rodata` those flags with records inside it (a
/// `core::panic::Location` in the `panicloc` stage fixture).
pub fn literal_sections<'a>(
    sections: impl IntoIterator<Item = (&'a str, &'a str, u64, u64)>,
) -> Vec<(u64, u64)> {
    sections
        .into_iter()
        .filter(|&(name, kind, _, size)| name == "__cstring" && kind == "ReadOnlyString" && size != 0)
        .filter_map(|(_, _, vma, size)| Some((vma, vma.checked_add(size)?)))
        .collect()
}

/// The character type a constant `val` inside a string-literal section is read
/// with, when the option is on and the constant is as wide as a pointer.
pub fn literal_char(arch: &Architecture, val: uintb, size: i32) -> Option<Rc<Datatype>> {
    if !arch.cstring_const || size != arch.types().get_size_of_pointer() {
        return None;
    }
    if !arch.cstring_ranges.iter().any(|&(start, end)| start <= val && val < end) {
        return None;
    }
    arch.types().get_type_char(1).ok()
}

/// Is `vn`, read by `op`, an argument of a call (any input but the call
/// target), directly or through a few copies and phis (`v = 0x28a62; ...;
/// (*v7)(a0, a1, v, 0, 0)`)?
pub fn is_call_argument(fd: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    const DEPTH: usize = 4;
    let mut frontier = vec![(op, vn)];
    for _ in 0..DEPTH {
        let mut next = Vec::new();
        for (op, vn) in frontier {
            let Some(o) = fd.obank().get(op) else { continue };
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    if (1..o.num_input()).any(|slot| o.get_in(slot) == Some(vn)) {
                        return true;
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => {
                    if let Some(out) = o.get_out() {
                        if let Some(v) = fd.vbank().get(out) {
                            next.extend(v.descend_iter().map(|user| (user, out)));
                        }
                    }
                }
                _ => {}
            }
        }
        if next.is_empty() {
            return false;
        }
        frontier = next;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::literal_sections;

    #[test]
    fn only_string_literal_sections_are_taken() {
        let rows = [
            ("__text", "Text", 0x1000, 0x100),
            ("__cstring", "ReadOnlyString", 0x2000, 0x40),
            ("__const", "ReadOnlyData", 0x3000, 0x10),
            ("__cstring", "ReadOnlyString", 0x4000, 0),
            (".rodata", "ReadOnlyString", 0x5000, 0x80),
        ];
        assert_eq!(literal_sections(rows), [(0x2000, 0x2040)]);
    }
}
