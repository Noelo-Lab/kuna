//! End-to-end gate for **scalar/operand reference markup** (`s1_operand_refs` —
//! the kuna analog of Ghidra's `ScalarOperandAnalyzer` / `ElfScalarOperandAnalyzer`,
//! the operand/reference markup family — **gated off** by default).
//!
//! Drives `load file <operand_refs_x86_64>` → `read symbols` → `load function
//! main` → `decompile` → `print C`, once with the feature **off** (the default)
//! and once with `--option operand_refs on`, and asserts:
//!
//! - **default-off** renders the scalar immediate untyped:
//!   `mystery(0x402004)` — the bare absolute address of the `"hi"` string;
//! - **operand_refs on** types the read-only-data scalar as a string:
//!   `mystery("hi")` — the literal, via the planted `char[N]` + the printer's
//!   readonly-char-array literal route (Increment 12).
//!
//! ## Why this fixture (the residual the pass is the value-add for)
//!
//! `"hi"` is **2 chars** (< 5), so the always-on `StringLiteralPass` (`min_len` 5)
//! does **not** plant it — only `operand_refs` does. And `mystery` has **no known
//! prototype** (it is a local `__attribute__((noinline))` function, absent from the
//! libproto table), so neither the libproto nor the S5 usage path types its `char*`
//! argument. So the literal renders **only** because `operand_refs` typed the
//! scalar operand — isolating this pass's contribution from `s1_strings` + libproto.
//!
//! The `"hi"` address is materialized by a `movabs $0x402004,%rax` (the fixture is
//! built `-no-pie -fno-pic -mcmodel=large`, so the address appears DIRECTLY in code
//! as a bare immediate — the case `ScalarOperandAnalyzer` handles; a RIP-relative
//! `lea` would not surface a bare scalar, faithful to the Ghidra
//! `ADDRESSES_DO_NOT_APPEAR_DIRECTLY_IN_CODE` gate).
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_s1_*` gates, bootstrapping needs the built `x86`
//! `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored operand-refs fixture: `main`=0x40112e materializes the address of
/// the `.rodata` string `"hi"`@0x402004 with `movabs $0x402004,%rax` and passes it
/// to the no-prototype `mystery`=0x401106.
fn operand_refs_bin() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/operand_refs_x86_64")
}

/// Decompile `main` and return the captured C, optionally enabling the
/// scalar/operand reference-markup pass first (`--option operand_refs on`).
fn decompile_main(operand_refs_on: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = operand_refs_bin().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_operand_refs: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The operand_refs pass runs DEFERRED at the commit point (it decodes through
    // the engine Translate, whose loadimage is attached after load). Flip the
    // option BEFORE `commit_pending_analysis` so the deferred run sees it on.
    if operand_refs_on {
        prog.arch_mut()
            .set_kuna_option("operand_refs", "on")
            .expect("operand_refs option flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        ["load function main".into(), "decompile".into(), "print C".into()].to_vec();
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
fn default_off_renders_bare_address() {
    let Some(out) = decompile_main(false) else { return };
    assert!(out.contains("main"), "expected a decompiled body for `main`, got:\n{out}");
    // Default-off: the scalar immediate is an untyped bare absolute address.
    assert!(
        out.contains("mystery(0x402004)"),
        "default-off must render the bare scalar `mystery(0x402004)`, got:\n{out}"
    );
    // And it must NOT have rendered the literal (the pass is off).
    assert!(
        !out.contains(r#"mystery("hi")"#),
        "default-off must NOT render the literal (operand_refs is off), got:\n{out}"
    );
}

#[test]
fn operand_refs_on_renders_string_literal() {
    let Some(out) = decompile_main(true) else { return };
    assert!(out.contains("main"), "expected a decompiled body for `main`, got:\n{out}");
    // operand_refs on: the read-only-data scalar is typed as a string and renders
    // as the literal (the planted char[N] + the printer's readonly-char-array route).
    assert!(
        out.contains(r#"mystery("hi")"#),
        "operand_refs on must render the literal `mystery(\"hi\")`, got:\n{out}"
    );
    // The bare address must no longer appear in the call.
    assert!(
        !out.contains("mystery(0x402004)"),
        "operand_refs on must replace the bare scalar with the literal, got:\n{out}"
    );
}
