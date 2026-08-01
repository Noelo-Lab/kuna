//! End-to-end gate: the decompiler must declare every local it references **exactly
//! once** — neither an undeclared local nor a re-declared one is C.
//!
//! Fixture: `declmerge_x86_64` (`+.c`), a non-PIE x86-64 ELF whose `make_dir_clone`
//! (a reduction of tar's `make_directory`) has a size phi `sz = (…) ? len : len + 1`
//! that kuna's merge phase declines to coalesce with the `strlen` result. The
//! un-coalesced phi output picks up an instance overlapping a **parameter register**,
//! so `PrintC::emit_local_var_decls`' `is_param` storage-containment test used to
//! SKIP it — while the statements still referenced it, producing an undeclared `v5`
//! (invalid C). The fix gates the `is_param` skip on the high actually being named as
//! one of the prototype's parameters, so a local that merely overlaps a param is
//! still declared.
//!
//! This gate re-parses the emitted C and asserts every referenced `vN` local appears
//! in the declaration block. Regression for the ghidra-beats-kuna merge/copy cluster's
//! invalid-C sub-bug (`docs/decbench/ghidra-gap-analysis.md` §3).
//!
//! It also asserts the mirror image: no name is declared **twice**. The printer walks
//! HighVariables where upstream walks ScopeLocal Symbols, so both directions are
//! reachable from the same walk — an over-strict skip drops the only declaration of a
//! referenced name, and an unmerged second live range of one mapped slot adds a second
//! declaration of a name that already has one (DIV-52, whose end-to-end witness is
//! `tests/stages/ghdec-symbol-keyed-local-decls.xml`). Pinning both here keeps a fix
//! for either from re-opening the other.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make specs`).
//! When absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/declmerge_x86_64")
}

/// Bootstrap the fixture and decompile `func`, returning the printed C
/// (`None` ⇒ specs-less skip).
fn decompile(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("verify_declared_vars: skipping (bootstrap failed, `make specs`): {}", e.explain());
            return None;
        }
    };

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

/// Every `vN` token that appears in the declaration block (the lines between the
/// function's opening `{` and the first blank line — the C++ printer's local decl
/// prelude), **in order and with repeats** so a re-declaration is visible.
fn declared_var_list(c: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_decls = false;
    for line in c.lines() {
        let t = line.trim();
        if t == "{" {
            in_decls = true;
            continue;
        }
        if in_decls {
            if t.is_empty() {
                break; // first blank line ends the declaration prelude
            }
            // A declaration line ends in `;` (possibly with a trailing `// comment`).
            for tok in tokens_vn(line) {
                out.push(tok);
            }
        }
    }
    out
}

/// All `vN` identifier tokens in a string (N = one or more digits).
fn tokens_vn(s: &str) -> Vec<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        // start of an identifier not preceded by an identifier char
        let prev_ident = i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        if !prev_ident && bytes[i] == b'v' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            // must not continue into a longer identifier (v1a)
            if i >= bytes.len() || !(bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                out.push(s[start..i].to_string());
            }
        } else {
            i += 1;
        }
    }
    out
}

#[test]
fn no_undeclared_local_variable_in_make_dir_clone() {
    let Some(c) = decompile("make_dir_clone") else {
        return; // specs-less skip
    };
    eprintln!("---- make_dir_clone ----\n{c}");

    let decl_list = declared_var_list(&c);
    // No name is declared twice: one ScopeLocal Symbol is one declaration, however
    // many HighVariables of it the merge left behind (DIV-52).
    let mut redeclared: Vec<&String> = Vec::new();
    for (i, name) in decl_list.iter().enumerate() {
        if decl_list[..i].contains(name) && !redeclared.contains(&name) {
            redeclared.push(name);
        }
    }
    assert!(
        redeclared.is_empty(),
        "emitted a RE-DECLARED local variable (invalid C): {:?}\n\
         declaration block={:?}\n--- C ---\n{}",
        redeclared, decl_list, c
    );

    let declared: std::collections::BTreeSet<String> = decl_list.into_iter().collect();
    // Every `vN` referenced anywhere in the body must be declared.
    let used: std::collections::BTreeSet<String> = tokens_vn(&c).into_iter().collect();
    let undeclared: Vec<&String> = used.iter().filter(|v| !declared.contains(*v)).collect();

    assert!(
        undeclared.is_empty(),
        "emitted an UNDECLARED local variable (invalid C): {:?}\n\
         declared={:?}\n--- C ---\n{}",
        undeclared, declared, c
    );
    // Sanity: the reduction really does still materialise several merged `vN`
    // locals, i.e. the fixture is exercising the merge/decl path it was built for.
    //
    // The threshold was 5 until the P6 Cover-dirty fix (`Merge::markImplied`
    // dirties its operands' Covers, and a Varnode `coverdirty` now reaches the
    // owning HighVariable as in `Varnode::setFlags`).  With Covers recomputed at
    // the right points the size phi finally coalesces with the `strlen` result the
    // way upstream Ghidra does, so the un-coalesced `v5` is gone and the reduction
    // renders 4 locals.  The undeclared-variable gate above is unchanged.
    assert!(
        used.len() >= 4,
        "expected the make_dir_clone reduction to still materialise several `vN` locals, got {:?}",
        used
    );
}
