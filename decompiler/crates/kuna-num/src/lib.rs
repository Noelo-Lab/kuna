//! Numeric semantics and raw p-code primitives, based on Ghidra's decompiler.
//!
//! [`multiprecision`] provides arithmetic over little-endian limbs; [`float`]
//! handles floating-point encodings and arithmetic. [`opcodes`] defines opcode
//! identities and names, [`opbehavior`] evaluates scalar operations, and
//! [`pcoderaw`] stores varnodes and raw operations.
//!
//! Circular ranges and split-varnode analysis live in `kuna-decomp`.

pub mod float;
pub mod multiprecision;
pub mod opcodes;
pub mod pcoderaw;
pub mod opbehavior;
