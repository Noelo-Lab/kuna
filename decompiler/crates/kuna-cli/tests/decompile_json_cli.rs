//! CLI end-to-end gate for the two agent-facing report contracts:
//!
//!  * `kuna decompile --json` answers in `decompile-all --json`'s record shape —
//!    the SAME shape, one element — so a caller parses one document whichever
//!    command it reached for. The byte-identity assertion below is what keeps
//!    that true: two emitters would drift, one cannot.
//!  * a run that discovers ZERO functions FAILS. It used to answer `count: 0`,
//!    exit 0 and a silent stderr, which is indistinguishable from a file that
//!    genuinely has no functions. The line is executable content, so a data-only
//!    object keeps its honest empty answer and its exit 0.
//!
//! ## `.sla` precondition
//!
//! Every test here bootstraps a real architecture, which needs the built `x86`
//! `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! command fails to build an architecture; the test prints that and returns
//! early — a specs-less CI is a visible skip, never a false green.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_ID: AtomicU64 = AtomicU64::new(0);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// A checked-in x86-64 ELF with real code and real symbols — the fixture both
/// acceptance probes target.
fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// Run the built `kuna` binary, returning `(stdout, stderr, exit code)`.
fn run_kuna(args: &[&str]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code(),
    )
}

/// `true` when the failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains("Could not find .sla")
}

/// The string value of a top-level `"key": "..."` / `"key": null` field, without
/// a JSON dependency (the documents are the CLI's own controlled output).
///
/// The value is JSON-escaped, so the terminator is the first quote that is not
/// itself escaped — the reported reason quotes the selector it could not find.
fn json_field<'a>(doc: &'a str, key: &str) -> Option<&'a str> {
    let at = doc.find(&format!("\"{key}\":"))? + key.len() + 3;
    let rest = doc[at..].trim_start();
    if rest.starts_with("null") {
        return None;
    }
    let body = rest.strip_prefix('"')?;
    let mut escaped = false;
    for (i, c) in body.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '"' => return Some(&body[..i]),
            _ => {}
        }
    }
    None
}

/// Write `bytes` to a private scratch file and return its path.
fn temp_image(tag: &str, bytes: &[u8]) -> PathBuf {
    let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "kuna_json_cli_{tag}_{}_{id}.bin",
        std::process::id()
    ));
    std::fs::write(&path, bytes).expect("write the scratch image");
    path
}

/// A minimal ELF64 executable: ONE `PF_X` `PT_LOAD` and no section table, whose
/// mapped bytes hold no discoverable function. It is the section-header-stripped
/// PIE shape of the dataset witness reduced to the part the contract turns on —
/// an image that carries code and yields nothing.
fn stripped_executable(payload: &[u8]) -> Vec<u8> {
    const EHDR: usize = 64;
    const PHDR: usize = 56;
    let mut out = vec![0u8; EHDR + PHDR + payload.len()];
    out[..4].copy_from_slice(b"\x7fELF");
    out[4] = 2; // ELFCLASS64
    out[5] = 1; // ELFDATA2LSB
    out[6] = 1; // EV_CURRENT
    out[16..18].copy_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
    out[18..20].copy_from_slice(&62u16.to_le_bytes()); // e_machine = EM_X86_64
    out[20..24].copy_from_slice(&1u32.to_le_bytes());
    out[32..40].copy_from_slice(&(EHDR as u64).to_le_bytes()); // e_phoff
    out[52..54].copy_from_slice(&(EHDR as u16).to_le_bytes()); // e_ehsize
    out[54..56].copy_from_slice(&(PHDR as u16).to_le_bytes()); // e_phentsize
    out[56..58].copy_from_slice(&1u16.to_le_bytes()); // e_phnum
    let total = (EHDR + PHDR + payload.len()) as u64;
    let p = EHDR;
    out[p..p + 4].copy_from_slice(&1u32.to_le_bytes()); // p_type = PT_LOAD
    out[p + 4..p + 8].copy_from_slice(&5u32.to_le_bytes()); // p_flags = PF_R|PF_X
    out[p + 16..p + 24].copy_from_slice(&0x40_0000u64.to_le_bytes()); // p_vaddr
    out[p + 32..p + 40].copy_from_slice(&total.to_le_bytes()); // p_filesz
    out[p + 40..p + 48].copy_from_slice(&total.to_le_bytes()); // p_memsz
    out[p + 48..p + 56].copy_from_slice(&0x1000u64.to_le_bytes()); // p_align
    out[EHDR + PHDR..].copy_from_slice(payload);
    out
}

/// A minimal ET_REL ELF64 whose only allocated section is `.data`: an object
/// that legitimately holds no functions at all.
fn data_only_object() -> Vec<u8> {
    const EHDR: usize = 64;
    const SHDR: usize = 64;
    let names: &[u8] = b"\0.data\0.shstrtab\0";
    let shoff = EHDR;
    let names_off = shoff + 3 * SHDR;
    let data_off = names_off + names.len();
    let mut out = vec![0u8; data_off + 4];
    out[..4].copy_from_slice(b"\x7fELF");
    out[4] = 2;
    out[5] = 1;
    out[6] = 1;
    out[16..18].copy_from_slice(&1u16.to_le_bytes()); // e_type = ET_REL
    out[18..20].copy_from_slice(&62u16.to_le_bytes());
    out[20..24].copy_from_slice(&1u32.to_le_bytes());
    out[40..48].copy_from_slice(&(shoff as u64).to_le_bytes()); // e_shoff
    out[52..54].copy_from_slice(&(EHDR as u16).to_le_bytes());
    out[58..60].copy_from_slice(&(SHDR as u16).to_le_bytes()); // e_shentsize
    out[60..62].copy_from_slice(&3u16.to_le_bytes()); // e_shnum
    out[62..64].copy_from_slice(&2u16.to_le_bytes()); // e_shstrndx
    out[names_off..data_off].copy_from_slice(names);

    fn shdr(out: &mut [u8], at: usize, name: u32, kind: u32, flags: u64, off: u64, size: u64) {
        out[at..at + 4].copy_from_slice(&name.to_le_bytes());
        out[at + 4..at + 8].copy_from_slice(&kind.to_le_bytes());
        out[at + 8..at + 16].copy_from_slice(&flags.to_le_bytes());
        out[at + 24..at + 32].copy_from_slice(&off.to_le_bytes());
        out[at + 32..at + 40].copy_from_slice(&size.to_le_bytes());
    }
    // `.data`: SHT_PROGBITS, SHF_ALLOC|SHF_WRITE — allocated, never executed.
    shdr(&mut out, shoff + SHDR, 1, 1, 0x3, data_off as u64, 4);
    shdr(&mut out, shoff + 2 * SHDR, 7, 3, 0, names_off as u64, names.len() as u64);
    out
}

// --- `kuna decompile --json` -------------------------------------------------

/// The acceptance probe of the `decompile-no-json` need, verbatim: the
/// single-function call answers exit 0 with the `decompile-all` record shape.
#[test]
fn decompile_json_emits_the_record_shape_for_one_function() {
    let (stdout, stderr, code) =
        run_kuna(&["decompile", &fixture(), "_DT_INIT", "--json", "--sleighpath", &specs()]);
    if code != Some(0) {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_json_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna decompile --json failed: {stderr}");
    }
    assert!(stdout.trim_start().starts_with('{'), "not a JSON object:\n{stdout}");
    assert!(stdout.contains("\"functions\": ["), "{stdout}");
    assert!(stdout.contains("\"name\": \"_DT_INIT\""), "{stdout}");
    assert!(stdout.contains("\"code\": \""), "the record must carry code:\n{stdout}");
    // The fields the text surface could never report: they are the reason the
    // flag exists, so their absence is a regression even with valid JSON.
    for field in ["\"address\":", "\"address_hex\":", "\"size\":", "\"line_mappings\":", "\"variables\":"] {
        assert!(stdout.contains(field), "missing {field} in:\n{stdout}");
    }
    assert!(stdout.contains("\"error\": null"), "a healthy run reports no error:\n{stdout}");
}

/// One shape, one emitter: `decompile --json <f>` and
/// `decompile-all --json --functions <f>` must produce the SAME BYTES. This is
/// the assertion that makes "same schema" enforceable rather than aspirational —
/// a second serializer would fail it on its first field.
#[test]
fn decompile_json_is_byte_identical_to_decompile_all_for_one_function() {
    let bin = fixture();
    let sp = specs();
    let (one, stderr, code) =
        run_kuna(&["decompile", &bin, "_DT_INIT", "--json", "--sleighpath", &sp]);
    if code != Some(0) {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
            return;
        }
        panic!("kuna decompile --json failed: {stderr}");
    }
    let (all, stderr, code) = run_kuna(&[
        "decompile-all",
        &bin,
        "--functions",
        "_DT_INIT",
        "--json",
        "--sleighpath",
        &sp,
    ]);
    assert_eq!(code, Some(0), "kuna decompile-all --json failed: {stderr}");
    assert_eq!(one, all, "the two surfaces must emit one document, byte for byte");
}

/// A `0x`-prefixed selector and an explicit `--addr` reach the same record, so
/// `--json` is not a flag that works for one selector kind and not the others.
#[test]
fn decompile_json_selects_by_address_too() {
    let bin = fixture();
    let sp = specs();
    let (auto, stderr, code) = run_kuna(&["decompile", &bin, "0x1000", "--json", "--sleighpath", &sp]);
    if code != Some(0) {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
            return;
        }
        panic!("kuna decompile 0x1000 --json failed: {stderr}");
    }
    assert!(auto.contains("\"address_hex\": \"0x1000\""), "{auto}");
    let (explicit, stderr, code) =
        run_kuna(&["decompile", &bin, "1000", "--addr", "--json", "--sleighpath", &sp]);
    assert_eq!(code, Some(0), "kuna decompile --addr --json failed: {stderr}");
    assert_eq!(auto, explicit, "the two ways of naming an address must agree");
}

/// The `decomp_dbg`-only flags are REFUSED, not ignored: the `--json` path loads
/// in-process and produces no console transcript, and silently dropping half a
/// request is the failure mode this flag exists to end.
#[test]
fn json_refuses_the_flags_it_cannot_honor() {
    let bin = fixture();
    for extra in [
        vec!["--raw"],
        vec!["--regions"],
        vec!["--kassert", "noop"],
        vec!["--decomp-dbg", "/nonexistent/decomp_dbg"],
    ] {
        let mut argv = vec!["decompile", bin.as_str(), "_DT_INIT", "--json"];
        argv.extend_from_slice(&extra);
        let (stdout, stderr, code) = run_kuna(&argv);
        assert_eq!(code, Some(2), "{extra:?} must be refused, got: {stderr}");
        assert!(stderr.contains("not supported with --json"), "{stderr}");
        assert!(stdout.is_empty(), "a refused flag does no work: {stdout}");
    }
}

/// A selector that resolves to nothing is a failed run that still answers in the
/// record shape: exit 1, the reason on stderr AND in the document's run-level
/// `error`, so a caller never has to reconcile empty stdout with prose stderr.
#[test]
fn an_unresolved_selector_answers_with_an_error_envelope() {
    let (stdout, stderr, code) =
        run_kuna(&["decompile", &fixture(), "nosuchfunction", "--json", "--sleighpath", &specs()]);
    if is_specs_skip(&stderr) {
        eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert_eq!(code, Some(1), "an unresolved selector is not a success: {stderr}");
    assert!(stdout.trim_start().starts_with('{'), "still JSON:\n{stdout}");
    assert!(stdout.contains("\"count\": 0"), "{stdout}");
    let error = json_field(&stdout, "error").expect("the envelope carries the reason");
    assert!(error.contains("nosuchfunction"), "{error}");
    assert!(stderr.contains("nosuchfunction"), "{stderr}");
}

// --- total discovery failure -------------------------------------------------

/// The `zero-functions-exit-0` contract on all three whole-binary surfaces: an
/// image that carries code and yields no function is a FAILED run — non-zero
/// exit, `(?i)no functions` on stderr — not an empty success.
#[test]
fn a_total_discovery_failure_is_reported_as_a_failure() {
    let image = temp_image("nofuncs", &stripped_executable(b"\x55\x48\x89\xe5\x5d\xc3"));
    let path = image.to_str().unwrap().to_string();
    let sp = specs();
    for surface in [
        vec!["functions", path.as_str(), "--json"],
        vec!["functions", path.as_str()],
        vec!["decompile-all", path.as_str(), "--json"],
        vec!["decompile-all", path.as_str()],
    ] {
        let mut argv = surface.clone();
        argv.extend_from_slice(&["--sleighpath", sp.as_str()]);
        let (stdout, stderr, code) = run_kuna(&argv);
        if !stderr.to_lowercase().contains("no functions") && is_specs_skip(&stderr) {
            eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
            let _ = std::fs::remove_file(&image);
            return;
        }
        assert_ne!(code, Some(0), "{surface:?} must not report success: {stderr}");
        assert!(
            stderr.to_lowercase().contains("no functions"),
            "{surface:?} must say why on stderr, got: {stderr}"
        );
        if surface.contains(&"--json") {
            assert!(
                json_field(&stdout, "error").is_some_and(|e| e.contains("no functions")),
                "{surface:?} must carry the reason in the document too:\n{stdout}"
            );
        }
    }
    let _ = std::fs::remove_file(&image);
}

/// The cause an agent can act on is named: a packed image says so, and points at
/// the command that fixes it, rather than leaving the caller to conclude the
/// binary has no code.
#[test]
fn a_packed_image_is_named_as_packed() {
    let image = temp_image("upx", &stripped_executable(b"UPX!\x00\x00\x00\x00"));
    let (stdout, stderr, code) =
        run_kuna(&["functions", image.to_str().unwrap(), "--json", "--sleighpath", &specs()]);
    let _ = std::fs::remove_file(&image);
    if !stderr.to_lowercase().contains("no functions") && is_specs_skip(&stderr) {
        eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert_ne!(code, Some(0), "{stderr}");
    assert!(stderr.contains("UPX-packed"), "the packer must be named: {stderr}");
    assert!(stderr.contains("kuna unpack"), "the recovery must be named: {stderr}");
    let error = json_field(&stdout, "error").expect("the document carries it too");
    assert!(error.contains("UPX-packed") && error.contains("kuna unpack"), "{error}");
}

/// The legitimate empty case, which the loud failure must NOT break: an image
/// with no executable content has no functions to find, so its empty inventory
/// stays a success.
#[test]
fn an_image_with_no_code_keeps_its_empty_success() {
    let image = temp_image("dataonly", &data_only_object());
    let (stdout, stderr, code) =
        run_kuna(&["functions", image.to_str().unwrap(), "--json", "--sleighpath", &specs()]);
    let _ = std::fs::remove_file(&image);
    if code != Some(0) && is_specs_skip(&stderr) {
        eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert_eq!(code, Some(0), "a data-only object is not a failed run: {stderr}");
    assert!(stdout.contains("\"count\": 0"), "{stdout}");
    assert!(stdout.contains("\"error\": null"), "{stdout}");
    assert!(!stderr.to_lowercase().contains("no functions"), "{stderr}");
}

/// A run that finds functions is untouched: exit 0, a null run-level error, and
/// the inventory it always had.
#[test]
fn a_run_that_finds_functions_still_exits_zero() {
    let (stdout, stderr, code) = run_kuna(&["functions", &fixture(), "--json", "--sleighpath", &specs()]);
    if code != Some(0) {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_json_cli: skipping (no `.sla`): {stderr}");
            return;
        }
        panic!("kuna functions failed: {stderr}");
    }
    assert!(!stdout.contains("\"count\": 0"), "the fixture has functions:\n{stdout}");
    assert!(stdout.contains("\"error\": null"), "{stdout}");
    assert!(stdout.contains("\"name\": \"_DT_INIT\""), "{stdout}");
    assert!(stderr.is_empty(), "a healthy run says nothing: {stderr}");
}
