//! The study view's commands on the browser example (`sample.elf`): `inspect`
//! against `decompile`, the instruction listing against the C and the file,
//! `read`, and the `--assert` plane end to end. Skips visibly without specs.

use std::path::PathBuf;

use crate::json::read::{parse, Value};
use crate::{run_request, Request};

fn fixture() -> Option<(String, String)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().ok()?;
    let binary = root.join("integrations/web/test/fixtures/sample.elf");
    let specs = root.join("specs");
    Some((binary.to_str()?.to_string(), specs.to_str()?.to_string()))
}

fn run(cmd: &str, args: &[&str], asserts: &[&str]) -> Option<Result<Value, String>> {
    let (binary, specs) = fixture()?;
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let asserts: Vec<String> = asserts.iter().map(|s| s.to_string()).collect();
    let out = run_request(&Request {
        binary: &binary,
        spec_root: &specs,
        cmd,
        args: &args,
        mode: None,
        language: None,
        asserts: &asserts,
    });
    match out {
        Err(e) if e.contains("could not build an architecture") => {
            eprintln!("study_tests: skipping: {e}");
            None
        }
        other => Some(other.map(|json| parse(&json))),
    }
}

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(fixture().expect("fixture").0).expect("fixture bytes")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn inspect_is_decompile_plus_a_consistent_map_of_the_function() {
    let Some(decompiled) = run("decompile", &["main"], &[]) else { return };
    let decompiled = decompiled.expect("decompile main");
    let Some(inspected) = run("inspect", &["main"], &[]) else { return };
    let inspected = inspected.expect("inspect main");
    let f = inspected.get("function");
    let code = f.get("code").str().expect("code");
    assert_eq!(Some(code), decompiled.get("functions").arr()[0].get("code").str());
    assert_eq!(inspected.get("language").str(), Some("c-language"));
    assert_eq!(inspected.get("target").get("bits").u64(), Some(64));
    assert_eq!(*f.get("tokens_error"), Value::Null);

    let lines: Vec<&str> = code.split('\n').collect();
    let tokens = f.get("tokens").arr();
    assert!(!tokens.is_empty());
    for t in tokens {
        let line = t.get("line").u64().expect("line") as usize;
        let col = t.get("col").u64().expect("col") as usize;
        let text = t.get("text").str().expect("text");
        let units: Vec<u16> = lines[line - 1].encode_utf16().collect();
        let len = text.encode_utf16().count();
        assert_eq!(String::from_utf16_lossy(&units[col..col + len]), text);
    }

    let entry = f.get("address").u64().expect("entry");
    let bytes = fixture_bytes();
    let instructions = f.get("instructions").arr();
    assert!(!instructions.is_empty());
    assert_eq!(f.get("instructions_truncated"), &Value::Bool(false));
    let starts: Vec<u64> = instructions.iter().filter_map(|i| i.get("address").u64()).collect();
    for i in instructions {
        let address = i.get("address").u64().expect("address");
        assert_eq!(i.get("offset").num(), Some((address - entry) as f64));
        for line in i.get("lines").arr() {
            let line = line.u64().expect("line") as usize;
            assert!((1..=lines.len()).contains(&line), "line {line} out of range");
        }
        let size = i.get("size").u64().expect("size") as usize;
        let offset = i.get("file_offset").u64().expect("sample.elf's .text is file-backed") as usize;
        assert_eq!(i.get("bytes").str(), Some(hex(&bytes[offset..offset + size]).as_str()));
    }
    for mapping in f.get("line_mappings").arr() {
        for address in mapping.get("addresses").arr() {
            assert!(starts.contains(&address.u64().expect("address")), "{mapping:?}");
        }
    }
    let first = &instructions[0];
    let Some(read) = run("read", &[first.get("address_hex").str().expect("hex"), "16"], &[]) else {
        return;
    };
    let read = read.expect("read");
    let first_bytes = first.get("bytes").str().expect("bytes");
    assert!(read.get("bytes").str().expect("bytes").starts_with(first_bytes));
    assert_eq!(read.get("size").u64(), Some(16));
    assert_eq!(read.get("file_offset"), first.get("file_offset"));
}

/// `function <entry>=<name>` renames the function everywhere the next load
/// looks: the inventory lists the new name with the old ones as aliases, the
/// name selects it, and a directive qualified with it lands.
#[test]
fn a_function_rename_reaches_the_inventory_and_qualifies_later_directives() {
    let rename = "function 0x1198=entry_main";
    let Some(list) = run("list", &[], &[rename]) else { return };
    let list = list.expect("list");
    let row = list
        .get("functions")
        .arr()
        .iter()
        .find(|f| f.get("address_hex").str() == Some("0x1198"))
        .expect("main's entry is listed");
    assert_eq!(row.get("name").str(), Some("entry_main"));
    assert!(row.get("aliases").arr().iter().any(|a| a.str() == Some("main")), "{row:?}");
    assert_eq!(list.get("assertions").arr()[0].get("status").str(), Some("applied"));
    assert!(list.get("sections").arr().iter().any(|s| s.get("name").str() == Some(".text")));

    let Some(inspected) = run("inspect", &["entry_main"], &[rename, "name entry_main::v1 total"]) else {
        return;
    };
    let inspected = inspected.expect("inspect the renamed function");
    let code = inspected.get("function").get("code").str().expect("code");
    assert!(code.contains("entry_main(") && code.contains("total"), "{code}");
    let statuses: Vec<&str> =
        inspected.get("assertions").arr().iter().filter_map(|a| a.get("status").str()).collect();
    assert_eq!(statuses, vec!["applied", "applied"]);
}

/// A `bytes` overlay is what every later read of the image sees.
#[test]
fn a_byte_patch_reaches_read_and_the_listing() {
    let patch = "bytes 0x1198 90909090";
    let Some(before) = run("read", &["0x1198", "4"], &[]) else { return };
    let before = before.expect("read");
    let offset = before.get("file_offset").u64().expect("file offset") as usize;
    assert_eq!(before.get("bytes").str(), Some(hex(&fixture_bytes()[offset..offset + 4]).as_str()));
    let Some(after) = run("read", &["0x1198", "4"], &[patch]) else { return };
    assert_eq!(after.expect("read").get("bytes").str(), Some("90909090"));
    let Some(inspected) = run("inspect", &["main"], &[patch]) else { return };
    let inspected = inspected.expect("inspect");
    let first = &inspected.get("function").get("instructions").arr()[0];
    assert_eq!(first.get("bytes").str(), Some("90"));
    assert_eq!(first.get("mnemonic").str(), Some("NOP"));
}

/// A directive that binds to nothing is a report row and the C still comes
/// back; one that does not parse is the run's error.
#[test]
fn a_rejected_directive_is_a_row_and_a_malformed_one_an_error() {
    let Some(inspected) = run("inspect", &["main"], &["name v999 nope"]) else { return };
    let inspected = inspected.expect("a rejected directive still returns the function");
    assert!(inspected.get("function").get("code").str().is_some());
    let row = &inspected.get("assertions").arr()[0];
    assert_eq!(row.get("status").str(), Some("rejected"));
    assert!(row.get("detail").str().is_some());

    let Some(bad) = run("inspect", &["main"], &["bogus directive"]) else { return };
    assert!(bad.expect_err("unparsable").starts_with("--assert \"bogus directive\""));
    let Some(two_lines) = run("inspect", &["main"], &["name v1 a\nname v2 b"]) else { return };
    assert!(two_lines.is_err());
}

#[test]
fn read_bounds_its_length_and_stops_at_unmapped_memory() {
    let Some(zero) = run("read", &["0x1198", "0"], &[]) else { return };
    assert!(zero.is_err());
    let Some(huge) = run("read", &["0x1198", "65537"], &[]) else { return };
    assert!(huge.is_err());
    let Some(nowhere) = run("read", &["0x7fff0000", "16"], &[]) else { return };
    let nowhere = nowhere.expect("unmapped is an empty read, not an error");
    assert_eq!(nowhere.get("size").u64(), Some(0));
    assert_eq!(nowhere.get("bytes").str(), Some(""));
    assert_eq!(*nowhere.get("file_offset"), Value::Null);
}
