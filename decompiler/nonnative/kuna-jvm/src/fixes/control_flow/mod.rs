//! Fix family `control_flow` (Java output): if, loop and try/catch structuring
//! of the statement tree the region and structure passes build from the CFG.

pub(crate) mod catch_tail_inline;
pub(crate) mod irreducible_with_exception_edges;
pub(crate) mod loop_first_if_exit;
