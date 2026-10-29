//! `cfstrings` end-to-end gate — render a Mach-O Core Foundation constant string
//! (`__cfstring`) as the string literal it holds, instead of `dat_<addr>`.
//!
//! Fixture: `tests/fixtures/macho_cfstring` — a self-contained arm64 Mach-O built
//! from `macho_cfstring.c` with `zig cc -target aarch64-macos -O0`. It hand-builds
//! three `__builtin_CFString` constants in `__DATA_CONST,__cfstring` (8-bit,
//! `flags` 0x7c8) and a getter returning each one's address. The second string is
//! longer than a 32-byte entry; the third entry sits right behind it. The char
//! pointers are classic `LC_DYLD_INFO` pointers into `__TEXT,__cstring`. The build
//! recipe + the pinned VMAs are in `tests/fixtures/README.md`.
//!
//! ## The two-state proof (the `verify_objc` posture)
//!
//! - **`--option cfstrings off`** (default): each getter returns the bare
//!   `__cfstring` entry address — the CFString struct the printer cannot follow on
//!   its own.
//! - **`--option cfstrings on`**: each returns its literal — the entry's char
//!   pointer resolved, the string seeded into the `StringManager`, the entry typed
//!   `char[N]` over its own 32 bytes only, so a long string never hides the next
//!   entry.
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

/// `(getter VMA, entry address rendered when off, literal rendered when on)`.
/// The second string is longer than one 32-byte entry, and the third entry sits
/// right behind it, so it is only reachable when the second entry's `char[N]`
/// stays inside its own entry.
const ENTRIES: [(u64, &str, &str); 3] = [
    (0x100000538, "0x100004008", "cfstring hello"),
    (0x100000544, "0x100004028", "a constant string longer than one entry"),
    (0x100000550, "0x100004048", "after the long one"),
];

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
fn run(mode: Mode, vma: u64) -> String {
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

    let name = name_at(&prog, vma);
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
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
/// default and becomes the string literal only when `cfstrings` is on, for every
/// entry, including the one behind an entry whose string is longer than it.
#[test]
fn cfstring_renders_as_literal_only_when_on() {
    for (vma, entry, text) in ENTRIES {
        let off = run(Mode::Off, vma);
        let on = run(Mode::On, vma);
        assert!(
            off.contains(&format!("return {entry};")),
            "cfstrings off must render the bare __cfstring entry address {entry}:\n{off}"
        );
        assert!(!off.contains(text), "cfstrings off must not render {text:?}:\n{off}");
        assert!(
            on.contains(&format!("return \"{text}\";")),
            "cfstrings on must render the CFString literal {text:?}:\n{on}"
        );
        assert!(on.contains("char *"), "cfstrings on types the constant a char pointer:\n{on}");
    }
}
