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
pub mod kuna_declhightype;
pub mod kuna_castsign; // (kuna) signed declarations for frame locals and the readers signedness vetoes
pub mod kuna_typeround; // (kuna) the declared-signedness rounding decision
pub mod kuna_dedupvardecls;
pub mod kuna_paramrefdecl;
pub mod kuna_truthycond;
pub mod kuna_braceelide;
pub mod kuna_warnstyle;
pub mod kuna_arraycoverwidth;
pub mod kuna_emptystrconst;
pub mod kuna_truncarg;
pub mod kuna_castimplied; // (kuna) casts C's own conversions already perform
pub mod kuna_castternary; // (kuna) a conditional arm keeps no cast the conditional performs
pub mod kuna_castarith; // (kuna) pointer arithmetic stays in pointer terms
pub mod kuna_structdefs;
pub mod kuna_globalref; // (kuna) a constant address used as a pointer prints as its global
pub mod kuna_lang; // (kuna) the output-language plane: profile + capabilities
pub mod kuna_langtypes; // (kuna) the type-spelling seam (TypeSpeller + SpellCtx)
pub mod kuna_langc; // (kuna) the c-language policy objects (CSpeller)
pub mod kuna_langrust; // (kuna) the rust-language policy objects (profile + caps)
pub mod kuna_rusttypes; // (kuna) the rust-language type speller
pub mod kuna_ctypes; // (kuna) valid per-architecture C spelling of the core types
pub mod coreaction_render;
