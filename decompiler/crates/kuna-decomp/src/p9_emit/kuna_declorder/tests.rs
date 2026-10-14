use super::*;

fn ordered_as(names: &[&str], mode: DeclOrder, angr_names: bool) -> Vec<String> {
    let mut decls: Vec<((), String)> = names.iter().map(|n| ((), n.to_string())).collect();
    decls.sort_by(|a, b| a.1.cmp(&b.1));
    order_decls(&mut decls, mode, angr_names);
    decls.into_iter().map(|(_, n)| n).collect()
}

fn ordered(names: &[&str], mode: DeclOrder) -> Vec<String> {
    ordered_as(names, mode, true)
}

#[test]
fn natural_puts_two_digit_names_after_one_digit_names() {
    let names = ["v2", "v10", "v1", "v11", "v9", "v3"];
    assert_eq!(ordered(&names, DeclOrder::Natural), ["v1", "v2", "v3", "v9", "v10", "v11"]);
    assert_eq!(ordered(&names, DeclOrder::Name), ["v1", "v10", "v11", "v2", "v3", "v9"]);
}

#[test]
fn natural_compares_text_before_and_between_numbers() {
    let names = ["buf10", "buf9", "count", "v1_2", "v1_10", "a2", "v2"];
    assert_eq!(
        ordered(&names, DeclOrder::Natural),
        ["a2", "buf9", "buf10", "count", "v1_2", "v1_10", "v2"]
    );
}

#[test]
fn ghidra_style_names_keep_string_order() {
    let names = ["local_1c", "local_108", "local_18", "uVar10", "uVar9"];
    assert_eq!(
        ordered_as(&names, DeclOrder::Natural, false),
        ["local_108", "local_18", "local_1c", "uVar10", "uVar9"]
    );
}

#[test]
fn natural_is_a_total_order_that_agrees_with_equality() {
    assert_eq!(natural_cmp("v01", "v1"), "v01".cmp("v1"));
    assert_ne!(natural_cmp("v01", "v1"), Ordering::Equal);
    assert_eq!(natural_cmp("v7", "v7"), Ordering::Equal);
    assert_eq!(natural_cmp("v", "v1"), Ordering::Less);
    assert_eq!(natural_cmp("v1", "v1x"), Ordering::Less);
    assert_eq!(natural_cmp("v18446744073709551616", "v9"), Ordering::Greater);
}

#[test]
fn option_parses_both_values_and_rejects_others() {
    assert_eq!(OptionDeclOrder.apply("natural").unwrap().0, DeclOrder::Natural);
    assert_eq!(OptionDeclOrder.apply("name").unwrap().0, DeclOrder::Name);
    assert!(OptionDeclOrder.apply("on").is_err());
}
