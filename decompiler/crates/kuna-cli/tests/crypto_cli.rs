//! Gate for `kuna crypto` — the crypto-constant inventory
//! (`docs/re-needs/crypto-constant-finder.md`), driven through the built binary
//! on the vendored `cryptoconst_x86_64` fixture: an AES S-box, SHA-256's K and a
//! custom Base64 alphabet in `.rodata`, each read by one named function, plus
//! the TEA delta and the CRC-32 polynomial as `.text` immediates.
//!
//! The reference walk needs the built x86 `.sla`; without it the attributed
//! cases print a skip and return. The `--no-xrefs` cases need no `.sla`.

use std::path::PathBuf;
use std::process::Command;

fn fixture() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/cryptoconst_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

fn run_kuna(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna")).args(args).output().expect("spawn kuna");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| l.split('\t').map(str::to_string).collect())
        .collect()
}

#[test]
fn the_scan_finds_every_shape_without_the_reference_walk() {
    let (out, err, code) = run_kuna(&["crypto", &fixture(), "--no-xrefs"]);
    assert_eq!(code, 0, "{err}");
    let rows = rows(&out);
    let at = |addr: &str| rows.iter().find(|r| r[0] == addr).unwrap_or_else(|| panic!("no row at {addr}:\n{out}"));
    assert_eq!(&at("0x402180")[1..6], ["AES", "S-box", "table", "u8", "256"]);
    assert_eq!(&at("0x402080")[1..6], ["SHA-256", "round constants K", "table", "u32le", "256"]);
    assert_eq!(at("0x402020")[1], "Base64");
    assert!(at("0x402020")[2].starts_with("alphabet (custom) ZYXW"));
    assert_eq!(at("0x4011f8")[1..4], ["CRC-32", "polynomial 0xEDB88320 (reflected)", "immediate"]);
    assert!(rows.iter().any(|r| r[1].starts_with("TEA") && r[3] == "immediate" && r[6] == ".text"));
    assert!(out.starts_with("# 6 crypto constants in "), "{out}");
}

#[test]
fn each_row_names_the_function_that_uses_it() {
    let (out, err, code) = run_kuna(&["crypto", &fixture()]);
    if code != 0 && is_specs_skip(&err) {
        eprintln!("skipping: {err}");
        return;
    }
    assert_eq!(code, 0, "{err}");
    let rows = rows(&out);
    let owner = |addr: &str| rows.iter().find(|r| r[0] == addr).map(|r| r[8].clone()).unwrap_or_default();
    assert_eq!(owner("0x402180"), "sub_bytes");
    assert_eq!(owner("0x402080"), "sha256_round");
    assert_eq!(owner("0x402020"), "b64_char");
    assert_eq!(owner("0x4011f8"), "crc32_update");
    assert!(rows.iter().filter(|r| r[1].starts_with("TEA")).all(|r| r[8] == "tea_encrypt"), "{out}");
}

#[test]
fn the_json_document_and_the_filters() {
    let (out, err, code) = run_kuna(&["crypto", &fixture(), "--json", "--no-xrefs", "--algorithm", "sha"]);
    assert_eq!(code, 0, "{err}");
    for key in ["\"scanned\": \"sections\"", "\"count\": 1", "\"algorithm\": \"SHA-256\"", "\"layout\": \"u32le\"", "\"byte_length\": 256", "\"xrefs\": false"] {
        assert!(out.contains(key), "missing {key}:\n{out}");
    }
    let (out, _, code) = run_kuna(&["crypto", &fixture(), "--no-xrefs", "--section", "text"]);
    assert_eq!(code, 0);
    assert!(rows(&out).iter().all(|r| r[6] == ".text" && r[3] == "immediate"), "{out}");
    let (_, err, code) = run_kuna(&["crypto", &fixture(), "--no-xrefs", "--section", ".nope"]);
    assert_eq!(code, 1);
    assert!(err.contains("no section named"), "{err}");
    let (_, _, code) = run_kuna(&["crypto"]);
    assert_eq!(code, 2);
}
