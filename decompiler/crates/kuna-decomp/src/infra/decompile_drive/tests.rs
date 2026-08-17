//! Unit tests for the drive's diagnostic surface.
//!
//! [`panic_message`] is what every caught per-function pipeline abort
//! (`LOSS-131`) reports as its reason.  It used to be called with a
//! `&Box<dyn Any + Send>`, which unsize-coerces to the *box* as the `Any`, so
//! both downcasts failed and every error record read "panic with non-string
//! payload".  These round-trips are the regression gate for that.

use std::collections::BTreeMap;

use crate::prettyprint::{MarkupAssociation, MarkupProvenance};

use super::{panic_message, resolve_markup_provenance_with_addresses};

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
