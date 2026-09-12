//! End-to-end gate for the walk's committed-entry seeding (`armdiscseed`) — the
//! recursive descent explores the call graph out of every entry the load-time
//! oracles agreed on, not just the ones `listing_seeds` can recompute.
//!
//! Fixture: `armdiscseed_le32`, a stripped non-PIE ARM32 ELF in the witness's
//! shape. It carries no symbol table, no `.eh_frame` and no `INIT_ARRAY`, so
//! `collect_entries` recovers `_start` and nothing else; `main` comes only from
//! the standalone `armlibcmain` oracle, and the validator `main` calls is named
//! by nothing at all.
//!
//! ## The two passes
//!
//! * **the bug:** `run_listing_consumers` rebuilt its seed set from the object,
//!   so an entry a standalone pass had already committed was not a root. The
//!   walk stopped at crt1, `main`'s body was never decoded, and the validator
//!   was absorbed into `main`'s address-contiguous extent.
//! * **the fix:** the caller's merged entries seed the walk too, so the descent
//!   follows `main`'s `bl` and the validator becomes a function.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `ARM` `.sla` under `specs/` (gitignored;
//! `make specs`). When it is absent the bootstrap fails; the test prints that
//! and returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// `_start`, the ELF entry point — the one address `collect_entries` recovers.
const START: u64 = 0x10160;
/// `main`, reachable only through the crt1 literal pool word (`armlibcmain`).
const MAIN: u64 = 0x10180;
/// The validator `main` calls — named by no oracle, reachable only by following
/// `main`'s call graph.
const VALIDATOR: u64 = 0x10190;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armdiscseed_le32")
}

/// Bootstrap the fixture the way every `kuna` driver does on a non-x86-64 image
/// (DIV-20/DIV-68: the Listing plus the discovery bundle). `None` is a visible
/// skip when the `.sla` is missing.
fn bootstrap() -> Option<ConsoleProgram> {
    let bin = fixture();
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_armdiscseed: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .expect("funcstart_patterns flips on");
    prog.arch_mut().set_kuna_option("aif", "on").expect("aif flips on");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn entries(prog: &ConsoleProgram) -> Vec<(String, u64, u64)> {
    prog.function_entries_canonical()
        .into_iter()
        .map(|e| (e.name, e.addr.get_offset(), e.size))
        .collect()
}

/// The inventory a `kuna functions` run reports: the validator is its own
/// function, and `main` stops where the validator starts.
#[test]
fn the_inventory_lists_the_function_main_calls() {
    let Some(prog) = bootstrap() else {
        return;
    };
    let got = entries(&prog);
    assert!(
        got.iter().any(|&(_, addr, _)| addr == VALIDATOR),
        "the callee `main` reaches by `bl` must be a function; got {got:?}"
    );
    let main = got.iter().find(|&&(_, addr, _)| addr == MAIN);
    assert_eq!(
        main.map(|&(_, _, size)| size),
        Some(VALIDATOR - MAIN),
        "`main` must stop at the validator rather than swallowing it; got {got:?}"
    );
}

/// The mechanism, isolated: the same walk over the same bytes, run once with
/// the committed entries as roots and once without. Only the seeded run reaches
/// the validator — which is the whole defect, since `main` is a committed entry
/// that `listing_seeds` cannot recompute.
#[test]
fn only_the_committed_entry_seeds_reach_the_validator() {
    let Some(prog) = bootstrap() else {
        return;
    };
    let bytes = std::fs::read(fixture()).expect("fixture readable");
    let path = fixture().to_str().unwrap().to_string();
    let image =
        kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_silent(&path, &bytes)
            .expect("fixture parses as a load image");
    let arch = prog.arch();

    let discovered = |seeds: &[u64]| -> Vec<u64> {
        let out = kuna_analysis::passes::run_listing_consumers(
            &bytes,
            &image,
            arch,
            arch.translate(),
            &[],
            &[],
            seeds,
            &kuna_analysis::listing::WalkPlan::serial(),
        );
        let mut vmas: Vec<u64> = out
            .iter()
            .filter(|(id, _)| *id == "funcdisc_recursive")
            .flat_map(|(_, o)| o.entries.iter().copied())
            .collect();
        vmas.sort_unstable();
        vmas.dedup();
        vmas
    };

    let unseeded = discovered(&[]);
    assert!(
        unseeded.contains(&START),
        "the ELF entry is recomputed from the object either way; got {unseeded:?}"
    );
    assert!(
        !unseeded.contains(&VALIDATOR),
        "without the committed entries the walk cannot reach `main`, so it cannot \
         reach what `main` calls; got {unseeded:?}"
    );

    let seeded = discovered(&[MAIN]);
    assert!(
        seeded.contains(&VALIDATOR),
        "seeded with the committed `main`, the walk follows its `bl`; got {seeded:?}"
    );
    // Purely additive: seeding adds roots, and the walk's function model only
    // ever grows.
    for vma in &unseeded {
        assert!(seeded.contains(vma), "seeding must not drop {vma:#x}: {seeded:?}");
    }
}

/// The seeds are exec-filtered, so a committed entry outside every executable
/// section — the shape a junk-word `bl` decode produces on an obfuscated image —
/// never becomes a function.
#[test]
fn a_committed_entry_outside_the_code_is_not_walked() {
    let Some(prog) = bootstrap() else {
        return;
    };
    let bytes = std::fs::read(fixture()).expect("fixture readable");
    let path = fixture().to_str().unwrap().to_string();
    let image =
        kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_silent(&path, &bytes)
            .expect("fixture parses as a load image");
    let arch = prog.arch();
    let out = kuna_analysis::passes::run_listing_consumers(
        &bytes,
        &image,
        arch,
        arch.translate(),
        &[],
        &[],
        &[MAIN, 0x59684C],
        &kuna_analysis::listing::WalkPlan::serial(),
    );
    let vmas: Vec<u64> = out
        .iter()
        .filter(|(id, _)| *id == "funcdisc_recursive")
        .flat_map(|(_, o)| o.entries.iter().copied())
        .collect();
    assert!(vmas.contains(&VALIDATOR), "the in-image seed still lands; got {vmas:?}");
    assert!(
        !vmas.contains(&0x59684C),
        "an address in no executable section is not a function; got {vmas:?}"
    );
}
