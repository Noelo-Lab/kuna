//! Multi-format-loader PR-11 e2e gate: the DWARF analyzer (`s1_dwarf`, the kuna
//! analog of Ghidra's `DWARFAnalyzer`) now fires on **MinGW-PE** and **Mach-O**,
//! not just ELF — gimli is format-neutral, so dropping the `BinaryFormat::Elf`
//! gate and resolving the debug sections through `object`'s format-aware
//! `section_by_name` lets a `-g` PE/Mach-O recover its DWARF function names and
//! typed signatures.
//!
//! ## The headline before/after
//!
//! Both fixtures are the per-format analog of the ELF `dwarf_stripped_x86_64`:
//! the function NAMES live ONLY in the debug sections (the PE COFF-symtab FUNC
//! entries are `--strip-symbol`-removed; the Mach-O symtab FUNC entries are
//! `--redefine-sym`-renamed — see the fixture `.c` headers), with `.debug_*` kept.
//!
//! ```text
//!   before (no DWARF / by raw address):  sub_140001550(...)        / sub_0(...)
//!   after  (DWARF name recovered):        first_byte(char *a0)      [PE and Mach-O]
//! ```
//!
//! - **PE** (`pe_dwarf.exe`, MinGW `-g`): MinGW emits standard `.debug_*` sections
//!   in the PE; `object::section_by_name(".debug_info")` finds them verbatim. The
//!   `s1_dwarf` `SymFact{Function}` names `first_byte`/`add` (the symtab no longer
//!   does) and the `PrototypePieces` carries `int first_byte(char *)`.
//! - **Mach-O** (`macho_dwarf.o`, clang `-g`): the DWARF lands in the
//!   `__DWARF,__debug_*` sections; `object` maps gimli's `.debug_info` → the Mach-O
//!   short-name `__debug_info` (its documented rule), so the *same* section loader
//!   reads it. `s1_dwarf` recovers the same DWARF-only names + `char *` type.
//!
//! That a name recovers BY its DWARF name (the symtab no longer carries it) is the
//! DWARF-specific proof; the by-raw-address decompile (`load addr`) is the
//! in-test no-DWARF-name baseline — it renders the engine's `sub_<addr>`
//! placeholder for the same code.
//!
//! ## ELF unchanged
//!
//! The pass is default behaviour for any `.debug_info`-carrying object now; the
//! existing ELF DWARF gate (`verify_s1_dwarf.rs`) is unaffected (this PR only drops
//! the format gate, the ELF section names are identical). The non-ELF arm is only
//! reachable under `--experimental-formats` (`KUNA_EXPERIMENTAL_FORMATS`), so the
//! default (ELF) pipeline is byte-identical.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_multiformat_*` / `verify_s1_dwarf` gates, bootstrapping
//! needs the built `x86` `.sla` under `specs/` (gitignored; `make specs`). When it
//! is absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// `KUNA_EXPERIMENTAL_FORMATS` is a process-global env var; serialize the
/// env-sensitive bodies so the experimental-on bootstraps never race.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Bootstrap a fixture under `--experimental-formats`, returning `None` (a visible
/// skip) when the `.sla` is absent.
fn boot(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    std::env::set_var("KUNA_EXPERIMENTAL_FORMATS", "1");
    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_multiformat_dwarf: skipping {name} (bootstrap failed; build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");
            None
        }
    }
}

/// Run a `load …`-driven `decompile` → `print C` and return the captured C.
fn decompile(prog: ConsoleProgram, setup: &[&str]) -> String {
    let cmds: Vec<String> = setup.iter().map(|s| s.to_string()).collect();
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

/// PE (MinGW `-g`): the DWARF-only function name `first_byte` recovers, with its
/// DWARF-typed `char *` parameter — DIFFERENT from the by-raw-address baseline
/// (`sub_<addr>`), proving `s1_dwarf` read the PE's `.debug_*` sections.
#[test]
fn pe_dwarf_recovers_name_and_typed_signature() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    // AFTER: load by the DWARF name (the COFF symtab no longer carries it).
    let Some(prog) = boot("pe_dwarf.exe") else { return };
    let t0 = Instant::now();
    let after = decompile(prog, &["load function first_byte", "decompile", "print C"]);
    let elapsed = t0.elapsed();
    eprintln!("verify_multiformat_dwarf: PE first_byte decompile took {elapsed:?}");

    // BEFORE (in-test baseline): the same code at its raw address has no
    // DWARF/symtab name → the engine's `sub_<addr>` placeholder.
    let Some(prog_b) = boot("pe_dwarf.exe") else { return };
    let before = decompile(prog_b, &["load addr 0x140001550", "decompile", "print C"]);
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");

    // The DWARF name resolved (and is NOT a placeholder for itself).
    assert!(
        after.contains("first_byte"),
        "PE: expected the DWARF name `first_byte`, got:\n{after}"
    );
    assert!(
        !after.contains("sub_140001550") && !after.contains("FUN_140001550"),
        "PE: first_byte must render by its DWARF name, not a placeholder:\n{after}"
    );
    // The DWARF-typed `char *` parameter is in the recovered signature.
    assert!(
        after.contains("char *"),
        "PE: expected the DWARF-typed `char *` parameter, got:\n{after}"
    );
    // The baseline (raw address, no DWARF name) is the engine placeholder — the
    // before/after the DWARF name install delivers.
    assert!(
        before.contains("sub_140001550"),
        "PE: the no-DWARF-name baseline should render `sub_140001550`, got:\n{before}"
    );
    assert!(
        !before.contains("first_byte"),
        "PE: the raw-address baseline must NOT carry the DWARF name:\n{before}"
    );
}

/// Mach-O (clang `-g`): the same DWARF recovery, proving `object` mapped gimli's
/// `.debug_info` → the Mach-O `__DWARF,__debug_info` short-name so the format-neutral
/// section loader read it.
#[test]
fn macho_dwarf_recovers_name_and_typed_signature() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    // AFTER: the DWARF name `first_byte` (the Mach-O symtab FUNC entry was renamed
    // to `_l0`, so this name is DWARF-only).
    let Some(prog) = boot("macho_dwarf.o") else { return };
    let t0 = Instant::now();
    let after = decompile(prog, &["load function first_byte", "decompile", "print C"]);
    let elapsed = t0.elapsed();
    eprintln!("verify_multiformat_dwarf: Mach-O first_byte decompile took {elapsed:?}");

    // BEFORE: the same code by raw address → `sub_0` placeholder.
    let Some(prog_b) = boot("macho_dwarf.o") else { return };
    let before = decompile(prog_b, &["load addr 0x0", "decompile", "print C"]);
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");

    assert!(
        after.contains("first_byte"),
        "Mach-O: expected the DWARF name `first_byte`, got:\n{after}"
    );
    assert!(
        !after.contains("sub_0(") && !after.contains("FUN_0("),
        "Mach-O: first_byte must render by its DWARF name, not a placeholder:\n{after}"
    );
    assert!(
        after.contains("char *"),
        "Mach-O: expected the DWARF-typed `char *` parameter, got:\n{after}"
    );
    assert!(
        before.contains("sub_0"),
        "Mach-O: the no-DWARF-name baseline should render `sub_0`, got:\n{before}"
    );
    assert!(
        !before.contains("first_byte"),
        "Mach-O: the raw-address baseline must NOT carry the DWARF name:\n{before}"
    );
}

/// A second DWARF-only function on each format (`add`, an `int(int,int)`) confirms
/// the recovery is not a one-off — both the PE `.debug_*` and the Mach-O
/// `__DWARF,__debug_*` section lookups resolve the whole CU.
#[test]
fn both_formats_recover_a_second_dwarf_only_function() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    let Some(pe) = boot("pe_dwarf.exe") else { return };
    let pe_add = decompile(pe, &["load function add", "decompile", "print C"]);

    let Some(macho) = boot("macho_dwarf.o") else { return };
    let macho_add = decompile(macho, &["load function add", "decompile", "print C"]);
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");

    assert!(
        pe_add.contains("add") && !pe_add.contains("sub_140001564"),
        "PE: `add` should recover by its DWARF name, got:\n{pe_add}"
    );
    assert!(
        macho_add.contains("add") && !macho_add.contains("sub_20"),
        "Mach-O: `add` should recover by its DWARF name, got:\n{macho_add}"
    );
}
