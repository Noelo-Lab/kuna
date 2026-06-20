//! kuna-console: console front-end of the kuna Rust port.
//!
//! Ports the interactive/test command surface (`decompiler/cpp/`):
//!
//! - `interface.{hh,cc}` (IfaceStatus command dispatch)
//! - `ifaceterm.{hh,cc}` (terminal handling)
//! - `ifacedecomp.{hh,cc}` (every `decomp_dbg` console command)
//! - `testfunction.{hh,cc}` (the XML datatest runner)
//! - the C-declaration parser behind `parse line` (`grammar.{hh,cc}`,
//!   hand-ported from the bison grammar)
//!
//! Binaries: `decomp_dbg` and `decomp_test_dbg` (exact upstream names), so
//! the XML datatests under `tests/datatests/` and the Python harness
//! (`kuna/run_tests.py`, `kuna/decompile.py`) drive the Rust engine through
//! the identical command surface, unchanged.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).

pub mod interface;
pub mod ifaceterm;
pub mod ifacedecomp;
pub mod engine;
pub mod codedata;
pub mod kuna_console;
pub mod grammar;
pub mod rulecompile;
pub mod unify;
