//! End-to-end two-pass gate for `importvote`: an import with no declaration
//! takes the arguments its other call sites pass.
//!
//! Fixture: `importvote_x86_64.so` (`gcc -O2 -fPIC -shared`, source alongside).
//! `g` is `add esi,1; jmp ext3@plt`, forwarding its `edx`; `h` calls
//! `ext3(x, 2, 3)` and returns its result plus one.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn decompile(func: &str, on: bool) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/importvote_x86_64.so");
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut().set_kuna_option("importvote", if on { "on" } else { "off" }).expect("importvote flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> = [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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
fn forwarder_drops_the_passed_through_argument_with_the_option_off() {
    let off = decompile("g", false);
    assert!(off.contains("ext3(a0,a1 + 1); // tail-call"), "got:\n{off}");
    assert!(off.contains("void g("), "got:\n{off}");
}

#[test]
fn forwarder_passes_what_the_other_call_site_passes() {
    let on = decompile("g", true);
    assert!(on.contains(",a2); // tail-call"), "the forwarded edx is the third argument, got:\n{on}");
    assert!(on.contains("return ext3("), "h reads ext3's result, so g returns it, got:\n{on}");
}
