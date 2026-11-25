//! The deobfuscation plane.
//!
//! Obfuscation is undone at the stage whose representation can undo it, so
//! this is not one pass: it is a set of deobfuscators, each acting at the
//! stage hooks it needs, and one contract every later stage reads.  The hooks
//! and the registry are in [`kuna_deobf`]; the contract is in [`kuna_body`].
//!
//! * The plane: [`kuna_deobf`] (stage hooks, registry), [`kuna_body`] (what a
//!   recovered body tells the stages after it).
//! * The symbolic evaluation kit, which depends on nothing else in the
//!   decompiler: [`kuna_symexpr`], [`kuna_symstate`], [`kuna_symemu`].
//! * Techniques: the VM devirtualizer, `deobfuscate devirtualize`, a port of
//!   Pushan (NDSS 2027) ([`kuna_devirtualize`],
//!   [`kuna_devirtflow`], [`kuna_devirtemit`], [`kuna_residuerules`]).

pub mod kuna_deobf; // (kuna deobf) stage hooks and the deobfuscator registry
pub mod kuna_body; // (kuna deobf) a recovered body's facts and analysis policy
pub mod kuna_symexpr; // (kuna deobf) symbolic expression arena
pub mod kuna_symstate; // (kuna deobf) byte-granular abstract registers and memory
pub mod kuna_symemu; // (kuna deobf) symbolic p-code emulator
pub mod kuna_devirtflow; // (kuna devirtualize) VPC-sensitive flat-CFG exploration
pub mod kuna_devirtemit; // (kuna devirtualize) flat CFG -> synthetic p-code overlay
pub mod kuna_devirtualize; // (kuna devirtualize) technique `devirtualize`: VM-protected functions (port of Pushan, NDSS 2027)
pub mod kuna_residuerules; // (kuna devirtualize) arithmetic residue of a devirtualized body
