//! `cstringconst`: a pointer-wide constant inside a string-literal section
//! prints as the literal there, whatever type it reaches the printer with.
//!
//! Fixture: `tests/fixtures/macho_cstringconst_arm64`, an arm64 dylib at base 0
//! built from `macho_cstringconst.c` (see the fixtures README). Its literals lie
//! below the pointer lower bound and are shorter than the string pass's minimum,
//! so nothing types them `char *`: `emit` passes `" ("` to a call through an
//! untyped function pointer whose other call site passes an integer there, and
//! `choose` picks between `".SH"` and what an untyped callee returns, then passes
//! the choice to the same pointer.
//!
//! - **`cstringconst` off**: `0x610` and `v1 = 0x613;`.
//! - **`cstringconst` on** (default): `" ("` and `v1 = ".SH";`.
//!
//! Guard fixture: `tests/fixtures/macho_cstringconst_guard_arm64`, from
//! `macho_cstringconst_guard.c`. `setvbuf`'s declared `size_t` `0x58b` (the first
//! byte of `"hello world"`) and `0x58d` (two bytes into it, through an untyped
//! function pointer) stay integers either way; `lit`'s `"id"` and `"hello world"`
//! are literals when on.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;

const BIN: &str = "macho_cstringconst_arm64";
const GUARD_BIN: &str = "macho_cstringconst_guard_arm64";

/// `(fixture, function VMA, expected off, expected on)`.
const CASES: [(&str, u64, &str, &str); 5] = [
    (BIN, 0x508, "(a0,0x610,0);", "(a0,\" (\",0);"),
    (BIN, 0x55c, "v1 = 0x613;", "v1 = \".SH\";"),
    (GUARD_BIN, 0x510, "setvbuf(a0,NULL,0,0x58b);", "setvbuf(a0,NULL,0,0x58b);"),
    (GUARD_BIN, 0x520, "(a0,0x58d,0);", "(a0,0x58d,0);"),
    (GUARD_BIN, 0x538, "(a0,0x588,0x58b);", "(a0,\"id\",\"hello world\");"),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
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

/// Bootstrap `bin`, set `cstringconst`, commit, then decompile the function at `vma`.
fn render(bin: &str, vma: u64, on: bool) -> String {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(bin);
    assert!(path.exists(), "missing fixture {path:?}");

    let mut prog = bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("cstringconst", if on { "on" } else { "off" })
        .expect("cstringconst flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let name = name_at(&prog, vma);
    let code_space =
        prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let fd = {
        let arch = prog.arch_mut();
        build_and_follow_flow(arch, &name, Address::new(code_space, vma), kuna_console::engine::UNBOUNDED_SIZE)
            .expect("the funcdata builds")
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

#[test]
fn literal_section_constants_print_as_literals_only_when_on() {
    for &(bin, vma, off_line, on_line) in &CASES {
        let off = render(bin, vma, false);
        let on = render(bin, vma, true);
        assert!(off.contains(off_line), "{bin} 0x{vma:x} cstringconst off must print `{off_line}`:\n{off}");
        assert!(on.contains(on_line), "{bin} 0x{vma:x} cstringconst on must print `{on_line}`:\n{on}");
    }
}
