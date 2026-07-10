//! Multi-format-loader PR-12+13 e2e gate: a **stripped** PE / Mach-O finds its
//! functions automatically — the discovered function decompiles by its
//! `sub_<addr>` name with **no `--addr`**, exactly as a stripped ELF does
//! (`verify_s1_entry`). Design §4.1 / §5.3 / §8 PR-12+13.
//!
//! ## The payoff (the headline before/after)
//!
//! A bare load of a stripped non-ELF binary knows only what `--addr` names: its
//! `.symtab` is empty, so `lookup_symbol` finds nothing and every function is
//! `sub_<addr>`-by-address-only. The `entry` PE/Mach-O oracles change that:
//!
//! - **PE** (`pe_imports_stripped.exe`, fully stripped) — the entry point and
//!   the **`.pdata`** RUNTIME_FUNCTION table (97 records — the `.eh_frame`
//!   analog) recover the functions. `main`@`0x140001592` is `.pdata`-covered, so
//!   `sub_140001592` registers and decompiles **without `--addr`**.
//!
//! - **Mach-O** (`macho_func_starts_stripped`, `helper` stripped) — the
//!   **`LC_FUNCTION_STARTS`** delta table recovers `helper`@`0x100000590`, whose
//!   symbol `ld64.lld -x` removed. `sub_100000590` registers and decompiles
//!   **without `--addr`** — proving function-starts is the source, not the symtab.
//!
//! ```text
//!   before (bare load):  lookup_symbol("sub_140001592") -> None  (no --addr, no function)
//!   after  (PR-12/13):   lookup_symbol("sub_140001592") -> Some  (decompiles by name)
//! ```
//!
//! ## Multi-format is the default (ELF stays byte-identical)
//!
//! PE/Mach-O load unconditionally — the same default `load file` dispatch as ELF,
//! with no flag. The ELF entry path (`verify_s1_entry`) is untouched: the
//! discovery oracles are no-ops on ELF (format-dispatched), so the 675/675 +
//! stage gates are structurally immune.
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). When absent the bootstrap fails and
//! the test prints that and returns early (a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Bootstrap a fixture (multi-format loading is unconditional), commit the
/// analysis facts (so the discovered entries become visible symbols), and return
/// the program — `None` (a visible skip) when the `.sla` is absent.
fn boot_committed(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    let mut prog = match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_multiformat_entry: skipping {name} (bootstrap failed; build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The discovered entries are committed at `read symbols`, gated by the per-pass
    // `--option` flags — a direct `bootstrap_from_object` consumer must trigger it.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Drive `<func_cmd>` → `decompile` → `print C` (no `--addr`) and return the C.
fn decompile_func(prog: ConsoleProgram, func_cmd: &str) -> String {
    let cmds: Vec<String> = [func_cmd, "decompile", "print C"]
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
    status.optr.clone()
}

/// PR-12: a **stripped PE** finds `main` via the entry + `.pdata` oracles, so
/// `sub_140001592` resolves and decompiles with NO supplied address.
#[test]
fn pe_stripped_discovers_main_without_addr() {
    let Some(prog) = boot_committed("pe_imports_stripped.exe") else { return };

    // `main`@0x140001592 is `.pdata`-covered; the angr-style name is `sub_140001592`.
    // (No `.symtab` symbol exists in this stripped PE — discovery is the ONLY source.)
    assert!(
        prog.lookup_symbol("sub_140001592").is_some(),
        "stripped PE: discovered `main` (sub_140001592) not registered — \
         `.pdata` entry discovery failed"
    );
    // The entry point (oracle 1) is discovered too.
    assert!(
        prog.lookup_symbol("sub_1400014f0").is_some(),
        "stripped PE: discovered entry (sub_1400014f0) not registered"
    );

    let out = decompile_func(prog, "load function sub_140001592");

    // A real decompilation of a discovered, never-symboled function: it names the
    // function and emits a C body — not the "no function" error.
    assert!(out.contains("sub_140001592"), "expected a body for sub_140001592, got:\n{out}");
    assert!(out.contains('{') && out.contains('}'), "expected a C body, got:\n{out}");
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "sub_140001592 should resolve to a function, got:\n{out}"
    );
    // Bonus: the discovered `main` calls the IAT-named `puts` (PR-4) — a stripped
    // PE recovers BOTH the function (PR-12) and its import name (PR-4).
    assert!(out.contains("puts("), "expected the IAT-named puts call, got:\n{out}");
}

/// PR-13: a **stripped Mach-O** finds the file-local `helper` via
/// `LC_FUNCTION_STARTS` (its symbol was removed by `ld64.lld -x`), so
/// `sub_100000590` resolves and decompiles with NO supplied address.
#[test]
fn macho_stripped_discovers_helper_via_function_starts() {
    let Some(prog) = boot_committed("macho_func_starts_stripped") else { return };

    // `helper`@0x100000590 has NO symbol (stripped); only LC_FUNCTION_STARTS lists
    // it. The angr-style discovered name is `sub_100000590`.
    assert!(
        prog.lookup_symbol("sub_100000590").is_some(),
        "stripped Mach-O: discovered `helper` (sub_100000590) not registered — \
         LC_FUNCTION_STARTS entry discovery failed"
    );

    let out = decompile_func(prog, "load function sub_100000590");

    assert!(out.contains("sub_100000590"), "expected a body for sub_100000590, got:\n{out}");
    assert!(out.contains('{') && out.contains('}'), "expected a C body, got:\n{out}");
    assert!(
        !out.contains("Unknown function") && !out.contains("no function"),
        "sub_100000590 should resolve to a function, got:\n{out}"
    );
}

/// Multi-format is the default: a bare `load file` of a stripped PE / Mach-O
/// routes to the object loader with no flag (the ELF arm and the 675/158 oracles
/// are structurally immune to the new entry oracles). A `.sla`-absent environment
/// surfaces as a load error; we only assert the dispatch ROUTES to the object
/// loader (no XML "not recognized" rejection).
#[test]
fn default_on_loads_stripped_pe_and_macho() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];

    for name in ["pe_imports_stripped.exe", "macho_func_starts_stripped"] {
        let path = fixtures().join(name);
        if let Err(e) =
            kuna_console::engine::bootstrap_from_file(path.to_str().unwrap(), "", &spec_roots)
        {
            let msg = e.explain();
            assert!(
                !msg.contains("Unable to recognize") && !msg.contains("XML"),
                "default-on: {name} must route to the object loader (got: {msg})"
            );
        }
    }
}
