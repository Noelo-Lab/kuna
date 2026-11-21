//! Fix family `lambdas`: lambdas and method references: their inference,
//! overloads and recursion.

pub(crate) mod lambda_argument_overload;
pub(crate) mod lambda_pertinence;
pub(crate) mod lambda_return_type_argument;
pub(crate) mod lambda_type_variable_target;
pub(crate) mod self_referencing_lambda;
