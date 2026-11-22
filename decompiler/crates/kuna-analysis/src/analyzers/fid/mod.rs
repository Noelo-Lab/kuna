//! FunctionID fingerprints instruction streams with an operand-masked FNV-1a64
//! digest. A database match can supply names for stripped functions.
//!
//! # Module set
//!
//! - [`hash`] — [`hash::Fnv1a64`] (the FNV-1a64 digest), [`hash::FidHasher`] (the
//!   operand-masked per-instruction fingerprinter), and [`hash::FidHashQuad`]
//!   (the four-field result). A line-faithful, **byte-exact** port of Ghidra's
//!   `MessageDigestFidHasher` / `FNV1a64MessageDigest`.
//! - [`extent`] — the reachable, address-ordered body shared by build and
//!   lookup. Another function entry bounds the walk; an unresolved indirect
//!   jump prevents fingerprinting.
//! - [`skipper`] — the [`skipper::InstructionSkipper`] trait and
//!   [`skipper::X86InstructionSkipper`] (the x86 NOP/alignment skipper).
//! - [`db`] — the native `.fid` database and the shared full-hash lookup trait.
//! - [`ghidrafid`] — the Ghidra `.fidb` and `.fidbf` database adapter.
//! - [`build`] — the `FidServiceLibraryIngest` analog ([`build::build_records`]):
//!   disassembles every named function in a parsed object through the Listing
//!   path, runs the [`hash::FidHasher`] over its extent, and emits the
//!   deduplicated records. Drives the `kuna fid build` CLI subcommand.
//!
//! # PR4 (this module) — the gated matching pass
//!
//! [`FidPass`] is the Listing/xref **consumer** (structurally cloned from
//! [`crate::noreturn_disc::NoReturnDiscoveredPass`]): over the built Listing it
//! hashes every function with the byte-exact [`hash::FidHasher`] (driving the PR1
//! [`Sleigh::instruction_mask`](kuna_sleigh::sleigh::Sleigh::instruction_mask)),
//! looks the full hash up in a `.fid` database, and — when the bucket collapses to
//! exactly one name ([`pick_unique_name`], conservative; never guesses on a tie) —
//! emits a [`crate::pass::FidMatch`]. The engine's commit seam then **renames** the
//! function (only if it still carries an engine `FUN_*`/`sub_*` placeholder — the
//! label gate). Default-OFF, inert without the Listing AND without a DB — so
//! the parity oracles are structurally untouched.
//!
//! The database path is read from the `kuna_fid_db` environment variable; a real `--fidb
//! <path>` flag is a deferred follow-on (`docs/history/fid-design.md` §7.2). Absent / empty
//! ⇒ the pass is a no-op. A configured database is matched only when it loads and its
//! header language applies to the image ([`kuna_fidapply`]); either way the pass
//! reports what became of it on [`AnalysisOutput::fid_databases`].

pub mod build;
pub mod db;
pub mod extent;
pub mod ghidradb;
pub mod ghidrafid;
pub mod hash;
pub mod kuna_fidapply;
pub mod schema;
pub mod skipper;
mod kuna_fidstrict;

use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::architecture::Architecture;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Phase};
use crate::fid::db::FidRecord;
use crate::fid::kuna_fidapply::LoadedFid;
use crate::fid::hash::{FidHashQuad, FidHasher, InsnFingerprint, NoRelocations};
use crate::fid::skipper::{InstructionSkipper, X86InstructionSkipper};
use crate::listing::Listing;

/// The environment variable naming the `.fid` fingerprint database to match
/// against. A loader-tier env gate: the DB source is
/// kept out of the per-decompilation `--option` surface in v1 (a real `--fidb
/// <path>` flag is deferred, `docs/history/fid-design.md` §7.2). Absent / empty ⇒ the pass
/// is inert (no DB to match against).
pub const FID_DB_ENV: &str = kuna_decomp::kuna_loadoptions::FID_DB_ENV;

/// The FID matching consumer pass — the kuna analog of Ghidra's FID identification
/// analyzer (`FidServiceImpl` over the function-id query service), the third
/// Listing/xref consumer.
///
/// Default-OFF (gate id `fid`). Doubly inert by default: it short-circuits to an
/// empty output when no Listing is built (`ctx.listing.is_none()`, the
/// `--option listing on` flag is off) AND when no `.fid` database is configured
/// (`FID_DB_ENV` unset / empty). A configured database that does not load or
/// does not apply to the image is reported and not matched. Every emitted [`FidMatch`] is a
/// *rename* fact the engine commit seam applies only to an engine `FUN_*`/`sub_*`
/// placeholder (the label gate) — a real `.symtab`/DWARF name is never overwritten.
#[derive(Default)]
pub struct FidPass;

impl AnalysisPass for FidPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "fid"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let Some(listing) = ctx.listing else {
            return AnalysisOutput::default();
        };
        let Some(path) = configured_database_path() else {
            return AnalysisOutput::default();
        };
        match_listing(ctx.arch, listing, &path, fingerprint_function)
    }
}

impl FidPass {
    /// Run after the raw discovery walk. The caller checks the listing and FID
    /// gates; no object metadata or relocations are available on this path.
    pub fn run_raw(arch: &Architecture, listing: &Listing, path: &Path) -> AnalysisOutput {
        match_listing(arch, listing, path, fingerprint_raw_function)
    }
}

fn match_listing(
    arch: &Architecture,
    listing: &Listing,
    path: &Path,
    fingerprint: impl Fn(&Listing, &Architecture, u64) -> Result<FidHashQuad, &'static str>,
) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    let (report, db) = kuna_fidapply::open_any(path, arch);
    out.fid_databases.push(report);
    if let LoadedFid::None = db {
        return out;
    }

    for (&entry, _) in listing.functions() {
        let Ok(quad) = fingerprint(listing, arch, entry) else {
            continue;
        };
        let records = db.find_by_full_hash(quad.full_hash);
        let selection = match &db {
            LoadedFid::Ghidra(_) => pick_ghidra_name(records, &quad),
            _ => pick_unique_name(records).map(|name| {
                (name, !arch.analysis_fid_strict || kuna_fidstrict::supports_rename(records, &quad))
            }),
        };
        let Some((name, strict_evidence)) = selection else {
            continue;
        };
        let fact = FidMatch { addr: entry, name: name.to_owned(), by_signature: true };
        if arch.analysis_fid_strict && !strict_evidence {
            out.signature_vetoes.push(fact);
        } else {
            out.fid_names.push(fact);
        }
    }

    out
}

/// The configured database path: `LoadOptions::fid_db`, else [`FID_DB_ENV`].
pub fn configured_database_path() -> Option<PathBuf> {
    use kuna_decomp::kuna_loadoptions::{env, scoped};
    scoped(|o| o.fid_db.clone()).unwrap_or_else(env::fid_db)
}

/// Fingerprint a discovered function with the same extent definition and mask
/// projection the FID builder uses ([`crate::fid::build`]), so a function hashes
/// identically whether it was built into a database or looked up against one.
///
/// The extent's end bound is the next discovered entry, exactly as the builder
/// bounds a symbolized function.
pub fn fingerprint_function(
    listing: &Listing,
    arch: &Architecture,
    entry: u64,
) -> Result<FidHashQuad, &'static str> {
    fingerprint(listing, arch, entry, None)
}

/// Fingerprint a raw function using its mapped byte range and the architecture's
/// configured context. Restore decoder context before trying another root.
pub fn fingerprint_raw_function(
    listing: &Listing,
    arch: &Architecture,
    entry: u64,
) -> Result<FidHashQuad, &'static str> {
    let mapped_range = listing
        .exec_ranges()
        .iter()
        .copied()
        .find(|&(lo, hi)| lo <= entry && entry < hi)
        .ok_or("extent_outside_image")?;
    if !listing.is_instruction_start(entry) {
        return Err("extent_decode_failed");
    }
    let scope = arch.translate().decode_context_scope().ok_or("unsupported_translator")?;
    if arch.input_arm_isa_override {
        let _ = scope.protect_variable(b"TMode");
    }
    fingerprint(listing, arch, entry, Some(mapped_range))
}

fn fingerprint(
    listing: &Listing,
    arch: &Architecture,
    entry: u64,
    mapped_range: Option<(u64, u64)>,
) -> Result<FidHashQuad, &'static str> {
    let space = arch.manage().get_default_code_space().ok_or("no_code_space")?;
    let translate = arch.translate().as_sleigh().ok_or("unsupported_translator")?;
    let end = listing.next_function_after(entry).map(|f| f.entry);
    let switches = std::cell::OnceCell::new();
    let extent = extent::decode_extent(arch, translate, entry, end, mapped_range, space, &switches)?;
    if extent.len() < hash::SHORT_HASH_CODE_UNIT_LENGTH as usize {
        return Err("too_short");
    }
    let mut fps: Vec<InsnFingerprint> = Vec::with_capacity(extent.len());
    for insn in &extent {
        let addr = Address::new(Rc::clone(space), insn.addr);
        let mask = translate.instruction_mask(&addr).map_err(|_| "mask_failed")?;
        fps.push(build::fingerprint_from_mask(insn.addr, insn.len, insn.is_call, mask));
    }
    let skippers: Vec<Box<dyn InstructionSkipper>> = if arch.archid.starts_with("x86:") {
        vec![Box::new(X86InstructionSkipper)]
    } else {
        vec![]
    };
    FidHasher::new(skippers).hash(&fps, &NoRelocations).ok_or("too_short")
}

/// The native full-hash bucket must collapse to one distinct name.
fn pick_unique_name(recs: &[FidRecord]) -> Option<&str> {
    let first = recs.first()?;
    recs.iter().all(|r| r.name == first.name).then_some(first.name.as_ref())
}

/// A Ghidra name must be unique and have a row whose policy Kuna can satisfy.
/// Relation evidence and unknown flags cannot license a name, but such rows can
/// still make a competing name uncertain. Strict evidence must come from one
/// eligible record, never from a record with restrictions we cannot satisfy.
fn pick_ghidra_name<'a>(records: &'a [FidRecord], query: &FidHashQuad) -> Option<(&'a str, bool)> {
    use ghidrafid::{AUTO_FAIL, FORCE_RELATION, FORCE_SPECIFIC, KNOWN_FLAGS};

    let mut name = None;
    let mut supported = false;
    let mut strict = false;
    for record in records {
        let flags = record.flags;
        let specific_matches = record.specific_hash == query.specific_hash;
        if flags & AUTO_FAIL != 0 || (flags & FORCE_SPECIFIC != 0 && !specific_matches) {
            continue;
        }
        let candidate = record.name.as_ref();
        if name.replace(candidate).is_some_and(|prior| prior != candidate) {
            return None;
        }
        if flags & (FORCE_RELATION | !KNOWN_FLAGS) == 0 {
            supported = true;
            strict |= specific_matches && record.code_unit_size == query.code_unit_size;
        }
    }
    name.filter(|_| supported).map(|name| (name, strict))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_identity() {
        let p = FidPass;
        assert_eq!(p.id(), "fid");
        assert_eq!(p.phase(), Phase::P1);
    }

    fn rec(full: u64, name: &str) -> FidRecord {
        FidRecord {
            full_hash: full,
            specific_hash: 0,
            code_unit_size: 10,
            specific_addl: 0,
            flags: 0,
            name: name.into(),
        }
    }

    fn quad() -> FidHashQuad {
        FidHashQuad { code_unit_size: 10, full_hash: 0x1234, specific_addl_size: 0, specific_hash: 0 }
    }

    #[test]
    fn unique_single_record_yields_its_name() {
        let recs = [rec(0x1234, "kuna_crc32")];
        assert_eq!(pick_unique_name(&recs), Some("kuna_crc32"));
    }

    #[test]
    fn empty_bucket_yields_nothing() {
        assert!(pick_unique_name(&[]).is_none());
    }

    #[test]
    fn collapsing_aliases_yield_the_shared_name() {
        // Several records that all carry the same name collapse to it (the same
        // function fingerprinted under aliased symbols).
        let recs = [rec(0x1234, "kuna_crc32"), rec(0x1234, "kuna_crc32")];
        assert_eq!(pick_unique_name(&recs), Some("kuna_crc32"));
    }

    #[test]
    fn genuine_multi_name_bucket_is_a_tie_no_guess() {
        // Two differently-named functions sharing a full hash → never guess.
        let recs = [rec(0x1234, "foo"), rec(0x1234, "bar")];
        assert!(pick_unique_name(&recs).is_none());
    }

    #[test]
    fn ghidra_policy_cannot_be_bypassed_by_auto_pass() {
        use ghidrafid::{AUTO_FAIL, AUTO_PASS, FORCE_RELATION, FORCE_SPECIFIC};

        let mut record = rec(0x1234, "fixture");
        for flag in [AUTO_FAIL, FORCE_RELATION, 0x80] {
            record.flags = flag | AUTO_PASS;
            assert_eq!(pick_ghidra_name(std::slice::from_ref(&record), &quad()), None);
        }
        record.flags = FORCE_SPECIFIC | AUTO_PASS;
        record.specific_hash = 1;
        assert_eq!(pick_ghidra_name(std::slice::from_ref(&record), &quad()), None);
        record.specific_hash = quad().specific_hash;
        assert_eq!(pick_ghidra_name(&[record], &quad()), Some(("fixture", true)));
    }

    #[test]
    fn ghidra_unsupported_rows_cannot_break_ties_or_supply_strict_evidence() {
        use ghidrafid::{AUTO_FAIL, FORCE_RELATION};

        let mut allowed = rec(0x1234, "fixture");
        allowed.specific_hash = 1;
        let mut blocked = rec(0x1234, "fixture");
        blocked.flags = FORCE_RELATION;
        assert_eq!(
            pick_ghidra_name(&[allowed.clone(), blocked.clone()], &quad()),
            Some(("fixture", false)),
        );
        blocked.name = "competing name".into();
        assert_eq!(pick_ghidra_name(&[allowed.clone(), blocked.clone()], &quad()), None);
        blocked.flags = AUTO_FAIL;
        assert_eq!(pick_ghidra_name(&[allowed, blocked], &quad()), Some(("fixture", false)));
    }
}
