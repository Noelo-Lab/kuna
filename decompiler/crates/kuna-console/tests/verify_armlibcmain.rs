//! End-to-end gate for the non-PIE ARM crt1 entry: a non-PIE ARM32 ELF recovers
//! the `main` its crt1 `_start` hands to `__libc_start_main` (P1 program prep).
//!
//! Fixtures: `armlibcmain_le32` (the classic `crt1.o` shape, `main` in a literal
//! pool word at 0x103dc) and `armlibcmain_got_le32` (the shape a modern
//! `-no-pie` link supplies, `main` at 0x10518 in a `.got` slot the linker filled
//! with no relocation to find it by). Both print a prompt from `main` and are
//! stripped, so nothing but `_start` says where the program's body is. Without
//! the pass `main` was not a function at all: its body fell inside the extent of
//! the entry before it, so the prompt it prints had no reader.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(fixture: &str) -> ConsoleProgram {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
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

/// Both crt1 shapes name the address `_start` hands over, and the body
/// decompiled there is the one that prints the prompt.
#[test]
fn both_crt1_shapes_recover_main() {
    for fixture in ["armlibcmain_le32", "armlibcmain_got_le32"] {
        let prog = bootstrap(fixture);
        assert!(prog.lookup_symbol("main").is_some(), "{fixture}: main must be discovered");
        let body = decompile(prog, "main");
        assert!(
            body.contains("main("),
            "{fixture}: and declared under that name; got:\n{body}"
        );
        // The body is the one that prints the prompt: the pool load feeding the
        // printf call, and the read that follows it, are both inside it.
        for want in ["__printf_chk(", "fgets("] {
            assert!(
                body.contains(want),
                "{fixture}: main's body must reach {want}; got:\n{body}"
            );
        }
    }
}

