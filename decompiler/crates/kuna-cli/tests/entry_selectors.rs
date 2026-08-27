//! CLI coverage for relocatable-object function selectors and provenance JSON.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/entry_selectors_x86_64.o")
        .to_string_lossy()
        .into_owned()
}

fn specs() -> String {
    repo_root().join("specs").to_string_lossy().into_owned()
}

fn decomp_dbg() -> String {
    repo_root()
        .join("decompiler/target/debug/decomp_dbg")
        .to_string_lossy()
        .into_owned()
}

fn project_out_dir() -> PathBuf {
    repo_root()
        .join("decompiler/target")
        .join(format!("entry-selectors-project-{}", std::process::id()))
}

fn run(args: &[&str]) -> (String, String, bool) {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("spawn kuna");
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

#[test]
fn functions_reports_both_duplicate_local_definitions_with_object_coordinates() {
    let binary = fixture();
    let sleigh = specs();
    let (stdout, stderr, ok) = run(&[
        "functions",
        &binary,
        "--json",
        "--sleighpath",
        &sleigh,
        "--mode",
        "reliable",
    ]);
    if !ok && specs_missing(&stderr) {
        eprintln!("entry_selectors: skipping (no built x86-64.sla): {stderr}");
        return;
    }
    assert!(ok, "kuna functions failed: {stderr}");
    assert_eq!(
        stdout.matches("\"name\": \"duplicate_local\"").count(),
        2,
        "{stdout}"
    );
    assert!(
        stdout.contains("\"section\": \".text.selector_a\""),
        "{stdout}"
    );
    assert!(
        stdout.contains("\"section\": \".text.selector_b\""),
        "{stdout}"
    );
    assert!(stdout.contains("\"section_index\": 4"), "{stdout}");
    assert!(stdout.contains("\"section_index\": 6"), "{stdout}");
    assert_eq!(
        stdout.matches("\"offset_hex\": \"0x0\"").count(),
        2,
        "{stdout}"
    );
}

#[test]
fn decompile_all_rejects_ambiguous_names_and_offsets_but_accepts_a_section_selector() {
    let binary = fixture();
    let sleigh = specs();

    for (flag, selector) in [("--functions", "duplicate_local"), ("--addr", "0x0")] {
        let (stdout, stderr, ok) = run(&[
            "decompile-all",
            &binary,
            flag,
            selector,
            "--json",
            "--sleighpath",
            &sleigh,
            "--mode",
            "reliable",
        ]);
        if !ok && specs_missing(&stderr) {
            eprintln!("entry_selectors: skipping (no built x86-64.sla): {stderr}");
            return;
        }
        assert!(!ok, "ambiguous selector {selector:?} succeeded: {stdout}");
        assert!(stderr.contains("ambiguous"), "{stderr}");
        assert!(stderr.contains(".text.selector_a+0x0"), "{stderr}");
        assert!(stderr.contains(".text.selector_b+0x0"), "{stderr}");
        assert!(stderr.contains("synthetic 0x"), "{stderr}");
        assert!(stderr.contains("local"), "{stderr}");
    }

    for selector in [".text.selector_b+0x0", "6:0x0"] {
        let (stdout, stderr, ok) = run(&[
            "decompile-all",
            &binary,
            "--addr",
            selector,
            "--json",
            "--sleighpath",
            &sleigh,
            "--mode",
            "reliable",
        ]);
        assert!(
            ok,
            "section-qualified decompile failed for {selector}: {stderr}"
        );
        assert!(stdout.contains("\"count\": 1"), "{stdout}");
        assert!(stdout.contains("\"section_index\": 6"), "{stdout}");
        assert!(
            stdout.contains("\"section\": \".text.selector_b\""),
            "{stdout}"
        );
        assert!(stdout.contains("return 2;"), "{stdout}");
    }
}

#[test]
fn single_decompile_uses_the_same_strict_selector_rules() {
    let binary = fixture();
    let sleigh = specs();
    let console = decomp_dbg();
    assert!(
        PathBuf::from(&console).is_file(),
        "workspace gate did not build decomp_dbg"
    );

    let (stdout, stderr, ok) = run(&[
        "decompile",
        &binary,
        "duplicate_local",
        "--sleighpath",
        &sleigh,
        "--decomp-dbg",
        &console,
        "--mode",
        "reliable",
    ]);
    if !ok && specs_missing(&stderr) {
        eprintln!("entry_selectors: skipping (no built x86-64.sla): {stderr}");
        return;
    }
    assert!(!ok, "ambiguous name succeeded: {stdout}");
    assert!(stderr.contains("ambiguous"), "{stderr}");
    assert!(stderr.contains(".text.selector_a+0x0"), "{stderr}");
    assert!(stderr.contains(".text.selector_b+0x0"), "{stderr}");

    let (stdout, stderr, ok) = run(&[
        "decompile",
        &binary,
        ".text.selector_b+0x0",
        "--addr",
        "--sleighpath",
        &sleigh,
        "--decomp-dbg",
        &console,
        "--mode",
        "reliable",
    ]);
    assert!(ok, "section-qualified single decompile failed: {stderr}");
    assert!(stdout.contains("duplicate_local"), "{stdout}");
    assert!(stdout.contains("return 2;"), "{stdout}");
}

#[test]
fn invalid_object_coordinates_fail_with_actionable_diagnostics() {
    let binary = fixture();
    let sleigh = specs();

    let (_stdout, stderr, ok) = run(&[
        "decompile-all",
        &binary,
        "--addr",
        "6:0x6",
        "--json",
        "--sleighpath",
        &sleigh,
        "--mode",
        "reliable",
    ]);
    if !ok && specs_missing(&stderr) {
        eprintln!("entry_selectors: skipping (no built x86-64.sla): {stderr}");
        return;
    }
    assert!(!ok, "one-past-end section selector succeeded");
    assert!(stderr.contains("no function matches \"6:0x6\""), "{stderr}");

    let (_stdout, stderr, ok) = run(&[
        "decompile-all",
        &binary,
        "--addr",
        "0x12345",
        "--json",
        "--sleighpath",
        &sleigh,
        "--mode",
        "reliable",
    ]);
    assert!(!ok, "unmapped synthetic address succeeded");
    assert!(stderr.contains("synthetic load VMAs"), "{stderr}");
    assert!(stderr.contains(".section+0xOFFSET"), "{stderr}");
    assert!(stderr.contains("SECTION_INDEX:0xOFFSET"), "{stderr}");
    assert!(!stderr.contains("external symbol"), "{stderr}");

    let (_stdout, stderr, ok) = run(&[
        "decompile-all",
        &binary,
        "--addr",
        "6:not-hex",
        "--json",
        "--sleighpath",
        &sleigh,
    ]);
    assert!(!ok, "malformed section selector succeeded");
    assert!(stderr.contains("invalid address \"6:not-hex\""), "{stderr}");
}

#[test]
fn decompile_project_accepts_a_section_index_selector() {
    let binary = fixture();
    let sleigh = specs();
    let output = project_out_dir();
    let _ = std::fs::remove_dir_all(&output);

    let (stdout, stderr, ok) = run(&[
        "decompile-project",
        &binary,
        "--addr",
        "6:0x0",
        "--output",
        output.to_str().unwrap(),
        "--sleighpath",
        &sleigh,
        "--mode",
        "reliable",
    ]);
    if !ok && specs_missing(&stderr) {
        eprintln!("entry_selectors: skipping project test (no built x86-64.sla): {stderr}");
        return;
    }
    assert!(ok, "section-index project export failed: {stderr}");
    assert!(stdout.contains("functions: 1 ok, 0 failed"), "{stdout}");

    let c_path = output.join("entry_selectors_x86_64.o.c");
    let c = std::fs::read_to_string(&c_path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", c_path.display()));
    assert!(c.contains("return 2;"), "{c}");
    assert!(
        !c.contains("return 1;"),
        "unselected section A leaked into project:\n{c}"
    );
    let _ = std::fs::remove_dir_all(&output);
}
