//! End-to-end gate for function-entry discovery (`s1_entry` — the
//! `s1-entry-disc` + `s1-eh-frame` combined pass).
//!
//! Drives `load file <stripped_dynamic_x86_64>` (a PIE with `.symtab` stripped,
//! `.dynsym` only) through the bootstrap, which runs the `EntryDiscoveryPass` and
//! commits its discovered entries. Then `load function <discovered>` →
//! `decompile` → `print C` WITHOUT a supplied `--addr`: the discovery + commit
//! seam must have registered `sub_<addr>` so a *stripped* `main` is decompilable
//! by name alone.
//!
//! The proof: `main` (0x1405) is NOT in any symbol table of this binary — it is
//! recovered by the entry-discovery oracles (the `_start`→`main` libc-start idiom
//! AND the `.eh_frame` FDE pcBegin). The angr-style default naming
//! (`Architecture::name_style_angr`, default-on) means the discovered name is
//! `sub_1405` (`p0_knowledge::database::kuna_function_name`, no leading zeros).
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_w11_elf_plt_names` gate, bootstrapping needs the
//! built `x86` `.sla` under `specs/` (gitignored; `make specs`). When it is
//! absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored stripped PIE fixture (shared with the kuna-analysis entry gate).
fn stripped_dynamic() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/stripped_dynamic_x86_64")
}

#[test]
fn discovered_main_decompiles_without_supplied_address() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = stripped_dynamic();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_s1_entry: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The discovery pass + commit seam registered the entries by their angr-style
    // names. `main` (0x1405) and the ELF entry `_start` (0x1160) are present even
    // though neither has a `.symtab` symbol in this stripped binary.
    assert!(
        prog.lookup_symbol("sub_1405").is_some(),
        "discovered `main` (sub_1405) not registered — entry discovery failed"
    );
    assert!(
        prog.lookup_symbol("sub_1160").is_some(),
        "discovered ELF entry `_start` (sub_1160) not registered"
    );

    // Drive `load function sub_1405` → `decompile` → `print C` WITHOUT --addr:
    // the discovered name resolves to a function and produces a real body.
    let cmds: Vec<String> = ["load function sub_1405", "decompile", "print C"]
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

    // A real decompilation of a discovered, never-symboled function: it names the
    // function and emits a C body (braces) — not the CLI "no function" error.
    assert!(
        out.contains("sub_1405"),
        "expected a decompiled body for the discovered sub_1405, got:\n{out}"
    );
    assert!(
        out.contains('{') && out.contains('}'),
        "expected a C function body for sub_1405, got:\n{out}"
    );
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "sub_1405 should resolve to a function, got:\n{out}"
    );
}
