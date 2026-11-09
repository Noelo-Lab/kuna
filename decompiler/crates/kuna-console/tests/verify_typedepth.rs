//! End-to-end gate for full-depth DWARF type resolution.
//!
//! The bug: the DIE→`Datatype` mapper reduced Ghidra's
//! `DWARFDataTypeImporter.trackRecursion` (a per-DIE-offset re-entry counter that
//! fires only on a genuine type cycle) to a flat three-hop budget counted over
//! EVERY link, transparent `typedef`/`const`/`volatile`/`restrict` included. Four
//! DIEs is an ordinary C declaration — `const char *const *`, `const size_t *`,
//! `char ***`, `state_t *` — so those chains ran out of budget and fell back to
//! `void`, and the stack locals, globals and deep-pointer parameters of a `-g`
//! binary rendered `void *`.
//!
//! The fixture `typedepth_x86_64` (source vendored alongside) is a `-g -O0` C
//! binary whose every function carries one of those shapes, plus the
//! self-referential and mutually recursive structs the budget claimed to be
//! guarding against.

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV;

/// Serializes the env-var flip + bootstrap across the concurrently-run tests.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture, decompile each of `funcs` and return the concatenated C.
fn decompile(funcs: &[&str]) -> String {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/typedepth_x86_64")
        .to_str().expect("UTF-8 fixture path")
        .to_string();

    // Pin the sibling `dwarfstructs` gate OFF so this file keeps measuring the
    // one axis it is about. That arm renames the anonymous fallback after the
    // shape it describes (`anon_struct_8_2`) and interns a name-colliding
    // aggregate under a size-suffixed name with its fields (`code_4 *base`, whose
    // members then resolve) instead of the shared anonymous fallback asserted
    // below — both strict improvements on what these assertions pin, and both
    // covered by `verify_dwarfstructs.rs`.
    std::env::set_var(DWARFSTRUCTS_ENV, "off");
    let prog = bootstrap_from_object(&path, "", &spec_roots);
    std::env::remove_var(DWARFSTRUCTS_ENV);
    let mut prog = prog.expect("bootstrap fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let mut cmds: Vec<String> = Vec::new();
    for f in funcs {
        cmds.push(format!("load function {f}"));
        cmds.push("decompile".into());
        cmds.push("print C".into());
    }
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

/// The headline plain-C case: a stack local declared `const char *const *`
/// (pointer → const → pointer → const → char) is five DIEs deep.
#[test]
fn stack_local_keeps_its_element_type() {
    let on = decompile(&["count_authors"]);
    assert!(
        on.contains("char **p") && !on.contains("void **p"),
        "`const char *const *p` should keep its element type, got:\n{on}"
    );
}

/// Three POINTER hops (`char ***`) exhausted the budget even after `cppproto`
/// collapsed the qualifier hops — the case that arm left on the table.
#[test]
fn three_pointer_hops_resolve() {
    let on = decompile(&["take_argvp"]);
    assert!(
        on.contains("char ***argvp"),
        "`char ***argvp` should resolve to its element type, got:\n{on}"
    );
}

/// An anonymous struct reached through its typedef is interned under the typedef
/// name, not the shared `anon_struct` (the `mbstate_t` shape).
#[test]
fn anonymous_aggregate_takes_its_typedef_name() {
    let on = decompile(&["use_state"]);
    assert!(
        on.contains("state_t *cur"),
        "the local should take its typedef name, got:\n{on}"
    );
}

/// ...unless that typedef name is already a kuna core type (`code`, which zlib's
/// `inftrees.h` really does typedef an anonymous struct to). The factory refuses
/// the redefinition, and the aggregate must fall back to the anonymous name
/// instead of letting the pointer degrade to `void *`.
#[test]
fn colliding_typedef_name_falls_back_not_to_void() {
    let on = decompile(&["walk_codes"]);
    assert!(
        !on.contains("void *base"),
        "a name collision must not degrade the pointer to void, got:\n{on}"
    );
    assert!(on.contains("anon_struct *base"), "expected the anonymous fallback, got:\n{on}");
}

/// A global whose element type is four DIEs deep was marked one byte wide, so it
/// rendered address-taken; with the real extent it decays like the array it is.
#[test]
fn global_array_gets_its_real_extent() {
    let on = decompile(&["main"]);
    assert!(
        on.contains("(char **)default_authors"),
        "the global array should carry its real extent, got:\n{on}"
    );
}

/// Termination on the shapes the depth budget claimed to be protecting against: a
/// self-referential struct and a mutually recursive pair still decompile (the
/// cycle counter, not a hop budget, is what keeps the walk finite).
#[test]
fn recursive_struct_types_terminate() {
    let on = decompile(&["list_len", "ping", "pong"]);
    assert!(on.contains("list_len"), "self-referential struct walk should decompile:\n{on}");
    assert!(on.contains("ping_s"), "mutually recursive struct should resolve:\n{on}");
    assert!(on.contains("pong_s"), "mutually recursive struct should resolve:\n{on}");
}
