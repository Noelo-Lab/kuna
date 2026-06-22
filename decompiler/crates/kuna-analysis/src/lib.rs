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
//! # Stage-aligned modules
//!
//! Analyses are grouped by the stage they feed (mostly P0/S1, before/around the
//! deep decompiler):
//!
//! - [`loadimage_object`] -- the real-ELF `LoadImage` backend (byte mapping +
//!   section/segment/symbol enumeration; substitutes the C++ console's GNU BFD).
//! - [`s1_loader`] -- S1 loader analyses: PLT/GOT import-name resolution
//!   ([`s1_loader::elf_plt`]); future: `.symtab`/`.dynsym` symbol reader,
//!   no-return detection.
//! - [`s1_sourcelang`] -- source-language / compiler detection
//!   (`detect_compiler`: Gcc/Clang/Rustc/Go), the kuna analog of Ghidra's
//!   `SourceLanguageAnalyzer`; gates Rust-specific behavior (the Rust no-return
//!   wildcard list in [`s1_loader::noreturn`]).
//! - [`pass`] -- the [`pass::AnalysisPass`] interface a new analysis implements
//!   (a focused, additive, never-failing producer of facts), plus the driver
//!   that merges pass outputs before they are committed to the engine.
//!
//! Future stage folders (one per planned analysis; see the roadmap):
//! `s1_strings/` (string-literal detection), `s1_dwarf/` (DWARF), `s1_demangle/`
//! (C++/Rust demangling), `s1_entry/` (function-start discovery), `s1_protos/`
//! (library prototype seeding).
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).

pub mod pass;
pub mod passes;
pub mod loadimage_object;
pub mod s1_loader;
pub mod s1_sourcelang;
pub mod s1_demangle;
pub mod s1_strings;
pub mod s1_protos;
pub mod s1_addrtable;
pub mod s1_entry;
pub mod s1_dwarf;
