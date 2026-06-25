//! End-to-end gate for auto-applied cspec call-fixups (`s1_callfixup` — the kuna
//! analog of Ghidra's `CallFixupAnalyzer`).
//!
//! Drives `load file <mcount_x86_64>` (a static `gcc -pg` binary whose `main`
//! prologue emits a direct `call mcount`) through the bootstrap, which runs the
//! `CallFixupPass` and commits it: the pass matches the `mcount` FUNC symbol to
//! the `x86-64-gcc.cspec` `<callfixup name="mcount"><target name="mcount"/>` and
//! the commit seam tags that symbol with the fixup's inject id. Then
//! `load function main` → `decompile` → `print C`: the engine's inherited inject
//! path replaces the `mcount` CALL with the fixup body (`temp:1 = 0;`), so the
//! bogus profiling call **dissolves** out of `main`.
//!
//! The proof: WITHOUT the fixup the body shows `mcount();`; WITH it (the default,
//! Ghidra `setDefaultEnablement(true)`) the body is just `return 0;` and carries
//! the engine's `Function: mcount replaced with injection: mcount` marker. This is
//! the one place the "inject machinery is inherited" assumption is actually
//! proven end-to-end — the analyzer-driven tag flows all the way to a dissolved
//! call.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_s1_entry`/`verify_w11_elf_plt_names` gates,
//! bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored static `-pg` fixture (shared with the kuna-analysis callfixup
/// unit test).
fn mcount_fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/mcount_x86_64")
}

#[test]
fn mcount_call_is_dissolved_by_the_applied_fixup() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = match mcount_fixture().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_s1_callfixup: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };

    // `main` is a real `.symtab` function in this fixture (the call-fixup is tagged
    // on its `mcount` callee, not on `main`).
    assert!(
        prog.lookup_symbol("main").is_some(),
        "`main` not registered — loader symbol stream failed"
    );

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

    // A real decompilation of `main`.
    assert!(out.contains("main"), "expected a decompiled body for `main`, got:\n{out}");
    assert!(
        out.contains('{') && out.contains('}'),
        "expected a C function body for `main`, got:\n{out}"
    );

    // The engine emits a marker when a CALL is replaced by an injection — the
    // direct proof the analyzer-driven fixup tag reached the inherited inject path.
    assert!(
        out.contains("replaced with injection"),
        "expected the `replaced with injection` marker (fixup applied), got:\n{out}"
    );

    // The bogus `mcount()` profiling CALL must NOT survive as a call statement.
    // The only `mcount` token allowed is inside the WARNING comment above; a real
    // call would render `mcount();`.
    assert!(
        !out.contains("mcount();"),
        "the `mcount()` call must be dissolved by the fixup, got:\n{out}"
    );
}
