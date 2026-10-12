//! End-to-end two-pass gate for `coldentry` — the extra entry points of a
//! multi-entry `.cold` fragment (P1 code/data partition).
//!
//! Fixture: `coldentry_x86_64`, a stripped hand-written-asm ELF. The `.cold`
//! fragment FDE `[0x401000, 0x401018)` holds two `mov $n,%edi; call die; ud2`
//! paths; `check`@`0x401030` enters the first with `je 0x401000` and the second
//! with `je 0x40100c`, both `rel32`.
//!
//! * **option OFF (the bug):** only the FDE start `sub_401000` is a function;
//!   `0x40100c` is strictly inside that FDE, so no oracle names it.
//! * **default (the fix):** `sub_40100c` is a function, entered from outside its
//!   FDE after a `ud2`, and the FDE start still is too.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

const SECOND: &str = "sub_40100c";
const FIRST: &str = "sub_401000";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(coldentry: bool) -> ConsoleProgram {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/coldentry_x86_64");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("coldentry", if coldentry { "on" } else { "off" })
        .expect("coldentry flips");
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

#[test]
fn second_cold_entry_is_missing_with_the_option_off() {
    let prog = bootstrap(false);
    assert!(prog.lookup_symbol(FIRST).is_some(), "the FDE start {FIRST} is always a function");
    assert!(
        prog.lookup_symbol(SECOND).is_none(),
        "with coldentry off 0x40100c must not be discovered — off restores the previous set"
    );
}

#[test]
fn second_cold_entry_is_a_function_by_default() {
    let prog = bootstrap(true);
    assert!(prog.lookup_symbol(FIRST).is_some(), "the FDE start {FIRST} must survive");
    assert!(
        prog.lookup_symbol(SECOND).is_some(),
        "0x40100c is entered by `je` from check@0x401030 after a ud2 — it must be a function"
    );
    let body = decompile(prog, SECOND);
    assert!(body.contains(SECOND), "expected a decompiled body for {SECOND}, got:\n{body}");
}
