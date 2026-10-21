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

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
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

fn load(lazy: bool) -> ConsoleProgram {
    let mut prog = bootstrap_from_image(fixture().to_str().unwrap(), "", &[specs()])
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
fn bodies(lazy: bool, inventory: &[u64]) -> Vec<(String, String)> {
    let mut prog = load(lazy);
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

#[test]
fn lazyfuncs_keeps_every_body_identical() {
    let inventory = entries(&load(false));
    let off = bodies(false, &inventory);
    let on = bodies(true, &inventory);
    assert!(!off.is_empty(), "the fixture must decompile");
    assert_eq!(off.len(), on.len(), "both arms must decompile the same entries");
    for ((name, off_c), (on_name, on_c)) in off.iter().zip(on.iter()) {
        assert_eq!(name, on_name);
        assert_eq!(off_c, on_c, "`{name}` decompiled differently with lazyfuncs on");
    }
}

#[test]
fn lazyfuncs_commits_names_only() {
    assert!(load(false).arch().lazy_entry_starts.is_empty(), "the faithful commit parks none");
    let on = load(true);
    let lazy = on.arch().lazy_entry_starts.clone();
    assert!(!lazy.is_empty(), "the lazy commit must park the entry starts");
    let space = on.arch().manage().get_default_code_space().cloned().expect("code space");
    for &vma in &lazy {
        let addr = Address::new(Rc::clone(&space), vma);
        assert!(
            on.arch().symboltab.find_function_across_scopes(&addr).is_none(),
            "entry {vma:#x} must not have a Function before anything reads it"
        );
    }
}

#[test]
fn lazyfuncs_is_off_by_default() {
    assert!(!load(false).arch().analysis_lazyfuncs, "the option must default off");
}
