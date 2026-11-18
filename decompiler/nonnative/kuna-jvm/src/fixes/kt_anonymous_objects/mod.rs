//! Fix family `kt_anonymous_objects` (Kotlin output): object expressions,
//! callable references and local classes.

pub(crate) mod kt_object_super_call;
pub(crate) mod kt_property_reference;
pub(crate) mod kt_reference_anonymous_super_call;
pub(crate) mod kt_reused_inline_object;
pub(crate) mod kt_synthetic_anonymous_object;
pub(crate) mod kt_unrepresentable_declared_type;
