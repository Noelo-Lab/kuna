//! End-to-end gate for the measured libc signature extension.
//!
//! The gap: the built-in prototype table carried 27 signatures, so every libc
//! callee outside them left its caller's arguments as an inferred `unsigned long`.
//! `fauxware`'s `authenticate` takes a path and hands it straight to `open`, and
//! with only the base table that parameter comes out `unsigned long a0`.
//!
//! ## Why a Rust integration test and not a `tests/stages/*.xml` case
//!
//! The XML datatest bootstrap builds its program from a `<binaryimage>` element
//! and never runs the analyzer tier — it produces no `AnalysisOutput` at all, so
//! no `kuna_analysis` pass (this one included) can fire there. Same reason
//! `verify_cppsig.rs` / `verify_cppproto.rs` / `verify_fdeinterior.rs` exist.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap `fauxware`, commit the analysis facts, decompile `func` and return
/// the captured C.
fn decompile(func: &str) -> String {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str().expect("UTF-8 fixture path")
        .to_string();
    let mut prog = bootstrap_from_object(&path, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
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
    status.optr.clone()
}

/// The headline: a caller's parameter that is only ever handed to a libc function
/// the base table does not know. `authenticate(char *username, char *password)`
/// passes its first argument to `open(2)`; without the table nothing says the slot
/// is a path, so it rendered `unsigned long`.
#[test]
fn caller_parameter_typed_from_the_callee_it_is_passed_to() {
    let on = decompile("authenticate");
    assert!(
        on.contains("authenticate(char *a0,char *a1)"),
        "the default must type the `open` path argument `char *`, got:\n{on}"
    );
}

/// The declaration of the local assigned `open(...)`'s result, found by what it
/// holds rather than by its `vN` number, which moves whenever an earlier local
/// is folded away.
fn fd_declaration(code: &str) -> Option<&str> {
    let name = code
        .lines()
        .find_map(|l| l.trim().split_once(" = open(").map(|(lhs, _)| lhs.trim()))?;
    let suffix = format!(" {name};");
    code.lines().map(str::trim).find(|l| l.split("//").next().unwrap().trim_end().ends_with(&suffix))
}

/// The same call's RETURN type travels too: `open` yields an `int` file
/// descriptor, so the local holding it stops being an `unsigned int` of unknown
/// provenance.
#[test]
fn callee_return_type_reaches_the_local_that_holds_it() {
    let on = decompile("authenticate");
    assert!(
        fd_declaration(&on).is_some_and(|d| d.starts_with("int4 ")),
        "the default must carry `open`'s int return onto the fd, got:\n{on}"
    );
}

/// The import's own thunk gains the signature, including the variadic tail:
/// `int open(const char *, int, ...)` — the third argument only exists for
/// `O_CREAT`, so the slot must stay variadic rather than being fixed at two.
#[test]
fn imported_thunk_carries_the_full_signature_with_its_variadic_tail() {
    let on = decompile("open");
    assert!(
        on.contains("int4 open(char *a0,int4 a1,...)"),
        "the default must give the thunk `int open(char *, int, ...)`, got:\n{on}"
    );
}

/// A zero-argument-after-the-first entry, and the no-return interaction:
/// `void exit(int)`. The name is already on the known-no-return list, so this
/// pins that adding a prototype does not disturb that.
#[test]
fn exit_takes_its_int_status_and_stays_no_return() {
    let on = decompile("exit");
    assert!(
        on.contains("void exit(int4 a0)"),
        "the default must give `exit` its int status, got:\n{on}"
    );

}

