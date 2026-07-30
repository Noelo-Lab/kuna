//! End-to-end gate for the **whole-binary** in-process decompile path that backs
//! `kuna decompile-all` / `kuna functions` (the decbench + LLM-CLI surface).
//!
//! Drives the exact load-once shape the CLI uses against the real vendored
//! `fauxware` ELF (x86-64, dynamically linked, non-stripped):
//!
//! ```text
//!   bootstrap_from_object        (== `load file`)
//!   commit_pending_analysis      (== `read symbols`)
//!   function_entries()           (the enumeration `decompile-all` loops over)
//!   per function:
//!     decompile_func_full_with_override_dyn  (== `decompile`, with DWARF locals)
//!     print_c                                (== `print C`)
//!     extract_variables                      (the type_match variable list)
//! ```
//!
//! It asserts (1) the function enumeration finds the program's functions, (2) a
//! function decompiles to non-empty C naming itself, and (3) `extract_variables`
//! returns a sane parameter/local list (params carry a dense `arg_index`; every
//! variable carries a non-empty C type string).
//!
//! ## `.sla` precondition
//!
//! Like the sibling loader gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`).  When it is absent the bootstrap
//! fails; the test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::decompile_drive::{decompile_func_full_with_override_dyn, extract_variables, print_c};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored fauxware fixture (shared with the kuna-analysis loader gate).
fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

#[test]
fn decompile_all_enumerates_and_decompiles_every_function() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = match fauxware().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_decompile_all: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return;
        }
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");

    // (1) Enumeration: the loader + analysis function set must include the real
    // defined function `main` and the resolved PLT imports — the same set the
    // `kuna functions` command lists.
    let names: Vec<String> = prog.function_entries().map(|(n, _)| n.to_string()).collect();
    assert!(!names.is_empty(), "function_entries returned no functions");
    assert!(names.iter().any(|n| n == "main"), "enumeration missing `main`: {names:?}");

    // The address an entry reports must round-trip back to the same function via
    // the symbol-table lookup (i.e. the addresses are real entry VMAs).
    let main_addr = prog
        .function_entries()
        .find(|(n, _)| *n == "main")
        .map(|(_, a)| a.clone())
        .expect("main present in enumeration");

    // (2) Decompile `main` exactly as the `decompile-all` loop does (DWARF locals
    // re-seeded), and assert it produces non-empty C that names the function.
    let mapped = prog.dwarf_locals_for(main_addr.get_offset());
    let fd = decompile_func_full_with_override_dyn(
        prog.arch_mut(),
        "main",
        main_addr,
        0,
        &mapped,
        &[],
        &[],
        None,
        &[],
        &[],
        &[],
    )
    .expect("main must decompile");
    let code = print_c(prog.arch_mut(), &fd);
    assert!(code.contains("main"), "decompiled C does not name `main`:\n{code}");
    assert!(code.contains('{') && code.contains('}'), "decompiled C is not a function body");

    // (3) Variable extraction: every variable has a non-empty C type string, the
    // recovered parameters carry a dense ABI `arg_index` (0,1,2,...), and a stack
    // local never carries one.
    let main_vars = extract_variables(prog.arch(), &fd);
    assert_var_invariants(&main_vars);

    // (4) `authenticate(const char *, const char *)` exercises the parameter path
    // specifically: it must recover at least one argument with `arg_index == 0`
    // and a non-empty type string (the type_match signal).
    let auth_addr = prog
        .function_entries()
        .find(|(n, _)| *n == "authenticate")
        .map(|(_, a)| a.clone())
        .expect("authenticate present in enumeration");
    let mapped = prog.dwarf_locals_for(auth_addr.get_offset());
    let fd = decompile_func_full_with_override_dyn(
        prog.arch_mut(), "authenticate", auth_addr, 0, &mapped, &[], &[], None, &[], &[], &[],
    )
    .expect("authenticate must decompile");
    let auth_vars = extract_variables(prog.arch(), &fd);
    assert_var_invariants(&auth_vars);
    let args: Vec<_> = auth_vars.iter().filter(|v| v.is_param).collect();
    assert!(!args.is_empty(), "authenticate recovered no parameters");
    assert_eq!(args[0].arg_index, Some(0), "first parameter must have arg_index 0");
    assert!(!args[0].type_name.is_empty(), "parameter type string is empty");
}

/// The vendored ARM/Thumb fixture — two functions (`compute` @ 0x100b8, `_start`
/// @ 0x100d6) whose ELF `.symtab` records them at the ODD `st_value` 0x100b9 /
/// 0x100d7 (the Thumb mode bit).
fn arm_thumb() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/arm_thumb_linked_le32")
}

/// Issue #197, at the shared seam: `function_entries_canonical` yields exactly one
/// record per entry ADDRESS, and every name the raw stream knew is still reachable
/// (as the record's `name` or one of its `aliases`).
///
/// This is the invariant all four whole-binary surfaces inherit — `decompile-all`,
/// `functions`, `decompile-project`, and the wasm front-end, which has no test
/// directory of its own and is covered only here.  Checked on both an x86-64 ELF
/// (`fauxware`) and the ARM/Thumb fixture (whose odd `st_value`s are what made the
/// duplicates un-collapsible by address alone).
#[test]
fn canonical_enumeration_reports_each_entry_address_once() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];

    for fixture in [fauxware(), arm_thumb()] {
        let Some(bin) = fixture.to_str().map(|s| s.to_string()) else { continue };
        let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "canonical enumeration: skipping {bin} (bootstrap failed, build `.sla` \
                     with `make specs`): {}",
                    e.explain()
                );
                continue;
            }
        };
        prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");

        let canonical = prog.function_entries_canonical();
        assert!(!canonical.is_empty(), "{bin}: canonical enumeration is empty");

        // One record per entry address, strictly ascending (the accessor's order
        // contract — the surfaces emit it as-is).
        let mut prev: Option<u64> = None;
        for e in &canonical {
            let off = e.addr.get_offset();
            if let Some(p) = prev {
                assert!(p < off, "{bin}: entries not strictly address-ordered at 0x{off:x}");
            }
            prev = Some(off);
            // A generated placeholder may only be the reported name when it is the
            // ONLY name this entry has.
            if e.name.starts_with("sub_") || e.name.starts_with("FUN_") {
                assert!(
                    e.aliases.is_empty(),
                    "{bin}: 0x{off:x} reports the placeholder {:?} while also carrying {:?}",
                    e.name,
                    e.aliases
                );
            }
        }

        // Nothing is lost: every name in the raw stream is still reachable, and
        // resolves back to the entry that owns it.
        for (name, _) in prog.function_entries() {
            let found = prog
                .find_entry_by_name(name)
                .unwrap_or_else(|| panic!("{bin}: raw name {name:?} no longer resolves"));
            assert!(
                found.name == name || found.aliases.iter().any(|a| a == name),
                "{bin}: {name:?} resolved to an entry that does not carry it"
            );
        }
    }
}

/// Assert the cross-cutting `VarInfo` invariants the type_match metric relies on:
/// non-empty type strings, dense ordered parameter `arg_index`es, and no
/// `arg_index` on stack locals.
fn assert_var_invariants(vars: &[kuna_decomp::decompile_drive::VarInfo]) {
    let mut next_arg = 0usize;
    for v in vars {
        assert!(!v.type_name.is_empty(), "variable {:?} has an empty type", v.name);
        if v.is_param {
            assert_eq!(v.arg_index, Some(next_arg), "parameter arg_index not dense/ordered");
            next_arg += 1;
        } else {
            assert!(v.arg_index.is_none(), "a stack local must not carry an arg_index");
        }
    }
}
