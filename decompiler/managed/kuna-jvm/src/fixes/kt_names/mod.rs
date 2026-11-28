//! Fix family `kt_names` (Kotlin output): names Kotlin escapes or reserves, and
//! parameter names from the metadata.

pub(crate) mod kt_escaped_class_name;
pub(crate) mod kt_function_parameter_name;
pub(crate) mod kt_java_keyword_variable_name;
pub(crate) mod kt_unescaped_name;
