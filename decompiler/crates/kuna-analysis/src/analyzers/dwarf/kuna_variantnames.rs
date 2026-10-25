//! (kuna `variantnames`) Synthesize an enum type for a recovered variant
//! part's `tag` field, so discriminant constants render as variant names.
//!
//! The variant-part importer (`kuna_dwarfvariants`) knows every
//! `DW_AT_discr_value` and every variant name, but installs the tag member
//! with the plain integer type DWARF declares for it — so the emitted C tests
//! `(*e).tag == 1` where the compiler said `1` selects `Err`. This module
//! builds the namemap the importer already holds into a kuna enum of the
//! tag's own width (`TYPE_ENUM_UINT` + namemap, exactly what the C-enum
//! import produces), and the existing enum-constant rendering does the rest:
//! `(*e).tag == Err`.
//!
//! What it deliberately does NOT do:
//! - **Niche layouts have no tag field** (the discriminant lives inside the
//!   payload bytes), so there is nothing to retype; an `Option<&T>` null test
//!   keeps today's rendering. Choosing the facet a niche value selects is the
//!   `match`-renderer's job, on the same side table.
//! - **A tag that is already an enum** (DWARF declared one) keeps it — the
//!   compiler's own vocabulary outranks a synthesized copy.
//! - **The DEFAULT variant** (no `DW_AT_discr_value`) gets no member: it is
//!   selected by every value the others did not claim, and a namemap entry
//!   needs one value. Its constants keep their numeric rendering.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::kuna_dwarfvariants::VariantFacet;

/// The discriminant→name map for the synthesized enum: every NAMED facet with
/// a concrete `DW_AT_discr_value`, masked to the tag's width (the key form the
/// printer looks a constant up under — the `build_enum` rule). First facet at
/// a value wins; a duplicate is an alias the printer could not disambiguate.
pub(super) fn tag_namemap(facets: &[VariantFacet], tag_size: int4) -> BTreeMap<u64, String> {
    let mask: u64 = if tag_size >= 8 { u64::MAX } else { (1u64 << (tag_size * 8)) - 1 };
    let mut nmap = BTreeMap::new();
    for f in facets {
        let Some(v) = f.discr else { continue };
        if f.name.is_empty() {
            continue;
        }
        nmap.entry(v & mask).or_insert_with(|| f.name.clone());
    }
    nmap
}

/// The type to install on the `tag` member: a synthesized enum named
/// `<enum>::Discriminant` carrying [`tag_namemap`], or `None` to keep the
/// DWARF-declared integer (gate off, tag already an enum, no named value, or
/// a same-named foreign type owns the name — never fight one).
pub(super) fn synthesized_tag_enum(
    types: &dyn TypeFactory,
    enum_name: &str,
    tag_type: &Rc<Datatype>,
    facets: &[VariantFacet],
) -> Option<Rc<Datatype>> {
    if !kuna_decomp::kuna_variantnames::variantnames_enabled() || tag_type.is_enum_type() {
        return None;
    }
    let tag_size = tag_type.get_size();
    if tag_size <= 0 {
        return None;
    }
    let nmap = tag_namemap(facets, tag_size);
    if nmap.is_empty() {
        return None;
    }
    // `enum_name` is already collision-resolved by the importer, so the only
    // way this name exists is a re-imported compilation unit: reuse on an
    // exact shape match (the `build_enum` rule), stand down otherwise.
    let name = format!("{enum_name}::Discriminant");
    if let Ok(Some(existing)) = types.find_by_name(&name) {
        return (existing.is_enum_type() && existing.get_size() == tag_size).then_some(existing);
    }
    let ct = types.get_type_enum_sized(&name, tag_size, type_metatype::TYPE_ENUM_UINT).ok()?;
    types.set_enum_values(&ct, nmap).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facet(name: &str, discr: Option<u64>) -> VariantFacet {
        VariantFacet {
            name: name.to_string(),
            discr,
            label: String::new(),
            payload_type: String::new(),
            fields: Vec::new(),
        }
    }

    #[test]
    fn namemap_masks_skips_default_and_keeps_first() {
        let facets = [
            facet("Ok", Some(0)),
            facet("Err", Some(1)),
            facet("None", None),            // default variant: no single value
            facet("", Some(2)),             // nameless: nothing to print
            facet("Alias", Some(0x1_0000_0001)), // masked to 1 at width 4 -> Err wins
        ];
        let m = tag_namemap(&facets, 4);
        assert_eq!(m.len(), 2);
        assert_eq!(m[&0], "Ok");
        assert_eq!(m[&1], "Err");
        // width 8: the alias keeps its own key.
        let m8 = tag_namemap(&facets, 8);
        assert_eq!(m8[&0x1_0000_0001], "Alias");
        // a one-byte tag masks to its width.
        let m1 = tag_namemap(&[facet("X", Some(0x101))], 1);
        assert_eq!(m1[&1], "X");
    }
}
