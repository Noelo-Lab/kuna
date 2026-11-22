//! (kuna) An M-profile ELF's Peripheral and System regions are Device memory:
//! a store to a peripheral register is kept, and a status register polled in a
//! loop is read on every iteration.
//!
//! Fixture: `cortexm3_device.elf`, built from `cortexm3_device.c` (the build
//! command is in its header).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    DECOMPILE_MODULE,
    IfaceDecompData,
    execute,
    register_decomp_commands,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn decompile(func: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = vec![root.join("specs").to_string_lossy().to_string()];
    let fixture = root.join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm3_device.elf");
    let mut prog = bootstrap_from_object(fixture.to_str().unwrap(), "", &specs)
        .expect("bootstrap the fixture");
    prog.commit_pending_analysis().expect("analysis commit");
    let cmds: Vec<String> = vec![
        format!("load function {func}"),
        "decompile".into(),
        "print C".into(),
    ];
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    data.as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(prog);
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn peripheral_stores_and_polls_are_kept() {
    let clock = decompile("clock_start");
    assert!(
        clock.contains("= 1;") && clock.contains("= 0x10001;"),
        "both stores to RCC_CR:\n{clock}"
    );
    assert!(
        clock.contains("while"),
        "the ready poll stays a loop:\n{clock}"
    );
    let send = decompile("uart_send2");
    assert_eq!(
        send.matches("_40013804 = ").count(),
        2,
        "two stores to the data register:\n{send}"
    );
    let read = decompile("uart_read2");
    assert_eq!(
        read.matches("_40013804").count(),
        2,
        "two reads of the data register:\n{read}"
    );
}

#[test]
fn system_region_stores_are_kept() {
    let freeze = decompile("debug_freeze");
    assert!(
        freeze.contains("= 0;") && freeze.contains("= 7;"),
        "both stores to 0xE0042004:\n{freeze}"
    );
}
