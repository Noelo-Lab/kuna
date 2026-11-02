//! (kuna, issue #1438) A `--define-function` the input does not wholly map is
//! kept as the assertion it is, and said to be on every surface: one stderr
//! `warning:`, a JSON `notes` entry, and a `declared` marker on the entry's
//! record. Before, `functions --json` listed it exactly like a discovered
//! entry, with no warning, and the first command that needed its bytes
//! refused it.

use crate::common;

const UNMAPPED: &str = "0x999999999-0x9999999a1=asserted_unmapped";
const UNMAPPED_WARNING: &str = "warning: declared function asserted_unmapped at \
     0x999999999-0x9999999a1: its entry 0x999999999 is not mapped in this input";

fn json(stdout: &str) -> serde_json::Value {
    serde_json::from_str(stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

fn record<'a>(doc: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    doc["functions"]
        .as_array()
        .expect("functions array")
        .iter()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("no {name} record: {doc}"))
}

fn warnings(stderr: &str) -> usize {
    stderr.matches(UNMAPPED_WARNING).count()
}

#[test]
fn an_unmapped_declaration_is_listed_marked_and_warned_once() {
    for fixture in ["protoorder_narrowvote_x86_64", "macho_imports_arm64", "pe_addrtaken_x86_64.exe"] {
        let bin = common::fixture(fixture);
        let (stdout, stderr, code) =
            common::run_kuna(&["functions", &bin, "--json", "--define-function", UNMAPPED]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        assert_eq!(warnings(&stderr), 1, "{fixture}: {stderr}");
        let doc = json(&stdout);
        let notes = doc["notes"].as_array().expect("notes");
        assert_eq!(notes.len(), 1, "{fixture}: {doc}");
        let sentence = &UNMAPPED_WARNING["warning: ".len()..];
        assert!(notes[0].as_str().unwrap().starts_with(sentence), "{fixture}: {doc}");
        assert_eq!(record(&doc, "asserted_unmapped")["declared"]["bytes"], "unmapped");
        let marked = doc["functions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row.get("declared").is_some())
            .count();
        assert_eq!(marked, 1, "{fixture}: only the declared row is marked");
    }
}

#[test]
fn a_partial_and_a_mapped_declaration_are_told_apart() {
    // `.fini` ends the R E segment at 0x1231; 0x1100 is `_start`.
    let bin = common::fixture("protoorder_narrowvote_x86_64");
    let (stdout, stderr, code) = common::run_kuna(&[
        "functions",
        &bin,
        "--json",
        "--define-function",
        "0x1224-0x1300=past_end",
        "--define-function",
        "0x1100-0x1120=inside",
    ]);
    assert_eq!(code, 0, "{stderr}");
    let doc = json(&stdout);
    assert_eq!(record(&doc, "past_end")["declared"]["bytes"], "partial");
    assert_eq!(record(&doc, "inside")["declared"]["bytes"], "mapped");
    let notes = doc["notes"].as_array().expect("notes");
    assert_eq!(notes.len(), 1, "{doc}");
    assert!(notes[0].as_str().unwrap().contains("the first unmapped byte is 0x1231"), "{doc}");
    assert_eq!(stderr.matches("warning: declared function past_end").count(), 1, "{stderr}");
    assert!(!stderr.contains("inside"), "{stderr}");
}

/// A segment the image declares inaccessible (arm64 Mach-O `__PAGEZERO`, which
/// kuna reads as zero fill) holds no bytes; the Mach header after it does.
#[test]
fn an_inaccessible_segment_is_unmapped() {
    let bin = common::fixture("macho_imports_arm64");
    let (stdout, stderr, code) = common::run_kuna(&[
        "functions",
        &bin,
        "--json",
        "--define-function",
        "0x1000=in_pagezero",
        "--define-function",
        "0xfffffff0-0x100000010=into_text",
        "--define-function",
        "0x100000000-0x100000010=mach_header",
    ]);
    assert_eq!(code, 0, "{stderr}");
    let doc = json(&stdout);
    assert_eq!(record(&doc, "in_pagezero")["declared"]["bytes"], "unmapped");
    assert_eq!(record(&doc, "into_text")["declared"]["bytes"], "unmapped");
    assert_eq!(record(&doc, "mach_header")["declared"]["bytes"], "mapped");
    assert_eq!(doc["notes"].as_array().map(Vec::len), Some(2), "{doc}");
    assert_eq!(stderr.matches("warning: declared function").count(), 2, "{stderr}");
}

/// The last byte of the address space ends no half-open extent, so it is not
/// mapped however the extent is spelled.
#[test]
fn the_top_byte_of_the_address_space_is_unmapped() {
    let bin = common::fixture("protoorder_narrowvote_x86_64");
    let (stdout, stderr, code) = common::run_kuna(&[
        "functions",
        &bin,
        "--json",
        "--define-function",
        "0xffffffffffffffff=at_top",
    ]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(record(&json(&stdout), "at_top")["declared"]["bytes"], "unmapped");
    assert_eq!(stderr.matches("warning: declared function at_top").count(), 1, "{stderr}");
}

#[test]
fn an_inventory_without_declarations_gains_no_key() {
    let bin = common::fixture("protoorder_narrowvote_x86_64");
    let (stdout, stderr, code) = common::run_kuna(&["functions", &bin, "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc = json(&stdout);
    assert!(doc.get("notes").is_none(), "{doc}");
    assert!(doc["functions"].as_array().unwrap().iter().all(|row| row.get("declared").is_none()));
}

#[test]
fn every_surface_that_takes_the_flag_warns_once() {
    let bin = common::fixture("protoorder_narrowvote_x86_64");
    let decl = ["--define-function", UNMAPPED];
    let run = |args: &[&str]| {
        let mut argv = args.to_vec();
        argv.extend(decl);
        common::run_kuna(&argv)
    };

    let (stdout, stderr, code) = run(&["decompile", &bin, "main"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("main("), "{stdout}");
    assert_eq!(warnings(&stderr), 1, "the script surface relays it once: {stderr}");

    let (stdout, stderr, code) = run(&["decompile", &bin, "main", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(warnings(&stderr), 1, "{stderr}");
    assert_eq!(json(&stdout)["notes"].as_array().map(Vec::len), Some(1));

    let (stdout, stderr, code) = run(&["decompile-all", &bin, "--json", "--jobs", "2"]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(warnings(&stderr), 1, "a worker does not repeat it: {stderr}");
    let doc = json(&stdout);
    assert_eq!(doc["notes"].as_array().map(Vec::len), Some(1));
    let row = record(&doc, "asserted_unmapped");
    assert_eq!(row["declared"]["bytes"], "unmapped");
    assert!(row["error"].as_str().unwrap_or("").contains("not mapped"), "{row}");

    let (stdout, stderr, code) = run(&["disassemble", &bin, "main", "--json"]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(warnings(&stderr), 1, "{stderr}");
    let notes = json(&stdout)["notes"].clone();
    assert!(notes.to_string().contains("asserted_unmapped"), "{notes}");

    let (_, stderr, code) = run(&["decompile", &bin, "0x999999999", "--addr"]);
    assert_eq!(code, 1);
    assert_eq!(warnings(&stderr), 1, "{stderr}");
    assert!(stderr.contains("error: address 0x999999999 is not mapped in this input"), "{stderr}");
}

/// The reason an unmapped declaration is kept rather than refused: it names
/// the calls into it, as a routine in a boot ROM the image calls needs.
#[test]
fn an_unmapped_declaration_still_names_its_call_sites() {
    let raw = common::scratch_file("declared-rom-call", "bin");
    std::fs::write(&raw, [0xe8, 0xfb, 0xff, 0x03, 0x00, 0xc3]).unwrap();
    let raw_path = raw.to_str().unwrap();
    let (stdout, stderr, code) = common::run_kuna(&[
        "decompile",
        raw_path,
        "0x10000",
        "--addr",
        "--raw-image",
        "--target",
        "x86:LE:64:default",
        "--base",
        "0x10000",
        "--define-function",
        "0x50000=rom_memcpy",
    ]);
    std::fs::remove_file(&raw).unwrap();
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("rom_memcpy();"), "{stdout}");
    assert_eq!(
        stderr.matches("warning: declared function rom_memcpy at 0x50000: its entry 0x50000").count(),
        1,
        "{stderr}"
    );
}
