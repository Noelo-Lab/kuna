//! Fix family `kt_delegation_calls` (Kotlin output): a constructor's superclass
//! or `this(..)` call, and the code after it.

pub(crate) mod kt_constructor_body_after_first_block;
pub(crate) mod kt_default_args_super_call;
pub(crate) mod kt_delegation_call_arguments;
pub(crate) mod kt_hidden_empty_super_call;
pub(crate) mod kt_supertype_type_arguments;
