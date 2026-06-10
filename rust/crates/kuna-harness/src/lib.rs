//! kuna-harness: dev-only differential helpers for the kuna Rust port.
//!
//! No C++ counterpart -- this crate never ships in the engine. It holds
//! tooling that diffs the Rust crates against the C++ oracle
//! (`decompiler/cpp/`, kept green throughout the port): golden-output
//! comparison, stage-boundary dump diffing, and ad-hoc harnesses for running
//! both engines over the same inputs.
//!
//! Depends on every other workspace crate (kuna-base, kuna-num, kuna-sleigh,
//! kuna-decomp, kuna-console) so any layer can be exercised in isolation.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).
