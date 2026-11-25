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
    /// An amd64 Go int result followed by an error's separate table and data words.
    GoIntErrorResults,
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

    /// Validate the integer and both interface words without conflating typed nil with nil.
    pub fn go_int_error_fields(&self, ty: &crate::dtype::Datatype) -> Option<[std::rc::Rc<crate::dtype::Datatype>; 3]> {
        use crate::dtype::{type_metatype, DatatypeKind};
        if *self != Self::GoIntErrorResults || ty.get_metatype() != type_metatype::TYPE_STRUCT
            || ty.get_size() != 24 || ty.get_alignment() != 8 || ty.is_incomplete()
            || ty.has_bitfields() || ty.num_depend() != 3
            || !matches!(&ty.kind, DatatypeKind::Struct { bitfield, .. } if bitfield.is_empty())
            || crate::kuna_typequal::is_volatile(ty)
        {
            return None;
        }
        let fields = [ty.get_field(0)?, ty.get_field(1)?, ty.get_field(2)?];
        for (index, field) in fields.iter().enumerate() {
            let field_type = &field.field_type;
            if field.offset != index as i32 * 8
                || field.name != ["result0", "error_tab", "error_data"][index]
                || field_type.get_size() != 8 || field_type.is_incomplete()
                || field_type.has_bitfields() || crate::kuna_typequal::is_volatile(field_type)
            {
                return None;
            }
            if index == 0 {
                if field_type.get_metatype() != type_metatype::TYPE_INT { return None; }
            } else {
                if field_type.get_metatype() != type_metatype::TYPE_PTR { return None; }
                let DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 } = &field_type.kind else {
                    return None;
                };
                if ptrto.get_metatype() != type_metatype::TYPE_VOID || ptrto.get_size() != 0
                    || ptrto.is_incomplete() || crate::kuna_typequal::is_volatile(ptrto)
                {
                    return None;
                }
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
        assert!(!LangFacet::GoIntErrorResults.is_text());
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

    fn int_error_carrier() -> crate::dtype::Datatype {
        use crate::dtype::{type_metatype::*, Datatype, DatatypeKind, TypeField};
        use std::rc::Rc;
        let mut pointer = Datatype::new_with_align(8, 8, TYPE_PTR);
        pointer.kind = DatatypeKind::Pointer {
            ptrto: Rc::new(Datatype::new_with_align(0, 1, TYPE_VOID)),
            spaceid: None, truncate: None, wordsize: 1,
        };
        let pointer = Rc::new(pointer);
        let mut carrier = Datatype::new_with_align(24, 8, TYPE_STRUCT);
        carrier.set_struct_fields(vec![
            TypeField::new(0, 0, "result0", Rc::new(Datatype::new_with_align(8, 8, TYPE_INT))),
            TypeField::new(8, 8, "error_tab", Rc::clone(&pointer)),
            TypeField::new(16, 16, "error_data", pointer),
        ], Vec::new(), 24, 8);
        carrier
    }

    #[test]
    fn int_error_metadata_is_distinct_from_interface_and_integer_pairs() {
        let carrier = int_error_carrier();
        let fields = LangFacet::GoIntErrorResults.go_int_error_fields(&carrier).unwrap();
        assert_eq!(fields.len(), 3);
        assert!(LangFacet::GoIface { eface: false }.go_int_error_fields(&carrier).is_none());
        assert!(LangFacet::GoIface { eface: true }.go_int_error_fields(&carrier).is_none());
        assert!(LangFacet::GoPairResults { signed: [true, true] }.go_int_error_fields(&carrier).is_none());
        assert!(LangFacet::GoIntErrorResults.go_pair_fields(&carrier).is_none());
    }

    #[test]
    fn int_error_metadata_rejects_incomplete_or_changed_carriers() {
        use crate::dtype::{flags, type_metatype::*, DatatypeKind, TypeBitField};
        use std::rc::Rc;
        for change in 0..9 {
            let mut carrier = int_error_carrier();
            match change {
                0 => carrier.size = 16,
                1 => carrier.alignment = 4,
                2 => carrier.metatype = TYPE_UNION,
                3 => carrier.flags |= flags::type_incomplete,
                4 => carrier.flags |= flags::has_bitfields,
                5 => carrier.qualifiers = crate::kuna_typequal::VOLATILE,
                6 => carrier.kind = DatatypeKind::Base,
                _ => {
                    let DatatypeKind::Struct { field, bitfield } = &mut carrier.kind else { unreachable!() };
                    if change == 7 { field.pop(); } else {
                        bitfield.push(TypeBitField::new(0, 1, false, "extra", Rc::clone(&field[0].field_type)));
                    }
                }
            }
            assert!(LangFacet::GoIntErrorResults.go_int_error_fields(&carrier).is_none(), "mutation {change}");
        }
    }

    #[test]
    fn int_error_metadata_rejects_changed_fields_and_pointer_targets() {
        use crate::dtype::{flags, type_metatype::*, Datatype, DatatypeKind};
        use std::rc::Rc;
        for index in 0..3 {
            for change in 0..10 {
                let mut carrier = int_error_carrier();
                let DatatypeKind::Struct { field, .. } = &mut carrier.kind else { unreachable!() };
                let member = &mut field[index];
                let ty = Rc::make_mut(&mut member.field_type);
                match change {
                    0 => member.offset += 8,
                    1 => member.name = "other".into(),
                    2 => ty.size = 4,
                    3 => ty.metatype = TYPE_UINT,
                    4 => ty.flags |= flags::type_incomplete,
                    5 => ty.qualifiers = crate::kuna_typequal::VOLATILE,
                    6 => ty.flags |= flags::has_bitfields,
                    _ if index == 0 => continue,
                    _ => {
                        let DatatypeKind::Pointer { ptrto, wordsize, .. } = &mut ty.kind else { unreachable!() };
                        match change {
                            7 => *ptrto = Rc::new(Datatype::new_with_align(8, 8, TYPE_INT)),
                            8 => Rc::make_mut(ptrto).qualifiers = crate::kuna_typequal::VOLATILE,
                            _ => *wordsize = 2,
                        }
                    }
                }
                assert!(LangFacet::GoIntErrorResults.go_int_error_fields(&carrier).is_none(), "field {index}, mutation {change}");
            }
        }
    }
}
