//! The target of a `jmp rel32` thunk is a function in every mode (GH-992), and
//! `--option thunkentry off` restores the thunk-only inventory.

use crate::common;

fn inventory(mode: &str, extra: &[&str]) -> Vec<(String, u64)> {
    let binary = common::fixture("thunk_table_pe_i386.exe");
    let specs = common::repo_root().join("specs");
    let mut args = vec![
        "functions",
        binary.as_str(),
        "--mode",
        mode,
        "--json",
        "--sleighpath",
        specs.to_str().unwrap(),
    ];
    args.extend_from_slice(extra);
    let (stdout, stderr, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{stdout}\n{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["address_hex"].as_str().unwrap().to_owned(),
                f["size"].as_u64().unwrap(),
            )
        })
        .collect()
}

fn listed(rows: &[(&str, u64)]) -> Vec<(String, u64)> {
    rows.iter().map(|&(a, s)| (a.to_owned(), s)).collect()
}

#[test]
fn every_thunk_target_is_a_function_in_every_mode() {
    let expected = listed(&[
        ("0x401000", 5),
        ("0x401005", 5),
        ("0x40100a", 6),
        ("0x401010", 32),
        ("0x401030", 16),
        ("0x401040", 20),
    ]);
    for mode in ["fast", "reliable", "aggressive"] {
        assert_eq!(inventory(mode, &[]), expected, "--mode {mode}");
    }
}

#[test]
fn thunkentry_off_restores_the_thunk_only_inventory() {
    assert_eq!(
        inventory("fast", &["--option", "thunkentry", "off"]),
        listed(&[("0x401000", 5), ("0x401005", 5), ("0x40100a", 74)]),
    );
    assert_eq!(
        inventory("reliable", &["--option", "thunkentry", "off"]),
        listed(&[
            ("0x401000", 5),
            ("0x401005", 5),
            ("0x40100a", 6),
            ("0x401010", 48),
            ("0x401040", 20),
        ]),
    );
}
