//! Unit tests for the `zerocallregs` option.

use super::*;

#[test]
fn option_parses_on_and_off() {
    assert_eq!(OptionZeroCallRegs::NAME, "zerocallregs");
    assert!(OptionZeroCallRegs.apply("on").unwrap().0);
    assert!(!OptionZeroCallRegs.apply("off").unwrap().0);
    assert!(OptionZeroCallRegs.apply("maybe").is_err());
}

#[test]
fn flag_ops_are_comparisons() {
    assert!(is_flag_op(OpCode::CPUI_INT_SLESS));
    assert!(is_flag_op(OpCode::CPUI_INT_EQUAL));
    assert!(!is_flag_op(OpCode::CPUI_COPY));
    assert!(!is_flag_op(OpCode::CPUI_INT_ADD));
}
