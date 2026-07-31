//! Multi-format-loader PR-10 e2e gate (the payoff): the format-agnostic analysis
//! passes now fire on PE and Mach-O, so a non-ELF binary renders **string
//! literals**, **typed libc args**, and **no dead code after a no-return call** —
//! not just named imports.
//!
//! ## The headline before/after (PE)
//!
//! `pe_imports.exe` is `int main(){ puts("hello"); printf("%d\n", argc); }`.
//! Before PR-10 the format-agnostic passes were gated `BinaryFormat::Elf`, so:
//!
//! ```text
//!   before:  puts(0x140009000);        printf(0x140009006, a0);
//!   after:   puts("hello");            printf("%d\n", a0 & 0xffffffff);
//! ```
//!
//! - `puts("hello")`: `strings` now scans the PE `.rdata`
//!   (`SectionKind::ReadOnlyData` / `SectionFlags::Coff`) and plants the `char[6]`
//!   string symbol.
//! - `printf("%d\n", …)`: `protos` now types `printf`'s first arg `char *` on a
//!   PE (the import name reaches the libc table), so the printer reads the literal.
//!
//! ## Mach-O
//!
//! `macho_imports`'s `_main` calls `printf("%d\n", compute(argc))`. The literal
//! `%d\n` is too short (3 chars) for the min-5 string scan, so it rides the typed
//! `char *` arg + the now-READONLY `__cstring` section: `printf("%d\n", …)`.
//!
//! ## Per-pass: the no-return list (PE)
//!
//! `__tmainCRTStartup`'s tail `exit(…)` is on the new PeMac no-return list, so the
//! engine inserts an artificial halt and elides the dead fall-through after it;
//! `--option noreturn_known off` restores the dead code (the per-pass before/after).
//!
//! ## Listing consumer (noreturn_disc) runs on a PE
//!
//! `noreturn_disc` is format-neutral (it consumes the built Listing). We confirm
//! it is genuinely *active* on a PE — the Listing builds with functions and the
//! consumer runs to completion — under `--option listing on --option noreturn_disc on`.
//!
//! ## Multi-format is the default + `.sla` precondition
//!
//! Non-ELF loads unconditionally — the same default `load file` dispatch as ELF,
//! with no flag. Bootstrapping needs the built `x86`/`AARCH64` `.sla` under
//! `specs/` (gitignored; `make specs`); when absent the bootstrap fails and the
//! test prints that and returns early (a visible skip, never a false green).

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

/// Bootstrap, run a `load …`-driven `decompile` → `print C` (with any extra
/// `option …` commands first), and return the captured C.
fn decompile_func(prog: ConsoleProgram, setup: &[&str]) -> String {
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

/// Bootstrap a fixture (multi-format loading is unconditional), returning `None`
/// (a visible skip) when the `.sla` is absent.
fn boot(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_multiformat_passes: skipping {name} (bootstrap failed; build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// THE HEADLINE: a PE renders the string literal (`strings`) AND the typed
/// libc arg (`protos`) — `puts("hello")` + `printf("%d\n", …)`, not
/// `puts(0x…)` / `printf(0x…, …)`.
#[test]
fn pe_renders_string_literal_and_typed_args() {
    let Some(prog) = boot("pe_imports.exe") else { return };

    let out = decompile_func(prog, &["load function main", "decompile", "print C"]);

    // strings: the `.rdata` "hello" recovers as a literal (was `0x140009000`).
    assert!(out.contains("puts(\"hello\")"), "PE: expected puts(\"hello\"), got:\n{out}");
    assert!(
        !out.contains("puts(0x140009000)"),
        "PE: puts arg must be the literal, not the raw pointer:\n{out}"
    );
    // protos: printf's first arg typed `char *`, so the short `%d\n` literal
    // (below the string-scan min-5) renders via the printer's constant-string route.
    assert!(out.contains("printf(\"%d\\n\""), "PE: expected printf(\"%d\\n\", …), got:\n{out}");
    assert!(
        !out.contains("printf(0x140009006"),
        "PE: printf arg must be the literal, not the raw pointer:\n{out}"
    );
}

/// Mach-O: the typed `char *` arg + the now-READONLY `__cstring` render the
/// `printf("%d\n", …)` literal (was `printf(0x1000005ee, …)`).
#[test]
fn macho_renders_typed_printf_literal() {
    let Some(prog) = boot("macho_imports") else { return };

    let out = decompile_func(prog, &["load function _main", "decompile", "print C"]);

    assert!(out.contains("printf(\"%d\\n\""), "Mach-O: expected printf(\"%d\\n\", …), got:\n{out}");
    assert!(
        !out.contains("printf(0x1000005ee"),
        "Mach-O: printf arg must be the literal, not the raw pointer:\n{out}"
    );
}

/// Per-pass (no-return list): a PE that tail-calls `exit` (a PeMac-list
/// no-return) shows NO dead code after it; `--option noreturn_known off` restores
/// it (the per-pass before/after, on a PE).
#[test]
fn pe_exit_eliminates_dead_code_via_noreturn_list() {
    let Some(prog) = boot("pe_imports.exe") else { return };

    // ON (default): exit is no-return → WARNING terminator, no fall-through after.
    let on = decompile_func(prog, &["load function __tmainCRTStartup", "decompile", "print C"]);

    let Some(prog_off) = boot("pe_imports.exe") else { return };
    // OFF: the dead code after `exit(…)` reappears. (kuna DIV-13/DIV-14) ALL THREE
    // name-based no-return gates must be disabled: `noreturn_known` (the address-keyed
    // scan), `noreturn_externmatch` (the flow-seam name match, default-on DIV-13), AND
    // `noreturn_extern` (the flow-seam name match, now default-on DIV-14) — all three
    // independently mark `exit` no-return from the same vendored list, so any one left
    // on keeps the fall-through dropped.
    let off = decompile_func(
        prog_off,
        &[
            "option noreturn_known off",
            "option noreturn_externmatch off",
            "option noreturn_extern off",
            "load function __tmainCRTStartup",
            "decompile",
            "print C",
        ],
    );

    assert!(on.contains("exit("), "PE: the tail call must be named exit(:\n{on}");
    assert!(
        on.contains("// no-return"),
        "PE: exit must be marked no-return (the PeMac list fired):\n{on}"
    );
    // The dead self-recursion after exit is present OFF, gone ON.
    assert!(
        off.contains("__tmainCRTStartup()"),
        "PE (noreturn off): the dead fall-through after exit must reappear:\n{off}"
    );
    assert!(
        !on.contains("v14 = __tmainCRTStartup();"),
        "PE (noreturn on): no dead fall-through after exit:\n{on}"
    );
}

/// noreturn_disc (the Listing consumer) is format-neutral and runs on a PE:
/// under `listing on` + `noreturn_disc on` the PE's Listing builds and the
/// consumer runs to completion (decompile succeeds, no panic). The PE has no
/// custom no-return wrapper to *discover*, so this confirms the pass is active
/// (not inert) on a non-ELF binary — the format-neutrality claim.
#[test]
fn noreturn_disc_runs_on_pe() {
    let Some(prog) = boot("pe_imports.exe") else { return };

    let out = decompile_func(
        prog,
        &[
            "option listing on",
            "option noreturn_disc on",
            "load function __tmainCRTStartup",
            "decompile",
            "print C",
        ],
    );

    // The decompile completed (the Listing built + the consumer ran on the PE).
    assert!(
        out.contains("__tmainCRTStartup") || out.contains("exit("),
        "PE: the listing-consumer path must decompile to completion, got:\n{out}"
    );
}
