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
//! ## What the TYPE cannot say: which variant an access belongs to
//!
//! The recovered type overlays the variants as a **union**, and a union member
//! selects itself by OFFSET — the discriminant is never consulted. In a tagged
//! enum every payload variant starts immediately after the tag, so `Ok` and
//! `Err` sit at the same offset and the field the union scorer picks is not
//! evidence of anything. A variant name is therefore INSTALLED on the type only
//! where exactly one variant claims the bytes (`Option<T>`, whose only
//! payload-carrying variant is `Some`); where two or more claim them, the
//! installed member name is an offset-derived `field_0x…`. See
//! [`VariantFacet::label`].
//!
//! ## The side table, and the analysis that reads it
//!
//! The recovered geometry is recorded in [`VariantLayout`], keyed by the interned
//! type name, on the `TypeFactory`
//! ([`crate::dtype::TypeFactory::kuna_record_variant_layout`]). Nothing in the
//! *type* reads it, so filling it cannot perturb emitted C on its own.
//!
//! It exists for [`crate::p5_types::kuna_variantguard`] (`variantguard`), the
//! dominating-guard analysis: the label above is suppressed because the union
//! model cannot select a facet, not because the answer is unknown. Where a
//! branch tested the discriminant, or a constant was stored over it, the
//! compiler stated which variant is live and this table states which value
//! selects which variant — so the P9 printer spells the `DW_TAG_variant` name at
//! that access while the installed label stays `field_0x…` everywhere the proof
//! does not reach.

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
    /// The union-member spelling actually INSTALLED on the recovered type, which
    /// is [`Self::name`] only when no other variant claims a byte this one
    /// claims. Where two variants overlay the same bytes — every `Result<T,E>`,
    /// and any enum with two or more payload-carrying variants — the union member
    /// selects itself by offset and the discriminant is never consulted, so a
    /// variant name on such an access would be a coin flip; the label is then an
    /// offset-derived `field_0x…` and no emitted field path claims a variant.
    /// **Empty for a FIELDLESS variant**, which gets no union member at all.
    pub label: String,
    /// The interned name of the payload struct built for this facet, i.e. the
    /// type of the union member [`Self::label`] names. **Empty for a FIELDLESS
    /// variant** (`None`, `Nil`, a unit variant), which overlays nothing and
    /// therefore gets no union member — see the importer's note on
    /// `ScoreUnionFields`. Its name and its discriminant value still live here,
    /// which is what a `match` renderer needs.
    pub payload_type: String,
    /// The variant's fields as `(name, offset within the ENUM)` — absolute, not
    /// relative to the payload struct, so a consumer never has to re-add
    /// [`VariantLayout::payload_offset`].
    pub fields: Vec<(String, i32)>,
}

/// The recovered geometry of one DWARF `DW_TAG_variant_part`.
///
/// Written once per interned enum type by the DWARF importer; read by the
/// `variantguard` dominating-guard analysis
/// ([`crate::p5_types::kuna_variantguard`]) and by the P9 printer's union seams.
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

    /// The variant that owns union member `idx` of [`Self::union_type`].
    ///
    /// A FIELDLESS variant overlays nothing and is given no union member, so the
    /// members are the payload-carrying variants in `DW_TAG_variant` order and
    /// the two indices do not coincide. `variantguard` pins a member index and
    /// the printer spells the variant, so both need this mapping.
    pub fn facet_for_union_field(&self, idx: i32) -> Option<&VariantFacet> {
        if idx < 0 {
            return None;
        }
        self.variants.iter().filter(|v| !v.label.is_empty()).nth(idx as usize)
    }

    /// The union-member index of `self.variants[i]`, or `None` when that variant
    /// is fieldless and therefore has no member. The inverse of
    /// [`Self::facet_for_union_field`].
    pub fn union_field_for_variant(&self, i: usize) -> Option<i32> {
        if self.variants.get(i).map(|v| v.label.is_empty()).unwrap_or(true) {
            return None;
        }
        Some(self.variants[..i].iter().filter(|v| !v.label.is_empty()).count() as i32)
    }

    /// Whether any emitted field path can name a variant of this enum, i.e.
    /// whether at least one payload-carrying variant kept its source name. False
    /// for every enum with two or more variants overlaying the same bytes — the
    /// suppression rule in [`VariantFacet::label`].
    pub fn names_any_variant(&self) -> bool {
        self.variants.iter().any(|v| !v.label.is_empty() && v.label == v.name)
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
                    label: String::new(),
                    payload_type: String::new(),
                    fields: vec![],
                },
                VariantFacet {
                    name: "Some".into(),
                    discr: None,
                    label: "Some".into(),
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

    /// A single payload-carrying variant keeps its name; two that overlay the
    /// same bytes do not, and `names_any_variant` is what says which happened.
    #[test]
    fn names_any_variant_tracks_the_suppression() {
        let l = layout();
        assert!(l.names_any_variant(), "`Some` is the only payload variant here");

        let mut r = layout();
        r.type_name = "Result<u64, u64>".into();
        r.variants[0] = VariantFacet {
            name: "Ok".into(),
            discr: Some(0),
            label: "field_0x8".into(),
            payload_type: "Result<u64, u64>::field_0x8".into(),
            fields: vec![("__0".into(), 8)],
        };
        r.variants[1] = VariantFacet {
            name: "Err".into(),
            discr: Some(1),
            label: "field_0x8".into(),
            payload_type: "Result<u64, u64>::field_0x8".into(),
            fields: vec![("__0".into(), 8)],
        };
        assert!(!r.names_any_variant(), "two variants at one offset name neither");
        // The SOURCE names are still on the table -- suppression is a rendering
        // decision, not a loss of what DWARF said.
        assert_eq!(r.facet_for_discr(1).unwrap().name, "Err");
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
