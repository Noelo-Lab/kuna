//! S8 -- Structured AST & goto quality: the structuring engine.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod blockaction;
pub mod kuna_gotoreduce;
pub mod kuna_loopbreak_recovery;
pub mod region_structurer;
