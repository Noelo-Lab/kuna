//! Fix family `constructor_calls` (Java output): outer instances, qualified
//! `super` calls and split constructor calls.

pub(crate) mod implicit_outer_instance;
pub(crate) mod qualified_super_call;
pub(crate) mod split_constructor_call;
pub(crate) mod synthetic_null_check_shape;
