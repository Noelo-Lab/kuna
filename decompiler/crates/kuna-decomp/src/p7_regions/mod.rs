//! S7 -- Region hierarchy: the angr RegionIdentifier port.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod kuna_securitycheck;
pub mod kuna_stackguard;
pub mod kuna_regiongraph;
pub mod kuna_regionid;
