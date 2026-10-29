//! `cfstrings` end-to-end gate — render a Mach-O Core Foundation constant string
//! (`__cfstring`) as the string literal it holds, instead of `dat_<addr>`.
//!
//! Fixture: `tests/fixtures/macho_cfstring` — a self-contained arm64 Mach-O built
//! from `macho_cfstring.c` with `zig cc -target aarch64-macos -O0`. It hand-builds
//! one `__builtin_CFString` constant in `__DATA_CONST,__cfstring`
//! (`{ ___CFConstantStringClassReference, 0x7c8 (8-bit), "cfstring hello", 14 }`)
//! and a function `_get_greeting` that returns its address. Its char pointer is a
//! classic `LC_DYLD_INFO` pointer into `__TEXT,__cstring`. The build recipe + the
//! pinned VMA are in `tests/fixtures/README.md`.
//!
//! ## The two-state proof (the `verify_objc` posture)
//!
//! - **`--option cfstrings off`** (default): `_get_greeting` returns the bare
//!   `__cfstring` entry address `0x100004008` — the CFString struct the printer
//!   cannot follow on its own.
//! - **`--option cfstrings on`**: the SAME function returns the literal
//!   `"cfstring hello"` — the entry's char pointer resolved, the string seeded
//!   into the `StringManager`, the entry typed `char[N]`.
//!
//! Both run on the real-Mach-O path, so the XML datatest oracles never reach this.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

/// The pinned VMA of `_get_greeting` in the fixture (PIE imagebase `0x100000000`).
const GET_GREETING_VMA: u64 = 0x100000538;

/// The resolved CFString text the pass renders.
const GREETING: &str = "cfstring hello";

/// The bare `__cfstring` entry address the default run renders instead.
const ENTRY_ADDR: &str = "0x100004008";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

enum Mode {
    Off,
    On,
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// Bootstrap the fixture, (optionally) enable `cfstrings`, commit, then decompile
/// `_get_greeting` and return its rendered C.
fn run(mode: Mode) -> String {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixtures().join("macho_cfstring");
    assert!(bin.exists(), "missing fixture {bin:?}");

    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");

    if let Mode::On = mode {
        prog.arch_mut().set_kuna_option("cfstrings", "on").expect("cfstrings flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let name = name_at(&prog, GET_GREETING_VMA);
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), GET_GREETING_VMA);
    let fd = {
        let arch = prog.arch_mut();
        build_and_follow_flow(arch, &name, addr, kuna_console::engine::UNBOUNDED_SIZE)
            .expect("the _get_greeting funcdata builds")
    };

    let cmds: Vec<String> = ["decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
        dcp.fd = Some(fd);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The two-state proof: a `__cfstring` reference is the bare entry address by
/// default and becomes the string literal only when `cfstrings` is on.
#[test]
fn cfstring_renders_as_literal_only_when_on() {
    let off = run(Mode::Off);
    let on = run(Mode::On);

    eprintln!("---- cfstrings OFF (default) ----\n{off}");
    eprintln!("---- cfstrings ON ----\n{on}");

    // Default: the bare __cfstring entry address, NOT the string.
    assert!(
        off.contains(ENTRY_ADDR),
        "cfstrings off must render the bare __cfstring entry address {ENTRY_ADDR}:\n{off}"
    );
    assert!(
        !off.contains(GREETING),
        "cfstrings off must NOT render the string {GREETING:?} (proves the value is the pass's):\n{off}"
    );

    // On: the resolved string literal.
    assert!(
        on.contains(&format!("\"{GREETING}\"")),
        "cfstrings on must render the CFString literal {GREETING:?}:\n{on}"
    );
    assert!(
        !on.contains(ENTRY_ADDR),
        "cfstrings on must replace the bare entry address with the literal:\n{on}"
    );
}
