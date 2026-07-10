//! The metadata/markup analyzers of the program-prep tier — one module per
//! analysis pass (each doubles as its settable-option name in `phases.toml`).
//!
//! Grouped here for layout only; `lib.rs` re-exports them flat, so public
//! paths stay `kuna_analysis::<analyzer>` (e.g. `kuna_analysis::dwarf`).

pub mod addrtable;
pub mod aif;
pub mod callfixup;
pub mod demangle;
pub mod dwarf;
pub mod entry;
pub mod fid;
pub mod formatstring;
pub mod noreturn_disc;
pub mod noreturn_propagate;
pub mod objc;
pub mod operand_refs;
pub mod pclntab;
pub mod pdb;
pub mod protos;
pub mod rtti;
pub mod sourcelang;
pub mod strings;
