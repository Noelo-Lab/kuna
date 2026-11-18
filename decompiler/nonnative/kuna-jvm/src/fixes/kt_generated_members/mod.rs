//! Fix family `kt_generated_members` (Kotlin output): members kotlinc generates
//! that the source does not declare.

pub(crate) mod kt_companion_static_copy;
pub(crate) mod kt_default_impls_delegate;
pub(crate) mod kt_file_class_constructor;
pub(crate) mod kt_generated_constructor_hidden;
pub(crate) mod kt_generated_overload_hidden;
pub(crate) mod kt_interface_companion_field;
