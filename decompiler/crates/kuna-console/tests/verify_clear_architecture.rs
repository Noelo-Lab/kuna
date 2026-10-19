//! `clear architecture` drops the console facts bound to the old image, so a
//! `parse line` prototype cannot land on the next image's function at the same
//! name or entry address.

use std::path::PathBuf;

use kuna_console::ifacedecomp::{
    execute, register_console_commands, register_decomp_commands, IfaceDecompData,
    DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::kuna_console::register_kuna_commands;

fn run(commands: &[String]) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = root.join("specs").to_str().unwrap().to_string();
    let mut status = ConsoleCommands::into_status(commands.to_vec());
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    register_console_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .spec_roots = vec![specs];
    for _ in 0..commands.len() {
        execute(&mut status);
    }
    status.optr.clone()
}

fn signature(output: &str) -> &str {
    output.lines().find(|line| line.contains("main(")).unwrap_or_else(|| panic!("no signature:\n{output}"))
}

#[test]
fn parse_line_prototype_does_not_survive_clear_architecture() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/aif_gap_x86_64");
    let load = vec![format!("load file {}", fixture.to_str().unwrap()), "read symbols".to_string()];
    let declare = "parse line extern double main(double a,double b,double c);".to_string();
    let tail = ["load function main", "decompile", "print C"].map(String::from);

    let fresh = run(&[load.clone(), tail.to_vec()].concat());
    let declared = run(&[load.clone(), vec![declare.clone()], tail.to_vec()].concat());
    let cleared = run(&[load.clone(), vec![declare, "clear architecture".into()], load, tail.to_vec()].concat());

    assert!(signature(&declared).contains("main(float8 a"), "declaration must apply:\n{declared}");
    assert_eq!(signature(&cleared), signature(&fresh), "cleared session:\n{cleared}");
}
