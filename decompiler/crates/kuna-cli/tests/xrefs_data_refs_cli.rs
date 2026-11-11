//! (GH #1333) `kuna xrefs` answers a pointer the image stores in its data as a
//! separate `pointer` row, from the image's own fixup metadata, and says which
//! metadata it read, so an empty answer is never ambiguous.

use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn xrefs(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("xrefs")
        .args(args)
        .output()
        .expect("run kuna xrefs");
    assert!(out.status.success(), "kuna xrefs {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn json(args: &[&str]) -> serde_json::Value {
    let mut argv = args.to_vec();
    argv.push("--json");
    serde_json::from_str(&xrefs(&argv)).expect("xrefs JSON")
}

/// The function-pointer slot `audit_func_ptr` is a `pointer` row beside the one
/// instruction call, in both chained-fixup layouts; `count` and `xrefs` keep
/// the instruction rows alone.
#[test]
fn a_stored_function_pointer_is_a_pointer_row() {
    for name in ["macho_ptrslots_arm64", "macho_ptrslots_offset_arm64"] {
        let doc = json(&[&fixture(name), "--to", "_audit_target"]);
        assert_eq!(doc["count"], 1, "{name}: the bl in audit_direct_call");
        assert_eq!(doc["xrefs"][0]["kind"], "call", "{name}");
        assert_eq!(doc["data_refs_coverage"], "chained-fixups", "{name}");
        assert_eq!(doc["data_refs_count"], 1, "{name}");
        let row = &doc["data_refs"][0];
        assert_eq!(row["kind"], "pointer");
        assert_eq!(row["from_address_hex"], "0x100004008");
        assert_eq!(row["to_address_hex"], "0x1000003c8");
        assert_eq!(row["width"], 8);
        assert_eq!(row["source"], "chained-fixups");

        let label = json(&[&fixture(name), "--to", "0x100000440"]);
        assert_eq!(label["data_refs"][0]["from_address_hex"], "0x100004000", "{name}: audit_label_ptr");
    }
}

/// `--kind` selects between the two: `pointer` alone drops the instruction rows,
/// an instruction kind alone drops the pointer rows. The text surface prints
/// them as their own block.
#[test]
fn kind_filters_pointer_rows() {
    let bin = fixture("macho_ptrslots_offset_arm64");
    let only = json(&[&bin, "--to", "_audit_target", "--kind", "pointer"]);
    assert_eq!((only["count"].as_u64(), only["data_refs_count"].as_u64()), (Some(0), Some(1)));
    let calls = json(&[&bin, "--to", "_audit_target", "--kind", "call"]);
    assert_eq!((calls["count"].as_u64(), calls["data_refs_count"].as_u64()), (Some(1), Some(0)));
    let text = xrefs(&[&bin, "--to", "_audit_target"]);
    assert!(
        text.contains("# 1 data-held pointer to _audit_target @ 0x1000003c8 (chained-fixups)\n0x100004008\tpointer\t"),
        "{text}"
    );
}

/// `--from` a data slot answers what it holds; a classic Mach-O lists its
/// pointers in rebase opcodes; a non-PIE ELF lists none and says so.
#[test]
fn coverage_is_reported_per_image() {
    let slot = json(&[&fixture("macho_ptrslots_offset_arm64"), "--from", "0x100004008"]);
    assert_eq!(slot["data_refs"][0]["to_address_hex"], "0x1000003c8");
    let classic = json(&[&fixture("macho_objc"), "--to", "0x100003000"]);
    assert_eq!(classic["data_refs_coverage"], "rebase-opcodes");
    assert_eq!(classic["data_refs"][0]["from_address_hex"], "0x100002000", "the __objc_classlist entry");
    let plain = json(&[&fixture("fauxware"), "--to", "main"]);
    assert_eq!(plain["data_refs_coverage"], "none");
    assert_eq!(plain["data_refs_count"], 0);
}

/// A shared object stores its exported function in a table through an absolute
/// relocation against the symbol; that word is a pointer row too.
#[test]
fn a_shared_objects_exported_function_table_is_listed() {
    let doc = json(&[&fixture("dataptrs_so_x86_64.so"), "--to", "audit_target"]);
    assert_eq!(doc["data_refs_coverage"], "dynamic-relocations");
    assert_eq!(doc["data_refs_count"], 1, "{doc}");
    assert_eq!(doc["data_refs"][0]["from_address_hex"], "0x4030");
    assert_eq!(doc["data_refs"][0]["from_name"], "exported_tab");
}

/// On x86 a relocated word inside a decoded instruction that names the same
/// address is that instruction's operand, not a stored pointer; a pointer table
/// in `.text` that starts just past such an instruction is still listed.
#[test]
fn an_x86_operand_is_the_instructions_row_and_a_text_table_is_a_pointer() {
    let exe = fixture("dataptrs_textptr_i386.exe");
    let rows = |doc: &serde_json::Value| -> Vec<String> {
        doc["data_refs"].as_array().unwrap().iter().map(|r| r["from_address_hex"].as_str().unwrap().to_string()).collect()
    };
    let dispatch = json(&[&exe, "--to", "0x401013"]);
    assert_eq!(dispatch["data_refs_coverage"], "base-relocations");
    assert_eq!(rows(&dispatch), ["0x401058", "0x403004"], "the mov's operand 0x40104f is not a pointer row: {dispatch}");
    assert!(dispatch["xrefs"].as_array().unwrap().iter().any(|r| r["from_address_hex"] == "0x40104e"), "{dispatch}");
    let helper = json(&[&exe, "--to", "0x40104e"]);
    assert_eq!(rows(&helper), ["0x401054", "0x403000"], "the push's operand 0x401001 is not a pointer row: {helper}");
}
