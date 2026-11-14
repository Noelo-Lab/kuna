//! (kuna, issues #1410, #1497) `--omit` narrows the `--json` record to what a
//! caller asked for, on `kuna decompile` and `kuna decompile-all` alike, and says
//! what it left out so an omitted collection is never read as an empty one.

use crate::common;

use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};

const ALL: &str = "line_mappings,variables,types";

fn json(args: &[&str]) -> (serde_json::Value, i32) {
    let (doc, _, code) = json_text(args);
    (doc, code)
}

fn json_text(args: &[&str]) -> (serde_json::Value, String, i32) {
    let specs = common::repo_root().join("specs");
    let mut argv = args.to_vec();
    argv.extend_from_slice(&["--sleighpath", specs.to_str().unwrap()]);
    let (stdout, stderr, code) = common::run_kuna(&argv);
    let doc = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("kuna {args:?} printed no JSON ({e}):\n{stdout}\n{stderr}"));
    (doc, stdout, code)
}

/// (issue #1497) An x86-64 object whose one `mov eax, 42; ret` body carries
/// 600 global names of 177 bytes each.
fn many_names() -> (String, String) {
    let mut o = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = o.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    o.append_section_data(text, &[0xb8, 42, 0, 0, 0, 0xc3], 1);
    let name = |i: usize| format!("audit_alias_{i:04}_{}", "x".repeat(160));
    for i in 0..600 {
        o.add_symbol(Symbol {
            name: name(i).into_bytes(), value: 0, size: 6, kind: SymbolKind::Text,
            scope: SymbolScope::Linkage, weak: false, section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let path = common::scratch_file("many-aliases", "o");
    std::fs::write(&path, o.write().unwrap()).unwrap();
    (path.to_str().unwrap().to_string(), name(300))
}

fn record(doc: &serde_json::Value) -> &serde_json::Map<String, serde_json::Value> {
    doc["functions"][0].as_object().expect("one function record")
}

#[test]
fn omitted_collections_are_absent_and_named_and_the_rest_is_unchanged() {
    let bin = common::fixture("fauxware");
    let (full, code) = json(&["decompile", &bin, "main", "--json"]);
    assert_eq!(code, 0);
    assert!(full.get("omitted").is_none(), "the default document carries no marker");
    assert!(!record(&full)["line_mappings"].as_array().unwrap().is_empty());

    let (compact, code) = json(&["decompile", &bin, "main", "--json", "--omit", ALL]);
    assert_eq!(code, 0);
    assert_eq!(
        compact["omitted"],
        serde_json::json!(["line_mappings", "variables", "types", "stack_objects"])
    );
    let (full_record, compact_record) = (record(&full), record(&compact));
    for key in ["line_mappings", "variables", "types", "stack_objects"] {
        assert!(!compact_record.contains_key(key), "{key} must be absent, not emptied");
    }
    for (key, value) in full_record {
        if !matches!(key.as_str(), "line_mappings" | "variables" | "types" | "stack_objects") {
            assert_eq!(compact_record.get(key), Some(value), "{key} must not change");
        }
    }
    for key in ["binary", "language", "count", "error", "assertions"] {
        assert_eq!(compact[key], full[key], "run-level {key} must not change");
    }

    let (lines_only, _) = json(&["decompile", &bin, "main", "--json", "--omit", "line_mappings"]);
    assert_eq!(lines_only["omitted"], serde_json::json!(["line_mappings"]));
    assert_eq!(
        record(&lines_only)["variables"], full_record["variables"],
        "variables keep their line evidence when only the mappings are omitted"
    );
}

#[test]
fn decompile_and_decompile_all_answer_with_one_compact_document() {
    let bin = common::fixture("fauxware");
    let (single, code) = json(&["decompile", &bin, "main", "--json", "--omit", ALL]);
    assert_eq!(code, 0);
    let (batch, code) = json(&[
        "decompile-all", &bin, "--functions", "main", "--json",
        "--omit", "types", "--omit", "line_mappings,variables",
    ]);
    assert_eq!(code, 0);
    assert_eq!(single, batch);

    let (single, _) = json(&["decompile", &bin, "main", "--json", "--no-vars"]);
    let (batch, _) = json(&["decompile-all", &bin, "--functions", "main", "--json", "--no-vars"]);
    assert_eq!(single, batch, "--no-vars means the same on both commands");
    assert!(single.get("omitted").is_none());
    assert_eq!(record(&single)["variables"], serde_json::json!([]));
    assert!(!record(&single)["line_mappings"].as_array().unwrap().is_empty());
}

#[test]
fn a_failed_selection_keeps_its_error_and_exit_status() {
    let bin = common::fixture("fauxware");
    let (doc, code) = json(&["decompile", &bin, "no_such_function", "--json", "--omit", ALL]);
    assert_eq!(code, 1);
    assert!(doc["error"].as_str().unwrap().contains("no_such_function"), "{doc}");
    assert_eq!(doc["count"], 0);
    assert!(doc["omitted"].is_array());
}

#[test]
fn record_selection_is_refused_without_json_or_with_an_unknown_key() {
    let bin = common::fixture("fauxware");
    for args in [
        vec!["decompile", bin.as_str(), "main", "--omit", "types"],
        vec!["decompile", bin.as_str(), "main", "--no-vars"],
        vec!["decompile-all", bin.as_str(), "--omit", "types"],
        vec!["decompile", bin.as_str(), "main", "--json", "--omit", "lines"],
        vec!["decompile-all", bin.as_str(), "--json", "--omit", ""],
        vec!["functions", bin.as_str(), "--json", "--omit", "types"],
    ] {
        let (stdout, stderr, code) = common::run_kuna(&args);
        assert_eq!(code, 2, "{args:?}: {stderr}");
        assert!(stdout.is_empty(), "{args:?} must not render a document");
        assert!(stderr.contains("--omit") || stderr.contains("--no-vars"), "{args:?}: {stderr}");
    }
}

#[test]
fn omitted_aliases_leave_the_name_address_and_code() {
    let (bin, alias) = many_names();
    let (full, full_text, code) = json_text(&["decompile", &bin, &alias, "--json"]);
    assert_eq!(code, 0);
    assert!(full.get("omitted").is_none());
    assert_eq!(record(&full)["aliases"].as_array().unwrap().len(), 599);

    let (compact, compact_text, code) =
        json_text(&["decompile", &bin, &alias, "--json", "--omit", "aliases"]);
    assert_eq!(code, 0);
    assert_eq!(compact["omitted"], serde_json::json!(["aliases"]));
    let (full_record, compact_record) = (record(&full), record(&compact));
    assert!(!compact_record.contains_key("aliases"), "aliases must be absent, not emptied");
    for (key, value) in full_record {
        if key != "aliases" {
            assert_eq!(compact_record.get(key), Some(value), "{key} must not change");
        }
    }
    assert!(compact_record["name"].as_str().unwrap().starts_with("audit_alias_"));
    assert!(compact_record["code"].as_str().unwrap().contains("return 0x2a;"));
    assert!(compact_text.len() * 20 < full_text.len(), "{} vs {}", compact_text.len(), full_text.len());

    let (all, code) = json(&["decompile", &bin, &alias, "--json", "--omit", "aliases", "--omit", ALL]);
    assert_eq!(code, 0);
    assert_eq!(
        all["omitted"],
        serde_json::json!(["aliases", "line_mappings", "variables", "types", "stack_objects"])
    );
    assert_eq!(record(&all)["code"], full_record["code"]);

    let (batch, code) = json(&[
        "decompile-all", &bin, "--functions", &alias, "--json", "--omit", "aliases",
    ]);
    assert_eq!(code, 0);
    assert_eq!(batch, compact);

    let (stdout, stderr, code) =
        common::run_kuna(&["functions", &bin, "--json", "--omit", "aliases"]);
    assert_eq!(code, 2, "the inventory keeps every name: {stderr}");
    assert!(stdout.is_empty());

    let (_, stderr, code) = common::run_kuna(&["decompile", &bin, &alias, "--json", "--omit", "alias"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("aliases, line_mappings, variables, types"), "{stderr}");
}
