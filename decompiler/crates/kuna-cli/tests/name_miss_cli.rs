//! (kuna, issue #666) What `kuna decompile <bin> <name>` answers when the name
//! is not in the image's symbol table: the C spelling of a Mach-O symbol is not
//! a miss at all, and a miss that IS one says what the image carries instead of
//! calling every image stripped.

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

/// The C spelling of a Mach-O function selects it.
#[test]
fn a_macho_c_name_decompiles() {
    let (stdout, stderr, ok) = decompile("macho_imports", "main");
    assert!(ok, "the C spelling of _main must select it: {stderr}");
    assert!(stdout.contains("main("), "no body for main:\n{stdout}");
}

/// A miss on an image that names its functions names the near spelling and does
/// not claim the image is stripped.
#[test]
fn a_near_miss_is_named_instead_of_blamed_on_stripping() {
    let (_stdout, stderr, ok) = decompile("fauxware", "start");
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
    assert!(!ok, "a stripped image answers no name: {stderr}");
    assert!(
        stderr.contains("for a stripped binary pass an address with --addr"),
        "the by-address advice must survive:\n{stderr}"
    );
}

/// (GH #1339) A Mach-O global is data: `functions` does not list it, and a
/// decompile by either spelling of its name says what it is instead of lifting
/// its bytes as code.
#[test]
fn a_macho_data_name_is_reported_as_data() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(["functions", &fixture("macho_datasyms_arm64"), "--sleighpath", &specs()])
        .output()
        .expect("run kuna functions");
    let listing = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "functions failed: {}", String::from_utf8_lossy(&out.stderr));
    let names: Vec<&str> = listing.lines().filter_map(|l| l.split('\t').nth(1)).collect();
    assert_eq!(names, ["_audit_read", "_main"], "only the two code entries:\n{listing}");

    for name in ["audit_global", "_audit_global"] {
        let (stdout, stderr, ok) = decompile("macho_datasyms_arm64", name);
        assert!(!ok, "{name} names data, not a function:\n{stdout}");
        assert!(
            stderr.contains("it names data at 0x100004000 in __data"),
            "the miss must say what {name} is:\n{stderr}"
        );
        assert!(!stdout.contains("UndefinedInstructionException"), "{stdout}");
    }

    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args([
            "decompile",
            &fixture("macho_datasyms_arm64"),
            "audit_message",
            "--json",
            "--sleighpath",
            &specs(),
        ])
        .output()
        .expect("run kuna decompile --json");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "audit_message names data:\n{stdout}");
    assert!(
        stdout.contains("it names data at 0x100004008 in __data") && stdout.contains("\"functions\": []"),
        "the JSON document must carry the same answer:\n{stdout}"
    );
}

/// (GH #1339) A Mach-O data symbol names the accesses an object of its width
/// makes: an `nlist` has no size, so the symbol extends to the next one (at most
/// 8 bytes), and a 32-bit increment of a global reads by name. The in-place
/// increment of a struct's first field covers part of the symbol, and its left
/// side is the store form, never a cast.
#[test]
fn a_macho_global_reads_by_name_at_its_width() {
    let (stdout, stderr, ok) = decompile("macho_datawidth_arm64", "_bump");
    assert!(ok, "decompile failed:\n{stderr}");
    assert!(stdout.contains("_counter += 1;"), "the 32-bit global reads by name:\n{stdout}");
    assert!(
        stdout.contains("*(int *)&_origin += 1;"),
        "the first field's increment is a store to part of _origin:\n{stdout}"
    );
    assert!(!stdout.contains("(int)_origin +="), "a cast is not an lvalue:\n{stdout}");
    assert!(!stdout.contains("dat_10000400"), "both globals are named:\n{stdout}");

    let (stdout, _, ok) = decompile("macho_datasyms_arm64", "_audit_read");
    assert!(ok && stdout.contains("_audit_global") && !stdout.contains("dat_100004000"), "{stdout}");
}
