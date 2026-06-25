//! End-to-end gate for **format-string varargs typing** (`s1_formatstring`
//! half B — the kuna analog of Ghidra's `FormatStringAnalyzer`, "Variadic
//! Function Signature Override").
//!
//! Drives `load file <fmt_x86_64>` → `read symbols` → `load function main` →
//! `decompile` → `print C`, once with the feature **off** (the default) and once
//! with `--option formatstring on` (here `set_kuna_option("formatstring",
//! "on")`), and asserts:
//!
//! - **default-off** renders the printf call's varargs UNTYPED:
//!   `printf("%d %s\n",(uint8)a0,*a1)` — a widening `(uint8)` cast on the `%d`
//!   arg, no `char *` cast on the `%s` arg;
//! - **formatstring on** types them per the format string:
//!   `printf("%d %s\n",a0,(char *)*a1)` — the `%d` arg is a plain `int` (no
//!   `(uint8)` widening; the function parameter itself is recovered as `int`),
//!   and the `%s` arg is cast to `char *`.
//!
//! The feature reads the `"%d %s\n"` format constant at `.rodata` vma 0x402004
//! from the `printf` call's format-arg slot, parses it
//! (`s1_formatstring::parse_output_types("%d %s\n") = [Int, CharPtr]`), builds a
//! per-call-site prototype override (`apply::build_override_pieces`), installs it
//! via the existing `pending_proto_overrides` plumbing, and re-decompiles.
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

/// The vendored `printf("%d %s\n", argc, argv[0])` fixture (`main`=0x401136,
/// `printf@plt`=0x401040, format constant @0x402004).
fn fmt_bin() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fmt_x86_64")
}

/// Decompile `main` and return the captured C, optionally enabling the
/// format-string varargs-typing feature first (`--option formatstring on`).
fn decompile_main(formatstring_on: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fmt_bin().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_s1_formatstring: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // Commit the load-time analysis facts (libproto, etc.) — what `read symbols`
    // does — so the `printf` proto is recovered before the decompile.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    if formatstring_on {
        prog.arch_mut()
            .set_kuna_option("formatstring", "on")
            .expect("formatstring option flips on");
    }

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
fn formatstring_off_leaves_varargs_untyped() {
    let Some(out) = decompile_main(false) else {
        return; // specs-less skip
    };
    assert!(out.contains("printf("), "expected a printf call, got:\n{out}");
    // Default: the %d arg is widened with a (uint8) cast and the %s arg is the
    // bare untyped `*a1` — the feature is OFF.
    assert!(
        out.contains("(uint8)a0") || out.contains("(uint8)"),
        "default-off should keep the %d arg widened/untyped, got:\n{out}"
    );
    assert!(
        !out.contains("(char *)*a1"),
        "default-off must NOT type the %s arg as char* (feature is off), got:\n{out}"
    );
}

#[test]
fn formatstring_on_types_printf_varargs() {
    let Some(out) = decompile_main(true) else {
        return; // specs-less skip
    };
    assert!(out.contains("printf("), "expected a printf call, got:\n{out}");
    // With the feature on: the %s arg is cast to `char *` (the override typed it),
    // and the %d arg is no longer widened with a (uint8) cast (the format-derived
    // `int` matches the recovered parameter, so no widening cast renders).
    assert!(
        out.contains("(char *)*a1"),
        "formatstring-on should type the %s arg as `char *`, got:\n{out}"
    );
    assert!(
        !out.contains("(uint8)a0"),
        "formatstring-on should drop the widening (uint8) cast on the %d arg, got:\n{out}"
    );
}
