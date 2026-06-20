//! kuna Rust port: the SLEIGH compiler (`sleigh_opt` / `slacomp`).
//!
//! This crate compiles SLEIGH specifications (`.slaspec` text) into the binary
//! `.sla` form that `kuna-sleigh`'s decoder *consumes*.  It is the last C++
//! dependency in the kuna tree; porting it lets `.sla` be built without C++.
//!
//! # The oracle
//!
//! C++ `sleigh_opt` is byte-deterministic, so the gold gate is the strongest
//! differential test: for every vendored `.slaspec`, this compiler's `.sla` must
//! be **byte-identical** to `sleigh_opt`'s output (`python -m kuna.slacomp --all`).
//!
//! # C++ -> module map (1:1 with the `SLACOMP` link group)
//!
//! | C++ source            | Rust module                  | Wave |
//! |-----------------------|------------------------------|------|
//! | `slghscan.l`          | [`slghscan`]                 | WS1  |
//! | `slghparse.y`         | [`slghparse`]                | WS2  |
//! | `pcodecompile.cc` (+) | [`pcodecompile_actions`]     | WS3  |
//! | `slgh_compile.cc`     | [`slgh_compile`]             | WS4  |
//! | encode orchestration  | [`encode`]                   | WS5  |
//!
//! The consumer-side symbol/pattern/semantics/encode types are **already ported**
//! in `kuna-sleigh` and are *reused*, not re-ported: the compiler BUILDS those
//! structures (via the parser actions) and ENCODES them (via their existing
//! `encode(...)` methods).  See `docs/rust-port/sleigh-compiler/map.md`.
//!
//! Module-file ownership is deliberately disjoint so the fleet's file-disjoint
//! porter agents never collide:
//! - WS1 owns `slghscan.rs`
//! - WS2 owns `slghparse.rs`
//! - WS3 owns `pcodecompile_actions.rs`
//! - WS4 owns `slgh_compile.rs`
//! - WS5 owns `encode.rs`

pub mod consistency;
pub mod encode;
pub mod pcodecompile_actions;
pub mod slgh_compile;
pub mod slghparse;
pub mod slghscan;

// Re-export the primary driver type and CLI entry so the `slacomp` binary (and
// the Python differential harness, via the binary) have a stable surface.
pub use slgh_compile::SleighCompile;
