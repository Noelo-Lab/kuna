//! End-to-end gate for the interior-entry rejection (`listing/kuna_falseentry.rs`).
//!
//! A Rust gate rather than a `tests/stages/*.xml` case for the reason the sibling
//! discovery gates give: the pass reads the Listing tier, which the datatest
//! harness never builds. It is always on wherever the walk's functions are
//! committed, so this pins the fixed inventory in one pass.
//!
//! Fixture `cortexm_falseentry_le32` (built by the committed
//! `cortexm_falseentry_le32.py`): a stripped Cortex-M image whose prologue-pattern
//! and frame scans plant three entries inside real functions, beside a
//! pattern-only function that must stay one.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap() -> kuna_console::engine::ConsoleProgram {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_falseentry_le32");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    for option in ["listing", "funcstart_patterns", "aif"] {
        prog.arch_mut().set_kuna_option(option, "on").expect("discovery option flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

fn decompile(prog: kuna_console::engine::ConsoleProgram, func: &str) -> String {
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

/// The push after `clamp`'s predicated return, the push `pick`'s `cbz` reaches
/// past its return, and the push inside the pointer-only handler are not
/// functions; the callers and the pattern-only `orphan` after `fatal`'s padding
/// are.
#[test]
fn interior_entries_are_not_functions() {
    let prog = bootstrap();
    for name in ["sub_8008026", "sub_8008046", "sub_8008084"] {
        assert!(prog.lookup_symbol(name).is_none(), "{name} is the middle of a function");
    }
    for name in ["sub_8008000", "sub_8008020", "sub_8008040", "sub_8008060", "sub_8008068"] {
        assert!(prog.lookup_symbol(name).is_some(), "{name} is a function");
    }
}

/// The handler is reached only through its `.rodata` pointer. Its planted frame
/// entry split the gap the pointer-entry scan needs, so its real start is
/// emitted in the planted entry's place and decompiles whole, null test first.
#[test]
fn the_pointer_only_handler_starts_at_its_pointer() {
    let prog = bootstrap();
    assert!(prog.lookup_symbol("sub_8008080").is_some(), "the handler's pointer target is its entry");
    let body = decompile(prog, "sub_8008080");
    assert!(body.contains("return NULL;") || body.contains("return 0;"), "the early exit is kept:\n{body}");
    assert!(body.contains(")();"), "the call through the field is kept:\n{body}");
    assert!(!body.contains("funcboundflow"), "nothing truncates the flow:\n{body}");
}
