//! Unit tests for the drive's diagnostic surface.
//!
//! [`panic_message`] is what every caught per-function pipeline abort
//! (`LOSS-131`) reports as its reason.  It used to be called with a
//! `&Box<dyn Any + Send>`, which unsize-coerces to the *box* as the `Any`, so
//! both downcasts failed and every error record read "panic with non-string
//! payload".  These round-trips are the regression gate for that.

use std::collections::{BTreeMap, BTreeSet};

use crate::prettyprint::{MarkupAssociation, MarkupProvenance};

use super::{
    align_markup_to_trimmed, panic_message, resolve_markup_provenance_with_addresses,
    rewrite_cookie_literal_returns, stack_storage_contains,
};

#[test]
fn aggregate_stack_storage_accepts_only_fully_contained_pieces() {
    assert!(stack_storage_contains(-88, 32, -88, 8));
    assert!(stack_storage_contains(-88, 32, -64, 8));

    // Partial overlaps on either edge belong to another slot, not this array.
    assert!(!stack_storage_contains(-88, 32, -89, 8));
    assert!(!stack_storage_contains(-88, 32, -64, 9));
    // A piece beginning at the next slot is adjacent, not contained.
    assert!(!stack_storage_contains(-88, 32, -56, 8));
    assert!(!stack_storage_contains(-88, 32, -88, 0));
    assert!(!stack_storage_contains(i64::MAX - 3, 8, i64::MAX - 3, 1));
}

fn cookie_calls(opref: u64) -> BTreeMap<u64, BTreeSet<String>> {
    BTreeMap::from([(opref, BTreeSet::from(["security_check_cookie".into()]))])
}

#[test]
fn exact_cookie_literal_return_rewrite_keeps_call_and_line_slots() {
    // Real documents begin with the sink newline inserted by `tag_line()`, but
    // emitter provenance numbers the following content as line 1.
    let before =
        "\n  v7 = 1;\n  security_check_cookie(cookie); // return-dupe\n  return v7;\n".to_string();
    let mut markup = MarkupProvenance {
        associations: vec![
            MarkupAssociation { line_number: 1, opref: Some(5), varref: Some(70) },
            MarkupAssociation { line_number: 2, opref: Some(7), varref: None },
            MarkupAssociation { line_number: 3, opref: Some(9), varref: Some(70) },
        ],
    };
    let after = rewrite_cookie_literal_returns(before, &cookie_calls(7), &mut markup);
    assert_eq!(
        after,
        "\n  \n  security_check_cookie(cookie); // return-dupe\n  return 1;\n"
    );

    let provenance = resolve_markup_provenance_with_addresses(
        &markup,
        &BTreeMap::from([(5, 0x401000), (7, 0x401004), (9, 0x401008)]),
    );
    assert_eq!(
        provenance.line_mappings,
        vec![
            super::LineMapping { line_number: 2, addresses: vec![0x401004] },
            super::LineMapping {
                line_number: 3,
                addresses: vec![0x401000, 0x401008],
            },
        ]
    );
    assert_eq!(provenance.variable_uses[&70].line_numbers, vec![3]);
    assert_eq!(provenance.variable_uses[&70].addresses, vec![0x401000, 0x401008]);
}

#[test]
fn exact_cookie_literal_return_rewrite_binds_the_marked_call_occurrence() {
    let before = concat!(
        "  v7 = 1;\n",
        "  security_check_cookie(cookie);\n",
        "  return v7;\n",
        "  v8 = 0;\n",
        "  security_check_cookie(cookie);\n",
        "  return v8;\n",
    )
    .to_string();
    let mut markup = MarkupProvenance {
        associations: vec![
            MarkupAssociation { line_number: 2, opref: Some(8), varref: None },
            MarkupAssociation { line_number: 5, opref: Some(7), varref: None },
        ],
    };
    let after = rewrite_cookie_literal_returns(before, &cookie_calls(7), &mut markup);
    assert_eq!(
        after,
        concat!(
            "  v7 = 1;\n",
            "  security_check_cookie(cookie);\n",
            "  return v7;\n",
            "  \n",
            "  security_check_cookie(cookie);\n",
            "  return 0;\n",
        )
    );
}

#[test]
fn exact_cookie_literal_return_rewrite_declines_ambiguous_text() {
    for text in [
        "  v7 = x;\n  security_check_cookie(cookie);\n  return v7;\n",
        "  v7 = 1;\n  other(cookie);\n  return v7;\n",
        "  v7 = 1;\n  security_check_cookie(cookie);\n  return v8;\n",
        "  v7 = 1;\n  side_effect();\n  security_check_cookie(cookie);\n  return v7;\n",
        "  v7 = 1;\n  security_check_cookie(cookie); side_effect();\n  return v7;\n",
        "  v7 = 1;\n  security_check_cookie(cookie) + side_effect();\n  return v7;\n",
    ] {
        let mut markup = MarkupProvenance {
            associations: vec![MarkupAssociation {
                line_number: 2,
                opref: Some(7),
                varref: None,
            }],
        };
        assert_eq!(
            rewrite_cookie_literal_returns(text.into(), &cookie_calls(7), &mut markup),
            text
        );
    }
}

/// A `panic!("literal")` payload is a `&'static str`.
#[test]
fn panic_message_recovers_str_payload() {
    let payload = std::panic::catch_unwind(|| panic!("un-ported stub: normalizeWriteSize"))
        .expect_err("the closure panics");
    assert_eq!(panic_message(payload), "un-ported stub: normalizeWriteSize");
}

/// A formatted `panic!("{}", ..)` payload — and the `unwrap`/`expect` family's —
/// is a `String`.
#[test]
fn panic_message_recovers_string_payload() {
    let payload = std::panic::catch_unwind(|| panic!("op {} has no output", 42))
        .expect_err("the closure panics");
    assert_eq!(panic_message(payload), "op 42 has no output");
}

/// The real shape the corpus hits: `Option::unwrap` on `None` (a `String`
/// payload built by the standard library).
#[test]
fn panic_message_recovers_unwrap_payload() {
    let payload = std::panic::catch_unwind(|| Option::<u32>::None.unwrap())
        .expect_err("the closure panics");
    assert!(
        panic_message(payload).contains("called `Option::unwrap()` on a `None` value"),
        "unwrap panics must round-trip their standard-library message"
    );
}

/// A payload that is neither string type still degrades to the documented text.
#[test]
fn panic_message_reports_non_string_payload() {
    let payload =
        std::panic::catch_unwind(|| std::panic::panic_any(7u32)).expect_err("the closure panics");
    assert_eq!(panic_message(payload), "panic with non-string payload");
}

#[test]
fn provenance_merges_sorted_line_and_variable_evidence() {
    let markup = MarkupProvenance {
        associations: vec![
            MarkupAssociation {
                line_number: 2,
                opref: Some(3),
                varref: None,
            },
            MarkupAssociation {
                line_number: 2,
                opref: Some(1),
                varref: Some(10),
            },
            MarkupAssociation {
                line_number: 3,
                opref: Some(2),
                varref: Some(11),
            },
            MarkupAssociation {
                line_number: 2,
                opref: Some(1),
                varref: Some(20),
            },
            MarkupAssociation {
                line_number: 0,
                opref: Some(1),
                varref: Some(12),
            },
            MarkupAssociation {
                line_number: 4,
                opref: Some(99),
                varref: None,
            },
        ],
    };
    let addresses = BTreeMap::from([(1, 0x401004), (2, 0x401008), (3, 0x401004)]);
    let provenance = resolve_markup_provenance_with_addresses(&markup, &addresses);

    assert_eq!(
        provenance.line_mappings,
        vec![
            super::LineMapping { line_number: 2, addresses: vec![0x401004] },
            super::LineMapping { line_number: 3, addresses: vec![0x401008] },
        ]
    );
    assert_eq!(provenance.variable_uses[&10].line_numbers, vec![2]);
    assert_eq!(provenance.variable_uses[&10].addresses, vec![0x401004]);
    assert_eq!(provenance.variable_uses[&11].line_numbers, vec![3]);
    assert_eq!(provenance.variable_uses[&11].addresses, vec![0x401008]);
    assert_eq!(provenance.variable_uses[&20].line_numbers, vec![2]);
}

/// A render that opens with two breaks (the Rust attribute line) numbers its
/// first reported line 2; the provenance must follow the trimmed text.
#[test]
fn markup_lines_follow_the_trimmed_text() {
    let association = |line_number| MarkupAssociation { line_number, opref: Some(1), varref: None };
    let mut markup = MarkupProvenance { associations: vec![association(2), association(4)] };
    align_markup_to_trimmed(&mut markup, "\n\n#[allow]\nfn f()\n{\n}");
    let lines: Vec<usize> = markup.associations.iter().map(|a| a.line_number).collect();
    assert_eq!(lines, vec![1, 3]);

    let mut single = MarkupProvenance { associations: vec![association(1), association(3)] };
    align_markup_to_trimmed(&mut single, "\nint f(void)\n{\n}");
    let lines: Vec<usize> = single.associations.iter().map(|a| a.line_number).collect();
    assert_eq!(lines, vec![1, 3]);
}
