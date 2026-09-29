use super::merge_type_definitions;

fn merge(blocks: &[&str]) -> String {
    let blocks: Vec<_> = blocks.iter().map(|block| (*block).to_owned()).collect();
    merge_type_definitions(&blocks, "test")
}

#[test]
fn first_containing_block_is_returned_verbatim() {
    let first = " \r\nstruct a;\r\n\n";
    assert_eq!(merge(&[first, "struct a;\n"]), first);
    let full = "\r\nstruct b;\r\nstruct a;\r\n";
    assert_eq!(merge(&["struct a;\n", full, "struct b;\n"]), full);
}

#[test]
fn shared_members_do_not_split_complete_definitions() {
    let first = "struct a {\n    int shared;\n};\n";
    let second = "struct b {\n    int shared;\n};\n";
    assert_eq!(merge(&[first, second, first]), format!("{first}{second}"));
}

#[test]
fn appended_definitions_keep_existing_line_normalization() {
    assert_eq!(
        merge(&["typedef int a;\n", "\r\nstruct b {\r\n    int x;\r\n};"]),
        "typedef int a;\n\nstruct b {\n    int x;\n};\n"
    );
    assert_eq!(merge(&["int a;\n", "int b;\r"]), "int a;\nint b;\r\n");
}

#[test]
fn the_first_block_is_not_given_a_missing_newline() {
    assert_eq!(merge(&["int a;", "int b;\n"]), "int a;int b;\n");
}

#[test]
fn unicode_blank_lines_preserve_the_spacing_decision() {
    assert_eq!(
        merge(&["int a;\n", "\u{a0}\nstruct é;\n"]),
        "int a;\n\nstruct é;\n"
    );
}

#[test]
fn incomplete_definitions_keep_their_remaining_lines() {
    assert_eq!(
        merge(&["struct a {\n    int x;\n\n", "int b;\n"]),
        "struct a {\n    int x;\n\nint b;\n"
    );
}

#[test]
fn blocks_without_items_still_preserve_the_first_block() {
    assert_eq!(merge(&[" \r\n\t\n", ""]), " \r\n\t\n");
    assert_eq!(merge(&[]), "");
}

#[test]
fn canonical_items_borrow_their_original_spans() {
    let block = "struct a;\n\nstruct b {\n    int x;\n};\n";
    let items = super::type_items(block);
    assert_eq!(items.len(), 2);
    assert!(items
        .iter()
        .all(|(_, text)| matches!(text, std::borrow::Cow::Borrowed(_))));
    assert_eq!(items[0], (false, std::borrow::Cow::Borrowed("struct a;\n")));
    assert_eq!(
        items[1],
        (
            true,
            std::borrow::Cow::Borrowed("struct b {\n    int x;\n};\n")
        )
    );
    let normalized = super::type_items("struct a;\r\nstruct b;");
    assert!(normalized
        .iter()
        .all(|(_, text)| matches!(text, std::borrow::Cow::Owned(_))));
    assert_eq!(normalized[0].1, "struct a;\n");
    assert_eq!(normalized[1].1, "struct b;\n");
}
