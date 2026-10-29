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
}

impl LangFacet {
    /// `true` for the facets whose pointed-at bytes are text (a bounded
    /// string literal is the right rendering for a constant one).
    pub fn is_text(&self) -> bool {
        matches!(self, LangFacet::RustStr | LangFacet::GoString)
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
    }
}
