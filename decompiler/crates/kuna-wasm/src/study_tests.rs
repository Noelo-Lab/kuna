//! The study view's commands on the browser example (`sample.elf`): `inspect`
//! against `decompile`, the instruction listing against the C and the file,
//! `read`, and the `--assert` plane end to end; `strings` on the crackme
//! example (`crackme.elf`); the type definitions `inspect` prints above a
//! function (`structs.elf`); and the globals a function names, the `data`
//! window around one and the `uses` of it (`globals.elf`, and its stripped
//! twin); and where each function's frame pointer and stack pointer sit
//! (`frames-O2.elf`, `frames-asm.elf`, `frames-i386.o`, `frames-arm.o`,
//! `sample_aarch64.o`).

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

/// `inspect`'s C for `function` of `binary` under `asserts`, as sorted lines
/// with each `(from, to)` identifier swapped.
fn inspected_lines(binary: &str, function: &str, asserts: &[&str], renames: &[(&str, &str)]) -> Vec<String> {
    let inspected = run_on(binary, "inspect", &[function], asserts).expect("inspect");
    let code = inspected["function"]["code"].as_str().expect("code");
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out: Vec<String> = code
        .lines()
        .map(|line| {
            line.split_inclusive(|c: char| !word(c))
                .map(|piece| {
                    let name = piece.trim_end_matches(|c: char| !word(c));
                    match renames.iter().find(|(from, _)| *from == name) {
                        Some((_, to)) => format!("{to}{}", &piece[name.len()..]),
                        None => piece.to_string(),
                    }
                })
                .collect::<String>()
        })
        .collect();
    out.sort();
    out
}

/// The page sends every rename on every request, each naming the local as the
/// first decompile printed it, so a rename must not rename another local:
/// `check`'s `len` (named after `strlen`'s result, `libcnames`) stays `len`
/// when `v1` is renamed, and `v1` stays `v1` when `len` is.
#[test]
fn a_local_rename_moves_no_other_local_number() {
    let plain = inspected_lines("crackme.elf", "check", &[], &[]);
    assert!(plain.iter().any(|line| line.contains(" len; // ")), "{plain:?}");
    assert!(plain.iter().any(|line| line.contains(" v1; // ")), "{plain:?}");
    assert_eq!(inspected_lines("crackme.elf", "check", &["name v1 ok"], &[("ok", "v1")]), plain);
    assert_eq!(inspected_lines("crackme.elf", "check", &["name len size"], &[("size", "len")]), plain);
}

/// The C of `check` in crackme under `asserts`, in `mode`.
fn check_in_mode(mode: Option<&str>, asserts: &[&str]) -> String {
    let (binary, specs) = fixture_named("crackme.elf");
    let args = vec!["check".to_string()];
    let asserts: Vec<String> = asserts.iter().map(|s| s.to_string()).collect();
    let out = run_request(&Request {
        binary: &binary,
        spec_root: &specs,
        cmd: "inspect",
        args: &args,
        mode,
        language: None,
        asserts: &asserts,
    })
    .expect("inspect check");
    let doc: Value = serde_json::from_str(&out).expect("valid JSON");
    assert!(
        doc["assertions"].as_array().is_none_or(|a| a.iter().all(|o| o["status"] == "applied")),
        "{mode:?} {asserts:?}: {}",
        doc["assertions"]
    );
    doc["function"]["code"].as_str().expect("code").to_string()
}

/// `check`'s `len` keeps the number it has in Reliable, so a rename the page
/// sent in Auto reaches the same local after a switch, whichever name it used.
#[test]
fn a_rename_reaches_the_same_local_after_a_mode_switch() {
    for asserts in [["name len size"], ["name v2 size"]] {
        for mode in [None, Some("reliable"), Some("aggressive")] {
            let code = check_in_mode(mode, &asserts);
            assert!(code.contains("  size = strlen(input);\n"), "{mode:?} {asserts:?}:\n{code}");
        }
    }
}

/// A rename to a number-shaped name is kept: `len` renamed `v7` prints `v7`,
/// and the next rename reaches it by that name.
#[test]
fn a_number_shaped_rename_of_a_library_name_is_kept() {
    let code = check_in_mode(None, &["name len v7"]);
    assert!(code.contains("  v7 = strlen(input);\n") && !code.contains(" len;"), "{code}");
    let code = check_in_mode(None, &["name len v7", "name v7 size"]);
    assert!(code.contains("  size = strlen(input);\n"), "{code}");
}

/// A rename onto `len` leaves `strlen`'s local its number `v2`, and the next
/// rename reaches it by that number.
#[test]
fn a_rename_onto_a_library_name_leaves_the_other_local_addressable() {
    let code = check_in_mode(None, &["name v1 len"]);
    assert!(code.contains("  v2 = strlen(input);\n") && code.contains("  len = 0;\n"), "{code}");
    let code = check_in_mode(None, &["name v1 len", "name v2 size"]);
    assert!(code.contains("  size = strlen(input);\n") && code.contains("  len = 0;\n"), "{code}");
}

/// Two renames in turn, the second naming the local the page showed as `v7`
/// after the first: both survive and nothing else moves.
#[test]
fn two_renames_in_turn_both_survive() {
    let plain = inspected_lines("long.elf", "main", &[], &[]);
    let renamed = inspected_lines(
        "long.elf",
        "main",
        &["name main::v6 word_count", "name main::v7 line_count"],
        &[("word_count", "v6"), ("line_count", "v7")],
    );
    assert_eq!(renamed, plain);
}

/// A rename names the whole printed variable: `long.elf`'s `v2` lives in
/// `eax` and two stack slots, and renaming it used to split off a new `v15`;
/// renaming `v3` used to swallow `v1`. `v3` prints as `fopen`'s `fp`
/// (`libcnames`), and the directive reaches it by its number.
#[test]
fn a_rename_names_the_whole_variable() {
    let plain = inspected_lines("long.elf", "main", &[], &[]);
    for (from, to, printed) in [("v2", "count", "v2"), ("v3", "input", "fp")] {
        let directive = format!("name main::{from} {to}");
        assert_eq!(inspected_lines("long.elf", "main", &[&directive], &[(to, printed)]), plain, "{directive}");
    }
}

/// A retype that keeps a local's width types the whole printed variable:
/// `type v2 unsigned int count` used to leave `v2`'s other definitions behind
/// as a new `unsigned int v15`. Only the declaration and the casts at its uses
/// change, whether the page retypes it alone or renames it first.
#[test]
fn a_same_width_retype_types_the_whole_variable() {
    let uncast = |lines: Vec<String>| -> Vec<String> {
        let mut out: Vec<String> = lines
            .into_iter()
            .map(|line| line.replace("(unsigned int)", "").replace("(int)", "").replace("  unsigned int v2;", "  int v2;"))
            .collect();
        out.sort();
        out
    };
    let plain = uncast(inspected_lines("long.elf", "main", &[], &[]));
    for asserts in [
        &["type main::v2 unsigned int count"][..],
        &["type main::v2 unsigned int"][..],
        &["name main::v2 count", "type main::count unsigned int count"][..],
    ] {
        let retyped = uncast(inspected_lines("long.elf", "main", asserts, &[("count", "v2")]));
        assert_eq!(retyped, plain, "{asserts:?}");
    }
}

/// A rename to a name the program already uses (`score` is a function) is
/// printed suffixed, and is applied: the page shows what it became instead of
/// a refusal.
#[test]
fn a_rename_the_printer_suffixes_is_applied() {
    let inspected = run_on("long.elf", "inspect", &["main"], &["name main::v7 score"]).expect("inspect");
    assert!(inspected["function"]["code"].as_str().expect("code").contains("  int score_1;"));
    let row = &inspected["assertions"].as_array().expect("array")[0];
    assert_eq!(row["status"].as_str(), Some("applied"));
    assert_eq!(row["detail"].as_str(), Some("applied as `score_1`: `score` is already another identifier here"));
}

/// The page edits a local by the name it shows: after `score` printed as
/// `score_1`, a rename and a retype of `score_1` reach that local.
#[test]
fn a_local_is_edited_by_its_suffixed_name() {
    let rows = |inspected: &serde_json::Value| -> Vec<(String, Option<String>)> {
        inspected["assertions"]
            .as_array()
            .expect("array")
            .iter()
            .map(|row| (row["status"].as_str().unwrap_or_default().to_string(), row["detail"].as_str().map(str::to_string)))
            .collect()
    };
    let renamed = run_on("long.elf", "inspect", &["main"], &["name main::v7 score", "name main::score_1 total"]).expect("inspect");
    assert!(renamed["function"]["code"].as_str().expect("code").contains("  int total;"));
    assert_eq!(rows(&renamed), vec![("applied".to_string(), None), ("applied".to_string(), None)]);
    let retyped =
        run_on("long.elf", "inspect", &["main"], &["name main::v7 score", "type main::score_1 unsigned int"]).expect("inspect");
    assert!(retyped["function"]["code"].as_str().expect("code").contains("  unsigned int score_1;"));
    assert!(rows(&retyped).iter().all(|(status, _)| status == "applied"), "{:?}", rows(&retyped));
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

fn token_globals(doc: &Value) -> Vec<(String, Option<String>)> {
    doc["function"]["tokens"]
        .as_array()
        .expect("tokens")
        .iter()
        .filter(|t| t["kind"].as_str() == Some("variable") && t.get("var").is_none() && t.get("decl").is_none())
        .map(|t| (t["text"].as_str().unwrap().to_string(), t["global_hex"].as_str().map(str::to_string)))
        .collect()
}

/// Every token that names global data carries the global's address: a symbol
/// the program names (`primes`) and one the decompiler names by its address
/// (`dat_4040` in the stripped twin), and `globals` lists both kinds. A local
/// never does.
#[test]
fn inspect_puts_the_address_of_global_data_on_its_tokens() {
    let named = run_on("globals.elf", "inspect", &["main"], &[]).expect("inspect main");
    let tokens = token_globals(&named);
    let at = |name: &str| tokens.iter().find(|(t, _)| t == name).and_then(|(_, g)| g.clone());
    assert_eq!(at("primes").as_deref(), Some("0x4040"), "{tokens:?}");
    assert_eq!(at("greeting").as_deref(), Some("0x4020"));
    assert_eq!(at("scratch").as_deref(), Some("0x40c0"));
    assert_eq!(at("origin").as_deref(), Some("0x4060"));
    assert_eq!(at("op").as_deref(), Some("0x4078"));
    assert_eq!(at("favorite").as_deref(), Some("0x4080"));
    assert_eq!(at("banner").as_deref(), Some("0x4070"));
    assert!(tokens.iter().filter(|(t, _)| t.starts_with('v')).all(|(_, g)| g.is_none()), "{tokens:?}");
    let globals = named["function"]["globals"].as_array().expect("globals");
    let primes = globals.iter().find(|g| g["name"].as_str() == Some("primes")).expect("primes listed");
    assert_eq!(primes["address_hex"].as_str(), Some("0x4040"));
    assert_eq!(primes["size"].as_u64(), Some(32));

    let retyped = run_on("globals.elf", "inspect", &["main"], &["data 0x4060 point origin"]).expect("inspect main");
    assert!(retyped["function"]["code"].as_str().unwrap().contains("(*op)(origin.x)"));
    assert!(token_globals(&retyped).iter().any(|(t, g)| t == "origin" && g.as_deref() == Some("0x4060")));

    let stripped = run_on("globals-stripped.elf", "inspect", &["main"], &[]).expect("inspect main");
    let tokens = token_globals(&stripped);
    assert!(tokens.iter().any(|(t, g)| t == "dat_4040" && g.as_deref() == Some("0x4040")), "{tokens:?}");
    let globals = stripped["function"]["globals"].as_array().expect("globals");
    let minted = globals.iter().find(|g| g["name"].as_str() == Some("dat_4078")).expect("dat_4078 listed");
    assert_eq!(minted["declaration"].as_str(), Some("int (*dat_4078)(unsigned int)"));
}

fn item<'a>(doc: &'a Value, name: &str) -> &'a Value {
    doc["items"]
        .as_array()
        .expect("items")
        .iter()
        .find(|i| i["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("{name} is an item: {:?}", doc["items"]))
}

/// `data` shows the section around a global: its labelled items with their
/// types and layouts, the bytes as loaded, and what each pointer word points
/// at. A retype is a `data` directive the window honours, a struct's fields
/// included; a `.bss` buffer has no file bytes behind it.
#[test]
fn data_shows_the_typed_items_around_a_global() {
    let doc = run_on("globals.elf", "data", &["0x4040"], &[]).expect("data");
    assert_eq!(doc["section"]["name"].as_str(), Some(".data"));
    assert_eq!(doc["pointer_size"].as_u64(), Some(8));
    let start = doc["start"].as_u64().expect("start");
    assert!(start <= 0x4020, "{doc:?}");
    let bytes = doc["bytes"].as_str().expect("bytes");
    let at = |addr: u64, len: usize| &bytes[(addr - start) as usize * 2..(addr - start) as usize * 2 + len * 2];
    assert_eq!(at(0x4040, 8), "0200000003000000");
    assert_eq!(at(0x4020, 12), hex(b"hello, data\0"));
    assert_eq!(item(&doc, "primes")["size"].as_u64(), Some(32));
    let targets = doc["targets"].as_array().expect("targets");
    let to = |at: &str| targets.iter().find(|t| t["at_hex"].as_str() == Some(at)).unwrap_or_else(|| panic!("{at}: {targets:?}"));
    assert_eq!(to("0x4078")["name"].as_str(), Some("twice"));
    assert_eq!(to("0x4080")["name"].as_str(), Some("primes"));
    assert_eq!(to("0x4080")["offset"].as_u64(), Some(12));
    assert_eq!(to("0x4070")["text"].as_str(), Some("kuna data tab"));

    let typed = run_on(
        "globals.elf",
        "data",
        &["0x4060"],
        &["data 0x4040 int primes[8]", "data 0x4060 point origin", "data 0x4078 int (*op)(int)"],
    )
    .expect("data");
    let primes = &item(&typed, "primes")["layout"];
    assert_eq!(primes["kind"].as_str(), Some("array"));
    assert_eq!(primes["count"].as_u64(), Some(8));
    assert_eq!(primes["elem"]["kind"].as_str(), Some("int"));
    let origin = item(&typed, "origin");
    assert_eq!(origin["declaration"].as_str(), Some("point origin"));
    let fields: Vec<(&str, u64, &str)> = origin["layout"]["fields"]
        .as_array()
        .expect("fields")
        .iter()
        .map(|f| (f["name"].as_str().unwrap(), f["offset"].as_u64().unwrap(), f["type"]["kind"].as_str().unwrap()))
        .collect();
    assert_eq!(fields, vec![("x", 0, "int"), ("y", 4, "int"), ("label", 8, "pointer")]);
    assert_eq!(item(&typed, "op")["declaration"].as_str(), Some("int (*op)(int)"));
    assert_eq!(item(&typed, "op")["layout"]["to"]["kind"].as_str(), Some("code"));

    let bss = run_on("globals.elf", "data", &["0x40c0"], &[]).expect("data");
    assert_eq!(bss["section"]["name"].as_str(), Some(".bss"));
    assert_eq!(bss["section"]["file_offset"], Value::Null);
    assert_eq!(item(&bss, "scratch")["size"].as_u64(), Some(64));

    let exact = run_on("globals.elf", "data", &["0x2010", "8"], &[]).expect("data");
    assert_eq!(exact["start_hex"].as_str(), Some("0x2010"));
    assert_eq!(exact["bytes"].as_str(), Some("e803000000000000"));
    assert_eq!(exact["section"]["name"].as_str(), Some(".rodata"));
    assert!(run_on("globals.elf", "data", &["0x4040", "0"], &[]).is_err());
    let nowhere = run_on("globals.elf", "data", &["0xfffffffffffffff8"], &[]).expect("data at the top of memory");
    assert_eq!(nowhere["size"].as_u64(), Some(0));
    assert_eq!(nowhere["section"], Value::Null);
}

/// `uses` names the instruction behind every use of the data in a range, with
/// the byte it uses: `main` takes the address of `primes` and reads `op`.
#[test]
fn uses_names_the_instructions_that_use_the_data() {
    let doc = run_on("globals.elf", "uses", &["0x4040", "0x48"], &[]).expect("uses");
    let uses = doc["uses"].as_array().expect("uses");
    let first = |to: &str| uses.iter().find(|u| u["to_hex"].as_str() == Some(to)).unwrap_or_else(|| panic!("{to}: {uses:?}"));
    assert_eq!(first("0x4040")["name"].as_str(), Some("main"));
    assert_eq!(first("0x4040")["kind"].as_str(), Some("data"));
    assert!(first("0x4040")["instruction"].as_str().unwrap().contains("0x4040"));
    assert_eq!(first("0x4078")["kind"].as_str(), Some("read"));
    let main = run_on("globals.elf", "inspect", &["main"], &[]).expect("inspect main");
    let sites: Vec<&str> = main["function"]["instructions"]
        .as_array()
        .expect("instructions")
        .iter()
        .filter_map(|i| i["address_hex"].as_str())
        .collect();
    for u in uses {
        assert_eq!(u["address_hex"], main["function"]["address_hex"], "{u:?}");
        assert!(sites.contains(&u["at_hex"].as_str().unwrap()), "{u:?}");
        let to = u["to"].as_u64().unwrap();
        assert!((0x4040..0x4088).contains(&to), "{u:?}");
    }
}

fn frame_of(name: &str, function: &str) -> (Value, Value, Value) {
    let doc = run_on(name, "inspect", &[function], &[]).unwrap_or_else(|e| panic!("inspect {function}: {e}"));
    let frame = &doc["function"]["frame"];
    (frame["frame_pointer"].clone(), frame["stack_pointer"].clone(), frame["base"].clone())
}

fn reg(register: &str, offset: i64) -> Value {
    serde_json::json!({ "register": register, "offset": offset })
}

/// `frame` says where the prologue leaves the frame pointer and the stack
/// pointer, from the stack pointer at entry, and which one the code reads its
/// stack through: RBP at the saved RBP after `PUSH RBP; MOV RBP,RSP`, also in a
/// leaf that pops it before returning; no frame pointer when optimized code
/// copies RSP into RBP after reserving its locals; the frame a shrink-wrapped
/// prologue sets up after its early-exit test or a conditional tail call, past a
/// CMOV between pushes, past an i386 get-PC thunk call (but not into a call
/// site's padding), and on ARM and Thumb past a conditional call or return; no stack pointer once a probe loop moves
/// it, also one entered at its test; x29 at AArch64's frame record, read
/// through sp when the code does, and none in a leaf that keeps no record.
#[test]
fn inspect_reports_the_frame_pointer_and_the_stack_pointer() {
    let (fp, sp) = (Value::from("frame_pointer"), Value::from("stack_pointer"));
    assert_eq!(frame_of("sample.elf", "main"), (reg("rbp", -8), reg("rsp", -0x28), fp.clone()));
    assert_eq!(frame_of("sample.elf", "add"), (reg("rbp", -8), reg("rsp", -8), fp.clone()));
    assert_eq!(frame_of("frames-O2.elf", "read_name"), (Value::Null, reg("rsp", -0x58), sp.clone()));
    assert_eq!(frame_of("frames-asm.elf", "shrink_wrapped"), (Value::Null, reg("rsp", -0x28), sp.clone()));
    assert_eq!(frame_of("frames-asm.elf", "probed"), (Value::Null, Value::Null, Value::Null));
    assert_eq!(frame_of("frames-asm.elf", "rotated"), (Value::Null, Value::Null, Value::Null));
    assert_eq!(frame_of("frames-asm.elf", "interleaved"), (Value::Null, reg("rsp", -0x20), sp.clone()));
    assert_eq!(frame_of("frames-asm.elf", "tail_first"), (Value::Null, reg("rsp", -0x28), sp.clone()));
    assert_eq!(frame_of("frames-i386.o", "pic"), (Value::Null, reg("esp", -0x44), sp.clone()));
    assert_eq!(frame_of("frames-i386.o", "padded"), (Value::Null, reg("esp", -0x1c), sp.clone()));
    for (function, depth) in [("condcall", -0x30), ("condret", -0x30), ("condpop", -0x40), ("tcondcall", -0x40)] {
        assert_eq!(frame_of("frames-arm.o", function), (Value::Null, reg("sp", depth), sp.clone()), "{function}");
    }
    assert_eq!(frame_of("sample_aarch64.o", "_start"), (reg("x29", -0x10), reg("sp", -0x30), sp.clone()));
    assert_eq!(frame_of("sample_aarch64.o", "sum_to"), (Value::Null, reg("sp", -0x20), sp));
}
