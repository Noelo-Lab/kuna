//! Fix family `var_merge` (Java output): merging the variables of one slot
//! (`VarDefinitionHelper::merge_vars`) and the checks a merge must pass.

pub(crate) mod var_merge_declared_type;
pub(crate) mod var_merge_liveness;
pub(crate) mod var_merge_narrowing;
