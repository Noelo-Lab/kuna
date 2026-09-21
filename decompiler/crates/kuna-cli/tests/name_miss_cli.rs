//! (kuna, issue #666) What `kuna decompile <bin> <name>` answers when the name
//! is not in the image's symbol table: the C spelling of a Mach-O symbol is not
//! a miss at all, and a miss that IS one says what the image carries instead of
//! calling every image stripped.
//!
//! The binaries are vendored fixtures; bootstrapping needs the built `.sla`
//! under `specs/` (gitignored, `make specs`), so a specs-less environment is a
//! visible skip rather than a false green.

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

fn decompile(binary: &str, selector: &str) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(["decompile", &fixture(binary), selector, "--sleighpath", &specs()])
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

/// The C spelling of a Mach-O function selects it.
#[test]
fn a_macho_c_name_decompiles() {
    let (stdout, stderr, ok) = decompile("macho_imports", "main");
    if !ok && is_specs_skip(&stderr) {
        eprintln!("name_miss_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(ok, "the C spelling of _main must select it: {stderr}");
    assert!(stdout.contains("main("), "no body for main:\n{stdout}");
}

/// A miss on an image that names its functions names the near spelling and does
/// not claim the image is stripped.
#[test]
fn a_near_miss_is_named_instead_of_blamed_on_stripping() {
    let (_stdout, stderr, ok) = decompile("fauxware", "start");
    if is_specs_skip(&stderr) {
        eprintln!("name_miss_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(!ok, "start is not a name fauxware carries: {stderr}");
    assert!(
        stderr.contains(r#"did you mean "_start" (0x400580)?"#),
        "the near spelling must be named:\n{stderr}"
    );
    assert!(
        !stderr.contains("stripped"),
        "fauxware has a symbol table:\n{stderr}"
    );
}

/// And a miss with no near spelling says how many names the image does have.
#[test]
fn a_plain_miss_says_the_image_is_not_stripped() {
    let (_stdout, stderr, ok) = decompile("fauxware", "zork");
    if is_specs_skip(&stderr) {
        eprintln!("name_miss_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(!ok, "zork is not a name fauxware carries: {stderr}");
    assert!(
        stderr.contains("none spelled that way") && !stderr.contains("stripped"),
        "a named image must not be reported as stripped:\n{stderr}"
    );
}

/// The by-address advice survives where it is true: every entry of this image is
/// an engine placeholder, so no name can select in it.
#[test]
fn a_stripped_image_still_gets_the_addr_advice() {
    let (_stdout, stderr, ok) = decompile("argclobber_x86_64", "zork");
    if is_specs_skip(&stderr) {
        eprintln!("name_miss_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(!ok, "a stripped image answers no name: {stderr}");
    assert!(
        stderr.contains("for a stripped binary pass an address with --addr"),
        "the by-address advice must survive:\n{stderr}"
    );
}
