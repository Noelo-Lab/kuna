//! The `--jobs` decode lanes produce the serial walk's Listing, exactly.
//!
//! Modelled on `verify_listing_queries.rs`'s partition comparator, widened to
//! everything the walk produces: each instruction's extent, classified flow,
//! static targets and disassembly text; whole `DiscoveredFunction` records; both
//! reference maps after `finalize_refs`; and the executable ranges. Comparing
//! addresses alone would pass on a walk that decoded the same bytes and
//! classified them differently.
//!
//! The matrix is lanes x intervals-per-lane x detail, with the size floor
//! lowered so the in-repo fixtures really take the parallel path -- without that
//! override every assertion here would pass vacuously on the serial walk -- so
//! every cell also asserts the lane counter moved, which a silently-refused run
//! cannot fake.
//!
//! The other half is the refusal: on ARM, MIPS and PowerPC the language's
//! constructors carry `globalset`, so a decode at one address can change how
//! another decodes and the gate must decline. Byte-identity there rests entirely
//! on that, so it is asserted rather than assumed.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::collections::BTreeMap;
use std::path::PathBuf;

use kuna_analysis::listing::model::{DiscoveredFunction, FlowType, RefKind};
use kuna_analysis::listing::{Listing, ListingDetail, WalkPlan};
use kuna_analysis::listing::kuna_pdecode::Refusal;
use kuna_console::engine::bootstrap_from_object;
use object::{Object, ObjectSymbol};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

fn spec_roots() -> Vec<String> {
    vec![repo_root().join("specs").to_str().unwrap().to_string()]
}

/// Every in-repo fixture is far under the 8 MiB floor, so a plan built here
/// passes a floor of zero and the lanes really run. A parameter rather than
/// `KUNA_DECODE_MIN_BYTES`: cargo runs these tests as threads of one process,
/// and writing the environment out from under a sibling's `getenv` is a data
/// race, not a fixture.
const NO_SIZE_FLOOR: u64 = 0;

/// Everything the walk produced, in one comparable value.
type Partition = (
    Vec<(u64, u32, Option<u64>, FlowType, Vec<u64>, String, String)>,
    Vec<(u64, DiscoveredFunction)>,
    Vec<(u64, Vec<(u64, u64, RefKind)>)>,
    Vec<(u64, Vec<(u64, u64, RefKind)>)>,
    Vec<(u64, u64)>,
);

fn partition(listing: &Listing) -> Partition {
    let insns = listing
        .instructions()
        .map(|(&a, i)| {
            (
                a,
                i.len,
                i.fall_through,
                i.flow,
                i.flows.clone(),
                i.mnemonic.clone(),
                i.operands.clone(),
            )
        })
        .collect();
    let funcs = listing.functions().map(|(&a, f)| (a, f.clone())).collect();
    // `refs_from`'s keys are the read API's only enumerable half; `refs_to`'s
    // key set is exactly the target set those buckets carry, so both maps are
    // reachable without adding a query nothing else wants.
    let refs_from: Vec<(u64, Vec<(u64, u64, RefKind)>)> = listing
        .ref_source_iter()
        .map(|a| (a, listing.refs_from(a).iter().map(|r| (r.from, r.to, r.kind)).collect()))
        .collect();
    let mut targets: Vec<u64> =
        refs_from.iter().flat_map(|(_, b)| b.iter().map(|&(_, to, _)| to)).collect();
    targets.sort_unstable();
    targets.dedup();
    let refs_to: Vec<(u64, Vec<(u64, u64, RefKind)>)> = targets
        .into_iter()
        .map(|a| (a, listing.refs_to(a).iter().map(|r| (r.from, r.to, r.kind)).collect()))
        .collect();
    (insns, funcs, refs_to, refs_from, listing.exec_ranges().to_vec())
}

struct Loaded {
    prog: kuna_console::engine::ConsoleProgram,
    bytes: Vec<u8>,
    path: String,
    seeds: Vec<u64>,
}

/// Bootstrap `name` and collect the seed set the live path walks with.
fn load(name: &str) -> Option<Loaded> {
    let bin = fixture(name);
    let path = bin.to_str().expect("fixture path").to_string();
    let prog = match bootstrap_from_object(&path, "", &spec_roots()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_pdecode: skipping {name} (bootstrap failed, build the `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    let bytes = std::fs::read(&bin).expect("read fixture bytes");
    let file = object::File::parse(&*bytes).expect("parse fixture");
    // The entry oracles alone reach ~2 addresses on a symbol-carrying fixture;
    // the union with the text symbols is a real program to walk.
    let mut seeds = kuna_analysis::entry::collect_entries(&file, &bytes);
    for sym in file.symbols() {
        if sym.kind() == object::SymbolKind::Text && sym.address() != 0 {
            seeds.push(sym.address());
        }
    }
    seeds.sort_unstable();
    seeds.dedup();
    Some(Loaded { prog, bytes, path, seeds })
}

/// Walk `name` serially, then on every (lanes, intervals-per-lane, detail) cell,
/// and assert each parallel Listing equals the serial one. Returns the number of
/// instructions the serial walk decoded.
fn lanes_agree_everywhere(name: &str) -> usize {
    let Some(loaded) = load(name) else { return 0 };
    let file = object::File::parse(&*loaded.bytes).expect("parse fixture");
    let image =
        kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&loaded.path, &loaded.bytes)
            .expect("open throwaway loadimage");
    let arch = loaded.prog.arch();

    let mut total = 0usize;
    for detail in [ListingDetail::PARTITION_ONLY, ListingDetail::FULL] {
        let want = Listing::build_with_meta_planned(
            &file,
            &image,
            arch,
            arch.translate(),
            &loaded.seeds,
            &loaded.seeds,
            &[],
            detail,
            &WalkPlan::serial(),
        );
        assert!(
            want.num_instructions() > 50 && want.function_count() > 2,
            "{name}: the fixture must decode a real program or the comparison is vacuous \
             ({} instructions, {} functions)",
            want.num_instructions(),
            want.function_count()
        );
        let reference = partition(&want);
        total = want.num_instructions();

        for lanes in [2usize, 3, 4, 8] {
            for per_lane in [1usize, 4, 32] {
                let plan = WalkPlan::for_lanes(arch, lanes, per_lane, NO_SIZE_FLOOR);
                assert_eq!(
                    plan.declined(),
                    None,
                    "{name}: an x86 fixture must reach the lane gate, not decline early"
                );
                let got = Listing::build_with_meta_planned(
                    &file,
                    &image,
                    arch,
                    arch.translate(),
                    &loaded.seeds,
                    &loaded.seeds,
                    &[],
                    detail,
                    &plan,
                );
                assert_eq!(
                    plan.engaged(),
                    1,
                    "{name}: {lanes} lanes x {per_lane} took the serial path, so the \
                     comparison below would be vacuous"
                );
                assert!(
                    partition(&got) == reference,
                    "{name}: {lanes} lanes x {per_lane} intervals each \
                     (assembly={}, refs={}) moved the walk",
                    detail.assembly,
                    detail.refs
                );
            }
        }
    }
    total
}

#[test]
fn decode_lanes_produce_the_serial_listing() {
    // fauxware (a small non-PIE ELF), a CET PIE whose GOT slots the loader
    // patches, a PE, and the two largest in-repo x86-64 images.
    let fixtures =
        ["fauxware", "cet_pie_x86_64", "pe_imports.exe", "mcount_x86_64", "regglobal_fmt_x86_64"];
    let mut total = 0usize;
    for name in fixtures {
        let n = lanes_agree_everywhere(name);
        eprintln!("verify_pdecode: {name}: {n} instructions agreed across the lane matrix");
        total += n;
    }
    assert!(
        total > 5_000,
        "the differential is meant to cover real programs; only {total} instructions walked"
    );
}

/// A language whose constructors carry `globalset` must be refused: a decode
/// there writes the shared context database at ANOTHER address, so `decode(a)`
/// stops being a function of `a` and the lanes would disagree with each other.
#[test]
fn a_context_committing_language_is_refused() {
    for name in ["arm_thumb_linked_le32", "plt_mips32", "plt_ppc64le"] {
        let Some(loaded) = load(name) else { continue };
        let plan = WalkPlan::for_lanes(loaded.prog.arch(), 8, 32, NO_SIZE_FLOOR);
        assert_eq!(
            plan.declined(),
            Some(Refusal::ContextCommits),
            "{name}: the gate must decline a language that commits context"
        );
        assert_eq!(Refusal::ContextCommits.reason(), "language commits context");
    }
}

/// The same walk, on the same fixture, run twice at the same lane count: the
/// schedule is not the output, so a repeat must be identical too.
#[test]
fn repeated_lane_runs_are_identical() {
    let Some(loaded) = load("mcount_x86_64") else { return };
    let file = object::File::parse(&*loaded.bytes).expect("parse fixture");
    let image =
        kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&loaded.path, &loaded.bytes)
            .expect("open throwaway loadimage");
    let arch = loaded.prog.arch();
    let plan = WalkPlan::for_lanes(arch, 8, 32, NO_SIZE_FLOOR);
    let build = || {
        Listing::build_with_meta_planned(
            &file,
            &image,
            arch,
            arch.translate(),
            &loaded.seeds,
            &loaded.seeds,
            &[],
            ListingDetail::FULL,
            &plan,
        )
    };
    let first = build();
    for round in 0..4 {
        assert!(partition(&build()) == partition(&first), "run {round} differed from the first");
    }
    assert_eq!(plan.engaged(), 5, "every run must have been a lane run");
    eprintln!(
        "verify_pdecode: 5 runs at 8 lanes agreed on {} instructions",
        first.num_instructions()
    );
}

/// The seed metadata survives the merge: a seeded entry keeps its name and
/// `from_symbol` even where a CALL in another interval also targets it.
#[test]
fn seed_names_survive_the_merge() {
    let Some(loaded) = load("fauxware") else { return };
    let file = object::File::parse(&*loaded.bytes).expect("parse fixture");
    let image =
        kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&loaded.path, &loaded.bytes)
            .expect("open throwaway loadimage");
    let arch = loaded.prog.arch();
    let names: Vec<(u64, String)> =
        loaded.seeds.iter().map(|&a| (a, format!("seed_{a:x}"))).collect();
    let build = |plan: &WalkPlan| {
        Listing::build_with_meta_planned(
            &file,
            &image,
            arch,
            arch.translate(),
            &loaded.seeds,
            &loaded.seeds,
            &names,
            ListingDetail::FULL,
            plan,
        )
    };
    let want: BTreeMap<u64, DiscoveredFunction> =
        build(&WalkPlan::serial()).functions().map(|(&a, f)| (a, f.clone())).collect();
    let plan = WalkPlan::for_lanes(arch, 4, 32, NO_SIZE_FLOOR);
    let got: BTreeMap<u64, DiscoveredFunction> =
        build(&plan).functions().map(|(&a, f)| (a, f.clone())).collect();
    assert_eq!(plan.engaged(), 1, "the lanes must have run at all");
    assert!(
        want.values().any(|f| f.from_symbol && f.name.is_some()),
        "the fixture must carry named seeds or this pins nothing"
    );
    assert_eq!(got, want, "the lanes changed a function record");
}

/// The refusal reasons are the feature's whole user interface when it declines,
/// so every spelling must be in the reference an agent reads. A 17th variant
/// with no line in `docs/cli.md` fails here rather than shipping undocumented.
#[test]
fn every_refusal_reason_is_documented_in_the_cli_reference() {
    let path = repo_root().join("docs/cli.md");
    let doc = std::fs::read_to_string(&path).expect("read docs/cli.md");
    // The reference wraps, so a spelling can be split across two lines.
    let flat = doc.split_whitespace().collect::<Vec<_>>().join(" ");
    for refusal in Refusal::ALL {
        assert!(
            flat.contains(refusal.reason()),
            "docs/cli.md does not spell out `{}` ({refusal:?})",
            refusal.reason()
        );
    }
    assert_eq!(Refusal::ALL.len(), Refusal::COUNT, "every variant must be in ALL");
}
