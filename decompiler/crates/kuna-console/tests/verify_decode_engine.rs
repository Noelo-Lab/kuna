//! Cross-engine decode agreement: an engine rebuilt from an `EngineRecipe`
//! decodes every address the same as the one that produced the recipe.
//!
//! The rebuilt engine is a bare `Sleigh` over the parent's LIVE image bytes
//! (dynamic relocations resolved, overlays applied) with the parent's context
//! values copied in by value. The gate is not a sample: for each fixture the
//! serial Listing is built first, and then EVERY address it decoded is decoded
//! again on the second engine and compared field for field — length,
//! fall-through, flow class, static targets, mnemonic and operands.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use kuna_analysis::listing::classify::classify;
use kuna_analysis::listing::decode::decode_one;
use kuna_analysis::listing::{Listing, ListingDetail};
use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::kuna_decodekit::{build_decode_engine, EngineRecipe};
use kuna_sleigh::kuna_ctxsnapshot::snapshot_context;
use object::{Object, ObjectSymbol};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
}

fn spec_roots() -> Vec<String> {
    vec![repo_root().join("specs").to_str().unwrap().to_string()]
}

fn specs_ready(relative: &[&str]) -> bool {
    relative.iter().all(|path| {
        repo_root()
            .join("specs/Ghidra/Processors")
            .join(path)
            .is_file()
    })
}

/// Build the program, then a second engine from its recipe, and assert the two
/// agree at every address the serial walk decoded. Returns the number of
/// addresses compared.
fn agree_everywhere(name: &str) -> usize {
    let bin = fixture(name);
    let path = bin.to_str().expect("fixture path").to_string();
    let prog = bootstrap_from_object(&path, "", &spec_roots())
        .unwrap_or_else(|e| panic!("{name}: bootstrap failed: {}", e.explain()));

    let bytes = std::fs::read(&bin).expect("read fixture bytes");
    let file = object::File::parse(&*bytes).expect("parse fixture");
    // The entry oracles plus every text symbol -- the same union the live path
    // seeds the walk with, so the comparison covers a real program rather than
    // the two addresses the oracles alone reach on a symbol-carrying fixture.
    let mut seeds = kuna_analysis::entry::collect_entries(&file, &bytes);
    for sym in file.symbols() {
        if sym.kind() == object::SymbolKind::Text && sym.address() != 0 {
            seeds.push(sym.address());
        }
    }
    seeds.sort_unstable();
    seeds.dedup();

    let arch = prog.arch();
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&path, &bytes)
        .expect("open throwaway loadimage");
    let listing = Listing::build_with_meta(
        &file,
        &image,
        arch,
        arch.translate(),
        &seeds,
        &[],
        &[],
        ListingDetail::FULL,
    );

    let recipe = arch
        .decode_recipe()
        .expect("a bootstrapped Sleigh engine has a decode recipe");
    assert!(
        !recipe.sla.is_empty(),
        "{name}: the recipe carries the .sla bytes"
    );

    // The LIVE bytes of the engine's own loader, not a re-parse of the file.
    let shared = arch
        .translate()
        .loader_rc()
        .borrow()
        .shared_bytes()
        .expect("the ELF/PE loader shares its bytes");

    let parent_space = Rc::clone(
        arch.manage()
            .get_default_code_space()
            .expect("a code space"),
    );
    let snapshot = arch.with_context_db_mut(|db| snapshot_context(db, &parent_space));

    let (engine, code_space) =
        build_decode_engine(&recipe, shared, &snapshot).expect("build the second engine");

    // The engine's space must be its OWN: identity is `Rc::ptr_eq` in the
    // loader's space check and in the context cache.
    assert!(
        !Rc::ptr_eq(&code_space, &parent_space),
        "{name}: the rebuilt engine must mint its own code space"
    );

    let mut compared = 0usize;
    let mut callback_sites = 0usize;
    for (&vma, want) in listing.instructions() {
        let got = decode_one(&engine, vma, &code_space, true, true).unwrap_or_else(|e| {
            panic!(
                "{name}: {vma:#x} decoded on the parent but not here: {}",
                e.explain()
            )
        });
        assert_eq!(got.len, want.len, "{name}: {vma:#x} length");
        let c = classify(&got.ops, vma, got.len);
        assert_eq!(
            c.fall_through, want.fall_through,
            "{name}: {vma:#x} fall-through"
        );
        assert_eq!(c.flow, want.flow, "{name}: {vma:#x} flow class");
        assert_eq!(c.flows, want.flows, "{name}: {vma:#x} static targets");
        assert_eq!(got.mnemonic, want.mnemonic, "{name}: {vma:#x} mnemonic");
        assert_eq!(got.operands, want.operands, "{name}: {vma:#x} operands");
        // The `PUSH imm` callback provenance is not on the Listing, so compare
        // it against the parent engine directly -- only where this engine found
        // some, which is a few dozen addresses in a whole program.
        if !got.stored_scalar_values.is_empty() {
            let parent = decode_one(arch.translate(), vma, &parent_space, true, true)
                .expect("the parent decodes what it already decoded");
            assert_eq!(
                got.stored_scalar_values, parent.stored_scalar_values,
                "{name}: {vma:#x} stored scalar values"
            );
            callback_sites += 1;
        }
        compared += 1;
    }
    eprintln!("verify_decode_engine: {name}: {callback_sites} PUSH-imm provenance sites agreed");
    compared
}

#[test]
fn a_rebuilt_engine_decodes_every_address_the_same() {
    if !specs_ready(&[
        "x86/data/languages/x86.sla",
        "x86/data/languages/x86-64.sla",
        "AARCH64/data/languages/AARCH64.sla",
    ]) {
        eprintln!("verify_decode_engine: skipping (required .sla files are absent; `make specs`)");
        return;
    }
    // fauxware (a small non-PIE ELF), a DWARF-typed ELF, a CET PIE (whose GOT
    // slots the loader patches, so a re-parse of the file would read zeroes),
    // a PE, and the two largest in-repo x86-64 images.
    let fixtures = [
        "fauxware",
        "dwarfstructs_x86_64",
        "cet_pie_x86_64",
        "pdb_prog.exe",
        "pe_imports.exe",
        "mcount_x86_64",
        "regglobal_fmt_x86_64",
        "entrymain_aarch64",
    ];
    let mut total = 0usize;
    for name in fixtures {
        let n = agree_everywhere(name);
        eprintln!("verify_decode_engine: {name}: {n} addresses agreed");
        total += n;
    }
    assert!(
        total > 100_000,
        "the gate is meant to cover real programs; only {total} addresses compared"
    );
}

#[test]
fn a_recipe_that_carries_no_sla_is_refused() {
    let recipe = EngineRecipe {
        archid: Arc::from("x86:LE:64:default"),
        sla: Arc::from(&b""[..]),
        truncations: Arc::from(Vec::new()),
    };
    let bin = fixture("fauxware");
    let path = bin.to_str().unwrap().to_string();
    let bytes = std::fs::read(&bin).expect("read fixture bytes");
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&path, &bytes)
        .expect("open loadimage");
    let shared = kuna_sleigh::loadimage::LoadImage::shared_bytes(&image).expect("shared bytes");

    assert!(
        build_decode_engine(&recipe, shared, &Default::default()).is_err(),
        "a recipe with no .sla bytes cannot build an engine"
    );
}

#[test]
fn arm_thumb_declares_its_context_commits() {
    if !specs_ready(&["ARM/data/languages/ARM7_le.sla"]) {
        eprintln!("verify_decode_engine: skipping ARM (ARM7_le.sla is absent; `make specs`)");
        return;
    }
    let bin = fixture("arm_thumb_linked_le32");
    let path = bin.to_str().unwrap().to_string();
    let prog = bootstrap_from_object(&path, "", &spec_roots())
        .unwrap_or_else(|e| panic!("ARM bootstrap failed: {}", e.explain()));
    let arch = prog.arch();
    let sleigh = arch
        .translate()
        .as_sleigh()
        .expect("a standalone Sleigh engine");
    assert!(
        sleigh.base().has_context_commits(),
        "ARM globalsets TMode, so a decode there can change how another address decodes"
    );
    assert_eq!(
        sleigh.base().max_delay_slot_bytes(),
        0,
        "ARM has no delay slots"
    );

    let recipe = arch
        .decode_recipe()
        .expect("the bootstrap completed a recipe");
    let shared = arch
        .translate()
        .loader_rc()
        .borrow()
        .shared_bytes()
        .expect("the object loader shares its bytes");
    let parent_space = Rc::clone(arch.manage().get_default_code_space().unwrap());
    let snapshot = arch.with_context_db_mut(|db| snapshot_context(db, &parent_space));
    let err = match build_decode_engine(&recipe, shared, &snapshot) {
        Ok(_) => panic!("an engine that suppresses ARM context commits is not decode-equivalent"),
        Err(err) => err,
    };
    assert!(format!("{err}").contains("commits context"));
}

#[test]
fn recipe_space_truncations_are_applied() {
    if !specs_ready(&["AARCH64/data/languages/AARCH64.sla"]) {
        eprintln!("verify_decode_engine: skipping AARCH64 (AARCH64.sla is absent; `make specs`)");
        return;
    }
    let bin = fixture("entrymain_aarch64");
    let path = bin.to_str().unwrap().to_string();
    let prog = bootstrap_from_object(&path, "", &spec_roots())
        .unwrap_or_else(|e| panic!("AARCH64 bootstrap failed: {}", e.explain()));
    let arch = prog.arch();
    let mut recipe = arch
        .decode_recipe()
        .expect("the bootstrap completed a recipe");
    recipe.truncations = Arc::from(vec![("ram".to_string(), 4)]);
    let shared = arch
        .translate()
        .loader_rc()
        .borrow()
        .shared_bytes()
        .expect("the object loader shares its bytes");
    let parent_space = Rc::clone(arch.manage().get_default_code_space().unwrap());
    let snapshot = arch.with_context_db_mut(|db| snapshot_context(db, &parent_space));
    let (_, code_space) =
        build_decode_engine(&recipe, shared, &snapshot).expect("build a truncated engine");
    assert_eq!(code_space.get_addr_size(), 4);
}
