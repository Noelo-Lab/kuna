//! Fix family `stack_var_inlining` (Java and Kotlin output): which values the
//! stack-variable inliner copies into their uses, and when.

pub(crate) mod field_read_all_paths;
pub(crate) mod field_read_reuse;
pub(crate) mod kt_field_read_reuse;
pub(crate) mod kt_inline_evaluation_order;
pub(crate) mod kt_inline_heap_read;
pub(crate) mod kt_inline_lock_local;
pub(crate) mod kt_partial_inline;
