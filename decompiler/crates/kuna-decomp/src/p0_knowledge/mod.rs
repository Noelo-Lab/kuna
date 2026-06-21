//! P0 -- Knowledge & configuration plane: symbol DB, options, overrides, the stage registry.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod options;
pub mod database;
pub mod overrides;
pub mod kuna_stages;
pub mod kuna_assert;
pub mod kuna_restartlog;
