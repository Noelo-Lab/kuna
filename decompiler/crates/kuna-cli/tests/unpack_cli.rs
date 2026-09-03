//! CLI end-to-end gate for `kuna unpack` -- drives the built `kuna` binary over
//! the vendored UPX-packed ELF and asserts both halves of the feature: that the
//! recovered file is a real, analyzable ELF, and that an image this build cannot
//! unpack is refused by name instead of half-written.
//!
//! ## Dispatch precondition
//!
//! `kuna-cli/src/main.rs` is owned by the integrator, so `kuna unpack` may not
//! be wired into the dispatch table yet when this test runs. An unwired build
//! answers `unknown subcommand` on stderr; that is a visible skip, never a false
//! green -- a *wired* build that unpacks wrongly still fails loudly here. The
//! unpacker's own end-to-end coverage does not depend on the wiring: it lives in
//! `kuna_analysis::upx`'s unit tests, which run against the same fixture.
//!
//! ## `.sla` precondition
//!
//! The "the output is analyzable" assertion bootstraps an architecture and so
//! needs the built `x86` `.sla` under `specs/` (gitignored; `make specs`). When
//! it is absent that one assertion prints why and returns, like
//! `decompile_all_cli.rs`.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored UPX 3.96 witness; see its `.provenance` sidecar.
fn packed_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

/// An ordinary, unpacked ELF -- the "this is not a packed file" case.
fn plain_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

fn run_kuna(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// `true` when the run failed only because the integrator has not wired the
/// subcommand into `main.rs` yet.
fn is_unwired(stderr: &str) -> bool {
    stderr.contains("unknown subcommand")
}

/// `true` when the failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

/// A per-test scratch directory that removes itself, so a `-o` target never
/// lands next to the fixture.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("kuna-unpack-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        Scratch(dir)
    }

    fn path(&self, name: &str) -> String {
        self.0.join(name).to_str().unwrap().to_string()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The acceptance probe of `docs/re-needs/unpack-upx-packed-executable.md`:
/// `kuna unpack <packed>` exits 0. Everything else here is the real bar behind
/// that -- that what it wrote is the original program.
#[test]
fn unpack_recovers_the_original_elf() {
    let scratch = Scratch::new("recover");
    let out = scratch.path("snake.unpacked");
    let (stdout, stderr, ok) = run_kuna(&["unpack", &packed_fixture(), "-o", &out]);
    if !ok {
        if is_unwired(&stderr) {
            eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
            return;
        }
        panic!("kuna unpack failed: {stderr}");
    }

    assert!(stdout.contains("NRV2B_LE32"), "report names no method:\n{stdout}");
    assert!(stdout.contains("linux/amd64"), "report names no format:\n{stdout}");
    assert!(stdout.contains(&out), "report names no output path:\n{stdout}");

    let bytes = std::fs::read(&out).expect("unpacked file was written");
    assert_eq!(bytes.len(), 31640, "unpacked size");
    assert_eq!(&bytes[..4], b"\x7fELF");
    // A section header table the packed original does not have: `e_shnum`.
    assert_eq!(u16::from_le_bytes([bytes[60], bytes[61]]), 31);
}

/// The point of the feature: what comes out is something kuna can then analyze.
/// The packed input reports zero functions; the recovered one reports the real
/// program, `main` included.
#[test]
fn the_unpacked_output_is_analyzable() {
    let scratch = Scratch::new("analyze");
    let out = scratch.path("snake.unpacked");
    let (_, stderr, ok) = run_kuna(&["unpack", &packed_fixture(), "-o", &out]);
    if !ok {
        if is_unwired(&stderr) {
            eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
            return;
        }
        panic!("kuna unpack failed: {stderr}");
    }

    let (stdout, stderr, ok) =
        run_kuna(&["functions", &out, "--json", "--sleighpath", &specs()]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("unpack_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna functions failed on the unpacked output: {stderr}");
    }
    let count = json_count(&stdout).expect("functions --json has a count");
    assert!(count >= 50, "expected the recovered program's functions, got {count}:\n{stdout}");
    for name in ["main", "gameplay", "chacha20_xor"] {
        assert!(stdout.contains(&format!("\"{name}\"")), "no {name} in:\n{stdout}");
    }
}

#[test]
fn unpack_json_reports_the_pack_header() {
    let scratch = Scratch::new("json");
    let out = scratch.path("snake.unpacked");
    let (stdout, stderr, ok) = run_kuna(&["unpack", &packed_fixture(), "-o", &out, "--json"]);
    if !ok {
        if is_unwired(&stderr) {
            eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
            return;
        }
        panic!("kuna unpack --json failed: {stderr}");
    }
    assert!(stdout.trim_start().starts_with('{'), "output is not a JSON object:\n{stdout}");
    for field in [
        "\"packer\": \"upx\"",
        "\"format\": 22",
        "\"format_name\": \"linux/amd64\"",
        "\"method\": 2",
        "\"method_name\": \"NRV2B_LE32\"",
        "\"packed_size\": 11364",
        "\"unpacked_size\": 31640",
        "\"count\": 9",
        "\"pack_header_offset_hex\": \"0x2c40\"",
    ] {
        assert!(stdout.contains(field), "missing {field} in:\n{stdout}");
    }
}

/// An unpacked binary is not an error in the file, but it is an error in the
/// request -- and it says so rather than writing an empty file.
#[test]
fn an_unpacked_binary_is_refused_by_name() {
    let scratch = Scratch::new("plain");
    let out = scratch.path("plain.unpacked");
    let (_, stderr, ok) = run_kuna(&["unpack", &plain_fixture(), "-o", &out]);
    if is_unwired(&stderr) {
        eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
        return;
    }
    assert!(!ok, "unpacking a plain ELF should fail");
    assert!(stderr.contains("no UPX PackHeader"), "unhelpful diagnostic:\n{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refused run must write nothing");
}

/// A method this build does not implement must be named, not guessed at: a
/// wrong unpacked binary is far worse than an honest refusal. The fixture is
/// re-headed as LZMA (method 14) with a repaired header checksum.
#[test]
fn an_unimplemented_method_is_refused_by_name() {
    let scratch = Scratch::new("lzma");
    let mut bytes = std::fs::read(packed_fixture()).expect("fixture");
    let ph = 0x2c40usize;
    bytes[ph + 6] = 14; // M_LZMA
    let sum: u32 = bytes[ph + 4..ph + 31].iter().map(|b| u32::from(*b)).sum();
    bytes[ph + 31] = (sum % 251) as u8;
    let input = scratch.path("lzma.packed");
    std::fs::write(&input, &bytes).expect("write the re-headed fixture");

    let out = scratch.path("lzma.unpacked");
    let (_, stderr, ok) = run_kuna(&["unpack", &input, "-o", &out]);
    if is_unwired(&stderr) {
        eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
        return;
    }
    assert!(!ok, "an LZMA-compressed image must not be unpacked by this build");
    assert!(stderr.contains("LZMA"), "the refusal does not name the method:\n{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refused run must write nothing");
}

/// Parse the `"count": N` field out of a `--json` document.
fn json_count(stdout: &str) -> Option<usize> {
    let i = stdout.find("\"count\":")? + "\"count\":".len();
    stdout[i..].trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
}
