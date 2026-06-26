//! S1 FID (Function-ID) fingerprinting — the byte-exact port of Ghidra's
//! FunctionID hashing mechanism.
//!
//! FID re-identifies a function by fingerprinting its instruction stream with an
//! operand-masked FNV-1a64 hash, then matching that fingerprint against a
//! database of known-library hashes — the capability that renames a `FUN_<addr>`
//! in a stripped binary back to (say) `kuna_crc32`. See `docs/fid-design.md` for
//! the full plan; this is **PR2**, the pure hashing core.
//!
//! # PR2 scope (this module set) — the mechanism, no wiring
//!
//! - [`hash`] — [`hash::Fnv1a64`] (the FNV-1a64 digest), [`hash::FidHasher`] (the
//!   operand-masked per-instruction fingerprinter), and [`hash::FidHashQuad`]
//!   (the four-field result). A line-faithful, **byte-exact** port of Ghidra's
//!   `MessageDigestFidHasher` / `FNV1a64MessageDigest`.
//! - [`extent`] — the `FunctionBodyFunctionExtentGenerator` analog: the
//!   address-contiguous `[entry, next_function_after(entry))` body clip over the
//!   Listing.
//! - [`skipper`] — the [`skipper::InstructionSkipper`] trait and
//!   [`skipper::X86InstructionSkipper`] (the x86 NOP/alignment skipper).
//!
//! These are pure, independently unit-testable, and **not yet wired** into any
//! pass / DB / option / engine path — that is PR3 (the `.fid` DB format +
//! generator) and PR4 (the `FidPass` + commit/rename + `--option fid` surface).
//! Because nothing here is reachable from a decompilation, PR2 structurally
//! cannot perturb the parity oracles.

pub mod extent;
pub mod hash;
pub mod skipper;
