use crate::common;

/// `calltable`: an unchecked table of functions behind an indexed call is a
/// call table that ends where it runs into the next switch's table; an
/// unchecked jump table of 16-aligned labels and an entry on a switch case
/// nothing calls are not functions.
#[test]
fn call_table_targets_are_functions_and_switch_cases_are_not() {
    let binary = common::fixture("calltable_x86_64");
    let list = |extra: &[&str]| {
        let mut args = vec!["functions", binary.as_str(), "--json"];
        args.extend_from_slice(extra);
        let (out, stderr, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{stderr}");
        ["0x401040", "0x401070", "0x401080", "0x4010a0", "0x4010b0", "0x4010ce", "0x4010d4"]
            .map(|a| out.contains(&format!("\"{a}\"")))
    };
    assert_eq!(list(&[]), [false, true, true, false, false, false, false]);
    assert_eq!(list(&["--option", "calltable", "off"]), [true, false, false, false, false, false, false]);
}
