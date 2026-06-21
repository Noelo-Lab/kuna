//! Infra -- Orchestration, scheduling & framework.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod action;
pub mod architecture;
pub mod decompile_drive;
pub mod capability;
pub mod libdecomp;
pub mod cpool;
pub mod graph;
pub mod callgraph;
pub mod signature;
pub mod analyzesigs;
pub mod paramid;
pub mod universalaction;
