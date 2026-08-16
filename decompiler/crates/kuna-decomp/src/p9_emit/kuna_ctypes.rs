//! (kuna) The C type speller: name a core data-type the way the target's own C
//! compiler would, instead of with Ghidra's internal vocabulary.
//!
//! # The defect
//!
//! kuna's core types are literally *named* `uint1`/`int4`/`float8`/`float10`/
//! `code` (`Architecture::build_core_types`) — a verbatim port of upstream's
//! no-`<coretypes>` fallback branch, which the real Ghidra application never
//! takes because its Java side ships its own names over the wire. The printer
//! prints those names, so none of them is valid C.
//!
//! Worse, the output *mixes* vocabularies. The existing `realtypes` relabel
//! (DIV-6) fires only for `TYPE_UNKNOWN`, so a residual `xunknown4` renders
//! `unsigned int` while the genuine `TYPE_UINT` core type `uint4` beside it does
//! not — one function declares `unsigned int v3;` and `int4 v1;` in the same
//! block.
//!
//! # The rule
//!
//! Match the target's own declared widths, in declaration order, first hit wins
//! — never a hard-coded 2/4/8 table. This is the port of Ghidra's
//! `DataOrganizationImpl.getIntegerCTypeApproximation`, and the order is what
//! makes it right: on LP64 both `long` and `long long` are 8 bytes and an 8-byte
//! integer must read `long`, while on ILP32 and LLP64 `long` is 4 so the same
//! size lands on `long long`. The widths come from the compiler spec's
//! `<data_organization>`; see chapter 5 of the spec.
//!
//! # What has no C spelling
//!
//! Sizes 3/5/6/7 (and 16-byte integers) name no C type on any target. They keep
//! their `undefined<N>` form rather than being widened, because widening changes
//! meaning: `(undefined3)x` is a 24-bit truncation and `(unsigned int)x` is not.
//! The recompile prelude typedefs those names for the project export.
//!
//! Floating-point is the one place an approximation is unavoidable. A 10-byte
//! `float10` is the x87 extended *register* value; `long double` is the widest
//! floating type C can name, and its `sizeof` is 12 (i386 ELF), 16 (x86-64) or 8
//! (MSVC, where it aliases `double`) — never 10. Spelling it `long double` is
//! therefore an approximation, and deliberately the same one the recompile
//! prelude already makes (`typedef long double float10; /* sizeof may differ */`),
//! so the emitted `.c` and the emitted `.h` cannot disagree.
//!
//! # Why the printer and not the factory
//!
//! Renaming the interned core types instead would be far more invasive and is
//! wrong on the merits: a core type's id is `hash_name(name)`, Ghidra-style
//! identifiers are derived from the first character of the type's name
//! (`float8` is what makes `fVar1`), and the console's C-type parser resolves
//! base types *solely* by `TypeFactory::find_by_name` with no `unsigned`/`long`
//! keywords in its grammar — 269 `<com> parse line` script lines across 87
//! corpus files feed `int4`/`float8` into exactly that path. This is a rendering
//! concern, so it is fixed at the rendering chokepoint.

use kuna_base::types::int4;

/// The target's C scalar widths, snapshotted from the type factory's decoded
/// `<data_organization>`. Copied rather than borrowed so the rendering context
/// stays `Copy` and threads cheaply through the declarator chokepoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CDataModel {
    pub char_size: int4,
    pub short_size: int4,
    pub int_size: int4,
    pub long_size: int4,
    pub long_long_size: int4,
    pub wchar_size: int4,
    pub float_size: int4,
    pub double_size: int4,
    pub long_double_size: int4,
}

impl CDataModel {
    /// The LP64 model (x86-64 System V), used as the neutral default by the
    /// disabled rendering context and by unit tests that do not care.
    pub const LP64: CDataModel = CDataModel {
        char_size: 1,
        short_size: 2,
        int_size: 4,
        long_size: 8,
        long_long_size: 8,
        wchar_size: 4,
        float_size: 4,
        double_size: 8,
        long_double_size: 10,
    };

    /// Read the model off a bootstrapped type factory.
    pub fn from_types(types: &dyn crate::dtype::TypeFactory) -> CDataModel {
        CDataModel {
            char_size: types.get_size_of_char(),
            short_size: types.get_size_of_short(),
            int_size: types.get_size_of_int(),
            long_size: types.get_size_of_long(),
            long_long_size: types.get_size_of_long_long(),
            wchar_size: types.get_size_of_wchar(),
            float_size: types.get_size_of_float(),
            double_size: types.get_size_of_double(),
            long_double_size: types.get_size_of_long_double(),
        }
    }

    /// The C spelling of an integer of `size` bytes, or `None` when the target
    /// names no type of that width (3/5/6/7, 16-byte integers).
    ///
    /// Declaration order, first hit wins. A 1-byte integer is spelled
    /// `signed char` / `unsigned char` rather than bare `char`, whose signedness
    /// is implementation-defined — and kuna has a separate `char` core type for
    /// text, which keeps its own spelling.
    pub fn integer_spelling(&self, size: int4, signed: bool) -> Option<&'static str> {
        let ranks: [(int4, &'static str, &'static str); 5] = [
            (self.char_size, "signed char", "unsigned char"),
            (self.short_size, "short", "unsigned short"),
            (self.int_size, "int", "unsigned int"),
            (self.long_size, "long", "unsigned long"),
            (self.long_long_size, "long long", "unsigned long long"),
        ];
        ranks
            .iter()
            .find(|(w, _, _)| *w == size)
            .map(|(_, s, u)| if signed { *s } else { *u })
    }

    /// The C spelling of a floating-point value of `size` bytes.
    ///
    /// An exact match wins. A width above `double` with no exact match is spelled
    /// `long double` — the widest floating type C can name — which is how the x87
    /// 80-bit `float10` and the 16-byte `float16` are reached. That is an
    /// approximation of storage, never of value: no target has a 10-byte
    /// `sizeof`. A width below `float` names nothing.
    pub fn float_spelling(&self, size: int4) -> Option<&'static str> {
        if size == self.float_size {
            return Some("float");
        }
        if size == self.double_size {
            return Some("double");
        }
        if size == self.long_double_size || size > self.double_size {
            return Some("long double");
        }
        None
    }
}

/// The C spelling of one core data-type, or `None` to keep its existing name.
///
/// `size` and `meta` are the type's own; `is_char` marks kuna's text types
/// (`char`, `wchar2`, `wchar4`), which are spelled as character types rather
/// than as integers of the same width.
///
/// `TYPE_UNKNOWN` is deliberately NOT handled here — that is the older
/// `realtypes` relabel, which answers a different question (an un-inferred value
/// has no known sign, so it is spelled unsigned) and must keep its own table so
/// `option realtypes off` still restores the upstream rendering exactly.
pub fn core_type_spelling(
    m: &CDataModel,
    meta: crate::dtype::type_metatype,
    size: int4,
    is_char: bool,
) -> Option<&'static str> {
    use crate::dtype::type_metatype::*;
    match meta {
        TYPE_INT if is_char => {
            // `char` is already C; a wide character type is `wchar_t` when it
            // matches the target's declared width, and otherwise falls back to a
            // plain integer of that width rather than claiming a width wchar_t
            // does not have here.
            if size == m.char_size {
                Some("char")
            } else if size == m.wchar_size {
                Some("wchar_t")
            } else {
                m.integer_spelling(size, true)
            }
        }
        TYPE_INT => m.integer_spelling(size, true),
        TYPE_UINT => m.integer_spelling(size, false),
        TYPE_FLOAT => m.float_spelling(size),
        // `code` is Ghidra's pseudo-type for a function body; the only way it
        // reaches the output is as `code *`, a function pointer. `void *` is the
        // honest C spelling of a pointer whose target has no expressible type.
        TYPE_CODE => Some("void"),
        // TYPE_BOOL already spells `bool` (C23, and C99 with <stdbool.h>, which
        // the recompile prelude includes), so it needs no relabel.
        _ => None,
    }
}

#[cfg(test)]
mod tests;
