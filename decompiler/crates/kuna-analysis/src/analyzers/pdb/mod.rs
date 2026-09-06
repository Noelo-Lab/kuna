//! `pdb` — the PE PDB (Program Database) metadata tier.
//!
//! PDB is **Windows' DWARF**: function names + addresses, full types, params +
//! typed stack locals, and source lines — except the debug info lives in an
//! *external* `.pdb` file, not in the PE itself. The PE carries only a CodeView
//! *fingerprint* (a GUID/signature + age + the `.pdb` path) in its debug
//! directory; that fingerprint is what links a stripped PE back to its `.pdb`.
//!
//! ## Layout
//!
//! - [`codeview`] — **PR-P0**: the PE CodeView debug-record extractor
//!   (`PdbInfoCodeView`/`PdbInfoDotNet` port). Decodes the RSDS/NB10 record into
//!   [`codeview::CodeViewInfo`] (`{guid|sig, age, path}`). A pure extractor: no
//!   `pdb` crate.
//! - [`locate`] — `.pdb` location: the ordered candidate list (an explicit
//!   `kuna_pdb_path`, the CodeView record's own filename beside the image, then
//!   `<image stem>.pdb` beside the image) + the GUID/age **fingerprint gate**
//!   (the FID full-hash-match discipline — never apply a wrong/stale PDB).
//! - [`walk`] — **PR-P1**: the global symbol-stream walk (`S_PUB32`/`S_GPROC32`
//!   → function name + VMA) through the `pdb` crate.
//!
//! ## The pass ([`PdbPass`], the headline)
//!
//! [`PdbPass`] is the `.pdb`-consuming `AnalysisPass`, modeled on [`crate::dwarf`]
//! (the type/name producer) + the [`crate::fid`] external-artifact precedent
//! (a *rename*-emitting, externally-gated pass). On a PE it:
//!   1. calls [`codeview::extract_codeview`] for the PE's `{guid, age, path}`;
//!   2. builds the ordered `.pdb` candidate list ([`locate::pdb_candidates`]);
//!   3. opens each in turn via the `pdb` crate and **fingerprint-gates** it
//!      ([`locate::fingerprint_ok`] — guid/age must match; the first match wins,
//!      a MISMATCH/ABSENT candidate moves on, an exhausted list emits nothing);
//!   4. on a match, walks the global symbols ([`walk::walk_functions`]) and emits
//!      function **renames** via `out.fid_names` (the label-gated `FUN_*`/`sub_*`
//!      placeholder rename, the FID precedent) + `out.symbols` (so the function
//!      exists for the rename to bind even when reached only through the metadata).
//!
//! Types/typed locals/lines are the deferred PR-P2/P3 — this PR is the NAME level:
//! stripped `FUN_*` → the real name.
//!
//! ## Gating + parity safety
//!
//! Default-ON (`--option pdb`, DIV-129), PE-format gated (registered in
//! `passes.rs` only for `BinaryFormat::Pe`, and the pass self-gates on PE in
//! `run`), and inert without a fingerprint-matching `.pdb`. A non-PE image, a PE
//! with no CodeView record, no candidate `.pdb` on disk, an unreadable one, or a
//! GUID/age mismatch all yield an empty output. The fingerprint gate is what makes
//! the automatic search safe to run by default: a `.pdb` that is not this image's
//! cannot be applied. Every parity gate (`make test`/`test-stages`/`rust-test`) is
//! byte-identical by construction — the XML datatest path never calls
//! `run_default_analyses`.

pub mod codeview;
pub mod locate;
pub mod walk;

use object::read::pe::{ImageNtHeaders, PeFile, PeFile32, PeFile64};
use object::{FileKind, Object};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Phase, SymFact, SymKind};

/// The PE PDB-consuming pass (gate id `pdb`, default-on, DIV-129).
///
/// The kuna analog of Ghidra's `PdbUniversalAnalyzer` (the pure-Java PDB analyzer;
/// the MS-DIA `PdbAnalyzer` is Windows-native and out of scope). Pure over `ctx`
/// (reads `ctx.bytes` for the CodeView record + image base and `ctx.image_path`
/// for the sidecar directory, then the located `.pdb` off the filesystem),
/// additive, never failing. Inert on every non-PE image, when no candidate `.pdb`
/// exists on disk, and when none of them fingerprint-match — so it cannot perturb
/// the parity oracles.
#[derive(Default)]
pub struct PdbPass;

impl AnalysisPass for PdbPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "pdb"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();

        // PE only (belt-and-suspenders on top of the format gate in passes.rs).
        if ctx.file.format() != object::BinaryFormat::Pe {
            return out;
        }
        // 1. The PE CodeView fingerprint (the {guid, age, path} the .pdb must match).
        //    No CodeView record ⇒ no PDB to apply.
        let Some(cv) = codeview::extract_codeview(ctx.bytes) else {
            return out;
        };
        // 2. Locate: the ordered candidate list (explicit `kuna_pdb_path`, then the
        //    record's own filename beside the image, then `<stem>.pdb`). Empty ⇒
        //    inert.
        let candidates = locate::pdb_candidates(ctx.image_path, &cv);
        if candidates.is_empty() {
            return out;
        }
        // The RVA→VMA addend: the PE ImageBase. Absent (unparsable) ⇒ inert.
        let Some(image_base) = pe_image_base(ctx.bytes) else {
            return out;
        };

        // 3. Open each candidate in turn and apply the fingerprint gate; the first
        //    match wins. Any failure (unreadable file, not a `.pdb`, no info
        //    stream, or a GUID/age MISMATCH) moves on — NEVER apply a wrong/stale
        //    PDB.
        let Some(mut pdb) = open_fingerprint_matched(&candidates, &cv) else {
            return out;
        };

        // 4. The fingerprint matched: walk the global symbols and emit the renames.
        for f in walk::walk_functions(&mut pdb, image_base) {
            if f.name.is_empty() {
                continue;
            }
            // The function must exist for the rename to bind, even when it is
            // reachable ONLY through the PDB (no call edge / no funcsym). The
            // idempotent SymFact add is a no-op when it already exists; the
            // placeholder name it installs is what the fid_names rename overwrites.
            out.symbols.push(SymFact {
                addr: f.vma,
                name: f.name.clone(),
                kind: SymKind::Function,
            });
            // The label-gated rename (the FID precedent): overwrite the engine's own
            // sub_*/FUN_* placeholder with the PDB name, never a real symbol.
            out.fid_names.push(FidMatch { addr: f.vma, name: f.name });
        }

        out
    }
}

/// Open the first candidate `.pdb` whose own `{guid, age}` matches the PE's
/// CodeView record, or `None` when the list is exhausted.
///
/// A candidate that does not exist is silently skipped — most images have no
/// sidecar and that is not a defect. A candidate that IS there but does not match
/// gets one line on stderr: with the search automatic, a stale `.pdb` left beside
/// a rebuilt binary is otherwise indistinguishable from having no `.pdb` at all,
/// and silence sends the user hunting for a pass that is working exactly as
/// designed.
fn open_fingerprint_matched(
    candidates: &[std::path::PathBuf],
    cv: &codeview::CodeViewInfo,
) -> Option<pdb::PDB<'static, std::fs::File>> {
    for path in candidates {
        let Ok(file) = std::fs::File::open(path) else {
            continue;
        };
        let Ok(mut pdb) = pdb::PDB::open(file) else {
            continue;
        };
        let Ok(info) = pdb.pdb_information() else {
            continue;
        };
        let guid = info.guid.to_string();
        if locate::fingerprint_ok(cv, &guid, info.age) {
            return Some(pdb);
        }
        eprintln!(
            "[kuna pdb] {}: age {} guid {} does not match the image's CodeView record \
             (age {} guid {}); not applied",
            path.display(),
            info.age,
            guid.to_uppercase(),
            cv.age(),
            cv.guid_string().unwrap_or_else(|| "<none, NB10 record>".to_string()),
        );
    }
    None
}

/// Read the PE `ImageBase` (the RVA→VMA addend) from the optional header via the
/// typed `object` PE parser (the neutral `object::File` view does not expose it).
/// `None` on a non-PE / unparsable image. Self-contained (the `rtti`
/// `pe_image_base` shape, kept local so the two metadata passes do not couple).
fn pe_image_base(bytes: &[u8]) -> Option<u64> {
    match FileKind::parse(bytes).ok()? {
        FileKind::Pe64 => Some(image_base_of(&PeFile64::parse(bytes).ok()?)),
        FileKind::Pe32 => Some(image_base_of(&PeFile32::parse(bytes).ok()?)),
        _ => None,
    }
}

/// `relative_address_base()` = the optional-header `ImageBase`.
fn image_base_of<Pe: ImageNtHeaders>(pe: &PeFile<Pe>) -> u64 {
    pe.relative_address_base()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_identity() {
        let p = PdbPass;
        assert_eq!(p.id(), "pdb");
        assert_eq!(p.phase(), Phase::P1);
    }

    /// `pe_image_base` reads the PE ImageBase from the real `pdb_min.exe` fixture
    /// (a freestanding x86-64 PE). The exact value is toolchain-dependent — assert
    /// only that it parses to a nonzero, page-aligned base.
    #[test]
    fn pe_image_base_reads_pdb_min() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/pdb_min.exe");
        let bytes = std::fs::read(path).expect("read pdb_min.exe fixture");
        let base = pe_image_base(&bytes).expect("pdb_min.exe is a PE with an image base");
        assert_ne!(base, 0, "a PE image base is nonzero");
        assert_eq!(base & 0xfff, 0, "the image base is page-aligned");
    }

    /// A non-PE byte blob yields `None` from `pe_image_base`.
    #[test]
    fn pe_image_base_non_pe_is_none() {
        assert_eq!(pe_image_base(b"\x7fELF not a PE"), None);
    }
}
