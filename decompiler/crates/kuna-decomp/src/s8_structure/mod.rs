//! S8 -- Structured AST & goto quality: the structuring engine.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod blockaction;
pub mod kuna_crossjumpreverter;
pub mod kuna_dedupitetail;
pub mod kuna_gotoreduce;
pub mod kuna_ifelseflatten;
pub mod kuna_iteregion;
pub mod kuna_loopbreak_recovery;
pub mod kuna_earlyreturn;
pub mod kuna_returndup;
pub mod kuna_switchreturn;
pub mod kuna_taildup;
pub mod region_structurer;
