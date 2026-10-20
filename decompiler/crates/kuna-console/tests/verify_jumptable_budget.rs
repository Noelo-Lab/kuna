//! The per-function budget reaches jump-table recovery (#1157).
//!
//! The fixture is the reported dispatch shape: one bounds check guarding two
//! MSVC two-stage tables (a byte map selecting a 4-byte offset that is added
//! to the image base), the second dispatch reached only through the first.

use crate::common;

use std::path::PathBuf;
use std::time::Duration;

use kuna_console::engine::{bootstrap_from_object_with_isa, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::project::decompile_targets;

fn specs() -> Vec<String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    vec![root.join("specs").to_str().unwrap().to_string()]
}

/// An x86-64 executable whose one segment maps the file at 0x400000, so the
/// dispatch's `lea r8, [0x400000]` is the image base.
fn double_dispatch_elf() -> Vec<u8> {
    const CODE: [u8; 100] = [
        0x8d, 0x51, 0xff, 0x83, 0xfa, 0x7f, 0x77, 0x56, 0x4c, 0x8d, 0x05, 0xf1, 0xef, 0xff, 0xff,
        0x48, 0x63, 0xc2, 0x41, 0x0f, 0xb6, 0x84, 0x00, 0x6c, 0x10, 0x00, 0x00, 0x41, 0x8b, 0x8c,
        0x80, 0x64, 0x10, 0x00, 0x00, 0x4c, 0x01, 0xc1, 0xff, 0xe1, 0x83, 0xfa, 0x7f, 0x77, 0x31,
        0x48, 0x63, 0xc2, 0x41, 0x0f, 0xb6, 0x84, 0x00, 0x00, 0x11, 0x00, 0x00, 0x41, 0x8b, 0x8c,
        0x80, 0xec, 0x10, 0x00, 0x00, 0x4c, 0x01, 0xc1, 0xff, 0xe1, 0xb8, 0x0a, 0x00, 0x00, 0x00,
        0xc3, 0xb8, 0x0b, 0x00, 0x00, 0x00, 0xc3, 0xb8, 0x0c, 0x00, 0x00, 0x00, 0xc3, 0xb8, 0x0d,
        0x00, 0x00, 0x00, 0xc3, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xc3,
    ];
    let flags = [1usize, 2, 4, 8, 16, 32, 64, 128];
    let case = |tag: usize| flags.iter().position(|&f| f == tag);
    let mut body = CODE.to_vec();
    for target in [0x1028u32, 0x105e] {
        body.extend(target.to_le_bytes());
    }
    body.extend((1..=128).map(|tag| if case(tag).is_some() { 0u8 } else { 1 }));
    for target in [0x1046u32, 0x104c, 0x1052, 0x1058, 0x105e] {
        body.extend(target.to_le_bytes());
    }
    body.extend((1..=128).map(|tag| case(tag).map_or(4u8, |k| (k % 4) as u8)));

    let mut image = vec![0; 0x1000];
    image[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        image[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let size = (0x1000 + body.len()) as u64;
    for (at, value) in
        [(24, 0x401000u64), (32, 64), (80, 0x400000), (88, 0x400000), (96, size), (104, size), (112, 0x1000)]
    {
        image[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    image.extend(body);
    image
}

fn program(budget: Option<Duration>) -> (ConsoleProgram, PathBuf) {
    let path = common::scratch_file("jumptable-budget", "elf");
    std::fs::write(&path, double_dispatch_elf()).unwrap();
    let mut program = bootstrap_from_object_with_isa(path.to_str().unwrap(), "", &specs(), None)
        .expect("bootstrap fixture with built processor specs");
    program.commit_pending_analysis().unwrap();
    program.arch_mut().kuna_fn_budget = budget;
    (program, path)
}

fn decompile(budget: Option<Duration>) -> (Option<String>, Option<String>) {
    let (mut program, path) = program(budget);
    let entry = program.find_entry_at(0x401000).expect("entry point");
    let mut result = decompile_targets(&mut program, vec![entry], true, false, false);
    std::fs::remove_file(path).unwrap();
    let r = result.remove(0);
    (r.code, r.error)
}

#[test]
fn both_byte_indexed_tables_recover() {
    let (code, error) = decompile(None);
    assert!(error.is_none(), "{error:?}");
    let code = code.unwrap();
    assert_eq!(code.matches("switch(").count(), 2, "{code}");
    assert!(code.contains("return 0xd;"), "{code}");
}

/// A deadline that has already passed when flow reaches the first table is
/// seen by the recovery driver itself, and the error names it.
#[test]
fn expired_budget_names_the_jump_table_being_recovered() {
    let (code, error) = decompile(Some(Duration::ZERO));
    assert!(code.is_none(), "{code:?}");
    assert_eq!(
        error.as_deref(),
        Some("per-function decompile budget exceeded (0 s) in jump-table recovery at 0x401026"),
    );
}

/// The console's transcript for `cmds` over the fixture.
fn console(budget: Option<Duration>, cmds: &[&str]) -> String {
    let mut status = ConsoleCommands::into_status(cmds.iter().map(|c| c.to_string()).collect());
    register_decomp_commands(&mut status);
    let (program, path) = program(budget);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap().conf = Some(program);
    for _ in cmds {
        execute(&mut status);
    }
    std::fs::remove_file(path).unwrap();
    status.optr
}

/// `load addr` follows flow before any drive (`kuna decompile`'s script), so
/// it is held to the same budget and fails rather than spin in table recovery.
#[test]
fn console_load_follows_flow_under_the_budget() {
    let unbudgeted = console(None, &["load addr 0x401000", "decompile", "print C"]);
    assert_eq!(unbudgeted.matches("switch(").count(), 2, "{unbudgeted}");
    let expired = console(Some(Duration::ZERO), &["load addr 0x401000"]);
    assert!(
        expired.contains(
            "Execution error: per-function decompile budget exceeded (0 s) in jump-table recovery at 0x401026"
        ),
        "{expired}"
    );
}
