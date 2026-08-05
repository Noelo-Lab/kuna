//! S1 -- Image & code partition: architecture/loader binding.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod kuna_cppsig;
pub mod sleigh_arch;
pub mod raw_arch;
pub mod xml_arch;
