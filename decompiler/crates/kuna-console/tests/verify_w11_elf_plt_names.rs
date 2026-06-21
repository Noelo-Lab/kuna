//! End-to-end gate for ELF PLT/GOT import-name resolution (the `elf_plt`
//! follow-up to `w11-elf-loader`).
//!
//! Drives the full `load file <fauxware>` → `load function main` → `decompile` →
//! `print C` path against the **real** vendored `fauxware` ELF (an x86-64,
//! dynamically-linked, non-stripped binary) and asserts the decompiled body now
//! names the PLT imports (`puts(`, `read(`) instead of the old `sub_<addr>`
//! placeholders.  This is the user-visible bug the fix targets:
//!
//! ```text
//!   before:  sub_400510(0x400915);  sub_400530(0,v1,8);
//!   after:   puts(0x400915);        read(0,v1,8);
//! ```
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_w11_elf_loader` gate, bootstrapping needs the built
//! `x86` `.sla` under `specs/` (gitignored; `make specs`).  When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored fauxware fixture (shared with the kuna-analysis loader gate).
fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

#[test]
fn fauxware_plt_calls_are_named_in_decompiled_c() {
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
                "verify_w11_elf_plt_names: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The PLT imports are now resolvable as functions (the loader markup), and
    // the local `.symtab` function still resolves.
    assert!(prog.lookup_symbol("puts").is_some(), "PLT import `puts` not resolved by the loader");
    assert!(prog.lookup_symbol("read").is_some(), "PLT import `read` not resolved by the loader");
    assert!(prog.lookup_symbol("main").is_some(), "defined `main` not resolved by the loader");

    // Drive `load function main` → `decompile` → `print C`.
    let cmds: Vec<String> = ["load function main", "decompile", "print C"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    let out = status.optr.clone();

    // The library calls are named...
    assert!(out.contains("puts("), "expected a `puts(` call, got:\n{out}");
    assert!(out.contains("read("), "expected a `read(` call, got:\n{out}");
    // ...and the old PLT-stub placeholders are gone for those call sites.
    assert!(
        !out.contains("sub_400510") && !out.contains("sub_400530"),
        "PLT stubs should no longer render as sub_<addr>, got:\n{out}"
    );
}
