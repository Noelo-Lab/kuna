//! kuna-harness: dev-only differential helpers for the kuna Rust port.
//!
//! No C++ counterpart -- this crate never ships in the engine. It holds
//! tooling that diffs the Rust crates against the C++ oracle
//! (`decompiler/cpp/`, kept green throughout the port): golden-output
//! comparison, phase-boundary dump diffing, and ad-hoc harnesses for running
//! both engines over the same inputs.
//!
//! Depends on every other workspace crate (kuna-base, kuna-num, kuna-sleigh,
//! kuna-decomp, kuna-console) so any layer can be exercised in isolation.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).
//!
//! ## Modules
//!
//! - [`liftdiff`]: the B2-grade STRUCTURAL differential — a parser for the C++
//!   `print raw` B2 snapshot format, the structural model both engines populate,
//!   and the comparator.
//! - [`corpus`]: the corpus driver around `liftdiff` — the pinned case table, the
//!   `XmlArchitecture` bootstrap, the Rust-side extractor (`FlowInfo` followFlow +
//!   the basic-block builder to the same B2 boundary the C++ snapshot represents),
//!   and the `run_self_test()` aggregate used by BOTH the `tests/lift_diff.rs`
//!   integration test and the `kuna-lift-diff` self-test binary.
//! - [`unittest`]: C++ `test.cc`/`test.hh` — the `UnitTest`/`UnitTestSuite`
//!   framework and the byte-faithful `N/M tests passed.` summary the Python
//!   harness parses off **stderr**, plus `add_exit_code`.
//! - [`testfunction`]: C++ `testfunction.cc`/`testfunction.hh` — the XML
//!   `<decompilertest>` runner (`FunctionTestProperty`, `FunctionTestCollection`,
//!   `run_test_files`) with the `<stringmatch>` min/max + multi-line matching and
//!   the `Success -- `/`FAIL -- `/`Total tests applied = N` stdout grammar.

pub mod corpus;
pub mod liftdiff;
pub mod testfunction;
pub mod unittest;
