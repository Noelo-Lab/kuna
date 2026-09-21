//! (kuna, issue #667) The ambiguity report has to offer a selector that works
//! on the image in hand: a linked image has no section-qualified coordinates,
//! so only the address separates two same-named entries.
//!
//! The fixture is vendored; bootstrapping needs the built `.sla` under `specs/`
//! (gitignored, `make specs`), so a specs-less environment is a visible skip
//! rather than a false green.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

fn decompile(binary: &str, args: &[&str]) -> (String, String, bool) {
    let bin = fixture(binary);
    let sp = specs();
    let mut argv = vec!["decompile", bin.as_str()];
    argv.extend_from_slice(args);
    argv.extend_from_slice(&["--sleighpath", sp.as_str()]);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(&argv)
        .output()
        .expect("run kuna");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

/// The report names the address form, not a section form that cannot work here.
#[test]
fn a_linked_image_ambiguity_offers_the_address_form() {
    let (_stdout, stderr, ok) = decompile("macho_dup_main", &["_main"]);
    if is_specs_skip(&stderr) {
        eprintln!("ambiguous_selector_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(!ok, "two definitions are spelled _main: {stderr}");
    assert!(
        stderr.contains("--addr 0x1000005a0") && stderr.contains("--addr 0x1000005b0"),
        "both candidates must be offered as address selectors:\n{stderr}"
    );
    assert!(
        !stderr.contains("section-qualified") && !stderr.contains("synthetic"),
        "a linked image has neither section coordinates nor synthetic addresses:\n{stderr}"
    );
}

/// And the form it offers resolves.
#[test]
fn the_offered_address_selects_one_candidate() {
    let (stdout, stderr, ok) = decompile("macho_dup_main", &["0x1000005a0", "--addr"]);
    if !ok && is_specs_skip(&stderr) {
        eprintln!("ambiguous_selector_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(ok, "the offered selector must work: {stderr}");
    assert!(stdout.contains("_main("), "no body for the chosen candidate:\n{stdout}");
}

/// A relocatable object keeps the section-qualified form, which is the one that
/// works there.
#[test]
fn a_relocatable_ambiguity_keeps_the_section_form() {
    let (_stdout, stderr, ok) = decompile("entry_selectors_x86_64.o", &["duplicate_local"]);
    if is_specs_skip(&stderr) {
        eprintln!("ambiguous_selector_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(!ok, "two .text definitions are both executable: {stderr}");
    assert!(
        stderr.contains("section-qualified") && stderr.contains("synthetic 0x"),
        "the relocatable report must be unchanged:\n{stderr}"
    );
}
