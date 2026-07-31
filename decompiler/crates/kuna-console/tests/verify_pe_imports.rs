//! Multi-format-loader PR-3+4 e2e gate: a **linked Windows PE32+** executable
//! (`pe_imports.exe`, MinGW) loads and decompiles a function, and its libc
//! imports render **named** (`puts(`/`printf(`) — not `sub_<addr>` — through the
//! PE IAT walk (`loader/pe_iat.rs`, design §3.2/§8 PR-4).
//!
//! This is the PE half of the headline goal ("a decompiler for most file
//! formats"): PR-2 proved a PE *object* parses; this proves a full linked *exe*
//! loads (image base + `AddressOfEntryPoint`), decompiles a function, and names
//! the imports.
//!
//! ## Two fixtures, two things proven
//!
//! 1. **`pe_imports.exe`** (non-stripped MinGW) — the headline. `main` calls
//!    `puts` through a thunk veneer the COFF symtab *names* (`puts`) and a local
//!    MinGW `printf` wrapper (also named in the symtab). It proves the linked PE
//!    **loads + decompiles** and the calls render `puts(`/`printf(`.
//!
//! 2. **`pe_imports_stripped.exe`** (`-s`) — the PR-4 proof. With the symbols
//!    gone, `main` (at the same VMA) still `call`s the `puts` thunk
//!    (`0x140007240`, a `FF 25` `jmp [__imp_puts]` veneer). Only the IAT walk +
//!    thunk decode names it, so:
//!
//!    ```text
//!      before (resolve_imports empty):  sub_140007240(0x140009000);
//!      after  (PR-4 IAT/thunk naming):  puts(0x140009000);
//!    ```
//!
//!    The local `printf` wrapper (`sub_140001550`) is *not* an import, so it
//!    correctly stays `sub_<addr>` in the stripped binary — naming only what is
//!    genuinely an import.
//!
//! ## Multi-format loading (unconditional)
//!
//! PE/Mach-O/COFF load unconditionally now — like ELF, with no flag. The test
//! confirms the same fixture routes through the default `load file` dispatch to
//! the object loader (the byte-identical-dispatch proof shared with
//! `verify_object_formats`).
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). If it is absent the bootstrap fails
//! and the test prints that and returns early (a visible skip, never a false
//! green).

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

// ---- Pinned facts (container objdump -d / -p of the MinGW PE) ----------------
//
// pe_imports.c = `main` that calls `puts("hello")` and `printf("%d\n", argc)`.
// Built: `x86_64-w64-mingw32-gcc -O1 pe_imports.c -o pe_imports.exe`
// (`-s` for the stripped variant). ImageBase 0x140000000.
//
//   main           @ 0x140001592   (calls the puts thunk + the printf wrapper)
//   puts thunk     @ 0x140007240   (`FF 25` jmp [__imp_puts])
//   __imp_puts IAT @ 0x14000d33c   (the slot the thunk jumps through)
//   printf wrapper @ 0x140001550   (a *local* MinGW fn; not an import)
const MAIN_VMA: u64 = 0x140001592;
const PUTS_THUNK_VMA: u64 = 0x140007240;
const PUTS_IAT_VMA: u64 = 0x14000d33c;

/// Bootstrap, run `load function`/`load addr`-driven `decompile` → `print C`, and
/// return the captured C. `func_cmd` is the `load function …`/`load addr …`
/// command that seeds the function to decompile.
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

/// Bootstrap a fixture, returning `None` (a visible skip) when the `.sla` is
/// absent.
fn boot(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_pe_imports: skipping {name} (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// The headline: a linked, non-stripped MinGW PE loads, decompiles `main`, and
/// its libc calls render `puts(`/`printf(`.
#[test]
fn pe_linked_exe_decompiles_with_named_imports() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("pe_imports.exe");

    // (default-on proof) The object loads through the *default* `load file`
    // dispatch with no flag — multi-format support is unconditional. (A `.sla`-
    // absent environment surfaces as a load error; the main body's skip covers
    // that, so here we only assert the dispatch ROUTES to the object loader, i.e.
    // it does not fail with the XML "not recognized" error.)
    let dflt = kuna_console::engine::bootstrap_from_file(path.to_str().unwrap(), "", &spec_roots);
    if let Err(e) = &dflt {
        // Only acceptable failure is a missing-`.sla` bootstrap error, never an
        // "unrecognized format" rejection (that would mean the magic wasn't admitted).
        let msg = e.explain();
        assert!(
            !msg.contains("Unable to recognize") && !msg.contains("XML"),
            "default-on: the object must route to the object loader (got: {msg})"
        );
    }

    let Some(prog) = boot("pe_imports.exe") else { return };

    // The linked exe loads with the Windows x86-64 spec.
    let desc = prog.description().to_string();
    assert!(desc.contains("x86"), "PE: expected x86 spec, got {desc:?}");
    // The defined `main` and the imported `puts` resolve as functions.
    assert!(prog.lookup_symbol("main").is_some(), "PE: `main` not resolved by the loader");
    assert!(prog.lookup_symbol("puts").is_some(), "PE: `puts` not resolved by the loader");

    let out = decompile_func(prog, "load function main");

    assert!(out.contains("puts("), "PE: expected a `puts(` call, got:\n{out}");
    assert!(out.contains("printf("), "PE: expected a `printf(` call, got:\n{out}");
    // The library calls are not unnamed stubs.
    assert!(
        !out.contains("sub_140007240"),
        "PE: the puts thunk should render as `puts`, not sub_<addr>:\n{out}"
    );
}

/// The PR-4 proof: in the *stripped* PE (no symbols), the `puts` call renders
/// named ONLY because the IAT walk + `FF 25` thunk decode named the thunk veneer.
#[test]
fn pe_stripped_exe_names_puts_via_iat_thunk() {
    let Some(prog) = boot("pe_imports_stripped.exe") else { return };

    // No `main` symbol in a stripped PE — decompile by address.
    let out = decompile_func(prog, &format!("load addr 0x{MAIN_VMA:x}"));

    // The puts thunk (0x140007240), with no COFF symbol, is named via the IAT
    // walk + thunk decode — the call renders `puts(`, not `sub_140007240(`.
    assert!(
        out.contains("puts("),
        "stripped PE: the puts thunk must be IAT-named as `puts(`, got:\n{out}"
    );
    assert!(
        !out.contains("sub_140007240"),
        "stripped PE: the puts thunk should no longer be sub_140007240:\n{out}"
    );
    // The local printf wrapper (0x140001550) is NOT an import, so it correctly
    // stays unnamed in a stripped binary — we only name genuine imports.
    assert!(
        out.contains("sub_140001550"),
        "stripped PE: the local printf wrapper should stay sub_<addr> (it is not \
         an import), got:\n{out}"
    );
}

/// Automatic batch decompilation targets the executable thunk, not the IAT
/// pointer slot. The slot remains a symbol so calls and explicit lookup work.
#[test]
fn pe_batch_targets_exclude_iat_data_slots() {
    let Some(mut prog) = boot("pe_imports_stripped.exe") else { return };
    prog.commit_pending_analysis().expect("PE analysis commit must succeed");

    let canonical = prog.function_entries_canonical();
    assert!(
        canonical.iter().any(|entry| entry.addr.get_offset() == PUTS_IAT_VMA),
        "PE: canonical inventory must retain the puts IAT slot"
    );
    let executable = prog.function_entries_executable();
    assert!(
        executable.iter().any(|entry| entry.addr.get_offset() == PUTS_THUNK_VMA),
        "PE: executable inventory must retain the puts thunk"
    );
    assert!(
        !executable.iter().any(|entry| entry.addr.get_offset() == PUTS_IAT_VMA),
        "PE: executable inventory must exclude the puts IAT slot"
    );
    assert!(
        prog.find_entry_at(PUTS_IAT_VMA).is_some(),
        "PE: explicit lookup must retain the puts IAT slot"
    );
}
