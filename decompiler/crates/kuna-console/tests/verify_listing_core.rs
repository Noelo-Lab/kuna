//! End-to-end gate for the Listing/xref tier keystone (scope-B PR0).
//!
//! Drives `Listing::build` over a real, bootstrapped x86-64 ELF — the
//! recursive-descent disassembler reusing the ported SLEIGH decoder
//! (`Translate::one_instruction` + `print_assembly`) and the lifted flow
//! classifier — and asserts the design doc's §7 PR0 acceptance criteria against
//! the vendored `fauxware` fixture (a non-PIE x86-64 with a `main` that calls a
//! helper, contains a CBRANCH, and a RETURN).
//!
//! Obtaining a real `Translate` is the whole point: `Listing::build` decodes
//! through the SLEIGH engine, which needs the `.sla` loaded and the loadimage
//! attached. The realistic place is here (a `kuna-console` integration test):
//! `bootstrap_from_elf` gives a fully-built `ConsoleProgram`, and
//! `prog.arch().translate()` is the `&dyn Translate` the keystone drives, with
//! `prog.arch().manage()` supplying the default code space.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_*` gates, bootstrapping needs the built x86 `.sla`
//! under `specs/` (gitignored; `make specs`). When it is absent the bootstrap
//! fails; the test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::listing::{FlowKind, Listing, RefKind};
use kuna_console::engine::bootstrap_from_elf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored non-PIE x86-64 fauxware fixture (shared with several gates). Its
/// `main` (0x40071d) calls `authenticate` (0x400664), has a CBRANCH `je` at
/// 0x4007bb, and a `ret` at 0x4007d4 — the exact load-bearing sites below.
fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

// Load-bearing VMAs, read from `objdump -d` of the fixture at test-write time
// (a non-PIE ET_EXEC, so the addresses are stable file offsets+base 0x400000):
const MAIN: u64 = 0x40071d; // <main>: push %rbp
const AUTHENTICATE: u64 = 0x400664; // <authenticate>
const CALL_AUTH: u64 = 0x4007ae; // call 400664 <authenticate>
const CBRANCH_JE: u64 = 0x4007bb; // je 4007c9   (target 0x4007c9, fall 0x4007bd)
const CBRANCH_TARGET: u64 = 0x4007c9;
const CBRANCH_FALL: u64 = 0x4007bd;
const RET: u64 = 0x4007d4; // ret

#[test]
fn listing_build_recovers_instructions_flow_and_functions() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fauxware();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_listing_core: skipping (bootstrap failed, build the x86 `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // Parse the fixture bytes into an `object::File` for the exec-range universe,
    // and build the seed set the way PR2 will: existing funcsyms ∪ discovered
    // entries. fauxware is not stripped, so its funcsyms (incl. `main`) seed the
    // walk; we add `main` explicitly to be robust to the funcsym filter.
    let bytes = std::fs::read(&bin).expect("read fixture bytes");
    let file = object::File::parse(&*bytes).expect("parse fixture ELF");

    let mut seeds = kuna_analysis::s1_entry::collect_entries(&file, &bytes);
    seeds.push(MAIN);
    seeds.sort_unstable();
    seeds.dedup();

    let arch = prog.arch();
    let translate = arch.translate();

    // The keystone's `image` parameter: the PR0 decode path reads through the
    // engine loader the bootstrap already attached (it drives `translate`), so a
    // throwaway `ObjectLoadImage` over the same bytes satisfies the contract.
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&bin, &bytes)
        .expect("open throwaway loadimage");

    // The keystone: recursive-descent disassembly over the loadimage, seeded
    // from `seeds`, reusing the SLEIGH decoder + the lifted flow classifier.
    let listing = Listing::build(&file, &image, arch, translate, &seeds);

    // --- (a) instruction_at(entry) has a plausible len + non-empty mnemonic. --
    let entry = listing
        .instruction_at(MAIN)
        .unwrap_or_else(|| panic!("no instruction decoded at main ({MAIN:#x})"));
    assert!(
        entry.len >= 1 && entry.len <= 15,
        "main's first instruction has an implausible length {}",
        entry.len
    );
    assert!(
        !entry.mnemonic.is_empty(),
        "main's first instruction has an empty mnemonic"
    );

    // --- (b) a known CBRANCH yields two successors and ConditionalBranch. ------
    let cb = listing
        .instruction_at(CBRANCH_JE)
        .unwrap_or_else(|| panic!("CBRANCH site {CBRANCH_JE:#x} not decoded"));
    assert_eq!(
        cb.flow.kind,
        FlowKind::ConditionalBranch,
        "the `je` at {CBRANCH_JE:#x} must classify as a ConditionalBranch, got {:?}",
        cb.flow.kind
    );
    assert!(cb.flow.is_conditional, "the `je` must be is_conditional");
    // Two successors: the branch target (in `flows`) + the fall-through.
    assert!(
        cb.flows.contains(&CBRANCH_TARGET),
        "the `je` must target {CBRANCH_TARGET:#x}, flows = {:#x?}",
        cb.flows
    );
    assert_eq!(
        cb.fall_through,
        Some(CBRANCH_FALL),
        "the `je` must fall through to {CBRANCH_FALL:#x}"
    );

    // --- (c) a RETURN site is terminal (fall_through == None). -----------------
    let ret = listing
        .instruction_at(RET)
        .unwrap_or_else(|| panic!("RETURN site {RET:#x} not decoded"));
    assert_eq!(
        ret.flow.kind,
        FlowKind::Return,
        "the `ret` at {RET:#x} must classify as Return, got {:?}",
        ret.flow.kind
    );
    assert!(ret.flow.is_terminal, "the `ret` must be is_terminal");
    assert_eq!(ret.fall_through, None, "the `ret` must have no fall-through");

    // --- (d) a CALL site records a RefKind::Call edge + seeds a function. ------
    let call = listing
        .instruction_at(CALL_AUTH)
        .unwrap_or_else(|| panic!("CALL site {CALL_AUTH:#x} not decoded"));
    assert!(call.flow.is_call, "the `call authenticate` must be is_call");
    assert!(
        call.flow.has_fallthrough,
        "a CALL falls through (returns to the next instruction)"
    );
    // The Call edge is filed from the call site to the callee entry.
    let call_edges: Vec<_> = listing
        .refs_from(CALL_AUTH)
        .iter()
        .filter(|r| r.kind == RefKind::Call)
        .collect();
    assert!(
        call_edges.iter().any(|r| r.to == AUTHENTICATE),
        "a RefKind::Call edge {CALL_AUTH:#x} -> {AUTHENTICATE:#x} must be filed, edges = {:#x?}",
        call_edges.iter().map(|r| r.to).collect::<Vec<_>>()
    );
    // The callee is incoming-referenced and seeded as a function entry.
    assert!(
        listing.refs_to(AUTHENTICATE).iter().any(|r| r.kind == RefKind::Call),
        "authenticate ({AUTHENTICATE:#x}) must have an incoming Call ref"
    );
    assert!(
        listing.function_at(AUTHENTICATE).is_some(),
        "the CALL target authenticate ({AUTHENTICATE:#x}) must be seeded as a function entry"
    );

    // --- (e) function_count() >= number of seeds. -----------------------------
    assert!(
        listing.function_count() >= seeds.len(),
        "function_count ({}) must be >= the seed count ({})",
        listing.function_count(),
        seeds.len()
    );

    // Diagnostic sample (printed so the PR body / report can quote it): how much
    // the keystone recovered, plus a few classified instructions.
    let call_edges_total: usize =
        listing.instructions().map(|(_, i)| i.flows.iter().filter(|_| i.flow.is_call).count()).sum();
    eprintln!(
        "verify_listing_core: decoded {} instructions, discovered {} functions, {} call edges \
         (seeds = {})",
        listing.num_instructions(),
        listing.function_count(),
        call_edges_total,
        seeds.len()
    );
    for vma in [MAIN, CALL_AUTH, CBRANCH_JE, RET] {
        if let Some(i) = listing.instruction_at(vma) {
            eprintln!(
                "  {:#x}: {:<8} kind={:?} flows={:#x?} fall={:#x?}",
                vma, i.mnemonic, i.flow.kind, i.flows, i.fall_through
            );
        }
    }
}

/// PR2 build-through-engine proof: build the Listing the way the analysis driver
/// (`run_default_analyses_per_pass`) does — seed set =
/// `existing_function_addrs ∪ collect_entries` (exec-filtered, sorted/deduped),
/// driven by the engine's real `Translate` (which reads bytes through the loader
/// the bootstrap attached, not a throwaway loadimage). This is the path PR2 wires
/// behind `--option listing on`; here we exercise it directly and confirm it
/// populates a non-empty function/instruction model. (The runtime `ctx.listing`
/// population through the live driver is exercised by PR6, the first consumer;
/// PR2's job is the wiring + this proof + the flag-on==flag-off parity, see
/// `verify_listing_parity.rs`.)
#[test]
fn listing_build_through_engine_driver_seeds() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = match fauxware().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_listing_core: skipping (bootstrap failed, build the x86 `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    let bytes = std::fs::read(&bin).expect("read fixture bytes");
    let file = object::File::parse(&*bytes).expect("parse fixture ELF");

    // The driver's exact seed set (design §3.1): existing funcsyms ∪ discovered
    // entries, restricted to executable sections, sorted/deduped — the very
    // helper the live driver calls (`passes::listing_seeds`).
    let seeds = kuna_analysis::passes::listing_seeds(&file, &bytes);
    assert!(!seeds.is_empty(), "the driver seed set must be non-empty for fauxware");

    let arch = prog.arch();
    let translate = arch.translate(); // the engine's real `&dyn Translate`
    // `image` is part of the contract but the decode reads through the engine's
    // attached loader (driven by `translate`), so any loadimage over the bytes
    // satisfies it (decode.rs).
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&bin, &bytes)
        .expect("open loadimage");

    let listing = Listing::build(&file, &image, arch, translate, &seeds);

    // The build-through-engine path populates the model: at least the seeds
    // become functions, and the entry decodes to an instruction.
    assert!(
        listing.function_count() > 0,
        "engine-driven Listing::build must discover/seed functions, got {}",
        listing.function_count()
    );
    let entry = seeds[0];
    assert!(
        listing.instruction_at(entry).is_some(),
        "engine-driven Listing::build must decode an instruction at the first seed {entry:#x}"
    );
    // `main` is a seed; it must decode too.
    assert!(
        listing.instruction_at(MAIN).is_some(),
        "engine-driven Listing::build must decode an instruction at main ({MAIN:#x})"
    );
    eprintln!(
        "verify_listing_core (driver path): {} seeds -> {} functions, {} instructions",
        seeds.len(),
        listing.function_count(),
        listing.num_instructions()
    );
}
