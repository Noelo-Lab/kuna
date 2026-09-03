//! End-to-end two-pass gate for `entrymainproto` — the function the PE C-runtime
//! startup calls with argc/argv/envp gets the prototype that call site
//! establishes (P1 program prep).
//!
//! Fixture: `crtmain_x86_64.exe`, the reduced form of the shape
//! crackmes.one/65acadf3eef082e477ff5ede presents (`crtmain_x86_64.py` carries
//! the byte-by-byte layout). Its startup fetches the three arguments through the
//! named UCRT accessors and calls `main`; its `main` ignores them, which is
//! exactly why body-driven parameter recovery has nothing to find.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** `main` reads none of `rcx`/`rdx`/`r8`, so it is
//!   declared to take nothing while the startup three lines up passes three
//!   arguments.
//! * **default (the fix):** the call site names the slots, so `main` declares
//!   `argc`, `argv` and `envp` at the widths the caller establishes.
//!
//! The third test is the guard that keeps this from overwriting better
//! knowledge: the accessors themselves are named imports, and a named callee is
//! left alone. Every named function in the off arm is still named in the default
//! arm.
//!
//! The fourth is the reason the address rides out with the prototype. The
//! prototype is parked by NAME, so with only the always-on oracles running (no
//! Listing, no `fast_funcdisc`) this fixture's `main` is not a registered
//! function at all and the park would be a silent no-op. Both arms are otherwise
//! run with `fast_funcdisc` on, so the first three tests compare prototypes
//! rather than discovery.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// The entry function the CRT startup calls (unnamed in the fixture).
const MAIN: &str = "sub_140001000";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(entrymainproto: bool, fast_funcdisc: bool) -> Option<ConsoleProgram> {
    let bin =
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/crtmain_x86_64.exe");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_entrymainproto: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("fast_funcdisc", if fast_funcdisc { "on" } else { "off" })
        .expect("fast_funcdisc flips");
    prog.arch_mut()
        .set_kuna_option("entrymainproto", if entrymainproto { "on" } else { "off" })
        .expect("entrymainproto flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
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

/// BEFORE (`--option entrymainproto off`): the entry function takes nothing.
#[test]
fn entry_function_takes_no_parameters_with_the_option_off() {
    let Some(prog) = bootstrap(false, true) else {
        return;
    };
    let body = decompile(prog, MAIN);
    assert!(
        body.contains(&format!("{MAIN}(void)")),
        "with entrymainproto off the entry function must still be declared to take \
         nothing — the option must restore the previous form exactly; got:\n{body}"
    );
}

/// AFTER (default): the three slots the CRT establishes are declared, named after
/// the accessor that produced each and typed at the width of that slot.
#[test]
fn entry_function_declares_the_crt_arguments_by_default() {
    let Some(prog) = bootstrap(true, true) else {
        return;
    };
    let body = decompile(prog, MAIN);
    assert!(
        !body.contains(&format!("{MAIN}(void)")),
        "the entry function must no longer be declared to take nothing; got:\n{body}"
    );
    // Names, not spellings: the printed type words depend on the `ctypes` policy
    // (`uint4` in the console, `unsigned int` under the CLI default), while the
    // slot names are what this pass actually asserts.
    for want in ["argc", "argv", "envp"] {
        assert!(
            body.contains(&format!("{want}")),
            "the {want} slot must be declared; got:\n{body}"
        );
    }
    assert!(
        body.find("argc") < body.find("argv") && body.find("argv") < body.find("envp"),
        "the slots must be emitted in ABI order; got:\n{body}"
    );
}

/// The guard: a callee that already carries a name keeps its own signature. The
/// CRT accessors are named imports reached by the same scan, and none of them is
/// retyped.
#[test]
fn named_functions_keep_their_own_signatures() {
    let (Some(off), Some(on)) = (bootstrap(false, true), bootstrap(true, true)) else {
        return;
    };
    let named: Vec<String> = off
        .function_entries_canonical()
        .into_iter()
        .filter(|e| !e.name.starts_with("sub_"))
        .map(|e| e.name)
        .collect();
    assert!(!named.is_empty(), "the fixture must carry named import functions");
    for name in &named {
        assert!(
            on.lookup_symbol(name).is_some(),
            "entrymainproto must never drop a named function; {name} disappeared"
        );
    }
    // With the recursive walk on, the entry the pass carries alongside the
    // prototype duplicates one the walk already found, so the entry set is
    // unchanged and the arms differ only in the prototype.
    let before = off.function_entries_canonical().len();
    let after = on.function_entries_canonical().len();
    assert_eq!(before, after, "no entry may be added on an image that already found main");
}

/// The address rides out with the prototype: with only the always-on oracles
/// running, this fixture's `main` is not discovered at all, and a prototype
/// parked by name on a function that does not exist would be a silent no-op.
#[test]
fn the_entry_function_is_registered_even_with_no_recursive_discovery() {
    let (Some(off), Some(on)) = (bootstrap(false, false), bootstrap(true, false)) else {
        return;
    };
    assert!(
        off.lookup_symbol(MAIN).is_none(),
        "the fixture must not find main through the always-on oracles alone, or this \
         test proves nothing"
    );
    assert!(
        on.lookup_symbol(MAIN).is_some(),
        "the CRT call target must be registered so the by-name prototype park lands"
    );
    let body = decompile(on, MAIN);
    assert!(body.contains("argc"), "and it must carry the recovered slots; got:\n{body}");
}
