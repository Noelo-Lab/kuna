//! (kuna) Language facets — what a C-shaped type *means* in the binary's
//! source language (`docs/multilang.md` W3).
//!
//! A Rust `&str`, a Rust `&[T]` and a Go `string` all reach the decompiler as
//! an ordinary sized struct (`{ptr, len}` words) or as its two words moved
//! separately; nothing in the `type_metatype` lattice can say which. kuna
//! never extends the metatype enum for this — its ~3,700 consumers match it
//! non-exhaustively, so a new variant would silently take default branches
//! (`kuna_dwarfvariants.rs` documents the same decision). Instead a facet is
//! a **name-keyed side record on the type factory**, the `VariantLayout`
//! pattern: the underlying type stays a plain struct every existing pass
//! handles, and the few places where language meaning changes behavior (the
//! `fatstr` literal rule now; printing and propagation later) look the facet
//! up by the type's name.
//!
//! Facets are registered by type providers — DWARF import, a language pack's
//! synthesized shells — through [`crate::dtype::TypeFactory`]'s
//! `kuna_register_lang_facet`, and survive `clear_noncore` exactly as
//! `VariantLayout` records do (both key by name, not by interned identity).

/// What a named type means in the source language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LangFacet {
    /// A Rust `&str`: `{ptr, len}`, length-carried UTF-8, never
    /// NUL-terminated.
    RustStr,
    /// A Rust `&[elem]` slice header: `{ptr, len}` with `len` counted in
    /// elements. The element is recorded by type NAME (the facet table's
    /// keying currency) so the record never pins an interned `Rc` across a
    /// `clear_noncore`.
    RustSlice {
        /// The element type's name.
        elem: String,
    },
    /// A Go `string` header: `{ptr, len}`, length-carried UTF-8, never
    /// NUL-terminated.
    GoString,
    /// A Go slice header: `{ptr, len, cap}` with `len`/`cap` counted in
    /// elements; the element is recorded by type name, as `RustSlice` does.
    GoSlice {
        /// The element type's name.
        elem: String,
    },
    /// A Go interface header: `{tab/type, data}` — two words. `eface` is the
    /// empty-interface form (`any`: runtime type word), otherwise an `iface`
    /// (itab word).
    GoIface {
        /// `true` for the empty-interface (`any`) form.
        eface: bool,
    },
    /// Two amd64 Go integer results carried by an ordinary joined struct.
    GoPairResults { signed: [bool; 2] },
}

impl LangFacet {
    /// `true` for the facets whose pointed-at bytes are text (a bounded
    /// string literal is the right rendering for a constant one).
    pub fn is_text(&self) -> bool {
        matches!(self, LangFacet::RustStr | LangFacet::GoString)
    }

    /// Validate the complete storage before treating a struct as paired results.
    pub fn go_pair_fields(&self, ty: &crate::dtype::Datatype) -> Option<[std::rc::Rc<crate::dtype::Datatype>; 2]> {
        use crate::dtype::{type_metatype, DatatypeKind};
        let Self::GoPairResults { signed } = self else { return None };
        if ty.get_metatype() != type_metatype::TYPE_STRUCT || ty.get_size() != 16
            || ty.get_alignment() != 8 || ty.is_incomplete() || ty.has_bitfields()
            || ty.num_depend() != 2
            || !matches!(&ty.kind, DatatypeKind::Struct { bitfield, .. } if bitfield.is_empty())
            || crate::kuna_typequal::is_volatile(ty)
        {
            return None;
        }
        let fields = [ty.get_field(0)?, ty.get_field(1)?];
        for (index, field) in fields.iter().enumerate() {
            let meta = if signed[index] { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
            if field.offset != index as i32 * 8 || field.name != ["result0", "result1"][index]
                || field.field_type.get_size() != 8 || field.field_type.get_metatype() != meta
                || crate::kuna_typequal::is_volatile(&field.field_type)
            {
                return None;
            }
        }
        Some(fields.map(|field| std::rc::Rc::clone(&field.field_type)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_facets() {
        assert!(LangFacet::RustStr.is_text());
        assert!(LangFacet::GoString.is_text());
        assert!(!LangFacet::RustSlice { elem: "u64".into() }.is_text());
        assert!(!LangFacet::GoSlice { elem: "u64".into() }.is_text());
        assert!(!LangFacet::GoIface { eface: true }.is_text());
        assert!(!LangFacet::GoPairResults { signed: [true, false] }.is_text());
    }

    #[test]
    fn pair_metadata_rejects_inconsistent_storage() {
        use crate::dtype::{type_metatype::*, Datatype, TypeField};
        use std::rc::Rc;
        let facet = LangFacet::GoPairResults { signed: [true, false] };
        let pair = |size, align, second_offset, second_name, second_size, second_meta| {
            let mut ty = Datatype::new_with_align(size, align, TYPE_STRUCT);
            ty.set_struct_fields(vec![
                TypeField::new(0, 0, "result0", Rc::new(Datatype::new_with_align(8, 8, TYPE_INT))),
                TypeField::new(8, second_offset, second_name, Rc::new(Datatype::new_with_align(second_size, 8, second_meta))),
            ], Vec::new(), size, align);
            ty
        };
        let valid = pair(16, 8, 8, "result1", 8, TYPE_UINT);
        assert!(facet.go_pair_fields(&valid).is_some());
        for malformed in [
            pair(24, 8, 8, "result1", 8, TYPE_UINT),
            pair(16, 4, 8, "result1", 8, TYPE_UINT),
            pair(16, 8, 0, "result1", 8, TYPE_UINT),
            pair(16, 8, 8, "second", 8, TYPE_UINT),
            pair(16, 8, 8, "result1", 4, TYPE_UINT),
            pair(16, 8, 8, "result1", 8, TYPE_INT),
            pair(16, 8, 8, "result1", 8, TYPE_FLOAT),
        ] {
            assert!(facet.go_pair_fields(&malformed).is_none());
        }
        assert!(LangFacet::GoIface { eface: false }.go_pair_fields(&valid).is_none());
    }
}
