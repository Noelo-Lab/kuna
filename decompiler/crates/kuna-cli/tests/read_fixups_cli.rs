//! `kuna read --json` over load-time fixup slots (GH #1379). The listed bytes
//! stay the load image's, which is what every other surface reads; each fixup
//! slot they overlap is reported in `fixups` with its kind, the symbol a bind
//! names, the file's own bytes and whether the listed bytes there are the
//! file's, the loader's value, or kuna's zero placeholder for an import.
//!
//! Fixtures: `macho_ptrslots_arm64` (two chained rebases and a bind at
//! 0x100004000, see `macho_ptrslots_arm64.provenance`) and
//! `dataptrs_so_x86_64.so` (see its `.provenance`). Integration tests require
//! the built processor specs under `specs/`.

use crate::common;

fn read(binary: &str, start: &str, bytes: &str, extra: &[&str]) -> serde_json::Value {
    let path = common::fixture(binary);
    let mut argv = vec!["read", &path, start, "--addr", "--bytes", bytes, "--json"];
    argv.extend_from_slice(extra);
    let (stdout, stderr, rc) = common::run_kuna(&argv);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    serde_json::from_str(&stdout).unwrap()
}

/// `(address, kind, symbol, target, file_bytes, image_bytes, returned)`.
type Row = (u64, String, Option<String>, Option<u64>, Option<String>, String, String);

fn fixups(doc: &serde_json::Value) -> Vec<Row> {
    let text = |v: &serde_json::Value| v.as_str().map(str::to_owned);
    doc["fixups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["address"].as_u64().unwrap(),
                text(&f["kind"]).unwrap(),
                text(&f["symbol"]),
                f["target"].as_u64(),
                text(&f["file_bytes"]),
                text(&f["image_bytes"]).unwrap(),
                text(&f["returned"]).unwrap(),
            )
        })
        .collect()
}

fn row(at: u64, kind: &str, symbol: Option<&str>, target: Option<u64>, file: &str, image: &str, returned: &str) -> Row {
    (
        at,
        kind.to_owned(),
        symbol.map(str::to_owned),
        target,
        Some(file.to_owned()),
        image.to_owned(),
        returned.to_owned(),
    )
}

/// The issue's question: a bind's zero is kuna's placeholder, not the file's
/// bytes (`0000000000000080`, the chain entry), and a rebase holds the pointer
/// the loader stores.
#[test]
fn a_chained_bind_is_a_placeholder_and_a_rebase_the_loaders_pointer() {
    let doc = read("macho_ptrslots_arm64", "0x100004000", "24", &[]);
    assert_eq!(doc["hex"], "4004000001000000c8030000010000000000000000000000", "the listed bytes are the load image's");
    assert_eq!(doc["fixups_coverage"], "chained-fixups");
    assert_eq!(
        fixups(&doc),
        [
            row(0x100004000, "rebase", None, Some(0x100000440), "4004000001001000", "4004000001000000", "applied"),
            row(0x100004008, "rebase", None, Some(0x1000003c8), "c803000001001000", "c803000001000000", "applied"),
            row(0x100004010, "bind", Some("_audit_imported"), None, "0000000000000080", "0000000000000000", "placeholder"),
        ]
    );
    let notes = doc["notes"].as_array().unwrap();
    assert!(notes.iter().any(|n| n.as_str().unwrap().contains("0x100004010 bind _audit_imported")), "{notes:?}");
}

/// A span that starts inside a slot lists the whole slot.
#[test]
fn a_slot_the_span_only_overlaps_is_listed_whole() {
    let doc = read("macho_ptrslots_arm64", "0x100004012", "4", &[]);
    assert_eq!(doc["hex"], "00000000");
    let rows = fixups(&doc);
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!((rows[0].0, rows[0].6.as_str()), (0x100004010, "placeholder"));
    assert_eq!(doc["fixups"][0]["size"], 8);
}

/// With the loader's fixups turned off the bytes are the file's, and say so.
#[test]
fn dynrelocs_off_lists_the_file_bytes_as_the_files() {
    let doc = read("macho_ptrslots_arm64", "0x100004010", "8", &["--option", "dynrelocs", "off"]);
    assert_eq!(doc["hex"], "0000000000000080");
    assert_eq!(
        fixups(&doc),
        [row(0x100004010, "bind", Some("_audit_imported"), None, "0000000000000080", "0000000000000080", "file")]
    );
}

/// An ELF's dynamic relocations: a `GLOB_DAT` to a symbol the object defines is
/// applied, an absolute relocation kuna does not apply is the file's bytes, and
/// a `RELATIVE` slot whose file word already holds the target is the file's.
#[test]
fn elf_dynamic_relocations_say_which_slots_kuna_applied() {
    let doc = read("dataptrs_so_x86_64.so", "0x3ff8", "72", &[]);
    assert_eq!(doc["fixups_coverage"], "dynamic-relocations");
    assert_eq!(
        fixups(&doc),
        [
            row(0x3ff8, "bind", Some("exported_tab"), Some(0x4030), "0000000000000000", "3040000000000000", "applied"),
            row(0x4020, "rebase", None, Some(0x2000), "0020000000000000", "0020000000000000", "file"),
            row(0x4030, "relocation", Some("audit_target"), None, "0000000000000000", "0000000000000000", "file"),
            row(0x4038, "rebase", None, Some(0x1010), "1010000000000000", "1010000000000000", "file"),
        ]
    );
}

/// Bytes no fixup touches carry an empty list and no note; an image whose
/// fixups kuna does not read says `none`.
#[test]
fn bytes_no_fixup_touches_list_none() {
    let code = read("macho_ptrslots_arm64", "0x1000003c8", "8", &[]);
    assert_eq!(code["fixups_coverage"], "chained-fixups");
    assert!(fixups(&code).is_empty());
    assert!(code["notes"].as_array().unwrap().is_empty(), "{}", code["notes"]);
    let pe = read("aifbracket_pe_i386.exe", "0x401000", "8", &[]);
    assert_eq!(pe["fixups_coverage"], "none");
    assert!(fixups(&pe).is_empty());
}
