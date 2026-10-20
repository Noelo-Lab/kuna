//! S9 -- Surface rendering & refinement: PrintC/PrintJava, casts, strings, naming.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod comment;
pub mod coreaction_casts;
pub mod printlanguage;
pub mod printc;
pub mod kuna_partialconcat;
pub mod kuna_partialaccess;
pub mod kuna_widelocal;
pub mod prettyprint;
pub mod printjava;
pub mod cast;
pub mod kuna_unknowncasts;
pub mod kuna_signedread; // (kuna) a signed read of an expression that prints unsigned casts it back
pub mod kuna_unsignedread; // (kuna) an unsigned read of an expression C evaluates as signed casts it
pub mod kuna_narrowselector; // (kuna) a switch on a narrow expression casts it to its own width
pub mod stringmanage;
pub mod kuna_naming;
pub mod kuna_varsources;
mod kuna_calltransport;
pub mod kuna_arraynotation;
pub mod kuna_declhightype;
pub mod kuna_castsign; // (kuna) signed declarations for frame locals and the readers signedness vetoes
pub mod kuna_typeround; // (kuna) the declared-signedness rounding decision
pub mod kuna_dedupvardecls;
pub mod kuna_paramrefdecl;
pub mod kuna_addressdecl;
pub mod kuna_callerstack;
pub mod kuna_bitcast;
pub(crate) mod kuna_floatslice;
pub(crate) mod kuna_memberbase;
pub mod kuna_stackmem;
pub mod kuna_stackassign;
pub mod kuna_stackbytes;
pub mod kuna_stacklayout;
pub mod kuna_truthycond;
pub mod kuna_braceelide;
pub mod kuna_labelstmt; // (kuna) every printed C label labels a statement
pub mod kuna_warnstyle;
pub mod kuna_infloopstyle;
pub mod kuna_arraycoverwidth;
pub mod kuna_bytearrayc;
pub mod kuna_armregistershift; // (kuna) defined C for ARM register-controlled LSL
mod kuna_armconstantshift;
pub mod kuna_emptystrconst;
pub mod kuna_truncarg;
pub mod kuna_pointerargs; // (kuna) a scalar's address passed to a character-pointer parameter
pub mod kuna_castimplied; // (kuna) casts C's own conversions already perform
pub mod kuna_castnarrow; // (kuna) a narrowing C's own assignment, argument or return conversion performs keeps no cast
pub mod kuna_declorder; // (kuna) the order local declarations print in
pub mod kuna_castternary; // (kuna) a conditional arm keeps no cast the conditional performs
pub mod kuna_castwiden; // (kuna) a 64-bit widening C performs by itself keeps no cast
pub mod kuna_castarith; // (kuna) pointer arithmetic stays in pointer terms
pub mod kuna_structdefs;
pub mod kuna_namedglobals;
pub mod kuna_globalref; // (kuna) a constant address used as a pointer prints as its global
pub mod kuna_namespaces; // (kuna) which namespaces print in front of a name
pub mod kuna_lang; // (kuna) the output-language plane: profile + capabilities
pub mod kuna_langtypes; // (kuna) the type-spelling seam (TypeSpeller + SpellCtx)
pub mod kuna_langc; // (kuna) the c-language policy objects (CSpeller)
pub mod kuna_langrust; // (kuna) the rust-language policy objects (profile + caps)
pub mod kuna_rusttypes; // (kuna) the rust-language type speller
pub mod kuna_ctypes; // (kuna) valid per-architecture C spelling of the core types
pub mod coreaction_render;
pub mod kuna_srcmap; // (kuna) the token-level source map of a rendered function

pub(crate) mod kuna_stackmarkers;
pub(crate) mod kuna_stackversions;

pub(crate) mod kuna_indirectargs;

pub mod kuna_callprototypes;
