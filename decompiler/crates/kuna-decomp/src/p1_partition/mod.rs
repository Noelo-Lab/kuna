//! S1 -- Image & code partition: architecture/loader binding.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod kuna_cppsig;
pub mod kuna_elfmaindecl; // (kuna) an executable's own main declared from its unlocked analysis
pub mod kuna_formatstring;
pub mod kuna_goreg; // (kuna) name+type the goroutine register as the `g` global (docs/multilang.md W4)
pub mod kuna_litpoolconst;
pub mod sleigh_arch;
pub mod raw_arch;
pub mod xml_arch;
