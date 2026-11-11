//! kuna fix `kt_default_accessor_flags`: a property accessor whose flags the
//! metadata leaves out has the default accessor flags of its property.
//!
//! kotlinc writes a property's `getter_flags` and `setter_flags` only when
//! they differ from `Flags.getAccessorFlags(hasAnnotations, visibility,
//! modality, false, false, false)` of the property's own flags; Kotlin's
//! reader takes that default when the field is absent. `KProperty.parse`
//! reads the field unconditionally, so an absent one is 0: an `internal`,
//! `final`, default accessor. The setter of a public `var` with a default
//! setter is then printed `internal set`, of a protected one `internal set`
//! too (which kotlinc rejects), of an `open var` `internal final set`. With
//! the default, the accessor has the property's visibility and modality and
//! nothing is printed for it, as the source has nothing.

use crate::kt_flags::Flags;

/// The flags of an accessor: `value` when the metadata has the field
/// (`present`), else the default for a property with `property_flags`.
pub fn accessor_flags(present: bool, value: i32, property_flags: i32) -> i32 {
    if present {
        return value;
    }
    match (
        Flags::VISIBILITY.get(property_flags),
        Flags::MODALITY.get(property_flags),
    ) {
        (Some(visibility), Some(modality)) => Flags::get_accessor_flags(
            Flags::HAS_ANNOTATIONS.get(property_flags),
            visibility,
            modality,
            false,
            false,
            false,
        ),
        _ => value,
    }
}
