//! With a program loaded and no option set, every catalog row that reports a live
//! `current` value reports its registry `default`: the engine starts where its
//! `phases.d/` row says it does, so a default changed on one side only fails here.

use std::path::PathBuf;

use kuna_console::ifacedecomp::{
    execute, register_console_commands, register_decomp_commands, IfaceDecompData,
    DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::kuna_console::register_kuna_commands;
use kuna_decomp::kuna_phases::{kuna_num_settables, kuna_settable_by_index};

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

#[test]
fn every_live_value_starts_at_its_registry_default() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/aif_gap_x86_64");
    let out = run(&[format!("load file {}", fixture.to_str().unwrap()), "phase catalog".into()]);
    let row = regex::Regex::new(
        r#"\{"option": "([^"]+)", "values": \[(?:"[^"]*"(?:, )?)*\], "default": "([^"]*)"(?:, "current": "([^"]*)")?"#,
    )
    .unwrap();
    let rows: Vec<_> = row.captures_iter(&out).collect();
    assert_eq!(rows.len(), kuna_num_settables(), "one catalog row per registry row:\n{out}");
    let mut live = 0;
    for (i, caps) in rows.iter().enumerate() {
        let st = kuna_settable_by_index(i);
        assert_eq!(&caps[1], st.option, "catalog row {i} is out of registry order");
        assert_eq!(&caps[2], st.shipped, "{}: catalog default", st.option);
        if let Some(current) = caps.get(3) {
            live += 1;
            assert_eq!(current.as_str(), st.shipped, "{}: engine starts away from its default", st.option);
        }
    }
    assert!(live > 0, "no row reported a live value; was the program loaded?\n{out}");
}
