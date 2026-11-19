//! `aif`'s data read/write veto: a gap entry the program only reads as data is
//! not a function, however well its bytes decode.

use std::process::Command;

fn kuna(command: &str, fixture: &str, extra: &[&str]) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let bin = root.join("tests/stages/fixtures").join(fixture);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([command, bin.to_str().unwrap(), "--sleighpath", root.join("specs").to_str().unwrap()])
        .args(extra)
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn functions(extra: &[&str]) -> String {
    kuna("functions", "aifdataveto/aifdataveto_x86_64", extra)
}

#[test]
fn a_function_shaped_table_the_code_reads_is_dropped_under_aif() {
    let with_aif = functions(&["--option", "listing", "on", "--option", "aif", "strict"]);
    assert!(with_aif.contains("0x401010"), "{with_aif}");
    assert!(!with_aif.contains("0x401020"), "the table is read as data, not a function:\n{with_aif}");

    let without = functions(&["--option", "listing", "on", "--option", "aif", "off"]);
    assert!(without.contains("0x401020"), "{without}");
}

/// An arm64e hardened jump table: the `cmp`/`csel` clamp bounds the switch,
/// and the inline table `LC_FUNCTION_STARTS` lists is read as data, not code.
#[test]
fn a_hardened_arm64e_jump_table_is_a_switch_and_no_entry() {
    let fixture = "relswitch/relswitch_arm64e";
    let c = kuna("decompile", fixture, &["_dispatch"]);
    assert!(c.contains("switch(a0 >> 0x3e)") && c.contains("case 3:"), "{c}");
    assert!(!c.contains("jump-as-call") && !c.contains("sub_4060"), "{c}");

    let without = kuna("functions", fixture, &["--option", "listing", "on", "--option", "aif", "off"]);
    assert!(without.contains("0x4060"), "the start is listed without the veto:\n{without}");
    let with_aif = kuna("functions", fixture, &["--option", "listing", "on", "--option", "aif", "strict"]);
    assert!(!with_aif.contains("0x4060"), "the table is read as data:\n{with_aif}");
}
