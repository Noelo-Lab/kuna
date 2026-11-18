//! Fix family `kt_default_args` (Kotlin output): default arguments: the
//! `$default` methods and constructors kotlinc generates, and calls through them.

pub(crate) mod kt_default_arg_assigned_param;
pub(crate) mod kt_default_args_bridge_lookup;
pub(crate) mod kt_default_args_call;
pub(crate) mod kt_default_value_coercion_cast;
pub(crate) mod kt_default_value_names;
