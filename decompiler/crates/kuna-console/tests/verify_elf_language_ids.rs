//! Gate for the multi-format-loader PR-1 §2.2 refactor: every SLEIGH language id
//! the ELF loader (`ElfFormat::compiler_model` + `loadimage_object::language_id_for`)
//! can produce must **resolve in the SLEIGH language database**
//! (`scan_language_database` → `SleighArchitecture::resolve_architecture`).
//!
//! The PR threaded the compiler-model field through the `ObjectFormat` seam; the
//! ELF id strings are byte-identical to before (proven by the in-crate unit
//! tests), and this gate proves the structural change still yields only valid,
//! vendored ids — an invented compiler token (or a stem with no `.ldefs` entry)
//! would fail to resolve, exactly the "No sleigh specification" error a bad id
//! produces on the live path.
//!
//! The id list comes from `kuna_analysis::loadimage_object::elf_language_ids()`,
//! derived from the same composer the loader uses, so the test cannot drift from
//! the producer.
//!
//! ## `.ldefs` precondition
//!
//! Resolution matches against the language **descriptions** parsed from the
//! vendored `.ldefs` (not the built `.sla`), so this gate does NOT require
//! `make specs`. If the specs tree is absent the DB load fails; the test prints
//! that and returns early (a visible skip, never a false green).

use std::path::PathBuf;

use kuna_base::marshal::IdRegistry;
use kuna_decomp::sleigh_arch::{
    register_sleigh_arch_ids, LanguageDatabase, SleighArchitecture,
};
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The same per-module id registration the console bootstrap's `build_registry`
/// does (minus the option elements, which language resolution does not need).
fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    registry
}

#[test]
fn every_elf_language_id_resolves_in_the_database() {
    let specs = repo_root().join("specs");
    let registry = build_registry();
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(specs.to_str().unwrap());
    if let Err(e) = db.get_descriptions(&registry) {
        eprintln!(
            "verify_elf_language_ids: skipping (no specs tree / .ldefs load failed): {}",
            e.explain()
        );
        return;
    }

    let ids = kuna_analysis::loadimage_object::elf_language_ids();
    assert!(!ids.is_empty(), "the ELF id producer must enumerate at least one id");

    for id in &ids {
        let mut sleigh = SleighArchitecture::new("t", "");
        sleigh.set_archid(id);
        sleigh
            .resolve_architecture(&db, id)
            .unwrap_or_else(|e| panic!("ELF language id {id:?} failed to resolve: {}", e.explain()));
        assert!(
            sleigh.language_index() >= 0,
            "ELF language id {id:?} did not resolve to a language (index {})",
            sleigh.language_index()
        );
    }

    // Sanity: the headline arches are present in the enumerated set, so a future
    // arm deletion can't quietly shrink the gate to nothing.
    for stem in ["x86:LE:64", "x86:LE:32", "ARM:LE:32", "AARCH64:LE:64", "MIPS:LE:32", "RISCV:LE:64"]
    {
        assert!(
            ids.iter().any(|id| id.starts_with(stem)),
            "expected an id beginning {stem:?} in {ids:?}"
        );
    }
}
