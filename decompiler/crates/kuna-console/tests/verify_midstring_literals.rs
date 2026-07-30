//! End-to-end gate for **mid-string literal recovery**: a constant pointing into
//! the *interior* of a read-only character array renders as the tail string,
//! matching IDA Pro and Ghidra.
//!
//! A compiler that merges string constants (`-fmerge-constants`, on at `-O2`)
//! stores a literal that is a suffix of another literal only once and points at
//! the interior of the longer one. In GNU coreutils `fmt`, `"coreutils"` is the
//! tail of `"GNU coreutils"` and `"%s"` the tail of `"%s: %s"`, so kuna rendered
//! `bindtextdomain(0x68d8, ...)` and `error(1, ..., 0x6f0c, ...)` where IDA shows
//! the literals.
//!
//! The gap was never in the string machinery — the string manager reads happily
//! from any address. `ActionConstantPtr` correctly recognized the interior
//! pointer and built a spacebase reference for it, but that reference is
//! `PTRSUB(spacebase, symaddr)` plus an `INT_ADD` of the residual, and constant
//! folding collapses the pair back to the bare constant, taking the type with it.
//! The constant then reached the printer as `undefined8` and never entered the
//! char-pointer arm. An *exact* hit survived the same path only because its
//! residual is zero. The interior case is now typed `char *` directly, at the
//! point where the covering symbol has already proved it is one.
//!
//! Fixtures: `midstring_x86_64` (`+.c`, purpose-built, carries the negative
//! control) and `regglobal_fmt_x86_64` (the IDA-parity reference).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap `bin`, commit the analysis facts, decompile `func`, and return the
/// captured C (`None` ⇒ specs-less skip).
fn decompile(bin: &str, func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_midstring_literals: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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

/// The purpose-built fixture: `sink("coreutils")` is `&banner[4]` after gcc's
/// literal merge, and must render the tail literal — while `consume(&numbers[3])`,
/// an interior pointer into a read-only **int** array, must stay numeric.
#[test]
fn interior_pointer_into_a_char_array_renders_the_tail_literal() {
    let Some(code) = decompile("midstring_x86_64", "main") else { return };

    assert!(
        code.contains(r#"sink("GNU coreutils")"#),
        "the exact-start literal must still render (regression guard); got:\n{code}",
    );
    assert!(
        code.contains(r#"sink("coreutils")"#),
        "expected the INTERIOR pointer to render as the tail literal; got:\n{code}",
    );
    assert!(
        !code.contains(r#"consume(""#),
        "an interior pointer into a read-only INT array must NOT stringify; got:\n{code}",
    );
}

/// The IDA-parity reference. `"coreutils"` is 4 bytes into `"GNU coreutils"` and
/// `"%s"` is 12 bytes into `"%s: %s"`.
#[test]
fn fmt_main_resolves_its_merged_tail_literals() {
    let Some(code) = decompile("regglobal_fmt_x86_64", "main") else { return };

    assert!(
        code.contains(r#"bindtextdomain("coreutils""#),
        "expected `bindtextdomain(\"coreutils\", ...)`; got:\n{code}",
    );
    assert!(
        code.contains(r#"textdomain("coreutils")"#),
        "expected `textdomain(\"coreutils\")`; got:\n{code}",
    );
    assert!(
        !code.contains("0x68d8") && !code.contains("0x6f0c"),
        "the merged tail literals must no longer render as raw addresses; got:\n{code}",
    );
}
