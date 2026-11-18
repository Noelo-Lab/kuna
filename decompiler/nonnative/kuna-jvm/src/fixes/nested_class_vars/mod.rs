//! Fix family `nested_class_vars` (Java output): captured values and variable
//! names in nested classes and lambdas.

pub(crate) mod captured_var_clash_name;
pub(crate) mod captured_var_through_cast;
pub(crate) mod lambda_catch_var_rename;
pub(crate) mod nested_param_rename;
pub(crate) mod refresh_printed_var_names;
