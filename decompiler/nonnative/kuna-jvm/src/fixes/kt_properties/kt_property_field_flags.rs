//! kuna fix `kt_property_field_flags`: a Kotlin property is written with the
//! annotations its backing field's JVM flags stand for.
//!
//! Kotlin has no `volatile` or `transient` modifier: `@Volatile`,
//! `@Transient` and `@JvmSynthetic` (on the field) are source-retained
//! annotations kotlinc turns into `ACC_VOLATILE`, `ACC_TRANSIENT` and
//! `ACC_SYNTHETIC` on the field, so the flag is all the class file keeps of
//! them. `KotlinWriter` prints them for a field it writes itself
//! (`appendJvmAnnotations`), but `KProperty` writes a property's annotations
//! from its attributes only, so every property lost them: a recompiled
//! `@Volatile var state` is a plain field, without the visibility and
//! ordering guarantees the code relies on. With the fix the property carries
//! them; on a delegated property they name the delegate's field
//! (`@delegate:Transient`).

use crate::code_constants::{ACC_SYNTHETIC, ACC_TRANSIENT, ACC_VOLATILE};
use crate::struct_field::StructField;

/// The annotations, in source form, that `field`'s flags stand for on its
/// property; `delegated` when `field` holds the property's delegate.
pub fn annotations(field: StructField, delegated: bool) -> Vec<String> {
    let target = if delegated { "delegate:" } else { "" };
    [
        (ACC_VOLATILE, "Volatile"),
        (ACC_TRANSIENT, "Transient"),
        (ACC_SYNTHETIC, "JvmSynthetic"),
    ]
    .into_iter()
    .filter(|&(flag, _)| field.has_modifier(flag))
    .map(|(_, name)| format!("@{target}{name}"))
    .collect()
}
