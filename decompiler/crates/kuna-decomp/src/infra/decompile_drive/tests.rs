//! Unit tests for the drive's diagnostic surface.
//!
//! [`panic_message`] is what every caught per-function pipeline abort
//! (`LOSS-131`) reports as its reason.  It used to be called with a
//! `&Box<dyn Any + Send>`, which unsize-coerces to the *box* as the `Any`, so
//! both downcasts failed and every error record read "panic with non-string
//! payload".  These round-trips are the regression gate for that.

use super::panic_message;

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
