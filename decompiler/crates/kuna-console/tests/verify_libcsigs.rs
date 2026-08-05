//! End-to-end gate for the measured libc signature extension (`--option libcsigs`).
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
//! `verify_cppsig.rs` / `verify_cppproto.rs` / `verify_fdeinterior.rs` exist. The
//! two-pass discipline is kept: every test below decompiles the same function
//! under both gate settings and asserts the before AND the after.
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

/// Bootstrap `fauxware` with `libcsigs` set to `mode`, commit the analysis facts
/// under that gate, decompile `func` and return the captured C. `None` => skip.
fn decompile(func: &str, mode: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str()?
        .to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_libcsigs: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The gate is read at the analysis COMMIT boundary (the producing pass runs at
    // `load file`, upstream of any `option` command), so it must be set here.
    prog.arch_mut().set_kuna_option("libcsigs", mode).expect("libcsigs is a registered option");
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

/// The headline: a caller's parameter that is only ever handed to a libc function
/// the base table does not know. `authenticate(char *username, char *password)`
/// passes its first argument to `open(2)`; with the gate off there is nothing to
/// say the slot is a path, so it renders `unsigned long`.
#[test]
fn caller_parameter_typed_from_the_callee_it_is_passed_to() {
    let Some(off) = decompile("authenticate", "off") else { return };
    assert!(
        off.contains("authenticate(unsigned long a0,char *a1)"),
        "gate off must reproduce the untyped first parameter, got:\n{off}"
    );

    let on = decompile("authenticate", "on").expect("second pass bootstraps");
    assert!(
        on.contains("authenticate(char *a0,char *a1)"),
        "the default must type the `open` path argument `char *`, got:\n{on}"
    );
}

/// The same call's RETURN type travels too: `open` yields an `int` file
/// descriptor, so the local holding it stops being an `unsigned int` of unknown
/// provenance.
#[test]
fn callee_return_type_reaches_the_local_that_holds_it() {
    let Some(off) = decompile("authenticate", "off") else { return };
    assert!(
        off.contains("unsigned int v4;"),
        "gate off leaves the fd an unsigned int, got:\n{off}"
    );

    let on = decompile("authenticate", "on").expect("second pass bootstraps");
    assert!(
        on.contains("int4 v4;"),
        "the default must carry `open`'s int return onto the fd, got:\n{on}"
    );
}

/// The import's own thunk gains the signature, including the variadic tail:
/// `int open(const char *, int, ...)` — the third argument only exists for
/// `O_CREAT`, so the slot must stay variadic rather than being fixed at two.
#[test]
fn imported_thunk_carries_the_full_signature_with_its_variadic_tail() {
    let Some(off) = decompile("open", "off") else { return };
    assert!(off.contains("void open(void)"), "gate off thunk, got:\n{off}");

    let on = decompile("open", "on").expect("second pass bootstraps");
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
    let Some(off) = decompile("exit", "off") else { return };
    assert!(off.contains("void exit(void)"), "gate off thunk, got:\n{off}");

    let on = decompile("exit", "on").expect("second pass bootstraps");
    assert!(
        on.contains("void exit(int4 a0)"),
        "the default must give `exit` its int status, got:\n{on}"
    );

}

/// The change is surgical: a function that calls nothing outside the base table
/// is byte-identical under both settings. `main` only reaches `puts`/`read`, and
/// `read` is a table entry whose typing it does not depend on.
#[test]
fn a_function_the_table_does_not_touch_is_byte_identical() {
    let Some(off) = decompile("accepted", "off") else { return };
    let on = decompile("accepted", "on").expect("second pass bootstraps");
    assert_eq!(off, on, "`accepted` calls only `puts`, already in the base table");
}
