use crate::common;

/// `calltable`: an unchecked table of functions is a call table (its targets
/// are functions); an entry on a switch case nothing calls is not a function.
#[test]
fn call_table_targets_are_functions_and_switch_cases_are_not() {
    let binary = common::fixture("calltable_x86_64");
    let list = |extra: &[&str]| {
        let mut args = vec!["functions", binary.as_str(), "--json"];
        args.extend_from_slice(extra);
        let (out, stderr, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{stderr}");
        ["0x401030", "0x401060", "0x401070"].map(|a| out.contains(&format!("\"{a}\"")))
    };
    assert_eq!(list(&[]), [false, true, true]);
    assert_eq!(list(&["--option", "calltable", "off"]), [true, false, false]);
}
