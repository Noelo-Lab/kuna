//! Fix family `generics` (Java output): generic signatures, type variables in
//! scope, diamonds.

pub(crate) mod diamond_argument_conflict;
pub(crate) mod external_class_diamond;
pub(crate) mod static_type_variable_scope;
pub(crate) mod thrown_type_variable;
