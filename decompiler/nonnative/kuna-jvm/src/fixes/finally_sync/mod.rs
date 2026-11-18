//! Fix family `finally_sync` (Java and Kotlin output): `finally` and
//! `synchronized` blocks.

pub(crate) mod finally_exit_versions;
pub(crate) mod inline_marker_finally_copy;
pub(crate) mod kt_semaphore_store_liveness;
pub(crate) mod protected_back_edge;
pub(crate) mod synchronized_break_exit;
pub(crate) mod synchronized_lock_assignment;
