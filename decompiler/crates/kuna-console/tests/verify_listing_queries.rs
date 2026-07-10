//! Read/query-surface gate for the Listing/xref tier (scope-B PR3 + PR4).
//!
//! PR0 built the model (the recursive-descent walk over a real, bootstrapped
//! x86-64 ELF — `verify_listing_core.rs`). This gate exercises the *read* surface
//! the future consumer (discovered-no-return) transliterates against (design
//! `docs/listing-tier-design.md` §6):
//!
//!  - the **code-unit partition** (PR3): `code_unit_at` / `is_data` /
//!    `is_undefined` / `first_undefined_after` / `num_instructions`,
//!  - the **ordered function model** (PR3): `function_at` / `function_containing`
//!    / `next_function_after` / `function_count`,
//!  - the **xref read API** (PR4): `refs_to` / `refs_from` / `ref_source_iter` /
//!    `has_refs_to` / `ref_count_to`.
//!
//! It reuses the PR0 fixture and bootstrap (`fauxware`, a non-PIE x86-64 with a
//! `main` that calls `authenticate` exactly once, a CBRANCH, and a `ret`).
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_*` gates and `verify_listing_core.rs`, bootstrapping
//! needs the built x86 `.sla` under `specs/` (gitignored; `make specs`). When it
//! is absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::listing::{CodeUnit, Listing, RefKind};
use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

// Load-bearing VMAs, read from `objdump -d` of the fixture (non-PIE ET_EXEC, so
// the addresses are stable file offsets + base 0x400000).
const MAIN: u64 = 0x40071d; // <main>: push %rbp  (1-byte insn)
const MAIN_MID: u64 = 0x40073e; // lea -0x10(%rbp),%rax — interior of <main>
const AUTHENTICATE: u64 = 0x400664; // <authenticate>, called exactly once
const CALL_AUTH: u64 = 0x4007ae; // the sole `call 400664 <authenticate>` site
const RET: u64 = 0x4007d4; // <main>'s `ret`
const CSU_INIT: u64 = 0x4007e0; // <__libc_csu_init> — the next funcsym after main

#[test]
fn listing_query_surface_partition_functions_and_xrefs() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = match fauxware().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_listing_queries: skipping (bootstrap failed, build the x86 `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    let bytes = std::fs::read(&bin).expect("read fixture bytes");
    let file = object::File::parse(&*bytes).expect("parse fixture ELF");

    // Same seed set as the PR0 gate (discovered entries ∪ main), plus the next
    // funcsym after main (`__libc_csu_init`) so the ordered `next_function_after`
    // query has a concrete next-by-address successor to return. Both are real
    // funcsyms that `existing_function_addrs` would seed on the live PR2 path.
    let mut seeds = kuna_analysis::entry::collect_entries(&file, &bytes);
    seeds.push(MAIN);
    seeds.push(CSU_INIT);
    seeds.sort_unstable();
    seeds.dedup();

    let arch = prog.arch();
    let translate = arch.translate();
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&bin, &bytes)
        .expect("open throwaway loadimage");

    let listing = Listing::build(&file, &image, arch, translate, &seeds);

    // =========================================================================
    // PR3 — code-unit partition
    // =========================================================================

    // A known instruction VMA ⇒ Instruction (its own start).
    assert_eq!(
        listing.code_unit_at(MAIN),
        CodeUnit::Instruction(MAIN),
        "a decoded instruction start ({MAIN:#x}) must partition as Instruction"
    );
    // An interior byte of a decoded instruction also partitions as Instruction,
    // anchored to that instruction's start. `main`'s entry `push %rbp` is a single
    // byte (0x40071d), so probe an interior byte of a multi-byte insn instead:
    // `mov %rsp,%rbp` at 0x40071e is 3 bytes (0x40071e..0x400721).
    let mov_rbp = 0x40071e;
    assert_eq!(
        listing.code_unit_at(mov_rbp),
        CodeUnit::Instruction(mov_rbp),
        "the 3-byte insn at {mov_rbp:#x} must be an Instruction at its start"
    );
    assert_eq!(
        listing.code_unit_at(mov_rbp + 1),
        CodeUnit::Instruction(mov_rbp),
        "an interior byte ({:#x}) of the insn at {mov_rbp:#x} must map to that insn",
        mov_rbp + 1
    );

    // A non-code VMA outside every executable range ⇒ Data (the conservative
    // partition). The ELF header at the image base (0x400000) is in a non-exec
    // mapping, well below `.text`.
    assert!(
        listing.is_data(0x400000),
        "the image base (0x400000, a non-exec address) must partition as Data, got {:?}",
        listing.code_unit_at(0x400000)
    );
    assert!(!listing.is_undefined(0x400000), "a non-exec address is Data, not Undefined");

    // `num_instructions` is non-trivial (the walk recovered the program).
    assert!(
        listing.num_instructions() > 10,
        "the walk should recover many instructions, got {}",
        listing.num_instructions()
    );

    // `first_undefined_after`: scanning forward from `main`'s entry never returns
    // an address inside a decoded instruction; if it returns Some(u), `u` is
    // genuinely undefined (an exec-range byte not covered by any instruction).
    if let Some(u) = listing.first_undefined_after(MAIN) {
        assert!(
            listing.is_undefined(u),
            "first_undefined_after({MAIN:#x}) = {u:#x} must itself be Undefined"
        );
        assert!(
            !listing.is_instruction_start(u),
            "first_undefined_after must not land on an instruction start ({u:#x})"
        );
    }

    // =========================================================================
    // PR3 — ordered function model
    // =========================================================================

    // `function_at` resolves an exact entry; `main` was seeded.
    assert!(
        listing.function_at(MAIN).is_some(),
        "function_at(main = {MAIN:#x}) must resolve the seeded entry"
    );

    // `function_containing(mid-of-main)` returns `main` (nearest preceding entry).
    let containing = listing
        .function_containing(MAIN_MID)
        .unwrap_or_else(|| panic!("function_containing({MAIN_MID:#x}) found nothing"));
    assert_eq!(
        containing.entry, MAIN,
        "function_containing({MAIN_MID:#x}) must be main ({MAIN:#x}), got {:#x}",
        containing.entry
    );

    // `next_function_after(main)` is the next-by-address function. With
    // `__libc_csu_init` seeded, the concrete successor is 0x4007e0; it must also
    // be self-consistent with the listing's own ordered function set.
    let expected_next = listing
        .functions()
        .map(|(&a, _)| a)
        .filter(|&a| a > MAIN)
        .min();
    let next = listing.next_function_after(MAIN).map(|f| f.entry);
    assert_eq!(
        next, expected_next,
        "next_function_after(main) must be the smallest entry > main; got {next:#x?}, \
         expected {expected_next:#x?}"
    );
    assert_eq!(
        next,
        Some(CSU_INIT),
        "next_function_after(main {MAIN:#x}) must be __libc_csu_init ({CSU_INIT:#x})"
    );

    // `function_count` equals the size of the ordered function set.
    assert_eq!(
        listing.function_count(),
        listing.functions().count(),
        "function_count must equal the iterated function set size"
    );
    assert!(
        listing.function_count() >= seeds.len(),
        "function_count ({}) must be >= the seed count ({})",
        listing.function_count(),
        seeds.len()
    );

    // =========================================================================
    // PR4 — xref read API
    // =========================================================================

    // `refs_to(authenticate)` contains the call site, kind == Call.
    let to_auth = listing.refs_to(AUTHENTICATE);
    assert!(
        to_auth.iter().any(|r| r.from == CALL_AUTH && r.kind == RefKind::Call),
        "refs_to(authenticate {AUTHENTICATE:#x}) must contain the Call site {CALL_AUTH:#x}; \
         got {:#x?}",
        to_auth.iter().map(|r| (r.from, r.kind)).collect::<Vec<_>>()
    );

    // `ref_source_iter()` includes the caller's call address (a reference source).
    assert!(
        listing.ref_source_iter().any(|s| s == CALL_AUTH),
        "ref_source_iter() must include the call site {CALL_AUTH:#x}"
    );
    // It is strictly ascending (the BTreeMap key order), and de-duplicated.
    let sources: Vec<u64> = listing.ref_source_iter().collect();
    assert!(
        sources.windows(2).all(|w| w[0] < w[1]),
        "ref_source_iter() must be strictly ascending / deduplicated"
    );

    // `has_refs_to` / `ref_count_to`: authenticate is called from exactly one
    // site, so it has incoming Call refs and the Call count is exactly 1.
    assert!(
        listing.has_refs_to(AUTHENTICATE),
        "has_refs_to(authenticate) must be true"
    );
    let call_refs_to_auth =
        listing.refs_to(AUTHENTICATE).iter().filter(|r| r.kind == RefKind::Call).count();
    assert_eq!(
        call_refs_to_auth, 1,
        "authenticate is called from exactly one site; Call ref_count = {call_refs_to_auth}"
    );
    // `ref_count_to` (all kinds) equals the (deduped) bucket length.
    assert_eq!(
        listing.ref_count_to(AUTHENTICATE),
        listing.refs_to(AUTHENTICATE).len(),
        "ref_count_to must equal the refs_to bucket length"
    );

    // `has_refs_to` is false for an address nothing references (the bare ELF base).
    assert!(
        !listing.has_refs_to(0x400000),
        "nothing references the image base; has_refs_to(0x400000) must be false"
    );

    // `refs_from(call_site)` carries the outgoing Call edge to authenticate.
    let from_call = listing.refs_from(CALL_AUTH);
    assert!(
        from_call.iter().any(|r| r.to == AUTHENTICATE && r.kind == RefKind::Call),
        "refs_from({CALL_AUTH:#x}) must contain the outgoing Call to authenticate; got {:#x?}",
        from_call.iter().map(|r| (r.to, r.kind)).collect::<Vec<_>>()
    );

    // The `ret` site is terminal: it files no outgoing edge.
    assert!(
        listing.refs_from(RET).is_empty(),
        "the terminal `ret` at {RET:#x} files no outgoing reference, got {:#x?}",
        listing.refs_from(RET).iter().map(|r| (r.to, r.kind)).collect::<Vec<_>>()
    );

    // Diagnostic sample for the PR body / report.
    eprintln!(
        "verify_listing_queries: {} instructions, {} functions, {} ref sources",
        listing.num_instructions(),
        listing.function_count(),
        listing.ref_source_iter().count(),
    );
    eprintln!(
        "  function_containing({MAIN_MID:#x}) = {:#x} ({})",
        containing.entry,
        containing.name.as_deref().unwrap_or("<unnamed>")
    );
    eprintln!("  next_function_after({MAIN:#x}) = {:#x?}", next);
    eprintln!(
        "  refs_to(authenticate {AUTHENTICATE:#x}) = {:#x?}",
        listing
            .refs_to(AUTHENTICATE)
            .iter()
            .map(|r| (r.from, r.kind))
            .collect::<Vec<_>>()
    );
    eprintln!("  ref_count_to(authenticate) (Call) = {call_refs_to_auth}");
}
