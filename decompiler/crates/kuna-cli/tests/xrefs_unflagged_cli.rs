//! `kuna xrefs` over code linked into a section not marked as code (GH #1406).
//! An Apple toolchain links a function placed in `__TEXT,__const` with no
//! instruction attribute, although `__TEXT` is executable; `--from` it answered
//! `count: 0` exactly as it answers a function with no references. An explicit
//! source in executable memory is now decoded when the image says it is code,
//! and `coverage` says whether the source was decoded and why, and which called
//! code the walk did not decode.
//!
//! Fixtures: `unflagged_*` (see `unflagged.provenance`). Integration tests
//! require the built processor specs under `specs/`.

use serde_json::Value;

use crate::common;

const MACHO: &str = "unflagged_macho_arm64";
const ELF: &str = "unflagged_elf_x86_64";

fn xrefs(binary: &str, direction: &str, target: &str) -> (Value, String) {
    let path = common::fixture(binary);
    let (stdout, stderr, rc) = common::run_kuna(&["xrefs", &path, direction, target, "--json"]);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    (serde_json::from_str(&stdout).unwrap(), stderr)
}

/// `(kind, from_address, to_address, to_function name)` of every row.
fn rows(doc: &Value) -> Vec<(String, u64, u64, String)> {
    doc["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["kind"].as_str().unwrap().to_owned(),
                r["from_address"].as_u64().unwrap(),
                r["to_address"].as_u64().unwrap(),
                r["to_function"]["name"].as_str().unwrap_or("-").to_owned(),
            )
        })
        .collect()
}

fn call(from: u64, to: u64, callee: &str) -> (String, u64, u64, String) {
    ("call".to_owned(), from, to, callee.to_owned())
}

fn unwalked(doc: &Value) -> Vec<(u64, String, String)> {
    doc["coverage"]["unwalked"]
        .as_array()
        .unwrap()
        .iter()
        .map(|u| {
            (
                u["address"].as_u64().unwrap(),
                u["name"].as_str().unwrap().to_owned(),
                u["section"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

/// `(decoded, section, code_section, executable)`.
fn coverage(doc: &Value) -> (bool, Option<&str>, Option<bool>, Option<bool>) {
    let c = &doc["coverage"];
    (
        c["decoded"].as_bool().unwrap(),
        c["section"].as_str(),
        c["code_section"].as_bool(),
        c["executable"].as_bool(),
    )
}

const UNFLAGGED: (Option<&str>, Option<bool>, Option<bool>) = (Some("__TEXT,__const"), Some(false), Some(true));

fn decoded_unflagged(doc: &Value) -> bool {
    coverage(doc) == (true, UNFLAGGED.0, UNFLAGGED.1, UNFLAGGED.2)
}

/// The issue's query, by address and by name: `_main` calls `_middle`, so its
/// `bl _goal` is answered, and the answer says it was decoded outside the code
/// sections.
#[test]
fn an_explicit_source_that_decoded_code_calls_is_decoded() {
    for target in ["0x1000003fc", "_middle"] {
        let (doc, stderr) = xrefs(MACHO, "--from", target);
        assert_eq!(doc["target"]["name"], "_middle", "{target}");
        assert_eq!(rows(&doc), [call(0x1000003fc, 0x1000003e4, "_goal")], "{target}");
        assert!(decoded_unflagged(&doc), "{target}: {}", doc["coverage"]);
        let notes = doc["coverage"]["notes"].as_array().unwrap();
        assert_eq!(notes.len(), 1, "{target}: {notes:?}");
        assert!(notes[0].as_str().unwrap().contains("a decoded direct call or branch targets it"), "{notes:?}");
        assert!(stderr.contains("note: 0x1000003fc is in __TEXT,__const"), "{target}: {stderr}");
    }
}

/// A decoded source with nothing to reference is an empty answer that says it
/// was decoded, which is what tells it apart from a source nobody examined:
/// an untyped symbol nothing calls, a string and a global.
#[test]
fn an_empty_answer_says_whether_the_source_was_decoded() {
    let (quiet, _) = xrefs(MACHO, "--from", "_quiet");
    assert_eq!(quiet["count"], 0);
    assert!(decoded_unflagged(&quiet), "{}", quiet["coverage"]);
    let (orphan, stderr) = xrefs(MACHO, "--from", "_orphan");
    assert_eq!(orphan["count"], 0, "a Mach-O symbol alone is not evidence of code");
    assert_eq!(coverage(&orphan), (false, UNFLAGGED.0, UNFLAGGED.1, UNFLAGGED.2));
    assert!(stderr.contains("nothing says 0x10000040c is code"), "{stderr}");
    for (target, section, executable) in [("_greeting", "__TEXT,__cstring", true), ("_data", "__DATA,__data", false)] {
        let (doc, stderr) = xrefs(MACHO, "--from", target);
        assert_eq!(doc["count"], 0, "{target}");
        assert_eq!(coverage(&doc), (false, Some(section), Some(false), Some(executable)), "{target}");
        assert!(stderr.contains(&format!("is in {section}")), "{target}: {stderr}");
    }
}

/// `--to` walks exactly what it walked before, so the call from `_middle` is
/// still missing, but the answer names the called code it did not decode. A
/// code-section source needs no note.
#[test]
fn to_lists_the_called_code_it_did_not_walk() {
    let (doc, stderr) = xrefs(MACHO, "--to", "_goal");
    assert_eq!(rows(&doc).iter().map(|r| r.1).collect::<Vec<_>>(), [0x1000003ec], "_direct_goal only");
    assert_eq!(
        unwalked(&doc),
        [
            (0x1000003fc, "_middle".to_owned(), "__TEXT,__const".to_owned()),
            (0x100000404, "_quiet".to_owned(), "__TEXT,__const".to_owned()),
        ]
    );
    assert!(stderr.contains("2 targets of decoded calls or branches were not walked"), "{stderr}");
    let (main, stderr) = xrefs(MACHO, "--from", "_main");
    assert_eq!(rows(&main), [call(0x1000003d8, 0x1000003fc, "-"), call(0x1000003dc, 0x100000404, "-")]);
    assert_eq!(coverage(&main), (true, Some("__TEXT,__text"), Some(true), Some(true)));
    assert!(main["coverage"]["notes"].as_array().unwrap().is_empty() && stderr.is_empty(), "{stderr}");
    let path = common::fixture(MACHO);
    let (text, _, rc) = common::run_kuna(&["xrefs", &path, "--from", "_main"]);
    assert_eq!(rc, 0);
    assert!(text.contains("0x1000003fc\tcall\t_middle\t@0x1000003d8"), "the symbol names the callee: {text}");
}

/// The inventory is GH #1339's: no function in `__TEXT,__const` is listed.
#[test]
fn the_inventory_still_leaves_unflagged_code_out() {
    let path = common::fixture(MACHO);
    let (stdout, stderr, rc) = common::run_kuna(&["functions", &path, "--json"]);
    assert_eq!(rc, 0, "{stderr}");
    let doc: Value = serde_json::from_str(&stdout).unwrap();
    let mut names: Vec<&str> =
        doc["functions"].as_array().unwrap().iter().map(|f| f["name"].as_str().unwrap()).collect();
    names.sort_unstable();
    assert_eq!(names, ["_direct_goal", "_goal", "_main", "_unrelated"]);
}

/// ELF's form of the layout: `.rodata` in the `R E` segment. A function symbol
/// there is decoded whether or not anything calls it; a data object and unnamed
/// bytes are not, although both decode as `call goal`.
#[test]
fn elf_rodata_code_needs_a_function_symbol_or_a_caller() {
    for (target, at) in [("middle", 0x4000c2), ("0x4000c2", 0x4000c2), ("lonely", 0x4000c8)] {
        let (doc, _) = xrefs(ELF, "--from", target);
        assert_eq!(rows(&doc), [call(at, 0x4000b6, "goal")], "{target}");
        assert_eq!(coverage(&doc), (true, Some(".rodata"), Some(false), Some(true)), "{target}");
    }
    for (target, says) in [("table", "the data object table"), ("0x4000ce", "the data object table"), ("0x4000d3", "nothing says")] {
        let (doc, stderr) = xrefs(ELF, "--from", target);
        assert_eq!(doc["count"], 0, "{target}");
        assert_eq!(coverage(&doc), (false, Some(".rodata"), Some(false), Some(true)), "{target}");
        assert!(stderr.contains(says), "{target}: {stderr}");
    }
    let (to, _) = xrefs(ELF, "--to", "goal");
    assert_eq!(rows(&to).iter().map(|r| r.1).collect::<Vec<_>>(), [0x4000bc], "direct_goal only");
    assert_eq!(unwalked(&to), [(0x4000c2, "middle".to_owned(), ".rodata".to_owned())]);
}
