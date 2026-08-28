//! Unit tests for the `variantguard` dominating-guard analysis.

use super::*;
use crate::kuna_dwarfvariants::VariantFacet;

/// `Result<u64,u64>`: tag u64 at 0, `Ok` = 0 and `Err` = 1 both overlaying
/// offset 8, so `dwarfvariants` suppressed both labels.
fn result_layout() -> VariantLayout {
    VariantLayout {
        type_name: "core::result::Result<u64, u64>".into(),
        size: 16,
        tag_offset: 0,
        tag_size: 8,
        payload_offset: 8,
        niche: false,
        union_type: "core::result::Result<u64, u64>::payload".into(),
        variants: vec![
            VariantFacet {
                name: "Ok".into(),
                discr: Some(0),
                label: "field_0x8".into(),
                payload_type: "core::result::Result<u64, u64>::field_0x8".into(),
                fields: vec![("__0".into(), 8)],
            },
            VariantFacet {
                name: "Err".into(),
                discr: Some(1),
                label: "field_0x8".into(),
                payload_type: "core::result::Result<u64, u64>::field_0x8".into(),
                fields: vec![("__0".into(), 8)],
            },
        ],
    }
}

/// `Option<&u32>`: `None` = 0, `Some` is the DEFAULT variant (niche).
fn niche_layout() -> VariantLayout {
    VariantLayout {
        type_name: "core::option::Option<&u32>".into(),
        size: 8,
        tag_offset: 0,
        tag_size: 8,
        payload_offset: 0,
        niche: true,
        union_type: "core::option::Option<&u32>".into(),
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
                payload_type: "core::option::Option<&u32>::Some".into(),
                fields: vec![("__0".into(), 0)],
            },
        ],
    }
}

/// `enum Three { A(u32), B(u64), C }` -- three variants, one fieldless, and the
/// two payloads at DISJOINT offsets, so both keep their names.
fn three_layout() -> VariantLayout {
    VariantLayout {
        type_name: "fx::Three".into(),
        size: 16,
        tag_offset: 0,
        tag_size: 4,
        payload_offset: 4,
        niche: false,
        union_type: "fx::Three::payload".into(),
        variants: vec![
            VariantFacet {
                name: "A".into(),
                discr: Some(0),
                label: "A".into(),
                payload_type: "fx::Three::A".into(),
                fields: vec![("__0".into(), 4)],
            },
            VariantFacet {
                name: "B".into(),
                discr: Some(1),
                label: "B".into(),
                payload_type: "fx::Three::B".into(),
                fields: vec![("__0".into(), 8)],
            },
            VariantFacet {
                name: "C".into(),
                discr: Some(2),
                label: String::new(),
                payload_type: String::new(),
                fields: vec![],
            },
        ],
    }
}

/// The pass considers only layouts a variant name was SUPPRESSED on: an enum
/// `dwarfvariants` already names cannot be disturbed by this pass at all.
#[test]
fn only_a_suppressed_layout_is_in_scope() {
    assert!(suppressed(&result_layout()), "Result's Ok/Err are both field_0x8");
    assert!(!suppressed(&niche_layout()), "Option's only payload variant keeps `Some`");
    assert!(!suppressed(&three_layout()), "disjoint payloads keep A and B");
}

/// A fieldless variant gets no union member, so the member indices and the
/// variant indices do not coincide and both directions must skip it.
#[test]
fn union_members_skip_the_fieldless_variant() {
    let t = three_layout();
    assert_eq!(t.union_field_for_variant(0), Some(0)); // A
    assert_eq!(t.union_field_for_variant(1), Some(1)); // B
    assert_eq!(t.union_field_for_variant(2), None); // C is fieldless
    assert_eq!(t.facet_for_union_field(0).unwrap().name, "A");
    assert_eq!(t.facet_for_union_field(1).unwrap().name, "B");
    assert!(t.facet_for_union_field(2).is_none());
    assert!(t.facet_for_union_field(-1).is_none());

    let n = niche_layout();
    assert_eq!(n.union_field_for_variant(0), None, "None carries no payload");
    assert_eq!(n.union_field_for_variant(1), Some(0), "Some is member 0");
    assert_eq!(n.facet_for_union_field(0).unwrap().name, "Some");
}

/// `tag == 0` on a `Result` admits `Ok` on the taken edge and `Err` on the other.
#[test]
fn an_equality_guard_splits_the_two_variants() {
    let l = result_layout();
    let p = TagPred { mask: u64::MAX, cmp: 0, eq: true };
    assert_eq!(facets_admitted(&l, &p), 0b01, "tag == 0 is Ok");
    let mut inv = p;
    inv.eq = false;
    assert_eq!(facets_admitted(&l, &inv), 0b10, "tag != 0 is Err");
}

/// The mask form rustc actually emits (`test al,1`) has to reach the same answer.
#[test]
fn a_masked_guard_splits_the_two_variants() {
    let l = result_layout();
    let p = TagPred { mask: 1, cmp: 0, eq: true };
    assert_eq!(facets_admitted(&l, &p), 0b01, "(tag & 1) == 0 is Ok");
    let mut inv = p;
    inv.eq = false;
    assert_eq!(facets_admitted(&l, &inv), 0b10, "(tag & 1) != 0 is Err");
}

/// The DEFAULT (niche) variant is admitted only where a value the others did not
/// claim satisfies the predicate -- counted over the discriminant domain, not
/// assumed.  `tag == 0` must exclude `Some`.
#[test]
fn the_default_variant_is_counted_not_assumed() {
    let l = niche_layout();
    let eq0 = TagPred { mask: u64::MAX, cmp: 0, eq: true };
    assert_eq!(facets_admitted(&l, &eq0), 0b01, "tag == 0 is None alone");
    let mut ne0 = eq0;
    ne0.eq = false;
    assert_eq!(facets_admitted(&l, &ne0), 0b10, "tag != 0 is Some alone");
}

/// A one-bit mask leaves 2^63 values satisfying it, so the default variant is
/// still possible even though an explicit variant also matches.
#[test]
fn a_wide_predicate_keeps_the_default_variant_possible() {
    let l = niche_layout();
    let p = TagPred { mask: 1, cmp: 0, eq: true };
    // `None` (0) satisfies it, and so do 2^63 - 1 other values, which the default
    // variant covers.
    assert_eq!(facets_admitted(&l, &p), 0b11, "ambiguous: refuse rather than pin");
}

/// An unsatisfiable predicate (`cmp` sets a bit the mask drops) admits nothing.
#[test]
fn an_impossible_predicate_admits_nothing() {
    let l = result_layout();
    let p = TagPred { mask: 1, cmp: 2, eq: true };
    assert_eq!(p.population(8), 0);
    assert_eq!(facets_admitted(&l, &p), 0, "no value can satisfy (tag & 1) == 2");
}

/// The population count is exact at every tag width, including the 64-bit edge
/// where `1 << bits` would overflow a u64.
#[test]
fn population_is_exact_at_every_tag_width() {
    // The mask is clamped to the tag's own width before it is counted, so an
    // all-ones mask on a 1-byte tag has exactly one solution, not zero.
    let eq = TagPred { mask: u64::MAX, cmp: 3, eq: true };
    assert_eq!(eq.population(1), 1);
    let eq1 = TagPred { mask: 0xff, cmp: 3, eq: true };
    assert_eq!(eq1.population(1), 1);
    let ne1 = TagPred { mask: 0xff, cmp: 3, eq: false };
    assert_eq!(ne1.population(1), 255);
    let eq8 = TagPred { mask: u64::MAX, cmp: 1, eq: true };
    assert_eq!(eq8.population(8), 1);
    let ne8 = TagPred { mask: u64::MAX, cmp: 1, eq: false };
    assert_eq!(ne8.population(8), (1u128 << 64) - 1);
}

/// A constant store must COVER the discriminant, and the value read out of it is
/// the tag's own bytes -- not the whole stored word.
#[test]
fn a_constant_store_must_cover_the_tag() {
    let l = result_layout();
    assert_eq!(tag_slice(&l, 0, 8, 1, false), Some(1));
    assert_eq!(tag_slice(&l, 0, 4, 1, false), None, "a 4-byte store misses half the tag");
    assert_eq!(tag_slice(&l, 8, 8, 1, false), None, "the payload is not the tag");

    let t = three_layout(); // tag u32 at 0
    assert_eq!(tag_slice(&t, 0, 4, 2, false), Some(2));
    assert_eq!(tag_slice(&t, 0, 8, 0x0000_0007_0000_0002, false), Some(2), "low word is the tag");
    assert_eq!(tag_slice(&t, 4, 4, 0, false), None);
}

/// The discriminant a store writes selects a variant by value, falling to the
/// DEFAULT variant for a value nothing claimed -- and to nothing at all when
/// there is no default.
#[test]
fn a_stored_discriminant_selects_its_variant() {
    let l = result_layout();
    assert_eq!(variant_index_for_discr(&l, 0), Some(0));
    assert_eq!(variant_index_for_discr(&l, 1), Some(1));
    assert_eq!(variant_index_for_discr(&l, 7), None, "no default variant on a Result");

    let n = niche_layout();
    assert_eq!(variant_index_for_discr(&n, 0), Some(0), "None");
    assert_eq!(variant_index_for_discr(&n, 0xdead_beef), Some(1), "the niche default is Some");
}
