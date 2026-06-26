//! S1 FID (Function-ID) fingerprinting — the byte-exact port of Ghidra's
//! FunctionID hashing mechanism.
//!
//! FID re-identifies a function by fingerprinting its instruction stream with an
//! operand-masked FNV-1a64 hash, then matching that fingerprint against a
//! database of known-library hashes — the capability that renames a `FUN_<addr>`
//! in a stripped binary back to (say) `kuna_crc32`. See `docs/fid-design.md` for
//! the full plan.
//!
//! # Module set
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
//! - [`db`] — the kuna-native `.fid` format ([`db::FidDb`] / [`db::FidRecord`]):
//!   a flat, full-hash-indexed, version-stamped file modeled field-for-field on
//!   Ghidra's FID `FunctionsTable`+`StringsTable`, behind the [`db::FidDatabase`]
//!   trait so a future `.fidbf` reader (deferred PR6) is a serialization swap.
//! - [`build`] — the `FidServiceLibraryIngest` analog ([`build::build_records`]):
//!   disassembles every named function in a parsed object through the Listing
//!   path, runs the [`hash::FidHasher`] over its extent, and emits the
//!   deduplicated records. Drives the `kuna fid build` CLI subcommand.
//!
//! # PR4 (this module) — the gated matching pass
//!
//! [`FidPass`] is the Listing/xref **consumer** (structurally cloned from
//! [`crate::s1_noreturn_disc::NoReturnDiscoveredPass`]): over the built Listing it
//! hashes every function with the byte-exact [`hash::FidHasher`] (driving the PR1
//! [`Sleigh::instruction_mask`](kuna_sleigh::sleigh::Sleigh::instruction_mask)),
//! looks the full hash up in a `.fid` database, and — when the bucket collapses to
//! exactly one name ([`pick_unique_name`], conservative; never guesses on a tie) —
//! emits a [`crate::pass::FidMatch`]. The engine's commit seam then **renames** the
//! function (only if it still carries an engine `FUN_*`/`sub_*` placeholder — the
//! label gate). Default-OFF, real-ELF path only, inert without the Listing AND
//! without a DB — so the parity oracles are structurally untouched.
//!
//! The database path is read from the `kuna_fid_db` environment variable (mirroring
//! how the i386-PIE-PLT loader gate reads `kuna_i386_pie_plt`); a real `--fidb
//! <path>` flag is a deferred follow-on (`docs/fid-design.md` §7.2). Absent / empty
//! / unreadable ⇒ the pass is a no-op.

pub mod build;
pub mod db;
pub mod extent;
pub mod hash;
pub mod skipper;

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Stage};
use crate::s1_fid::db::{FidDatabase, FidDb, FidRecord};
use crate::s1_fid::hash::{FidHashQuad, FidHasher, InsnFingerprint, NoRelocations};
use crate::s1_fid::skipper::{InstructionSkipper, X86InstructionSkipper};

/// The environment variable naming the `.fid` fingerprint database to match
/// against. Mirrors the loader-tier `kuna_i386_pie_plt` env gate: the DB source is
/// kept out of the per-decompilation `--option` surface in v1 (a real `--fidb
/// <path>` flag is deferred, `docs/fid-design.md` §7.2). Absent / empty ⇒ the pass
/// is inert (no DB to match against).
pub const FID_DB_ENV: &str = "kuna_fid_db";

/// The FID matching consumer pass — the kuna analog of Ghidra's FID identification
/// analyzer (`FidServiceImpl` over the function-id query service), the third
/// Listing/xref consumer.
///
/// Default-OFF (gate id `fid`). Doubly inert by default: it short-circuits to an
/// empty output when no Listing is built (`ctx.listing.is_none()`, the
/// `--option listing on` flag is off) AND when no `.fid` database is configured
/// (`FID_DB_ENV` unset / empty / unreadable). Every emitted [`FidMatch`] is a
/// *rename* fact the engine commit seam applies only to an engine `FUN_*`/`sub_*`
/// placeholder (the label gate) — a real `.symtab`/DWARF name is never overwritten.
#[derive(Default)]
pub struct FidPass;

impl AnalysisPass for FidPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "fid"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Listing-dependent: a no-op when the Listing is absent (the `--option
        // listing on` flag is off). Keeps the pass inert by default even if its own
        // gate is flipped on without the Listing.
        let Some(listing) = ctx.listing else {
            return out;
        };
        // DB-dependent: a no-op without a configured, loadable `.fid` database
        // (`kuna_fid_db` unset / empty / unreadable / bad bytes). The DB carries the
        // language id the hashes were computed for; matching across languages is not
        // meaningful, so the absence of a DB is the inert path (faithful to Ghidra,
        // which only queries a language-matched FID DB).
        let Some(db) = load_fid_db(ctx) else {
            return out;
        };

        // The per-architecture skipper set: x86 ⇒ the NOP/alignment skipper
        // (matching the generator); non-x86 ⇒ empty (never skips). Must match the
        // skipper set the DB was built with or the hashes diverge.
        let is_x86 = ctx.arch.archid.starts_with("x86:");

        let Some(code_space) = ctx.arch.manage().get_default_code_space() else {
            return out;
        };
        let code_space = Rc::clone(code_space);
        let translate = ctx.arch.translate();

        // The relocation oracle: a fully-linked image (the stripped `-static
        // -no-pie` re-identification target) carries no dynamic relocations over its
        // text, so `NoRelocations` is correct — and it matches the build side, which
        // sees no relocations for a self-contained body (the full hash is identical
        // either way; see `hash.rs`'s relocation test). A future PIE/relocatable
        // lookup path can supply an `elf_reloc`-backed oracle here.
        let relocs = NoRelocations;

        for (&entry, _f) in listing.functions() {
            // §4.3 extent: the address-contiguous `[entry, next)` clip over the
            // Listing's decoded instructions (ascending).
            let extent = extent::calculate_extent(listing, entry);
            if extent.len() < hash::SHORT_HASH_CODE_UNIT_LENGTH as usize {
                continue;
            }

            // Assemble each instruction's fingerprint via the PR1 SLEIGH mask —
            // the SAME projection the generator (`build::fingerprint_from_mask`)
            // uses, so a function fingerprints identically whether built into the DB
            // or matched against it. A decode failure on any instruction drops the
            // whole function (a partially-masked body would mis-hash).
            let mut fps: Vec<InsnFingerprint> = Vec::with_capacity(extent.len());
            let mut decode_ok = true;
            for insn in &extent {
                let addr = Address::new(Rc::clone(&code_space), insn.addr);
                match translate.instruction_mask(&addr) {
                    Ok(mask) => fps.push(build::fingerprint_from_mask(
                        insn.addr,
                        insn.len,
                        insn.flow.is_call,
                        mask,
                    )),
                    Err(_) => {
                        decode_ok = false;
                        break;
                    }
                }
            }
            if !decode_ok {
                continue;
            }

            let skippers: Vec<Box<dyn InstructionSkipper>> = if is_x86 {
                vec![Box::new(X86InstructionSkipper)]
            } else {
                vec![]
            };
            let mut hasher = FidHasher::new(skippers);
            let Some(quad) = hasher.hash(&fps, &relocs) else {
                continue;
            };

            // The only hot-path query: the full-hash bucket (§5.1 / §6.1).
            let recs = db.find_by_full_hash(quad.full_hash);
            if let Some(name) = pick_unique_name(recs, &quad) {
                out.fid_names.push(FidMatch { addr: entry, name });
            }
        }

        out
    }
}

/// Resolve + load the `.fid` database named by [`FID_DB_ENV`]. `None` (the inert
/// path) when the variable is unset / empty, the file is unreadable, or its bytes
/// are not a valid `.fid` — every failure is a silent no-op (FID never fails a
/// decompilation, faithful to the analysis-pass contract).
fn load_fid_db(_ctx: &AnalysisCtx) -> Option<FidDb> {
    let path = std::env::var_os(FID_DB_ENV)?;
    if path.is_empty() {
        return None;
    }
    let bytes = std::fs::read(PathBuf::from(path)).ok()?;
    FidDb::load(&bytes).ok()
}

/// §6.1 — exact full-hash match → a unique name.
///
/// Returns a name ONLY when the full-hash bucket collapses to exactly one distinct
/// name (one record, or several records that all carry the same name — e.g. the
/// same function fingerprinted under aliased symbols). Genuine multi-name buckets
/// (a real collision or two differently-named functions sharing a full hash) →
/// `None`: the v1 matcher is conservative and **never guesses on a tie**. The
/// specific hash / `code_unit_size` tiebreak is the deferred PR5 seeker's job; v1
/// stores those fields but does not score on them.
fn pick_unique_name(recs: &[FidRecord], _quad: &FidHashQuad) -> Option<String> {
    let first = recs.first()?;
    if recs.iter().all(|r| r.name == first.name) {
        Some(first.name.clone())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_identity() {
        let p = FidPass;
        assert_eq!(p.id(), "fid");
        assert_eq!(p.stage(), Stage::S1);
    }

    fn rec(full: u64, name: &str) -> FidRecord {
        FidRecord {
            full_hash: full,
            specific_hash: 0,
            code_unit_size: 10,
            specific_addl: 0,
            flags: 0,
            name: name.to_string(),
        }
    }

    fn quad() -> FidHashQuad {
        FidHashQuad { code_unit_size: 10, full_hash: 0x1234, specific_addl_size: 0, specific_hash: 0 }
    }

    #[test]
    fn unique_single_record_yields_its_name() {
        let recs = [rec(0x1234, "kuna_crc32")];
        assert_eq!(pick_unique_name(&recs, &quad()).as_deref(), Some("kuna_crc32"));
    }

    #[test]
    fn empty_bucket_yields_nothing() {
        assert!(pick_unique_name(&[], &quad()).is_none());
    }

    #[test]
    fn collapsing_aliases_yield_the_shared_name() {
        // Several records that all carry the same name collapse to it (the same
        // function fingerprinted under aliased symbols).
        let recs = [rec(0x1234, "kuna_crc32"), rec(0x1234, "kuna_crc32")];
        assert_eq!(pick_unique_name(&recs, &quad()).as_deref(), Some("kuna_crc32"));
    }

    #[test]
    fn genuine_multi_name_bucket_is_a_tie_no_guess() {
        // Two differently-named functions sharing a full hash → never guess.
        let recs = [rec(0x1234, "foo"), rec(0x1234, "bar")];
        assert!(pick_unique_name(&recs, &quad()).is_none());
    }
}
