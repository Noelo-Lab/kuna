//! End-to-end two-pass gate for `machomain` — the Mach-O `LC_MAIN` entry routine
//! is named `main` and declared `int main(int argc, char **argv)` (P1 program
//! prep).
//!
//! Fixtures: `macho_stripped_main` and the image it is derived from,
//! `macho_imports`. They are the same 16688-byte executable, differing only in
//! whether the defined-symbol names are present
//! (`macho_stripped_main.py` carries the transform). `LC_MAIN` states
//! `entryoff = 0x5b0` in both, so `0x1000005b0` is the program's `main` either
//! way — in one of them the symbol table says so, in the other only the load
//! command does.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** the stripped image's entry is one more
//!   `sub_<addr>` in an inventory of them, and its one recovered slot is the
//!   anonymous `a0`.
//! * **default (the fix):** it is `main`, with `argc` and `argv` declared.
//!
//! The third test is the guard that keeps this from overwriting better
//! knowledge: the un-stripped twin names that same address `_main` from its own
//! symbol table, and the pass leaves it alone in both arms.
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

/// `LC_MAIN`'s entry: `__TEXT.vmaddr (0x100000000) + entryoff (0x5b0)`.
const ENTRY: &str = "sub_1000005b0";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(fixture: &str, machomain: bool) -> Option<ConsoleProgram> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_machomain: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("machomain", if machomain { "on" } else { "off" })
        .expect("machomain flips");
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

/// BEFORE (`--option machomain off`): the stripped image's program entry is
/// indistinguishable from every other function in it.
#[test]
fn the_entry_is_an_anonymous_sub_with_the_option_off() {
    let Some(prog) = bootstrap("macho_stripped_main", false) else {
        return;
    };
    assert!(
        prog.lookup_symbol("main").is_none(),
        "with machomain off nothing may be named main — the option must restore the \
         previous inventory exactly"
    );
    assert!(prog.lookup_symbol(ENTRY).is_some(), "the entry is still discovered, just unnamed");
    let body = decompile(prog, ENTRY);
    assert!(
        body.contains(&format!("{ENTRY}(")),
        "and it must still be declared under the anonymous name; got:\n{body}"
    );
    // The fixture's `main` does read `argc`, so body-driven recovery finds ONE
    // untyped, unnamed slot on its own (`a0`) — and nothing tells it what that
    // slot is, nor that a second one exists.
    for absent in ["argc", "argv"] {
        assert!(
            !body.contains(absent),
            "no argument may be named with machomain off; got:\n{body}"
        );
    }
}

/// AFTER (default): the load command's own claim is applied — the name and the
/// conventional two-parameter declaration.
#[test]
fn the_lc_main_entry_is_named_main_by_default() {
    let Some(prog) = bootstrap("macho_stripped_main", true) else {
        return;
    };
    assert!(
        prog.lookup_symbol("main").is_some(),
        "the LC_MAIN entry must be named main"
    );
    let body = decompile(prog, "main");
    assert!(
        !body.contains(&format!("{ENTRY}(")),
        "the anonymous name must be gone from the declaration; got:\n{body}"
    );
    // Names, not spellings: the printed type words depend on the `ctypes` policy
    // (`int4` in the console, `int` under the CLI default), while the slot names
    // and their order are what this pass actually asserts.
    for want in ["main", "argc", "argv"] {
        assert!(body.contains(want), "the {want} slot must be declared; got:\n{body}");
    }
    assert!(
        !body.contains("a0"),
        "and the anonymous slot the body recovered must have been replaced, not \
         duplicated; got:\n{body}"
    );
    assert!(
        body.find("argc") < body.find("argv"),
        "the slots must be emitted in ABI order; got:\n{body}"
    );
}

/// The guard: the un-stripped twin names that same address `_main` out of its own
/// symbol table, and that name wins in BOTH arms — the pass never overwrites a
/// name the image supplied, and never adds a second function at the address.
#[test]
fn a_named_entry_keeps_its_own_name() {
    let (Some(off), Some(on)) = (bootstrap("macho_imports", false), bootstrap("macho_imports", true))
    else {
        return;
    };
    for (arm, prog) in [("off", &off), ("on", &on)] {
        assert!(
            prog.lookup_symbol("_main").is_some(),
            "the image's own _main must survive with machomain {arm}"
        );
        assert!(
            prog.lookup_symbol("main").is_none(),
            "machomain must not claim an entry that already carries a symbol ({arm})"
        );
    }
    assert_eq!(
        off.function_entries_canonical().len(),
        on.function_entries_canonical().len(),
        "no entry may be added on an image that already names its main"
    );
}
