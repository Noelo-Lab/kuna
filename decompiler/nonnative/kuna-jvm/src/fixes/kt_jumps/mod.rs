//! Fix family `kt_jumps` (Kotlin output): break, continue and return out of
//! statements, lambdas and `when`.

pub(crate) mod kt_jump_to_non_loop_label;
pub(crate) mod kt_lambda_singleton_returns;
pub(crate) mod kt_return_outside_lambda;
pub(crate) mod kt_unlabeled_sequence_break;
pub(crate) mod kt_when_break;
