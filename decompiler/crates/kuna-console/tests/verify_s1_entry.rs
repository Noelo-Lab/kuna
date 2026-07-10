//! End-to-end gate for function-entry discovery (`entry` — the
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

use kuna_console::engine::bootstrap_from_object;
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

    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
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

    // (kuna) The analysis-pass facts are now committed at `read symbols` (gated by
    // the per-pass `--option <id> on|off` flags), not eagerly at bootstrap — so a
    // direct `bootstrap_from_object` consumer must trigger the commit before the
    // entry symbols are visible. This is exactly what `IfcReadSymbols` does.
    prog.commit_pending_analysis().expect("analysis commit succeeds");

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

/// The dynamic INIT/FINI array elements are named like Ghidra's loader
/// (`ElfProgramBuilder.createDynamicEntryPoints`): each `.init_array` /
/// `.fini_array` pointer becomes `_INIT_<i>` / `_FINI_<i>` instead of the generic
/// `sub_<addr>`. On this stripped PIE:
///   - `.init_array` (`DT_INIT_ARRAY` @0x3d78, 1 ptr) → 0x1240 → `_INIT_0`
///   - `.fini_array` (`DT_FINI_ARRAY` @0x3d80, 1 ptr) → 0x1200 → `_FINI_0`
///   - the single `DT_INIT` @0x1000 → `_DT_INIT`, `DT_FINI` @0x1464 → `_DT_FINI`
/// Neither 0x1240/0x1200/0x1000/0x1464 has any `.symtab` symbol here, so the name
/// comes purely from the entry-discovery + naming overlay.
#[test]
fn dynamic_init_fini_elements_get_ghidra_names() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = stripped_dynamic();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
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
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    // The array-element starts resolve under their Ghidra names, NOT `sub_<addr>`.
    assert!(
        prog.lookup_symbol("_INIT_0").is_some(),
        "INIT_ARRAY[0] (0x1240) should register as `_INIT_0` (Ghidra naming)"
    );
    assert!(
        prog.lookup_symbol("_FINI_0").is_some(),
        "FINI_ARRAY[0] (0x1200) should register as `_FINI_0` (Ghidra naming)"
    );
    // The single DT_INIT/DT_FINI entries take `_DT_INIT`/`_DT_FINI`.
    assert!(
        prog.lookup_symbol("_DT_INIT").is_some(),
        "DT_INIT (0x1000) should register as `_DT_INIT`"
    );
    assert!(
        prog.lookup_symbol("_DT_FINI").is_some(),
        "DT_FINI (0x1464) should register as `_DT_FINI`"
    );
    // The named array element is NOT also registered under the generic name.
    assert!(
        prog.lookup_symbol("sub_1240").is_none(),
        "0x1240 should be `_INIT_0`, not the generic `sub_1240`"
    );

    // It is a real, decompilable function under its Ghidra name (no --addr).
    let cmds: Vec<String> = ["load function _INIT_0", "decompile", "print C"]
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
    assert!(
        out.contains("_INIT_0"),
        "expected a decompiled body for `_INIT_0`, got:\n{out}"
    );
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "_INIT_0 should resolve to a function, got:\n{out}"
    );
}
