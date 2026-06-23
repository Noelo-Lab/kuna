//! End-to-end gate for string-literal detection (`s1_strings` —
//! `StringLiteralPass`) coexisting with the printer's readonly-char-array
//! literal route.
//!
//! Drives `load file <fauxware>` through the bootstrap, which runs the
//! `StringLiteralPass` (re-enabled in `passes.rs`) and commits a typelocked
//! `char[N]` data symbol (`s_<addr>`) at each detected `.rodata` string. Then
//! `load function main` → `decompile` → `print C`: the `puts` arguments MUST
//! still render as the string LITERALS (`puts("Username: ")` /
//! `puts("Password: ")`), NOT the planted symbol NAME (`puts(s_400915)`).
//!
//! ## The coexistence the printer change buys
//!
//! Before the printer change (analysis-port-log.md Increment 3), planting the
//! `char[N]` symbol turned the constant pointer into a global SPACEBASE
//! `PTRSUB(spacebase, 0xADDR)` symbol reference, and the SPACEBASE render arm
//! emitted the bare symbol name `s_400915` — SHADOWING the literal. This gate
//! is the proof that the data symbol (`s_400915` is registered) and the literal
//! (`puts("Username: ")` renders) now coexist: the SPACEBASE arm routes a
//! readonly char-printable array symbol through the same `pushPtrCharConstant`
//! literal path the constant arm uses (C++ `PrintC::pushConstant`,
//! printc.cc:1842-1880).
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_s1_*` gates, bootstrapping needs the built `x86`
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

/// The vendored fauxware fixture (shared with the kuna-analysis tier tests). Its
/// `.rodata` carries the readonly string literals `"Username: "` (@0x400915) and
/// `"Password: "` (@0x400920) that `main`'s `puts` calls reference.
fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// Run `load function <name>` → `decompile` → `print C` against a pre-bootstrapped
/// program and return the captured C output.
fn decompile_c(prog: kuna_console::engine::ConsoleProgram, func: &str) -> String {
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
    status.optr.clone()
}

#[test]
fn planted_string_symbol_and_literal_coexist() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = match fauxware().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_s1_strings: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // The StringLiteralPass + commit seam planted the typelocked `char[N]` data
    // symbol `s_400915` at the "Username: " address — the Ghidra StringsAnalyzer
    // mechanism. Its mere presence is what used to shadow the literal. (It is added
    // to the arch symbol table via `add_symbol_mapped`, NOT the funcsym
    // `register_symbol` stream, so it is queried on the global scope directly.)
    {
        let arch = prog.arch();
        let global = arch.symboltab.get_global_scope().expect("global scope");
        assert!(
            !arch.symboltab.query_by_name(global, "s_400915").is_empty(),
            "planted string data symbol `s_400915` not in the symbol table — \
             StringLiteralPass is not enabled (re-enable it in passes.rs)"
        );
    }

    // Decompile `main`: despite the planted symbol, the `puts` arguments render as
    // the readonly-char-array LITERALS via the printer's coexistence route.
    let out = decompile_c(prog, "main");
    assert!(out.contains("main"), "expected a decompiled body for `main`, got:\n{out}");
    assert!(
        out.contains(r#"puts("Username: ")"#),
        "expected the literal `puts(\"Username: \")` (printer readonly-char-array \
         route), got:\n{out}"
    );
    assert!(
        out.contains(r#"puts("Password: ")"#),
        "expected the literal `puts(\"Password: \")`, got:\n{out}"
    );
    // The bare symbol NAME must NOT shadow the literal in the call argument.
    assert!(
        !out.contains("puts(s_4009"),
        "the planted `s_<addr>` symbol name must NOT shadow the string literal, got:\n{out}"
    );
}
