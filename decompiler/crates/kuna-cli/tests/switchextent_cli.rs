use crate::common;

/// `switchextent`: a compact AArch64 table no range check bounds ends where
/// the function's next table begins; the function's last table, and one
/// followed only by an array the function reads, stay computed calls.
#[test]
fn an_unchecked_table_ends_at_the_functions_next_table() {
    let binary = common::fixture("switchextent_aarch64");
    let decompile = |name: &str, extra: &[&str]| {
        let mut args = vec!["decompile", binary.as_str(), name];
        args.extend_from_slice(extra);
        let (out, stderr, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{stderr}");
        out
    };
    let unchecked = decompile("unchecked", &[]);
    assert!(unchecked.contains("switch(pick(a0) & 0xffffffff) {"), "{unchecked}");
    assert_eq!(unchecked.matches("case ").count(), 3, "{unchecked}");
    assert_eq!(unchecked.matches("// jump-as-call").count(), 1, "the last table has no bound: {unchecked}");
    let off = decompile("unchecked", &["--option", "switchextent", "off"]);
    assert!(!off.contains("switch(") && off.contains("// jump-as-call"), "{off}");
    let last = decompile("last", &[]);
    assert!(!last.contains("switch(") && last.contains("dat_1116e["), "bytes_last is not a table: {last}");
    let entering = decompile("entering", &[]);
    assert_eq!(entering.matches("case ").count(), 3, "{entering}");
}

/// The reference walk reads the same tables: the guarded dispatch, the one
/// whose check branches into its block, and the unchecked one the next table
/// bounds file a jump to each case; the function's last table files none.
#[test]
fn the_reference_walk_files_a_jump_to_each_case() {
    let binary = common::fixture("switchextent_aarch64");
    let jumps = |from: &str| {
        let (out, stderr, rc) = common::run_kuna(&["xrefs", binary.as_str(), "--from", from, "--json"]);
        assert_eq!(rc, 0, "{stderr}");
        let doc: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut to: Vec<String> = doc["xrefs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x["kind"] == "jump")
            .map(|x| x["to_address_hex"].as_str().unwrap_or_default().to_string())
            .collect();
        to.sort();
        to
    };
    assert_eq!(jumps("0x1001c"), ["0x10020", "0x10028", "0x10030", "0x10038"], "guarded");
    assert_eq!(jumps("0x1006c"), ["0x10070", "0x10078", "0x10080"], "entering");
    assert_eq!(jumps("0x100b8"), ["0x100bc", "0x100c4", "0x100cc"], "unchecked, first table");
    assert!(jumps("0x100ec").is_empty(), "unchecked, last table");
    assert!(jumps("0x10138").is_empty(), "last");
}

/// `switchextent`: a Swift enum's table is bounded by the enum's case count
/// from its descriptor, also when it is the function's last; the halfwords
/// after it are not rows.
#[test]
fn a_swift_enum_table_has_its_enums_case_count() {
    let binary = common::fixture("swiftenumcase_aarch64");
    let decompile = |extra: &[&str]| {
        let mut args = vec!["decompile", binary.as_str(), "dispatch"];
        args.extend_from_slice(extra);
        let (out, stderr, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{stderr}");
        out
    };
    let on = decompile(&[]);
    assert!(on.contains("switch(swift_getEnumCaseMultiPayload(a0,v1) & 0xffffffff) {"), "{on}");
    assert_eq!(on.matches("case ").count(), 3, "{on}");
    assert!(!on.contains("// jump-as-call"), "{on}");
    let off = decompile(&["--option", "switchextent", "off"]);
    assert!(!off.contains("switch(") && off.contains("// jump-as-call"), "{off}");
    let nometa = decompile(&["--option", "swiftmetadata", "off"]);
    assert!(!nometa.contains("switch(") && nometa.contains("// jump-as-call"), "{nometa}");
}
