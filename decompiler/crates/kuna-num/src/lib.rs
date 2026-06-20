//! kuna-num: numeric semantics of the kuna Rust port.
//!
//! Ports the pure-arithmetic C++ files (`decompiler/cpp/`):
//!
//! - `multiprecision.{hh,cc}` (128/256-bit limb arithmetic)
//! - `float.{hh,cc}` / `double.{hh,cc}` (FloatFormat: host-independent IEEE
//!   emulation and split-double reconstruction)
//! - `CircleRange`, extracted from `rangeutil.{hh,cc}` (circular value-set
//!   domain; the rest of rangeutil's ValueSet machinery lives in kuna-decomp)
//!
//! Integer semantics follow ADR 0003: `uintb -> u64`, `intb -> i64`, all
//! ported arithmetic goes through the wrapping-helper trait, and `calc_mask`
//! truncation semantics are preserved exactly.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).

pub mod float;
pub mod multiprecision;
pub mod opcodes;
pub mod pcoderaw;
pub mod opbehavior;
