//! Tests for the return-register pairing sub-stage (kuna GH-6990):
//! [`parse_return_pair_form`] (the `OptionReturnPair::apply` decision body) and
//! [`keep_single_return`] (the `buildReturnOutput` resize gate), including the
//! gate-off (`pair`) behavior.

use super::*;

// --- option parse (OptionReturnPair::apply) ----------------------------------

#[test]
fn parse_single_sets_return_single() {
    let (form, msg) = parse_return_pair_form("single").unwrap();
    assert_eq!(form, ReturnPairForm::Single);
    assert!(form.return_single()); // glb->return_single = true
    assert_eq!(form.as_str(), "single");
    assert_eq!(msg, "Return-register pairing set to single form");
}

#[test]
fn parse_pair_clears_return_single() {
    let (form, msg) = parse_return_pair_form("pair").unwrap();
    assert_eq!(form, ReturnPairForm::Pair);
    assert!(!form.return_single()); // glb->return_single = false (upstream join)
    assert_eq!(form.as_str(), "pair");
    assert_eq!(msg, "Return-register pairing set to pair form");
}

#[test]
fn parse_unknown_is_error() {
    // C++ returns the literal "Must specify pair or single"; the Rust port
    // surfaces it as a parse error carrying the same text.
    let err = parse_return_pair_form("both").unwrap_err();
    assert!(format!("{err}").contains("Must specify pair or single"));
    // empty string is also rejected (not "pair"/"single").
    assert!(parse_return_pair_form("").is_err());
}

// --- buildReturnOutput resize gate -------------------------------------------

#[test]
fn keep_single_truncates_multi_register_when_single() {
    // single + (indirect ref + lo + hi) = 3 > 2  => truncate to [ref, lo].
    assert!(keep_single_return(true, 3));
    // even more registers gathered.
    assert!(keep_single_return(true, 4));
}

#[test]
fn keep_single_passes_through_zero_or_one_return() {
    // newparam = [indirect ref]            (size 1: void return)
    assert!(!keep_single_return(true, 1));
    // newparam = [indirect ref, lo]        (size 2: single-register return)
    assert!(!keep_single_return(true, 2));
}

#[test]
fn gate_off_pair_never_truncates() {
    // `pair` form (return_single = false): the join is always allowed,
    // regardless of how many registers were gathered (upstream behavior).
    assert!(!keep_single_return(false, 1));
    assert!(!keep_single_return(false, 2));
    assert!(!keep_single_return(false, 3));
    assert!(!keep_single_return(false, 8));
    // And the parsed `pair` form yields exactly that flag.
    let (form, _) = parse_return_pair_form("pair").unwrap();
    assert!(!keep_single_return(form.return_single(), 3));
}

/// End-to-end: the `single` option flows into the gate and truncates a
/// three-register gather; the `pair` option leaves it intact.
#[test]
fn parsed_form_drives_gate() {
    let (single, _) = parse_return_pair_form("single").unwrap();
    let (pair, _) = parse_return_pair_form("pair").unwrap();
    assert!(keep_single_return(single.return_single(), 3));
    assert!(!keep_single_return(pair.return_single(), 3));
}
