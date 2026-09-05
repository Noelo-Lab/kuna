//! Tests for the `retsplitglobal` predicates: which epilogue COPY counts as an
//! observable global store, and where the clone bound falls.

use super::*;

#[test]
fn a_copy_into_a_persistent_location_is_a_global_store() {
    assert!(is_global_store(true, true, false, false));
}

#[test]
fn a_copy_into_a_temporary_is_not() {
    // The register/unique moves upstream's filter was written for.
    assert!(!is_global_store(true, false, false, false));
}

#[test]
fn a_guard_returns_self_copy_is_not_a_store() {
    // `Heritage::guardReturns` puts one `glob = glob` liveness marker per
    // persistent global before every RETURN; counting them would price a bare
    // epilogue by how many globals the whole program has.  Either recogniser
    // alone is enough.
    assert!(!is_global_store(true, true, true, false), "the return_copy flag");
    assert!(!is_global_store(true, true, false, true), "input and output name the same storage");
}

#[test]
fn only_a_copy_is_counted() {
    // A MULTIEQUAL writing a persistent Varnode is a phi, not a write.
    assert!(!is_global_store(false, true, false, false));
}

#[test]
fn the_bound_is_what_declines_a_split_not_the_first_store() {
    // A small global-writing epilogue still splits, exactly as upstream.
    for n in 0..=MAX_CLONED_GLOBAL_STORES {
        assert!(!split_is_declined(true, n), "{n} stores must still split");
    }
    assert!(split_is_declined(true, MAX_CLONED_GLOBAL_STORES + 1));
    assert!(split_is_declined(true, 72), "the witness epilogue");
}

#[test]
fn the_gate_off_restores_the_upstream_predicate() {
    assert!(!split_is_declined(false, 72));
}

#[test]
fn the_option_parses_on_and_off() {
    assert!(OptionRetSplitGlobal.apply("on").unwrap().0);
    assert!(!OptionRetSplitGlobal.apply("off").unwrap().0);
    assert_eq!(OptionRetSplitGlobal::NAME, "retsplitglobal");
}
