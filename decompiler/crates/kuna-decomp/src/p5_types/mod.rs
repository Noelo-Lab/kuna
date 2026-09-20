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
pub mod kuna_rodatastring;
pub mod kuna_ptrdepth;
pub mod kuna_codescalar;
pub mod kuna_boolbyte; // (kuna) type a byte that is only ever a truth value as bool
pub mod kuna_charbyte; // (kuna) keep char for a byte loaded through a char pointer
pub mod kuna_charptr; // (kuna) commit a pointer used only on characters to char *
pub mod kuna_ptrfromuse; // (kuna) type a dereferenced-only function input as a pointer
pub mod kuna_pebnames; // (kuna) type the Windows TEB segment base so PEB/TEB field reads are named
pub mod kuna_structsynth; // (kuna) synthesize a struct type from a pointer parameter's constant-offset dereferences
