//! Unit tests for the `decodehalt` option surface.

use super::*;
use crate::op::pcodeop_flags as f;

#[test]
fn option_parses_on_and_off() {
    assert!(OptionDecodeHalt.apply("on").unwrap().0);
    assert!(!OptionDecodeHalt.apply("off").unwrap().0);
    assert!(OptionDecodeHalt.apply("halt").is_err());
}

#[test]
fn decode_failure_bits_map_to_their_upstream_pseudo_calls() {
    assert_eq!(halt_call_name(f::badinstruction), Some("halt_baddata"));
    assert_eq!(halt_call_name(f::unimplemented), Some("halt_unimplemented"));
    assert_eq!(halt_call_name(f::missing), Some("halt_missing"));
}

#[test]
fn noreturn_and_halt_and_plain_returns_are_left_alone() {
    assert_eq!(halt_call_name(0), None);
    assert_eq!(halt_call_name(f::noreturn), None);
    assert_eq!(halt_call_name(f::halt), None);
}

#[test]
fn bad_data_wins_a_combined_mask() {
    // `op_mark_halt` ORs, so a re-marked op can carry two bits; the upstream
    // switch order decides, and bad data is the more specific fact.
    assert_eq!(
        halt_call_name(f::badinstruction | f::noreturn),
        Some("halt_baddata")
    );
}

#[test]
fn warning_texts_are_the_upstream_strings() {
    assert!(BAD_INSTRUCTION_WARNING.starts_with("Bad instruction"));
    assert!(BAD_INSTRUCTION_HEADER.contains("bad instruction data"));
    assert!(UNIMPLEMENTED_WARNING.starts_with("Unimplemented instruction"));
    assert!(UNIMPLEMENTED_HEADER.contains("encountered unimplemented"));
    assert!(UNIMPLEMENTED_IGNORED_HEADER.contains("ignored unimplemented"));
}

#[test]
fn element_id_is_in_the_kuna_range() {
    assert!(ELEM_DECODEHALT.get_id() >= 4000);
    assert_eq!(ELEM_DECODEHALT.get_name(), "decodehalt");
}
