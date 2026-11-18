//! kuna fix `kt_jvm_field_setter`: a `var` compiled without accessors
//! (`@JvmField`) is written without `private set`.
//!
//! `KProperty` writes `private set` for every non-private `var` without a
//! setter method, taking the missing method for a default private setter,
//! which kotlinc does not generate. A `@JvmField var` has no setter method
//! either: kotlinc compiles it to a field of the property's own visibility
//! and no accessors, and rejects the printed `private set` ("JvmField cannot
//! be applied to a property with a custom accessor"); a recompile without
//! `@JvmField` would turn the field private behind accessors. kotlinc makes a
//! backing field non-private only for such a property (and for a `lateinit`
//! one, whose field takes its setter's visibility), so a non-private field is
//! what tells it from a private setter.

use crate::code_constants::ACC_PRIVATE;
use crate::struct_field::StructField;

/// Whether a `var` without a setter method, with the backing field `field`
/// (`None` when it has none, or `delegated` when it holds the delegate),
/// has no setter at all rather than a private default one.
pub fn has_no_setter(field: Option<StructField>, delegated: bool) -> bool {
    !delegated && field.is_some_and(|f| !f.has_modifier(ACC_PRIVATE))
}
