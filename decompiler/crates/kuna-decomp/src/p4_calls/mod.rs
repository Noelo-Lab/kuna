//! S4 -- Call & prototype model.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_callsite;
pub mod fspec;
pub mod modelrules;
pub mod coreaction_protos;
pub mod kuna_callsitestackargs;
pub mod kuna_dfunaffected;
pub mod kuna_noreturnretuse;
pub mod kuna_returnpair;
pub mod kuna_retinputhalf;
pub mod kuna_returnuncomputed;
pub mod kuna_spillargtrial;
pub mod kuna_rustabi; // (kuna) the rustc two-register return: keep the pair, connect it at the call
pub mod kuna_langabi; // (kuna) the ABI seam: per-language `extern` rendering
