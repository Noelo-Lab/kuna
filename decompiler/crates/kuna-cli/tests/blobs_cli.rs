//! Gate for `kuna blobs`, the embedded-file inventory, driven through the built
//! binary on the `embedded_blobs_x86_64.so` fixture: a PNG, a GIF, a JPEG, a zip
//! and a gzip in `.rodata`, each returned by one function, and a PNG cut before
//! its `IEND` chunk.
//!
//! The reference walk and the decompile need the built x86 spec. `--no-xrefs`
//! needs no spec.

use std::path::PathBuf;
use std::process::Command;

fn fixture() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/embedded_blobs_x86_64.so")
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

fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| l.split('\t').map(str::to_string).collect())
        .collect()
}

#[test]
fn every_whole_file_is_listed_with_its_length_and_the_truncated_one_is_not() {
    let (out, err, code) = run_kuna(&["blobs", &fixture(), "--no-xrefs"]);
    assert_eq!(code, 0, "{err}");
    let found: Vec<Vec<String>> = rows(&out).into_iter().map(|r| r[..4].to_vec()).collect();
    let want = [
        ["0x3c0", "png", "69", ".rodata"],
        ["0x410", "gif", "47", ".rodata"],
        ["0x440", "jpeg", "331", ".rodata"],
        ["0x590", "zip", "297", ".rodata"],
        ["0x6c0", "gzip", "86", ".rodata"],
    ];
    assert_eq!(found, want, "{out}");
    assert!(out.starts_with("# 5 embedded blobs in "), "{out}");
}

#[test]
fn each_row_names_the_function_that_references_it() {
    let (out, err, code) = run_kuna(&["blobs", &fixture()]);
    assert_eq!(code, 0, "{err}");
    let owners: Vec<(String, String)> = rows(&out).into_iter().map(|r| (r[1].clone(), r[5].clone())).collect();
    let want = [("png", "get_logo"), ("gif", "get_icon"), ("jpeg", "get_photo"), ("zip", "get_archive"), ("gzip", "get_packed")];
    assert_eq!(owners.iter().map(|(k, f)| (k.as_str(), f.as_str())).collect::<Vec<_>>(), want, "{out}");
}

#[test]
fn extract_writes_each_file_byte_for_byte() {
    let dir = std::env::temp_dir().join(format!("kuna-blobs-{}", std::process::id()));
    let (out, err, code) = run_kuna(&["blobs", &fixture(), "--no-xrefs", "--kind", "png,gzip", "--extract", dir.to_str().unwrap()]);
    assert_eq!(code, 0, "{err}");
    let image = std::fs::read(fixture()).unwrap();
    let png = std::fs::read(dir.join("png_3c0.png")).unwrap();
    let gzip = std::fs::read(dir.join("gzip_6c0.gz")).unwrap();
    assert_eq!(png, &image[0x3c0..0x3c0 + 69]);
    assert_eq!(gzip, &image[0x6c0..0x6c0 + 86]);
    assert!(png.ends_with(b"IEND\xae\x42\x60\x82"));
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2, "{out}");
    assert!(rows(&out).iter().all(|r| r[6].ends_with(".png") || r[6].ends_with(".gz")), "{out}");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_json_document_and_the_filters() {
    let (out, err, code) = run_kuna(&["blobs", &fixture(), "--json", "--no-xrefs", "--kind", "zip"]);
    assert_eq!(code, 0, "{err}");
    for key in ["\"scanned\": \"sections\"", "\"count\": 1", "\"kind\": \"zip\"", "\"length\": 297", "\"address_hex\": \"0x590\"", "\"file\": null", "\"xrefs\": false"] {
        assert!(out.contains(key), "missing {key}:\n{out}");
    }
    let (out, _, code) = run_kuna(&["blobs", &fixture(), "--no-xrefs", "--section", "text"]);
    assert_eq!(code, 0);
    assert!(out.starts_with("# 0 embedded blobs in "), "{out}");
    let (_, err, code) = run_kuna(&["blobs", &fixture(), "--no-xrefs", "--section", ".nope"]);
    assert_eq!(code, 1);
    assert!(err.contains("no section named"), "{err}");
    let (_, err, code) = run_kuna(&["blobs", &fixture(), "--kind", "tiff"]);
    assert_eq!(code, 2);
    assert!(err.contains("unknown --kind"), "{err}");
    let (_, _, code) = run_kuna(&["blobs"]);
    assert_eq!(code, 2);
}

#[test]
fn decompile_returns_each_file_by_name_unless_the_option_is_off() {
    let (out, err, code) = run_kuna(&["decompile", &fixture(), "get_logo"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("unsigned char * get_logo(") && out.contains("return png_3c0;"), "{out}");
    let (out, err, code) = run_kuna(&["decompile", &fixture(), "get_logo", "--option", "blobs", "off"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("return 0x3c0;"), "{out}");
    let (out, err, code) = run_kuna(&["decompile", &fixture(), "get_broken"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("return 0x720;"), "{out}");
}
