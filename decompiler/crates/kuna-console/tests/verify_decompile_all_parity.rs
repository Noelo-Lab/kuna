//! End-to-end gate for **surface parity** (DIV-66): the whole-binary decompile
//! loop behind `kuna decompile-all` / `decompile-project` / wasm must produce the
//! same C as the console `decompile` command for the same binary, function and
//! options.
//!
//! Both surfaces now go through one shared per-function step
//! (`kuna_console::decompile_step::decompile_one`). Before it existed the console
//! command alone ran Ghidra's `FormatStringAnalyzer` half-B loop — decompile,
//! read the printf/scanf format constants, install per-call-site prototype
//! overrides, decompile again — with read-only propagation enabled for the
//! duration. The whole-binary loop called the drive directly, so `--option
//! formatstring on` was a **silent no-op** there even though `--mode aggressive`
//! (and therefore `auto` under 500 KiB) names it: every whole-binary and
//! benchmark run was measured on the weaker surface.
//!
//! Two properties are asserted over the four vendored
//! `printf("%d %s\n", argc, argv[0])` fixtures (`fmt_x86_64` / `fmt_aarch64` /
//! `fmt_arm` / `fmt_riscv64` — the same ones `verify_formatstring_crossarch`
//! uses, so the ARM read-only literal-pool case is covered too):
//!
//! 1. **Two-pass on the whole-binary surface** — `formatstring` off leaves the
//!    printf varargs untyped; `formatstring` on types the `%s` argument as
//!    `char *`. Off is the bug this PR fixes, on is the fix.
//! 2. **Parity** — for BOTH option states the whole-binary loop's rendered C for
//!    `main` is byte-identical to the console command's.
//!
//! ## `.sla` precondition
//!
//! Like the sibling gates, bootstrapping needs the built per-arch `.sla` under
//! `specs/` (gitignored; `make specs`). When it is absent the bootstrap fails;
//! the test prints that and returns early (a specs-less CI is a visible skip,
//! never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::project::decompile_targets;

/// The four arches whose `fmt_<arch>` fixture is vendored.
const ARCHES: &[&str] = &["x86_64", "aarch64", "arm", "riscv64"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap `fmt_<arch>`, run the analysis commit (`read symbols`), and apply
/// the `formatstring` gate. `None` is a visible specs-less skip.
fn load(arch: &str, formatstring_on: bool) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(format!("fmt_{arch}"))
        .to_str()?
        .to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_decompile_all_parity[{arch}]: skipping (bootstrap failed, build \
                 `.sla` with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    if formatstring_on {
        prog.arch_mut()
            .set_kuna_option("formatstring", "on")
            .expect("formatstring option flips on");
    }
    Some(prog)
}

/// `main` through the WHOLE-BINARY loop (`decompile_targets`, the
/// `decompile-all` / `decompile-project` / wasm path).
fn whole_binary_main(arch: &str, formatstring_on: bool) -> Option<String> {
    let mut prog = load(arch, formatstring_on)?;
    let entry = prog
        .function_entries_canonical()
        .into_iter()
        .find(|e| e.name == "main")
        .expect("the fixture exports main");
    let out = decompile_targets(&mut prog, vec![entry], /* no_vars= */ true, false);
    assert_eq!(out.len(), 1, "[{arch}] one target in, one result out");
    Some(out[0].code.clone().unwrap_or_else(|| {
        panic!("[{arch}] main failed to decompile: {:?}", out[0].error)
    }))
}

/// `main` through the CONSOLE command (`load function` / `decompile` / `print C`
/// — what `kuna decompile` drives in its `decomp_dbg` subprocess).
fn console_main(arch: &str, formatstring_on: bool) -> Option<String> {
    let prog = load(arch, formatstring_on)?;
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
    // Without a `decomp_dbg`-style `openfile` redirect the bulk `print C` output
    // lands in the same buffer as the command notices, so drop the transcript
    // (`Decompiling <fn>` / the re-decompile notice / `Decompilation complete`)
    // and keep the C.  `decompile_targets` trims the surrounding newlines; trim
    // here too.
    let text = status
        .optr
        .rsplit_once("Decompilation complete\n")
        .unwrap_or_else(|| panic!("[{arch}] console decompile did not complete:\n{}", status.optr))
        .1
        .to_string();
    Some(text.trim_matches('\n').to_string())
}

/// Property 1, gate OFF — the bug: the whole-binary surface leaves the printf
/// varargs untyped, exactly as the console surface does with the gate off.
#[test]
fn whole_binary_leaves_varargs_untyped_with_formatstring_off() {
    for arch in ARCHES {
        let Some(c) = whole_binary_main(arch, false) else { continue };
        assert!(c.contains("printf("), "[{arch}] expected a printf call, got:\n{c}");
        assert!(
            !c.contains("(char *)*a1"),
            "[{arch}] formatstring off must NOT type the %s arg, got:\n{c}"
        );
    }
}

/// Property 1, gate ON — the fix: the whole-binary surface types the varargs from
/// the format string. This is what `--mode aggressive` (and `auto` under 500 KiB)
/// asks for and used to silently not get.
#[test]
fn whole_binary_types_varargs_with_formatstring_on() {
    for arch in ARCHES {
        let Some(c) = whole_binary_main(arch, true) else { continue };
        assert!(c.contains("printf("), "[{arch}] expected a printf call, got:\n{c}");
        assert!(
            c.contains("(char *)*a1"),
            "[{arch}] formatstring on must type the %s arg as `char *` on the \
             WHOLE-BINARY surface too (DIV-66), got:\n{c}"
        );
    }
}

/// Property 2 — the invariant this PR exists to establish: the two surfaces agree
/// byte for byte, in both option states.
#[test]
fn whole_binary_matches_console_in_both_option_states() {
    for arch in ARCHES {
        for on in [false, true] {
            let (Some(w), Some(c)) = (whole_binary_main(arch, on), console_main(arch, on))
            else {
                continue;
            };
            assert_eq!(
                w, c,
                "[{arch}] formatstring={on}: decompile-all and console `decompile` \
                 must render the same C (DIV-66)"
            );
        }
    }
}
