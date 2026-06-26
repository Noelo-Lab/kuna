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
//! # PR3 scope (added) — the `.fid` database + generator
//!
//! - [`db`] — the kuna-native `.fid` format ([`db::FidDb`] / [`db::FidRecord`]):
//!   a flat, full-hash-indexed, version-stamped file modeled field-for-field on
//!   Ghidra's FID `FunctionsTable`+`StringsTable`, behind the [`db::FidDatabase`]
//!   trait so a future `.fidbf` reader (deferred PR6) is a serialization swap.
//!   `load`/`serialize` are robust (never panic on a malformed file).
//! - [`build`] — the `FidServiceLibraryIngest` analog ([`build::build_records`]):
//!   disassembles every named function in a parsed object through the Listing
//!   path, runs the PR2 [`hash::FidHasher`] over its extent (assembling each
//!   instruction's fingerprint from [`Sleigh::instruction_mask`](kuna_sleigh::sleigh::Sleigh::instruction_mask)),
//!   and emits the deduplicated records. Drives the `kuna fid build` CLI
//!   subcommand.
//!
//! Still **not wired** into any pass / option / engine path — that is PR4 (the
//! `FidPass` + commit/rename + `--option fid` surface). The generator is a
//! build-time tool (it produces the fixture `.fid`), never reached from a
//! decompilation, so PR3 structurally cannot perturb the parity oracles.

pub mod build;
pub mod db;
pub mod extent;
pub mod hash;
pub mod skipper;
