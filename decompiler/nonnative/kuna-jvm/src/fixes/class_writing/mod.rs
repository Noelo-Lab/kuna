//! Fix family `class_writing` (Java output): class headers, enum bodies,
//! records and imports.

pub(crate) mod compact_record_constructor;
pub(crate) mod enum_body_separator;
pub(crate) mod enum_constant_in_values_array;
pub(crate) mod external_inherited_member_type;
pub(crate) mod nonpublic_nested_import;
pub(crate) mod override_inaccessible_method;
pub(crate) mod sealed_private_permits;
