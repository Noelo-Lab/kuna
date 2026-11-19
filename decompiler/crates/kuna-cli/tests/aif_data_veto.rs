//! `aif`'s data read/write veto: a gap entry the program only reads as data is
//! not a function, however well its bytes decode.

use std::process::Command;

fn functions(extra: &[&str]) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let bin = root.join("tests/stages/fixtures/aifdataveto/aifdataveto_x86_64");
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", bin.to_str().unwrap(), "--sleighpath", root.join("specs").to_str().unwrap()])
        .args(extra)
        .output()
        .expect("run source-built kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn a_function_shaped_table_the_code_reads_is_dropped_under_aif() {
    let with_aif = functions(&["--option", "listing", "on", "--option", "aif", "strict"]);
    assert!(with_aif.contains("0x401010"), "{with_aif}");
    assert!(!with_aif.contains("0x401020"), "the table is read as data, not a function:\n{with_aif}");

    let without = functions(&["--option", "listing", "on", "--option", "aif", "off"]);
    assert!(without.contains("0x401020"), "{without}");
}
