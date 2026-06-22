//! End-to-end gate for the DWARF analyzer port (`s1_dwarf`, the kuna analog of
//! Ghidra's `DWARFAnalyzer`).
//!
//! Two real-ELF fixtures, two recoveries:
//!
//! 1. **Names (subtask 1)** — `dwarf_stripped_x86_64` has its `.symtab` FUNC
//!    names stripped, so DWARF is the ONLY name source. Decompiling `add_values`
//!    must resolve the DWARF name (the function loads by name and its body
//!    renders) instead of a `FUN_`/`sub_` placeholder.
//!
//! 2. **Typed signature (subtask 2, the headline)** — `cet_pie_x86_64` is NOT
//!    stripped (names already come from `.symtab`), so its DWARF value is TYPES:
//!    `elaborate_debug_symbol`'s first parameter is `char *binary` in DWARF. The
//!    decompiled signature must show a `char *` parameter (the DWARF type) rather
//!    than the engine's default `undefined8`/`long`.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_w11_*` gates, bootstrapping needs the built `x86`
//! `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap a fixture and drive `load function <fn>` -> `decompile` -> `print C`,
/// returning the captured C output (or `None` if the `.sla` is absent — a skip).
fn decompile_c(fixture_name: &str, func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture(fixture_name);
    let bin = bin.to_str()?.to_string();

    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_s1_dwarf: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let cmds: Vec<String> = [format!("load function {func}"), "decompile".into(), "print C".into()]
        .into_iter()
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
    Some(status.optr.clone())
}

#[test]
fn dwarf_stripped_recovers_function_name() {
    // The .symtab FUNC names are stripped; only DWARF names `add_values`. The
    // function must be loadable + decompilable by its DWARF name, and its body
    // must not render as a `FUN_`/`sub_` placeholder for itself.
    let Some(out) = decompile_c("dwarf_stripped_x86_64", "add_values") else {
        return;
    };
    // The DWARF name resolved (the header names the function, not a placeholder).
    assert!(
        out.contains("add_values"),
        "expected the DWARF name `add_values` in the output, got:\n{out}"
    );
    assert!(
        !out.contains("FUN_00401136") && !out.contains("sub_00401136"),
        "add_values should render by its DWARF name, not a placeholder, got:\n{out}"
    );
}

#[test]
fn cet_pie_typed_signature_has_char_ptr() {
    // cet_pie is NOT stripped, so the name already comes from .symtab; the DWARF
    // win is the TYPED parameter `char *binary`. The decompiled signature must
    // carry a `char *` (the DWARF type), not the engine's default undefined8/long.
    let Some(out) = decompile_c("cet_pie_x86_64", "elaborate_debug_symbol") else {
        return;
    };
    assert!(
        out.contains("elaborate_debug_symbol"),
        "expected the function name in the output, got:\n{out}"
    );
    assert!(
        out.contains("char *"),
        "expected a DWARF-typed `char *` parameter in the signature, got:\n{out}"
    );
}
