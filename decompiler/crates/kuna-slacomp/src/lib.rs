//! SLEIGH compiler: `.slaspec` source to `.sla` images for `kuna-sleigh`.
//!
//! The scanner and parser drive [`SleighCompile`], which builds and checks the
//! runtime's symbol, pattern and semantic-template types before encoding them.

pub mod consistency;
pub mod encode;
mod local_collisions;
pub mod pcodecompile_actions;
pub mod slgh_compile;
pub mod slghparse;
pub mod slghscan;

// Re-export the primary driver type and CLI entry so the `slacomp` binary (and
// the Python differential harness, via the binary) have a stable surface.
pub use slgh_compile::SleighCompile;
