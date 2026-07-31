//! S9 -- Surface rendering & refinement: PrintC/PrintJava, casts, strings, naming.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod comment;
pub mod coreaction_casts;
pub mod printlanguage;
pub mod printc;
pub mod prettyprint;
pub mod printjava;
pub mod cast;
pub mod stringmanage;
pub mod kuna_naming;
pub mod kuna_arraynotation;
pub mod kuna_dedupvardecls;
pub mod kuna_truthycond;
pub mod coreaction_render;
