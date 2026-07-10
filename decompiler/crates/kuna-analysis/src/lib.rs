//! kuna-analysis: the program-prep loader + analyzer layer.
//!
//! Ghidra's C++ decompiler (ported as `kuna-decomp`) consumes a *prepared*
//! program: functions named, imports resolved, data typed, strings detected.
//! In the Ghidra application that preparation is done by the Java **loader +
//! analyzer** tier (the analyzers that run when you import a binary and click
//! "Run Analysis"). A standalone port does not inherit that tier -- so this
//! crate is its home. See [`docs/missing-analyses.md`](../../../docs/missing-analyses.md)
//! for the gap inventory and port roadmap.
//!
//! # Layering
//!
//! `kuna-analysis` sits *above* `kuna-decomp` so an analysis can read the
//! parsed object (via `kuna-sleigh`'s loader + the `object` crate) AND seed the
//! engine's symbol/type tables (`kuna_decomp::{architecture, database}`):
//!
//! ```text
//! kuna-base / kuna-num -> kuna-sleigh -> kuna-decomp -> kuna-analysis -> kuna-console / kuna-cli
//! ```
//!
//! # Layout
//!
//! - [`loadimage_object`] -- the real-object `LoadImage` backend (ELF/Mach-O/
//!   PE/COFF byte mapping + section/segment/symbol enumeration).
//! - [`loader`] -- image-format markup: PLT/GOT import naming, relocations,
//!   ARM/MIPS mapping markers, the known no-return name lists.
//! - [`analyzers`] -- the metadata analyzers, one module per pass (strings,
//!   dwarf, demangle, entry, protos, addrtable, operand_refs, formatstring,
//!   callfixup, sourcelang, noreturn_disc, noreturn_propagate, aif, pclntab,
//!   fid, objc, rtti, pdb); re-exported flat as `kuna_analysis::<analyzer>`.
//! - [`listing`] -- the deferred whole-image Listing tier (decode/classify/
//!   xref walk) and its consumer queries.
//! - [`pass`] / [`passes`] -- the [`pass::AnalysisPass`] interface (a focused,
//!   additive, never-failing producer of facts) and the default pass roster.
//!
//! # The hand-off contract
//!
//! This crate NEVER calls the per-function decompiler pipeline. Each pass
//! produces an [`pass::AnalysisOutput`] of facts; the console engine stashes
//! them at load (`bootstrap_from_object`) and commits the enabled subset into
//! the engine's `Database`/tables at the `read symbols` boundary
//! (`commit_pending_analysis`). Every pass is a phase-P1 settable option in
//! `phases.toml` (`kuna catalog` lists them; `--option <id> on|off` gates the
//! commit, not the run).
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).

pub mod pass;
pub mod passes;
pub mod loadimage_object;
pub mod loader;
pub mod analyzers;
pub mod listing;

// Flat re-export: analyzer paths stay `kuna_analysis::<analyzer>` (the
// `analyzers/` folder is a layout grouping, not an API namespace).
pub use analyzers::*;
