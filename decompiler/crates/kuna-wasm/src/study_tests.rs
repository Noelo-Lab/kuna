//! The study view's commands on the browser example (`sample.elf`): `inspect`
//! against `decompile`, the instruction listing against the C and the file,
//! `read`, and the `--assert` plane end to end; `strings` on the crackme
//! example (`crackme.elf`); and the type definitions `inspect` prints above a
//! function (`structs.elf`).

use std::path::PathBuf;

use kuna_console::disasm::hex;

use crate::{run_request, Request};
use serde_json::Value;

fn fixture() -> (String, String) {
    fixture_named("sample.elf")
}

fn fixture_named(name: &str) -> (String, String) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repository root");
    let binary = root.join("integrations/web/test/fixtures").join(name);
    let specs = root.join("specs");
    (
        binary.to_str().unwrap().to_string(),
        specs.to_str().unwrap().to_string(),
    )
}

fn run(cmd: &str, args: &[&str], asserts: &[&str]) -> Result<Value, String> {
    run_on("sample.elf", cmd, args, asserts)
}

fn run_on(name: &str, cmd: &str, args: &[&str], asserts: &[&str]) -> Result<Value, String> {
    let (binary, specs) = fixture_named(name);
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
    out.map(|json| serde_json::from_str(&json).expect("valid JSON"))
}

fn fixture_bytes() -> Vec<u8> {
    std::fs::read(fixture().0).expect("fixture bytes")
}

#[test]
fn inspect_is_decompile_plus_a_consistent_map_of_the_function() {
    let decompiled = run("decompile", &["main"], &[]);
    let decompiled = decompiled.expect("decompile main");
    let inspected = run("inspect", &["main"], &[]);
    let inspected = inspected.expect("inspect main");
    let f = &inspected["function"];
    let code = f["code"].as_str().expect("code");
    assert_eq!(
        Some(code),
        decompiled["functions"].as_array().expect("array")[0]["code"].as_str()
    );
    assert_eq!(inspected["language"].as_str(), Some("c-language"));
    assert_eq!(inspected["target"]["bits"].as_u64(), Some(64));
    assert_eq!(f["tokens_error"], Value::Null);

    let lines: Vec<&str> = code.split('\n').collect();
    let tokens = f["tokens"].as_array().expect("array");
    assert!(!tokens.is_empty());
    for t in tokens {
        let line = t["line"].as_u64().expect("line") as usize;
        let col = t["col"].as_u64().expect("col") as usize;
        let text = t["text"].as_str().expect("text");
        let units: Vec<u16> = lines[line - 1].encode_utf16().collect();
        let len = text.encode_utf16().count();
        assert_eq!(String::from_utf16_lossy(&units[col..col + len]), text);
    }

    let entry = f["address"].as_u64().expect("entry");
    let bytes = fixture_bytes();
    let instructions = f["instructions"].as_array().expect("array");
    assert!(!instructions.is_empty());
    assert_eq!(f["instructions_truncated"], Value::Bool(false));
    let starts: Vec<u64> = instructions
        .iter()
        .filter_map(|i| i["address"].as_u64())
        .collect();
    for i in instructions {
        let address = i["address"].as_u64().expect("address");
        assert_eq!(i["offset"].as_u64(), Some(address - entry));
        for line in i["lines"].as_array().expect("array") {
            let line = line.as_u64().expect("line") as usize;
            assert!(
                (1..=lines.len()).contains(&line),
                "line {line} out of range"
            );
        }
        let size = i["size"].as_u64().expect("size") as usize;
        let offset = i["file_offset"]
            .as_u64()
            .expect("sample.elf's .text is file-backed") as usize;
        assert_eq!(
            i["bytes"].as_str(),
            Some(hex(&bytes[offset..offset + size]).as_str())
        );
    }
    for mapping in f["line_mappings"].as_array().expect("array") {
        for address in mapping["addresses"].as_array().expect("array") {
            assert!(
                starts.contains(&address.as_u64().expect("address")),
                "{mapping:?}"
            );
        }
    }
    let first = &instructions[0];
    let read = run(
        "read",
        &[first["address_hex"].as_str().expect("hex"), "16"],
        &[],
    );
    let read = read.expect("read");
    let first_bytes = first["bytes"].as_str().expect("bytes");
    assert!(read["bytes"]
        .as_str()
        .expect("bytes")
        .starts_with(first_bytes));
    assert_eq!(read["size"].as_u64(), Some(16));
    assert_eq!(read["file_offset"], first["file_offset"]);
}

/// `function <entry>=<name>` renames the function everywhere the next load
/// looks: the inventory lists the new name with the old ones as aliases, the
/// name selects it, and a directive qualified with it lands.
#[test]
fn a_function_rename_reaches_the_inventory_and_qualifies_later_directives() {
    let rename = "function 0x1198=entry_main";
    let list = run("list", &[], &[rename]);
    let list = list.expect("list");
    let row = list["functions"]
        .as_array()
        .expect("array")
        .iter()
        .find(|f| f["address_hex"].as_str() == Some("0x1198"))
        .expect("main's entry is listed");
    assert_eq!(row["name"].as_str(), Some("entry_main"));
    assert!(
        row["aliases"]
            .as_array()
            .expect("array")
            .iter()
            .any(|a| a.as_str() == Some("main")),
        "{row:?}"
    );
    assert_eq!(
        list["assertions"].as_array().expect("array")[0]["status"].as_str(),
        Some("applied")
    );
    assert!(list["sections"]
        .as_array()
        .expect("array")
        .iter()
        .any(|s| s["name"].as_str() == Some(".text")));

    let inspected = run(
        "inspect",
        &["entry_main"],
        &[rename, "name entry_main::v1 total"],
    );
    let inspected = inspected.expect("inspect the renamed function");
    let code = inspected["function"]["code"].as_str().expect("code");
    assert!(
        code.contains("entry_main(") && code.contains("total"),
        "{code}"
    );
    let statuses: Vec<&str> = inspected["assertions"]
        .as_array()
        .expect("array")
        .iter()
        .filter_map(|a| a["status"].as_str())
        .collect();
    assert_eq!(statuses, vec!["applied", "applied"]);
}

/// A `bytes` overlay is what every later read of the image sees.
#[test]
fn a_byte_patch_reaches_read_and_the_listing() {
    let patch = "bytes 0x1198 90909090";
    let before = run("read", &["0x1198", "4"], &[]);
    let before = before.expect("read");
    let offset = before["file_offset"].as_u64().expect("file offset") as usize;
    assert_eq!(
        before["bytes"].as_str(),
        Some(hex(&fixture_bytes()[offset..offset + 4]).as_str())
    );
    let after = run("read", &["0x1198", "4"], &[patch]);
    assert_eq!(after.expect("read")["bytes"].as_str(), Some("90909090"));
    let inspected = run("inspect", &["main"], &[patch]);
    let inspected = inspected.expect("inspect");
    let first = &inspected["function"]["instructions"]
        .as_array()
        .expect("array")[0];
    assert_eq!(first["bytes"].as_str(), Some("90"));
    assert_eq!(first["mnemonic"].as_str(), Some("NOP"));
}

/// A directive that binds to nothing is a report row and the C still comes
/// back; one that does not parse is the run's error.
#[test]
fn a_rejected_directive_is_a_row_and_a_malformed_one_an_error() {
    let inspected = run("inspect", &["main"], &["name v999 nope"]);
    let inspected = inspected.expect("a rejected directive still returns the function");
    assert!(inspected["function"]["code"].as_str().is_some());
    let row = &inspected["assertions"].as_array().expect("array")[0];
    assert_eq!(row["status"].as_str(), Some("rejected"));
    assert!(row["detail"].as_str().is_some());

    let bad = run("inspect", &["main"], &["bogus directive"]);
    assert!(bad
        .expect_err("unparsable")
        .starts_with("--assert \"bogus directive\""));
    let two_lines = run("inspect", &["main"], &["name v1 a\nname v2 b"]);
    assert!(two_lines.is_err());
}

#[test]
fn read_bounds_its_length_and_stops_at_unmapped_memory() {
    let zero = run("read", &["0x1198", "0"], &[]);
    assert!(zero.is_err());
    let huge = run("read", &["0x1198", "65537"], &[]);
    assert!(huge.is_err());
    // sample.elf's executable segment ends at 0x11f9 (`.fini`), and a hole
    // follows: a read across the end returns the four mapped bytes only.
    let tail = run("read", &["0x11f5", "64"], &[]);
    let tail = tail.expect("a read across the end of the segment");
    assert_eq!(tail["size"].as_u64(), Some(4));
    let offset = tail["file_offset"].as_u64().expect("file-backed") as usize;
    assert_eq!(
        tail["bytes"].as_str(),
        Some(hex(&fixture_bytes()[offset..offset + 4]).as_str())
    );
    let nowhere = run("read", &["0x7fff0000", "16"], &[]);
    let nowhere = nowhere.expect("unmapped is an empty read, not an error");
    assert_eq!(nowhere["size"].as_u64(), Some(0));
    assert_eq!(nowhere["bytes"].as_str(), Some(""));
    assert_eq!(nowhere["file_offset"], Value::Null);
}

/// `xrefs` answers from the CLI's reference walk: `main` calls `add`,
/// `sum_to` and `printf` in that order and loads the format string, and
/// `sum_to`'s one caller is `main`.
#[test]
fn xrefs_names_both_ends_of_every_reference() {
    let main = run("xrefs", &["main"], &[]);
    let main = main.expect("xrefs main");
    let callees: Vec<&str> = main["callees"]
        .as_array()
        .expect("array")
        .iter()
        .filter_map(|r| r["name"].as_str())
        .collect();
    assert_eq!(callees, vec!["add", "sum_to", "printf"]);
    assert!(main["callees"]
        .as_array()
        .expect("array")
        .iter()
        .all(|r| r["kind"].as_str() == Some("call")));
    let data = main["data_refs"].as_array().expect("array");
    assert!(
        data.iter()
            .any(|r| r["address_hex"].as_str() == Some("0x2004")),
        "{data:?}"
    );
    let sum_to = run("xrefs", &["sum_to"], &[]);
    let sum_to = sum_to.expect("xrefs sum_to");
    let callers = sum_to["callers"].as_array().expect("array");
    assert_eq!(callers.len(), 1);
    assert_eq!(callers[0]["name"].as_str(), Some("main"));
    assert_eq!(callers[0]["address_hex"].as_str(), Some("0x1198"));
    assert_eq!(callers[0]["instruction"].as_str(), Some("CALL 0x1161"));
    assert!(callers[0].get("from_hex").is_some());
}

/// `strings` names the function behind every use of a literal, at the
/// instruction that makes it: one loaded directly (the failure message, from
/// `check` and `main`) and one read through the global pointer that holds it
/// (the flag, through `secret`, twice in `check`).
#[test]
fn strings_names_who_uses_each_literal_including_through_a_pointer() {
    let doc = run_on("crackme.elf", "strings", &[], &[]).expect("strings");
    let rows = doc["strings"].as_array().expect("array");
    assert_eq!(doc["count"].as_u64(), Some(rows.len() as u64));
    let row = |text: &str| {
        rows.iter()
            .find(|s| s["text"].as_str() == Some(text))
            .unwrap_or_else(|| panic!("{text:?} is listed: {rows:?}"))
    };
    let users = |text: &str| -> Vec<String> {
        row(text)["uses"]
            .as_array()
            .expect("uses")
            .iter()
            .map(|u| u["name"].as_str().unwrap_or("?").to_string())
            .collect()
    };
    assert_eq!(users("Nope, that is not the flag."), vec!["check", "main"]);
    assert_eq!(users("Enter the flag: "), vec!["main"]);
    assert_eq!(users("Correct! You found the flag."), vec!["main"]);
    assert_eq!(row("Enter the flag: ")["uses"][0]["via"], Value::Null);
    assert_eq!(row("Enter the flag: ")["in_code"], Value::Bool(false));

    let flag = row("flag{str1ngs_4re_3asy}");
    assert_eq!(flag["section"].as_str(), Some(".rodata"));
    let uses = flag["uses"].as_array().expect("uses");
    assert_eq!(uses.len(), 2, "{uses:?}");
    let check = run_on("crackme.elf", "inspect", &["check"], &[]).expect("inspect check");
    let sites: Vec<&str> = check["function"]["instructions"]
        .as_array()
        .expect("instructions")
        .iter()
        .filter_map(|i| i["address_hex"].as_str())
        .collect();
    for u in uses {
        assert_eq!(u["name"].as_str(), Some("check"));
        assert_eq!(u["address_hex"], check["function"]["address_hex"]);
        assert_eq!(u["kind"].as_str(), Some("read"));
        assert_eq!(u["via"]["name"].as_str(), Some("secret"));
        let slot = u["via"]["address_hex"].as_str().expect("pointer address");
        assert!(u["instruction"].as_str().expect("text").contains(slot), "{u:?}");
        assert!(sites.contains(&u["at_hex"].as_str().expect("site")), "{u:?} is an instruction of check");
    }
}

/// `inspect` prints the definitions of the types a function uses above it
/// (`structdefs on`), so a struct the decompiler worked out is defined where it
/// is read. The function's own text is unchanged, `decompile` keeps the CLI's
/// default, and the token map and line mappings follow the shifted lines.
#[test]
fn inspect_defines_the_types_a_function_uses_above_it() {
    let inspected = run_on("structs.elf", "inspect", &["make_item"], &[]).expect("inspect make_item");
    let f = &inspected["function"];
    let code = f["code"].as_str().expect("code");
    let plain = run_on("structs.elf", "decompile", &["make_item"], &[]).expect("decompile make_item");
    let body = plain["functions"][0]["code"].as_str().expect("code");
    assert!(!body.contains("struct struct_0 {"), "decompile keeps the CLI's default: {body}");
    let preamble = code
        .strip_suffix(body)
        .unwrap_or_else(|| panic!("inspect is the definitions plus the decompile text:\n{code}"));
    assert!(preamble.starts_with("typedef struct struct_0 struct_0;\n"), "{preamble}");
    assert!(preamble.contains("\nstruct struct_0 {\n") && preamble.ends_with("};\n\n"), "{preamble}");
    let types = f["types"].as_array().expect("types");
    assert_eq!(types.len(), 1, "{types:?}");
    assert_eq!(types[0]["name"].as_str(), Some("struct_0"));
    assert_eq!(types[0]["size"].as_i64(), Some(0x28));
    assert_eq!(f["tokens_error"], Value::Null);
    let shift = preamble.lines().count() as u64;
    let lines: Vec<u64> = f["instructions"]
        .as_array()
        .expect("instructions")
        .iter()
        .flat_map(|i| i["lines"].as_array().expect("lines").iter().filter_map(Value::as_u64))
        .collect();
    assert!(!lines.is_empty() && lines.iter().all(|&l| l > shift + 1), "{shift} {lines:?}");
}
