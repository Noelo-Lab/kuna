//! Substrate -- shared IR, containers, and rewrite helpers used by every stage.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod varnode;
pub mod op;
pub mod block;
pub mod funcdata;
pub mod funcdata_block;
pub mod funcdata_encode;
pub mod funcdata_op;
pub mod funcdata_printraw;
pub mod funcdata_varnode;
pub mod dtype;
pub mod seams;
pub mod expression;
pub mod transform;
pub mod addtreestate;
