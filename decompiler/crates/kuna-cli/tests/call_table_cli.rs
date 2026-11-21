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

/// `calltable`: an unchecked call table ends at the next table its code names
/// even when nothing decoded that code (`ops` runs into the table of `bsw`,
/// reached only through `ops`), and the walk votes a switch table it read
/// without a count by the strict rule, so `sw`'s table running into a table of
/// known functions stays a switch.
#[test]
fn call_table_ends_at_the_next_named_table_and_uncounted_switches_stay_switches() {
    let binary = common::fixture("calltable_split_x86_64");
    let (out, stderr, rc) = common::run_kuna(&["functions", binary.as_str(), "--json"]);
    assert_eq!(rc, 0, "{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&out).unwrap();
    let listed: Vec<&str> = doc["functions"].as_array().unwrap().iter().map(|f| f["address_hex"].as_str().unwrap()).collect();
    assert!(listed.contains(&"0x401050"), "h0, reached only through ops: {listed:?}");
    let bsw = ["0x401138", "0x401150", "0x401160", "0x401170", "0x401180", "0x401190", "0x4011a0", "0x4011b0"];
    for case in bsw {
        assert!(!listed.contains(&case), "{case} is a case of bsw: {listed:?}");
    }
    for case in ["0x401138", "0x401160", "0x401268", "0x401290"] {
        let (out, stderr, rc) = common::run_kuna(&["xrefs", binary.as_str(), "--to", case, "--json"]);
        assert_eq!(rc, 0, "{stderr}");
        let doc: serde_json::Value = serde_json::from_str(&out).unwrap();
        let kinds: Vec<&str> = doc["xrefs"].as_array().unwrap().iter().map(|x| x["kind"].as_str().unwrap()).collect();
        assert_eq!(kinds, ["jump"], "{case} is a switch case");
    }
}
