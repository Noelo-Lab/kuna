//! `importdata` end-to-end gate — render a data read through a loader-marked
//! import slot as the import itself, instead of a dereference of the slot.
//!
//! Fixture: `tests/fixtures/macho_gotdata` — a self-contained arm64 Mach-O built
//! from `macho_gotdata.c` with `zig cc -target aarch64-macos -O1`. `_main` reads
//! two imported data symbols, `imported_counter` and `imported_name`, each through
//! its `__DATA_CONST,__got` slot (the slot holds the import's address). The build
//! recipe + the pinned VMAs are in `tests/fixtures/README.md`.
//!
//! ## The two-state proof
//!
//! - **`--option importdata off`** (default): each read is a dereference of the
//!   anonymous slot, `*dat_100004000` / `*dat_100004008`.
//! - **`--option importdata on`**: the SAME reads are `imported_counter` /
//!   `imported_name` — `*slot` where the slot holds `&import` is the import.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

/// The pinned VMA of `_main` in the fixture (PIE imagebase `0x100000000`).
const MAIN_VMA: u64 = 0x100000448;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap the fixture, set `importdata`, commit, then decompile `_main`.
fn run(on: bool) -> String {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixtures().join("macho_gotdata");
    assert!(bin.exists(), "missing fixture {bin:?}");

    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("importdata", if on { "on" } else { "off" })
        .expect("importdata flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let name = name_at(&prog, MAIN_VMA);
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), MAIN_VMA);
    let fd = {
        let arch = prog.arch_mut();
        build_and_follow_flow(arch, &name, addr, kuna_console::engine::UNBOUNDED_SIZE)
            .expect("the _main funcdata builds")
    };

    let cmds: Vec<String> = ["decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
        dcp.fd = Some(fd);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The two-state proof: a data read through an import slot is the slot's
/// dereference by default and the import itself only when `importdata` is on.
#[test]
fn import_slot_data_read_renders_the_import_only_when_on() {
    let off = run(false);
    let on = run(true);

    eprintln!("---- importdata OFF (default) ----\n{off}");
    eprintln!("---- importdata ON ----\n{on}");

    for slot in ["*dat_100004000", "*dat_100004008"] {
        assert!(off.contains(slot), "importdata off must read through the slot {slot}:\n{off}");
        assert!(!on.contains(slot), "importdata on must not leave the slot dereference {slot}:\n{on}");
    }
    for import in ["imported_counter", "imported_name"] {
        assert!(on.contains(import), "importdata on must render the import {import}:\n{on}");
    }
    assert!(
        !off.contains("v1 = imported_counter;"),
        "importdata off must not render the import read (proves the value is the option's):\n{off}"
    );
}
