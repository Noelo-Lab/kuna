//! End-to-end coverage for direct calls through Mach-O import-pointer slots.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn boot() -> Option<ConsoleProgram> {
    let root = repo_root();
    let specs = vec![root.join("specs").to_str().unwrap().to_string()];
    let fixture = root.join("decompiler/crates/kuna-analysis/tests/fixtures/macho_import_slots");
    match bootstrap_from_object(fixture.to_str().unwrap(), "", &specs) {
        Ok(program) => Some(program),
        Err(error) => {
            eprintln!(
                "verify_macho_import_slots: skipping (build `.sla` with `make specs`): {}",
                error.explain()
            );
            None
        }
    }
}

fn decompile(program: ConsoleProgram, setup: &[&str]) -> String {
    let commands: Vec<String> = setup.iter().map(|command| command.to_string()).collect();
    let count = commands.len();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        data.as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap()
            .conf = Some(program);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn peimportcall_binds_only_the_typed_macho_import_slot() {
    let Some(on_program) = boot() else { return };
    let on = decompile(
        on_program,
        &["load function _call_slots", "decompile", "print C"],
    );
    assert!(
        on.contains("objc_msgSend("),
        "typed __got call was not bound:\n{on}"
    );
    assert!(
        !on.contains("dat_100001000"),
        "typed __got slot leaked anonymously:\n{on}"
    );
    assert!(
        on.contains("(*dat_100002000)("),
        "ordinary __objc_msgrefs data must remain anonymous:\n{on}"
    );

    let Some(off_program) = boot() else { return };
    let off = decompile(
        off_program,
        &[
            "option peimportcall off",
            "load function _call_slots",
            "decompile",
            "print C",
        ],
    );
    assert!(
        !off.contains("objc_msgSend("),
        "off must restore the old rendering:\n{off}"
    );
    assert!(
        off.contains("(*dat_100001000)("),
        "off did not restore the raw __got call:\n{off}"
    );
    assert!(
        off.contains("(*dat_100002000)("),
        "control call changed with the option:\n{off}"
    );
}
