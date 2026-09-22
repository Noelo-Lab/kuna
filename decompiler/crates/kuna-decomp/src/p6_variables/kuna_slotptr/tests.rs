//! Unit tests for the `slotptr` option parser and evidence record.

use super::*;

#[test]
fn slotptr_values_round_trip() {
    for v in ["off", "on", "void"] {
        assert_eq!(SlotPtrMode::parse(v).map(|m| m.as_str()), Some(v));
    }
    assert_eq!(SlotPtrMode::parse("yes"), None);
}
