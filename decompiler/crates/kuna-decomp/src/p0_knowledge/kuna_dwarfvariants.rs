//! (kuna `dwarfvariants`) The DWARF **discriminated-union** gate, plus the
//! recovered-layout side table it fills.
//!
//! `dwarfstructs` (DIV-86) reads a `DW_TAG_structure_type`'s `DW_AT_byte_size`
//! and its `DW_TAG_member` children and deliberately stops there: a Rust enum
//! carries no `DW_TAG_member` at all, so it recovers its width and **no fields**.
//! Its whole layout lives under a `DW_TAG_variant_part`, which states three
//! things codegen cannot:
//!
//! * `DW_AT_discr` — WHICH member is the discriminant (offset, width, type);
//! * `DW_AT_discr_value` on each `DW_TAG_variant` — which value selects it;
//! * the variant's `DW_TAG_member` — the variant's NAME (`Ok`, `Err`, `Some`,
//!   `None`) and the payload struct describing its fields.
//!
//! A `DW_TAG_variant` with **no** `DW_AT_discr_value` is the default (niche /
//! untagged) variant: "any discriminant value the other variants did not claim".
//!
//! ## Scope, stated up front
//!
//! This needs **full debug info** (`-C debuginfo=2`, cargo `debug = true`). A
//! release Rust binary built without it carries no type DIEs, and this feature
//! then does exactly nothing — it is not "degraded", it is inert. That is a
//! deliberate limit: the alternative is inferring an enum from codegen shape,
//! and shape does not distinguish a `Result` from a `#[repr(C)]` struct, a
//! `(u64,u64)` tuple or a `&'static str` fat pointer.
//!
//! ## The side table
//!
//! The recovered geometry is recorded in [`VariantLayout`], keyed by the interned
//! type name, on the `TypeFactory`
//! ([`crate::dtype::TypeFactory::kuna_record_variant_layout`]). It is the
//! `kuna_wire_symbols` arrangement: a table that **never enters the analysis
//! scope** and is read by nothing in this change, so it cannot perturb emitted C.
//! It exists so a later pass can render `match` / `if let` / `Ok(v)` from the
//! compiler's own answer instead of re-deriving it.

/// Environment variable that gates DWARF variant-part import (read by
/// `kuna-analysis::analyzers::dwarf::kuna_dwarfvariants`). Absent or any value
/// other than the off-tokens ⇒ **enabled**; set to `off`/`0`/`false` ⇒ disabled.
///
/// An env var rather than an [`crate::architecture::Architecture`] bool for the
/// same reason as the sibling `dwarfstructs`/`typedepth` gates: the types are
/// baked into the analysis facts inside `load file`, which runs upstream of every
/// `option` command.
pub const DWARFVARIANTS_ENV: &str = "KUNA_DWARFVARIANTS";

/// Whether DWARF variant-part import is enabled for this process.
/// Default **on**: only an explicit off-token in [`DWARFVARIANTS_ENV`] disables it.
pub fn dwarfvariants_enabled() -> bool {
    match std::env::var(DWARFVARIANTS_ENV) {
        Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false"),
        Err(_) => true, // unset => default-on
    }
}

/// Bridge a `set_kuna_option("dwarfvariants", val)` toggle to
/// [`DWARFVARIANTS_ENV`] so a later `load file` in the *same* process sees the
/// choice (the interactive `option dwarfvariants ...` before `load file` path;
/// the CLI sets the env var on the subprocess directly).
pub fn set_dwarfvariants_env(on: bool) {
    std::env::set_var(DWARFVARIANTS_ENV, if on { "on" } else { "off" });
}

/// One variant of a recovered discriminated union.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantFacet {
    /// The variant's source name, verbatim from its `DW_TAG_member`
    /// (`Ok`, `Err`, `Some`, `None`, `Cons`, …).
    pub name: String,
    /// `DW_AT_discr_value`, masked to the discriminant's width. `None` marks the
    /// DEFAULT variant — the one DWARF gives no value, selected by every
    /// discriminant the other variants did not claim (Rust's niche encoding).
    pub discr: Option<u64>,
    /// The interned name of the payload struct built for this facet, i.e. the
    /// union member that names it. **Empty for a FIELDLESS variant** (`None`,
    /// `Nil`, a unit variant), which overlays nothing and therefore gets no union
    /// member — see the importer's note on `ScoreUnionFields`. Its name and its
    /// discriminant value still live here, which is what a `match` renderer needs.
    pub payload_type: String,
    /// The variant's fields as `(name, offset within the ENUM)` — absolute, not
    /// relative to the payload struct, so a consumer never has to re-add
    /// [`VariantLayout::payload_offset`].
    pub fields: Vec<(String, i32)>,
}

/// The recovered geometry of one DWARF `DW_TAG_variant_part`.
///
/// Written once per interned enum type by the DWARF importer; read by nothing in
/// the change that introduced it (see the module header).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantLayout {
    /// The interned kuna type name the layout describes (the key).
    pub type_name: String,
    /// `DW_AT_byte_size` of the enum.
    pub size: i32,
    /// Byte offset of the discriminant member (`DW_AT_discr` →
    /// `DW_AT_data_member_location`).
    pub tag_offset: i32,
    /// Width in bytes of the discriminant member's type.
    pub tag_size: i32,
    /// Byte offset of the overlay union WITHIN the recovered type — the lowest
    /// offset any variant places a field at, and 0 for a niche (where the overlay
    /// is the whole type). [`VariantFacet::fields`] offsets are absolute within
    /// the enum either way, so a consumer never has to add this back.
    pub payload_offset: i32,
    /// True when the discriminant's bytes and the payload OVERLAP, i.e. Rust read
    /// the tag out of a niche inside the payload rather than from a field of its
    /// own. The recovered type is then the overlay union ALONE (no enclosing
    /// struct and no `tag` field), because a tag field would have to sit at an
    /// offset a variant already owns.
    pub niche: bool,
    /// The interned name of the union that overlays the variants. Equal to
    /// [`Self::type_name`] for a niche, where the union is the whole type.
    pub union_type: String,
    /// The variants, in `DW_TAG_variant` order.
    pub variants: Vec<VariantFacet>,
}

impl VariantLayout {
    /// The variant a concrete discriminant value selects: the facet whose
    /// [`VariantFacet::discr`] matches, else the DEFAULT facet (the one DWARF
    /// gave no `DW_AT_discr_value`), else `None`.
    pub fn facet_for_discr(&self, value: u64) -> Option<&VariantFacet> {
        self.variants
            .iter()
            .find(|v| v.discr == Some(value))
            .or_else(|| self.variants.iter().find(|v| v.discr.is_none()))
    }

    /// The facet with the given source name.
    pub fn facet_named(&self, name: &str) -> Option<&VariantFacet> {
        self.variants.iter().find(|v| v.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env) and only the off-tokens disable it.
    /// Serialized by being a single test (env is process-global).
    #[test]
    fn dwarfvariants_env_gate() {
        std::env::remove_var(DWARFVARIANTS_ENV);
        assert!(dwarfvariants_enabled(), "unset => default-on");
        for off in ["off", "0", "false", "OFF", " off "] {
            std::env::set_var(DWARFVARIANTS_ENV, off);
            assert!(!dwarfvariants_enabled(), "`{off}` should disable");
        }
        for on in ["on", "1", "true", "anything"] {
            std::env::set_var(DWARFVARIANTS_ENV, on);
            assert!(dwarfvariants_enabled(), "`{on}` should enable");
        }
        set_dwarfvariants_env(false);
        assert!(!dwarfvariants_enabled());
        set_dwarfvariants_env(true);
        assert!(dwarfvariants_enabled());
        std::env::remove_var(DWARFVARIANTS_ENV);
    }

    fn layout() -> VariantLayout {
        VariantLayout {
            type_name: "Option<&u32>".into(),
            size: 8,
            tag_offset: 0,
            tag_size: 8,
            payload_offset: 0,
            niche: true,
            union_type: "Option<&u32>::payload".into(),
            variants: vec![
                VariantFacet {
                    name: "None".into(),
                    discr: Some(0),
                    payload_type: "Option<&u32>::None".into(),
                    fields: vec![],
                },
                VariantFacet {
                    name: "Some".into(),
                    discr: None,
                    payload_type: "Option<&u32>::Some".into(),
                    fields: vec![("__0".into(), 0)],
                },
            ],
        }
    }

    /// A value a variant claims picks that variant; every other value falls to
    /// the DEFAULT variant, which is the whole point of the niche encoding.
    #[test]
    fn facet_for_discr_prefers_exact_then_default() {
        let l = layout();
        assert_eq!(l.facet_for_discr(0).unwrap().name, "None");
        assert_eq!(l.facet_for_discr(1).unwrap().name, "Some");
        assert_eq!(l.facet_for_discr(0xdeadbeef).unwrap().name, "Some");
        assert_eq!(l.facet_named("Some").unwrap().fields, vec![("__0".to_string(), 0)]);
        assert!(l.facet_named("Err").is_none());
    }

    /// With no default variant an unclaimed value selects nothing -- the lookup
    /// must not fall back to "the first one".
    #[test]
    fn facet_for_discr_without_default_refuses_unclaimed() {
        let mut l = layout();
        l.niche = false;
        l.variants[1].discr = Some(1);
        assert_eq!(l.facet_for_discr(1).unwrap().name, "Some");
        assert!(l.facet_for_discr(7).is_none(), "no default variant => no answer");
    }
}
