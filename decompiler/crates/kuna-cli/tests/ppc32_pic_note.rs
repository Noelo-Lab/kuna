use crate::common;
use std::process::Command;

fn pic_image() -> String {
    common::repo_root().join("decompiler/crates/kuna-cli/tests/fixtures/ppc32-pic-imports.elf").to_str().unwrap().to_owned()
}

const NOTE: &str = "PPC32 PIC import stub";

#[test]
fn a_ppc32_pic_run_without_abi_says_how_to_name_its_imports() {
    let image = pic_image();
    let (_, err, code) = common::run_kuna(&["functions", &image, "--mode", "reliable"]);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains(NOTE) && err.contains("--option ppc32picimports abi"), "{err}");
    let (_, err, _) = common::run_kuna(&["decompile", &image, "check", "--mode", "fast"]);
    assert!(err.contains(NOTE), "{err}");
    let (_, err, _) = common::run_kuna(&["functions", &image, "--mode", "reliable", "--option", "ppc32picimports", "on"]);
    assert!(err.contains(NOTE), "caller-local recovery still leaves the stubs unnamed: {err}");
    for run in [&["disassemble", &image, "check", "--mode", "reliable"][..], &["xrefs", &image, "--from", "check"]] {
        let (_, err, _) = common::run_kuna(run);
        assert!(err.contains(NOTE) && err.contains("may name"), "{}: {err}", run[0]);
    }
    let (_, err, _) = common::run_kuna(&["strings", &image]);
    assert!(!err.contains(NOTE), "strings prints no import names: {err}");
}

#[test]
fn a_run_that_proves_r30_or_has_no_pic_stubs_prints_no_note() {
    let image = pic_image();
    let (_, err, _) = common::run_kuna(&["functions", &image, "--mode", "reliable", "--option", "ppc32picimports", "abi"]);
    assert!(!err.contains(NOTE), "{err}");
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", &image, "--mode", "reliable"])
        .env("KUNA_PPC32PICIMPORTS", "abi")
        .output()
        .unwrap();
    assert!(!String::from_utf8_lossy(&out.stderr).contains(NOTE));
    let (_, err, _) = common::run_kuna(&["functions", &common::fixture("fauxware")]);
    assert!(!err.contains(NOTE), "{err}");
    let (_, err, _) = common::run_kuna(&["functions", &common::fixture("plt_ppc64le")]);
    assert!(!err.contains(NOTE), "{err}");
}

#[test]
fn the_summary_carries_the_note_instead_of_stderr() {
    let image = pic_image();
    let (out, err, code) = common::run_kuna(&["functions", &image, "--mode", "reliable", "--summary", "--json"]);
    assert_eq!(code, 0, "{err}");
    assert!(!err.contains(NOTE), "{err}");
    assert!(out.contains("\"ppc32-pic-imports\"") && out.contains(NOTE), "{out}");
    let (out, _, _) = common::run_kuna(&["functions", &image, "--mode", "reliable", "--summary", "--json", "--option", "ppc32picimports", "abi"]);
    assert!(!out.contains("\"ppc32-pic-imports\""), "{out}");
}
