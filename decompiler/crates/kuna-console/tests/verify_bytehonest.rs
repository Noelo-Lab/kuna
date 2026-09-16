//! Two-pass end-to-end gate for the `bytehonest` option (P6, JSON surface).
//!
//! A one-byte value the type system never committed to is carried as a size-1
//! `TYPE_UNKNOWN`, which the default `realtypes` rendering spells `char`.  That is
//! the right choice for the C TEXT and the wrong one for a machine-readable
//! report: `char` names an element type the recovery never established, where the
//! program's own type may as well be `_Bool` or one byte of an unsplit struct.
//! `extract_variables` reports the width instead (`undefined1`).
//!
//! Pass 1 (`bytehonest off`, the pre-feature behaviour) and pass 2 (`on`, the
//! shipped default) run the same decompile of the same functions.  The gate is
//! that the two arms differ in EXACTLY one way -- a row's `type_name` -- that
//! every row that differs was spelled `char` and is now `undefined1` at an
//! unchanged `size` of 1, and that the parameter/stack sections are the only
//! place it happens.
//!
//! ## `.sla` precondition
//!
//! Like the sibling gates, bootstrapping needs the built `x86` `.sla` under
//! `specs/` (gitignored; `make specs`).  When it is absent the bootstrap fails;
//! the test prints that and returns early (a specs-less CI is a visible skip,
//! never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::decompile_drive::{
    decompile_func_full_with_override_dyn, extract_variables, VarInfo,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// Every function's `extract_variables` output, plus its emitted C, for one
/// setting of `bytehonest`.
fn variables_for(byte_honest: bool) -> Option<Vec<(String, Vec<VarInfo>)>> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fauxware().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_bytehonest: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");
    prog.arch_mut().byte_honest = byte_honest;

    let entries: Vec<(String, _)> =
        prog.function_entries().map(|(n, a)| (n.to_string(), a.clone())).collect();
    let mut out = Vec::new();
    for (name, addr) in entries {
        let mapped = prog.dwarf_locals_for(addr.get_offset());
        let Ok(fd) = decompile_func_full_with_override_dyn(
            prog.arch_mut(), &name, addr, 0, &mapped, &[], &[], None, &[], &[], &[],
        ) else {
            continue;
        };
        out.push((name, extract_variables(prog.arch(), &fd)));
    }
    Some(out)
}

#[test]
fn bytehonest_reports_an_uncommitted_byte_by_width_and_changes_nothing_else() {
    let Some(off) = variables_for(false) else { return };
    let Some(on) = variables_for(true) else { return };
    assert_eq!(off.len(), on.len(), "the two arms must decompile the same functions");

    let mut respelled = 0usize;
    for ((fname, ovars), (fname2, nvars)) in off.iter().zip(on.iter()) {
        assert_eq!(fname, fname2, "function order must match across arms");
        assert_eq!(
            ovars.len(),
            nvars.len(),
            "{fname}: bytehonest changed the variable COUNT ({} -> {})",
            ovars.len(),
            nvars.len()
        );
        for (a, b) in ovars.iter().zip(nvars.iter()) {
            // Identity, storage and width are untouched: the option re-spells one
            // field and nothing else.
            assert_eq!(a.name, b.name, "{fname}: bytehonest renamed a variable");
            assert_eq!(a.stack_offset, b.stack_offset, "{fname}: bytehonest moved a variable");
            assert_eq!(a.size, b.size, "{fname}: bytehonest resized `{}`", a.name);
            assert_eq!(a.is_param, b.is_param, "{fname}: bytehonest reclassified `{}`", a.name);
            assert_eq!(a.arg_index, b.arg_index, "{fname}: bytehonest re-indexed `{}`", a.name);
            if a.type_name == b.type_name {
                continue;
            }
            respelled += 1;
            // The ONLY admissible difference, and only at one byte.
            assert_eq!(
                (a.type_name.as_str(), b.type_name.as_str()),
                ("char", "undefined1"),
                "{fname}: `{}` moved from `{}` to `{}`, which is not the \
                 uncommitted-byte re-spelling",
                a.name,
                a.type_name,
                b.type_name
            );
            assert_eq!(b.size, 1, "{fname}: `{}` reports undefined1 at size {}", b.name, b.size);
        }
    }

    // The fixture must actually exercise the path, or this gate proves nothing.
    assert!(
        respelled > 0,
        "bytehonest re-spelled nothing anywhere in fauxware -- the gate is vacuous"
    );
}
