//! Fix family `kt_suspend` (Kotlin output): suspend functions and lambdas,
//! rebuilt from their state machines.

pub(crate) const CONTINUATION: &str = "kotlin/coroutines/Continuation";

pub(crate) mod kt_coroutine_boxing;
pub(crate) mod kt_for_inline_copy;
pub(crate) mod kt_suspend_call_continuation;
pub(crate) mod kt_suspend_function_type;
pub(crate) mod kt_suspend_impl;
pub(crate) mod kt_suspend_lambda;
pub(crate) mod kt_suspend_state_machine;
pub(crate) mod kt_suspend_without_metadata;
