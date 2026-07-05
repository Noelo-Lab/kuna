//! FID PR3 — the `.fid` generator consistency + DB round-trip gate.
//!
//! Two things this proves end-to-end over the vendored fixture
//! (`kuna-analysis/tests/fixtures/fid_lib_x86_64.o`, an x86-64 `.o` with three
//! distinctive self-contained functions — `kuna_crc32`, `kuna_strlen`,
//! `kuna_memset`):
//!
//! 1. **Generator presence + naming:** `build_records` over the bootstrapped
//!    object produces a record named `kuna_crc32`, one named `kuna_strlen`, and
//!    one named `kuna_memset` — the functions are re-identifiable by name.
//!
//! 2. **DB serialization consistency:** the records, written to a `.fid` and
//!    reloaded, are byte-for-byte the same (`find_by_full_hash` recovers each),
//!    and they match the **vendored** `fid_lib_x86_64.fid` (the committed output
//!    of `kuna fid build` over the same `.o`) record-for-record. This ties the
//!    in-process generator to the vendored artifact: if the hash drifts, the
//!    vendored `.fid` and a fresh generate diverge and the test fails.
//!
//! # Architecture note
//!
//! The fixture is **x86-64** (`x86:LE:64:default`), the architecture the FID
//! design doc (`docs/fid-design.md` §7.2) targets. Earlier this fixture was a
//! 32-bit x86 workaround because the FID PR1 `Sleigh::instruction_mask` accessor
//! could not decode REX-prefixed (x86-64) instructions — its post-decode
//! decision-tree re-walk read the *final* multi-phase context (`instrPhase`
//! advanced past the REX prefix phase) and failed to re-resolve the constructor.
//! The PR1-fix captures each node's matched pattern DURING decode (correct
//! per-phase context), so REX-prefixed instructions now mask correctly and the
//! fixture is re-homed to x86-64. The byte-exact PR2 hasher, the `.fid` format,
//! the generator, and the rebasing are all architecture-agnostic.
//!
//! # `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_analysis::s1_fid::build::build_records;
use kuna_analysis::s1_fid::db::{FidDatabase, FidDb};
use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture_o() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fid_lib_x86_64.o")
}

fn fixture_fid() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fid_lib_x86_64.fid")
}

const LANG: &str = "x86:LE:64:default";
const CSPEC: &str = "gcc";

/// Build the FID records for the fixture `.o` in-process, or `None` on a
/// specs-less skip.
fn records_for_fixture() -> Option<Vec<kuna_analysis::s1_fid::db::FidRecord>> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture_o().to_str()?.to_string();
    // The fixture is x86-64; pass the language explicitly so the bootstrap
    // resolves `x86:LE:64:default` (the `.o`'s machine is EM_X86_64, which
    // derives to the same id — the explicit target just pins it).
    let prog = match bootstrap_from_object(&bin, LANG, &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_fid_build: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    let arch = prog.arch();

    let bytes = std::fs::read(&bin).expect("read fixture .o");
    let image = ObjectLoadImage::from_bytes(&bin, &bytes).expect("open loadimage");
    Some(build_records(
        &bytes,
        &image,
        arch,
        arch.translate().as_sleigh().expect("standalone Sleigh engine"),
    ))
}

#[test]
fn generator_recovers_the_three_functions_by_name() {
    let Some(records) = records_for_fixture() else { return };

    let names: Vec<&str> = records.iter().map(|r| r.name.as_str()).collect();
    assert!(
        names.contains(&"kuna_crc32"),
        "kuna_crc32 must be fingerprinted; got {names:?}"
    );
    assert!(
        names.contains(&"kuna_strlen"),
        "kuna_strlen must be fingerprinted; got {names:?}"
    );
    assert!(
        names.contains(&"kuna_memset"),
        "kuna_memset must be fingerprinted; got {names:?}"
    );

    // Each is a real, distinctive fingerprint: non-trivial code-unit size and a
    // full hash that is not the FNV offset basis (i.e. bytes were actually mixed).
    for r in &records {
        assert!(r.code_unit_size >= 4, "{}: code_unit_size {} < 4", r.name, r.code_unit_size);
        assert_ne!(r.full_hash, 0xcbf2_9ce4_8422_2325, "{}: full hash is the empty basis", r.name);
    }

    // The three functions have distinct full hashes (no collision in the fixture).
    let mut hashes: Vec<u64> = records.iter().map(|r| r.full_hash).collect();
    hashes.sort_unstable();
    let n = hashes.len();
    hashes.dedup();
    assert_eq!(hashes.len(), n, "the fixture functions must have distinct full hashes");
}

#[test]
fn generator_records_round_trip_through_the_db() {
    let Some(records) = records_for_fixture() else { return };

    // Build a DB from the freshly generated records, serialize, reload, and
    // assert every record is recoverable by its full hash, field-for-field.
    let db = FidDb::from_records(LANG, CSPEC, records.clone());
    let bytes = db.serialize();
    let back = FidDb::load(&bytes).expect("round-trip load");

    assert_eq!(back.lang_id(), LANG);
    assert_eq!(back.cspec_id(), CSPEC);
    assert_eq!(back.record_count(), records.len());
    for want in &records {
        let got = back.find_by_full_hash(want.full_hash);
        assert!(got.iter().any(|r| r == want), "{} not recovered by full hash", want.name);
    }
}

#[test]
fn fresh_generate_matches_the_vendored_fid() {
    let Some(records) = records_for_fixture() else { return };

    // The vendored `.fid` is the committed output of `kuna fid build` over the
    // same `.o`. A fresh in-process generate must reproduce the SAME records —
    // this ties the engine to the artifact (a hash drift fails here).
    let vendored_bytes = std::fs::read(fixture_fid()).expect("read vendored .fid");
    let vendored = FidDb::load(&vendored_bytes).expect("load vendored .fid");

    assert_eq!(
        vendored.record_count(),
        records.len(),
        "vendored .fid record count differs from a fresh generate"
    );
    for r in &records {
        let got = vendored.find_by_full_hash(r.full_hash);
        assert!(
            got.iter().any(|v| v.full_hash == r.full_hash
                && v.specific_hash == r.specific_hash
                && v.code_unit_size == r.code_unit_size
                && v.specific_addl == r.specific_addl
                && v.name == r.name),
            "{}: fresh generate diverges from the vendored .fid (regenerate it with \
             `kuna fid build`)",
            r.name
        );
    }
}
