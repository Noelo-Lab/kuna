//! P6, variables: version splitting and merging, naming and declaration placement.

pub(crate) mod kuna_fix_var_merge_narrowing;
pub(crate) mod kuna_fix_switch_case_scope;
pub(crate) mod kuna_fix_var_merge_liveness;
pub mod kuna_source_locals;
pub(crate) mod kuna_fix_kt_java_keyword_variable_name;
pub mod var_definition_helper;
pub mod var_names_collector;
pub mod var_processor;
pub mod var_version_node;
pub mod var_version_pair;
pub mod var_versions_graph;
pub mod var_versions_processor;
pub(crate) mod kuna_fix_var_merge_declared_type;
pub(crate) mod kuna_fix_local_final_writes;
pub(crate) mod kuna_fix_refresh_printed_var_names;
pub mod kuna_source_scopes;
