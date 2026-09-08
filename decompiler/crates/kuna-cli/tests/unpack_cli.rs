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

/// The same original program as `packed_fixture`, repacked with `upx --lzma`;
/// see its `.provenance` sidecar.
fn lzma_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_lzma_x86_64")
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
/// re-headed as DEFLATE (method 15) with a repaired header checksum.
#[test]
fn an_unimplemented_method_is_refused_by_name() {
    let scratch = Scratch::new("deflate");
    let input = scratch.path("deflate.packed");
    reheaded_fixture(15, &input);

    let out = scratch.path("deflate.unpacked");
    let (_, stderr, ok) = run_kuna(&["unpack", &input, "-o", &out]);
    if is_unwired(&stderr) {
        eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
        return;
    }
    assert!(!ok, "a DEFLATE-compressed image must not be unpacked by this build");
    assert!(stderr.contains("DEFLATE"), "the refusal does not name the method:\n{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refused run must write nothing");
}

/// The other half of the same contract, now that method 14 *is* implemented: an
/// NRV block relabelled as LZMA must fail in the decoder rather than produce a
/// file. The `b_info` is given plausible LZMA property bytes first, so the range
/// coder really runs -- nothing about the block's sizes gives the lie away, and
/// this is exactly the case where a lenient decoder writes a plausible-looking
/// image of garbage.
#[test]
fn a_block_mislabelled_as_lzma_is_refused_not_guessed_at() {
    let scratch = Scratch::new("mislabelled");
    let mut bytes = std::fs::read(packed_fixture()).expect("fixture");
    let b_info = 0x100usize; // the first block, right after `l_info` + `p_info`
    bytes[b_info + 8] = 14; // b_method := M_LZMA
    bytes[b_info + 12] = 0x1a; // pb = 2
    bytes[b_info + 13] = 0x03; // lc = 3, lp = 0
    bytes[b_info + 14] = 0x00; // a range coder starts on a zero byte
    let input = scratch.path("mislabelled.packed");
    std::fs::write(&input, &bytes).expect("write the relabelled fixture");

    let out = scratch.path("mislabelled.unpacked");
    let (_, stderr, ok) = run_kuna(&["unpack", &input, "-o", &out]);
    if is_unwired(&stderr) {
        eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
        return;
    }
    assert!(!ok, "an NRV block relabelled as LZMA must not unpack");
    assert!(stderr.contains("block at 0x100"), "the refusal does not name the block:\n{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refused run must write nothing");
}

/// `docs/re-needs/upx-lzma-compression-blocks.md`: a `--lzma` image used to exit
/// 1 with "unsupported UPX image: compression method 14 (LZMA)". It is the same
/// original program as `packed_fixture`, so the recovered bytes are checkable
/// against the NRV witness's.
#[test]
fn unpack_recovers_an_lzma_packed_elf() {
    let scratch = Scratch::new("lzma");
    let out = scratch.path("snake.lzma.unpacked");
    let (stdout, stderr, ok) = run_kuna(&["unpack", &lzma_fixture(), "-o", &out, "--json"]);
    if !ok {
        if is_unwired(&stderr) {
            eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
            return;
        }
        panic!("kuna unpack failed on an LZMA image: {stderr}");
    }
    assert!(!stderr.contains("unsupported UPX image"), "still refused:\n{stderr}");
    for field in ["\"method\": 14", "\"method_name\": \"LZMA\"", "\"unpacked_size\": 31640"] {
        assert!(stdout.contains(field), "missing {field} in:\n{stdout}");
    }
    assert!(json_count(&stdout).is_some_and(|n| n > 0), "no blocks reported:\n{stdout}");

    let bytes = std::fs::read(&out).expect("unpacked file was written");
    assert_eq!(bytes.len(), 31640);
    assert_eq!(&bytes[..4], b"\x7fELF");
    // The NRV and LZMA witnesses pack the same program: the two recoveries must
    // agree byte for byte.
    let nrv_out = scratch.path("snake.nrv.unpacked");
    let (_, _, ok) = run_kuna(&["unpack", &packed_fixture(), "-o", &nrv_out]);
    assert!(ok, "the NRV witness stopped unpacking");
    assert_eq!(bytes, std::fs::read(&nrv_out).expect("read"), "the two packings disagree");
}

/// The PE64 whose payload is a real UPX LZMA1 stream and whose `PackHeader`
/// does not exist; built by its `.py` sidecar out of `lzma_fixture`'s first
/// block. `--raw-lzma` is the only thing that recovers it.
fn headerless_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/rawlzma_headerless_pe_x86_64.exe")
        .to_str()
        .unwrap()
        .to_string()
}

/// The stream's virtual extent in that fixture, and what it decodes to.
const HEADERLESS_RANGE: &str = "0x140002000:0x1400020ab";
const HEADERLESS_SIZE: usize = 792;

/// The acceptance probe of `docs/re-needs/headerless-lzma-payload-unpacked.md`,
/// against a vendored stand-in for its 1.4 MB dataset image: the range the
/// analyst read out of the decompiler, and a size nobody passed in.
#[test]
fn raw_lzma_recovers_a_stream_no_pack_header_describes() {
    let scratch = Scratch::new("rawlzma");
    let out = scratch.path("payload.bin");
    let (stdout, stderr, ok) = run_kuna(&[
        "unpack",
        &headerless_fixture(),
        "-o",
        &out,
        "--raw-lzma",
        HEADERLESS_RANGE,
        "--json",
    ]);
    if !ok {
        if is_unwired(&stderr) {
            eprintln!("unpack_cli: skipping (kuna unpack not wired into main.rs yet): {stderr}");
            return;
        }
        panic!("kuna unpack --raw-lzma failed: {stderr}");
    }

    assert!(!stderr.contains("no UPX PackHeader found"), "discovery ran anyway:\n{stderr}");
    assert!(stdout.contains(r#""unpacked_size": 792"#), "size is not a result:\n{stdout}");
    assert!(stdout.contains(r#""range_kind": "vma""#), "range was not read as an address:\n{stdout}");
    assert!(stdout.contains(r#""end_marker": false"#), "UPX writes no end marker:\n{stdout}");

    let bytes = std::fs::read(&out).expect("payload was written");
    assert_eq!(bytes.len(), HEADERLESS_SIZE);
    assert_eq!(&bytes[..4], b"\x7fELF");
}

/// The trap the override must not fall into: an image that genuinely is not
/// UPX-packed still gets the correct refusal, and still gets no file. Passing
/// the acceptance by making `unpack` exit 0 on everything would ship a
/// regression of a right answer.
#[test]
fn the_pack_header_search_is_not_degraded_by_the_override() {
    let scratch = Scratch::new("nodegrade");
    let out = scratch.path("payload.bin");
    let (_stdout, stderr, ok) = run_kuna(&["unpack", &headerless_fixture(), "-o", &out]);
    if is_unwired(&stderr) {
        return;
    }
    assert!(!ok, "a file with no PackHeader must not unpack:\n{stderr}");
    assert!(stderr.contains("no UPX PackHeader found"), "{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refusal wrote a file anyway");
}

/// The other two spellings an analyst reaches for: a file offset (the only
/// reading available when nothing maps the image) and explicit coder
/// parameters (a stream carrying no two-byte prefix). Both must land on the
/// same bytes as the address form.
#[test]
fn raw_lzma_accepts_file_offsets_and_explicit_properties() {
    let scratch = Scratch::new("rawspell");
    let out = scratch.path("payload.bin");
    // 0x200 is where the section's raw data starts; +2 skips the prefix that
    // --lzma-props then supplies by hand.
    let (stdout, stderr, ok) = run_kuna(&[
        "unpack",
        &headerless_fixture(),
        "-o",
        &out,
        "--raw-lzma",
        "0x202:0x2ab",
        "--raw-offsets",
        "--lzma-props",
        "2,0,3",
        "--json",
    ]);
    if is_unwired(&stderr) {
        return;
    }
    assert!(ok, "kuna unpack --raw-offsets failed: {stderr}");
    assert!(stdout.contains(r#""range_kind": "file-offset""#), "{stdout}");
    let bytes = std::fs::read(&out).expect("payload was written");
    assert_eq!(bytes.len(), HEADERLESS_SIZE);
    assert_eq!(&bytes[..4], b"\x7fELF");

    // The same two bytes given as the prefix pair rather than as three fields.
    let hex = scratch.path("payload.hex.bin");
    let (_o, stderr, ok) = run_kuna(&[
        "unpack",
        &headerless_fixture(),
        "-o",
        &hex,
        "--raw-lzma",
        "0x202+169",
        "--raw-offsets",
        "--lzma-props",
        "0x1a03",
    ]);
    assert!(ok, "the prefix-pair spelling failed: {stderr}");
    assert_eq!(std::fs::read(&hex).unwrap(), bytes);
}

/// A range that is not a stream, and a range outside the image: refused by
/// name, and no file written. The `--raw-max-size` cap is what stops a decode
/// that would otherwise run until memory does.
#[test]
fn a_range_that_is_not_a_stream_is_refused_not_half_written() {
    let scratch = Scratch::new("rawbad");
    let out = scratch.path("payload.bin");
    let (_o, stderr, ok) =
        run_kuna(&["unpack", &headerless_fixture(), "-o", &out, "--raw-lzma", "0x140009000:0x14000a000"]);
    if is_unwired(&stderr) {
        return;
    }
    assert!(!ok, "an unmapped range must not decode");
    assert!(stderr.contains("not inside any section"), "{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refusal wrote a file anyway");

    let (_o, stderr, ok) = run_kuna(&[
        "unpack",
        &headerless_fixture(),
        "-o",
        &out,
        "--raw-lzma",
        HEADERLESS_RANGE,
        "--raw-max-size",
        "128",
    ]);
    assert!(!ok, "the cap must bound a length-less decode");
    assert!(stderr.contains("output cap"), "{stderr}");
    assert!(!PathBuf::from(&out).exists(), "a refusal wrote a file anyway");
}

/// The raw-mode modifiers are inert on their own, so they are a usage error
/// rather than a silent fall-back to a PackHeader search.
#[test]
fn a_raw_modifier_without_a_range_is_a_usage_error() {
    let (_o, stderr, ok) = run_kuna(&["unpack", &plain_fixture(), "--lzma-props", "2,0,3"]);
    if is_unwired(&stderr) {
        return;
    }
    assert!(!ok);
    assert!(stderr.contains("only applies with --raw-lzma"), "{stderr}");
}

/// Write the NRV witness out with its `b_method` overwritten and its header
/// checksum repaired, so the walk gets as far as the block decode.
fn reheaded_fixture(method: u8, to: &str) {
    let mut bytes = std::fs::read(packed_fixture()).expect("fixture");
    let ph = 0x2c40usize;
    bytes[ph + 6] = method;
    let sum: u32 = bytes[ph + 4..ph + 31].iter().map(|b| u32::from(*b)).sum();
    bytes[ph + 31] = (sum % 251) as u8;
    std::fs::write(to, &bytes).expect("write the re-headed fixture");
}

/// Parse the `"count": N` field out of a `--json` document.
fn json_count(stdout: &str) -> Option<usize> {
    let i = stdout.find("\"count\":")? + "\"count\":".len();
    stdout[i..].trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
}
