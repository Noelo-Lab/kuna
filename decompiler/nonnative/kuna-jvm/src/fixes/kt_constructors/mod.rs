//! Fix family `kt_constructors` (Kotlin output): which methods are
//! constructors, and how they are declared and called.

pub(crate) mod kt_constructor_accessor_call;
pub(crate) mod kt_constructor_without_metadata;
pub(crate) mod kt_data_class_property_parameters;
pub(crate) mod kt_inline_class_constructor_accessor;
pub(crate) mod kt_inner_class_constructor;
