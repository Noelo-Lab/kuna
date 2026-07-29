//! S4 -- Call & prototype model.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_callsite;
pub mod fspec;
pub mod modelrules;
pub mod coreaction_protos;
pub mod kuna_returnpair;
pub mod kuna_returnuncomputed;
