//! Two-pass gate for the `lazyfuncs` option (P1, image commit).
//!
//! Off is Ghidra's whole-inventory commit; on installs the same names and builds a
//! `Function` only where the decompiler reads one.  The gate is that every body is
//! unchanged, that the lazy commit really leaves the inventory unmaterialized, and
//! that the option arrives off.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_image, ConsoleProgram};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
}

fn specs() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("specs")
        .to_string_lossy()
        .into_owned()
}

fn load(image: &str, lazy: bool) -> ConsoleProgram {
    let mut prog = bootstrap_from_image(fixture(image).to_str().unwrap(), "", &[specs()])
        .expect("bootstrap fixture with built processor specs");
    if lazy {
        prog.arch_mut().set_kuna_option("lazyfuncs", "on").expect("settable");
    }
    prog.commit_pending_analysis().expect("read symbols (analysis commit)");
    prog
}

/// The image's entries, address-ordered and deduped -- the engine's own symbol stream,
/// which is mode independent.
fn entries(prog: &ConsoleProgram) -> Vec<u64> {
    let mut out: Vec<u64> = prog.function_entries().map(|(_, a)| a.get_offset()).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Every entry's emitted C, decompiled the way `kuna decompile <image> <addr>` does.
fn bodies(image: &str, lazy: bool, inventory: &[u64]) -> Vec<(String, String)> {
    let mut prog = load(image, lazy);
    let mut out = Vec::new();
    for &vma in inventory {
        // Rebuild in THIS program's manager: an `Address` from another program carries
        // that program's `AddrSpace`, whose load image is not the one under test.
        let Some(space) = prog.arch().manage().get_default_code_space().cloned() else { continue };
        let entry = Address::new(space, vma);
        let Ok(name) = prog.ensure_function_symbol(entry.clone()) else { continue };
        let size = prog.declared_extent(vma);
        let step = decompile_one(
            prog.arch_mut(),
            &name,
            entry,
            size,
            &DecompileSeed::plain(&[], &[]),
            &[],
        );
        let Ok(fd) = step.result else { continue };
        out.push((name, kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd)));
    }
    out
}

fn assert_identical(image: &str) {
    let inventory = entries(&load(image, false));
    let off = bodies(image, false, &inventory);
    let on = bodies(image, true, &inventory);
    assert!(!off.is_empty(), "the fixture must decompile");
    assert_eq!(off.len(), on.len(), "both arms must decompile the same entries");
    for ((name, off_c), (on_name, on_c)) in off.iter().zip(on.iter()) {
        assert_eq!(name, on_name);
        assert_eq!(off_c, on_c, "`{name}` decompiled differently with lazyfuncs on");
    }
}

#[test]
fn lazyfuncs_keeps_every_body_identical() {
    assert_identical("fauxware");
}

/// A stripped image's discovered entries carry the facts the commit lands on a
/// `Function` -- the entry oracle's `main` name and declaration, discovered no-return
/// verdicts -- so a name-only commit of those entries would change their callers.
#[test]
fn lazyfuncs_keeps_stripped_bodies_identical() {
    assert_identical("stripped_dynamic_x86_64");
    assert_identical("pe_imports_stripped.exe");
}

/// String facts stay compact until a body reads one, including a byte loaded from
/// inside a literal rather than the literal's address taken as a constant.
#[test]
fn lazyfuncs_keeps_string_reads_identical() {
    assert_identical("xrefpairs_macho_arm64");
}

#[test]
fn lazyfuncs_commits_names_only() {
    let off = load("fauxware", false);
    assert!(off.arch().lazy_entry_starts.is_empty(), "the faithful commit parks none");
    let on = load("fauxware", true);
    let lazy = on.arch().lazy_entry_starts.clone();
    assert!(!lazy.is_empty(), "the lazy commit must park the entry starts");
    let space = on.arch().manage().get_default_code_space().cloned().expect("code space");
    for &vma in lazy.iter() {
        let addr = Address::new(Rc::clone(&space), vma);
        assert!(
            on.arch().symboltab.find_function_across_scopes(&addr).is_none(),
            "entry {vma:#x} must not have a Function before anything reads it"
        );
    }
}

#[test]
fn lazyfuncs_is_off_by_default() {
    assert!(!load("fauxware", false).arch().analysis_lazyfuncs, "the option must default off");
}

#[test]
fn lazy_inventory_preserves_names_aliases_and_extents() {
    for image in ["fauxware", "stripped_dynamic_x86_64", "pe_imports_stripped.exe"] {
        let off = load(image, false);
        let on = load(image, true);
        let expected = off.function_entries_canonical();
        for entry in &expected {
            let selected = on.find_entry_at(entry.addr.get_offset()).expect("entry address");
            assert_eq!(selected.name, entry.name);
            assert_eq!(selected.aliases, entry.aliases);
            assert_eq!(selected.size, entry.size);
            for name in std::iter::once(&entry.name).chain(&entry.aliases) {
                assert_eq!(
                    on.lookup_symbol(name).map(|a| a.get_offset()),
                    off.lookup_symbol(name).map(|a| a.get_offset()),
                );
            }
        }
        let raw = |prog: &ConsoleProgram| prog.function_entries()
            .map(|(name, addr)| (name.to_string(), addr.get_offset())).collect::<Vec<_>>();
        assert_eq!(raw(&on), raw(&off));
        let actual = on.function_entries_canonical();
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(&expected) {
            assert_eq!(actual.name, expected.name);
            assert_eq!(actual.addr.get_offset(), expected.addr.get_offset());
            assert_eq!(actual.aliases, expected.aliases);
            assert_eq!(actual.size, expected.size);
        }
    }
}
