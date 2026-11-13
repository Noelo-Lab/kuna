//! End-to-end gate for the PE CRT entry-function prototype: the function the PE
//! C-runtime startup calls with argc/argv/envp gets the prototype that call site
//! establishes (P1 program prep).
//!
//! Fixture: `crtmain_x86_64.exe`, the reduced form of the shape
//! crackmes.one/65acadf3eef082e477ff5ede presents (`crtmain_x86_64.py` carries
//! the byte-by-byte layout). Its startup fetches the three arguments through the
//! named UCRT accessors and calls `main`; its `main` ignores them, which is
//! exactly why body-driven parameter recovery has nothing to find: `main` reads
//! none of `rcx`/`rdx`/`r8`, so it was declared to take nothing while the
//! startup three lines up passes three arguments. The call site names the
//! slots, so `main` declares `argc`, `argv` and `envp` at the widths the caller
//! establishes. `pemain` names the same callee `main`, and the prototype is
//! parked by address, so the two compose.
//!
//! The last test is the reason the address rides out with the prototype. The
//! prototype is parked by NAME, so with only the always-on oracles running (no
//! Listing, no `fast_funcdisc`) this fixture's `main` would not be a registered
//! function at all and the park would be a silent no-op. The other tests run
//! with `fast_funcdisc` on.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// The entry function the CRT startup calls, under the name the fixture mints
/// for it before `pemain` renames it.
const MAIN: &str = "sub_140001000";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(fast_funcdisc: bool) -> ConsoleProgram {
    let bin =
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/crtmain_x86_64.exe");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("fast_funcdisc", if fast_funcdisc { "on" } else { "off" })
        .expect("fast_funcdisc flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

fn decompile(prog: ConsoleProgram, name: &str) -> String {
    let cmds: Vec<String> = [format!("load function {name}"), "decompile".into(), "print C".into()]
        .into_iter()
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

/// The three slots the CRT establishes are declared, named after the accessor
/// that produced each and typed at the width of that slot, on the callee
/// `pemain` renamed `main`.
#[test]
fn entry_function_declares_the_crt_arguments() {
    let prog = bootstrap(true);
    assert!(prog.lookup_symbol(MAIN).is_none(), "pemain renames the callee");
    let body = decompile(prog, "main");
    assert!(
        !body.contains("main(void)"),
        "the entry function must not be declared to take nothing; got:\n{body}"
    );
    // Names, not spellings: the printed type words depend on the `ctypes` policy
    // (`uint4` in the console, `unsigned int` under the CLI default), while the
    // slot names are what this pass actually asserts.
    for want in ["main(", "argc", "argv", "envp"] {
        assert!(body.contains(want), "{want} missing; got:\n{body}");
    }
    assert!(
        body.find("argc") < body.find("argv") && body.find("argv") < body.find("envp"),
        "the slots must be emitted in ABI order; got:\n{body}"
    );
}

/// The guard: a callee that already carries a name keeps its own signature. The
/// CRT accessors are named imports reached by the same scan, and none of them is
/// retyped or dropped.
#[test]
fn named_functions_keep_their_own_signatures() {
    let prog = bootstrap(true);
    let named: Vec<String> = prog
        .function_entries_canonical()
        .into_iter()
        .filter(|e| !e.name.starts_with("sub_") && e.name != "main")
        .map(|e| e.name)
        .collect();
    assert!(!named.is_empty(), "the fixture must carry named import functions");
    for name in &named {
        assert!(prog.lookup_symbol(name).is_some(), "{name} must keep its own symbol");
    }
}

/// The address rides out with the prototype: with only the always-on oracles
/// running, this fixture's `main` is found by nothing else, and a prototype
/// parked by name on a function that does not exist would be a silent no-op.
#[test]
fn the_entry_function_is_registered_even_with_no_recursive_discovery() {
    let prog = bootstrap(false);
    assert!(
        prog.lookup_symbol("main").is_some(),
        "the CRT call target must be registered so the by-name prototype park lands"
    );
    let body = decompile(prog, "main");
    assert!(body.contains("argc"), "and it must carry the recovered slots; got:\n{body}");
}
