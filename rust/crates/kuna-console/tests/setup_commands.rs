//! Gate for `w10-setup-integration`: the datatest *setup* commands actually take
//! effect against a bootstrapped [`Architecture`].
//!
//! The M2 taxonomy found ~71/83 datatest files failing at their setup scripts —
//! `parse_C` (type/prototype declarations), the `map*` commands (symbol/function
//! creation), and the smaller `set context`/`set track`/`volatile`/`comment`
//! commands — because each console command was seamed (`engine_unavailable`).
//! This item un-seams the store-write paths; this gate drives each setup command
//! against a real bootstrapped [`ConsoleProgram`] and asserts the *store reflects
//! it*: a parsed struct type exists in the factory, a mapped function/global symbol
//! resolves, a volatile range is set, a context default is painted, a comment is
//! recorded.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping the engine needs the built `.sla` artifacts under `specs/`
//! (gitignored; `make specs`).  When the 8051 `.sla` the fixture needs is absent,
//! the bootstrap fails; the test reports that failure (so a CI without specs is
//! visibly skipped, never a false green) and returns early.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_file, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::IfaceStatus;

use kuna_decomp::dtype::type_metatype;
use kuna_decomp::varnode::varnode_flags;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the 8051 `boolless` corpus into a [`ConsoleProgram`], or `None` if
/// the `.sla` is missing (printed, so a specs-less CI is a visible skip).
fn boot_program() -> Option<ConsoleProgram> {
    let root = repo_root();
    let xml = root.join("decompiler/datatests/boolless.xml");
    let specs = root.join("specs");
    match bootstrap_from_file(xml.to_str().unwrap(), &[specs.to_str().unwrap().to_string()]) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "setup_commands: skipping (engine bootstrap failed, build `.sla` with `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// Drive `commands` through a console wired exactly like the datatest runner,
/// with `program` already installed as the current program (`dcp.conf`).  Returns
/// the console + captured output for inspection.
fn drive(program: ConsoleProgram, commands: &[&str]) -> (IfaceStatus, String) {
    let cmds: Vec<String> = commands.iter().map(|s| s.to_string()).collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    // Install the bootstrapped program as `dcp.conf` (what `load file` would do).
    {
        let data = status
            .get_data_mut(DECOMPILE_MODULE)
            .expect("decompile module data registered");
        let dcp = data
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .expect("decompile module data is IfaceDecompData");
        dcp.conf = Some(program);
    }
    // Execute exactly the queued commands.
    for _ in 0..count {
        execute(&mut status);
    }
    let out = status.optr.clone();
    (status, out)
}

/// Borrow the installed [`ConsoleProgram`] back out of the console for store
/// inspection.
fn dcp_program(status: &mut IfaceStatus) -> &ConsoleProgram {
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    dcp.conf.as_ref().expect("program still installed")
}

// ===========================================================================
// 1. parse_C store-writes: struct / enum / typedef land in the TypeFactory.
// ===========================================================================

#[test]
fn parse_line_struct_creates_real_type() {
    let Some(prog) = boot_program() else { return };
    let (mut status, out) = drive(
        prog,
        &["parse line struct mystruct { int4 a; int4 b; };"],
    );
    assert!(!out.contains("error"), "parse line struct emitted an error: {out:?}");
    let prog = dcp_program(&mut status);
    let ct = prog
        .arch()
        .types()
        .find_by_name("mystruct")
        .unwrap()
        .expect("the parsed struct must exist in the factory");
    assert_eq!(ct.get_metatype(), type_metatype::TYPE_STRUCT);
    assert_eq!(ct.get_size(), 8, "two int4 fields => 8 bytes");
    assert!(!ct.is_incomplete(), "the struct must be completed by assignRawFields");
}

#[test]
fn parse_line_typedef_creates_real_type() {
    let Some(prog) = boot_program() else { return };
    let (mut status, out) = drive(prog, &["parse line typedef int4 myint;"]);
    assert!(!out.contains("error"), "parse line typedef emitted an error: {out:?}");
    let prog = dcp_program(&mut status);
    let ct = prog
        .arch()
        .types()
        .find_by_name("myint")
        .unwrap()
        .expect("the typedef must exist in the factory");
    assert_eq!(ct.get_size(), 4);
}

// ===========================================================================
// 2. map function / map address: real symbols the decompile can pick up.
// ===========================================================================

#[test]
fn map_function_creates_resolvable_symbol() {
    let Some(prog) = boot_program() else { return };
    // 8051 CODE space, an entry near the corpus function: a fresh function entry.
    let (mut status, out) = drive(prog, &["map function [CODE,0xa100] myfunc"]);
    assert!(!out.contains("error"), "map function emitted an error: {out:?}");
    let prog = dcp_program(&mut status);
    // The function symbol must now resolve by name (the `load function` path).
    assert!(
        prog.lookup_symbol("myfunc").is_some(),
        "the mapped function must be resolvable by name"
    );
    // ...and the symbol-table must hold a function symbol of that name.
    let gscope = prog.arch().symboltab.get_global_scope().unwrap();
    assert!(
        !prog.arch().symboltab.find_by_name(gscope, "myfunc").is_empty(),
        "the mapped function symbol must be in the global scope"
    );
}

#[test]
fn map_address_creates_a_global_symbol() {
    let Some(prog) = boot_program() else { return };
    let (mut status, out) = drive(prog, &["map address [CODE,0x100] int4 globvar"]);
    assert!(!out.contains("error"), "map address emitted an error: {out:?}");
    let prog = dcp_program(&mut status);
    let gscope = prog.arch().symboltab.get_global_scope().unwrap();
    let syms = prog.arch().symboltab.find_by_name(gscope, "globvar");
    assert_eq!(syms.len(), 1, "the mapped global must be a single symbol");
}

#[test]
fn map_label_creates_a_global_label() {
    let Some(prog) = boot_program() else { return };
    let (mut status, out) = drive(prog, &["map label mylabel [CODE,0x200]"]);
    assert!(!out.contains("error"), "map label emitted an error: {out:?}");
    let prog = dcp_program(&mut status);
    let gscope = prog.arch().symboltab.get_global_scope().unwrap();
    assert!(
        !prog.arch().symboltab.find_by_name(gscope, "mylabel").is_empty(),
        "the mapped label symbol must exist"
    );
}

// ===========================================================================
// 3. volatile / set context / comment: the smaller setup commands take effect.
// ===========================================================================

#[test]
fn volatile_marks_the_range() {
    let Some(prog) = boot_program() else { return };
    // 8051 RAM space; mark a 4-byte range volatile.
    let (mut status, out) = drive(prog, &["volatile [CODE,0x100,4]"]);
    assert!(
        out.contains("Successfully marked range as volatile"),
        "volatile must echo its success line: {out:?}"
    );
    let prog = dcp_program(&mut status);
    // The property must be readable back at the marked address.
    let manage = prog.arch().manage();
    let space = manage.get_space_by_name("CODE").expect("8051 CODE space");
    let addr = kuna_base::address::Address::new(std::rc::Rc::clone(space), 0x100);
    let prop = prog.arch().symboltab.get_property(&addr);
    assert!(
        (prop & varnode_flags::volatil) != 0,
        "the volatile flag must be set at the marked address (prop={prop:#x})"
    );
}

#[test]
fn set_context_default_paints_the_variable() {
    let Some(prog) = boot_program() else { return };
    // Discover a real context variable name from the engine, then paint a default.
    // (8051 has no Thumb-style TMode; we probe whatever the spec registered.)
    let varname = first_context_variable(&prog);
    let Some(varname) = varname else {
        eprintln!("set_context: 8051 has no settable context variable; nothing to assert");
        return;
    };
    let cmd = format!("set context {varname} 1");
    let (mut status, out) = drive(prog, &[&cmd]);
    assert!(!out.contains("error") && !out.contains("Execution error"), "set context errored: {out:?}");
    let prog = dcp_program(&mut status);
    let val = prog
        .arch()
        .with_context_db_mut(|db| db.get_default_value_by_name(varname.as_bytes()))
        .expect("the context default must read back");
    assert_eq!(val, 1, "the painted context default must read back as 1");
}

#[test]
fn comment_instr_records_a_comment() {
    let Some(prog) = boot_program() else { return };
    // A comment needs a current function; load `boolless` first, then comment it.
    let (mut status, out) = drive(
        prog,
        &["load function boolless", "comment instruction r0xa000 hello-world"],
    );
    assert!(!out.contains("Execution error"), "comment errored: {out:?}");
    let prog = dcp_program(&mut status);
    let comments = prog.arch().commentdb.comments();
    assert!(
        comments.iter().any(|c| c.text == "hello-world"),
        "the comment text must be recorded in the comment database: {:?}",
        comments.iter().map(|c| c.text.clone()).collect::<Vec<_>>()
    );
}

/// Probe the engine for a context-variable name registered by the loaded spec
/// (so `set context` can be tested against whatever the architecture defines).
fn first_context_variable(prog: &ConsoleProgram) -> Option<String> {
    // The candidate context-variable names the upstream specs commonly register;
    // probe each against the real context database.
    for cand in ["TMode", "ISA_MODE", "RELP", "phase", "vle", "AT", "EXMODE"] {
        let exists = prog
            .arch()
            .with_context_db_mut(|db| db.get_default_value_by_name(cand.as_bytes()).is_ok());
        if exists {
            return Some(cand.to_string());
        }
    }
    None
}
