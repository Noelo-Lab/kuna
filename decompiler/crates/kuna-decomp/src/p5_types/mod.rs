//! S5 -- Value & type facts: the type system + type inference.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_union;
pub mod kuna_thumbfuncptr;
pub mod kuna_inferfuncentry;
pub mod typeop;
pub mod unionresolve;
pub mod unionresolve_run;
pub mod rangeutil;
pub mod double;
pub mod bitfield;
pub mod constseq;
pub mod prefersplit;
pub mod coreaction_infertypes;
pub mod kuna_memsetsequence;
