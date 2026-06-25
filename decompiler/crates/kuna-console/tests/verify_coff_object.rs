//! Multi-format-loader PR-5 e2e gate: a pre-link **COFF object** (`coff_obj.obj`,
//! a `.obj` from clang `-target x86_64-pc-windows-gnu`) loads under
//! `--experimental-formats` and a **named** defined function resolves *by name*
//! from the COFF symbol table and decompiles its real body — the object-file arm
//! of the loader seam (design §3.6, §4, §8 PR-5).
//!
//! ## COFF object vs COFF image (design §3.6)
//!
//! `object` reports two things as `BinaryFormat::Coff`: a pre-link **object**
//! (this fixture — `.obj`/`.o`) and, historically, a COFF-flavored image. A
//! *linked* PE reports as `BinaryFormat::Pe` and routes through the PE arm
//! (`verify_pe_imports`); this gate is specifically the object case. A COFF
//! object has **no IAT and no resolved imports** — externals are unresolved
//! *symbols*, not addresses — so `CoffFormat::resolve_imports` is empty. Its
//! value is the COFF symbol table (defined function names) + the format-agnostic
//! passes (strings/demangle/no-return-name/protos) riding a COFF object.
//!
//! ## What this proves (before → after PR-2/PR-5)
//!
//! `coff_obj.c`:
//!   `int compute(int x){ return x*3+1; }`
//!   `int run(int n){ const char *s="hi"; puts(s); return compute(n)+(int)s[0]; }`
//! built `clang -target x86_64-pc-windows-gnu -O1 -c coff_obj.c -o coff_obj.obj`
//! (Intel amd64 COFF — verified `objdump -t`: `compute` @ `.text`+0x0, `run` @
//! +0x10, `puts` an undefined external; the `"hi"` literal in `.rdata`).
//!
//! ```text
//!   before (PR-2 / PR-5 absent):  `.obj` → "not an ELF object" (rejected)
//!   after  (this PR):             loads + `compute`/`run` resolve by COFF-symtab
//!                                 name and decompile (`compute` → `a0*3+1`)
//! ```
//!
//! The defined-function-at-VMA-0 case is the COFF subtlety: a relocatable `.obj`
//! places `compute` at `.text`+0, which the old `addr == 0` funcsym skip (the ELF
//! UND convention) would have dropped — the loader now skips on `is_undefined()`
//! instead, registering `compute` while still dropping the undefined `puts`
//! placeholder (loadimage_object.rs source #1).
//!
//! ## Flag gating (default-off ⇒ byte-identical on ELF)
//!
//! The `.obj` only loads when `KUNA_EXPERIMENTAL_FORMATS` is set (the
//! `--experimental-formats` CLI flag). The test confirms that with it **unset**
//! the same fixture is rejected (the default-off, byte-identical-dispatch proof
//! shared with `verify_object_formats` / `verify_pe_imports`).
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). If it is absent the bootstrap fails
//! and the test prints that and returns early (a visible skip, never a false
//! green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// `KUNA_EXPERIMENTAL_FORMATS` is a process-global env var both tests in this file
/// toggle; serialize their env-sensitive bodies so the default-off check in one
/// never races the experimental-on bootstrap in the other (cargo runs the two
/// tests in parallel within one process).
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Bootstrap, run `load function <name>` → `decompile` → `print C`, and return the
/// captured C.
fn decompile_func(prog: ConsoleProgram, func_cmd: &str) -> String {
    let cmds: Vec<String> = [func_cmd, "decompile", "print C"]
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
    status.optr.clone()
}

/// Bootstrap the COFF object under `--experimental-formats`, returning `None` (a
/// visible skip) when the `.sla` is absent.
fn boot() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("coff_obj.obj");
    assert!(path.exists(), "missing fixture {path:?}");

    std::env::set_var("KUNA_EXPERIMENTAL_FORMATS", "1");
    match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_coff_object: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");
            None
        }
    }
}

/// The headline: a pre-link COFF `.obj` loads, its defined function `compute`
/// resolves *by name* from the COFF symbol table, and decompiles to `x*3+1`.
#[test]
fn coff_object_decompiles_named_function() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("coff_obj.obj");

    // (before / default-off proof) Without the flag the `.obj` must NOT load — it
    // routes to the XML branch, which cannot parse it (the pre-PR-2/PR-5 "not an
    // ELF object" rejection).
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");
    let off = kuna_console::engine::bootstrap_from_file(path.to_str().unwrap(), "", &spec_roots);
    assert!(
        off.is_err(),
        "default-off: a COFF object must NOT load without --experimental-formats"
    );

    let Some(prog) = boot() else { return };

    // Loads with the Windows x86-64 spec (COFF objects are MSVC-flavored).
    let desc = prog.description().to_string();
    assert!(desc.contains("x86"), "COFF: expected x86 spec, got {desc:?}");

    // The COFF symbol table's defined functions register as funcsyms — including
    // `compute`, which sits at `.text`+0 (the relocatable-object-at-VMA-0 case the
    // `is_undefined()` funcsym skip now keeps). `puts` is an *undefined* external
    // (no IAT in a pre-link object), so it does NOT register as a funcsym.
    assert!(
        prog.lookup_symbol("compute").is_some(),
        "COFF: `compute` (defined @ .text+0) not resolved by the loader"
    );
    assert!(
        prog.lookup_symbol("run").is_some(),
        "COFF: `run` not resolved by the loader"
    );
    assert!(
        prog.lookup_symbol("puts").is_none(),
        "COFF: `puts` is an unresolved external symbol — must NOT be a funcsym \
         (no IAT in a pre-link object, design §3.6)"
    );

    // `kuna decompile coff_obj.obj compute` (by name) → the real body `x*3+1`.
    let out = decompile_func(prog, "load function compute");
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");

    // The function is named `compute` (COFF-symtab name resolved), not `FUN_*`.
    assert!(
        out.contains("compute"),
        "COFF: decompiled function should be named `compute`, got:\n{out}"
    );
    // `x*3+1` — the LEA (rcx,rcx,2)/ADD 1 the x86-64 Windows ABI emits for
    // `return x*3+1` (first arg in rcx).
    assert!(
        out.contains("* 3 + 1"),
        "COFF: `compute` should decompile to `x*3+1`, got:\n{out}"
    );
    assert!(
        !out.contains("not an ELF object"),
        "COFF: loader rejected the object as not-ELF:\n{out}"
    );
}

/// The second defined function `run` also resolves by name and decompiles — proof
/// the COFF symtab funcsym source carries every defined function (the §4
/// symbol-source table), not just the first.
#[test]
fn coff_object_decompiles_second_named_function() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(prog) = boot() else { return };

    let out = decompile_func(prog, "load function run");
    std::env::remove_var("KUNA_EXPERIMENTAL_FORMATS");

    assert!(
        out.contains("run"),
        "COFF: decompiled function should be named `run`, got:\n{out}"
    );
    // `run` returns `compute(n) + (int)s[0]` with `puts(s)` an unresolved
    // external — `compute(n)=n*3+1` const-folds with `+'h'`(104) to `n*3+0x69`.
    assert!(
        out.contains("* 3 +"),
        "COFF: `run` should decompile its multiply-add body, got:\n{out}"
    );
}
